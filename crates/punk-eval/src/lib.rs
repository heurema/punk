//! Minimal deterministic smoke eval harness for the current Punk core.
//!
//! This Phase 1/2/3 bridge stays library-first. It assesses the existing flow,
//! event, receipt, gate, and proof kernels without activating `.punk/` runtime
//! state, baseline comparison, waiver storage, or a full eval platform.

use std::fmt::Write as _;

use punk_contract::{
    approve_contract, validate_contract, ContractDraft, ContractError, ContractId, ContractScope,
    ContractStatus,
};
use punk_core::{
    is_canonical_artifact_digest, is_valid_repo_relative_artifact_ref, validate_artifact_digest,
    validate_repo_relative_artifact_ref, ArtifactDigest, ArtifactHashPolicyError,
    RepoRelativeArtifactRef, ARTIFACT_HASH_POLICY_CAPABILITIES, ARTIFACT_HASH_POLICY_VERSION,
};
use punk_domain::{ContractRef, ProducedAt, RunId, RunReceiptId, RunScopeRef};
use punk_events::{schema_fixture, MemoryEventLog};
use punk_flow::{transition_attempt_event_draft, FlowCommand, FlowInstance, FlowState};
use punk_gate::{
    GateBoundaryNote, GateContractRef, GateCreatedAt, GateDecision, GateDecisionId,
    GateDecisionOutcome, GateEvalRef, GateEventRef, GateRunReceiptRef,
};
use punk_proof::{
    positive_acceptance_preconditions_met, PositiveAcceptanceInputs, ProofArtifactDigest,
    ProofArtifactHash, ProofArtifactKind, ProofArtifactRef, ProofBoundaryNote, ProofContractRef,
    ProofCreatedAt, ProofEvalRef, ProofEventRef, ProofGateDecisionRef, ProofOutputArtifactRef,
    ProofRunReceiptRef, Proofpack, ProofpackId,
};

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const SMOKE_SUITE_ID: &str = "smoke.v0";
pub const SMOKE_REPORT_SCHEMA_VERSION: &str = "smoke-eval-report.v0.1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmokeEvalStatus {
    Pass,
    Fail,
}

impl SmokeEvalStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmokeEvalCaseResult {
    pub case_id: &'static str,
    pub summary: &'static str,
    pub status: SmokeEvalStatus,
    pub assessment: String,
}

impl SmokeEvalCaseResult {
    fn pass(case_id: &'static str, summary: &'static str, assessment: impl Into<String>) -> Self {
        Self {
            case_id,
            summary,
            status: SmokeEvalStatus::Pass,
            assessment: assessment.into(),
        }
    }

    fn fail(case_id: &'static str, summary: &'static str, assessment: impl Into<String>) -> Self {
        Self {
            case_id,
            summary,
            status: SmokeEvalStatus::Fail,
            assessment: assessment.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmokeEvalSummary {
    suite_id: &'static str,
    smoke_result: SmokeEvalStatus,
    assessment: String,
    mode: &'static str,
    runtime_persistence: &'static str,
    report_storage: &'static str,
}

impl SmokeEvalSummary {
    pub fn suite_id(&self) -> &'static str {
        self.suite_id
    }

    pub fn smoke_result(&self) -> SmokeEvalStatus {
        self.smoke_result
    }

    pub fn assessment(&self) -> &str {
        &self.assessment
    }

    pub fn mode(&self) -> &'static str {
        self.mode
    }

    pub fn runtime_persistence(&self) -> &'static str {
        self.runtime_persistence
    }

    pub fn report_storage(&self) -> &'static str {
        self.report_storage
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmokeEvalReport {
    summary: SmokeEvalSummary,
    cases: Vec<SmokeEvalCaseResult>,
    boundary_notes: Vec<&'static str>,
    deferred_notes: Vec<&'static str>,
}

impl SmokeEvalReport {
    pub fn summary(&self) -> &SmokeEvalSummary {
        &self.summary
    }

    pub fn suite_id(&self) -> &'static str {
        self.summary.suite_id()
    }

    pub fn smoke_result(&self) -> SmokeEvalStatus {
        self.summary.smoke_result()
    }

    pub fn assessment(&self) -> &str {
        self.summary.assessment()
    }

    pub fn mode(&self) -> &'static str {
        self.summary.mode()
    }

    pub fn runtime_persistence(&self) -> &'static str {
        self.summary.runtime_persistence()
    }

    pub fn report_storage(&self) -> &'static str {
        self.summary.report_storage()
    }

    pub fn cases(&self) -> &[SmokeEvalCaseResult] {
        &self.cases
    }

    pub fn boundary_notes(&self) -> &[&'static str] {
        &self.boundary_notes
    }

    pub fn deferred_notes(&self) -> &[&'static str] {
        &self.deferred_notes
    }

    pub fn passed(&self) -> bool {
        self.smoke_result() == SmokeEvalStatus::Pass
    }

    pub fn exit_code(&self) -> u8 {
        if self.passed() {
            0
        } else {
            1
        }
    }

    pub fn render_json(&self) -> String {
        let mut output = String::new();
        output.push_str("{\n");

        write_json_field(
            &mut output,
            1,
            "schema_version",
            JsonValue::String(SMOKE_REPORT_SCHEMA_VERSION),
            true,
        );
        write_json_field(
            &mut output,
            1,
            "suite_id",
            JsonValue::String(self.suite_id()),
            true,
        );
        write_json_field(&mut output, 1, "run_id", JsonValue::Null, true);
        write_json_field(
            &mut output,
            1,
            "smoke_result",
            JsonValue::String(self.smoke_result().as_str()),
            true,
        );
        write_json_field(&mut output, 1, "mode", JsonValue::String(self.mode()), true);
        write_json_field(
            &mut output,
            1,
            "runtime_persistence",
            JsonValue::String(self.runtime_persistence()),
            true,
        );
        write_json_field(
            &mut output,
            1,
            "report_storage",
            JsonValue::String(self.report_storage()),
            true,
        );

        output.push_str("  \"case_results\": [\n");
        for (index, case) in self.cases().iter().enumerate() {
            output.push_str("    {\n");
            write_json_field(
                &mut output,
                3,
                "case_id",
                JsonValue::String(case.case_id),
                true,
            );
            write_json_field(
                &mut output,
                3,
                "status",
                JsonValue::String(case.status.as_str()),
                true,
            );
            write_json_field(
                &mut output,
                3,
                "summary",
                JsonValue::String(case.summary),
                true,
            );
            write_json_field(
                &mut output,
                3,
                "assessment",
                JsonValue::String(case.assessment.as_str()),
                false,
            );
            output.push_str("    }");
            if index + 1 != self.cases().len() {
                output.push(',');
            }
            output.push('\n');
        }
        output.push_str("  ],\n");

        write_json_field(
            &mut output,
            1,
            "boundary_notes",
            JsonValue::StringArray(self.boundary_notes()),
            true,
        );
        write_json_field(
            &mut output,
            1,
            "deferred",
            JsonValue::StringArray(self.deferred_notes()),
            false,
        );
        output.push_str("}");

        output
    }

    pub fn render_human(&self) -> String {
        let mut output = String::new();
        writeln!(&mut output, "punk eval run smoke").expect("writing to String should succeed");
        writeln!(&mut output, "mode: {}", self.mode()).expect("writing to String should succeed");
        writeln!(
            &mut output,
            "runtime_persistence: {}",
            self.runtime_persistence()
        )
        .expect("writing to String should succeed");
        writeln!(&mut output, "report_storage: {}", self.report_storage())
            .expect("writing to String should succeed");
        writeln!(&mut output, "suite_id: {}", self.suite_id())
            .expect("writing to String should succeed");
        writeln!(
            &mut output,
            "smoke_result: {}",
            self.smoke_result().as_str()
        )
        .expect("writing to String should succeed");
        writeln!(&mut output, "assessment: {}", self.assessment())
            .expect("writing to String should succeed");
        writeln!(&mut output, "case_results:").expect("writing to String should succeed");

        for case in self.cases() {
            writeln!(&mut output, "  - id: {}", case.case_id)
                .expect("writing to String should succeed");
            writeln!(&mut output, "    status: {}", case.status.as_str())
                .expect("writing to String should succeed");
            writeln!(&mut output, "    summary: {}", case.summary)
                .expect("writing to String should succeed");
            writeln!(&mut output, "    assessment: {}", case.assessment)
                .expect("writing to String should succeed");
        }

        writeln!(&mut output, "notes:").expect("writing to String should succeed");
        for note in self.boundary_notes() {
            writeln!(&mut output, "  - {note}").expect("writing to String should succeed");
        }

        writeln!(&mut output, "deferred:").expect("writing to String should succeed");
        for note in self.deferred_notes() {
            writeln!(&mut output, "  - {note}").expect("writing to String should succeed");
        }

        output.trim_end().to_owned()
    }
}

