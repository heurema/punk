#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from pathlib import Path


REPO = Path(__file__).resolve().parent.parent


def p(rel_path: str) -> Path:
    return REPO / rel_path


FORBIDDEN_BASE = {
    "auto apply enabled": "auto_apply: true",
    "approval auto apply enabled": "approval_is_auto_apply: true",
    "runtime activated": "runtime_activated: true",
    "manifest parser activated": "manifest_parser_activated: true",
    "overlay resolver activated": "overlay_resolver_activated: true",
    "resolver activated": "resolver_activated: true",
    "writer activated": "writer_activated: true",
    "user local writer activated": "user_local_config_writer_activated: true",
    "reflection scheduler activated": "reflection_scheduler_activated: true",
    "adapter invocation activated": "adapter_invocation_activated: true",
    "publishing activated": "publishing_activated: true",
    "metrics activated": "metrics_collection_activated: true",
    "external research activated": "external_research_activated: true",
    "event writing activated": "event_writing_activated: true",
    "gate writer activated": "gate_writer_activated: true",
    "proofpack writer activated": "proofpack_writer_activated: true",
    "acceptance claim activated": "acceptance_claim_activated: true",
    "selected current behavior": "selected_as_current_behavior: true",
    "secrets present": "contains_secrets: true",
    "private data present": "contains_private_data: true",
    "executable code allowed": "executable_code_allowed: true",
}


def read_required(path: Path, issues: list[str]) -> str:
    if not path.exists():
        issues.append(f"missing file: {path.relative_to(REPO)}")
        return ""
    return path.read_text(encoding="utf-8")


def key_value_marker_pattern(token: str, *, exact_value: bool) -> re.Pattern[str] | None:
    marker = re.match(r"^([A-Za-z0-9_-]+)\s*[:=]\s*(.*)$", token.strip())
    if not marker:
        return None
    key = marker.group(1)
    value = marker.group(2).strip()
    if value:
        value_pattern = r"\s+".join(re.escape(part) for part in re.split(r"\s+", value))
        suffix = r"\s*(?:#.*)?$" if exact_value else r"(?:\s+.*|\s*(?:#.*)?$)"
        value_pattern = rf"\s*{value_pattern}{suffix}"
    else:
        value_pattern = r"\s*[^\n]*(?:#.*)?$"
    return re.compile(
        rf"(?im)^[ \t-]*{re.escape(key)}\s*[:=]{value_pattern}"
    )


def has_marker(text: str, token: str, *, exact_value: bool) -> bool:
    pattern = key_value_marker_pattern(token, exact_value=exact_value)
    if pattern is not None:
        if pattern.search(text) is not None:
            return True
        if not exact_value:
            uncommented = "\n".join(
                line for line in text.splitlines() if not line.lstrip().startswith("#")
            )
            return token in uncommented
        return False
    return token in text


def require_tokens(label: str, text: str, tokens: dict[str, str], issues: list[str]) -> None:
    for name, token in tokens.items():
        if not has_marker(text, token, exact_value=False):
            issues.append(f"{label} missing {name}: {token}")


def reject_tokens(label: str, text: str, tokens: dict[str, str], issues: list[str]) -> None:
    for name, token in tokens.items():
        if has_marker(text, token, exact_value=True):
            issues.append(f"{label} contains forbidden {name}: {token}")


def run_marker_check(
    label: str,
    files: tuple[tuple[str, Path, dict[str, str], dict[str, str]], ...],
    success_lines: tuple[str, ...],
) -> int:
    issues: list[str] = []
    for file_label, path, required_tokens, forbidden_tokens in files:
        text = read_required(path, issues)
        require_tokens(file_label, text, required_tokens, issues)
        reject_tokens(file_label, text, forbidden_tokens, issues)

    if issues:
        print(f"{label}: FAIL")
        for issue in issues:
            print(f"- {issue}")
        return 1

    print(f"{label}: PASS")
    for line in success_lines:
        print(line)
    return 0


MODULE_CONTROL_PLANE_CONTRACT = {
    "core rule": "Reflection may propose improvements. It must not apply them.",
    "manual tuning section": "## Operator-triggered tuning flow",
    "manual tuning request shape": "manual_in_session_tuning_request:",
    "manual tuning proposal output": "requested_output_kind: tuning_proposal",
    "manual tuning no auto apply": "auto_apply: false",
    "manifest section": "## Module control manifest",
    "manifest shape": "module_control_manifest:",
    "behavior artifacts": "behavior_artifacts:",
    "immutable core": "immutable_core:",
    "capability grants empty": "granted: []",
    "capability denies publishing": "external_publish",
    "capability denies credentials": "credential_read",
    "capability denies adapters": "adapter_invoke",
    "reflection trigger none": "trigger: none",
    "scheduled proposal trigger": "scheduled_proposal_only",
    "behavior overlay section": "## Behavior overlay stack",
    "overlay shape": "behavior_overlay_stack:",
    "packaged defaults layer": "packaged_defaults",
    "project workspace layer": "project_workspace",
    "user local layer": "user_local",
    "run local layer": "run_local",
    "overlay no capability grants": "overlays_cannot_grant_capabilities",
    "overlay no side-effect expansion": "overlays_cannot_expand_side_effects",
    "user-local section": "## User-local customization artifacts",
    "user-local shape": "user_local_behavior_artifact:",
    "local-private authority": "authority: local_private",
    "outside repo storage": "storage_policy: local_private_outside_repo",
    "not exported": "exported_to_project: false",
    "not project truth": "project_truth: false",
    "no secrets": "contains_secrets: false",
    "no private data": "contains_private_data: false",
    "no executable code": "executable_code_allowed: false",
    "tuning proposal section": "## Tuning proposal",
    "tuning proposal shape": "tuning_proposal:",
    "capability change status": "capability_change_requested: false",
    "immutable core status": "immutable_core_touched: false",
    "side effect status": "side_effect_policy_changed: false",
    "promotion section": "## Promotion boundary",
    "promotion shape": "behavior_artifact_promotion:",
    "no capability delta": "capability_delta: none",
    "no side-effect delta": "side_effect_delta: none",
    "non authority": "non_authority: true",
    "non goals": "## Non-goals",
    "validator command": "scripts/check.sh module-control-plane",
}

