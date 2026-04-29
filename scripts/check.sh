#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

if [[ $# -eq 0 ]]; then
  exec python3 scripts/check_docs_governance.py --repo "$repo_root" --staged
fi

command="$1"
shift

case "$command" in
  docs-governance)
    if [[ $# -eq 0 ]]; then
      exec python3 scripts/check_docs_governance.py --repo "$repo_root" --staged
    fi
    exec python3 scripts/check_docs_governance.py --repo "$repo_root" "$@"
    ;;
  pr-intake-gate)
    exec python3 scripts/test_pr_intake_gate.py
    ;;
  *)
    echo "Unknown check target: $command" >&2
    echo "Usage:" >&2
    echo "  scripts/check.sh" >&2
    echo "  scripts/check.sh docs-governance [--staged|--base <ref> --head <ref>|--files ...|--report ...]" >&2
    echo "  scripts/check.sh pr-intake-gate" >&2
    exit 2
    ;;
esac