pub fn run_smoke_suite() -> SmokeEvalReport {
    let cases = vec![
        eval_flow_allows_approval_transition(),
        eval_flow_denies_run_before_approval(),
        eval_denied_transition_preserves_state(),
        eval_flow_transition_produces_event_evidence(),
        eval_event_log_is_append_only(),
        eval_contract_ready_for_bounded_work_allows_start_run(),
        eval_contract_draft_denies_start_run(),
        eval_contract_invalid_scope_denies_start_run(),
        eval_contract_denial_does_not_mutate_flow_state(),
        eval_contract_guard_result_remains_evidence_not_decision(),
        eval_contract_receipt_allowed_path_produces_evidence(),
        eval_contract_receipt_draft_denial_produces_no_receipt(),
        eval_contract_receipt_invalid_scope_produces_no_receipt(),
        eval_contract_receipt_remains_pre_gate_evidence(),
        eval_gate_authority_requires_proof_before_acceptance(),
        eval_proofpack_is_post_gate_provenance_not_decision(),
        eval_acceptance_requires_accepting_decision_and_matching_proofpack(),
        eval_proofpack_integrity_ready_when_declared_digest_links_are_complete(),
        eval_proofpack_integrity_missing_digest_blocks_readiness(),
        eval_artifact_hash_policy_accepts_canonical_digest(),
        eval_artifact_hash_policy_rejects_invalid_digest(),
        eval_artifact_hash_policy_accepts_repo_relative_ref(),
        eval_artifact_hash_policy_rejects_invalid_ref(),
        eval_artifact_hash_policy_helper_boundary_flags(),
    ];
    let smoke_result = if cases
        .iter()
        .all(|case| case.status == SmokeEvalStatus::Pass)
    {
        SmokeEvalStatus::Pass
    } else {
        SmokeEvalStatus::Fail
    };
    let assessment = if smoke_result == SmokeEvalStatus::Pass {
        "local deterministic smoke harness passed over current contract, flow, receipt, event, gate, proof, and artifact hash policy helper kernels"
            .to_owned()
    } else {
        "local deterministic smoke harness found one or more failing cases over current contract, flow, receipt, event, gate, proof, and artifact hash policy helper kernels"
            .to_owned()
    };

    SmokeEvalReport {
        summary: SmokeEvalSummary {
            suite_id: SMOKE_SUITE_ID,
            smoke_result,
            assessment,
            mode: "local-smoke-check",
            runtime_persistence: "inactive",
            report_storage: "inactive",
        },
        cases,
        boundary_notes: vec![
            "local assessment only; no authority is written here",
            "no .punk/evals runtime state is read or written",
            "run receipt evidence remains pre-gate and does not imply final acceptance",
            "gate/proof smoke cases remain local assessment and do not claim acceptance",
            "artifact hash policy smoke cases validate helper shape only and do not compute hashes",
            "JSON output is opt-in only and does not imply a stable public contract",
        ],
        deferred_notes: vec![
            "baseline, waiver, and stored eval reports are not active",
            "schema validation and export adapters are not active",
            "active artifact hash computation and byte normalization are not active",
        ],
    }
}

enum JsonValue<'a> {
    String(&'a str),
    StringArray(&'a [&'a str]),
    Null,
}

fn write_json_field(
    output: &mut String,
    indent_level: usize,
    key: &str,
    value: JsonValue<'_>,
    trailing_comma: bool,
) {
    output.push_str(&"  ".repeat(indent_level));
    push_json_string(output, key);
    output.push_str(": ");

    match value {
        JsonValue::String(value) => push_json_string(output, value),
        JsonValue::StringArray(values) => {
            output.push('[');
            for (index, value) in values.iter().enumerate() {
                if index > 0 {
                    output.push_str(", ");
                }
                push_json_string(output, value);
            }
            output.push(']');
        }
        JsonValue::Null => output.push_str("null"),
    }

    if trailing_comma {
        output.push(',');
    }
    output.push('\n');
}

fn push_json_string(output: &mut String, value: &str) {
    output.push('"');
    for ch in value.chars() {
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{08}' => output.push_str("\\b"),
            '\u{0C}' => output.push_str("\\f"),
            ch if ch.is_control() => {
                write!(output, "\\u{:04x}", ch as u32).expect("writing to String should succeed");
            }
            ch => output.push(ch),
        }
    }
    output.push('"');
}

fn eval_flow_allows_approval_transition() -> SmokeEvalCaseResult {
    let instance = FlowInstance::new(FlowState::AwaitingApproval);
    match instance.transition(FlowCommand::Approve) {
        Ok(next) if next.state() == FlowState::Approved => SmokeEvalCaseResult::pass(
            "eval_flow_allows_approval_transition",
            "allowed transition remains deterministic",
            "approval still advances AwaitingApproval -> Approved",
        ),
        Ok(next) => SmokeEvalCaseResult::fail(
            "eval_flow_allows_approval_transition",
            "allowed transition remains deterministic",
            format!(
                "approval returned unexpected next state {}",
                next.state().as_str()
            ),
        ),
        Err(error) => SmokeEvalCaseResult::fail(
            "eval_flow_allows_approval_transition",
            "allowed transition remains deterministic",
            format!(
                "approval was denied with next allowed commands {}",
                format_commands(error.next_allowed_commands)
            ),
        ),
    }
}

fn eval_flow_denies_run_before_approval() -> SmokeEvalCaseResult {
    let instance = FlowInstance::new(FlowState::AwaitingApproval);
    match instance.transition(FlowCommand::StartRun) {
        Ok(next) => SmokeEvalCaseResult::fail(
            "eval_flow_denies_run_before_approval",
            "illegal run transition stays denied",
            format!(
                "StartRun unexpectedly moved to {}",
                next.state().as_str()
            ),
        ),
        Err(error)
            if error.current_state == FlowState::AwaitingApproval
                && error.attempted_command == FlowCommand::StartRun
                && error.next_allowed_commands.contains(&FlowCommand::Approve) =>
        {
            SmokeEvalCaseResult::pass(
                "eval_flow_denies_run_before_approval",
                "illegal run transition stays denied",
                "StartRun remains denied before approval and still points to Approve as the next allowed command",
            )
        }
        Err(error) => SmokeEvalCaseResult::fail(
            "eval_flow_denies_run_before_approval",
            "illegal run transition stays denied",
            format!(
                "denial shape drifted; next allowed commands are {}",
                format_commands(error.next_allowed_commands)
            ),
        ),
    }
}