MODULE_CONTROL_PLANE_SPEC = {
    "case 001": "MODULE-CONTROL-001",
    "case 002": "MODULE-CONTROL-002",
    "case 003": "MODULE-CONTROL-003",
    "case 004": "MODULE-CONTROL-004",
    "case 005": "MODULE-CONTROL-005",
    "case 006": "MODULE-CONTROL-006",
    "case 007": "MODULE-CONTROL-007",
    "case 008": "MODULE-CONTROL-008",
    "case 009": "MODULE-CONTROL-009",
    "case 010": "MODULE-CONTROL-010",
    "case 011": "MODULE-CONTROL-011",
    "case 012": "MODULE-CONTROL-012",
    "case 013": "MODULE-CONTROL-013",
    "case 014": "MODULE-CONTROL-014",
    "case 015": "MODULE-CONTROL-015",
    "case 016": "MODULE-CONTROL-016",
    "case 017": "MODULE-CONTROL-017",
    "case 018": "MODULE-CONTROL-018",
    "case 019": "MODULE-CONTROL-019",
    "case 020": "MODULE-CONTROL-020",
    "case 021": "MODULE-CONTROL-021",
    "case 022": "MODULE-CONTROL-022",
    "case 023": "MODULE-CONTROL-023",
    "case 024": "MODULE-CONTROL-024",
    "case 025": "MODULE-CONTROL-025",
    "case 026": "MODULE-CONTROL-026",
    "case 027": "MODULE-CONTROL-027",
    "case 028": "MODULE-CONTROL-028",
    "case 029": "MODULE-CONTROL-029",
    "case 030": "MODULE-CONTROL-030",
    "case 031": "MODULE-CONTROL-031",
    "case 032": "MODULE-CONTROL-032",
    "case 033": "MODULE-CONTROL-033",
    "case 034": "MODULE-CONTROL-034",
    "fixture shape": "module_control_plane_fixture:",
    "manifest fixture": "manifest:",
    "overlay fixture": "behavior_overlay_stack:",
    "user-local fixture": "user_local_behavior_artifact:",
    "proposal fixture": "tuning_proposal:",
    "promotion fixture": "behavior_artifact_promotion:",
    "validator command": "scripts/check.sh module-control-plane",
}


def check_module_control_plane() -> int:
    return run_marker_check(
        "Module control plane check",
        (
            ("contract", p("docs/product/MODULE-CONTROL-PLANE.md"), MODULE_CONTROL_PLANE_CONTRACT, FORBIDDEN_BASE),
            ("spec", p("evals/specs/module-control-plane.v0.1.md"), MODULE_CONTROL_PLANE_SPEC, FORBIDDEN_BASE),
        ),
        (
            "Contract: docs/product/MODULE-CONTROL-PLANE.md",
            "Spec: evals/specs/module-control-plane.v0.1.md",
        ),
    )


