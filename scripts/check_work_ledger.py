#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from pathlib import Path


REPO = Path(__file__).resolve().parent.parent
STATUS_PATH = REPO / "work" / "STATUS.md"
GOALS_DIR = REPO / "work" / "goals"
ALLOWED_STATUSES = {"draft", "ready", "in_progress", "blocked", "done", "superseded"}
REQUIRED_STATUS_SECTIONS = [
    "## Now",
    "## Next Candidates",
    "## Blocked",
    "## Done Recently",
    "## Validation",
]
LIST_KEYS = {"blocked_by", "knowledge_refs", "contract_refs", "report_refs", "decision_refs", "proof_refs", "supersedes"}
SCALAR_KEYS = {"status", "selected_next", "latest_proof_ref", "superseded_by"}
REF_LIST_KEYS = {"blocked_by", "knowledge_refs", "contract_refs", "report_refs", "decision_refs", "proof_refs", "supersedes"}
REF_SCALAR_KEYS = {"selected_next", "latest_proof_ref", "superseded_by"}


def parse_scalar(raw: str) -> object:
    value = raw.strip()
    if value in {"null", "Null", "NULL"}:
        return None
    if value in {"[]", ""}:
        return []
    if (value.startswith('"') and value.endswith('"')) or (value.startswith("'") and value.endswith("'")):
        return value[1:-1]
    if value.startswith("[") and value.endswith("]"):
        inner = value[1:-1].strip()
        if not inner:
            return []
        items = []
        for part in inner.split(","):
            part = part.strip()
            if (part.startswith('"') and part.endswith('"')) or (part.startswith("'") and part.endswith("'")):
                part = part[1:-1]
            items.append(part)
        return items
    return value


def load_frontmatter(path: Path) -> dict[str, object]:
    text = path.read_text(encoding="utf-8")
    match = re.match(r"^---\n(.*?)\n---\n", text, re.DOTALL)
    if not match:
        raise ValueError(f"{path.relative_to(REPO)} is missing frontmatter")

    frontmatter = match.group(1).splitlines()
    data: dict[str, object] = {}
    current_list_key: str | None = None

    for line in frontmatter:
        if not line.strip():
            continue
        top = re.match(r"^([A-Za-z0-9_]+):(?:\s*(.*))?$", line)
        if top:
            key = top.group(1)
            raw_value = top.group(2) or ""
            current_list_key = None
            if key in LIST_KEYS:
                value = parse_scalar(raw_value)
                if isinstance(value, list):
                    data[key] = value
                elif value is None:
                    data[key] = []
                else:
                    data[key] = [str(value)]
                if raw_value == "":
                    current_list_key = key
            elif key in SCALAR_KEYS:
                data[key] = parse_scalar(raw_value)
            continue

        if current_list_key and line.startswith("  - "):
            item = parse_scalar(line[4:])
            items = data.setdefault(current_list_key, [])
            if isinstance(items, list):
                items.append("" if item is None else str(item))

    return data


def is_valid_repo_ref(ref: str) -> bool:
    if not ref:
        return True
    if ref.startswith("/"):
        return False
    parts = [part for part in ref.split("/") if part not in {"", "."}]
    return ".." not in parts


def validate_repo_ref(ref: str, label: str, issues: list[str]) -> Path | None:
    if not is_valid_repo_ref(ref):
        issues.append(f"{label} must be repo-relative and must not contain '..': {ref}")
        return None
    return REPO / ref


def require_sections(text: str, issues: list[str]) -> None:
    for heading in REQUIRED_STATUS_SECTIONS:
        if heading not in text:
            issues.append(f"Missing required section in work/STATUS.md: {heading}")


def validate_status_file(issues: list[str]) -> str | None:
    if not STATUS_PATH.exists():
        issues.append("work/STATUS.md does not exist")
        return None

    text = STATUS_PATH.read_text(encoding="utf-8")
    require_sections(text, issues)

    try:
        frontmatter = load_frontmatter(STATUS_PATH)
    except ValueError as exc:
        issues.append(str(exc))
        return None

    selected_next = frontmatter.get("selected_next")
    if not isinstance(selected_next, str) or not selected_next:
        issues.append("work/STATUS.md.selected_next is missing or empty")
        return None

    selected_path = validate_repo_ref(selected_next, "work/STATUS.md.selected_next", issues)
    if selected_path is None:
        return None
    if not selected_path.exists():
        issues.append(f"work/STATUS.md.selected_next points to a missing goal: {selected_next}")
        return None

    return selected_next


def validate_goals(selected_next: str | None, issues: list[str]) -> None:
    in_progress_count = 0

    for goal_path in sorted(GOALS_DIR.glob("*.md")):
        frontmatter = load_frontmatter(goal_path)
        status = frontmatter.get("status")

        if not isinstance(status, str) or status not in ALLOWED_STATUSES:
            issues.append(f"{goal_path.relative_to(REPO)} has invalid status: {status!r}")
            continue

        if status == "in_progress":
            in_progress_count += 1

        for key in REF_LIST_KEYS:
            value = frontmatter.get(key, [])
            if value is None:
                continue
            if not isinstance(value, list):
                issues.append(f"{goal_path.relative_to(REPO)} has non-list {key}")
                continue
            for ref in value:
                if not ref:
                    continue
                ref_path = validate_repo_ref(str(ref), f"{goal_path.relative_to(REPO)} {key}", issues)
                if ref_path is None:
                    continue
                if key in {"report_refs", "decision_refs"} and not ref_path.exists():
                    issues.append(f"{goal_path.relative_to(REPO)} references missing {key[:-5]} artifact: {ref}")

        for key in REF_SCALAR_KEYS:
            value = frontmatter.get(key)
            if value in (None, ""):
                continue
            if not isinstance(value, str):
                issues.append(f"{goal_path.relative_to(REPO)} has non-string {key}")
                continue
            validate_repo_ref(value, f"{goal_path.relative_to(REPO)} {key}", issues)

        if selected_next and str(goal_path.relative_to(REPO)) == selected_next and status != "ready":
            issues.append(f"selected_next goal must be ready, got {status!r} in {selected_next}")

    if in_progress_count > 1:
        issues.append(f"More than one goal is in_progress: {in_progress_count}")


def main() -> int:
    issues: list[str] = []
    selected_next = validate_status_file(issues)
    validate_goals(selected_next, issues)

    if issues:
        print("Work ledger check: FAIL")
        for issue in issues:
            print(f"- {issue}")
        return 1

    print("Work ledger check: PASS")
    print(f"Selected next: {selected_next}")
    print(f"Goals checked: {len(list(GOALS_DIR.glob('*.md')))}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
