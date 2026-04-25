//! Minimal side-effect-free gate decision kernel for Punk Phase 3.
//!
//! This crate models final decision authority as data only. It does not write
//! `.punk/decisions`, expose CLI behavior, create proofpacks, claim
//! acceptance, run validators, or require runtime storage.

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const GATE_DECISION_SCHEMA_VERSION: &str = "punk.gate_decision.v0.1";

fn non_empty(
    value: impl Into<String>,
    error: GateDecisionError,
) -> Result<String, GateDecisionError> {
    let value = value.into().trim().to_string();

    if value.is_empty() {
        return Err(error);
    }

    Ok(value)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GateDecisionId(String);

impl GateDecisionId {
    pub fn new(value: impl Into<String>) -> Result<Self, GateDecisionError> {
        Ok(Self(non_empty(value, GateDecisionError::EmptyDecisionId)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GateCreatedAt(String);

impl GateCreatedAt {
    pub fn new(value: impl Into<String>) -> Result<Self, GateDecisionError> {
        Ok(Self(non_empty(value, GateDecisionError::EmptyCreatedAt)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GateContractRef(String);

impl GateContractRef {
    pub fn new(value: impl Into<String>) -> Result<Self, GateDecisionError> {
        Ok(Self(non_empty(value, GateDecisionError::EmptyContractRef)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GateRunReceiptRef(String);

impl GateRunReceiptRef {
    pub fn new(value: impl Into<String>) -> Result<Self, GateDecisionError> {
        Ok(Self(non_empty(
            value,
            GateDecisionError::EmptyRunReceiptRef,
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GateEvalRef(String);

impl GateEvalRef {
    pub fn new(value: impl Into<String>) -> Result<Self, GateDecisionError> {
        Ok(Self(non_empty(value, GateDecisionError::EmptyEvalRef)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GateEventRef(String);

impl GateEventRef {
    pub fn new(value: impl Into<String>) -> Result<Self, GateDecisionError> {
        Ok(Self(non_empty(value, GateDecisionError::EmptyEventRef)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GateBoundaryNote(String);

impl GateBoundaryNote {
    pub fn new(value: impl Into<String>) -> Result<Self, GateDecisionError> {
        Ok(Self(non_empty(
            value,
            GateDecisionError::EmptyBoundaryNote,
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GateDecisionOutcome {
    Accepted,
    Rejected,
    NeedsWork,
    Blocked,
    Deferred,
}

impl GateDecisionOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::NeedsWork => "needs_work",
            Self::Blocked => "blocked",
            Self::Deferred => "deferred",
        }
    }

    pub fn is_accepting(self) -> bool {
        matches!(self, Self::Accepted)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateDecision {
    id: GateDecisionId,
    schema_version: &'static str,
    outcome: GateDecisionOutcome,
    contract_refs: Vec<GateContractRef>,
    run_receipt_refs: Vec<GateRunReceiptRef>,
    eval_refs: Vec<GateEvalRef>,
    event_refs: Vec<GateEventRef>,
    created_at: GateCreatedAt,
    boundary_notes: Vec<GateBoundaryNote>,
}

impl GateDecision {
    pub fn new(
        id: GateDecisionId,
        outcome: GateDecisionOutcome,
        contract_refs: Vec<GateContractRef>,
        run_receipt_refs: Vec<GateRunReceiptRef>,
        created_at: GateCreatedAt,
        boundary_notes: Vec<GateBoundaryNote>,
    ) -> Result<Self, GateDecisionError> {
        if contract_refs.is_empty() {
            return Err(GateDecisionError::MissingContractRefs);
        }

        if run_receipt_refs.is_empty() {
            return Err(GateDecisionError::MissingRunReceiptRefs);
        }

        if boundary_notes.is_empty() {
            return Err(GateDecisionError::MissingBoundaryNotes);
        }

        Ok(Self {
            id,
            schema_version: GATE_DECISION_SCHEMA_VERSION,
            outcome,
            contract_refs,
            run_receipt_refs,
            eval_refs: Vec::new(),
            event_refs: Vec::new(),
            created_at,
            boundary_notes,
        })
    }

    pub fn with_eval_ref(mut self, eval_ref: GateEvalRef) -> Self {
        self.eval_refs.push(eval_ref);
        self
    }

    pub fn with_event_ref(mut self, event_ref: GateEventRef) -> Self {
        self.event_refs.push(event_ref);
        self
    }

    pub fn id(&self) -> &GateDecisionId {
        &self.id
    }

    pub fn schema_version(&self) -> &str {
        self.schema_version
    }

    pub fn outcome(&self) -> GateDecisionOutcome {
        self.outcome
    }

    pub fn contract_refs(&self) -> &[GateContractRef] {
        &self.contract_refs
    }

    pub fn run_receipt_refs(&self) -> &[GateRunReceiptRef] {
        &self.run_receipt_refs
    }

    pub fn eval_refs(&self) -> &[GateEvalRef] {
        &self.eval_refs
    }

    pub fn event_refs(&self) -> &[GateEventRef] {
        &self.event_refs
    }

    pub fn created_at(&self) -> &GateCreatedAt {
        &self.created_at
    }

    pub fn boundary_notes(&self) -> &[GateBoundaryNote] {
        &self.boundary_notes
    }

    pub fn boundary(&self) -> GateDecisionBoundary {
        gate_decision_boundary()
    }

    pub fn is_final_decision_authority(&self) -> bool {
        self.boundary().writes_final_decision
    }

    pub fn requires_proof_for_acceptance_claim(&self) -> bool {
        self.outcome.is_accepting() && self.boundary().requires_matching_proof_for_acceptance
    }

    pub fn acceptance_claimable_without_proof(&self) -> bool {
        false
    }

    pub fn references_evidence_without_absorbing(&self) -> bool {
        !self.boundary().absorbs_evidence
            && !self.contract_refs.is_empty()
            && !self.run_receipt_refs.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateDecisionError {
    EmptyDecisionId,
    EmptyCreatedAt,
    EmptyContractRef,
    EmptyRunReceiptRef,
    EmptyEvalRef,
    EmptyEventRef,
    EmptyBoundaryNote,
    MissingContractRefs,
    MissingRunReceiptRefs,
    MissingBoundaryNotes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateDecisionBoundary {
    pub writes_final_decision: bool,
    pub implies_acceptance_by_itself: bool,
    pub requires_matching_proof_for_acceptance: bool,
    pub writes_proofpack: bool,
    pub requires_runtime_storage: bool,
    pub writes_cli_output: bool,
    pub creates_acceptance_claim: bool,
    pub absorbs_evidence: bool,
}

pub const fn gate_decision_boundary() -> GateDecisionBoundary {
    GateDecisionBoundary {
        writes_final_decision: true,
        implies_acceptance_by_itself: false,
        requires_matching_proof_for_acceptance: true,
        writes_proofpack: false,
        requires_runtime_storage: false,
        writes_cli_output: false,
        creates_acceptance_claim: false,
        absorbs_evidence: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_decision(outcome: GateDecisionOutcome) -> GateDecision {
        GateDecision::new(
            GateDecisionId::new("decision_local_001").expect("decision id should be valid"),
            outcome,
            vec![GateContractRef::new("contract_local_001").expect("contract ref should be valid")],
            vec![GateRunReceiptRef::new("receipt_local_001").expect("receipt ref should be valid")],
            GateCreatedAt::new("2026-04-25T19:00:00Z").expect("created_at should be valid"),
            vec![GateBoundaryNote::new(
                "Decision references evidence; proof is still required before acceptance.",
            )
            .expect("boundary note should be valid")],
        )
        .expect("decision should be valid")
        .with_eval_ref(
            GateEvalRef::new("work/reports/2026-04-25-run-receipt-kernel-minimal-fields-v0-1.md")
                .expect("eval ref should be valid"),
        )
        .with_event_ref(
            GateEventRef::new("evt_0000000000000001").expect("event ref should be valid"),
        )
    }

    #[test]
    fn decision_can_reference_evidence_without_absorbing_it() {
        let decision = sample_decision(GateDecisionOutcome::Accepted);

        assert_eq!(decision.id().as_str(), "decision_local_001");
        assert_eq!(decision.schema_version(), GATE_DECISION_SCHEMA_VERSION);
        assert_eq!(decision.outcome(), GateDecisionOutcome::Accepted);
        assert_eq!(decision.outcome().as_str(), "accepted");
        assert_eq!(decision.contract_refs()[0].as_str(), "contract_local_001");
        assert_eq!(decision.run_receipt_refs()[0].as_str(), "receipt_local_001");
        assert_eq!(decision.eval_refs().len(), 1);
        assert_eq!(decision.event_refs().len(), 1);
        assert_eq!(decision.created_at().as_str(), "2026-04-25T19:00:00Z");
        assert_eq!(decision.boundary_notes().len(), 1);
        assert!(decision.references_evidence_without_absorbing());
        assert!(!decision.boundary().absorbs_evidence);
    }

    #[test]
    fn decision_requires_core_refs_and_boundary_notes() {
        let id = GateDecisionId::new("decision_local_002").expect("decision id should be valid");
        let created_at =
            GateCreatedAt::new("2026-04-25T19:01:00Z").expect("created_at should be valid");
        let note = GateBoundaryNote::new("Evidence-only refs").expect("note should be valid");

        assert_eq!(
            GateDecision::new(
                id.clone(),
                GateDecisionOutcome::NeedsWork,
                Vec::new(),
                vec![GateRunReceiptRef::new("receipt").expect("receipt ref should be valid")],
                created_at.clone(),
                vec![note.clone()],
            ),
            Err(GateDecisionError::MissingContractRefs)
        );
        assert_eq!(
            GateDecision::new(
                id.clone(),
                GateDecisionOutcome::NeedsWork,
                vec![GateContractRef::new("contract").expect("contract ref should be valid")],
                Vec::new(),
                created_at.clone(),
                vec![note],
            ),
            Err(GateDecisionError::MissingRunReceiptRefs)
        );
        assert_eq!(
            GateDecision::new(
                id,
                GateDecisionOutcome::NeedsWork,
                vec![GateContractRef::new("contract").expect("contract ref should be valid")],
                vec![GateRunReceiptRef::new("receipt").expect("receipt ref should be valid")],
                created_at,
                Vec::new(),
            ),
            Err(GateDecisionError::MissingBoundaryNotes)
        );
    }

    #[test]
    fn accepting_decision_is_final_authority_but_not_acceptance_without_proof() {
        let decision = sample_decision(GateDecisionOutcome::Accepted);
        let boundary = decision.boundary();

        assert!(decision.is_final_decision_authority());
        assert!(decision.requires_proof_for_acceptance_claim());
        assert!(!decision.acceptance_claimable_without_proof());
        assert!(boundary.writes_final_decision);
        assert!(!boundary.implies_acceptance_by_itself);
        assert!(boundary.requires_matching_proof_for_acceptance);
        assert!(!boundary.creates_acceptance_claim);
    }

    #[test]
    fn non_accepting_decisions_do_not_require_proof_for_acceptance_claims() {
        for outcome in [
            GateDecisionOutcome::Rejected,
            GateDecisionOutcome::NeedsWork,
            GateDecisionOutcome::Blocked,
            GateDecisionOutcome::Deferred,
        ] {
            let decision = sample_decision(outcome);

            assert!(!outcome.is_accepting());
            assert!(!decision.requires_proof_for_acceptance_claim());
            assert!(!decision.acceptance_claimable_without_proof());
        }
    }

    #[test]
    fn decision_kernel_has_no_runtime_or_proofpack_side_effects() {
        let boundary = gate_decision_boundary();

        assert!(boundary.writes_final_decision);
        assert!(!boundary.writes_proofpack);
        assert!(!boundary.requires_runtime_storage);
        assert!(!boundary.writes_cli_output);
        assert!(!boundary.absorbs_evidence);
    }

    #[test]
    fn refs_reject_empty_values() {
        assert_eq!(
            GateDecisionId::new(" "),
            Err(GateDecisionError::EmptyDecisionId)
        );
        assert_eq!(
            GateCreatedAt::new(" "),
            Err(GateDecisionError::EmptyCreatedAt)
        );
        assert_eq!(
            GateContractRef::new(" "),
            Err(GateDecisionError::EmptyContractRef)
        );
        assert_eq!(
            GateRunReceiptRef::new(" "),
            Err(GateDecisionError::EmptyRunReceiptRef)
        );
        assert_eq!(GateEvalRef::new(" "), Err(GateDecisionError::EmptyEvalRef));
        assert_eq!(
            GateEventRef::new(" "),
            Err(GateDecisionError::EmptyEventRef)
        );
        assert_eq!(
            GateBoundaryNote::new(" "),
            Err(GateDecisionError::EmptyBoundaryNote)
        );
    }
}