BEHAVIOR_BOUNDARY_CONTRACT = {
    "overlay section": "## Behavior overlay stack",
    "overlay shape": "behavior_overlay_stack:",
    "overlay immutable core ref": "immutable_core_ref:",
    "packaged defaults layer": "packaged_defaults",
    "project workspace layer": "project_workspace",
    "user local layer": "user_local",
    "run local layer": "run_local",
    "module package location": "location_kind: module_package",
    "workspace location": "location_kind: repo_tracked | external_workspace",
    "user local location": "location_kind: user_local_config",
    "run local location": "location_kind: ephemeral_run",
    "module cannot write overlays": "writable_by_module: false",
    "overlay conflict policy": "conflict_policy: later_layer_overrides_behavior_only",
    "overlay capability policy": "capability_policy: overlays_cannot_grant_capabilities",
    "overlay side effect policy": "side_effect_policy: overlays_cannot_expand_side_effects",
    "overlay provenance required": "provenance_required: true",
    "overlay not resolver": "not a resolver, loader, parser, config writer",
    "resolved behavior set": "resolved behavior artifact set",
    "user local section": "## User-local customization artifacts",
    "user local shape": "user_local_behavior_artifact:",
    "local private authority": "authority: local_private",
    "owner scope": "owner_scope: single_operator | single_installation",
    "artifact kind": "artifact_kind: hook_profile | style_profile | prompt_addendum",
    "outside repo storage": "storage_policy: local_private_outside_repo",
    "not exported": "exported_to_project: false",
    "not project truth": "project_truth: false",
    "no secrets": "contains_secrets: false",
    "no private data": "contains_private_data: false",
    "no executable code": "executable_code_allowed: false",
    "no capability delta": "capability_delta: none",
    "no side effect delta": "side_effect_delta: none",
    "immutable core untouched": "immutable_core_touched: false",
    "tuning section": "## Tuning proposal",
    "tuning shape": "tuning_proposal:",
    "manual or scheduled trigger": "trigger: manual | scheduled_proposal_only",
    "target artifact refs": "target_artifact_refs:",
    "evidence limitations": "evidence_limitations:",
    "checks recorded": "check_refs: []",
    "capability change status": "capability_change_requested: false",
    "side effect status": "side_effect_policy_changed: false",
    "drift budget impact": "drift_budget_impact:",
    "proposal not writer": "This is not a patch writer and not an approval.",
    "incomplete proposal rule": "An incomplete proposal may still be useful",
    "promotion section": "## Promotion boundary",
    "promotion shape": "behavior_artifact_promotion:",
    "approval boundary": "Operator approval does not weaken this boundary.",
    "promotion path": "promotion_path_ref:",
    "artifact location kind": "artifact_location_kind: repo_tracked | external_workspace | user_local_config",
    "before artifact": "before_artifact_ref:",
    "after artifact": "after_artifact_ref:",
    "applied change": "applied_change_ref:",
    "promotion provenance": "provenance_ref:",
    "promotion rollback": "rollback_ref:",
    "promotion current ref rule": "The promoted artifact set, not the proposal text",
    "focused validator command": "scripts/check.sh module-behavior-boundaries",
}

BEHAVIOR_BOUNDARY_SPEC = {
    "promotion before after case": "MODULE-CONTROL-020",
    "promotion refs case": "MODULE-CONTROL-021",
    "promotion no capability smuggling case": "MODULE-CONTROL-022",
    "explicit external refs case": "MODULE-CONTROL-023",
    "rollback case": "MODULE-CONTROL-024",
    "overlay order case": "MODULE-CONTROL-025",
    "user local not truth case": "MODULE-CONTROL-026",
    "overlay capabilities case": "MODULE-CONTROL-027",
    "run local ephemeral case": "MODULE-CONTROL-028",
    "resolved behavior set case": "MODULE-CONTROL-029",
    "user local scoped case": "MODULE-CONTROL-030",
    "user local private case": "MODULE-CONTROL-031",
    "no secrets case": "MODULE-CONTROL-032",
    "not executable case": "MODULE-CONTROL-033",
    "visible provenance case": "MODULE-CONTROL-034",
    "focused validator command": "scripts/check.sh module-behavior-boundaries",
}


def check_module_behavior_boundaries() -> int:
    return run_marker_check(
        "Module behavior boundaries check",
        (
            ("contract", p("docs/product/MODULE-CONTROL-PLANE.md"), BEHAVIOR_BOUNDARY_CONTRACT, FORBIDDEN_BASE),
            ("spec", p("evals/specs/module-control-plane.v0.1.md"), BEHAVIOR_BOUNDARY_SPEC, FORBIDDEN_BASE),
        ),
        (
            "Contract: docs/product/MODULE-CONTROL-PLANE.md",
            "Spec: evals/specs/module-control-plane.v0.1.md",
        ),
    )


TUNING_TEMPLATE_TOKENS = {
    "purpose": "Provide a generic fill-in template for one module tuning session.",
    "pubpunk specialization": "docs/modules/pubpunk-live-tuning-handoff-template.md",
    "boundary": "## Boundary",
    "not active handoff": "not an active handoff, runtime input",
    "not approval": "operator approval as permission to apply behavior changes",
    "session section": "## Session header",
    "session shape": "module_tuning_session:",
    "control plane ref": "control_plane_ref: docs/product/MODULE-CONTROL-PLANE.md",
    "validation runbook ref": "validation_runbook_ref: docs/product/MODULE-CONTROL-VALIDATION.md",
    "request channel": "request_channel: text | voice | scheduled_proposal_only",
    "trigger": "trigger: manual | scheduled_proposal_only",
    "target behavior": "target_behavior:",
    "evidence plan": "evidence_plan_ref:",
    "evidence section": "## Evidence packet",
    "evidence shape": "module_tuning_evidence_packet:",
    "local evidence": "local_evidence_refs: []",
    "metrics": "metrics_refs: []",
    "external research allowed": "external_research_allowed: false",
    "external research refs": "external_research_refs: []",
    "evidence limitations": "evidence_limitations: []",
    "no template evidence collection": "no_local_evidence_collection_performed_by_template: true",
    "no template metrics": "no_metrics_collection_performed_by_template: true",
    "no template research": "no_external_research_performed_by_template: true",
    "proposal section": "## Tuning proposal",
    "proposal shape": "module_tuning_proposal:",
    "template eval ref": "evals/specs/module-tuning-handoff-template.v0.1.md",
    "template check ref": "scripts/check.sh module-tuning-handoff-template",
    "suite check ref": "scripts/check.sh module-control-suite",
    "capability change status": "capability_change_requested: false",
    "immutable core status": "immutable_core_touched: false",
    "side effect status": "side_effect_policy_changed: false",
    "drift budget impact": "drift_budget_impact: unknown",
    "decision section": "## Decision record",
    "decision shape": "module_tuning_decision:",
    "decision route": "approved_for_user_local | approved_for_project_promotion",
    "approval not auto apply": "approval_is_auto_apply: false",
    "handoff section": "## Handoff packet",
    "handoff shape": "module_tuning_handoff:",
    "location kind": "proposed_artifact_location_kind: user_local_config | repo_tracked | external_workspace",
    "promotion packet ref": "promotion_packet_ref:",
    "resolved behavior set ref": "resolved_behavior_set_ref:",
    "no capability delta": "capability_delta: none",
    "no side effect delta": "side_effect_delta: none",
    "no secrets": "contains_secrets: false",
    "no private data": "contains_private_data: false",
    "no executable code": "executable_code_allowed: false",
    "completion checklist": "## Completion checklist",
    "request channel checklist": "request channel is recorded",
    "metrics checklist": "metrics refs are present when metrics shaped the proposal",
    "research checklist": "external research refs are present when research shaped the proposal",
    "resolved behavior checklist": "resolved behavior set is cited before future outputs use tuned behavior",
    "validation": "## Validation",
    "non goals": "## Non-goals",
}