fn eval_denied_transition_preserves_state() -> SmokeEvalCaseResult {
    let instance = FlowInstance::new(FlowState::AwaitingApproval);
    let attempt = instance.attempt_transition(FlowCommand::StartRun);

    if instance.state() == FlowState::AwaitingApproval && attempt.next_state().is_none() {
        SmokeEvalCaseResult::pass(
            "eval_denied_transition_preserves_state",
            "denied transitions do not mutate state",
            "denied StartRun attempt leaves AwaitingApproval unchanged",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_denied_transition_preserves_state",
            "denied transitions do not mutate state",
            "denied transition changed state evidence unexpectedly",
        )
    }
}

fn eval_flow_transition_produces_event_evidence() -> SmokeEvalCaseResult {
    let attempt =
        FlowInstance::new(FlowState::AwaitingApproval).attempt_transition(FlowCommand::StartRun);
    let draft = transition_attempt_event_draft(
        &attempt,
        "smoke_eval_preview",
        Some("work/goals/goal_add_smoke_eval_harness.md"),
    );

    if draft.kind.as_str() == "transition_denied"
        && draft.result.status.as_str() == "denied"
        && draft.result.guard_code.as_deref() == Some("CUT_REQUIRES_APPROVED_CONTRACT")
    {
        SmokeEvalCaseResult::pass(
            "eval_flow_transition_produces_event_evidence",
            "transition attempts still emit event evidence",
            "denied flow attempt produces guard-denial event evidence without decision authority",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_flow_transition_produces_event_evidence",
            "transition attempts still emit event evidence",
            format!(
                "event evidence drifted to kind={} status={} guard_code={:?}",
                draft.kind.as_str(),
                draft.result.status.as_str(),
                draft.result.guard_code,
            ),
        )
    }
}

fn eval_event_log_is_append_only() -> SmokeEvalCaseResult {
    let mut log = MemoryEventLog::default();
    let first = log.append(schema_fixture());

    let second_attempt =
        FlowInstance::new(FlowState::AwaitingApproval).attempt_transition(FlowCommand::Approve);
    let second_draft = transition_attempt_event_draft(&second_attempt, "smoke_eval_append", None);
    let second = log.append(second_draft);

    match (first, second) {
        (Ok(first), Ok(second))
            if first.sequence == 1
                && second.sequence == 2
                && log.events().len() == 2
                && first.event_id == "evt_0000000000000001"
                && second.event_id == "evt_0000000000000002" =>
        {
            SmokeEvalCaseResult::pass(
                "eval_event_log_is_append_only",
                "append-only event log stays monotonic",
                "append-only log preserved prior records and emitted monotonic ids and sequences",
            )
        }
        (Ok(first), Ok(second)) => SmokeEvalCaseResult::fail(
            "eval_event_log_is_append_only",
            "append-only event log stays monotonic",
            format!(
                "append-only evidence drifted; first={} second={} len={}",
                first.event_id,
                second.event_id,
                log.events().len(),
            ),
        ),
        (Err(error), _) | (_, Err(error)) => SmokeEvalCaseResult::fail(
            "eval_event_log_is_append_only",
            "append-only event log stays monotonic",
            format!("append failed with {error:?}"),
        ),
    }
}

fn valid_contract_scope() -> ContractScope {
    ContractScope::new()
        .with_ref("work/goals/goal_add_contract_lifecycle_minimal.md")
        .with_ref("work/goals/goal_connect_contract_to_flow_state.md")
}

fn valid_contract_draft() -> ContractDraft {
    ContractDraft::new(
        ContractId::new("contract_eval_001").expect("contract id should be valid"),
        "Contract-aware flow guard smoke coverage",
        valid_contract_scope(),
    )
}

fn eval_contract_ready_for_bounded_work_allows_start_run() -> SmokeEvalCaseResult {
    let contract = approve_contract(valid_contract_draft()).expect("contract should approve");
    let instance = FlowInstance::new(FlowState::Approved);

    match instance.transition_with_contract(
        FlowCommand::StartRun,
        contract.status(),
        contract.scope_valid(),
    ) {
        Ok(next) if next.state() == FlowState::Running => SmokeEvalCaseResult::pass(
            "eval_contract_ready_for_bounded_work_allows_start_run",
            "bounded-work-ready contract still authorizes bounded run start",
            "ApprovedForRun with explicit scope still allows Approved -> Running without implying final acceptance",
        ),
        Ok(next) => SmokeEvalCaseResult::fail(
            "eval_contract_ready_for_bounded_work_allows_start_run",
            "bounded-work-ready contract still authorizes bounded run start",
            format!(
                "contract-authorized StartRun returned unexpected next state {}",
                next.state().as_str()
            ),
        ),
        Err(error) => SmokeEvalCaseResult::fail(
            "eval_contract_ready_for_bounded_work_allows_start_run",
            "bounded-work-ready contract still authorizes bounded run start",
            format!(
                "bounded-work-ready contract was denied with next allowed commands {}",
                format_commands(error.next_allowed_commands)
            ),
        ),
    }
}

fn eval_contract_draft_denies_start_run() -> SmokeEvalCaseResult {
    let draft = valid_contract_draft();
    let instance = FlowInstance::new(FlowState::Approved);
    let attempt = instance.attempt_transition_with_contract(
        FlowCommand::StartRun,
        ContractStatus::Draft,
        !draft.scope().is_empty(),
    );

    if instance.state() == FlowState::Approved
        && attempt.next_state().is_none()
        && attempt.guard_code() == Some("RUN_REQUIRES_APPROVED_FOR_RUN_CONTRACT")
    {
        SmokeEvalCaseResult::pass(
            "eval_contract_draft_denies_start_run",
            "draft contract still denies bounded run start",
            "draft contract keeps StartRun denied until ApprovedForRun is present",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_contract_draft_denies_start_run",
            "draft contract still denies bounded run start",
            format!(
                "draft contract denial drifted; next_state={:?} guard_code={:?}",
                attempt.next_state(),
                attempt.guard_code()
            ),
        )
    }
}

fn eval_contract_invalid_scope_denies_start_run() -> SmokeEvalCaseResult {
    let invalid_draft = ContractDraft::new(
        ContractId::new("contract_eval_002").expect("contract id should be valid"),
        "Invalid scope should deny start run",
        ContractScope::new(),
    );
    let instance = FlowInstance::new(FlowState::Approved);
    let validation = validate_contract(&invalid_draft);
    let attempt = instance.attempt_transition_with_contract(
        FlowCommand::StartRun,
        ContractStatus::Draft,
        !invalid_draft.scope().is_empty(),
    );

    if validation == Err(ContractError::EmptyScope)
        && instance.state() == FlowState::Approved
        && attempt.next_state().is_none()
        && attempt.guard_code() == Some("RUN_REQUIRES_EXPLICIT_CONTRACT_SCOPE")
    {
        SmokeEvalCaseResult::pass(
            "eval_contract_invalid_scope_denies_start_run",
            "empty contract scope still denies bounded run start",
            "empty scope remains invalid and keeps StartRun denied before any run begins",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_contract_invalid_scope_denies_start_run",
            "empty contract scope still denies bounded run start",
            format!(
                "invalid scope denial drifted; validation={validation:?} next_state={:?} guard_code={:?}",
                attempt.next_state(),
                attempt.guard_code()
            ),
        )
    }
}

