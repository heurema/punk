//! Minimal run receipt kernel for Punk Phase 3.
//!
//! This crate intentionally stays deterministic and side-effect free in this
//! first implementation step. It models pre-gate run receipt evidence only.
//! No `.punk/` storage, CLI behavior, gate decisions, proofpacks, or run
//! execution are introduced here.

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunReceipt {
    id: RunReceiptId,
    contract_ref: ContractRef,
    run_id: RunId,
    run_scope_ref: RunScopeRef,
    produced_artifact_refs: Vec<RunArtifactRef>,
    event_refs: Vec<EventRef>,
    eval_report_refs: Vec<EvalReportRef>,
}

impl RunReceipt {
    pub fn new(
        id: RunReceiptId,
        contract_ref: ContractRef,
        run_id: RunId,
        run_scope_ref: RunScopeRef,
    ) -> Self {
        Self {
            id,
            contract_ref,
            run_id,
            run_scope_ref,
            produced_artifact_refs: Vec::new(),
            event_refs: Vec::new(),
            eval_report_refs: Vec::new(),
        }
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

    pub fn id(&self) -> &RunReceiptId {
        &self.id
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

    pub fn produced_artifact_refs(&self) -> &[RunArtifactRef] {
        &self.produced_artifact_refs
    }

    pub fn event_refs(&self) -> &[EventRef] {
        &self.event_refs
    }

    pub fn eval_report_refs(&self) -> &[EvalReportRef] {
        &self.eval_report_refs
    }

    pub fn boundary(&self) -> RunReceiptBoundary {
        run_receipt_boundary()
    }

    pub fn run_evidence_only(&self) -> bool {
        let boundary = self.boundary();

        !boundary.implies_final_acceptance
            && !boundary.writes_gate_decision
            && !boundary.creates_proofpack
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunReceiptError {
    EmptyReceiptId,
    EmptyContractRef,
    EmptyRunId,
    EmptyRunScopeRef,
    EmptyArtifactRef,
    EmptyEventRef,
    EmptyEvalReportRef,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunReceiptBoundary {
    pub implies_final_acceptance: bool,
    pub writes_gate_decision: bool,
    pub creates_proofpack: bool,
    pub requires_runtime_storage: bool,
    pub denied_transition_creates_receipt: bool,
}

pub const fn run_receipt_boundary() -> RunReceiptBoundary {
    RunReceiptBoundary {
        implies_final_acceptance: false,
        writes_gate_decision: false,
        creates_proofpack: false,
        requires_runtime_storage: false,
        denied_transition_creates_receipt: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_receipt() -> RunReceipt {
        RunReceipt::new(
            RunReceiptId::new("receipt_local_001").expect("receipt id should be valid"),
            ContractRef::new("contract_local_001").expect("contract ref should be valid"),
            RunId::new("run_local_001").expect("run id should be valid"),
            RunScopeRef::new("work/goals/goal_add_run_receipt_minimal.md")
                .expect("run scope ref should be valid"),
        )
        .with_artifact_ref(
            RunArtifactRef::new("work/reports/2026-04-22-run-receipt-minimal.md")
                .expect("artifact ref should be valid"),
        )
        .with_event_ref(EventRef::new("evt_0000000000000001").expect("event ref should be valid"))
        .with_eval_report_ref(
            EvalReportRef::new("work/reports/2026-04-22-contract-flow-smoke-eval.md")
                .expect("eval report ref should be valid"),
        )
    }

    #[test]
    fn receipt_can_be_created_for_bounded_run_with_refs() {
        let receipt = sample_receipt();

        assert_eq!(receipt.id().as_str(), "receipt_local_001");
        assert_eq!(receipt.contract_ref().as_str(), "contract_local_001");
        assert_eq!(receipt.run_id().as_str(), "run_local_001");
        assert_eq!(
            receipt.run_scope_ref().as_str(),
            "work/goals/goal_add_run_receipt_minimal.md"
        );
        assert_eq!(receipt.produced_artifact_refs().len(), 1);
        assert_eq!(receipt.event_refs().len(), 1);
        assert_eq!(receipt.eval_report_refs().len(), 1);
    }

    #[test]
    fn receipt_requires_non_empty_receipt_id() {
        let error = RunReceiptId::new("   ").expect_err("empty receipt id must be rejected");

        assert_eq!(error, RunReceiptError::EmptyReceiptId);
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
}