TUNING_TEMPLATE_SPEC = {
    "case 001": "MODULE-TUNING-HANDOFF-001",
    "case 002": "MODULE-TUNING-HANDOFF-002",
    "case 003": "MODULE-TUNING-HANDOFF-003",
    "case 004": "MODULE-TUNING-HANDOFF-004",
    "case 005": "MODULE-TUNING-HANDOFF-005",
    "case 006": "MODULE-TUNING-HANDOFF-006",
    "case 007": "MODULE-TUNING-HANDOFF-007",
    "case 008": "MODULE-TUNING-HANDOFF-008",
    "case 009": "MODULE-TUNING-HANDOFF-009",
    "case 010": "MODULE-TUNING-HANDOFF-010",
    "case 011": "MODULE-TUNING-HANDOFF-011",
    "case 012": "MODULE-TUNING-HANDOFF-012",
    "minimal markers": "## Minimal template markers",
    "session marker": "module_tuning_session",
    "evidence marker": "module_tuning_evidence_packet",
    "proposal marker": "module_tuning_proposal",
    "decision marker": "module_tuning_decision",
    "handoff marker": "module_tuning_handoff",
    "request channel": "request_channel: text | voice | scheduled_proposal_only",
    "trigger": "trigger: manual | scheduled_proposal_only",
    "auto apply": "auto_apply: false",
    "approval not auto apply": "approval_is_auto_apply: false",
    "no capability delta": "capability_delta: none",
    "no side effect delta": "side_effect_delta: none",
    "immutable core": "immutable_core_touched: false",
    "no secrets": "contains_secrets: false",
    "no private data": "contains_private_data: false",
    "no executable code": "executable_code_allowed: false",
    "template command": "scripts/check.sh module-tuning-handoff-template",
    "suite command": "scripts/check.sh module-control-suite",
}


def check_module_tuning_handoff_template() -> int:
    return run_marker_check(
        "Module tuning handoff template check",
        (
            ("template", p("docs/product/MODULE-TUNING-HANDOFF-TEMPLATE.md"), TUNING_TEMPLATE_TOKENS, FORBIDDEN_BASE),
            ("spec", p("evals/specs/module-tuning-handoff-template.v0.1.md"), TUNING_TEMPLATE_SPEC, FORBIDDEN_BASE),
        ),
        (
            "Template: docs/product/MODULE-TUNING-HANDOFF-TEMPLATE.md",
            "Spec: evals/specs/module-tuning-handoff-template.v0.1.md",
        ),
    )