fn eval_contract_denial_does_not_mutate_flow_state() -> SmokeEvalCaseResult {
    let draft = valid_contract_draft();
    let instance = FlowInstance::new(FlowState::Approved);
    let attempt = instance.attempt_transition_with_contract(
        FlowCommand::StartRun,
        ContractStatus::Draft,
        !draft.scope().is_empty(),
    );

    if instance.state() == FlowState::Approved && attempt.next_state().is_none() {
        SmokeEvalCaseResult::pass(
            "eval_contract_denial_does_not_mutate_flow_state",
            "contract-aware denial still preserves flow state",
            "denied contract-aware StartRun leaves Approved unchanged",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_contract_denial_does_not_mutate_flow_state",
            "contract-aware denial still preserves flow state",
            format!(
                "contract-aware denial changed state evidence unexpectedly; current_state={} next_state={:?}",
                instance.state().as_str(),
                attempt.next_state()
            ),
        )
    }
}

fn eval_contract_guard_result_remains_evidence_not_decision() -> SmokeEvalCaseResult {
    let draft_contract = valid_contract_draft();
    let attempt = FlowInstance::new(FlowState::Approved).attempt_transition_with_contract(
        FlowCommand::StartRun,
        ContractStatus::Draft,
        !draft_contract.scope().is_empty(),
    );
    let draft = transition_attempt_event_draft(
        &attempt,
        "smoke_eval_contract_guard",
        Some("work/goals/goal_connect_contract_to_flow_state.md"),
    );

    if draft.result.status.as_str() == "denied"
        && draft.result.guard_code.as_deref() == Some("RUN_REQUIRES_APPROVED_FOR_RUN_CONTRACT")
        && !draft.kind.as_str().contains("decision")
        && !draft.kind.as_str().contains("gate")
    {
        SmokeEvalCaseResult::pass(
            "eval_contract_guard_result_remains_evidence_not_decision",
            "contract guard results remain evidence only",
            "contract-aware denial still emits guard evidence without becoming authoritative closure",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_contract_guard_result_remains_evidence_not_decision",
            "contract guard results remain evidence only",
            format!(
                "contract guard evidence drifted to kind={} status={} guard_code={:?}",
                draft.kind.as_str(),
                draft.result.status.as_str(),
                draft.result.guard_code,
            ),
        )
    }
}

fn eval_contract_receipt_allowed_path_produces_evidence() -> SmokeEvalCaseResult {
    let contract = approve_contract(valid_contract_draft()).expect("contract should approve");
    let receipt_attempt = FlowInstance::new(FlowState::Approved)
        .attempt_transition_with_contract_receipt(
            FlowCommand::StartRun,
            contract.status(),
            contract.scope_valid(),
            RunReceiptId::new("receipt_eval_001").expect("receipt id should be valid"),
            ProducedAt::new("2026-04-25T18:55:00Z").expect("produced_at should be valid"),
            ContractRef::new(contract.id().as_str()).expect("contract ref should be valid"),
            RunId::new("run_eval_001").expect("run id should be valid"),
            RunScopeRef::new("work/goals/goal_add_run_receipt_smoke_eval.md")
                .expect("run scope should be valid"),
        );

    match receipt_attempt.receipt() {
        Some(receipt)
            if receipt_attempt.transition().next_state() == Some(FlowState::Running)
                && receipt.id().as_str() == "receipt_eval_001"
                && receipt.contract_ref().as_str() == contract.id().as_str()
                && receipt.run_id().as_str() == "run_eval_001" =>
        {
            SmokeEvalCaseResult::pass(
                "eval_contract_receipt_allowed_path_produces_evidence",
                "allowed receipt-aware path produces receipt evidence",
                "ApprovedForRun with valid scope now returns receipt evidence alongside the allowed StartRun transition",
            )
        }
        Some(receipt) => SmokeEvalCaseResult::fail(
            "eval_contract_receipt_allowed_path_produces_evidence",
            "allowed receipt-aware path produces receipt evidence",
            format!(
                "receipt evidence drifted; next_state={:?} receipt_id={} contract_ref={} run_id={}",
                receipt_attempt.transition().next_state(),
                receipt.id().as_str(),
                receipt.contract_ref().as_str(),
                receipt.run_id().as_str()
            ),
        ),
        None => SmokeEvalCaseResult::fail(
            "eval_contract_receipt_allowed_path_produces_evidence",
            "allowed receipt-aware path produces receipt evidence",
            "allowed contract-aware StartRun no longer produces receipt evidence",
        ),
    }
}

fn eval_contract_receipt_draft_denial_produces_no_receipt() -> SmokeEvalCaseResult {
    let receipt_attempt = FlowInstance::new(FlowState::Approved)
        .attempt_transition_with_contract_receipt(
            FlowCommand::StartRun,
            ContractStatus::Draft,
            true,
            RunReceiptId::new("receipt_eval_002").expect("receipt id should be valid"),
            ProducedAt::new("2026-04-25T18:55:00Z").expect("produced_at should be valid"),
            ContractRef::new("contract_eval_002").expect("contract ref should be valid"),
            RunId::new("run_eval_002").expect("run id should be valid"),
            RunScopeRef::new("work/goals/goal_add_run_receipt_smoke_eval.md")
                .expect("run scope should be valid"),
        );

    if receipt_attempt.transition().next_state().is_none()
        && receipt_attempt.transition().guard_code()
            == Some("RUN_REQUIRES_APPROVED_FOR_RUN_CONTRACT")
        && receipt_attempt.receipt().is_none()
    {
        SmokeEvalCaseResult::pass(
            "eval_contract_receipt_draft_denial_produces_no_receipt",
            "draft denial still produces no receipt",
            "draft contract denial stays on the guard/event path and does not create receipt evidence",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_contract_receipt_draft_denial_produces_no_receipt",
            "draft denial still produces no receipt",
            format!(
                "draft denial drifted; next_state={:?} guard_code={:?} receipt_present={}",
                receipt_attempt.transition().next_state(),
                receipt_attempt.transition().guard_code(),
                receipt_attempt.receipt().is_some()
            ),
        )
    }
}

fn eval_contract_receipt_invalid_scope_produces_no_receipt() -> SmokeEvalCaseResult {
    let contract = approve_contract(valid_contract_draft()).expect("contract should approve");
    let receipt_attempt = FlowInstance::new(FlowState::Approved)
        .attempt_transition_with_contract_receipt(
            FlowCommand::StartRun,
            contract.status(),
            false,
            RunReceiptId::new("receipt_eval_003").expect("receipt id should be valid"),
            ProducedAt::new("2026-04-25T18:55:00Z").expect("produced_at should be valid"),
            ContractRef::new(contract.id().as_str()).expect("contract ref should be valid"),
            RunId::new("run_eval_003").expect("run id should be valid"),
            RunScopeRef::new("work/goals/goal_add_run_receipt_smoke_eval.md")
                .expect("run scope should be valid"),
        );

    if receipt_attempt.transition().next_state().is_none()
        && receipt_attempt.transition().guard_code() == Some("RUN_REQUIRES_EXPLICIT_CONTRACT_SCOPE")
        && receipt_attempt.receipt().is_none()
    {
        SmokeEvalCaseResult::pass(
            "eval_contract_receipt_invalid_scope_produces_no_receipt",
            "invalid scope denial still produces no receipt",
            "invalid scope keeps StartRun denied and still produces no receipt evidence",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_contract_receipt_invalid_scope_produces_no_receipt",
            "invalid scope denial still produces no receipt",
            format!(
                "invalid scope denial drifted; next_state={:?} guard_code={:?} receipt_present={}",
                receipt_attempt.transition().next_state(),
                receipt_attempt.transition().guard_code(),
                receipt_attempt.receipt().is_some()
            ),
        )
    }
}

