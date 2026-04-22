#!/usr/bin/env python3
from __future__ import annotations

import re
from pathlib import Path


REPO = Path(__file__).resolve().parent.parent
STATUS_PATH = REPO / "work" / "STATUS.md"
GOALS_DIR = REPO / "work" / "goals"
GOAL_TEMPLATE = REPO / "work" / "_templates" / "goal.md"
ALLOWED = {"R0", "R1", "R2", "R3"}
REQUIRED_KEYS = {
    "classification",
    "required",
    "rationale",
    "research_refs",
    "external_research_refs",
    "blocked_reason",
}


def frontmatter_block(path: Path) -> str:
    text = path.read_text(encoding="utf-8")
    match = re.match(r"^---\n(.*?)\n---\n", text, re.DOTALL)
    if not match:
        raise ValueError(f"{path.relative_to(REPO)} is missing frontmatter")
    return match.group(1)


def parse_status_selected_next() -> str:
    frontmatter = frontmatter_block(STATUS_PATH)
    match = re.search(r"^selected_next:\s*['\"]?([^'\"\n]+)['\"]?$", frontmatter, re.MULTILINE)
    if not match:
        raise ValueError("work/STATUS.md.selected_next is missing")
    return match.group(1).strip()


def parse_scalar(raw: str) -> object:
    value = raw.strip()
    if value in {"true", "True", "TRUE"}:
        return True
    if value in {"false", "False", "FALSE"}:
        return False
    if value in {"null", "Null", "NULL", ""}:
        return None
    if (value.startswith('"') and value.endswith('"')) or (value.startswith("'") and value.endswith("'")):
        return value[1:-1]
    return value


def parse_listish(raw: str) -> list[str]:
    value = raw.strip()
    if value in {"", "[]"}:
        return []
    if value.startswith("[") and value.endswith("]"):
        inner = value[1:-1].strip()
        if not inner:
            return []
        items: list[str] = []
        for part in inner.split(","):
            parsed = parse_scalar(part)
            if isinstance(parsed, str) and parsed:
                items.append(parsed)
        return items
    parsed = parse_scalar(value)
    return [parsed] if isinstance(parsed, str) and parsed else []


def parse_research_gate(path: Path) -> dict[str, object] | None:
    block = frontmatter_block(path)
    lines = block.splitlines()
    in_gate = False
    gate_lines: list[str] = []

    for line in lines:
        if not in_gate:
            if line.strip() == "research_gate:":
                in_gate = True
            continue
        if re.match(r"^[A-Za-z0-9_]+:", line):
            break
        if line.startswith("  ") or not line.strip():
            gate_lines.append(line)

    if not gate_lines:
        return None

    data: dict[str, object] = {}
    current_list: str | None = None

    for line in gate_lines:
        if not line.strip():
            continue
        list_item = re.match(r"^\s+-\s+(.*)$", line)
        if list_item and current_list:
            items = data.setdefault(current_list, [])
            assert isinstance(items, list)
            parsed = parse_scalar(list_item.group(1))
            if isinstance(parsed, str) and parsed:
                items.append(parsed)
            continue

        field = re.match(r"^\s{2}([A-Za-z0-9_]+):(?:\s*(.*))?$", line)
        if not field:
            continue
        key = field.group(1)
        raw = (field.group(2) or "").strip()
        current_list = None
        if key in {"research_refs", "external_research_refs"}:
            data[key] = parse_listish(raw)
            if raw == "":
                current_list = key
        else:
            data[key] = parse_scalar(raw)

    return data


def is_repo_ref(ref: str) -> bool:
    if not ref or ref.startswith("/"):
        return False
    parts = [part for part in ref.split("/") if part not in {"", "."}]
    return ".." not in parts


def validate_goal(path: Path, issues: list[str], *, selected: bool = False, current: bool = False) -> None:
    gate = parse_research_gate(path)
    label = path.relative_to(REPO)

    if gate is None:
        issues.append(f"{label} is missing research_gate")
        return

    missing = REQUIRED_KEYS - set(gate)
    if missing:
        issues.append(f"{label} research_gate is missing keys: {', '.join(sorted(missing))}")
        return

    classification = gate.get("classification")
    if classification not in ALLOWED:
        issues.append(f"{label} has invalid research_gate.classification: {classification!r}")

    required = gate.get("required")
    if not isinstance(required, bool):
        issues.append(f"{label} has non-boolean research_gate.required: {required!r}")

    rationale = gate.get("rationale")
    if not isinstance(rationale, str) or ((selected or current) and not rationale.strip()):
        issues.append(f"{label} must have non-empty research_gate.rationale for selected/current work")

    for key in ("research_refs", "external_research_refs"):
        refs = gate.get(key)
        if not isinstance(refs, list):
            issues.append(f"{label} has non-list research_gate.{key}")
            continue
        for ref in refs:
            if not isinstance(ref, str) or not ref:
                issues.append(f"{label} has empty research_gate.{key} ref")
                continue
            if not is_repo_ref(ref):
                issues.append(f"{label} has invalid repo-relative research_gate.{key} ref: {ref}")
                continue
            if not (REPO / ref).exists():
                issues.append(f"{label} references missing research artifact in research_gate.{key}: {ref}")

    blocked_reason = gate.get("blocked_reason")
    if blocked_reason is not None and not isinstance(blocked_reason, str):
        issues.append(f"{label} has non-string research_gate.blocked_reason")

    if current and classification == "R0" and required is True:
        issues.append(f"{label} cannot require research_gate with classification R0")


def main() -> int:
    issues: list[str] = []

    if not GOAL_TEMPLATE.exists():
        issues.append("work/_templates/goal.md is missing")
    else:
        validate_goal(GOAL_TEMPLATE, issues)

    selected_goal: Path | None = None
    if not STATUS_PATH.exists():
        issues.append("work/STATUS.md is missing")
    else:
        try:
            selected_next = parse_status_selected_next()
        except ValueError as exc:
            issues.append(str(exc))
            selected_next = None
        if selected_next:
            candidate = REPO / selected_next
            if not candidate.exists():
                issues.append(f"selected_next points to missing goal: {selected_next}")
            else:
                selected_goal = candidate
                validate_goal(candidate, issues, selected=True, current=True)

    for goal_path in sorted(GOALS_DIR.glob("*.md")):
        try:
            frontmatter = frontmatter_block(goal_path)
        except ValueError as exc:
            issues.append(str(exc))
            continue
        match = re.search(r"^status:\s*([^\n]+)$", frontmatter, re.MULTILINE)
        if not match:
            continue
        status = match.group(1).strip().strip('"').strip("'")
        if status == "in_progress":
            validate_goal(goal_path, issues, current=True)

    if issues:
        print("Research Gate check: FAIL")
        for issue in issues:
            print(f"- {issue}")
        return 1

    print("Research Gate check: PASS")
    if selected_goal is not None:
        print(f"Selected next: {selected_goal.relative_to(REPO)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