PUBPUNK_CONTROL_FIXTURE = {
    "manifest shape": "module_control_manifest:",
    "manifest id": "id: module_control_pubpunk_v0_1",
    "module id": "module_id: pubpunk",
    "pubpunk ref": "docs/modules/pubpunk.md",
    "workspace instructions ref": "docs/modules/pubpunk-workspace-instructions.md",
    "hook fixture ref": "docs/modules/pubpunk-article-hook-tuning-fixture.md",
    "control plane ref": "docs/product/MODULE-CONTROL-PLANE.md",
    "authoring ref": "docs/product/MODULE-AUTHORING.md",
    "conformance ref": "docs/product/MODULE-CONFORMANCE.md",
    "host contract ref": "docs/product/MODULE-HOST-CONTRACT.md",
    "non-authoritative authority": "authority: non_authoritative",
    "behavior artifacts": "behavior_artifacts:",
    "instruction source": "kind: instruction_source",
    "immutable core": "immutable_core:",
    "capability envelope": "capability_envelope",
    "host validation": "host_validation",
    "gate interface": "gate_interface",
    "receipt emission": "receipt_emission",
    "proof authority": "proof_authority",
    "side-effect policy": "side_effect_policy",
    "credential policy": "credential_policy",
    "event-log policy": "direct_event_log_write_policy",
    "capability grants empty": "granted: []",
    "capability requests empty": "requests: []",
    "deny external publish": "external_publish",
    "deny credential read": "credential_read",
    "deny adapter invoke": "adapter_invoke",
    "deny browser automation": "browser_automation",
    "deny metrics": "metrics_collect",
    "deny direct event log": "direct_event_log_write",
    "deny final decision": "final_decision_write",
    "deny proofpack": "proofpack_write",
    "reflection disabled": "trigger: none",
    "manual future trigger": "manual",
    "scheduled proposal trigger": "scheduled_proposal_only",
    "proposal-only output": "output_kind: tuning_proposal",
    "auto apply disabled": "auto_apply: false",
    "provenance": "provenance:",
    "drift budget": "drift_budget:",
    "no current overlay": "No current PubPunk user-local overlay is selected",
    "no current user-local artifact": "No current PubPunk user-local behavior artifact is selected",
    "no current promotion": "No current PubPunk behavior artifact promotion is selected",
    "no current tuning proposal": "No current PubPunk tuning proposal is selected",
    "hook fixture non-current": "current PubPunk behavior",
    "user-local not project truth": "project_truth: false",
    "non authority": "non_authority: true",
    "validator command": "scripts/check.sh pubpunk-control-manifest",
}

PUBPUNK_CONTROL_SPEC = {
    "case 001": "PUBPUNK-CONTROL-001",
    "case 002": "PUBPUNK-CONTROL-002",
    "case 003": "PUBPUNK-CONTROL-003",
    "case 004": "PUBPUNK-CONTROL-004",
    "case 005": "PUBPUNK-CONTROL-005",
    "case 006": "PUBPUNK-CONTROL-006",
    "case 007": "PUBPUNK-CONTROL-007",
    "case 008": "PUBPUNK-CONTROL-008",
    "case 009": "PUBPUNK-CONTROL-009",
    "case 010": "PUBPUNK-CONTROL-010",
    "case 011": "PUBPUNK-CONTROL-011",
    "case 012": "PUBPUNK-CONTROL-012",
    "case 013": "PUBPUNK-CONTROL-013",
    "case 014": "PUBPUNK-CONTROL-014",
    "fixture shape": "pubpunk_control_manifest_fixture:",
    "manifest id": "id: module_control_pubpunk_v0_1",
    "module id": "module_id: pubpunk",
    "workspace instructions ref": "docs/modules/pubpunk-workspace-instructions.md",
    "control plane ref": "docs/product/MODULE-CONTROL-PLANE.md",
    "validator command": "scripts/check.sh pubpunk-control-manifest",
}

PUBPUNK_EXTRA_FORBIDDEN = {
    **FORBIDDEN_BASE,
    "selected user-local overlay": "user_local_overlay_selected: true",
    "selected user-local artifact": "user_local_artifact_selected: true",
    "selected promotion": "behavior_promotion_selected: true",
    "selected tuning proposal": "tuning_proposal_selected: true",
}


def check_pubpunk_control_manifest() -> int:
    return run_marker_check(
        "PubPunk control manifest check",
        (
            ("fixture", p("docs/modules/pubpunk-control-manifest.md"), PUBPUNK_CONTROL_FIXTURE, PUBPUNK_EXTRA_FORBIDDEN),
            ("spec", p("evals/specs/pubpunk-control-manifest.v0.1.md"), PUBPUNK_CONTROL_SPEC, PUBPUNK_EXTRA_FORBIDDEN),
        ),
        (
            "Fixture: docs/modules/pubpunk-control-manifest.md",
            "Spec: evals/specs/pubpunk-control-manifest.v0.1.md",
        ),
    )


HOOK_FIXTURE = {
    "manual tuning request": "manual_in_session_tuning_request:",
    "evidence packet": "hook_tuning_evidence_packet:",
    "tuning proposal": "tuning_proposal:",
    "user-local artifact": "user_local_behavior_artifact:",
    "promotion packet": "behavior_artifact_promotion:",
    "resolved behavior set": "resolved_behavior_set:",
    "example-only boundary": "example-only",
    "auto apply disabled": "auto_apply: false",
    "local-private authority": "authority: local_private",
    "outside repo storage": "storage_policy: local_private_outside_repo",
    "not exported": "exported_to_project: false",
    "not project truth": "project_truth: false",
    "no secrets": "contains_secrets: false",
    "no private data": "contains_private_data: false",
    "no executable code": "executable_code_allowed: false",
    "no capability delta": "capability_delta: none",
    "no side-effect delta": "side_effect_delta: none",
    "immutable core untouched": "immutable_core_touched: false",
    "provenance ref": "provenance_ref:",
    "rollback ref": "rollback_ref:",
    "non authority": "non_authority: true",
}