fn eval_contract_receipt_remains_pre_gate_evidence() -> SmokeEvalCaseResult {
    let contract = approve_contract(valid_contract_draft()).expect("contract should approve");
    let receipt_attempt = FlowInstance::new(FlowState::Approved)
        .attempt_transition_with_contract_receipt(
            FlowCommand::StartRun,
            contract.status(),
            contract.scope_valid(),
            RunReceiptId::new("receipt_eval_004").expect("receipt id should be valid"),
            ProducedAt::new("2026-04-25T18:55:00Z").expect("produced_at should be valid"),
            ContractRef::new(contract.id().as_str()).expect("contract ref should be valid"),
            RunId::new("run_eval_004").expect("run id should be valid"),
            RunScopeRef::new("work/goals/goal_add_run_receipt_smoke_eval.md")
                .expect("run scope should be valid"),
        );

    match receipt_attempt.receipt() {
        Some(receipt)
            if receipt.run_evidence_only()
                && !receipt.boundary().implies_final_acceptance
                && !receipt.boundary().writes_gate_decision
                && !receipt.boundary().creates_proofpack =>
        {
            SmokeEvalCaseResult::pass(
                "eval_contract_receipt_remains_pre_gate_evidence",
                "receipt remains pre-gate evidence only",
                "receipt evidence remains pre-gate and does not become acceptance, authoritative closure, or proofpack",
            )
        }
        Some(receipt) => SmokeEvalCaseResult::fail(
            "eval_contract_receipt_remains_pre_gate_evidence",
            "receipt remains pre-gate evidence only",
            format!(
                "receipt boundary drifted; acceptance={} gate={} proofpack={}",
                receipt.boundary().implies_final_acceptance,
                receipt.boundary().writes_gate_decision,
                receipt.boundary().creates_proofpack
            ),
        ),
        None => SmokeEvalCaseResult::fail(
            "eval_contract_receipt_remains_pre_gate_evidence",
            "receipt remains pre-gate evidence only",
            "allowed receipt-aware StartRun failed to produce receipt evidence for boundary validation",
        ),
    }
}

fn sample_gate_decision() -> GateDecision {
    GateDecision::new(
        GateDecisionId::new("decision_eval_001").expect("decision id should be valid"),
        GateDecisionOutcome::Accepted,
        vec![GateContractRef::new("contract_eval_001").expect("contract ref should be valid")],
        vec![GateRunReceiptRef::new("receipt_eval_001").expect("receipt ref should be valid")],
        GateCreatedAt::new("2026-04-25T21:00:00Z").expect("created_at should be valid"),
        vec![GateBoundaryNote::new(
            "Decision references evidence; proof is still required before acceptance.",
        )
        .expect("boundary note should be valid")],
    )
    .expect("gate decision should be valid")
    .with_eval_ref(GateEvalRef::new("eval_smoke_gate_proof").expect("eval ref should be valid"))
    .with_event_ref(GateEventRef::new("evt_eval_001").expect("event ref should be valid"))
}

fn sample_proofpack(decision_ref: &str) -> Proofpack {
    sample_proofpack_with_partial_integrity(decision_ref)
        .with_artifact_digest(ProofArtifactDigest::new(
            ProofArtifactKind::RunReceipt,
            ProofArtifactRef::new("receipt_eval_001").expect("artifact ref should be valid"),
            ProofArtifactHash::new("sha256:receipthash").expect("artifact hash should be valid"),
        ))
        .with_artifact_digest(ProofArtifactDigest::new(
            ProofArtifactKind::Eval,
            ProofArtifactRef::new("eval_smoke_gate_proof").expect("artifact ref should be valid"),
            ProofArtifactHash::new("sha256:evalhash").expect("artifact hash should be valid"),
        ))
        .with_artifact_digest(ProofArtifactDigest::new(
            ProofArtifactKind::Event,
            ProofArtifactRef::new("evt_eval_001").expect("artifact ref should be valid"),
            ProofArtifactHash::new("sha256:eventhash").expect("artifact hash should be valid"),
        ))
        .with_artifact_digest(ProofArtifactDigest::new(
            ProofArtifactKind::OutputArtifact,
            ProofArtifactRef::new("target/debug/punk").expect("artifact ref should be valid"),
            ProofArtifactHash::new("sha256:outputhash").expect("artifact hash should be valid"),
        ))
}

fn sample_proofpack_with_partial_integrity(decision_ref: &str) -> Proofpack {
    Proofpack::new(
        ProofpackId::new("proofpack_eval_001").expect("proofpack id should be valid"),
        ProofGateDecisionRef::new(decision_ref).expect("gate decision ref should be valid"),
        vec![ProofContractRef::new("contract_eval_001").expect("contract ref should be valid")],
        vec![ProofRunReceiptRef::new("receipt_eval_001").expect("receipt ref should be valid")],
        ProofCreatedAt::new("2026-04-25T21:01:00Z").expect("created_at should be valid"),
        vec![
            ProofBoundaryNote::new("Proofpack references evidence; gate remains the authority.")
                .expect("boundary note should be valid"),
        ],
    )
    .expect("proofpack should be valid")
    .with_eval_ref(ProofEvalRef::new("eval_smoke_gate_proof").expect("eval ref should be valid"))
    .with_event_ref(ProofEventRef::new("evt_eval_001").expect("event ref should be valid"))
    .with_output_artifact_ref(
        ProofOutputArtifactRef::new("target/debug/punk")
            .expect("output artifact ref should be valid"),
    )
    .with_artifact_digest(ProofArtifactDigest::new(
        ProofArtifactKind::GateDecision,
        ProofArtifactRef::new(decision_ref).expect("artifact ref should be valid"),
        ProofArtifactHash::new("sha256:decisionhash").expect("artifact hash should be valid"),
    ))
    .with_artifact_digest(ProofArtifactDigest::new(
        ProofArtifactKind::Contract,
        ProofArtifactRef::new("contract_eval_001").expect("artifact ref should be valid"),
        ProofArtifactHash::new("sha256:contracthash").expect("artifact hash should be valid"),
    ))
}

fn eval_gate_authority_requires_proof_before_acceptance() -> SmokeEvalCaseResult {
    let decision = sample_gate_decision();
    let boundary = decision.boundary();

    if decision.is_final_decision_authority()
        && decision.requires_proof_for_acceptance_claim()
        && !decision.acceptance_claimable_without_proof()
        && boundary.writes_final_decision
        && !boundary.writes_proofpack
        && !boundary.creates_acceptance_claim
        && !boundary.requires_runtime_storage
        && !boundary.writes_cli_output
    {
        SmokeEvalCaseResult::pass(
            "eval_gate_authority_requires_proof_before_acceptance",
            "gate authority requires proof before acceptance",
            "accepting closure authority still cannot claim acceptance until matching proof exists",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_gate_authority_requires_proof_before_acceptance",
            "gate authority requires proof before acceptance",
            format!(
                "gate boundary drifted; authority={} requires_proof={} claim_without_proof={} proofpack={} acceptance={} storage={} cli={}",
                decision.is_final_decision_authority(),
                decision.requires_proof_for_acceptance_claim(),
                decision.acceptance_claimable_without_proof(),
                boundary.writes_proofpack,
                boundary.creates_acceptance_claim,
                boundary.requires_runtime_storage,
                boundary.writes_cli_output
            ),
        )
    }
}

