#!/usr/bin/env python3
"""Minimal deterministic docs-governance checker.

Scope:
- touched/new canonical docs under docs/product/ and docs/archive/
- relative markdown link checks
- DocImpact presence via report files for meaningful changes
- simple superseded/current-ref checks
- simple parked-as-active wording checks

This is internal tooling, not a public CLI contract.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

REQUIRED_DOC_KEYS = {
    "id",
    "kind",
    "status",
    "authority",
    "owner",
    "created_at",
    "updated_at",
}
ALLOWED_DOC_STATUS = {"draft", "active", "accepted", "superseded", "archived", "retired"}
ALLOWED_DOC_AUTHORITY = {"canonical", "historical", "advisory"}
ALLOWED_DOC_IMPACT_CLASSIFICATION = {
    "none",
    "docs-only",
    "code-doc",
    "architecture",
    "dependency",
    "public-claim",
    "research-promotion",
}
CANONICAL_DOC_PREFIXES = ("docs/product/", "docs/archive/")
MEANINGFUL_CHANGE_PREFIXES = (
    "docs/product/",
    "docs/archive/",
    "docs/_schema/",
    "docs/adr/",
    "public/",
    "crates/",
    "evals/specs/",
)
PARKED_CAPABILITY_TERMS = {
    "module-host": ["module host"],
    "provider-adapters": ["provider adapter", "provider adapters"],
    "pubpunk-publishing": ["pubpunk publishing automation", "pubpunk automation"],
    "plugin-marketplace": ["plugin marketplace"],
    "saas-workspace": ["saas workspace"],
    "cloud-sync": ["cloud sync"],
    "ui-first-workflow": ["ui-first workflow"],
}
ACTIVE_PHRASES = (
    "current default operator path",
    "default operator path",
    "active by default",
    "part of the active surface",
    "current operator path",
)
NEGATING_PHRASES = (
    "do not",
    "not active",
    "not the default",
    "not part of",
    "remains parked",
    "parked",
)
MARKDOWN_LINK_RE = re.compile(r"!?\[[^\]]*\]\(([^)]+)\)")
DOC_PATH_RE = re.compile(r"(?<![A-Za-z0-9_])((?:docs|knowledge|public)/[^\s`)]*\.md(?:#[^\s`)]+)?)")
HEADING_RE = re.compile(r"^#{1,6}\s+(.*)$", re.MULTILINE)
STATUS_LINE_RE = re.compile(r"^Status:\s*(.+)$", re.MULTILINE | re.IGNORECASE)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Run minimal deterministic docs-governance checks.")
    parser.add_argument("--repo", default=".", help="Repository root. Defaults to current directory.")
    parser.add_argument("--base", help="Base git ref for changed-file detection.")
    parser.add_argument("--head", default="HEAD", help="Head git ref for changed-file detection. Defaults to HEAD.")
    parser.add_argument("--staged", action="store_true", help="Use staged changes from git diff --cached.")
    parser.add_argument("--files", nargs="*", default=[], help="Explicit changed files, relative to repo root.")
    parser.add_argument("--report", action="append", default=[], help="Explicit report path(s) to inspect for DocImpact.")
    parser.add_argument("--json", action="store_true", help="Print machine-readable JSON to stdout instead of the human summary.")
    parser.add_argument("--json-out", help="Write machine-readable JSON to a file.")
    parser.add_argument("--summary-out", help="Write the human summary to a file.")
    return parser.parse_args()


def run_git(repo: Path, args: list[str]) -> str:
    return subprocess.check_output(["git", "-C", str(repo), *args], text=True).strip()


def get_changed_files(repo: Path, args: argparse.Namespace) -> list[str]:
    if args.files:
        return sorted({normalize_rel_path(item) for item in args.files if normalize_rel_path(item)})
    if args.staged:
        out = run_git(repo, ["diff", "--cached", "--name-only", "--diff-filter=ACMR"])
        return sorted(filter(None, out.splitlines()))
    if args.base:
        out = run_git(repo, ["diff", "--name-only", "--diff-filter=ACMR", f"{args.base}..{args.head}"])
        return sorted(filter(None, out.splitlines()))
    try:
        out = run_git(repo, ["diff", "--name-only", "--diff-filter=ACMR", "HEAD~1", args.head])
    except subprocess.CalledProcessError:
        out = ""
    return sorted(filter(None, out.splitlines()))


def normalize_rel_path(value: str) -> str:
    if not value:
        return ""
    return str(Path(value).as_posix()).lstrip("./")


def load_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def parse_frontmatter(text: str) -> dict[str, object] | None:
    if not text.startswith("---\n"):
        return None
    end = text.find("\n---\n", 4)
    if end == -1:
        return None
    block = text[4:end]
    data: dict[str, object] = {}
    current_key: str | None = None
    for raw in block.splitlines():
        if not raw.strip():
            continue
        if raw.startswith("  - ") and current_key:
            if not isinstance(data.get(current_key), list):
                data[current_key] = []
            data[current_key].append(parse_scalar(raw[4:].strip()))
            continue
        match = re.match(r"^([A-Za-z0-9_]+):\s*(.*)$", raw)
        if not match:
            current_key = None
            continue
        key, value = match.group(1), match.group(2)
        if value == "":
            data[key] = []
            current_key = key
        else:
            data[key] = parse_scalar(value.strip())
            current_key = key if isinstance(data[key], list) else None
    return data
def parse_scalar(value: str) -> object:
    value = value.strip()
    if value in {"null", "Null", "NULL"}:
        return None
    if (value.startswith('"') and value.endswith('"')) or (value.startswith("'") and value.endswith("'")):
        return value[1:-1]
    return value


def parse_doc_status(text: str) -> str | None:
    frontmatter = parse_frontmatter(text)
    if frontmatter and isinstance(frontmatter.get("status"), str):
        return str(frontmatter["status"]).strip().lower()
    match = STATUS_LINE_RE.search(text)
    if match:
        return match.group(1).strip().lower()
    return None


def extract_markdown_links(text: str) -> list[str]:
    return [target.strip() for target in MARKDOWN_LINK_RE.findall(text)]


def markdown_slug(title: str) -> str:
    slug = title.strip().lower()
    slug = re.sub(r"[`*_~]+", "", slug)
    slug = re.sub(r"[^a-z0-9\-\s]", "", slug)
    slug = re.sub(r"\s+", "-", slug)
    slug = re.sub(r"-+", "-", slug).strip("-")
    return slug


def extract_heading_slugs(text: str) -> set[str]:
    return {markdown_slug(match.group(1)) for match in HEADING_RE.finditer(text) if markdown_slug(match.group(1))}


def is_relative_doc_link(target: str) -> bool:
    if not target or target.startswith(("http://", "https://", "mailto:", "#")):
        return False
    return not Path(target.split("#", 1)[0]).is_absolute()


def resolve_relative_target(source_rel: str, target: str) -> tuple[str, str | None]:
    file_part, anchor = (target.split("#", 1) + [None])[:2]
    resolved = (Path(source_rel).parent / file_part).resolve()
    return str(resolved), anchor


def collect_doc_refs(text: str) -> set[str]:
    refs: set[str] = set()
    for target in extract_markdown_links(text):
        if target.startswith(("http://", "https://", "mailto:")):
            continue
        refs.add(target)
    for match in DOC_PATH_RE.finditer(text):
        refs.add(match.group(1))
    return refs


def parse_doc_impact_from_report(text: str) -> dict[str, object] | None:
    section = re.search(r"^##\s+Doc impact\s*```yaml\n(.*?)```", text, re.IGNORECASE | re.MULTILINE | re.DOTALL)
    if not section:
        return None
    block = section.group(1)
    classification = re.search(r"^\s{2}classification:\s*(.+)$", block, re.MULTILINE)
    reason = re.search(r"^\s{2}reason:\s*(.+)$", block, re.MULTILINE)
    if not classification or not reason:
        return None
    return {
        "classification": parse_scalar(classification.group(1).strip()),
        "reason": parse_scalar(reason.group(1).strip()),
    }


def is_canonical_doc(rel_path: str) -> bool:
    return rel_path.endswith(".md") and rel_path.startswith(CANONICAL_DOC_PREFIXES)


def is_meaningful_change(rel_path: str) -> bool:
    return rel_path.startswith(MEANINGFUL_CHANGE_PREFIXES)


def add_issue(issues: list[dict[str, object]], severity: str, code: str, path: str, message: str, **extra: object) -> None:
    issue = {
        "severity": severity,
        "code": code,
        "path": path,
        "message": message,
    }
    issue.update(extra)
    issues.append(issue)


def check_frontmatter(rel_path: str, text: str, issues: list[dict[str, object]]) -> None:
    frontmatter = parse_frontmatter(text)
    if not frontmatter:
        add_issue(issues, "failure", "DOC_FRONTMATTER_MISSING", rel_path, "Touched canonical doc is missing frontmatter.")
        return
    missing = sorted(REQUIRED_DOC_KEYS - set(frontmatter.keys()))
    for key in missing:
        add_issue(
            issues,
            "failure",
            "DOC_FRONTMATTER_MISSING_REQUIRED_KEY",
            rel_path,
            f"Touched canonical doc is missing required frontmatter key: {key}.",
            key=key,
        )
    status = frontmatter.get("status")
    if isinstance(status, str) and status not in ALLOWED_DOC_STATUS:
        add_issue(issues, "failure", "DOC_FRONTMATTER_INVALID_STATUS", rel_path, f"Invalid doc status: {status}.", status=status)
    authority = frontmatter.get("authority")
    if isinstance(authority, str) and authority not in ALLOWED_DOC_AUTHORITY:
        add_issue(
            issues,
            "failure",
            "DOC_FRONTMATTER_INVALID_AUTHORITY",
            rel_path,
            f"Invalid doc authority: {authority}.",
            authority=authority,
        )


def check_links(repo: Path, rel_path: str, text: str, issues: list[dict[str, object]]) -> None:
    for target in extract_markdown_links(text):
        if not is_relative_doc_link(target):
            continue
        resolved_str, anchor = resolve_relative_target(rel_path, target)
        resolved = Path(resolved_str)
        if not resolved.exists():
            add_issue(
                issues,
                "failure",
                "DOC_BROKEN_LINK",
                rel_path,
                f"Relative link target does not exist: {target}.",
                target=normalize_rel_path(str(resolved.relative_to(repo))) if resolved.is_relative_to(repo) else target,
            )
            continue
        if anchor:
            slugs = extract_heading_slugs(load_text(resolved))
            if markdown_slug(anchor) not in slugs:
                add_issue(
                    issues,
                    "failure",
                    "DOC_BROKEN_ANCHOR",
                    rel_path,
                    f"Relative link anchor does not exist: {target}.",
                    target=target,
                )


def check_superseded_refs(repo: Path, rel_path: str, text: str, issues: list[dict[str, object]]) -> None:
    if rel_path.startswith("docs/archive/"):
        return
    for ref in sorted(collect_doc_refs(text)):
        target_part = ref.split("#", 1)[0]
        if not target_part.startswith(("docs/", "knowledge/", "public/")):
            continue
        target_path = (repo / target_part).resolve()
        if not target_path.exists() or not target_path.is_file():
            continue
        status = parse_doc_status(load_text(target_path))
        if status in {"superseded", "archived", "retired"}:
            add_issue(
                issues,
                "failure",
                "DOC_SUPERSEDED_DOC_REFERENCED_AS_CURRENT",
                rel_path,
                f"Current doc references a non-current target: {target_part} ({status}).",
                target=target_part,
                target_status=status,
            )


def check_parked_as_active(rel_path: str, text: str, issues: list[dict[str, object]]) -> None:
    for raw_line in text.splitlines():
        line = raw_line.strip().lower()
        if not line:
            continue
        if not any(phrase in line for phrase in ACTIVE_PHRASES):
            continue
        if any(phrase in line for phrase in NEGATING_PHRASES):
            continue
        for capability, terms in PARKED_CAPABILITY_TERMS.items():
            if any(term in line for term in terms):
                add_issue(
                    issues,
                    "failure",
                    "DOC_PARKED_CAPABILITY_DESCRIBED_AS_ACTIVE",
                    rel_path,
                    f"Parked or future capability is described as active: {capability}.",
                    capability=capability,
                    line=raw_line.strip(),
                )


def check_doc_impact(repo: Path, changed_files: list[str], report_paths: list[str], issues: list[dict[str, object]]) -> None:
    meaningful = [path for path in changed_files if is_meaningful_change(path)]
    if not meaningful:
        return
    if not report_paths:
        add_issue(
            issues,
            "failure",
            "DOC_IMPACT_REQUIRED",
            meaningful[0],
            "Meaningful change requires a report DocImpact block.",
        )
        return
    parsed_any = False
    for rel_path in report_paths:
        report_file = repo / rel_path
        if not report_file.exists():
            add_issue(issues, "failure", "DOC_IMPACT_REPORT_MISSING", rel_path, "Configured report path does not exist.")
            continue
        impact = parse_doc_impact_from_report(load_text(report_file))
        if not impact:
            add_issue(issues, "failure", "DOC_IMPACT_BLOCK_MISSING", rel_path, "Report does not contain a parseable DocImpact block.")
            continue
        parsed_any = True
        classification = impact.get("classification")
        reason = impact.get("reason")
        if classification not in ALLOWED_DOC_IMPACT_CLASSIFICATION:
            add_issue(
                issues,
                "failure",
                "DOC_IMPACT_INVALID_CLASSIFICATION",
                rel_path,
                f"DocImpact classification is invalid: {classification}.",
                classification=classification,
            )
        if not isinstance(reason, str) or not reason.strip():
            add_issue(issues, "failure", "DOC_IMPACT_MISSING_REASON", rel_path, "DocImpact reason is required.")
    if report_paths and not parsed_any:
        return


def human_summary(result: dict[str, object]) -> str:
    lines = [
        f"Docs governance check: {str(result['status']).upper()}",
        f"Changed files: {len(result['changed_files'])}",
        f"Canonical docs checked: {len(result['canonical_docs_checked'])}",
        f"Reports checked: {len(result['report_paths'])}",
        f"Failures: {len(result['failures'])}",
        f"Warnings: {len(result['warnings'])}",
    ]
    failures = result["failures"]
    warnings = result["warnings"]
    if failures:
        lines.append("")
        lines.append("Failures:")
        for issue in failures:
            lines.append(f"- [{issue['code']}] {issue['path']}: {issue['message']}")
    if warnings:
        lines.append("")
        lines.append("Warnings:")
        for issue in warnings:
            lines.append(f"- [{issue['code']}] {issue['path']}: {issue['message']}")
    return "\n".join(lines)


def main() -> int:
    args = parse_args()
    repo = Path(args.repo).resolve()
    changed_files = get_changed_files(repo, args)
    report_paths = sorted({normalize_rel_path(p) for p in args.report} | {p for p in changed_files if p.startswith("work/reports/") and p.endswith(".md")})
    issues: list[dict[str, object]] = []

    canonical_docs_checked: list[str] = []
    for rel_path in changed_files:
        if not is_canonical_doc(rel_path):
            continue
        doc_path = repo / rel_path
        if not doc_path.exists():
            continue
        canonical_docs_checked.append(rel_path)
        text = load_text(doc_path)
        check_frontmatter(rel_path, text, issues)
        check_links(repo, rel_path, text, issues)
        check_superseded_refs(repo, rel_path, text, issues)
        check_parked_as_active(rel_path, text, issues)

    check_doc_impact(repo, changed_files, report_paths, issues)

    failures = [issue for issue in issues if issue["severity"] == "failure"]
    warnings = [issue for issue in issues if issue["severity"] == "warning"]
    result = {
        "status": "pass" if not failures else "fail",
        "repo": str(repo),
        "changed_files": changed_files,
        "canonical_docs_checked": canonical_docs_checked,
        "report_paths": report_paths,
        "meaningful_change_paths": [path for path in changed_files if is_meaningful_change(path)],
        "failures": failures,
        "warnings": warnings,
    }
    json_text = json.dumps(result, indent=2, ensure_ascii=False)
    summary_text = human_summary(result)

    if args.json_out:
        Path(args.json_out).write_text(json_text + "\n", encoding="utf-8")
    if args.summary_out:
        Path(args.summary_out).write_text(summary_text + "\n", encoding="utf-8")

    if args.json:
        print(json_text)
    else:
        print(summary_text)
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