HOOK_SPEC = {
    "non-applying case": "PUBPUNK-HOOK-TUNING-001",
    "operator request case": "PUBPUNK-HOOK-TUNING-002",
    "evidence limitation case": "PUBPUNK-HOOK-TUNING-003",
    "proposal complete case": "PUBPUNK-HOOK-TUNING-004",
    "private hook profile case": "PUBPUNK-HOOK-TUNING-005",
    "safe hook profile case": "PUBPUNK-HOOK-TUNING-006",
    "promotion provenance case": "PUBPUNK-HOOK-TUNING-007",
    "resolved behavior case": "PUBPUNK-HOOK-TUNING-008",
    "no capability escalation case": "PUBPUNK-HOOK-TUNING-009",
    "no hidden project truth case": "PUBPUNK-HOOK-TUNING-010",
    "not selected": "selected_as_current_behavior: false",
    "runtime inactive": "runtime_activated: false",
    "writer inactive": "writer_activated: false",
    "publishing inactive": "publishing_activated: false",
    "metrics inactive": "metrics_collection_activated: false",
    "external research inactive": "external_research_activated: false",
}


def check_pubpunk_hook_tuning_fixture() -> int:
    return run_marker_check(
        "PubPunk hook tuning fixture check",
        (
            ("fixture", p("docs/modules/pubpunk-article-hook-tuning-fixture.md"), HOOK_FIXTURE, FORBIDDEN_BASE),
            ("spec", p("evals/specs/pubpunk-article-hook-tuning-fixture.v0.1.md"), HOOK_SPEC, FORBIDDEN_BASE),
        ),
        (
            "Fixture: docs/modules/pubpunk-article-hook-tuning-fixture.md",
            "Spec: evals/specs/pubpunk-article-hook-tuning-fixture.v0.1.md",
        ),
    )


LIVE_RUNBOOK = {
    "request shape": "pubpunk_live_tuning_request:",
    "decision shape": "pubpunk_live_tuning_decision:",
    "handoff shape": "pubpunk_live_tuning_handoff:",
    "target behavior": "target_behavior:",
    "evidence plan": "Build the evidence plan.",
    "evidence packet": "Produce an evidence packet.",
    "tuning proposal": "Draft a tuning proposal.",
    "approval not auto-apply": "Approval is promotion input only.",
    "stop conditions": "## Stop conditions",
    "template ref": "docs/modules/pubpunk-live-tuning-handoff-template.md",
    "example ref": "docs/modules/pubpunk-article-hook-live-tuning-handoff-example.md",
    "validator command": "scripts/check.sh pubpunk-live-tuning-pack",
}

LIVE_RUNBOOK_SPEC = {
    "case 001": "PUBPUNK-LIVE-TUNING-001",
    "case 002": "PUBPUNK-LIVE-TUNING-002",
    "case 003": "PUBPUNK-LIVE-TUNING-003",
    "case 004": "PUBPUNK-LIVE-TUNING-004",
    "case 005": "PUBPUNK-LIVE-TUNING-005",
    "case 006": "PUBPUNK-LIVE-TUNING-006",
    "case 007": "PUBPUNK-LIVE-TUNING-007",
    "case 008": "PUBPUNK-LIVE-TUNING-008",
    "case 009": "PUBPUNK-LIVE-TUNING-009",
    "case 010": "PUBPUNK-LIVE-TUNING-010",
    "case 011": "PUBPUNK-LIVE-TUNING-011",
    "case 012": "PUBPUNK-LIVE-TUNING-012",
    "case 013": "PUBPUNK-LIVE-TUNING-013",
    "case 014": "PUBPUNK-LIVE-TUNING-014",
    "case 015": "PUBPUNK-LIVE-TUNING-015",
    "validator command": "scripts/check.sh pubpunk-live-tuning-pack",
}

LIVE_TEMPLATE = {
    "session": "pubpunk_live_tuning_session:",
    "evidence packet": "pubpunk_live_tuning_evidence_packet:",
    "proposal": "pubpunk_live_tuning_proposal:",
    "decision": "pubpunk_live_tuning_decision:",
    "handoff": "pubpunk_live_tuning_handoff:",
    "example ref": "docs/modules/pubpunk-article-hook-live-tuning-handoff-example.md",
    "approval not auto-apply": "approval_is_auto_apply: false",
    "auto apply disabled": "auto_apply: false",
    "no article reads by template": "no_article_reads_performed_by_template: true",
    "no metrics by template": "no_metrics_collection_performed_by_template: true",
    "no research by template": "no_external_research_performed_by_template: true",
    "completion checklist": "## Completion checklist",
    "validator command": "scripts/check.sh pubpunk-live-tuning-pack",
}

LIVE_TEMPLATE_SPEC = {
    "case 001": "PUBPUNK-LIVE-HANDOFF-001",
    "case 002": "PUBPUNK-LIVE-HANDOFF-002",
    "case 003": "PUBPUNK-LIVE-HANDOFF-003",
    "case 004": "PUBPUNK-LIVE-HANDOFF-004",
    "case 005": "PUBPUNK-LIVE-HANDOFF-005",
    "case 006": "PUBPUNK-LIVE-HANDOFF-006",
    "case 007": "PUBPUNK-LIVE-HANDOFF-007",
    "case 008": "PUBPUNK-LIVE-HANDOFF-008",
    "case 009": "PUBPUNK-LIVE-HANDOFF-009",
    "case 010": "PUBPUNK-LIVE-HANDOFF-010",
    "case 011": "PUBPUNK-LIVE-HANDOFF-011",
    "session marker": "pubpunk_live_tuning_session",
    "handoff marker": "pubpunk_live_tuning_handoff",
    "approval not auto-apply": "approval_is_auto_apply: false",
    "validator command": "scripts/check.sh pubpunk-live-tuning-pack",
}