fn eval_proofpack_is_post_gate_provenance_not_decision() -> SmokeEvalCaseResult {
    let proofpack = sample_proofpack("decision_eval_001");
    let boundary = proofpack.boundary();

    if proofpack.is_post_gate_provenance_bundle()
        && proofpack.references_evidence_without_absorbing()
        && !proofpack.is_final_decision_authority()
        && !proofpack.creates_acceptance_claim()
        && !proofpack.can_claim_acceptance_by_itself()
        && !boundary.writes_proofpack
        && !boundary.writes_final_decision
        && !boundary.requires_runtime_storage
        && !boundary.writes_cli_output
        && !boundary.absorbs_evidence
    {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_is_post_gate_provenance_not_decision",
            "proofpack stays provenance only",
            "proofpack references post-gate evidence without deciding, writing runtime state, or claiming acceptance",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_is_post_gate_provenance_not_decision",
            "proofpack stays provenance only",
            format!(
                "proofpack boundary drifted; post_gate={} final_authority={} writes_proofpack={} writes_decision={} acceptance={} storage={} cli={} absorbs={}",
                proofpack.is_post_gate_provenance_bundle(),
                proofpack.is_final_decision_authority(),
                boundary.writes_proofpack,
                boundary.writes_final_decision,
                boundary.creates_acceptance_claim,
                boundary.requires_runtime_storage,
                boundary.writes_cli_output,
                boundary.absorbs_evidence
            ),
        )
    }
}

fn eval_acceptance_requires_accepting_decision_and_matching_proofpack() -> SmokeEvalCaseResult {
    let decision = sample_gate_decision();
    let matching_decision_ref =
        ProofGateDecisionRef::new(decision.id().as_str()).expect("decision ref should be valid");
    let proofpack = sample_proofpack(decision.id().as_str());
    let mismatch = ProofGateDecisionRef::new("decision_eval_other")
        .expect("other decision ref should be valid");

    let matching = proofpack.matches_gate_decision_ref(&matching_decision_ref);
    let ready_matching = proofpack.is_matching_proof_ready_for_acceptance(&matching_decision_ref);
    let mismatching = proofpack.matches_gate_decision_ref(&mismatch);
    let ready_mismatching = proofpack.is_matching_proof_ready_for_acceptance(&mismatch);
    let acceptance_allowed = positive_acceptance_preconditions_met(PositiveAcceptanceInputs {
        accepting_gate_decision: decision.outcome().is_accepting(),
        matching_proofpack: ready_matching,
    });
    let missing_decision_blocked =
        !positive_acceptance_preconditions_met(PositiveAcceptanceInputs {
            accepting_gate_decision: false,
            matching_proofpack: ready_matching,
        });
    let missing_proof_blocked = !positive_acceptance_preconditions_met(PositiveAcceptanceInputs {
        accepting_gate_decision: decision.outcome().is_accepting(),
        matching_proofpack: false,
    });

    if matching
        && ready_matching
        && !mismatching
        && !ready_mismatching
        && acceptance_allowed
        && missing_decision_blocked
        && missing_proof_blocked
    {
        SmokeEvalCaseResult::pass(
            "eval_acceptance_requires_accepting_decision_and_matching_proofpack",
            "positive acceptance requires matching decision and proof",
            "acceptance preconditions pass only when accepting authority and matching proofpack are both present",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_acceptance_requires_accepting_decision_and_matching_proofpack",
            "positive acceptance requires matching decision and proof",
            format!(
                "acceptance preconditions drifted; matching={matching} ready_matching={ready_matching} mismatching={mismatching} ready_mismatching={ready_mismatching} allowed={acceptance_allowed} missing_decision_blocked={missing_decision_blocked} missing_proof_blocked={missing_proof_blocked}"
            ),
        )
    }
}

fn eval_proofpack_integrity_ready_when_declared_digest_links_are_complete() -> SmokeEvalCaseResult {
    let decision = sample_gate_decision();
    let matching_decision_ref =
        ProofGateDecisionRef::new(decision.id().as_str()).expect("decision ref should be valid");
    let proofpack = sample_proofpack(decision.id().as_str());
    let integrity = proofpack.link_hash_integrity_report();
    let boundary = proofpack.boundary();

    if integrity.required_digest_refs().len() == 6
        && integrity.is_complete()
        && !integrity.has_missing_required_digests()
        && proofpack.has_complete_link_hash_integrity()
        && proofpack.is_matching_proof_ready_for_acceptance(&matching_decision_ref)
        && boundary.checks_structural_link_hash_integrity
        && !boundary.computes_hashes
        && !boundary.normalizes_hashes
    {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_integrity_ready_when_declared_digest_links_are_complete",
            "proofpack integrity readiness requires complete declared digest links",
            "all declared decision, contract, receipt, eval, event, and output refs have structural digest links without hash computation",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_integrity_ready_when_declared_digest_links_are_complete",
            "proofpack integrity readiness requires complete declared digest links",
            format!(
                "proofpack integrity drifted; required={} missing={} complete={} ready={} structural={} computes_hashes={} normalizes_hashes={}",
                integrity.required_digest_refs().len(),
                integrity.missing_digest_refs().len(),
                integrity.is_complete(),
                proofpack.is_matching_proof_ready_for_acceptance(&matching_decision_ref),
                boundary.checks_structural_link_hash_integrity,
                boundary.computes_hashes,
                boundary.normalizes_hashes
            ),
        )
    }
}

fn eval_proofpack_integrity_missing_digest_blocks_readiness() -> SmokeEvalCaseResult {
    let decision = sample_gate_decision();
    let matching_decision_ref =
        ProofGateDecisionRef::new(decision.id().as_str()).expect("decision ref should be valid");
    let proofpack = sample_proofpack_with_partial_integrity(decision.id().as_str());
    let integrity = proofpack.link_hash_integrity_report();
    let missing_kinds = integrity
        .missing_digest_refs()
        .iter()
        .map(|missing| missing.kind())
        .collect::<Vec<_>>();

    let missing_receipt = missing_kinds.contains(&ProofArtifactKind::RunReceipt);
    let missing_eval = missing_kinds.contains(&ProofArtifactKind::Eval);
    let missing_event = missing_kinds.contains(&ProofArtifactKind::Event);
    let missing_output = missing_kinds.contains(&ProofArtifactKind::OutputArtifact);

    if integrity.required_digest_refs().len() == 6
        && integrity.missing_digest_refs().len() == 4
        && integrity.has_missing_required_digests()
        && !integrity.is_complete()
        && !proofpack.has_complete_link_hash_integrity()
        && !proofpack.is_matching_proof_ready_for_acceptance(&matching_decision_ref)
        && missing_receipt
        && missing_eval
        && missing_event
        && missing_output
    {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_integrity_missing_digest_blocks_readiness",
            "missing proofpack digests block proof readiness",
            "missing required receipt, eval, event, and output digest links remain visible and keep matching proof readiness false",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_integrity_missing_digest_blocks_readiness",
            "missing proofpack digests block proof readiness",
            format!(
                "missing digest handling drifted; required={} missing={} complete={} ready={} missing_receipt={missing_receipt} missing_eval={missing_eval} missing_event={missing_event} missing_output={missing_output}",
                integrity.required_digest_refs().len(),
                integrity.missing_digest_refs().len(),
                integrity.is_complete(),
                proofpack.is_matching_proof_ready_for_acceptance(&matching_decision_ref)
            ),
        )
    }
}

fn valid_artifact_digest() -> String {
    format!("sha256:{}", "0123456789abcdef".repeat(4))
}

