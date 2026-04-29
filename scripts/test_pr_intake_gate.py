#!/usr/bin/env python3
"""Fixture-backed tests for scripts/pr_intake_gate.py."""

from __future__ import annotations

import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

from scripts.pr_intake_gate import (  # noqa: E402
    get_label_details,
    is_gate_comment,
    load_minimal_yaml,
    missing_required_sections,
    path_matches,
)

FULL_EXTERNAL_BODY = """## External contributor context

### Problem

A concrete problem.

### Why now

This blocks current contributor work.

### Existing options checked

The current docs and issues do not cover this.

### Alternatives considered

Docs-only and process-only options were considered.

### No-code alternative

No-code alternative is insufficient because the behavior needs a repository check.

### Why code is needed

A deterministic check is required.

Closes #42
"""

FULL_CONTEXT_NO_LINK_BODY = FULL_EXTERNAL_BODY.replace("\nCloses #42\n", "\n")
MISSING_NO_CODE_BODY = """## External contributor context

### Problem

A concrete problem.

### Why now

This blocks current contributor work.

### Existing options checked

The current docs and issues do not cover this.

### Alternatives considered

Docs-only and process-only options were considered.

### Why code is needed

A deterministic check is required.

Closes #42
"""
MISSING_CONTEXT_BODY = """## External contributor context

### Problem

A concrete problem.

### No-code alternative

No-code alternative is insufficient.

Closes #42
"""


def write_event(path: Path, body: str, labels: list[str], association: str, author_login: str = "contributor") -> None:
    event = {
        "repository": {"full_name": "heurema/punk"},
        "pull_request": {
            "number": 123,
            "title": "Test PR",
            "body": body,
            "author_association": association,
            "user": {"login": author_login},
            "labels": [{"name": label} for label in labels],
            "base": {"sha": "base-sha"},
            "head": {"sha": "head-sha"},
        },
    }
    path.write_text(json.dumps(event), encoding="utf-8")