LIVE_EXAMPLE = {
    "session": "pubpunk_live_tuning_session:",
    "evidence packet": "pubpunk_live_tuning_evidence_packet:",
    "proposal": "pubpunk_live_tuning_proposal:",
    "decision": "pubpunk_live_tuning_decision:",
    "handoff": "pubpunk_live_tuning_handoff:",
    "example status": "status: example_only",
    "no real articles": "No real articles were read.",
    "no metrics": "No metrics were collected.",
    "no research": "No external research",
    "no user-local write": "No user-local artifact was written.",
    "no current behavior": "current PubPunk behavior.",
    "approval not auto-apply": "approval_is_auto_apply: false",
    "auto apply disabled": "auto_apply: false",
    "user-local location": "proposed_artifact_location_kind: user_local_config",
    "example hook profile": "The text above is example content.",
    "completion checklist": "## Completion checklist",
    "validator command": "scripts/check.sh pubpunk-live-tuning-pack",
}

LIVE_EXAMPLE_SPEC = {
    "case 001": "PUBPUNK-HOOK-LIVE-HANDOFF-001",
    "case 002": "PUBPUNK-HOOK-LIVE-HANDOFF-002",
    "case 003": "PUBPUNK-HOOK-LIVE-HANDOFF-003",
    "case 004": "PUBPUNK-HOOK-LIVE-HANDOFF-004",
    "case 005": "PUBPUNK-HOOK-LIVE-HANDOFF-005",
    "case 006": "PUBPUNK-HOOK-LIVE-HANDOFF-006",
    "case 007": "PUBPUNK-HOOK-LIVE-HANDOFF-007",
    "case 008": "PUBPUNK-HOOK-LIVE-HANDOFF-008",
    "case 009": "PUBPUNK-HOOK-LIVE-HANDOFF-009",
    "example status": "status: example_only",
    "validator command": "scripts/check.sh pubpunk-live-tuning-pack",
}

SAFE_DEFAULT_TOKENS = {
    "no capability delta": "capability_delta: none",
    "no side-effect delta": "side_effect_delta: none",
    "immutable core untouched": "immutable_core_touched: false",
    "no secrets": "contains_secrets: false",
    "no private data": "contains_private_data: false",
    "no executable code": "executable_code_allowed: false",
}


def check_pubpunk_live_tuning_pack() -> int:
    return run_marker_check(
        "PubPunk live tuning pack check",
        (
            ("runbook", p("docs/modules/pubpunk-live-tuning-runbook.md"), {**LIVE_RUNBOOK, **SAFE_DEFAULT_TOKENS}, FORBIDDEN_BASE),
            ("runbook spec", p("evals/specs/pubpunk-live-tuning-runbook.v0.1.md"), LIVE_RUNBOOK_SPEC, FORBIDDEN_BASE),
            ("template", p("docs/modules/pubpunk-live-tuning-handoff-template.md"), {**LIVE_TEMPLATE, **SAFE_DEFAULT_TOKENS}, FORBIDDEN_BASE),
            ("template spec", p("evals/specs/pubpunk-live-tuning-handoff-template.v0.1.md"), LIVE_TEMPLATE_SPEC, FORBIDDEN_BASE),
            ("example", p("docs/modules/pubpunk-article-hook-live-tuning-handoff-example.md"), {**LIVE_EXAMPLE, **SAFE_DEFAULT_TOKENS}, FORBIDDEN_BASE),
            ("example spec", p("evals/specs/pubpunk-article-hook-live-tuning-handoff-example.v0.1.md"), LIVE_EXAMPLE_SPEC, FORBIDDEN_BASE),
        ),
        (
            "Runbook: docs/modules/pubpunk-live-tuning-runbook.md",
            "Template: docs/modules/pubpunk-live-tuning-handoff-template.md",
            "Example: docs/modules/pubpunk-article-hook-live-tuning-handoff-example.md",
        ),
    )


VALIDATION_COMMANDS = {
    "module control plane command": "scripts/check.sh module-control-plane",
    "behavior boundaries command": "scripts/check.sh module-behavior-boundaries",
    "tuning handoff template command": "scripts/check.sh module-tuning-handoff-template",
    "pubpunk manifest command": "scripts/check.sh pubpunk-control-manifest",
    "pubpunk hook fixture command": "scripts/check.sh pubpunk-hook-tuning-fixture",
    "pubpunk live tuning pack command": "scripts/check.sh pubpunk-live-tuning-pack",
    "validation command": "scripts/check.sh module-control-validation",
    "suite command": "scripts/check.sh module-control-suite",
}