fn eval_artifact_hash_policy_accepts_canonical_digest() -> SmokeEvalCaseResult {
    let digest_value = valid_artifact_digest();
    let digest = ArtifactDigest::new(digest_value.clone());

    match digest {
        Ok(digest)
            if digest.as_str() == digest_value
                && is_canonical_artifact_digest(digest.as_str())
                && validate_artifact_digest(digest.as_str()).is_ok()
                && ARTIFACT_HASH_POLICY_VERSION == "artifact-hash-policy.v0.1" =>
        {
            SmokeEvalCaseResult::pass(
                "eval_artifact_hash_policy_accepts_canonical_digest",
                "canonical artifact digest shape remains valid",
                "sha256-prefixed lowercase 64-hex digest matches valid helper-level policy shape",
            )
        }
        Ok(digest) => SmokeEvalCaseResult::fail(
            "eval_artifact_hash_policy_accepts_canonical_digest",
            "canonical artifact digest shape remains valid",
            format!(
                "canonical digest helper drifted; digest={} is_canonical={} version={}",
                digest.as_str(),
                is_canonical_artifact_digest(digest.as_str()),
                ARTIFACT_HASH_POLICY_VERSION
            ),
        ),
        Err(error) => SmokeEvalCaseResult::fail(
            "eval_artifact_hash_policy_accepts_canonical_digest",
            "canonical artifact digest shape remains valid",
            format!("canonical digest was rejected with {error:?}"),
        ),
    }
}

fn eval_artifact_hash_policy_rejects_invalid_digest() -> SmokeEvalCaseResult {
    let bare_digest = "0123456789abcdef".repeat(4);
    let uppercase_digest = format!("sha256:{}", "ABCDEF0123456789".repeat(4));

    let placeholder_rejected =
        validate_artifact_digest("unknown") == Err(ArtifactHashPolicyError::PlaceholderDigest);
    let bare_rejected =
        validate_artifact_digest(&bare_digest) == Err(ArtifactHashPolicyError::BareDigest);
    let uppercase_rejected = validate_artifact_digest(&uppercase_digest)
        == Err(ArtifactHashPolicyError::InvalidDigestHex);
    let short_rejected = validate_artifact_digest("sha256:abc")
        == Err(ArtifactHashPolicyError::InvalidDigestLength {
            expected: 64,
            actual: 3,
        });

    if placeholder_rejected && bare_rejected && uppercase_rejected && short_rejected {
        SmokeEvalCaseResult::pass(
            "eval_artifact_hash_policy_rejects_invalid_digest",
            "invalid artifact digest shapes remain rejected",
            "placeholder, bare, uppercase, and short digests remain rejected by helper validation",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_artifact_hash_policy_rejects_invalid_digest",
            "invalid artifact digest shapes remain rejected",
            format!(
                "invalid digest rejection drifted; placeholder={placeholder_rejected} bare={bare_rejected} uppercase={uppercase_rejected} short={short_rejected}"
            ),
        )
    }
}

fn eval_artifact_hash_policy_accepts_repo_relative_ref() -> SmokeEvalCaseResult {
    let ref_value = "work/reports/2026-04-26-artifact-hash-policy-smoke-eval-coverage.md";
    let artifact_ref = RepoRelativeArtifactRef::new(ref_value);

    match artifact_ref {
        Ok(artifact_ref)
            if artifact_ref.as_str() == ref_value
                && is_valid_repo_relative_artifact_ref(artifact_ref.as_str())
                && validate_repo_relative_artifact_ref(artifact_ref.as_str()).is_ok() =>
        {
            SmokeEvalCaseResult::pass(
                "eval_artifact_hash_policy_accepts_repo_relative_ref",
                "repo-relative artifact refs remain valid",
                "repo-relative work report path matches valid helper-level policy shape",
            )
        }
        Ok(artifact_ref) => SmokeEvalCaseResult::fail(
            "eval_artifact_hash_policy_accepts_repo_relative_ref",
            "repo-relative artifact refs remain valid",
            format!(
                "repo-relative artifact ref helper drifted; ref={} valid={}",
                artifact_ref.as_str(),
                is_valid_repo_relative_artifact_ref(artifact_ref.as_str())
            ),
        ),
        Err(error) => SmokeEvalCaseResult::fail(
            "eval_artifact_hash_policy_accepts_repo_relative_ref",
            "repo-relative artifact refs remain valid",
            format!("repo-relative artifact ref was rejected with {error:?}"),
        ),
    }
}

fn eval_artifact_hash_policy_rejects_invalid_ref() -> SmokeEvalCaseResult {
    let absolute_rejected = validate_repo_relative_artifact_ref("/tmp/events.jsonl")
        == Err(ArtifactHashPolicyError::AbsoluteArtifactRef);
    let parent_rejected = validate_repo_relative_artifact_ref("../events.jsonl")
        == Err(ArtifactHashPolicyError::InvalidArtifactRefSegment);
    let url_rejected = validate_repo_relative_artifact_ref("https://example.com/events.jsonl")
        == Err(ArtifactHashPolicyError::UrlArtifactRef);
    let backslash_rejected = validate_repo_relative_artifact_ref("work\\reports\\events.jsonl")
        == Err(ArtifactHashPolicyError::BackslashArtifactRef);

    if absolute_rejected && parent_rejected && url_rejected && backslash_rejected {
        SmokeEvalCaseResult::pass(
            "eval_artifact_hash_policy_rejects_invalid_ref",
            "invalid artifact refs remain rejected",
            "absolute, parent traversal, URL, and backslash refs remain rejected by helper validation",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_artifact_hash_policy_rejects_invalid_ref",
            "invalid artifact refs remain rejected",
            format!(
                "invalid ref rejection drifted; absolute={absolute_rejected} parent={parent_rejected} url={url_rejected} backslash={backslash_rejected}"
            ),
        )
    }
}

fn eval_artifact_hash_policy_helper_boundary_flags() -> SmokeEvalCaseResult {
    let capabilities = ARTIFACT_HASH_POLICY_CAPABILITIES;

    if capabilities.validates_digest_format
        && capabilities.validates_repo_relative_refs
        && !capabilities.computes_hashes
        && !capabilities.normalizes_artifact_bytes
        && !capabilities.writes_runtime_state
    {
        SmokeEvalCaseResult::pass(
            "eval_artifact_hash_policy_helper_boundary_flags",
            "artifact hash policy helpers stay validation-only",
            "helper capabilities still validate digest/ref shape without hash computation, byte normalization, or runtime writes",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_artifact_hash_policy_helper_boundary_flags",
            "artifact hash policy helpers stay validation-only",
            format!(
                "artifact hash helper boundary drifted; digest={} refs={} computes={} normalizes={} writes={}",
                capabilities.validates_digest_format,
                capabilities.validates_repo_relative_refs,
                capabilities.computes_hashes,
                capabilities.normalizes_artifact_bytes,
                capabilities.writes_runtime_state
            ),
        )
    }
}

