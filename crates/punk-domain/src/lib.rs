//! Minimal run receipt kernel for Punk Phase 3.
//!
//! This crate intentionally stays deterministic and side-effect free in this
//! first implementation step. It models pre-gate run receipt evidence only.
//! No `.punk/` storage, CLI behavior, gate decisions, proofpacks, or run
//! execution are introduced here.

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const RUN_RECEIPT_SCHEMA_VERSION: &str = "punk.run_receipt.v0.1";

fn non_empty(value: impl Into<String>, error: RunReceiptError) -> Result<String, RunReceiptError> {
    let value = value.into().trim().to_string();

    if value.is_empty() {
        return Err(error);
    }

    Ok(value)
}

fn optional_non_empty(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let value = value.trim().to_string();
        if value.is_empty() {
            None
        } else {
            Some(value)
        }
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProducedAt(String);

impl ProducedAt {
    pub fn new(value: impl Into<String>) -> Result<Self, RunReceiptError> {
        Ok(Self(non_empty(value, RunReceiptError::EmptyProducedAt)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoundaryNote(String);

impl BoundaryNote {
    pub fn new(value: impl Into<String>) -> Result<Self, RunReceiptError> {
        Ok(Self(non_empty(value, RunReceiptError::EmptyBoundaryNote)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkGoalRef(String);

impl WorkGoalRef {
    pub fn new(value: impl Into<String>) -> Result<Self, RunReceiptError> {
        Ok(Self(non_empty(value, RunReceiptError::EmptyWorkGoalRef)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlowStateRef(String);

impl FlowStateRef {
    pub fn new(value: impl Into<String>) -> Result<Self, RunReceiptError> {
        Ok(Self(non_empty(value, RunReceiptError::EmptyFlowStateRef)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RunReceiptId(String);

impl RunReceiptId {
    pub fn new(value: impl Into<String>) -> Result<Self, RunReceiptError> {
        let value = value.into().trim().to_string();

        if value.is_empty() {
            return Err(RunReceiptError::EmptyReceiptId);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContractRef(String);

impl ContractRef {
    pub fn new(value: impl Into<String>) -> Result<Self, RunReceiptError> {
        let value = value.into().trim().to_string();

        if value.is_empty() {
            return Err(RunReceiptError::EmptyContractRef);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RunId(String);

impl RunId {
    pub fn new(value: impl Into<String>) -> Result<Self, RunReceiptError> {
        let value = value.into().trim().to_string();

        if value.is_empty() {
            return Err(RunReceiptError::EmptyRunId);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RunScopeRef(String);

impl RunScopeRef {
    pub fn new(value: impl Into<String>) -> Result<Self, RunReceiptError> {
        let value = value.into().trim().to_string();

        if value.is_empty() {
            return Err(RunReceiptError::EmptyRunScopeRef);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RunArtifactRef(String);

impl RunArtifactRef {
    pub fn new(value: impl Into<String>) -> Result<Self, RunReceiptError> {
        let value = value.into().trim().to_string();

        if value.is_empty() {
            return Err(RunReceiptError::EmptyArtifactRef);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventRef(String);

impl EventRef {
    pub fn new(value: impl Into<String>) -> Result<Self, RunReceiptError> {
        let value = value.into().trim().to_string();

        if value.is_empty() {
            return Err(RunReceiptError::EmptyEventRef);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EvalReportRef(String);

impl EvalReportRef {
    pub fn new(value: impl Into<String>) -> Result<Self, RunReceiptError> {
        let value = value.into().trim().to_string();

        if value.is_empty() {
            return Err(RunReceiptError::EmptyEvalReportRef);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidatorRequiredness {
    Required,
    Advisory,
    Conditional,
}

impl ValidatorRequiredness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Advisory => "advisory",
            Self::Conditional => "conditional",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidatorOutcome {
    Passed,
    Failed,
    Unavailable,
    Skipped,
    Unsupported,
    Deferred,
}

impl ValidatorOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Unavailable => "unavailable",
            Self::Skipped => "skipped",
            Self::Unsupported => "unsupported",
            Self::Deferred => "deferred",
        }
    }

    pub fn is_passing(self) -> bool {
        matches!(self, Self::Passed)
    }

    pub fn is_missing_evidence_gap(self) -> bool {
        matches!(
            self,
            Self::Unavailable | Self::Skipped | Self::Unsupported | Self::Deferred
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatorEvidence {
    validator_id: String,
    validator_kind: String,
    requiredness: ValidatorRequiredness,
    outcome: ValidatorOutcome,
    scope_ref: String,
    reason_code: Option<String>,
    summary: String,
    evidence_ref: Option<String>,
    observed_at: String,
    follow_up_ref: Option<String>,
}

impl ValidatorEvidence {
    pub fn new(
        validator_id: impl Into<String>,
        validator_kind: impl Into<String>,
        requiredness: ValidatorRequiredness,
        outcome: ValidatorOutcome,
        scope_ref: impl Into<String>,
        reason_code: Option<String>,
        summary: impl Into<String>,
        evidence_ref: Option<String>,
        observed_at: impl Into<String>,
        follow_up_ref: Option<String>,
    ) -> Result<Self, RunReceiptError> {
        let reason_code = optional_non_empty(reason_code);

        if !outcome.is_passing() && reason_code.is_none() {
            return Err(RunReceiptError::MissingValidatorReasonCode);
        }

        Ok(Self {
            validator_id: non_empty(validator_id, RunReceiptError::EmptyValidatorId)?,
            validator_kind: non_empty(validator_kind, RunReceiptError::EmptyValidatorKind)?,
            requiredness,
            outcome,
            scope_ref: non_empty(scope_ref, RunReceiptError::EmptyValidatorScopeRef)?,
            reason_code,
            summary: non_empty(summary, RunReceiptError::EmptyValidatorSummary)?,
            evidence_ref: optional_non_empty(evidence_ref),
            observed_at: non_empty(observed_at, RunReceiptError::EmptyValidatorObservedAt)?,
            follow_up_ref: optional_non_empty(follow_up_ref),
        })
    }

    pub fn validator_id(&self) -> &str {
        &self.validator_id
    }

    pub fn validator_kind(&self) -> &str {
        &self.validator_kind
    }

    pub fn requiredness(&self) -> ValidatorRequiredness {
        self.requiredness
    }

    pub fn outcome(&self) -> ValidatorOutcome {
        self.outcome
    }

    pub fn scope_ref(&self) -> &str {
        &self.scope_ref
    }

    pub fn reason_code(&self) -> Option<&str> {
        self.reason_code.as_deref()
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn evidence_ref(&self) -> Option<&str> {
        self.evidence_ref.as_deref()
    }

    pub fn observed_at(&self) -> &str {
        &self.observed_at
    }

    pub fn follow_up_ref(&self) -> Option<&str> {
        self.follow_up_ref.as_deref()
    }

    pub fn is_required_missing_evidence_gap(&self) -> bool {
        self.requiredness == ValidatorRequiredness::Required
            && self.outcome.is_missing_evidence_gap()
    }

    pub fn blocks_proof_claim(&self) -> bool {
        self.requiredness == ValidatorRequiredness::Required && !self.outcome.is_passing()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunReceipt {
    id: RunReceiptId,
    schema_version: &'static str,
    produced_at: ProducedAt,
    contract_ref: ContractRef,
    run_id: RunId,
    run_scope_ref: RunScopeRef,
    work_goal_ref: Option<WorkGoalRef>,
    flow_state_before: Option<FlowStateRef>,
    flow_state_after: Option<FlowStateRef>,
    produced_artifact_refs: Vec<RunArtifactRef>,
    event_refs: Vec<EventRef>,
    eval_report_refs: Vec<EvalReportRef>,
    validator_evidence: Vec<ValidatorEvidence>,
    boundary_notes: Vec<BoundaryNote>,
}

impl RunReceipt {
    pub fn new(
        id: RunReceiptId,
        produced_at: ProducedAt,
        contract_ref: ContractRef,
        run_id: RunId,
        run_scope_ref: RunScopeRef,
    ) -> Self {
        Self {
            id,
            schema_version: RUN_RECEIPT_SCHEMA_VERSION,
            produced_at,
            contract_ref,
            run_id,
            run_scope_ref,
            work_goal_ref: None,
            flow_state_before: None,
            flow_state_after: None,
            produced_artifact_refs: Vec::new(),
            event_refs: Vec::new(),
            eval_report_refs: Vec::new(),
            validator_evidence: Vec::new(),
            boundary_notes: Vec::new(),
        }
    }

    pub fn with_work_goal_ref(mut self, work_goal_ref: WorkGoalRef) -> Self {
        self.work_goal_ref = Some(work_goal_ref);
        self
    }

    pub fn with_flow_state_before(mut self, flow_state_before: FlowStateRef) -> Self {
        self.flow_state_before = Some(flow_state_before);
        self
    }

    pub fn with_flow_state_after(mut self, flow_state_after: FlowStateRef) -> Self {
        self.flow_state_after = Some(flow_state_after);
        self
    }

    pub fn with_artifact_ref(mut self, artifact_ref: RunArtifactRef) -> Self {
        self.produced_artifact_refs.push(artifact_ref);
        self
    }

    pub fn with_event_ref(mut self, event_ref: EventRef) -> Self {
        self.event_refs.push(event_ref);
        self
    }

    pub fn with_eval_report_ref(mut self, eval_report_ref: EvalReportRef) -> Self {
        self.eval_report_refs.push(eval_report_ref);
        self
    }

    pub fn with_validator_evidence(mut self, validator_evidence: ValidatorEvidence) -> Self {
        self.validator_evidence.push(validator_evidence);
        self
    }

    pub fn with_boundary_note(mut self, boundary_note: BoundaryNote) -> Self {
        self.boundary_notes.push(boundary_note);
        self
    }

    pub fn id(&self) -> &RunReceiptId {
        &self.id
    }

    pub fn schema_version(&self) -> &str {
        self.schema_version
    }

    pub fn produced_at(&self) -> &ProducedAt {
        &self.produced_at
    }

    pub fn contract_ref(&self) -> &ContractRef {
        &self.contract_ref
    }

    pub fn run_id(&self) -> &RunId {
        &self.run_id
    }

    pub fn run_scope_ref(&self) -> &RunScopeRef {
        &self.run_scope_ref
    }

    pub fn work_goal_ref(&self) -> Option<&WorkGoalRef> {
        self.work_goal_ref.as_ref()
    }

    pub fn flow_state_before(&self) -> Option<&FlowStateRef> {
        self.flow_state_before.as_ref()
    }

    pub fn flow_state_after(&self) -> Option<&FlowStateRef> {
        self.flow_state_after.as_ref()
    }

    pub fn produced_artifact_refs(&self) -> &[RunArtifactRef] {
        &self.produced_artifact_refs
    }

    pub fn event_refs(&self) -> &[EventRef] {
        &self.event_refs
    }

    pub fn eval_report_refs(&self) -> &[EvalReportRef] {
        &self.eval_report_refs
    }

    pub fn validator_evidence(&self) -> &[ValidatorEvidence] {
        &self.validator_evidence
    }

    pub fn boundary_notes(&self) -> &[BoundaryNote] {
        &self.boundary_notes
    }

    pub fn boundary(&self) -> RunReceiptBoundary {
        run_receipt_boundary()
    }

    pub fn has_required_validator_evidence_gaps(&self) -> bool {
        self.validator_evidence
            .iter()
            .any(ValidatorEvidence::is_required_missing_evidence_gap)
    }

    pub fn has_required_validator_blockers(&self) -> bool {
        self.validator_evidence
            .iter()
            .any(ValidatorEvidence::blocks_proof_claim)
    }

    pub fn run_evidence_only(&self) -> bool {
        let boundary = self.boundary();

        !boundary.implies_final_acceptance
            && !boundary.writes_gate_decision
            && !boundary.creates_proofpack
            && !boundary.hides_missing_validator_gaps
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunReceiptError {
    EmptyReceiptId,
    EmptyProducedAt,
    EmptyContractRef,
    EmptyRunId,
    EmptyRunScopeRef,
    EmptyWorkGoalRef,
    EmptyFlowStateRef,
    EmptyArtifactRef,
    EmptyEventRef,
    EmptyEvalReportRef,
    EmptyBoundaryNote,
    EmptyValidatorId,
    EmptyValidatorKind,
    EmptyValidatorScopeRef,
    EmptyValidatorSummary,
    EmptyValidatorObservedAt,
    MissingValidatorReasonCode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunReceiptBoundary {
    pub implies_final_acceptance: bool,
    pub writes_gate_decision: bool,
    pub creates_proofpack: bool,
    pub requires_runtime_storage: bool,
    pub denied_transition_creates_receipt: bool,
    pub hides_missing_validator_gaps: bool,
}

pub const fn run_receipt_boundary() -> RunReceiptBoundary {
    RunReceiptBoundary {
        implies_final_acceptance: false,
        writes_gate_decision: false,
        creates_proofpack: false,
        requires_runtime_storage: false,
        denied_transition_creates_receipt: false,
        hides_missing_validator_gaps: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_receipt() -> RunReceipt {
        RunReceipt::new(
            RunReceiptId::new("receipt_local_001").expect("receipt id should be valid"),
            ProducedAt::new("2026-04-25T18:45:00Z").expect("produced_at should be valid"),
            ContractRef::new("contract_local_001").expect("contract ref should be valid"),
            RunId::new("run_local_001").expect("run id should be valid"),
            RunScopeRef::new("work/goals/goal_add_run_receipt_minimal.md")
                .expect("run scope ref should be valid"),
        )
        .with_work_goal_ref(
            WorkGoalRef::new("work/goals/goal_extend_run_receipt_kernel_minimal_fields_v0_1.md")
                .expect("work goal ref should be valid"),
        )
        .with_flow_state_before(FlowStateRef::new("Approved").expect("state should be valid"))
        .with_flow_state_after(FlowStateRef::new("Running").expect("state should be valid"))
        .with_artifact_ref(
            RunArtifactRef::new("work/reports/2026-04-22-run-receipt-minimal.md")
                .expect("artifact ref should be valid"),
        )
        .with_event_ref(EventRef::new("evt_0000000000000001").expect("event ref should be valid"))
        .with_eval_report_ref(
            EvalReportRef::new("work/reports/2026-04-22-contract-flow-smoke-eval.md")
                .expect("eval report ref should be valid"),
        )
        .with_validator_evidence(required_unavailable_validator())
        .with_boundary_note(
            BoundaryNote::new("Receipt is evidence only; missing validators stay visible.")
                .expect("boundary note should be valid"),
        )
    }

    fn required_unavailable_validator() -> ValidatorEvidence {
        ValidatorEvidence::new(
            "cargo-test",
            "command",
            ValidatorRequiredness::Required,
            ValidatorOutcome::Unavailable,
            "crates/**",
            Some("COMMAND_NOT_FOUND".to_string()),
            "cargo test could not run in the target environment",
            None,
            "2026-04-25T18:45:00Z",
            Some("work/goals/goal_extend_run_receipt_kernel_minimal_fields_v0_1.md".to_string()),
        )
        .expect("validator evidence should be valid")
    }

    #[test]
    fn receipt_can_be_created_for_bounded_run_with_refs() {
        let receipt = sample_receipt();

        assert_eq!(receipt.id().as_str(), "receipt_local_001");
        assert_eq!(receipt.schema_version(), RUN_RECEIPT_SCHEMA_VERSION);
        assert_eq!(receipt.produced_at().as_str(), "2026-04-25T18:45:00Z");
        assert_eq!(receipt.contract_ref().as_str(), "contract_local_001");
        assert_eq!(receipt.run_id().as_str(), "run_local_001");
        assert_eq!(
            receipt.run_scope_ref().as_str(),
            "work/goals/goal_add_run_receipt_minimal.md"
        );
        assert_eq!(
            receipt
                .work_goal_ref()
                .expect("work goal ref should be present")
                .as_str(),
            "work/goals/goal_extend_run_receipt_kernel_minimal_fields_v0_1.md"
        );
        assert_eq!(
            receipt
                .flow_state_before()
                .expect("before state should be present")
                .as_str(),
            "Approved"
        );
        assert_eq!(
            receipt
                .flow_state_after()
                .expect("after state should be present")
                .as_str(),
            "Running"
        );
        assert_eq!(receipt.produced_artifact_refs().len(), 1);
        assert_eq!(receipt.event_refs().len(), 1);
        assert_eq!(receipt.eval_report_refs().len(), 1);
        assert_eq!(receipt.validator_evidence().len(), 1);
        assert_eq!(receipt.boundary_notes().len(), 1);
    }

    #[test]
    fn receipt_requires_non_empty_receipt_id() {
        let error = RunReceiptId::new("   ").expect_err("empty receipt id must be rejected");

        assert_eq!(error, RunReceiptError::EmptyReceiptId);
    }

    #[test]
    fn receipt_requires_produced_at() {
        let error = ProducedAt::new("   ").expect_err("empty produced_at must be rejected");

        assert_eq!(error, RunReceiptError::EmptyProducedAt);
    }

    #[test]
    fn receipt_requires_contract_ref() {
        let error = ContractRef::new("").expect_err("empty contract ref must be rejected");

        assert_eq!(error, RunReceiptError::EmptyContractRef);
    }

    #[test]
    fn receipt_requires_run_id() {
        let error = RunId::new(" ").expect_err("empty run id must be rejected");

        assert_eq!(error, RunReceiptError::EmptyRunId);
    }

    #[test]
    fn receipt_requires_scope_ref() {
        let error = RunScopeRef::new(" ").expect_err("empty scope ref must be rejected");

        assert_eq!(error, RunReceiptError::EmptyRunScopeRef);
    }

    #[test]
    fn receipt_does_not_imply_final_acceptance() {
        let receipt = sample_receipt();

        assert!(receipt.run_evidence_only());
        assert!(!receipt.boundary().implies_final_acceptance);
    }

    #[test]
    fn receipt_does_not_write_gate_decision_or_create_proofpack() {
        let boundary = sample_receipt().boundary();

        assert!(!boundary.writes_gate_decision);
        assert!(!boundary.creates_proofpack);
    }

    #[test]
    fn denied_transitions_are_not_receipts() {
        let boundary = run_receipt_boundary();

        assert!(!boundary.denied_transition_creates_receipt);
    }

    #[test]
    fn receipt_does_not_require_runtime_storage() {
        let boundary = sample_receipt().boundary();

        assert!(!boundary.requires_runtime_storage);
    }

    #[test]
    fn required_missing_validator_gap_stays_visible() {
        let receipt = sample_receipt();
        let validator = &receipt.validator_evidence()[0];

        assert_eq!(validator.validator_id(), "cargo-test");
        assert_eq!(validator.validator_kind(), "command");
        assert_eq!(validator.requiredness(), ValidatorRequiredness::Required);
        assert_eq!(validator.requiredness().as_str(), "required");
        assert_eq!(validator.outcome(), ValidatorOutcome::Unavailable);
        assert_eq!(validator.outcome().as_str(), "unavailable");
        assert_eq!(validator.scope_ref(), "crates/**");
        assert_eq!(validator.reason_code(), Some("COMMAND_NOT_FOUND"));
        assert_eq!(
            validator.summary(),
            "cargo test could not run in the target environment"
        );
        assert_eq!(validator.observed_at(), "2026-04-25T18:45:00Z");
        assert_eq!(
            validator.follow_up_ref(),
            Some("work/goals/goal_extend_run_receipt_kernel_minimal_fields_v0_1.md")
        );
        assert!(validator.is_required_missing_evidence_gap());
        assert!(validator.blocks_proof_claim());
        assert!(receipt.has_required_validator_evidence_gaps());
        assert!(receipt.has_required_validator_blockers());
        assert!(!receipt.boundary().hides_missing_validator_gaps);
    }

    #[test]
    fn failed_required_validator_blocks_but_is_not_missing() {
        let validator = ValidatorEvidence::new(
            "unit-tests",
            "command",
            ValidatorRequiredness::Required,
            ValidatorOutcome::Failed,
            "crates/punk-domain/src/lib.rs",
            Some("ASSERTION_FAILED".to_string()),
            "unit tests ran and failed",
            Some("work/reports/failure.md".to_string()),
            "2026-04-25T18:46:00Z",
            None,
        )
        .expect("failed validator evidence should be valid");

        assert!(!validator.is_required_missing_evidence_gap());
        assert!(validator.blocks_proof_claim());
        assert_eq!(validator.evidence_ref(), Some("work/reports/failure.md"));
    }

    #[test]
    fn non_passing_validator_requires_reason_code() {
        let error = ValidatorEvidence::new(
            "cargo-test",
            "command",
            ValidatorRequiredness::Required,
            ValidatorOutcome::Skipped,
            "crates/**",
            None,
            "validator was skipped",
            None,
            "2026-04-25T18:47:00Z",
            None,
        )
        .expect_err("non-passing validator evidence should require a reason code");

        assert_eq!(error, RunReceiptError::MissingValidatorReasonCode);
    }
}