VALIDATION_RUNBOOK = {
    **VALIDATION_COMMANDS,
    "marker script": "scripts/check_module_control_markers.py",
    "purpose": "Define the operator runbook for the current Module Control Plane marker checks.",
    "boundary section": "## Boundary",
    "command map": "## Command map",
    "repo research gate": "python3 scripts/check_research_gate.py",
    "repo work ledger": "python3 scripts/check_work_ledger.py",
    "docs governance": "scripts/check.sh docs-governance --files <changed-files> --report <work-report>",
    "tooling policy": "## Tooling policy",
    "python marker boundary": "Use Python only for static repo-governance and docs/spec marker checks.",
    "rust runtime boundary": "belong in Rust",
    "marker meaning": "Passing marker checks mean the expected boundary markers are still present",
    "not manifest parsing": "manifest parsing is active",
    "not overlay runtime": "overlays are resolved at runtime",
    "not writer": "behavior artifacts can write themselves",
    "not config writer": "user-local config can be written",
    "not reflection": "reflection can run in the background",
    "not metrics": "metrics were collected",
    "not research": "external research was run",
    "not adapters": "adapters or browsers were invoked",
    "not publishing": "publishing happened",
    "not acceptance": "events, gates, proofpacks, or acceptance claims were written",
    "smallest check": "Use the smallest check that covers the edited surface",
    "suite before closure": "before closing a",
    "wiring changes": "whenever a command name,",
    "failure handling": "## Failure handling",
    "do not remove markers": "do not remove a no-runtime, no-auto-apply, no-writer",
    "runtime separate goal": "separate selected goal",
    "suite membership": "## Suite script",
    "suite non-authority": "not a gate, proofpack",
    "non goals": "## Non-goals",
    "validation section": "## Validation",
}

VALIDATION_SPEC = {
    **VALIDATION_COMMANDS,
    "case 001": "MODULE-CONTROL-VALIDATION-001",
    "case 002": "MODULE-CONTROL-VALIDATION-002",
    "case 003": "MODULE-CONTROL-VALIDATION-003",
    "case 004": "MODULE-CONTROL-VALIDATION-004",
    "case 005": "MODULE-CONTROL-VALIDATION-005",
    "case 006": "MODULE-CONTROL-VALIDATION-006",
    "case 007": "MODULE-CONTROL-VALIDATION-007",
    "case 008": "MODULE-CONTROL-VALIDATION-008",
    "case 009": "MODULE-CONTROL-VALIDATION-009",
    "case 010": "MODULE-CONTROL-VALIDATION-010",
    "case 011": "MODULE-CONTROL-VALIDATION-011",
    "case 012": "MODULE-CONTROL-VALIDATION-012",
    "case 013": "MODULE-CONTROL-VALIDATION-013",
    "case 014": "MODULE-CONTROL-VALIDATION-014",
    "case 015": "MODULE-CONTROL-VALIDATION-015",
    "not runtime proof": "marker checks are not runtime proof",
    "failure preserves boundaries": "failure handling preserves boundaries",
    "non authoritative": "validation is non-authoritative",
    "python limited": "Python is limited to marker checks",
}

CHECK_SH_TOKENS = {
    "target script": "scripts/check_module_control_markers.py",
    "validation target": "module-control-validation)",
    "suite target": "module-control-suite)",
    "usage": "scripts/check.sh module-control-validation",
}


def check_module_control_validation() -> int:
    return run_marker_check(
        "Module control validation check",
        (
            ("runbook", p("docs/product/MODULE-CONTROL-VALIDATION.md"), VALIDATION_RUNBOOK, FORBIDDEN_BASE),
            ("spec", p("evals/specs/module-control-validation.v0.1.md"), VALIDATION_SPEC, FORBIDDEN_BASE),
            ("check.sh", p("scripts/check.sh"), CHECK_SH_TOKENS, {}),
        ),
        (
            "Runbook: docs/product/MODULE-CONTROL-VALIDATION.md",
            "Spec: evals/specs/module-control-validation.v0.1.md",
        ),
    )


CHECKS = {
    "module-control-plane": ("Module Control Plane", check_module_control_plane),
    "module-behavior-boundaries": ("Module behavior boundaries", check_module_behavior_boundaries),
    "module-tuning-handoff-template": ("Module tuning handoff template", check_module_tuning_handoff_template),
    "pubpunk-control-manifest": ("PubPunk control manifest", check_pubpunk_control_manifest),
    "pubpunk-hook-tuning-fixture": ("PubPunk hook tuning fixture", check_pubpunk_hook_tuning_fixture),
    "pubpunk-live-tuning-pack": ("PubPunk live tuning pack", check_pubpunk_live_tuning_pack),
    "module-control-validation": ("Module control validation", check_module_control_validation),
}


def check_module_control_suite() -> int:
    failed = False
    for _target, (label, check) in CHECKS.items():
        print(f"== {label} ==", flush=True)
        if check() != 0:
            failed = True

    if failed:
        print("Module control suite check: FAIL")
        return 1

    print("Module control suite check: PASS")
    return 0


def usage() -> str:
    targets = "\n".join(f"  {target}" for target in sorted([*CHECKS, "module-control-suite"]))
    return f"Usage: check_module_control_markers.py <target>\n\nTargets:\n{targets}"


def main(argv: list[str]) -> int:
    if len(argv) != 2 or argv[1] in {"-h", "--help"}:
        print(usage(), file=sys.stderr)
        return 2

    target = argv[1]
    if target == "module-control-suite":
        return check_module_control_suite()

    item = CHECKS.get(target)
    if item is None:
        print(f"Unknown module control marker target: {target}", file=sys.stderr)
        print(usage(), file=sys.stderr)
        return 2

    return item[1]()


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