def run_case(
    name: str,
    expected_status: int,
    expected_verdict: str,
    files: list[dict[str, object]],
    body: str = "",
    labels: list[str] | None = None,
    association: str = "CONTRIBUTOR",
    author_permission: str | None = None,
) -> dict[str, object]:
    labels = labels or []
    with tempfile.TemporaryDirectory(prefix=f"punk-pr-intake-{name}-") as tmp_raw:
        tmp = Path(tmp_raw)
        event_path = tmp / "event.json"
        stdout_path = tmp / "stdout.json"
        summary_path = tmp / "summary.md"
        write_event(event_path, body, labels, association)

        env = os.environ.copy()
        env.update(
            {
                "GITHUB_EVENT_PATH": str(event_path),
                "GITHUB_STEP_SUMMARY": str(summary_path),
                "PR_INTAKE_GATE_CHANGED_FILES_JSON": json.dumps(files),
                "PR_INTAKE_GATE_DRY_RUN": "1",
            }
        )
        if author_permission is not None:
            env["PR_INTAKE_GATE_AUTHOR_PERMISSION"] = author_permission
        result = subprocess.run(
            [sys.executable, "scripts/pr_intake_gate.py"],
            cwd=ROOT,
            env=env,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        stdout_path.write_text(result.stdout, encoding="utf-8")

        if result.returncode != expected_status:
            raise AssertionError(
                f"{name}: expected exit {expected_status}, got {result.returncode}\n"
                f"stdout:\n{result.stdout}\nstderr:\n{result.stderr}"
            )
        payload = json.loads(result.stdout)
        if payload["verdict"] != expected_verdict:
            raise AssertionError(f"{name}: expected verdict {expected_verdict}, got {payload['verdict']}")
        summary = summary_path.read_text(encoding="utf-8")
        if "PR Intake Gate" not in summary:
            raise AssertionError(f"{name}: missing step summary")
        print(f"ok - {name}")
        return payload


def main() -> int:
    marker = "<!-- punk-pr-intake-gate -->"
    assert path_matches("README.md", "README.md")
    assert path_matches("docs/usage.md", "docs/**/*.md")
    assert path_matches("docs/product/START-HERE.md", "docs/product/**")
    assert path_matches(".github/workflows/pr-intake-gate.yml", ".github/**")
    assert not path_matches("src/runtime.md", "*.md")
    assert not is_gate_comment({"body": marker, "user": {"login": "contributor", "type": "User"}}, marker)
    assert is_gate_comment({"body": marker, "user": {"login": "github-actions[bot]", "type": "Bot"}}, marker)
    config = load_minimal_yaml(str(ROOT / ".github" / "pr-intake-gate.yml"))
    assert get_label_details(config, "intake/pass")["color"] == "2ea44f"
    assert get_label_details(config, "intake/high-risk")["description"]
    assert not missing_required_sections(FULL_EXTERNAL_BODY, config["external_context"]["required_sections"])
    assert "No-code alternative" in missing_required_sections(MISSING_NO_CODE_BODY, config["external_context"]["required_sections"])
    print("ok - helper semantics")

    trusted_permission = run_case(
        "trusted_permission_passes_high_risk",
        0,
        "pass",
        [{"filename": ".github/workflows/docs-governance.yml", "additions": 1, "deletions": 0}],
        association="CONTRIBUTOR",
        author_permission="admin",
    )
    assert trusted_permission["trusted_author"] is True
    assert trusted_permission["trust_source"] == "permission:admin"

    trusted_fallback = run_case(
        "trusted_association_fallback_passes_high_risk",
        0,
        "pass",
        [{"filename": ".github/workflows/docs-governance.yml", "additions": 1, "deletions": 0}],
        association="OWNER",
        author_permission="none",
    )
    assert trusted_fallback["trusted_author"] is True
    assert trusted_fallback["trust_source"] == "author_association:OWNER"

    run_case(
        "external_docs_only_passes",
        0,
        "pass",
        [{"filename": "docs/usage.md", "additions": 2, "deletions": 1}],
    )
    run_case(
        "external_high_risk_fails",
        1,
        "high-risk",
        [{"filename": ".github/workflows/docs-governance.yml", "additions": 1, "deletions": 0}],
    )
    first_time = run_case(
        "first_time_external_high_risk_fails_with_signal",
        1,
        "high-risk",
        [{"filename": ".github/workflows/docs-governance.yml", "additions": 1, "deletions": 0}],
        association="FIRST_TIMER",
    )
    assert first_time["first_time_external"] is True
    run_case(
        "external_non_trivial_missing_no_code_fails",
        1,
        "no-code-alternative",
        [{"filename": "docs/usage.md", "additions": 31, "deletions": 0}],
        body=MISSING_NO_CODE_BODY,
    )
    run_case(
        "external_non_trivial_missing_context_fails",
        1,
        "needs-more-context",
        [{"filename": "docs/usage.md", "additions": 31, "deletions": 0}],
        body=MISSING_CONTEXT_BODY,
    )
    run_case(
        "external_full_context_without_link_fails",
        1,
        "needs-linked-intent",
        [{"filename": "docs/usage.md", "additions": 31, "deletions": 0}],
        body=FULL_CONTEXT_NO_LINK_BODY,
    )
    run_case(
        "external_full_context_with_link_passes",
        0,
        "pass",
        [{"filename": "docs/usage.md", "additions": 31, "deletions": 0}],
        body=FULL_EXTERNAL_BODY,
    )
    run_case(
        "accepted_external_non_high_risk_passes",
        0,
        "pass",
        [{"filename": "docs/usage.md", "additions": 31, "deletions": 0}],
        labels=["intake/accepted-for-pr"],
    )
    run_case(
        "override_passes_external_high_risk",
        0,
        "pass",
        [{"filename": ".github/workflows/docs-governance.yml", "additions": 1, "deletions": 0}],
        labels=["maintainer/override-intake"],
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
