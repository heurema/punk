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
  pubpunk-hook-tuning-fixture)
    exec python3 scripts/check_module_control_markers.py pubpunk-hook-tuning-fixture "$@"
    ;;
  module-control-plane)
    exec python3 scripts/check_module_control_markers.py module-control-plane "$@"
    ;;
  module-behavior-boundaries)
    exec python3 scripts/check_module_control_markers.py module-behavior-boundaries "$@"
    ;;
  module-control-validation)
    exec python3 scripts/check_module_control_markers.py module-control-validation "$@"
    ;;
  module-tuning-handoff-template)
    exec python3 scripts/check_module_control_markers.py module-tuning-handoff-template "$@"
    ;;
  pubpunk-control-manifest)
    exec python3 scripts/check_module_control_markers.py pubpunk-control-manifest "$@"
    ;;
  pubpunk-live-tuning-pack)
    exec python3 scripts/check_module_control_markers.py pubpunk-live-tuning-pack "$@"
    ;;
  module-control-suite)
    exec python3 scripts/check_module_control_markers.py module-control-suite "$@"
    ;;
  *)
    echo "Unknown check target: $command" >&2
    echo "Usage:" >&2
    echo "  scripts/check.sh" >&2
    echo "  scripts/check.sh docs-governance [--staged|--base <ref> --head <ref>|--files ...|--report ...]" >&2
    echo "  scripts/check.sh pubpunk-hook-tuning-fixture" >&2
    echo "  scripts/check.sh module-control-plane" >&2
    echo "  scripts/check.sh module-behavior-boundaries" >&2
    echo "  scripts/check.sh module-control-validation" >&2
    echo "  scripts/check.sh module-tuning-handoff-template" >&2
    echo "  scripts/check.sh pubpunk-control-manifest" >&2
    echo "  scripts/check.sh pubpunk-live-tuning-pack" >&2
    echo "  scripts/check.sh module-control-suite" >&2
    exit 2
    ;;
esac