fn format_commands(commands: &[FlowCommand]) -> String {
    commands
        .iter()
        .map(|command| command.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::{
        run_smoke_suite, SmokeEvalCaseResult, SmokeEvalReport, SmokeEvalStatus, SmokeEvalSummary,
        SMOKE_REPORT_SCHEMA_VERSION, SMOKE_SUITE_ID,
    };

    #[test]
    fn smoke_report_contains_expected_summary_and_cases() {
        let report = run_smoke_suite();

        assert_eq!(report.suite_id(), SMOKE_SUITE_ID);
        assert_eq!(report.smoke_result(), SmokeEvalStatus::Pass);
        assert_eq!(report.mode(), "local-smoke-check");
        assert_eq!(report.runtime_persistence(), "inactive");
        assert_eq!(report.report_storage(), "inactive");
        assert_eq!(report.cases().len(), 24);
    }

    #[test]
    fn smoke_report_passes_against_current_flow_and_event_kernels() {
        let report = run_smoke_suite();

        assert!(report.passed(), "all current smoke cases should pass");
        assert!(report
            .cases()
            .iter()
            .all(|case| case.status == SmokeEvalStatus::Pass));
    }

    #[test]
    fn smoke_report_renders_human_output_with_boundaries_and_deferred_notes() {
        let report = run_smoke_suite();
        let rendered = report.render_human();

        assert!(rendered.contains("punk eval run smoke"));
        assert!(rendered.contains("mode: local-smoke-check"));
        assert!(rendered.contains("runtime_persistence: inactive"));
        assert!(rendered.contains("report_storage: inactive"));
        assert!(rendered.contains("smoke_result: pass"));
        assert!(rendered.contains(
            "assessment: local deterministic smoke harness passed over current contract, flow, receipt, event, gate, proof, and artifact hash policy helper kernels"
        ));
        assert!(rendered.contains("case_results:"));
        assert!(rendered.contains("  - id: eval_flow_allows_approval_transition"));
        assert!(rendered.contains("  - id: eval_contract_ready_for_bounded_work_allows_start_run"));
        assert!(rendered.contains("  - id: eval_contract_receipt_allowed_path_produces_evidence"));
        assert!(rendered.contains("  - id: eval_gate_authority_requires_proof_before_acceptance"));
        assert!(rendered.contains("  - id: eval_proofpack_is_post_gate_provenance_not_decision"));
        assert!(rendered.contains(
            "  - id: eval_acceptance_requires_accepting_decision_and_matching_proofpack"
        ));
        assert!(rendered.contains(
            "  - id: eval_proofpack_integrity_ready_when_declared_digest_links_are_complete"
        ));
        assert!(
            rendered.contains("  - id: eval_proofpack_integrity_missing_digest_blocks_readiness")
        );
        assert!(rendered.contains("  - id: eval_artifact_hash_policy_accepts_canonical_digest"));
        assert!(rendered.contains("  - id: eval_artifact_hash_policy_rejects_invalid_digest"));
        assert!(rendered.contains("  - id: eval_artifact_hash_policy_accepts_repo_relative_ref"));
        assert!(rendered.contains("  - id: eval_artifact_hash_policy_rejects_invalid_ref"));
        assert!(rendered.contains("  - id: eval_artifact_hash_policy_helper_boundary_flags"));
        assert!(rendered.contains("    status: pass"));
        assert!(rendered.contains("notes:"));
        assert!(rendered.contains("local assessment only; no authority is written here"));
        assert!(rendered
            .contains("run receipt evidence remains pre-gate and does not imply final acceptance"));
        assert!(rendered.contains(
            "gate/proof smoke cases remain local assessment and do not claim acceptance"
        ));
        assert!(rendered.contains(
            "artifact hash policy smoke cases validate helper shape only and do not compute hashes"
        ));
        assert!(rendered
            .contains("JSON output is opt-in only and does not imply a stable public contract"));
        assert!(rendered.contains("deferred:"));
        assert!(rendered.contains("baseline, waiver, and stored eval reports are not active"));
        assert!(rendered
            .contains("active artifact hash computation and byte normalization are not active"));
    }

    #[test]
    fn smoke_report_uses_assessment_not_decision_wording() {
        let report = run_smoke_suite();
        let rendered = report.render_human();

        assert!(report
            .assessment()
            .contains("local deterministic smoke harness"));
        assert!(!rendered.contains("accepted"));
        assert!(!rendered.contains("approved_final"));
        assert!(!rendered.contains("gate decision"));
        assert!(!rendered.contains("proof complete"));
        assert!(!rendered.contains("final decision"));
    }

    #[test]
    fn smoke_report_renders_json_output_with_v0_1_shape() {
        let report = run_smoke_suite();
        let rendered = report.render_json();

        assert!(rendered.starts_with("{\n"));
        assert!(rendered.contains(&format!(
            "\"schema_version\": \"{SMOKE_REPORT_SCHEMA_VERSION}\""
        )));
        assert!(rendered.contains("\"suite_id\": \"smoke.v0\""));
        assert!(rendered.contains("\"run_id\": null"));
        assert!(rendered.contains("\"smoke_result\": \"pass\""));
        assert!(rendered.contains("\"mode\": \"local-smoke-check\""));
        assert!(rendered.contains("\"runtime_persistence\": \"inactive\""));
        assert!(rendered.contains("\"report_storage\": \"inactive\""));
        assert!(rendered.contains("\"case_results\": ["));
        assert!(rendered.contains("\"case_id\": \"eval_flow_allows_approval_transition\""));
        assert!(rendered
            .contains("\"case_id\": \"eval_contract_ready_for_bounded_work_allows_start_run\""));
        assert!(rendered
            .contains("\"case_id\": \"eval_contract_receipt_allowed_path_produces_evidence\""));
        assert!(rendered
            .contains("\"case_id\": \"eval_gate_authority_requires_proof_before_acceptance\""));
        assert!(rendered
            .contains("\"case_id\": \"eval_proofpack_is_post_gate_provenance_not_decision\""));
        assert!(rendered.contains(
            "\"case_id\": \"eval_acceptance_requires_accepting_decision_and_matching_proofpack\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_proofpack_integrity_ready_when_declared_digest_links_are_complete\""
        ));
        assert!(rendered
            .contains("\"case_id\": \"eval_proofpack_integrity_missing_digest_blocks_readiness\""));
        assert!(rendered
            .contains("\"case_id\": \"eval_artifact_hash_policy_accepts_canonical_digest\""));
        assert!(
            rendered.contains("\"case_id\": \"eval_artifact_hash_policy_rejects_invalid_digest\"")
        );
        assert!(rendered
            .contains("\"case_id\": \"eval_artifact_hash_policy_accepts_repo_relative_ref\""));
        assert!(rendered.contains("\"case_id\": \"eval_artifact_hash_policy_rejects_invalid_ref\""));
        assert!(
            rendered.contains("\"case_id\": \"eval_artifact_hash_policy_helper_boundary_flags\"")
        );
        assert!(rendered.contains("\"status\": \"pass\""));
        assert!(rendered.contains("\"boundary_notes\": ["));
        assert!(rendered.contains("\"deferred\": ["));
    }

    #[test]
    fn smoke_report_json_escapes_quotes_backslashes_and_newlines() {
        let report = SmokeEvalReport {
            summary: SmokeEvalSummary {
                suite_id: SMOKE_SUITE_ID,
                smoke_result: SmokeEvalStatus::Pass,
                assessment: "hello \"punk\"\npath\\to\\file".to_owned(),
                mode: "local-smoke-check",
                runtime_persistence: "inactive",
                report_storage: "inactive",
            },
            cases: vec![SmokeEvalCaseResult::pass(
                "case_quotes",
                "line\nbreak",
                "hello \"punk\"\npath\\to\\file",
            )],
            boundary_notes: vec!["line\nbreak"],
            deferred_notes: vec!["path\\to\\file"],
        };

        let rendered = report.render_json();

        assert!(rendered.contains("hello \\\"punk\\\"\\npath\\\\to\\\\file"));
        assert!(rendered.contains("\"summary\": \"line\\nbreak\""));
        assert!(rendered.contains("\"deferred\": [\"path\\\\to\\\\file\"]"));
    }
}
