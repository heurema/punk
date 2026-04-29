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

from scripts.pr_intake_gate import get_label_details, is_gate_comment, load_minimal_yaml, path_matches  # noqa: E402


def write_event(path: Path, body: str, labels: list[str]) -> None:
    event = {
        "repository": {"full_name": "heurema/punk"},
        "pull_request": {
            "number": 123,
            "title": "Test PR",
            "body": body,
            "author_association": "CONTRIBUTOR",
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
) -> None:
    labels = labels or []
    with tempfile.TemporaryDirectory(prefix=f"punk-pr-intake-{name}-") as tmp_raw:
        tmp = Path(tmp_raw)
        event_path = tmp / "event.json"
        stdout_path = tmp / "stdout.json"
        summary_path = tmp / "summary.md"
        write_event(event_path, body, labels)

        env = os.environ.copy()
        env.update(
            {
                "GITHUB_EVENT_PATH": str(event_path),
                "GITHUB_STEP_SUMMARY": str(summary_path),
                "PR_INTAKE_GATE_CHANGED_FILES_JSON": json.dumps(files),
                "PR_INTAKE_GATE_DRY_RUN": "1",
            }
        )
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
    print("ok - helper semantics")

    run_case(
        "docs_only_passes",
        0,
        "pass",
        [{"filename": "docs/usage.md", "additions": 2, "deletions": 1}],
    )
    run_case(
        "non_trivial_without_intent_fails",
        1,
        "needs-linked-intent",
        [{"filename": "docs/usage.md", "additions": 31, "deletions": 0}],
    )
    run_case(
        "linked_non_trivial_passes_with_work_goal",
        0,
        "pass",
        [{"filename": "docs/usage.md", "additions": 31, "deletions": 0}],
        body="Linked goal: work/goals/goal_example.md",
    )
    run_case(
        "product_doc_is_high_risk",
        1,
        "high-risk",
        [{"filename": "docs/product/START-HERE.md", "additions": 1, "deletions": 0}],
    )
    run_case(
        "workflow_is_high_risk",
        1,
        "high-risk",
        [{"filename": ".github/workflows/docs-governance.yml", "additions": 1, "deletions": 0}],
    )
    run_case(
        "override_passes_high_risk",
        0,
        "pass",
        [{"filename": ".github/workflows/docs-governance.yml", "additions": 1, "deletions": 0}],
        labels=["maintainer/override-intake"],
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
