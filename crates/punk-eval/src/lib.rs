//! Minimal deterministic smoke eval harness for the current Punk core.
//!
//! This Phase 1/2/3 bridge stays library-first. It assesses the existing flow,
//! event, receipt, gate, and proof kernels without activating `.punk/` runtime
//! state, baseline comparison, waiver storage, or a full eval platform.

use std::fmt::Write as _;
use std::fs;
use std::process;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use punk_contract::{
    approve_contract, assess_gate_input_policy, assess_hard_clause_mappings,
    assess_proof_requirements, assess_receipt_requirements, confirm_contract_draft,
    contract_draft_confirmation_boundary, contract_gate_input_policy_v0_1,
    contract_gate_policy_blueprint, contract_proof_requirements_blueprint,
    contract_proof_requirements_v0_1, contract_schema_blueprint_v0_1,
    contract_writer_authority_boundary, validate_contract, ContractClauseBlueprint,
    ContractClauseKind, ContractClauseMode, ContractClauseSeverity, ContractDraft,
    ContractDraftApprovalEvidence, ContractDraftConfirmation, ContractDraftConfirmationBlocker,
    ContractDraftConfirmationOutcome, ContractDraftUnknownDisposition,
    ContractDraftUnknownHandling, ContractError, ContractGateInputPolicy, ContractId,
    ContractProofRequirements, ContractReceiptRequirement, ContractSchemaFieldStatus,
    ContractSchemaSection, ContractScope, ContractStatus, GateInputEvidence, GateInputRequirement,
    GateInputRequirementSource, GateInputRequirementStatus, HardClauseMappingStatus,
    HardClauseMappingTarget, ProofRequirement, ProofRequirementSource, ProofRequirementStatus,
    ReceiptRequirementSource, ReceiptRequirementStatus, UserIntentContractDraftBlocker,
    UserIntentContractDraftModel, UserIntentContractDraftReadiness, UserIntentDownstreamClosure,
    UserIntentResearchGate, UserIntentResearchGateClassification, UserIntentUnknown,
    CONTRACT_GATE_INPUT_POLICY_BASELINE_REQUIREMENTS, CONTRACT_STATUS_VALUES,
};
use punk_core::{
    compute_artifact_digest, compute_artifact_file_digest, is_canonical_artifact_digest,
    is_valid_repo_relative_artifact_ref, validate_artifact_digest,
    validate_repo_relative_artifact_ref, verify_referenced_artifact_digest, ArtifactDigest,
    ArtifactHashPolicyError, FileArtifactHashError, ReferencedArtifactVerificationOutcome,
    RepoRelativeArtifactRef, RepoRoot, ARTIFACT_HASH_POLICY_CAPABILITIES,
    ARTIFACT_HASH_POLICY_VERSION, FILE_ARTIFACT_HASHING_CAPABILITIES,
    REFERENCED_ARTIFACT_VERIFICATION_CAPABILITIES,
};
use punk_domain::{ContractRef, ProducedAt, RunId, RunReceiptId, RunScopeRef};
use punk_events::{append_local_flow_event, schema_fixture, MemoryEventLog};
use punk_flow::{transition_attempt_event_draft, FlowCommand, FlowInstance, FlowState};
use punk_gate::{
    GateBoundaryNote, GateContractRef, GateCreatedAt, GateDecision, GateDecisionId,
    GateDecisionOutcome, GateEvalRef, GateEventRef, GateRunReceiptRef,
};
use punk_mod_pub::{
    assess_pubpunk_inventory, PubPunkAssessmentAuthority, PubPunkAssessmentStatus,
    PubPunkCapabilityGrant, PubPunkInventoryInput, PubPunkInventoryItemInput,
    PubPunkInventoryItemKind, PubPunkInventoryItemStatus,
};
use punk_module_host::{
    model_module_side_effect_receipt_writer_active_behavior, preflight_module_policy_gate,
    preflight_module_side_effect_receipt_writer, propose_module_assessment_receipt,
    propose_module_side_effect_request, wrap_module_assessment, ModuleCapabilityGrant,
    ModuleHostStatus, ModuleInvocationEnvelope, ModuleOutputAuthority, ModuleOutputBoundaryFlags,
    ModuleOutputStatus, ModuleOutputSummary, ModulePolicyGatePreflightDraft,
    ModulePolicyGatePreflightRequirement, ModuleReceiptProposalField, ModuleSideEffectKind,
    ModuleSideEffectPrecondition, ModuleSideEffectReceiptWriterModeledStep,
    ModuleSideEffectReceiptWriterObservation, ModuleSideEffectReceiptWriterOutcome,
    ModuleSideEffectReceiptWriterPreflightDraft, ModuleSideEffectReceiptWriterPreflightRequirement,
    ModuleSideEffectRequestDraft,
};
use punk_project::{
    default_instruction_page_index_nodes, init_level0_project, init_project,
    locate_publishing_workspace, render_instruction_page_index_json,
    source_corpus_manifest_claim_field_allowed, source_corpus_manifest_render_canonical_bytes,
    source_corpus_manifest_writer_write_first_slice, ProjectId, ProjectInitArtifactKind,
    ProjectInitArtifactStatus, ProjectInitEntryMode, PublishingLocateStatus, SourceCorpusItem,
    SourceCorpusItemId, SourceCorpusManifest, SourceCorpusManifestAuthority,
    SourceCorpusManifestId, SourceCorpusManifestStatus, SourceCorpusManifestWriterConflictPolicy,
    SourceCorpusManifestWriterFirstSliceBlocker, SourceCorpusManifestWriterManifestInspection,
    SourceCorpusManifestWriterOperationEvidenceStatus, SourceCorpusManifestWriterPreflight,
    SourceCorpusManifestWriterPreflightFinding, SourceCorpusManifestWriterPreflightInput,
    SourceCorpusManifestWriterSymlinkAncestorStatus, SourceCorpusManifestWriterTarget,
    SourceCorpusObservedKind, SourceCorpusRepoRelativePath, SourceCorpusSourceClass,
    INSTRUCTIONS_INDEX_PATH, PUBLISHING_BINDING_PATH, PUBLISHING_LOCAL_POINTER_PATH,
    SOURCE_CORPUS_DEFAULT_EXCLUDED_PATHS, SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH,
};
use punk_proof::{
    compute_proofpack_manifest_digest, positive_acceptance_preconditions_met,
    proofpack_writer_first_active_write_slice_boundary,
    proofpack_writer_hash_reference_integration_model_boundary,
    proofpack_writer_write_first_active_slice, PositiveAcceptanceInputs, ProofArtifactDigest,
    ProofArtifactHash, ProofArtifactKind, ProofArtifactRef, ProofBoundaryNote, ProofContractRef,
    ProofCreatedAt, ProofEvalRef, ProofEventRef, ProofGateDecisionRef, ProofOutputArtifactRef,
    ProofRunReceiptRef, Proofpack, ProofpackId, ProofpackWriterAbortState,
    ProofpackWriterActiveBehaviorModel, ProofpackWriterAttemptedAt,
    ProofpackWriterCanonicalArtifactLayout, ProofpackWriterCanonicalArtifactModel,
    ProofpackWriterCanonicalArtifactStatus, ProofpackWriterConcretePathStoragePolicyBlocker,
    ProofpackWriterConcretePathStoragePolicyModel, ProofpackWriterConcretePathStoragePolicyRefs,
    ProofpackWriterConcretePathStoragePolicyStatus, ProofpackWriterDeclaredArtifactDigestEvidence,
    ProofpackWriterDeclaredArtifactDigestStatus, ProofpackWriterDiagnosticPathRef,
    ProofpackWriterFileIoBlocker, ProofpackWriterFileIoDiagnostic,
    ProofpackWriterFileIoDiagnosticSource, ProofpackWriterFileIoErrorReason,
    ProofpackWriterFileIoErrorReasonModel, ProofpackWriterFileIoFailureVisibility,
    ProofpackWriterFileIoObservation, ProofpackWriterFileIoOutcomeModel, ProofpackWriterFileIoPlan,
    ProofpackWriterFirstActiveWriteSliceBlocker, ProofpackWriterHashReferenceIntegrationBlocker,
    ProofpackWriterHashReferenceIntegrationModel, ProofpackWriterHashReferenceIntegrationStatus,
    ProofpackWriterHostPathBlocker, ProofpackWriterHostPathKind,
    ProofpackWriterHostPathObservationStatus, ProofpackWriterHostPathPolicyRef,
    ProofpackWriterHostPathPolicyRefs, ProofpackWriterHostPathResolutionModel,
    ProofpackWriterIdempotencyBasis, ProofpackWriterIdempotencyObservation,
    ProofpackWriterMissingPrecondition, ProofpackWriterNonCanonicalArtifactSurface,
    ProofpackWriterObservedPartialState, ProofpackWriterObservedTargetState,
    ProofpackWriterObservedWriteResult, ProofpackWriterOperationEvidence,
    ProofpackWriterOperationId, ProofpackWriterOperationKind, ProofpackWriterOperationOutcome,
    ProofpackWriterPlannedSideEffect, ProofpackWriterPreflightIntegrationBlocker,
    ProofpackWriterPreflightIntegrationModel, ProofpackWriterPreflightIntegrationStatus,
    ProofpackWriterPreflightPlan, ProofpackWriterReferencedArtifactVerificationEvidence,
    ProofpackWriterReferencedArtifactVerificationStatus, ProofpackWriterSideEffectStatus,
    ProofpackWriterStorageRootRef, ProofpackWriterTargetArtifactRefPolicyModel,
    ProofpackWriterTargetArtifactRefPolicyReason, ProofpackWriterTargetArtifactRefPolicyStatus,
    ProofpackWriterTargetPathPolicyModel, ProofpackWriterTargetPathPolicyReason,
    ProofpackWriterTargetPathPolicyStatus, ProofpackWriterTargetPathRef, ProofpackWriterTargetRef,
    ProofpackWriterTempAtomicPolicy, ProofpackWriterWritePolicy,
    PROOFPACK_WRITER_ACTIVE_BEHAVIOR_MODEL_SCHEMA_VERSION,
    PROOFPACK_WRITER_CANONICAL_ARTIFACT_MODEL_SCHEMA_VERSION,
    PROOFPACK_WRITER_CONCRETE_PATH_STORAGE_POLICY_MODEL_SCHEMA_VERSION,
    PROOFPACK_WRITER_FIRST_ACTIVE_WRITE_SLICE_SCHEMA_VERSION,
    PROOFPACK_WRITER_HASH_REFERENCE_INTEGRATION_MODEL_SCHEMA_VERSION,
    PROOFPACK_WRITER_HOST_PATH_RESOLUTION_MODEL_SCHEMA_VERSION,
    PROOFPACK_WRITER_PREFLIGHT_INTEGRATION_MODEL_SCHEMA_VERSION,
    PROOFPACK_WRITER_TARGET_ARTIFACT_REF_POLICY_MODEL_SCHEMA_VERSION,
};

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const SMOKE_SUITE_ID: &str = "smoke.v0";
pub const SMOKE_REPORT_SCHEMA_VERSION: &str = "smoke-eval-report.v0.1";
const PROOF_HASH_GATE_DECISION: &str =
    "sha256:0000000000000000000000000000000000000000000000000000000000000001";
const PROOF_HASH_CONTRACT: &str =
    "sha256:0000000000000000000000000000000000000000000000000000000000000002";
const PROOF_HASH_RUN_RECEIPT: &str =
    "sha256:0000000000000000000000000000000000000000000000000000000000000003";
const PROOF_HASH_EVAL: &str =
    "sha256:0000000000000000000000000000000000000000000000000000000000000004";
const PROOF_HASH_EVENT: &str =
    "sha256:0000000000000000000000000000000000000000000000000000000000000005";
const PROOF_HASH_OUTPUT: &str =
    "sha256:0000000000000000000000000000000000000000000000000000000000000006";

static SMOKE_TEMP_PATH_COUNTER: AtomicU64 = AtomicU64::new(0);

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
        eval_local_flow_event_writer_appends_project_event(),
        eval_instruction_page_index_model_is_deterministic_and_advisory(),
        eval_publishing_locate_resolves_local_pointer_without_side_effects(),
        eval_pubpunk_inventory_assessment_model_is_side_effect_free(),
        eval_module_host_wraps_pubpunk_assessment_without_side_effects(),
        eval_module_host_receipt_proposal_model_is_side_effect_free(),
        eval_module_host_side_effect_request_proposal_model_is_side_effect_free(),
        eval_module_host_policy_gate_preflight_model_is_side_effect_free(),
        eval_module_host_side_effect_receipt_writer_preflight_model_is_side_effect_free(),
        eval_module_host_side_effect_receipt_writer_active_behavior_model_is_side_effect_free(),
        eval_project_init_creates_level0_manual_memory_scaffold(),
        eval_project_init_brownfield_scaffold_shape(),
        eval_project_init_refuses_to_overwrite_existing_memory(),
        eval_project_init_conflict_is_atomic_noop(),
        eval_source_corpus_manifest_model_is_side_effect_free(),
        eval_source_corpus_manifest_writer_preflight_model_is_side_effect_free(),
        eval_source_corpus_manifest_writer_first_slice_writes_prepared_manifest(),
        eval_contract_ready_for_bounded_work_allows_start_run(),
        eval_contract_draft_denies_start_run(),
        eval_contract_invalid_scope_denies_start_run(),
        eval_contract_denial_does_not_mutate_flow_state(),
        eval_contract_guard_result_remains_evidence_not_decision(),
        eval_contract_receipt_allowed_path_produces_evidence(),
        eval_contract_receipt_draft_denial_produces_no_receipt(),
        eval_contract_receipt_invalid_scope_produces_no_receipt(),
        eval_contract_receipt_remains_pre_gate_evidence(),
        eval_contract_schema_blueprint_has_required_top_level_sections(),
        eval_contract_schema_blueprint_marks_status_required_now_deferred_parked_future(),
        eval_contract_status_does_not_include_acceptance_decisions(),
        eval_hard_clause_requires_validator_receipt_proof_or_gate_review_mapping(),
        eval_hard_clause_with_validator_ref_is_mapped(),
        eval_hard_clause_with_required_receipt_field_is_mapped(),
        eval_hard_clause_with_proof_requirement_ref_is_mapped(),
        eval_hard_clause_with_explicit_human_gate_review_and_reason_is_mapped(),
        eval_hard_clause_with_blank_human_review_reason_is_blocking(),
        eval_hard_clause_with_no_mapping_is_unsupported_blocking(),
        eval_unsupported_hard_clause_blocks_approved_for_run(),
        eval_soft_clause_without_mapping_does_not_block_approval(),
        eval_advisory_clause_without_mapping_does_not_block_approval(),
        eval_hard_rationale_clause_is_invalid_or_blocking(),
        eval_ready_confirmed_draft_with_all_hard_clauses_mapped_can_approve(),
        eval_ready_confirmed_draft_with_unmapped_hard_clause_cannot_approve(),
        eval_hard_clause_mapping_does_not_create_gate_decision(),
        eval_hard_clause_mapping_does_not_create_proofpack(),
        eval_hard_clause_mapping_does_not_invoke_writer(),
        eval_contract_status_excludes_acceptance_outcomes_after_hard_clause_mapping(),
        eval_receipt_requirement_covers_artifact_hashes_hard_clause(),
        eval_receipt_requirement_covers_validator_results_hard_clause(),
        eval_receipt_requirement_covers_deviation_flags_hard_clause(),
        eval_receipt_requirement_covers_side_effects_hard_clause(),
        eval_missing_receipt_requirement_blocks_approved_for_run(),
        eval_unknown_receipt_requirement_field_is_blocking(),
        eval_duplicate_receipt_requirements_are_non_conflicting(),
        eval_soft_clause_receipt_mapping_absent_is_non_blocking(),
        eval_advisory_clause_receipt_mapping_absent_is_non_blocking(),
        eval_executor_claims_receipt_field_remains_unverified_evidence(),
        eval_receipt_requirements_do_not_create_run_receipt(),
        eval_receipt_requirements_do_not_write_punk_runs_storage(),
        eval_receipt_requirements_do_not_create_gate_decision(),
        eval_receipt_requirements_do_not_create_proofpack(),
        eval_receipt_requirements_do_not_invoke_writer(),
        eval_contract_status_excludes_acceptance_outcomes_after_receipt_requirements(),
        eval_proof_requirements_require_contract_ref(),
        eval_proof_requirements_require_run_receipt_ref(),
        eval_proof_requirements_require_gate_decision_ref(),
        eval_proof_requirements_can_require_eval_report_ref_and_hash(),
        eval_proof_requirements_can_require_output_artifact_refs_and_hashes(),
        eval_missing_contract_proof_target_is_blocking(),
        eval_missing_run_receipt_proof_target_is_blocking(),
        eval_missing_gate_decision_proof_target_is_blocking(),
        eval_unsupported_proof_target_is_blocking(),
        eval_duplicate_proof_targets_are_non_conflicting(),
        eval_proof_requirements_do_not_create_proofpack(),
        eval_proof_requirements_do_not_write_punk_proofs_storage(),
        eval_proof_requirements_do_not_compute_artifact_hashes_from_filesystem(),
        eval_proof_requirements_do_not_write_gate_decision(),
        eval_proof_requirements_do_not_create_acceptance_claim(),
        eval_proof_requirements_do_not_invoke_writer(),
        eval_proofpack_is_not_required_before_gate_decision(),
        eval_contract_status_excludes_acceptance_outcomes_after_proof_requirements(),
        eval_gate_policy_does_not_write_decision(),
        eval_gate_input_policy_requires_contract_ref(),
        eval_gate_input_policy_requires_approved_for_run_status(),
        eval_gate_input_policy_requires_run_receipt_ref(),
        eval_gate_input_policy_requires_receipt_requirement_coverage(),
        eval_gate_input_policy_requires_hard_clause_mapping_assessment(),
        eval_gate_input_policy_can_require_validator_or_eval_report(),
        eval_gate_input_policy_can_require_deviation_status(),
        eval_gate_input_policy_can_require_executor_claim_status_without_proof(),
        eval_missing_contract_ref_blocks_gate_readiness(),
        eval_missing_approved_for_run_status_blocks_gate_readiness(),
        eval_missing_run_receipt_ref_blocks_gate_readiness(),
        eval_missing_receipt_requirement_coverage_blocks_gate_readiness(),
        eval_missing_hard_clause_mapping_assessment_blocks_gate_readiness(),
        eval_unsupported_gate_input_blocks_gate_readiness(),
        eval_duplicate_gate_input_requirements_are_non_conflicting(),
        eval_gate_input_policy_does_not_write_gate_decision(),
        eval_gate_input_policy_does_not_create_proofpack(),
        eval_gate_input_policy_does_not_require_existing_proofpack(),
        eval_gate_input_policy_does_not_create_acceptance_claim(),
        eval_gate_input_policy_does_not_invoke_writer(),
        eval_contract_status_excludes_acceptance_outcomes_after_gate_input_policy(),
        eval_approved_for_run_is_not_ready_for_gate(),
        eval_writer_is_not_upstream_contract_authority(),
        eval_user_intent_contract_draft_model_ready_for_confirmation(),
        eval_user_intent_contract_draft_model_requires_clarification(),
        eval_user_intent_contract_draft_model_refuses_or_defers(),
        eval_user_intent_contract_draft_model_blocks_missing_evidence(),
        eval_user_intent_contract_draft_model_keeps_writer_downstream(),
        eval_ready_draft_with_explicit_confirmation_becomes_approved_for_run(),
        eval_ready_draft_without_explicit_confirmation_is_not_approved(),
        eval_clarification_required_draft_cannot_be_approved(),
        eval_refused_or_deferred_draft_cannot_be_approved(),
        eval_blocked_draft_cannot_be_approved(),
        eval_unresolved_unknowns_block_approval(),
        eval_unknowns_converted_to_assumptions_allow_approval(),
        eval_approved_for_run_preserves_scope_and_non_scope(),
        eval_approved_for_run_preserves_evidence_plan(),
        eval_approved_for_run_preserves_side_effect_boundaries(),
        eval_user_confirmation_does_not_create_gate_decision(),
        eval_user_confirmation_does_not_create_proofpack(),
        eval_user_confirmation_does_not_invoke_writer(),
        eval_contract_status_still_excludes_acceptance_outcomes(),
        eval_gate_authority_requires_proof_before_acceptance(),
        eval_proofpack_is_post_gate_provenance_not_decision(),
        eval_acceptance_requires_accepting_decision_and_matching_proofpack(),
        eval_proofpack_integrity_ready_when_declared_digest_links_are_complete(),
        eval_proofpack_integrity_missing_digest_blocks_readiness(),
        eval_proofpack_manifest_renderer_is_deterministic_and_side_effect_free(),
        eval_proofpack_manifest_digest_matches_exact_renderer_bytes(),
        eval_proofpack_writer_canonical_artifact_model_is_side_effect_free(),
        eval_proofpack_writer_target_artifact_ref_policy_model_is_side_effect_free(),
        eval_proofpack_writer_operation_evidence_model_is_side_effect_free(),
        eval_proofpack_writer_preflight_plan_model_is_side_effect_free(),
        eval_proofpack_writer_file_io_plan_model_is_side_effect_free(),
        eval_proofpack_writer_file_io_outcome_model_is_side_effect_free(),
        eval_proofpack_writer_file_io_error_reason_model_is_side_effect_free(),
        eval_proofpack_writer_target_path_policy_model_is_side_effect_free(),
        eval_proofpack_writer_preflight_integration_model_is_side_effect_free(),
        eval_proofpack_writer_active_behavior_model_is_side_effect_free(),
        eval_proofpack_writer_host_path_resolution_model_is_side_effect_free(),
        eval_proofpack_writer_concrete_path_storage_policy_model_is_side_effect_free(),
        eval_proofpack_writer_first_active_write_slice_writes_exact_bytes(),
        eval_proofpack_writer_hash_reference_integration_model_is_side_effect_free(),
        eval_artifact_hash_policy_accepts_canonical_digest(),
        eval_artifact_hash_policy_rejects_invalid_digest(),
        eval_artifact_hash_policy_accepts_repo_relative_ref(),
        eval_artifact_hash_policy_rejects_invalid_ref(),
        eval_artifact_hash_computation_matches_known_vectors(),
        eval_artifact_hash_computation_preserves_exact_bytes(),
        eval_file_artifact_hashing_helper_hashes_explicit_regular_file(),
        eval_referenced_artifact_verification_helper_compares_expected_digest(),
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
        "local deterministic smoke harness passed over current contract, contract schema blueprint model, user intent-to-contract draft model, contract draft confirmation boundary model, hard clause mapping model, contract receipt requirements model, contract gate input policy model, contract proof requirements model, flow, receipt, event, local event writer, instruction page-index model, publishing locate resolver, PubPunk inventory assessment model, module-host invocation envelope, module-host receipt proposal model, module-host side-effect request proposal model, module-host policy gate preflight model, module-host side-effect receipt writer preflight model, module-host side-effect receipt writer active behavior model, greenfield and brownfield project init scaffolds, brownfield source corpus manifest side-effect-free model, brownfield source corpus manifest writer preflight model, brownfield source corpus manifest writer first slice, gate, proof, proofpack manifest renderer, proofpack manifest digest helper, proofpack writer canonical artifact model, proofpack writer target artifact ref policy model, proofpack writer operation evidence model, proofpack writer preflight plan model, proofpack writer file IO plan model, proofpack writer file IO outcome model, proofpack writer file IO error reason model, proofpack writer target path policy model, proofpack writer preflight integration model, proofpack writer active behavior model, proofpack writer host path resolution model, proofpack writer concrete path/storage policy model, proofpack writer first active write slice, proofpack writer hash/reference integration model, artifact hash policy, exact-byte hash computation helper, file IO artifact hashing helper, and referenced artifact verification helper kernels"
            .to_owned()
    } else {
        "local deterministic smoke harness found one or more failing cases over current contract, contract schema blueprint model, user intent-to-contract draft model, contract draft confirmation boundary model, hard clause mapping model, contract receipt requirements model, contract gate input policy model, contract proof requirements model, flow, receipt, event, local event writer, instruction page-index model, publishing locate resolver, PubPunk inventory assessment model, module-host invocation envelope, module-host receipt proposal model, module-host side-effect request proposal model, module-host policy gate preflight model, module-host side-effect receipt writer preflight model, module-host side-effect receipt writer active behavior model, greenfield and brownfield project init scaffolds, brownfield source corpus manifest side-effect-free model, brownfield source corpus manifest writer preflight model, brownfield source corpus manifest writer first slice, gate, proof, proofpack manifest renderer, proofpack manifest digest helper, proofpack writer canonical artifact model, proofpack writer target artifact ref policy model, proofpack writer operation evidence model, proofpack writer preflight plan model, proofpack writer file IO plan model, proofpack writer file IO outcome model, proofpack writer file IO error reason model, proofpack writer target path policy model, proofpack writer preflight integration model, proofpack writer active behavior model, proofpack writer host path resolution model, proofpack writer concrete path/storage policy model, proofpack writer first active write slice, proofpack writer hash/reference integration model, artifact hash policy, exact-byte hash computation helper, file IO artifact hashing helper, and referenced artifact verification helper kernels"
            .to_owned()
    };

    SmokeEvalReport {
        summary: SmokeEvalSummary {
            suite_id: SMOKE_SUITE_ID,
            smoke_result,
            assessment,
            mode: "local-smoke-check",
            runtime_persistence: "local-event-log-writer",
            report_storage: "inactive",
        },
        cases,
        boundary_notes: vec![
            "local assessment only; no authority is written here",
            "no .punk/evals runtime state is read or written",
            "local event writer smoke coverage writes only .punk/events/flow.jsonl under an explicit temporary project root with a .punk/project.toml marker",
            "instruction page-index smoke case renders a deterministic advisory tree from source instruction refs without creating .punk/views, summaries, vector DBs, hidden truth stores, LLM calls, or runtime side effects",
            "publishing locate smoke case reads only .punk/publishing.toml and .punk/publishing.local.toml under an explicit temporary project root and invokes no publishing, browser, network API, credential, adapter, bot, or file-write behavior",
            "PubPunk inventory assessment smoke case is module-owned, side-effect-free, no-IO, no-CLI, advisory-only, and does not publish, create receipts, read credentials, invoke adapters, or write gate/proof authority",
            "module-host invocation envelope smoke case wraps advisory module output only and does not load plugins, invoke modules, expose CLI behavior, read or write files, create receipts, mutate event logs, call APIs, read credentials, invoke adapters, publish, or write gate/proof authority",
            "module-host receipt proposal smoke case models future module receipt fields only and does not write receipts, mutate event logs, read or write files, call APIs, read credentials, invoke modules or adapters, publish, or write gate/proof authority",
            "module-host side-effect request proposal smoke case models future external action preconditions only and does not invoke adapters, publish, comment, create pull requests, write receipts, mutate event logs, call APIs, read credentials, or write gate/proof authority",
            "module-host policy gate preflight smoke case models future policy evidence readiness only and does not invoke policy engines, invoke gate, write decisions, invoke adapters, publish, comment, create pull requests, write receipts, mutate event logs, call APIs, read credentials, or write proof authority",
            "module-host side-effect receipt writer preflight smoke case models future receipt writer readiness only and does not write receipts, mutate event logs, read or write files, invoke adapters, invoke policy engines, invoke gate, publish, comment, create pull requests, call APIs, read credentials, write proofpacks, or claim acceptance",
            "module-host side-effect receipt writer active behavior smoke case models planned, written, idempotent, conflict, write-failed, and partial receipt outcomes without writing receipts, mutating event logs, persisting operation evidence, reading or writing files, invoking adapters, invoking policy engines, invoking gate, publishing, commenting, creating pull requests, calling APIs, reading credentials, writing proofpacks, or claiming acceptance",
            "greenfield init smoke cases create compact .punk/memory project-memory scaffold files plus thin .punk/instructions entrypoints with project_id, entry_mode, and .punk marker files without root-level Punk memory dirs, brownfield reconstruction, grayfield reconciliation, network behavior, .punk runtime stores, .punk/views, contracts, receipts, gate artifacts, proofpacks, or acceptance claims",
            "brownfield init smoke case creates only an advisory .punk/memory/reconstruction workspace plus thin .punk/instructions entrypoints with reconstruction_status not_started and no repo scan, AI summary, contracts, claims, runtime stores, .punk/views, gate artifacts, proofpacks, or acceptance claims",
            "brownfield source corpus manifest model smoke case is side-effect-free and does not scan repositories, walk files, read file contents, compute file hashes, write manifests, create claims, infer intent, use network, or use remote AI",
            "brownfield source corpus manifest writer preflight model smoke case is side-effect-free and models target/path/conflict/claim/runtime blockers without scanning repositories, walking files, reading contents, computing filesystem hashes, writing manifests, generating manifests, creating claims, using network, or using remote AI",
            "brownfield source corpus manifest writer first slice smoke case writes deterministic canonical bytes from an already-constructed model to one explicit safe target after preflight, without scanning repositories, walking files, reading source contents, computing source file hashes, creating claims, activating runtime storage, or promoting authority",
            "contract schema blueprint smoke cases preserve target shape, field status split, clause mapping, gate input policy, proof requirements, and Writer authority boundaries without runtime activation",
            "user intent-to-contract draft model smoke cases classify readiness in memory only and do not create contracts, runtime storage, CLI behavior, gate-writing behavior, proofpacks, or Writer behavior",
            "contract draft confirmation smoke cases require explicit user confirmation before approved_for_run model state without runtime storage, CLI behavior, gate-writing behavior, proofpacks, or Writer behavior",
            "hard clause mapping smoke cases require hard clauses to have validator, receipt, proof, or explicit human review paths before approved_for_run model state without runtime validator execution, storage, CLI, proofpacks, or Writer behavior",
            "contract receipt requirements smoke cases connect hard-clause receipt-field mappings to future run receipt fields without writing receipts, runtime storage, validator execution, gate-writing behavior, proofpacks, or Writer behavior",
            "contract gate input policy smoke cases distinguish approved_for_run from ready_for_gate and require gate inputs without proofpack-as-input, runtime gate writing, proofpack creation, acceptance claims, validator execution, storage, CLI, or Writer behavior",
            "contract proof requirements smoke cases declare future proofpack links and hashes after gate outcome without proofpack writing, artifact hash runtime, .punk/proofs storage, gate-writing behavior, acceptance claims, or Writer behavior",
            "run receipt evidence remains pre-gate and does not imply final acceptance",
            "gate/proof smoke cases remain local assessment and do not claim acceptance",
            "proofpack manifest renderer smoke case renders in memory only and does not write proofpacks",
            "proofpack manifest digest smoke case hashes exact in-memory renderer bytes only and does not verify referenced artifacts",
            "proofpack writer canonical artifact model smoke case keeps exact manifest bytes canonical and surrounding metadata non-canonical without writer side effects",
            "proofpack writer target artifact ref policy smoke case requires proofpack id plus manifest self-digest and renders logical non-path refs without writer side effects",
            "proofpack writer operation evidence smoke case models writer outcomes without writing proofpacks or acceptance claims",
            "proofpack writer preflight plan smoke case models writer-ready plans without attempting side effects",
            "proofpack writer file IO plan smoke case models storage roots, target paths, write policy, idempotency, and rollback visibility without filesystem side effects",
            "proofpack writer file IO outcome smoke case maps explicit observations to operation evidence without filesystem, storage, CLI, schema, or acceptance side effects",
            "proofpack writer file IO error reason smoke case models stable diagnostic reason codes without filesystem, storage, CLI, schema, or acceptance side effects",
            "proofpack writer target path policy smoke case rejects path injection and escape refs without filesystem, storage, CLI, schema, or acceptance side effects",
            "proofpack writer preflight integration smoke case composes explicit model inputs and fail-closed blockers without filesystem, storage, CLI, schema, or acceptance side effects",
            "proofpack writer active behavior smoke case models selected, attempted, completed, and failed side effects without filesystem, storage, CLI, schema, or acceptance side effects",
            "proofpack writer host path resolution smoke case models explicit host path observations and fail-closed blockers without filesystem, storage, CLI, schema, or acceptance side effects",
            "proofpack writer concrete path/storage policy smoke case models selected policy refs and host path evidence without filesystem, storage, CLI, schema, or acceptance side effects",
            "proofpack writer first active write slice smoke case writes exact bytes only to an explicit temporary target without runtime storage, CLI, schema, persisted evidence, gate, or acceptance side effects",
            "proofpack writer hash/reference integration smoke case composes declared digest, structural integrity, optional verification, and manifest self-digest evidence without filesystem, storage, CLI, schema, or acceptance side effects",
            "artifact hash smoke cases validate helper shape, exact-byte computation, explicit file IO hashing, and referenced artifact verification without runtime writes",
            "JSON output is opt-in only and does not imply a stable public contract",
        ],
        deferred_notes: vec![
            "baseline, waiver, and stored eval reports are not active",
            "schema validation and export adapters are not active",
            "runtime contract storage, CLI contract commands, Writer activation, gate writers, and proof writers remain inactive",
            "runtime proof storage and .punk/proofs remain inactive; only the explicit first active test-target write slice is active",
            "byte normalization and active proofpack writer referenced artifact verification file reads remain inactive; hash/reference integration is evidence-only",
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

fn eval_local_flow_event_writer_appends_project_event() -> SmokeEvalCaseResult {
    let temp_path = unique_smoke_temp_path();
    if let Err(error) = fs::create_dir_all(temp_path.join(".punk")) {
        return SmokeEvalCaseResult::fail(
            "eval_local_flow_event_writer_appends_project_event",
            "local flow event writer appends bounded .punk event evidence",
            format!("temporary project marker setup failed with {error:?}"),
        );
    }
    if let Err(error) = fs::write(
        temp_path.join(".punk/project.toml"),
        "schema_version = \"punk.project.v0.1\"\nproject_id = \"smoke-eval\"\n",
    ) {
        let _ = fs::remove_dir_all(&temp_path);
        return SmokeEvalCaseResult::fail(
            "eval_local_flow_event_writer_appends_project_event",
            "local flow event writer appends bounded .punk event evidence",
            format!("temporary project marker write failed with {error:?}"),
        );
    }

    let attempt =
        FlowInstance::new(FlowState::AwaitingApproval).attempt_transition(FlowCommand::StartRun);
    let draft = transition_attempt_event_draft(
        &attempt,
        "smoke_eval_local_flow",
        Some("work/goals/goal_start_runtime_automation_spine_v0_1.md"),
    );
    let append_result = append_local_flow_event(&temp_path, draft);
    let rendered = fs::read_to_string(temp_path.join(".punk/events/flow.jsonl"));
    let forbidden_runtime_created = temp_path.join(".punk/runs").exists()
        || temp_path.join(".punk/decisions").exists()
        || temp_path.join(".punk/proofs").exists();

    let result = match (append_result, rendered) {
        (Ok(record), Ok(text))
            if record.sequence == 1
                && record.event_id == "evt_0000000000000001"
                && text.lines().count() == 1
                && text.contains("\"kind\":\"transition_denied\"")
                && text.contains("\"status\":\"denied\"")
                && !text.contains("decision_id")
                && !forbidden_runtime_created =>
        {
            SmokeEvalCaseResult::pass(
                "eval_local_flow_event_writer_appends_project_event",
                "local flow event writer appends bounded .punk event evidence",
                "explicit temporary project root received one .punk/events/flow.jsonl event without decision/proof/runtime side effects",
            )
        }
        (Ok(record), Ok(text)) => SmokeEvalCaseResult::fail(
            "eval_local_flow_event_writer_appends_project_event",
            "local flow event writer appends bounded .punk event evidence",
            format!(
                "local event writer evidence drifted; event_id={} sequence={} lines={} forbidden_runtime_created={}",
                record.event_id,
                record.sequence,
                text.lines().count(),
                forbidden_runtime_created,
            ),
        ),
        (Err(error), _) => SmokeEvalCaseResult::fail(
            "eval_local_flow_event_writer_appends_project_event",
            "local flow event writer appends bounded .punk event evidence",
            format!("local event append failed with {error:?}"),
        ),
        (_, Err(error)) => SmokeEvalCaseResult::fail(
            "eval_local_flow_event_writer_appends_project_event",
            "local flow event writer appends bounded .punk event evidence",
            format!("local event log read failed with {error:?}"),
        ),
    };

    let _ = fs::remove_dir_all(&temp_path);
    result
}

fn eval_instruction_page_index_model_is_deterministic_and_advisory() -> SmokeEvalCaseResult {
    let nodes = default_instruction_page_index_nodes();
    let rendered = render_instruction_page_index_json(&nodes);
    let rendered_again = render_instruction_page_index_json(&nodes);
    let model_ok = rendered == rendered_again
        && rendered.contains("\"schema_version\": \"punk-instruction-page-index.v0.1\"")
        && rendered.contains("\"authority\": \"advisory\"")
        && rendered.contains("\"generated_from\": \"source_instruction_pages\"")
        && rendered.contains("\"source_ref\": \".punk/instructions/INDEX.md\"")
        && rendered.contains("\"source_ref\": \".punk/instructions/pages/init.md\"")
        && rendered.contains("\"status\": \"parked\"")
        && rendered.contains("\"module_id\": \"module-host\"")
        && !rendered.contains("raw_prompt")
        && !rendered.contains("transcript");

    if model_ok {
        SmokeEvalCaseResult::pass(
            "eval_instruction_page_index_model_is_deterministic_and_advisory",
            "instruction page-index model stays deterministic and advisory",
            "default instruction page-index tree renders stable source refs without views, vector DB, hidden truth, summaries, or runtime side effects",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_instruction_page_index_model_is_deterministic_and_advisory",
            "instruction page-index model stays deterministic and advisory",
            "instruction page-index model drifted from deterministic advisory source-ref tree",
        )
    }
}

fn eval_publishing_locate_resolves_local_pointer_without_side_effects() -> SmokeEvalCaseResult {
    let temp_path = unique_smoke_temp_path();
    let workspace_path = temp_path.join("external-publishing-workspace");
    let setup_result = fs::create_dir_all(temp_path.join(".punk"))
        .and_then(|()| fs::create_dir_all(&workspace_path))
        .and_then(|()| {
            fs::write(
                temp_path.join(PUBLISHING_BINDING_PATH),
                r#"schema_version = "punk.publishing.binding.v1"
project_id = "goalrail"
workspace_ref = "punk-publishing://project/goalrail"

[workspace]
workspace_ref_kind = "logical"
"#,
            )
        })
        .and_then(|()| {
            fs::write(
                temp_path.join(PUBLISHING_LOCAL_POINTER_PATH),
                format!(
                    r#"schema_version = "punk.publishing.local.v1"
workspace_ref = "punk-publishing://project/goalrail"
workspace_root = "{}"
"#,
                    workspace_path.display()
                ),
            )
        });

    if let Err(error) = setup_result {
        let _ = fs::remove_dir_all(&temp_path);
        return SmokeEvalCaseResult::fail(
            "eval_publishing_locate_resolves_local_pointer_without_side_effects",
            "publishing locate resolves local pointer without side effects",
            &format!("could not set up temporary publishing locate fixture: {error}"),
        );
    }

    let file_count_before = count_regular_files(&temp_path);
    let report = locate_publishing_workspace(&temp_path);
    let file_count_after = count_regular_files(&temp_path);
    let rendered_json = report.render_json();
    let ok = report.status() == PublishingLocateStatus::Located
        && report.workspace_ref() == Some("punk-publishing://project/goalrail")
        && report.workspace_root() == Some(workspace_path.as_path())
        && report.workspace_exists()
        && file_count_before == file_count_after
        && rendered_json.contains("\"writes_files\": false")
        && rendered_json.contains("\"external_side_effects\": false")
        && !temp_path.join(".punk/publishing/published").exists()
        && !temp_path.join(".punk/views").exists()
        && !rendered_json.contains("secret_value")
        && !rendered_json.contains("token_value");

    let _ = fs::remove_dir_all(&temp_path);

    if ok {
        SmokeEvalCaseResult::pass(
            "eval_publishing_locate_resolves_local_pointer_without_side_effects",
            "publishing locate resolves local pointer without side effects",
            "publishing locate resolved binding/local pointer config and existing workspace without writes, publishing, network/API, browser, credential, adapter, or bot behavior",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_publishing_locate_resolves_local_pointer_without_side_effects",
            "publishing locate resolves local pointer without side effects",
            "publishing locate resolver drifted from local-only read/locate boundary",
        )
    }
}

fn eval_pubpunk_inventory_assessment_model_is_side_effect_free() -> SmokeEvalCaseResult {
    let input = PubPunkInventoryInput::new(
        "v0.1",
        "contracts/pubpunk-inventory-cycle-0",
        "runs/pubpunk-inventory-assessment",
        "project/punk",
        "punk-publishing://project/punk",
    )
    .with_granted_capabilities(vec![PubPunkCapabilityGrant::AssessProvidedInventory])
    .with_expected_receipt_fields(vec![
        "module_id",
        "operation",
        "source_refs",
        "capability_grants",
        "side_effects",
    ])
    .with_items(vec![
        PubPunkInventoryItemInput::new(
            "publishing/posts/community-lab.md",
            PubPunkInventoryItemKind::PostDraft,
            PubPunkInventoryItemStatus::ReadyForReview,
        )
        .with_channel("github-discussions"),
        PubPunkInventoryItemInput::new(
            "publishing/posts/pubpunk-plan.md",
            PubPunkInventoryItemKind::PreviewDraft,
            PubPunkInventoryItemStatus::PreviewOnly,
        ),
        PubPunkInventoryItemInput::new(
            "publishing/publications/community-lab.md",
            PubPunkInventoryItemKind::PublicationReceipt,
            PubPunkInventoryItemStatus::Published,
        )
        .with_publication_receipt(true),
    ]);

    let assessment = assess_pubpunk_inventory(&input);
    let ok = assessment.status == PubPunkAssessmentStatus::Ready
        && assessment.authority == PubPunkAssessmentAuthority::Advisory
        && assessment.candidate_count == 2
        && assessment.receipt_gap_count == 2
        && !assessment.has_blockers()
        && assessment.boundary_flags.all_side_effect_flags_false()
        && assessment.refs.module_id == "pubpunk"
        && assessment.refs.publishing_workspace_ref == "punk-publishing://project/punk";

    if ok {
        SmokeEvalCaseResult::pass(
            "eval_pubpunk_inventory_assessment_model_is_side_effect_free",
            "PubPunk inventory assessment model stays module-owned and side-effect-free",
            "PubPunk inventory assessment counted caller-provided candidate refs and receipt gaps without filesystem, CLI, external API, credential, adapter, receipt writer, gate, proof, or acceptance behavior",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_pubpunk_inventory_assessment_model_is_side_effect_free",
            "PubPunk inventory assessment model stays module-owned and side-effect-free",
            "PubPunk inventory assessment model drifted from advisory no-IO module boundary",
        )
    }
}

fn eval_module_host_wraps_pubpunk_assessment_without_side_effects() -> SmokeEvalCaseResult {
    let pubpunk_input = PubPunkInventoryInput::new(
        "v0.1",
        "contracts/pubpunk-inventory-cycle-0",
        "runs/pubpunk-inventory-assessment",
        "project/punk",
        "punk-publishing://project/punk",
    )
    .with_granted_capabilities(vec![PubPunkCapabilityGrant::AssessProvidedInventory])
    .with_expected_receipt_fields(vec![
        "module_id",
        "operation",
        "source_refs",
        "capability_grants",
        "side_effects",
    ])
    .with_items(vec![PubPunkInventoryItemInput::new(
        "publishing/posts/community-lab.md",
        PubPunkInventoryItemKind::PostDraft,
        PubPunkInventoryItemStatus::ReadyForReview,
    )]);
    let pubpunk_assessment = assess_pubpunk_inventory(&pubpunk_input);

    let invocation = ModuleInvocationEnvelope::new(
        "pubpunk",
        "v0.1",
        "contracts/pubpunk-inventory-cycle-0",
        "runs/pubpunk-inventory-assessment",
        "project/punk",
        "assess_inventory",
    )
    .with_input_refs(vec!["publishing/posts/community-lab.md"])
    .with_granted_capabilities(vec![ModuleCapabilityGrant::AssessProvidedInput])
    .with_expected_receipt_fields(vec![
        "module_id",
        "operation",
        "source_refs",
        "capability_grants",
        "side_effects",
    ]);

    let output = ModuleOutputSummary::new(
        "work/module-assessments/pubpunk-inventory.md",
        ModuleOutputStatus::Ready,
        ModuleOutputAuthority::Advisory,
        pubpunk_assessment.findings.len(),
        ModuleOutputBoundaryFlags::side_effect_free(),
    );
    let envelope = wrap_module_assessment(&invocation, &output);

    let ok = pubpunk_assessment.status == PubPunkAssessmentStatus::Ready
        && pubpunk_assessment.authority == PubPunkAssessmentAuthority::Advisory
        && pubpunk_assessment
            .boundary_flags
            .all_side_effect_flags_false()
        && envelope.status == ModuleHostStatus::Ready
        && !envelope.has_blockers()
        && envelope.module_id == "pubpunk"
        && envelope.module_output_ref == "work/module-assessments/pubpunk-inventory.md"
        && envelope.boundary_flags.all_side_effect_flags_false();

    if ok {
        SmokeEvalCaseResult::pass(
            "eval_module_host_wraps_pubpunk_assessment_without_side_effects",
            "module-host envelope wraps advisory PubPunk assessment without side effects",
            "module-host envelope wrapped side-effect-free PubPunk advisory output as evidence without plugin loading, module invocation, filesystem IO, receipts, APIs, credentials, adapters, event-log mutation, gate, proof, or acceptance behavior",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_module_host_wraps_pubpunk_assessment_without_side_effects",
            "module-host envelope wraps advisory PubPunk assessment without side effects",
            "module-host envelope drifted from pure advisory no-IO module boundary",
        )
    }
}

fn eval_module_host_receipt_proposal_model_is_side_effect_free() -> SmokeEvalCaseResult {
    let pubpunk_input = PubPunkInventoryInput::new(
        "v0.1",
        "contracts/pubpunk-inventory-cycle-0",
        "runs/pubpunk-inventory-assessment",
        "project/punk",
        "punk-publishing://project/punk",
    )
    .with_granted_capabilities(vec![PubPunkCapabilityGrant::AssessProvidedInventory])
    .with_expected_receipt_fields(vec![
        "module_id",
        "operation",
        "source_refs",
        "capability_grants",
        "side_effects",
    ])
    .with_items(vec![PubPunkInventoryItemInput::new(
        "publishing/posts/community-lab.md",
        PubPunkInventoryItemKind::PostDraft,
        PubPunkInventoryItemStatus::ReadyForReview,
    )]);
    let pubpunk_assessment = assess_pubpunk_inventory(&pubpunk_input);

    let invocation = ModuleInvocationEnvelope::new(
        "pubpunk",
        "v0.1",
        "contracts/pubpunk-inventory-cycle-0",
        "runs/pubpunk-inventory-assessment",
        "project/punk",
        "assess_inventory",
    )
    .with_input_refs(vec!["publishing/posts/community-lab.md"])
    .with_granted_capabilities(vec![ModuleCapabilityGrant::AssessProvidedInput])
    .with_expected_receipt_fields(vec![
        "module_id",
        "module_version",
        "operation",
        "contract_ref",
        "run_ref",
        "project_ref",
        "input_refs",
        "capability_grants",
        "module_output_ref",
        "module_output_status",
        "boundary_flags",
        "side_effects",
        "host_validation",
    ]);

    let output = ModuleOutputSummary::new(
        "work/module-assessments/pubpunk-inventory.md",
        ModuleOutputStatus::Ready,
        ModuleOutputAuthority::Advisory,
        pubpunk_assessment.findings.len(),
        ModuleOutputBoundaryFlags::side_effect_free(),
    );
    let envelope = wrap_module_assessment(&invocation, &output);
    let proposal = propose_module_assessment_receipt(&invocation, &envelope);

    let ok = pubpunk_assessment.status == PubPunkAssessmentStatus::Ready
        && envelope.status == ModuleHostStatus::Ready
        && proposal.status == ModuleHostStatus::Ready
        && !proposal.has_blockers()
        && proposal
            .required_fields
            .contains(&ModuleReceiptProposalField::ModuleOutputRef)
        && proposal
            .covered_fields
            .contains(&ModuleReceiptProposalField::HostValidation)
        && proposal.boundary_flags.all_side_effect_flags_false()
        && envelope.boundary_flags.all_side_effect_flags_false();

    if ok {
        SmokeEvalCaseResult::pass(
            "eval_module_host_receipt_proposal_model_is_side_effect_free",
            "module-host receipt proposal model stays local and side-effect-free",
            "module-host receipt proposal modeled future host receipt fields from advisory PubPunk evidence without writing receipts, event logs, files, APIs, credentials, adapters, gate authority, proofpacks, or acceptance claims",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_module_host_receipt_proposal_model_is_side_effect_free",
            "module-host receipt proposal model stays local and side-effect-free",
            "module-host receipt proposal drifted from pure no-IO advisory boundary",
        )
    }
}

fn eval_module_host_side_effect_request_proposal_model_is_side_effect_free() -> SmokeEvalCaseResult
{
    let pubpunk_input = PubPunkInventoryInput::new(
        "v0.1",
        "contracts/pubpunk-inventory-cycle-0",
        "runs/pubpunk-inventory-assessment",
        "project/punk",
        "punk-publishing://project/punk",
    )
    .with_granted_capabilities(vec![PubPunkCapabilityGrant::AssessProvidedInventory])
    .with_expected_receipt_fields(vec![
        "module_id",
        "operation",
        "source_refs",
        "capability_grants",
        "side_effects",
    ])
    .with_items(vec![PubPunkInventoryItemInput::new(
        "publishing/posts/community-lab.md",
        PubPunkInventoryItemKind::PostDraft,
        PubPunkInventoryItemStatus::ReadyForReview,
    )]);
    let pubpunk_assessment = assess_pubpunk_inventory(&pubpunk_input);

    let invocation = ModuleInvocationEnvelope::new(
        "pubpunk",
        "v0.1",
        "contracts/pubpunk-inventory-cycle-0",
        "runs/pubpunk-inventory-assessment",
        "project/punk",
        "assess_inventory",
    )
    .with_input_refs(vec!["publishing/posts/community-lab.md"])
    .with_granted_capabilities(vec![ModuleCapabilityGrant::AssessProvidedInput])
    .with_expected_receipt_fields(vec![
        "module_id",
        "module_version",
        "operation",
        "contract_ref",
        "run_ref",
        "project_ref",
        "input_refs",
        "capability_grants",
        "module_output_ref",
        "module_output_status",
        "boundary_flags",
        "side_effects",
        "host_validation",
    ]);

    let output = ModuleOutputSummary::new(
        "work/module-assessments/pubpunk-inventory.md",
        ModuleOutputStatus::Ready,
        ModuleOutputAuthority::Advisory,
        pubpunk_assessment.findings.len(),
        ModuleOutputBoundaryFlags::side_effect_free(),
    );
    let envelope = wrap_module_assessment(&invocation, &output);
    let receipt_proposal = propose_module_assessment_receipt(&invocation, &envelope);
    let request = ModuleSideEffectRequestDraft::new(
        "work/module-side-effects/pubpunk-publish-community-lab.md",
        ModuleSideEffectKind::Publish,
    )
    .with_target_ref("publishing/channels/github-discussions-community-lab.md")
    .with_intent_ref("work/goals/goal_pubpunk_publish_cycle_0.md")
    .with_policy_ref("docs/modules/pubpunk.md")
    .with_receipt_proposal_ref("work/module-receipts/pubpunk-publish-community-lab.md")
    .with_adapter_ref("adapters/github-discussions")
    .with_payload_ref("publishing/posts/community-lab.md");

    let side_effect_proposal =
        propose_module_side_effect_request(&invocation, &receipt_proposal, &request);

    let ok = pubpunk_assessment.status == PubPunkAssessmentStatus::Ready
        && receipt_proposal.status == ModuleHostStatus::Ready
        && side_effect_proposal.status == ModuleHostStatus::Ready
        && !side_effect_proposal.has_blockers()
        && side_effect_proposal.kind == ModuleSideEffectKind::Publish
        && side_effect_proposal
            .covered_preconditions
            .contains(&ModuleSideEffectPrecondition::ReadyReceiptProposal)
        && side_effect_proposal
            .covered_preconditions
            .contains(&ModuleSideEffectPrecondition::AdapterInvocationReceipt)
        && side_effect_proposal
            .covered_preconditions
            .contains(&ModuleSideEffectPrecondition::GateOrPolicyApproval)
        && side_effect_proposal
            .boundary_flags
            .all_side_effect_flags_false();

    if ok {
        SmokeEvalCaseResult::pass(
            "eval_module_host_side_effect_request_proposal_model_is_side_effect_free",
            "module-host side-effect request proposal model stays local and side-effect-free",
            "module-host side-effect request proposal modeled future publish preconditions from advisory PubPunk evidence without invoking adapters, publishing, creating pull requests, writing receipts, event logs, files, APIs, credentials, gate authority, proofpacks, or acceptance claims",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_module_host_side_effect_request_proposal_model_is_side_effect_free",
            "module-host side-effect request proposal model stays local and side-effect-free",
            "module-host side-effect request proposal drifted from pure no-IO advisory boundary",
        )
    }
}

fn eval_module_host_policy_gate_preflight_model_is_side_effect_free() -> SmokeEvalCaseResult {
    let pubpunk_input = PubPunkInventoryInput::new(
        "v0.1",
        "contracts/pubpunk-inventory-cycle-0",
        "runs/pubpunk-inventory-assessment",
        "project/punk",
        "punk-publishing://project/punk",
    )
    .with_granted_capabilities(vec![PubPunkCapabilityGrant::AssessProvidedInventory])
    .with_expected_receipt_fields(vec![
        "module_id",
        "operation",
        "source_refs",
        "capability_grants",
        "side_effects",
    ])
    .with_items(vec![PubPunkInventoryItemInput::new(
        "publishing/posts/community-lab.md",
        PubPunkInventoryItemKind::PostDraft,
        PubPunkInventoryItemStatus::ReadyForReview,
    )]);
    let pubpunk_assessment = assess_pubpunk_inventory(&pubpunk_input);

    let invocation = ModuleInvocationEnvelope::new(
        "pubpunk",
        "v0.1",
        "contracts/pubpunk-inventory-cycle-0",
        "runs/pubpunk-inventory-assessment",
        "project/punk",
        "assess_inventory",
    )
    .with_input_refs(vec!["publishing/posts/community-lab.md"])
    .with_granted_capabilities(vec![ModuleCapabilityGrant::AssessProvidedInput])
    .with_expected_receipt_fields(vec![
        "module_id",
        "module_version",
        "operation",
        "contract_ref",
        "run_ref",
        "project_ref",
        "input_refs",
        "capability_grants",
        "module_output_ref",
        "module_output_status",
        "boundary_flags",
        "side_effects",
        "host_validation",
    ]);

    let output = ModuleOutputSummary::new(
        "work/module-assessments/pubpunk-inventory.md",
        ModuleOutputStatus::Ready,
        ModuleOutputAuthority::Advisory,
        pubpunk_assessment.findings.len(),
        ModuleOutputBoundaryFlags::side_effect_free(),
    );
    let envelope = wrap_module_assessment(&invocation, &output);
    let receipt_proposal = propose_module_assessment_receipt(&invocation, &envelope);
    let request = ModuleSideEffectRequestDraft::new(
        "work/module-side-effects/pubpunk-publish-community-lab.md",
        ModuleSideEffectKind::Publish,
    )
    .with_target_ref("publishing/channels/github-discussions-community-lab.md")
    .with_intent_ref("work/goals/goal_pubpunk_publish_cycle_0.md")
    .with_policy_ref("docs/modules/pubpunk.md")
    .with_receipt_proposal_ref("work/module-receipts/pubpunk-publish-community-lab.md")
    .with_adapter_ref("adapters/github-discussions")
    .with_payload_ref("publishing/posts/community-lab.md");
    let side_effect_proposal =
        propose_module_side_effect_request(&invocation, &receipt_proposal, &request);
    let policy_gate_draft = ModulePolicyGatePreflightDraft::new(
        "work/module-policy-gate/pubpunk-publish-community-lab.md",
    )
    .with_policy_ref("docs/modules/pubpunk.md")
    .with_gate_input_ref("work/module-gate-inputs/pubpunk-publish-community-lab.md")
    .with_side_effect_receipt_proposal_ref("work/module-receipts/pubpunk-publish-community-lab.md")
    .with_adapter_invocation_receipt_ref("work/module-receipts/github-discussions-invocation.md")
    .with_payload_ref("publishing/posts/community-lab.md")
    .with_proof_requirement_ref("work/module-proof-requirements/pubpunk-publish-community-lab.md");

    let policy_gate_preflight =
        preflight_module_policy_gate(&side_effect_proposal, &policy_gate_draft);

    let ok = pubpunk_assessment.status == PubPunkAssessmentStatus::Ready
        && receipt_proposal.status == ModuleHostStatus::Ready
        && side_effect_proposal.status == ModuleHostStatus::Ready
        && policy_gate_preflight.status == ModuleHostStatus::Ready
        && !policy_gate_preflight.has_blockers()
        && policy_gate_preflight
            .covered_requirements
            .contains(&ModulePolicyGatePreflightRequirement::GateInputRef)
        && policy_gate_preflight
            .covered_requirements
            .contains(&ModulePolicyGatePreflightRequirement::SideEffectReceiptProposalRef)
        && policy_gate_preflight
            .covered_requirements
            .contains(&ModulePolicyGatePreflightRequirement::AdapterInvocationReceiptRef)
        && policy_gate_preflight
            .covered_requirements
            .contains(&ModulePolicyGatePreflightRequirement::ProofRequirementRef)
        && policy_gate_preflight
            .boundary_flags
            .all_side_effect_flags_false();

    if ok {
        SmokeEvalCaseResult::pass(
            "eval_module_host_policy_gate_preflight_model_is_side_effect_free",
            "module-host policy gate preflight model stays local and side-effect-free",
            "module-host policy gate preflight modeled future policy evidence readiness without invoking policy engines, invoking gate, writing decisions, invoking adapters, publishing, creating pull requests, writing receipts, event logs, files, APIs, credentials, proofpacks, or acceptance claims",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_module_host_policy_gate_preflight_model_is_side_effect_free",
            "module-host policy gate preflight model stays local and side-effect-free",
            "module-host policy gate preflight drifted from pure no-IO advisory boundary",
        )
    }
}

fn eval_module_host_side_effect_receipt_writer_preflight_model_is_side_effect_free(
) -> SmokeEvalCaseResult {
    let pubpunk_input = PubPunkInventoryInput::new(
        "v0.1",
        "contracts/pubpunk-inventory-cycle-0",
        "runs/pubpunk-inventory-assessment",
        "project/punk",
        "punk-publishing://project/punk",
    )
    .with_granted_capabilities(vec![PubPunkCapabilityGrant::AssessProvidedInventory])
    .with_expected_receipt_fields(vec![
        "module_id",
        "operation",
        "source_refs",
        "capability_grants",
        "side_effects",
    ])
    .with_items(vec![PubPunkInventoryItemInput::new(
        "publishing/posts/community-lab.md",
        PubPunkInventoryItemKind::PostDraft,
        PubPunkInventoryItemStatus::ReadyForReview,
    )]);
    let pubpunk_assessment = assess_pubpunk_inventory(&pubpunk_input);

    let invocation = ModuleInvocationEnvelope::new(
        "pubpunk",
        "v0.1",
        "contracts/pubpunk-inventory-cycle-0",
        "runs/pubpunk-inventory-assessment",
        "project/punk",
        "assess_inventory",
    )
    .with_input_refs(vec!["publishing/posts/community-lab.md"])
    .with_granted_capabilities(vec![ModuleCapabilityGrant::AssessProvidedInput])
    .with_expected_receipt_fields(vec![
        "module_id",
        "module_version",
        "operation",
        "contract_ref",
        "run_ref",
        "project_ref",
        "input_refs",
        "capability_grants",
        "module_output_ref",
        "module_output_status",
        "boundary_flags",
        "side_effects",
        "host_validation",
    ]);

    let output = ModuleOutputSummary::new(
        "work/module-assessments/pubpunk-inventory.md",
        ModuleOutputStatus::Ready,
        ModuleOutputAuthority::Advisory,
        pubpunk_assessment.findings.len(),
        ModuleOutputBoundaryFlags::side_effect_free(),
    );
    let envelope = wrap_module_assessment(&invocation, &output);
    let receipt_proposal = propose_module_assessment_receipt(&invocation, &envelope);
    let request = ModuleSideEffectRequestDraft::new(
        "work/module-side-effects/pubpunk-publish-community-lab.md",
        ModuleSideEffectKind::Publish,
    )
    .with_target_ref("publishing/channels/github-discussions-community-lab.md")
    .with_intent_ref("work/goals/goal_pubpunk_publish_cycle_0.md")
    .with_policy_ref("docs/modules/pubpunk.md")
    .with_receipt_proposal_ref("work/module-receipts/pubpunk-publish-community-lab.md")
    .with_adapter_ref("adapters/github-discussions")
    .with_payload_ref("publishing/posts/community-lab.md");
    let side_effect_proposal =
        propose_module_side_effect_request(&invocation, &receipt_proposal, &request);
    let policy_gate_draft = ModulePolicyGatePreflightDraft::new(
        "work/module-policy-gate/pubpunk-publish-community-lab.md",
    )
    .with_policy_ref("docs/modules/pubpunk.md")
    .with_gate_input_ref("work/module-gate-inputs/pubpunk-publish-community-lab.md")
    .with_side_effect_receipt_proposal_ref("work/module-receipts/pubpunk-publish-community-lab.md")
    .with_adapter_invocation_receipt_ref("work/module-receipts/github-discussions-invocation.md")
    .with_payload_ref("publishing/posts/community-lab.md")
    .with_proof_requirement_ref("work/module-proof-requirements/pubpunk-publish-community-lab.md");
    let policy_gate_preflight =
        preflight_module_policy_gate(&side_effect_proposal, &policy_gate_draft);
    let receipt_writer_draft = ModuleSideEffectReceiptWriterPreflightDraft::new(
        "work/module-receipt-writer/pubpunk-publish-community-lab.md",
    )
    .with_policy_gate_preflight_ref("work/module-policy-gate/pubpunk-publish-community-lab.md")
    .with_receipt_target_ref("work/module-receipts/pubpunk-publish-community-lab.md")
    .with_storage_ref(".punk/runs/pubpunk-publish-community-lab")
    .with_operation_evidence_ref("work/module-operation-evidence/pubpunk-publish-community-lab.md")
    .with_idempotency_ref("work/module-idempotency/pubpunk-publish-community-lab.md")
    .with_rollback_ref("work/module-rollback/pubpunk-publish-community-lab.md")
    .with_error_ref("work/module-errors/pubpunk-publish-community-lab.md")
    .with_adapter_invocation_receipt_ref("work/module-receipts/github-discussions-invocation.md")
    .with_payload_ref("publishing/posts/community-lab.md");

    let receipt_writer_preflight =
        preflight_module_side_effect_receipt_writer(&policy_gate_preflight, &receipt_writer_draft);

    let ok = pubpunk_assessment.status == PubPunkAssessmentStatus::Ready
        && receipt_proposal.status == ModuleHostStatus::Ready
        && side_effect_proposal.status == ModuleHostStatus::Ready
        && policy_gate_preflight.status == ModuleHostStatus::Ready
        && receipt_writer_preflight.status == ModuleHostStatus::Ready
        && !receipt_writer_preflight.has_blockers()
        && receipt_writer_preflight
            .covered_requirements
            .contains(&ModuleSideEffectReceiptWriterPreflightRequirement::ReadyPolicyGatePreflight)
        && receipt_writer_preflight
            .covered_requirements
            .contains(&ModuleSideEffectReceiptWriterPreflightRequirement::StorageRef)
        && receipt_writer_preflight
            .covered_requirements
            .contains(&ModuleSideEffectReceiptWriterPreflightRequirement::OperationEvidenceRef)
        && receipt_writer_preflight
            .covered_requirements
            .contains(&ModuleSideEffectReceiptWriterPreflightRequirement::IdempotencyRef)
        && receipt_writer_preflight
            .covered_requirements
            .contains(&ModuleSideEffectReceiptWriterPreflightRequirement::RollbackRef)
        && receipt_writer_preflight
            .covered_requirements
            .contains(&ModuleSideEffectReceiptWriterPreflightRequirement::ErrorRef)
        && receipt_writer_preflight.covered_requirements.contains(
            &ModuleSideEffectReceiptWriterPreflightRequirement::AdapterInvocationReceiptRef,
        )
        && receipt_writer_preflight
            .boundary_flags
            .all_side_effect_flags_false();

    if ok {
        SmokeEvalCaseResult::pass(
            "eval_module_host_side_effect_receipt_writer_preflight_model_is_side_effect_free",
            "module-host side-effect receipt writer preflight model stays local and side-effect-free",
            "module-host side-effect receipt writer preflight modeled future receipt writer readiness without writing receipts, mutating event logs, reading or writing files, invoking adapters, invoking policy engines, invoking gate, publishing, creating pull requests, calling APIs, reading credentials, writing proofpacks, or claiming acceptance",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_module_host_side_effect_receipt_writer_preflight_model_is_side_effect_free",
            "module-host side-effect receipt writer preflight model stays local and side-effect-free",
            "module-host side-effect receipt writer preflight drifted from pure no-IO advisory boundary",
        )
    }
}

fn eval_module_host_side_effect_receipt_writer_active_behavior_model_is_side_effect_free(
) -> SmokeEvalCaseResult {
    let pubpunk_input = PubPunkInventoryInput::new(
        "v0.1",
        "contracts/pubpunk-inventory-cycle-0",
        "runs/pubpunk-inventory-assessment",
        "project/punk",
        "punk-publishing://project/punk",
    )
    .with_granted_capabilities(vec![PubPunkCapabilityGrant::AssessProvidedInventory])
    .with_expected_receipt_fields(vec![
        "module_id",
        "operation",
        "source_refs",
        "capability_grants",
        "side_effects",
    ])
    .with_items(vec![PubPunkInventoryItemInput::new(
        "publishing/posts/community-lab.md",
        PubPunkInventoryItemKind::PostDraft,
        PubPunkInventoryItemStatus::ReadyForReview,
    )]);
    let pubpunk_assessment = assess_pubpunk_inventory(&pubpunk_input);

    let invocation = ModuleInvocationEnvelope::new(
        "pubpunk",
        "v0.1",
        "contracts/pubpunk-inventory-cycle-0",
        "runs/pubpunk-inventory-assessment",
        "project/punk",
        "assess_inventory",
    )
    .with_input_refs(vec!["publishing/posts/community-lab.md"])
    .with_granted_capabilities(vec![ModuleCapabilityGrant::AssessProvidedInput])
    .with_expected_receipt_fields(vec![
        "module_id",
        "module_version",
        "operation",
        "contract_ref",
        "run_ref",
        "project_ref",
        "input_refs",
        "capability_grants",
        "module_output_ref",
        "module_output_status",
        "boundary_flags",
        "side_effects",
        "host_validation",
    ]);

    let output = ModuleOutputSummary::new(
        "work/module-assessments/pubpunk-inventory.md",
        ModuleOutputStatus::Ready,
        ModuleOutputAuthority::Advisory,
        pubpunk_assessment.findings.len(),
        ModuleOutputBoundaryFlags::side_effect_free(),
    );
    let envelope = wrap_module_assessment(&invocation, &output);
    let receipt_proposal = propose_module_assessment_receipt(&invocation, &envelope);
    let request = ModuleSideEffectRequestDraft::new(
        "work/module-side-effects/pubpunk-publish-community-lab.md",
        ModuleSideEffectKind::Publish,
    )
    .with_target_ref("publishing/channels/github-discussions-community-lab.md")
    .with_intent_ref("work/goals/goal_pubpunk_publish_cycle_0.md")
    .with_policy_ref("docs/modules/pubpunk.md")
    .with_receipt_proposal_ref("work/module-receipts/pubpunk-publish-community-lab.md")
    .with_adapter_ref("adapters/github-discussions")
    .with_payload_ref("publishing/posts/community-lab.md");
    let side_effect_proposal =
        propose_module_side_effect_request(&invocation, &receipt_proposal, &request);
    let policy_gate_draft = ModulePolicyGatePreflightDraft::new(
        "work/module-policy-gate/pubpunk-publish-community-lab.md",
    )
    .with_policy_ref("docs/modules/pubpunk.md")
    .with_gate_input_ref("work/module-gate-inputs/pubpunk-publish-community-lab.md")
    .with_side_effect_receipt_proposal_ref("work/module-receipts/pubpunk-publish-community-lab.md")
    .with_adapter_invocation_receipt_ref("work/module-receipts/github-discussions-invocation.md")
    .with_payload_ref("publishing/posts/community-lab.md")
    .with_proof_requirement_ref("work/module-proof-requirements/pubpunk-publish-community-lab.md");
    let policy_gate_preflight =
        preflight_module_policy_gate(&side_effect_proposal, &policy_gate_draft);
    let receipt_writer_draft = ModuleSideEffectReceiptWriterPreflightDraft::new(
        "work/module-receipt-writer/pubpunk-publish-community-lab.md",
    )
    .with_policy_gate_preflight_ref("work/module-policy-gate/pubpunk-publish-community-lab.md")
    .with_receipt_target_ref("work/module-receipts/pubpunk-publish-community-lab.md")
    .with_storage_ref(".punk/runs/pubpunk-publish-community-lab")
    .with_operation_evidence_ref("work/module-operation-evidence/pubpunk-publish-community-lab.md")
    .with_idempotency_ref("work/module-idempotency/pubpunk-publish-community-lab.md")
    .with_rollback_ref("work/module-rollback/pubpunk-publish-community-lab.md")
    .with_error_ref("work/module-errors/pubpunk-publish-community-lab.md")
    .with_adapter_invocation_receipt_ref("work/module-receipts/github-discussions-invocation.md")
    .with_payload_ref("publishing/posts/community-lab.md");
    let receipt_writer_preflight =
        preflight_module_side_effect_receipt_writer(&policy_gate_preflight, &receipt_writer_draft);

    let planned =
        model_module_side_effect_receipt_writer_active_behavior(&receipt_writer_preflight, None);
    let written = model_module_side_effect_receipt_writer_active_behavior(
        &receipt_writer_preflight,
        Some(&ModuleSideEffectReceiptWriterObservation::target_missing_written()),
    );
    let idempotent = model_module_side_effect_receipt_writer_active_behavior(
        &receipt_writer_preflight,
        Some(&ModuleSideEffectReceiptWriterObservation::target_exists_matching()),
    );
    let conflict = model_module_side_effect_receipt_writer_active_behavior(
        &receipt_writer_preflight,
        Some(&ModuleSideEffectReceiptWriterObservation::target_exists_different()),
    );
    let write_failed = model_module_side_effect_receipt_writer_active_behavior(
        &receipt_writer_preflight,
        Some(&ModuleSideEffectReceiptWriterObservation::write_failed()),
    );
    let partial = model_module_side_effect_receipt_writer_active_behavior(
        &receipt_writer_preflight,
        Some(&ModuleSideEffectReceiptWriterObservation::partial_or_ambiguous()),
    );

    let ok = receipt_writer_preflight.status == ModuleHostStatus::Ready
        && planned.status == ModuleHostStatus::Ready
        && planned.outcome == ModuleSideEffectReceiptWriterOutcome::PlannedOnly
        && planned
            .selected_steps
            .contains(&ModuleSideEffectReceiptWriterModeledStep::ReceiptWrite)
        && planned.attempted_steps.is_empty()
        && written.outcome == ModuleSideEffectReceiptWriterOutcome::Written
        && written.selected_step_completed(ModuleSideEffectReceiptWriterModeledStep::ReceiptWrite)
        && idempotent.outcome == ModuleSideEffectReceiptWriterOutcome::Idempotent
        && !idempotent
            .selected_step_was_attempted(ModuleSideEffectReceiptWriterModeledStep::ReceiptWrite)
        && conflict.outcome == ModuleSideEffectReceiptWriterOutcome::Conflict
        && conflict.has_conflict()
        && conflict.error_visible()
        && write_failed.outcome == ModuleSideEffectReceiptWriterOutcome::WriteFailed
        && write_failed.has_write_failure()
        && write_failed.rollback_visible()
        && write_failed.error_visible()
        && partial.outcome == ModuleSideEffectReceiptWriterOutcome::PartialOrAmbiguous
        && partial.has_partial_or_ambiguous_state()
        && partial.rollback_visible()
        && partial.error_visible()
        && partial.boundary_flags.all_side_effect_flags_false()
        && partial.is_evidence_only();

    if ok {
        SmokeEvalCaseResult::pass(
            "eval_module_host_side_effect_receipt_writer_active_behavior_model_is_side_effect_free",
            "module-host side-effect receipt writer active behavior model stays local and side-effect-free",
            "module-host side-effect receipt writer active behavior modeled planned, written, idempotent, conflict, write-failed, and partial outcomes without writing receipts, mutating event logs, persisting operation evidence, reading or writing files, invoking adapters, invoking policy engines, invoking gate, publishing, creating pull requests, calling APIs, reading credentials, writing proofpacks, or claiming acceptance",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_module_host_side_effect_receipt_writer_active_behavior_model_is_side_effect_free",
            "module-host side-effect receipt writer active behavior model stays local and side-effect-free",
            "module-host side-effect receipt writer active behavior drifted from pure no-IO advisory boundary",
        )
    }
}

fn eval_project_init_creates_level0_manual_memory_scaffold() -> SmokeEvalCaseResult {
    let temp_path = unique_smoke_temp_path();
    if let Err(error) = fs::create_dir_all(&temp_path) {
        return SmokeEvalCaseResult::fail(
            "eval_project_init_creates_level0_manual_memory_scaffold",
            "greenfield project init creates Level 0 manual memory scaffold",
            format!("temporary project root setup failed with {error:?}"),
        );
    }

    let project_id = ProjectId::parse("weekend-project").expect("smoke project id should parse");
    let report = init_level0_project(&temp_path, project_id);
    let status_path = temp_path.join(".punk/memory/STATUS.md");
    let goal_path = temp_path.join(".punk/memory/goals/goal_initial_project_setup.md");
    let instruction_index_path = temp_path.join(INSTRUCTIONS_INDEX_PATH);
    let status_text = fs::read_to_string(&status_path);
    let goal_text = fs::read_to_string(&goal_path);
    let instruction_index_text = fs::read_to_string(&instruction_index_path);
    let marker_text = fs::read_to_string(temp_path.join(".punk/project.toml"));
    let status_exists = status_path.is_file();
    let goal_exists = goal_path.is_file();
    let instructions_ok = instruction_index_path.is_file()
        && temp_path
            .join(".punk/instructions/pages/getting-started.md")
            .is_file()
        && temp_path
            .join(".punk/instructions/pages/layout.md")
            .is_file()
        && temp_path.join(".punk/instructions/pages/init.md").is_file()
        && temp_path
            .join(".punk/instructions/pages/modules.md")
            .is_file()
        && temp_path
            .join(".punk/instructions/pages/authority.md")
            .is_file()
        && temp_path
            .join(".punk/instructions/modules/README.md")
            .is_file();
    let punk_marker_ok = temp_path.join(".punk/README.md").is_file()
        && temp_path.join(".punk/project.toml").is_file();
    let punk_runtime_store_absent = !temp_path.join(".punk/events").exists()
        && !temp_path.join(".punk/runtime").exists()
        && !temp_path.join(".punk/cache").exists()
        && !temp_path.join(".punk/contracts").exists()
        && !temp_path.join(".punk/runs").exists()
        && !temp_path.join(".punk/evals").exists()
        && !temp_path.join(".punk/decisions").exists()
        && !temp_path.join(".punk/proofs").exists()
        && !temp_path.join(".punk/views").exists()
        && !temp_path.join(".punk/indexes").exists();
    let root_layout_absent = !temp_path.join("work").exists()
        && !temp_path.join("knowledge").exists()
        && !temp_path.join("docs").exists()
        && !temp_path.join("docs/adr").exists()
        && !temp_path.join("publishing").exists();
    let cleanup_ok = fs::remove_dir_all(&temp_path).is_ok();

    let status_ok = status_text.as_ref().is_ok_and(|text| {
        text.contains("dogfooding_level: 0")
            && text.contains("project_id: \"weekend-project\"")
            && text.contains("entry_mode: greenfield")
            && text.contains("selected_next: \".punk/memory/goals/goal_initial_project_setup.md\"")
    });
    let goal_ok = goal_text.as_ref().is_ok_and(|text| {
        text.contains("id: goal_initial_project_setup")
            && text.contains("project_id: \"weekend-project\"")
            && text.contains("entry_mode: greenfield")
            && text.contains("- \".punk/instructions/**\"")
    });
    let instruction_index_ok = instruction_index_text.as_ref().is_ok_and(|text| {
        text.contains("# Punk Instructions")
            && text.contains("[Getting started](pages/getting-started.md)")
            && text.contains(".punk/views/instructions/page-index.json")
            && text.contains("rebuildable and advisory")
    });
    let marker_ok = marker_text.as_ref().is_ok_and(|text| {
        text.contains("project_id = \"weekend-project\"")
            && text.contains("entry_mode = \"greenfield\"")
            && text.contains("runtime_persistence = \"inactive\"")
            && text.contains("[memory]")
            && text.contains("layout = \"compact\"")
            && text.contains("root = \".punk/memory\"")
            && text.contains("[instructions]")
            && text.contains("root = \".punk/instructions\"")
            && text.contains("index = \".punk/instructions/INDEX.md\"")
            && text.contains("page_index_view = \".punk/views/instructions/page-index.json\"")
            && text.contains("views_active = false")
            && text.contains("[runtime]")
            && text.contains("active = false")
            && text.contains("root = \".punk/runtime\"")
    });
    let artifact_ok = report.artifacts().iter().any(|artifact| {
        artifact.repo_relative_path() == ".punk/memory/STATUS.md"
            && artifact.kind() == ProjectInitArtifactKind::File
            && artifact.status() == ProjectInitArtifactStatus::Created
    }) && report.artifacts().iter().any(|artifact| {
        artifact.repo_relative_path() == ".punk/memory/goals/goal_initial_project_setup.md"
            && artifact.kind() == ProjectInitArtifactKind::File
            && artifact.status() == ProjectInitArtifactStatus::Created
    }) && report.artifacts().iter().any(|artifact| {
        artifact.repo_relative_path() == ".punk"
            && artifact.kind() == ProjectInitArtifactKind::Directory
            && artifact.status() == ProjectInitArtifactStatus::Created
    }) && report.artifacts().iter().any(|artifact| {
        artifact.repo_relative_path() == ".punk/project.toml"
            && artifact.kind() == ProjectInitArtifactKind::File
            && artifact.status() == ProjectInitArtifactStatus::Created
    }) && report.artifacts().iter().any(|artifact| {
        artifact.repo_relative_path() == INSTRUCTIONS_INDEX_PATH
            && artifact.kind() == ProjectInitArtifactKind::File
            && artifact.status() == ProjectInitArtifactStatus::Created
    });
    let scaffold_ok = !report.blocked()
        && report.exit_code() == 0
        && status_exists
        && goal_exists
        && instructions_ok
        && punk_marker_ok
        && punk_runtime_store_absent
        && root_layout_absent
        && status_ok
        && goal_ok
        && instruction_index_ok
        && marker_ok
        && artifact_ok
        && cleanup_ok;

    if scaffold_ok {
        SmokeEvalCaseResult::pass(
            "eval_project_init_creates_level0_manual_memory_scaffold",
            "greenfield project init creates Level 0 manual memory scaffold",
            "greenfield Level 0 scaffold created compact .punk/memory files and thin .punk/instructions source pages with project_id, entry_mode, and .punk marker files while leaving root-level Punk memory dirs, brownfield/grayfield, .punk/views, and .punk runtime stores absent",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_project_init_creates_level0_manual_memory_scaffold",
            "greenfield project init creates Level 0 manual memory scaffold",
            format!(
                "init scaffold drifted; blocked={} exit_code={} status_exists={} goal_exists={} instructions_ok={} punk_marker_ok={} punk_runtime_store_absent={} root_layout_absent={} status_ok={} goal_ok={} instruction_index_ok={} marker_ok={} artifact_ok={} cleanup_ok={} status_read={:?} goal_read={:?} instruction_index_read={:?} marker_read={:?}",
                report.blocked(),
                report.exit_code(),
                status_exists,
                goal_exists,
                instructions_ok,
                punk_marker_ok,
                punk_runtime_store_absent,
                root_layout_absent,
                status_ok,
                goal_ok,
                instruction_index_ok,
                marker_ok,
                artifact_ok,
                cleanup_ok,
                status_text.err(),
                goal_text.err(),
                instruction_index_text.err(),
                marker_text.err()
            ),
        )
    }
}

fn eval_project_init_brownfield_scaffold_shape() -> SmokeEvalCaseResult {
    let temp_path = unique_smoke_temp_path();
    if let Err(error) = fs::create_dir_all(&temp_path) {
        return SmokeEvalCaseResult::fail(
            "eval_project_init_brownfield_scaffold_shape",
            "brownfield project init creates advisory reconstruction scaffold",
            format!("temporary project root setup failed with {error:?}"),
        );
    }

    let existing_source_setup = fs::create_dir_all(temp_path.join("src"))
        .and_then(|_| fs::write(temp_path.join("src/main.rs"), b"fn main() {}\n"))
        .and_then(|_| fs::write(temp_path.join("README.md"), b"# Existing project\n"));
    if let Err(error) = existing_source_setup {
        let _ = fs::remove_dir_all(&temp_path);
        return SmokeEvalCaseResult::fail(
            "eval_project_init_brownfield_scaffold_shape",
            "brownfield project init creates advisory reconstruction scaffold",
            format!("temporary source setup failed with {error:?}"),
        );
    }

    let project_id = ProjectId::parse("weekend-project").expect("smoke project id should parse");
    let report = init_project(&temp_path, project_id, ProjectInitEntryMode::Brownfield);
    let status_path = temp_path.join(".punk/memory/STATUS.md");
    let goal_path = temp_path.join(".punk/memory/goals/goal_brownfield_reconstruction_baseline.md");
    let reconstruction_root = temp_path.join(".punk/memory/reconstruction");
    let source_manifest_path = reconstruction_root.join("source-corpus-manifest.md");
    let claim_ledger_path = reconstruction_root.join("claim-ledger.md");
    let contract_readiness_path = reconstruction_root.join("contract-readiness.md");
    let instruction_index_path = temp_path.join(INSTRUCTIONS_INDEX_PATH);
    let status_text = fs::read_to_string(&status_path);
    let instruction_index_text = fs::read_to_string(&instruction_index_path);
    let marker_text = fs::read_to_string(temp_path.join(".punk/project.toml"));
    let source_manifest_text = fs::read_to_string(&source_manifest_path);
    let claim_ledger_text = fs::read_to_string(&claim_ledger_path);
    let contract_readiness_text = fs::read_to_string(&contract_readiness_path);

    let status_exists = status_path.is_file();
    let goal_exists = goal_path.is_file();
    let reconstruction_placeholders_ok = reconstruction_root.join("README.md").is_file()
        && source_manifest_path.is_file()
        && claim_ledger_path.is_file()
        && reconstruction_root.join("unknowns.md").is_file()
        && reconstruction_root.join("contradictions.md").is_file()
        && contract_readiness_path.is_file();
    let instructions_ok = instruction_index_path.is_file()
        && temp_path.join(".punk/instructions/pages/init.md").is_file()
        && temp_path
            .join(".punk/instructions/pages/modules.md")
            .is_file()
        && temp_path
            .join(".punk/instructions/modules/README.md")
            .is_file();
    let runtime_store_absent = !temp_path.join(".punk/events").exists()
        && !temp_path.join(".punk/runtime").exists()
        && !temp_path.join(".punk/cache").exists()
        && !temp_path.join(".punk/contracts").exists()
        && !temp_path.join(".punk/runs").exists()
        && !temp_path.join(".punk/evals").exists()
        && !temp_path.join(".punk/decisions").exists()
        && !temp_path.join(".punk/proofs").exists()
        && !temp_path.join(".punk/views").exists()
        && !temp_path.join(".punk/indexes").exists();
    let root_layout_absent = !temp_path.join("work").exists()
        && !temp_path.join("knowledge").exists()
        && !temp_path.join("docs").exists()
        && !temp_path.join("docs/adr").exists()
        && !temp_path.join("publishing").exists();
    let status_ok = status_text.as_ref().is_ok_and(|text| {
        text.contains("entry_mode: brownfield")
            && text.contains("reconstruction_status: not_started")
            && text.contains(
                "selected_next: \".punk/memory/goals/goal_brownfield_reconstruction_baseline.md\"",
            )
            && text.contains("No project knowledge has been reconstructed yet.")
            && text.contains("Existing code/docs/history are not Punk truth.")
            && text
                .contains("Future reconstruction artifacts are advisory candidates until reviewed.")
    });
    let marker_ok = marker_text.as_ref().is_ok_and(|text| {
        text.contains("project_id = \"weekend-project\"")
            && text.contains("entry_mode = \"brownfield\"")
            && text.contains("[memory]")
            && text.contains("layout = \"compact\"")
            && text.contains("root = \".punk/memory\"")
            && text.contains("[instructions]")
            && text.contains("root = \".punk/instructions\"")
            && text.contains("index = \".punk/instructions/INDEX.md\"")
            && text.contains("views_active = false")
            && text.contains("[runtime]")
            && text.contains("active = false")
            && text.contains("[brownfield]")
            && text.contains("reconstruction_status = \"not_started\"")
            && text.contains("authority = \"advisory_candidates_only\"")
    });
    let placeholders_ok = source_manifest_text.as_ref().is_ok_and(|text| {
        text.contains(
            "Future source-linked inventory of files, docs, tests, CI, schemas, and history sources.",
        ) && text.contains("Status: not_started.")
            && !text.contains("src/main.rs")
            && !text.contains("Existing project")
    }) && claim_ledger_text.as_ref().is_ok_and(|text| {
        text.contains("No claims are accepted automatically.")
            && text.contains("No claims have been reconstructed or reviewed yet.")
            && !text.contains("fn main")
            && !text.contains("Existing project")
    }) && contract_readiness_text.as_ref().is_ok_and(|text| {
        text.contains("This is not a contract.")
            && text.contains("This is not a gate decision.")
            && text.contains("This is not proof.")
    });
    let instruction_index_ok = instruction_index_text.as_ref().is_ok_and(|text| {
        text.contains("# Punk Instructions")
            && text.contains("Module-specific instruction trees live under `modules/<module-id>/`")
            && text.contains("No module is active just because this directory exists.")
    });
    let artifact_ok = report.artifacts().iter().any(|artifact| {
        artifact.repo_relative_path() == ".punk/memory/STATUS.md"
            && artifact.kind() == ProjectInitArtifactKind::File
            && artifact.status() == ProjectInitArtifactStatus::Created
    }) && report.artifacts().iter().any(|artifact| {
        artifact.repo_relative_path()
            == ".punk/memory/goals/goal_brownfield_reconstruction_baseline.md"
            && artifact.kind() == ProjectInitArtifactKind::File
            && artifact.status() == ProjectInitArtifactStatus::Created
    }) && report.artifacts().iter().any(|artifact| {
        artifact.repo_relative_path() == ".punk/memory/reconstruction/claim-ledger.md"
            && artifact.kind() == ProjectInitArtifactKind::File
            && artifact.status() == ProjectInitArtifactStatus::Created
    }) && report.artifacts().iter().any(|artifact| {
        artifact.repo_relative_path() == INSTRUCTIONS_INDEX_PATH
            && artifact.kind() == ProjectInitArtifactKind::File
            && artifact.status() == ProjectInitArtifactStatus::Created
    });
    let cleanup_ok = fs::remove_dir_all(&temp_path).is_ok();

    let scaffold_ok = !report.blocked()
        && report.exit_code() == 0
        && report.entry_mode() == "brownfield"
        && status_exists
        && goal_exists
        && reconstruction_placeholders_ok
        && instructions_ok
        && runtime_store_absent
        && root_layout_absent
        && status_ok
        && marker_ok
        && placeholders_ok
        && instruction_index_ok
        && artifact_ok
        && cleanup_ok;

    if scaffold_ok {
        SmokeEvalCaseResult::pass(
            "eval_project_init_brownfield_scaffold_shape",
            "brownfield project init creates advisory reconstruction scaffold",
            "brownfield scaffold recorded not_started advisory reconstruction state, created empty placeholders and thin .punk/instructions source pages, did not scan existing source files, and left runtime, views, contracts, proofs, and root dogfooding dirs absent",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_project_init_brownfield_scaffold_shape",
            "brownfield project init creates advisory reconstruction scaffold",
            format!(
                "brownfield scaffold drifted; blocked={} exit_code={} status_exists={} goal_exists={} reconstruction_placeholders_ok={} instructions_ok={} runtime_store_absent={} root_layout_absent={} status_ok={} marker_ok={} placeholders_ok={} instruction_index_ok={} artifact_ok={} cleanup_ok={} status_read={:?} instruction_index_read={:?} marker_read={:?} source_manifest_read={:?} claim_ledger_read={:?} contract_readiness_read={:?}",
                report.blocked(),
                report.exit_code(),
                status_exists,
                goal_exists,
                reconstruction_placeholders_ok,
                instructions_ok,
                runtime_store_absent,
                root_layout_absent,
                status_ok,
                marker_ok,
                placeholders_ok,
                instruction_index_ok,
                artifact_ok,
                cleanup_ok,
                status_text.err(),
                instruction_index_text.err(),
                marker_text.err(),
                source_manifest_text.err(),
                claim_ledger_text.err(),
                contract_readiness_text.err()
            ),
        )
    }
}

fn eval_project_init_refuses_to_overwrite_existing_memory() -> SmokeEvalCaseResult {
    let temp_path = unique_smoke_temp_path();
    let status_path = temp_path.join(".punk/memory/STATUS.md");
    let setup_result = fs::create_dir_all(temp_path.join(".punk/memory"))
        .and_then(|_| fs::write(&status_path, b"custom status\n"));

    if let Err(error) = setup_result {
        let _ = fs::remove_dir_all(&temp_path);
        return SmokeEvalCaseResult::fail(
            "eval_project_init_refuses_to_overwrite_existing_memory",
            "project init refuses to overwrite existing project memory",
            format!("temporary conflict setup failed with {error:?}"),
        );
    }

    let project_id = ProjectId::parse("weekend-project").expect("smoke project id should parse");
    let report = init_level0_project(&temp_path, project_id);
    let status_text = fs::read_to_string(&status_path);
    let cleanup_ok = fs::remove_dir_all(&temp_path).is_ok();

    let conflict_ok = report.blocked()
        && report.exit_code() == 1
        && report.artifacts().iter().any(|artifact| {
            artifact.repo_relative_path() == ".punk/memory/STATUS.md"
                && artifact.kind() == ProjectInitArtifactKind::File
                && artifact.status() == ProjectInitArtifactStatus::Conflict
        });
    let preserved_ok = status_text
        .as_ref()
        .is_ok_and(|text| text == "custom status\n");

    if conflict_ok && preserved_ok && cleanup_ok {
        SmokeEvalCaseResult::pass(
            "eval_project_init_refuses_to_overwrite_existing_memory",
            "project init refuses to overwrite existing project memory",
            "existing project-memory files with different contents are reported as conflicts and preserved unchanged",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_project_init_refuses_to_overwrite_existing_memory",
            "project init refuses to overwrite existing project memory",
            format!(
                "init overwrite guard drifted; conflict_ok={} preserved_ok={} cleanup_ok={} blocked={} exit_code={} status_read={:?}",
                conflict_ok,
                preserved_ok,
                cleanup_ok,
                report.blocked(),
                report.exit_code(),
                status_text
            ),
        )
    }
}

fn eval_project_init_conflict_is_atomic_noop() -> SmokeEvalCaseResult {
    let temp_path = unique_smoke_temp_path();
    let status_path = temp_path.join(".punk/memory/STATUS.md");
    let setup_result = fs::create_dir_all(temp_path.join(".punk/memory"))
        .and_then(|_| fs::write(&status_path, b"custom status\n"));

    if let Err(error) = setup_result {
        let _ = fs::remove_dir_all(&temp_path);
        return SmokeEvalCaseResult::fail(
            "eval_project_init_conflict_is_atomic_noop",
            "project init conflict leaves no partial scaffold",
            format!("temporary conflict setup failed with {error:?}"),
        );
    }

    let project_id = ProjectId::parse("weekend-project").expect("smoke project id should parse");
    let report = init_level0_project(&temp_path, project_id);
    let status_text = fs::read_to_string(&status_path);
    let no_partial_ok = !temp_path
        .join(".punk/memory/goals/goal_initial_project_setup.md")
        .exists()
        && !temp_path.join(".punk/memory/reports/README.md").exists()
        && !temp_path.join(".punk/memory/adr/README.md").exists()
        && !temp_path.join(".punk/memory/knowledge").exists()
        && !temp_path.join(".punk/README.md").exists()
        && !temp_path.join(".punk/project.toml").exists();
    let cleanup_ok = fs::remove_dir_all(&temp_path).is_ok();

    let conflict_ok = report.blocked()
        && report.exit_code() == 1
        && report.artifacts().iter().any(|artifact| {
            artifact.repo_relative_path() == ".punk/memory/STATUS.md"
                && artifact.kind() == ProjectInitArtifactKind::File
                && artifact.status() == ProjectInitArtifactStatus::Conflict
        })
        && report.artifacts().iter().any(|artifact| {
            artifact.repo_relative_path() == ".punk/memory/goals/goal_initial_project_setup.md"
                && artifact.kind() == ProjectInitArtifactKind::File
                && artifact.status() == ProjectInitArtifactStatus::Planned
        });
    let preserved_ok = status_text
        .as_ref()
        .is_ok_and(|text| text == "custom status\n");

    if conflict_ok && preserved_ok && no_partial_ok && cleanup_ok {
        SmokeEvalCaseResult::pass(
            "eval_project_init_conflict_is_atomic_noop",
            "project init conflict leaves no partial scaffold",
            "init preflight reported the existing status conflict, preserved it, and created no other scaffold artifacts",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_project_init_conflict_is_atomic_noop",
            "project init conflict leaves no partial scaffold",
            format!(
                "init atomicity drifted; conflict_ok={} preserved_ok={} no_partial_ok={} cleanup_ok={} blocked={} exit_code={} status_read={:?}",
                conflict_ok,
                preserved_ok,
                no_partial_ok,
                cleanup_ok,
                report.blocked(),
                report.exit_code(),
                status_text
            ),
        )
    }
}

fn eval_source_corpus_manifest_model_is_side_effect_free() -> SmokeEvalCaseResult {
    let project_id = match ProjectId::parse("weekend-project") {
        Ok(project_id) => project_id,
        Err(error) => {
            return SmokeEvalCaseResult::fail(
                "eval_source_corpus_manifest_model_is_side_effect_free",
                "brownfield source corpus manifest model stays side-effect-free",
                format!("project id setup failed with {error:?}"),
            );
        }
    };
    let manifest_id = match SourceCorpusManifestId::parse(
        "brownfield-source-corpus-manifest.v0.1:weekend-project:smoke",
    ) {
        Ok(manifest_id) => manifest_id,
        Err(error) => {
            return SmokeEvalCaseResult::fail(
                "eval_source_corpus_manifest_model_is_side_effect_free",
                "brownfield source corpus manifest model stays side-effect-free",
                format!("manifest id setup failed with {error:?}"),
            );
        }
    };
    let item_id = match SourceCorpusItemId::parse("item:smoke") {
        Ok(item_id) => item_id,
        Err(error) => {
            return SmokeEvalCaseResult::fail(
                "eval_source_corpus_manifest_model_is_side_effect_free",
                "brownfield source corpus manifest model stays side-effect-free",
                format!("item id setup failed with {error:?}"),
            );
        }
    };
    let repo_relative_path = match SourceCorpusRepoRelativePath::parse("crates/example/src/lib.rs")
    {
        Ok(path) => path,
        Err(error) => {
            return SmokeEvalCaseResult::fail(
                "eval_source_corpus_manifest_model_is_side_effect_free",
                "brownfield source corpus manifest model stays side-effect-free",
                format!("repo-relative path setup failed with {error:?}"),
            );
        }
    };
    let item = match SourceCorpusItem::new(
        &manifest_id,
        item_id,
        repo_relative_path,
        SourceCorpusObservedKind::File,
        SourceCorpusSourceClass::SourceCode,
    ) {
        Ok(item) => item,
        Err(error) => {
            return SmokeEvalCaseResult::fail(
                "eval_source_corpus_manifest_model_is_side_effect_free",
                "brownfield source corpus manifest model stays side-effect-free",
                format!("item setup failed with {error:?}"),
            );
        }
    };
    let manifest = SourceCorpusManifest::new(manifest_id, project_id, vec![item]);
    let capabilities = manifest.capabilities();

    let authority_ok = manifest.status() == SourceCorpusManifestStatus::Advisory
        && manifest.authority() == SourceCorpusManifestAuthority::ObservedStructure
        && !manifest.has_project_truth_authority();
    let path_ok = manifest.items().first().is_some_and(|item| {
        item.repo_relative_path().as_str() == "crates/example/src/lib.rs"
            && item.evidence_ref().as_str()
                == "brownfield-source-corpus-manifest.v0.1:weekend-project:smoke#item:smoke"
            && !item.content_policy().reads_contents()
            && !item.content_policy().stores_snippets()
            && !item.content_policy().summarizes_contents()
            && !item.hash_policy().requires_filesystem_hashing()
            && !item.size_policy().requires_filesystem_metadata()
            && !item.has_claim_authority()
    }) && SourceCorpusRepoRelativePath::parse("/Users/vi/project/src/lib.rs")
        .is_err()
        && SourceCorpusRepoRelativePath::parse("~/project/src/lib.rs").is_err()
        && SourceCorpusRepoRelativePath::parse("../project/src/lib.rs").is_err();
    let boundary_ok = !source_corpus_manifest_claim_field_allowed("claims_created")
        && SOURCE_CORPUS_DEFAULT_EXCLUDED_PATHS.contains(&".git")
        && SOURCE_CORPUS_DEFAULT_EXCLUDED_PATHS.contains(&".punk/runtime")
        && SOURCE_CORPUS_DEFAULT_EXCLUDED_PATHS.contains(&"node_modules")
        && SourceCorpusSourceClass::ALL
            .iter()
            .any(|class| class.as_str() == "unknown");
    let side_effect_free_ok = !capabilities.scans_repository()
        && !capabilities.walks_files()
        && !capabilities.reads_file_contents()
        && !capabilities.computes_file_hashes()
        && !capabilities.writes_manifest()
        && !capabilities.creates_claims()
        && !capabilities.infers_intent()
        && !capabilities.uses_network()
        && !capabilities.uses_remote_ai();

    if authority_ok && path_ok && boundary_ok && side_effect_free_ok {
        SmokeEvalCaseResult::pass(
            "eval_source_corpus_manifest_model_is_side_effect_free",
            "brownfield source corpus manifest model stays side-effect-free",
            "source corpus manifest model kept advisory observed-structure authority, repo-relative path policy, no-content defaults, deferred hash/size policy, no claim fields, and no scan/read/write/network/AI capabilities",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_source_corpus_manifest_model_is_side_effect_free",
            "brownfield source corpus manifest model stays side-effect-free",
            format!(
                "source corpus manifest model drifted; authority_ok={} path_ok={} boundary_ok={} side_effect_free_ok={}",
                authority_ok, path_ok, boundary_ok, side_effect_free_ok
            ),
        )
    }
}

fn eval_source_corpus_manifest_writer_preflight_model_is_side_effect_free() -> SmokeEvalCaseResult {
    let project_id = match ProjectId::parse("weekend-project") {
        Ok(project_id) => project_id,
        Err(error) => {
            return SmokeEvalCaseResult::fail(
                "eval_source_corpus_manifest_writer_preflight_model_is_side_effect_free",
                "brownfield source corpus manifest writer preflight model stays side-effect-free",
                format!("project id setup failed with {error:?}"),
            );
        }
    };
    let manifest_id = match SourceCorpusManifestId::parse(
        "brownfield-source-corpus-manifest.v0.1:weekend-project:preflight-smoke",
    ) {
        Ok(manifest_id) => manifest_id,
        Err(error) => {
            return SmokeEvalCaseResult::fail(
                "eval_source_corpus_manifest_writer_preflight_model_is_side_effect_free",
                "brownfield source corpus manifest writer preflight model stays side-effect-free",
                format!("manifest id setup failed with {error:?}"),
            );
        }
    };
    let item_id = match SourceCorpusItemId::parse("item:preflight-smoke") {
        Ok(item_id) => item_id,
        Err(error) => {
            return SmokeEvalCaseResult::fail(
                "eval_source_corpus_manifest_writer_preflight_model_is_side_effect_free",
                "brownfield source corpus manifest writer preflight model stays side-effect-free",
                format!("item id setup failed with {error:?}"),
            );
        }
    };
    let repo_relative_path = match SourceCorpusRepoRelativePath::parse("crates/example/src/lib.rs")
    {
        Ok(path) => path,
        Err(error) => {
            return SmokeEvalCaseResult::fail(
                "eval_source_corpus_manifest_writer_preflight_model_is_side_effect_free",
                "brownfield source corpus manifest writer preflight model stays side-effect-free",
                format!("repo-relative path setup failed with {error:?}"),
            );
        }
    };
    let item = match SourceCorpusItem::new(
        &manifest_id,
        item_id,
        repo_relative_path,
        SourceCorpusObservedKind::File,
        SourceCorpusSourceClass::SourceCode,
    ) {
        Ok(item) => item,
        Err(error) => {
            return SmokeEvalCaseResult::fail(
                "eval_source_corpus_manifest_writer_preflight_model_is_side_effect_free",
                "brownfield source corpus manifest writer preflight model stays side-effect-free",
                format!("item setup failed with {error:?}"),
            );
        }
    };
    let manifest = SourceCorpusManifest::new(manifest_id, project_id, vec![item]);
    let allowed = SourceCorpusManifestWriterPreflight::evaluate(
        SourceCorpusManifestWriterPreflightInput::new(
            SourceCorpusManifestWriterTarget::new(
                SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH,
            ),
            manifest.clone(),
        ),
    );
    let absolute = SourceCorpusManifestWriterPreflight::evaluate(
        SourceCorpusManifestWriterPreflightInput::new(
            SourceCorpusManifestWriterTarget::new(
                "/example/project/.punk/memory/reconstruction/source-corpus-manifest.md",
            ),
            manifest.clone(),
        ),
    );
    let escape = SourceCorpusManifestWriterPreflight::evaluate(
        SourceCorpusManifestWriterPreflightInput::new(
            SourceCorpusManifestWriterTarget::new(
                ".punk/memory/reconstruction/../source-corpus-manifest.md",
            ),
            manifest.clone(),
        ),
    );
    let runtime = SourceCorpusManifestWriterPreflight::evaluate(
        SourceCorpusManifestWriterPreflightInput::new(
            SourceCorpusManifestWriterTarget::new(".punk/runtime/source-corpus-manifest.md"),
            manifest.clone(),
        ),
    );
    let symlink = SourceCorpusManifestWriterPreflight::evaluate(
        SourceCorpusManifestWriterPreflightInput::new(
            SourceCorpusManifestWriterTarget::new(
                SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH,
            ),
            manifest.clone(),
        )
        .with_symlink_ancestor_status(SourceCorpusManifestWriterSymlinkAncestorStatus::Escape),
    );
    let conflict = SourceCorpusManifestWriterPreflight::evaluate(
        SourceCorpusManifestWriterPreflightInput::new(
            SourceCorpusManifestWriterTarget::new(
                SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH,
            ),
            manifest.clone(),
        )
        .with_conflict_policy(SourceCorpusManifestWriterConflictPolicy::DifferentExistingTarget),
    );
    let claims_created = SourceCorpusManifestWriterPreflight::evaluate(
        SourceCorpusManifestWriterPreflightInput::new(
            SourceCorpusManifestWriterTarget::new(
                SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH,
            ),
            manifest.clone(),
        )
        .with_manifest_inspection(
            SourceCorpusManifestWriterManifestInspection::for_manifest(&manifest)
                .with_claim_field("claims_created"),
        ),
    );

    let allowed_ok = allowed.allowed_to_write()
        && allowed.has_finding(SourceCorpusManifestWriterPreflightFinding::TargetAllowed)
        && allowed.has_finding(SourceCorpusManifestWriterPreflightFinding::TargetConflictMissing)
        && allowed
            .has_finding(SourceCorpusManifestWriterPreflightFinding::OperationEvidenceIsNotProof);
    let blockers_ok = absolute.blocked()
        && absolute.has_finding(SourceCorpusManifestWriterPreflightFinding::TargetAbsolutePath)
        && escape.blocked()
        && escape.has_finding(SourceCorpusManifestWriterPreflightFinding::TargetPathEscape)
        && runtime.blocked()
        && runtime
            .has_finding(SourceCorpusManifestWriterPreflightFinding::RuntimeStorageTargetForbidden)
        && symlink.blocked()
        && symlink
            .has_finding(SourceCorpusManifestWriterPreflightFinding::TargetSymlinkAncestorEscape)
        && conflict.blocked()
        && conflict
            .has_finding(SourceCorpusManifestWriterPreflightFinding::TargetConflictDifferentBlocks)
        && claims_created.blocked()
        && claims_created
            .has_finding(SourceCorpusManifestWriterPreflightFinding::ManifestContainsClaimField)
        && claims_created
            .has_finding(SourceCorpusManifestWriterPreflightFinding::ManifestContainsClaimsCreated);
    let capabilities = allowed.capabilities();
    let side_effect_free_ok = !capabilities.scans_repository()
        && !capabilities.walks_files()
        && !capabilities.reads_file_contents()
        && !capabilities.computes_file_hashes()
        && !capabilities.writes_manifest()
        && !capabilities.generates_manifest()
        && !capabilities.creates_claims()
        && !capabilities.infers_intent()
        && !capabilities.uses_network()
        && !capabilities.uses_remote_ai()
        && !capabilities.writes_runtime_storage()
        && !allowed.operation_evidence_is_proof()
        && !allowed.operation_evidence_is_gate_decision()
        && !allowed.operation_evidence_is_acceptance()
        && !allowed.operation_evidence_is_project_truth();

    if allowed_ok && blockers_ok && side_effect_free_ok {
        SmokeEvalCaseResult::pass(
            "eval_source_corpus_manifest_writer_preflight_model_is_side_effect_free",
            "brownfield source corpus manifest writer preflight model stays side-effect-free",
            "source corpus manifest writer preflight model kept target, path escape, symlink, conflict, runtime, and claim blockers explicit without scanning, reading, hashing, writing, generating manifests, creating claims, or treating operation evidence as proof",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_source_corpus_manifest_writer_preflight_model_is_side_effect_free",
            "brownfield source corpus manifest writer preflight model stays side-effect-free",
            format!(
                "source corpus manifest writer preflight model drifted; allowed_ok={} blockers_ok={} side_effect_free_ok={}",
                allowed_ok, blockers_ok, side_effect_free_ok
            ),
        )
    }
}

fn eval_source_corpus_manifest_writer_first_slice_writes_prepared_manifest() -> SmokeEvalCaseResult
{
    let project_id = match ProjectId::parse("weekend-project") {
        Ok(project_id) => project_id,
        Err(error) => {
            return SmokeEvalCaseResult::fail(
                "eval_source_corpus_manifest_writer_first_slice_writes_prepared_manifest",
                "brownfield source corpus manifest writer first slice writes prepared manifest",
                format!("project id setup failed with {error:?}"),
            );
        }
    };
    let manifest_id = match SourceCorpusManifestId::parse(
        "brownfield-source-corpus-manifest.v0.1:weekend-project:first-slice-smoke",
    ) {
        Ok(manifest_id) => manifest_id,
        Err(error) => {
            return SmokeEvalCaseResult::fail(
                "eval_source_corpus_manifest_writer_first_slice_writes_prepared_manifest",
                "brownfield source corpus manifest writer first slice writes prepared manifest",
                format!("manifest id setup failed with {error:?}"),
            );
        }
    };
    let item_id = match SourceCorpusItemId::parse("item:first-slice-smoke") {
        Ok(item_id) => item_id,
        Err(error) => {
            return SmokeEvalCaseResult::fail(
                "eval_source_corpus_manifest_writer_first_slice_writes_prepared_manifest",
                "brownfield source corpus manifest writer first slice writes prepared manifest",
                format!("item id setup failed with {error:?}"),
            );
        }
    };
    let repo_relative_path = match SourceCorpusRepoRelativePath::parse("crates/example/src/lib.rs")
    {
        Ok(path) => path,
        Err(error) => {
            return SmokeEvalCaseResult::fail(
                "eval_source_corpus_manifest_writer_first_slice_writes_prepared_manifest",
                "brownfield source corpus manifest writer first slice writes prepared manifest",
                format!("repo-relative path setup failed with {error:?}"),
            );
        }
    };
    let item = match SourceCorpusItem::new(
        &manifest_id,
        item_id,
        repo_relative_path,
        SourceCorpusObservedKind::File,
        SourceCorpusSourceClass::SourceCode,
    ) {
        Ok(item) => item,
        Err(error) => {
            return SmokeEvalCaseResult::fail(
                "eval_source_corpus_manifest_writer_first_slice_writes_prepared_manifest",
                "brownfield source corpus manifest writer first slice writes prepared manifest",
                format!("item setup failed with {error:?}"),
            );
        }
    };
    let manifest = SourceCorpusManifest::new(manifest_id, project_id, vec![item]);
    let target =
        SourceCorpusManifestWriterTarget::new(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH);
    let preflight = SourceCorpusManifestWriterPreflight::evaluate(
        SourceCorpusManifestWriterPreflightInput::new(target.clone(), manifest.clone())
            .with_conflict_policy(SourceCorpusManifestWriterConflictPolicy::MissingTarget),
    );
    let storage_root = unique_smoke_temp_path();
    let parent_path = storage_root.join(".punk/memory/reconstruction");
    if let Err(error) = fs::create_dir_all(&parent_path) {
        return SmokeEvalCaseResult::fail(
            "eval_source_corpus_manifest_writer_first_slice_writes_prepared_manifest",
            "brownfield source corpus manifest writer first slice writes prepared manifest",
            format!("test setup could not create explicit parent directory: {error:?}"),
        );
    }

    let result = source_corpus_manifest_writer_write_first_slice(
        &manifest,
        &storage_root,
        target,
        &preflight,
    );
    let target_path = storage_root.join(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH);
    let written = fs::read(&target_path);
    let canonical = source_corpus_manifest_render_canonical_bytes(&manifest);
    let rendered = String::from_utf8(canonical.clone()).unwrap_or_default();
    let idempotent_preflight = SourceCorpusManifestWriterPreflight::evaluate(
        SourceCorpusManifestWriterPreflightInput::new(
            SourceCorpusManifestWriterTarget::new(
                SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH,
            ),
            manifest.clone(),
        )
        .with_conflict_policy(SourceCorpusManifestWriterConflictPolicy::IdenticalExistingTarget),
    );
    let idempotent = source_corpus_manifest_writer_write_first_slice(
        &manifest,
        &storage_root,
        SourceCorpusManifestWriterTarget::new(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH),
        &idempotent_preflight,
    );
    let stale_setup_ok = fs::write(&target_path, b"changed after preflight").is_ok();
    let stale_identical = source_corpus_manifest_writer_write_first_slice(
        &manifest,
        &storage_root,
        SourceCorpusManifestWriterTarget::new(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH),
        &idempotent_preflight,
    );
    let conflict_preflight = SourceCorpusManifestWriterPreflight::evaluate(
        SourceCorpusManifestWriterPreflightInput::new(
            SourceCorpusManifestWriterTarget::new(
                SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH,
            ),
            manifest.clone(),
        )
        .with_conflict_policy(SourceCorpusManifestWriterConflictPolicy::DifferentExistingTarget),
    );
    let conflict = source_corpus_manifest_writer_write_first_slice(
        &manifest,
        &storage_root,
        SourceCorpusManifestWriterTarget::new(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH),
        &conflict_preflight,
    );
    let runtime_exists = storage_root.join(".punk/runtime").exists();
    let claim_ledger_exists = storage_root
        .join(".punk/memory/reconstruction/claim-ledger.md")
        .exists();
    let cleanup_ok = fs::remove_dir_all(&storage_root).is_ok();

    let write_ok = preflight.allowed_to_write()
        && result.evidence_status() == SourceCorpusManifestWriterOperationEvidenceStatus::Written
        && result.wrote_manifest()
        && result.write_attempted()
        && result.blockers().is_empty()
        && matches!(&written, Ok(bytes) if *bytes == canonical)
        && result.canonical_byte_len() == canonical.len();
    let render_ok = rendered.contains("manifest_status: advisory\n")
        && rendered.contains("authority: observed_structure\n")
        && rendered.contains("generated_at_policy: no_runtime_clock\n")
        && rendered.contains("read_contents: false\n")
        && rendered.contains("hash_policy:\n")
        && !rendered.contains("/Users/")
        && !rendered.contains("/home/")
        && !rendered.contains("claims_created")
        && !rendered.contains("created_at:")
        && !rendered.contains("generated_at:");
    let blocker_ok = idempotent.is_idempotent()
        && !idempotent.write_attempted()
        && stale_setup_ok
        && stale_identical.has_conflict()
        && stale_identical
            .has_blocker(SourceCorpusManifestWriterFirstSliceBlocker::ExistingTargetDifferent)
        && !stale_identical.write_attempted()
        && conflict.has_conflict()
        && conflict
            .has_blocker(SourceCorpusManifestWriterFirstSliceBlocker::ExistingTargetDifferent)
        && !conflict.write_attempted();
    let capabilities = result.capabilities();
    let boundary_ok = capabilities.takes_manifest_model_input()
        && capabilities.renders_canonical_bytes()
        && capabilities.requires_preflight_pass()
        && capabilities.writes_one_safe_target()
        && capabilities.emits_operation_evidence()
        && !capabilities.scans_repository()
        && !capabilities.walks_files()
        && !capabilities.reads_source_file_contents()
        && !capabilities.computes_source_file_hashes()
        && !capabilities.generates_manifest_from_repo()
        && !capabilities.creates_claims()
        && !capabilities.promotes_manifest_authority()
        && !capabilities.writes_runtime_storage()
        && !capabilities.activates_punk_writer()
        && !capabilities.writes_gate_or_proof_artifacts()
        && !capabilities.uses_network()
        && !capabilities.uses_remote_ai()
        && !result.operation_evidence_is_proof()
        && !result.operation_evidence_is_gate_decision()
        && !result.operation_evidence_is_acceptance()
        && !result.operation_evidence_is_project_truth()
        && !result.creates_claims()
        && !result.promotes_manifest_authority();
    let no_forbidden_side_effects = !runtime_exists && !claim_ledger_exists && cleanup_ok;

    if write_ok && render_ok && blocker_ok && boundary_ok && no_forbidden_side_effects {
        SmokeEvalCaseResult::pass(
            "eval_source_corpus_manifest_writer_first_slice_writes_prepared_manifest",
            "brownfield source corpus manifest writer first slice writes prepared manifest",
            "source corpus manifest writer first slice wrote deterministic canonical bytes from an already-constructed model to one explicit safe target after preflight, kept idempotent/conflict evidence in memory, and left scanning, source content reads, hashing, claims, runtime storage, and authority promotion inactive",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_source_corpus_manifest_writer_first_slice_writes_prepared_manifest",
            "brownfield source corpus manifest writer first slice writes prepared manifest",
            format!(
                "source corpus manifest writer first slice drifted; write_ok={} render_ok={} blocker_ok={} boundary_ok={} no_forbidden_side_effects={} status={:?} blockers={:?} idempotent={:?} stale_identical={:?} conflict={:?} written={:?} cleanup_ok={}",
                write_ok,
                render_ok,
                blocker_ok,
                boundary_ok,
                no_forbidden_side_effects,
                result.evidence_status(),
                result.blockers(),
                idempotent.evidence_status(),
                stale_identical.evidence_status(),
                conflict.evidence_status(),
                written.as_ref().map(|bytes| bytes.len()),
                cleanup_ok
            ),
        )
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

fn eval_contract_schema_blueprint_has_required_top_level_sections() -> SmokeEvalCaseResult {
    let blueprint = contract_schema_blueprint_v0_1();

    let sections_ok = blueprint.has_required_sections()
        && [
            ContractSchemaSection::Identity,
            ContractSchemaSection::Authority,
            ContractSchemaSection::Lifecycle,
            ContractSchemaSection::Work,
            ContractSchemaSection::Boundaries,
            ContractSchemaSection::Clauses,
            ContractSchemaSection::Validation,
            ContractSchemaSection::Evidence,
            ContractSchemaSection::ReceiptRequirements,
            ContractSchemaSection::GatePolicy,
            ContractSchemaSection::ProofRequirements,
            ContractSchemaSection::ChangeControl,
            ContractSchemaSection::MemoryLinks,
        ]
        .iter()
        .all(|section| blueprint.sections().contains(section));
    let fields_ok = blueprint
        .sections()
        .iter()
        .all(|section| blueprint.fields_for_section(*section).next().is_some());

    if sections_ok && fields_ok {
        SmokeEvalCaseResult::pass(
            "eval_contract_schema_blueprint_has_required_top_level_sections",
            "contract schema blueprint preserves required sections",
            "blueprint exposes identity, authority, lifecycle, work, boundaries, clauses, validation, evidence, receipt, gate policy, proof requirements, change control, and memory links in memory only",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_contract_schema_blueprint_has_required_top_level_sections",
            "contract schema blueprint preserves required sections",
            format!(
                "contract schema blueprint section coverage drifted; sections_ok={sections_ok} fields_ok={fields_ok}"
            ),
        )
    }
}

fn eval_contract_schema_blueprint_marks_status_required_now_deferred_parked_future(
) -> SmokeEvalCaseResult {
    let blueprint = contract_schema_blueprint_v0_1();
    let status_ok = [
        ContractSchemaFieldStatus::RequiredNow,
        ContractSchemaFieldStatus::Deferred,
        ContractSchemaFieldStatus::Parked,
        ContractSchemaFieldStatus::Future,
    ]
    .iter()
    .all(|status| {
        blueprint
            .fields()
            .iter()
            .any(|field| field.status == *status)
    });
    let representative_fields_ok = blueprint.has_field(ContractSchemaSection::Lifecycle, "status")
        && blueprint.has_field(
            ContractSchemaSection::GatePolicy,
            "partial_acceptance_policy",
        )
        && blueprint.has_field(ContractSchemaSection::Authority, "pki_signature")
        && blueprint.has_field(ContractSchemaSection::Work, "semantic_intent_recovery");

    if status_ok && representative_fields_ok {
        SmokeEvalCaseResult::pass(
            "eval_contract_schema_blueprint_marks_status_required_now_deferred_parked_future",
            "contract schema blueprint preserves field status split",
            "blueprint marks active-core, deferred, parked, and future fields explicitly without promoting parked capabilities",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_contract_schema_blueprint_marks_status_required_now_deferred_parked_future",
            "contract schema blueprint preserves field status split",
            format!(
                "contract schema blueprint status split drifted; status_ok={status_ok} representative_fields_ok={representative_fields_ok}"
            ),
        )
    }
}

fn eval_contract_status_does_not_include_acceptance_decisions() -> SmokeEvalCaseResult {
    let status_names = CONTRACT_STATUS_VALUES
        .iter()
        .map(|status| status.as_str())
        .collect::<Vec<_>>();
    let lifecycle_ok = status_names == vec!["draft", "approved_for_run", "superseded", "cancelled"];
    let no_acceptance_surface = !status_names.contains(&"accepted")
        && !status_names.contains(&"rejected")
        && !status_names.contains(&"partially_accepted")
        && CONTRACT_STATUS_VALUES
            .iter()
            .all(|status| !status.is_gate_acceptance_decision());
    let readiness_ok = ContractStatus::ApprovedForRun.ready_for_bounded_work()
        && !ContractStatus::Draft.ready_for_bounded_work()
        && !ContractStatus::Superseded.ready_for_bounded_work()
        && !ContractStatus::Cancelled.ready_for_bounded_work();

    if lifecycle_ok && no_acceptance_surface && readiness_ok {
        SmokeEvalCaseResult::pass(
            "eval_contract_status_does_not_include_acceptance_decisions",
            "contract status remains lifecycle-only",
            "contract lifecycle statuses cover draft, approved_for_run, superseded, and cancelled while closure authority stays outside ContractStatus",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_contract_status_does_not_include_acceptance_decisions",
            "contract status remains lifecycle-only",
            format!(
                "contract status drifted; lifecycle_ok={lifecycle_ok} no_acceptance_surface={no_acceptance_surface} readiness_ok={readiness_ok} names={status_names:?}"
            ),
        )
    }
}

fn hard_contract_clause(id: &'static str) -> ContractClauseBlueprint {
    ContractClauseBlueprint {
        id,
        kind: ContractClauseKind::Invariant,
        mode: ContractClauseMode::Hybrid,
        severity: ContractClauseSeverity::Hard,
        has_validator_refs: false,
        has_required_receipt_fields: false,
        required_receipt_fields: &[],
        has_proof_requirement_refs: false,
        gate_review_required: false,
        human_gate_review_reason: None,
        unsupported_clause_finding: false,
    }
}

fn hard_contract_clause_with_receipt_fields(
    id: &'static str,
    fields: &'static [&'static str],
) -> ContractClauseBlueprint {
    ContractClauseBlueprint {
        id,
        has_required_receipt_fields: true,
        required_receipt_fields: fields,
        ..hard_contract_clause("clause.base")
    }
}

fn receipt_requirement_for_hard_clause(
    field: &'static str,
    clause_id: &'static str,
) -> ContractReceiptRequirement {
    ContractReceiptRequirement::new(field, ReceiptRequirementSource::HardClause(clause_id))
}

fn eval_hard_clause_requires_validator_receipt_proof_or_gate_review_mapping() -> SmokeEvalCaseResult
{
    let unmapped = hard_contract_clause("clause.unmapped");
    let validator_mapped = ContractClauseBlueprint {
        id: "clause.validator",
        has_validator_refs: true,
        ..unmapped
    };
    let receipt_mapped = ContractClauseBlueprint {
        id: "clause.receipt",
        has_required_receipt_fields: true,
        ..unmapped
    };
    let proof_mapped = ContractClauseBlueprint {
        id: "clause.proof",
        has_proof_requirement_refs: true,
        ..unmapped
    };
    let gate_mapped = ContractClauseBlueprint {
        id: "clause.gate",
        gate_review_required: true,
        human_gate_review_reason: Some("manual review required for hybrid clause"),
        ..unmapped
    };
    let unsupported_mapped = ContractClauseBlueprint {
        id: "clause.unsupported",
        unsupported_clause_finding: true,
        ..unmapped
    };
    let advisory_unmapped = ContractClauseBlueprint {
        id: "clause.advisory",
        severity: ContractClauseSeverity::Advisory,
        ..unmapped
    };
    let mapping_ok = !unmapped.hard_clause_mapping_satisfied()
        && validator_mapped.hard_clause_mapping_satisfied()
        && receipt_mapped.hard_clause_mapping_satisfied()
        && proof_mapped.hard_clause_mapping_satisfied()
        && gate_mapped.hard_clause_mapping_satisfied()
        && !unsupported_mapped.hard_clause_mapping_satisfied()
        && advisory_unmapped.hard_clause_mapping_satisfied();

    if mapping_ok {
        SmokeEvalCaseResult::pass(
            "eval_hard_clause_requires_validator_receipt_proof_or_gate_review_mapping",
            "hard clauses require an explicit mapping",
            "hard clauses need validator refs, receipt fields, proof requirement refs, or explicit human review; unsupported findings remain blocking",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_hard_clause_requires_validator_receipt_proof_or_gate_review_mapping",
            "hard clauses require an explicit mapping",
            format!("hard clause mapping drifted; mapping_ok={mapping_ok}"),
        )
    }
}

fn eval_hard_clause_with_validator_ref_is_mapped() -> SmokeEvalCaseResult {
    let clause = ContractClauseBlueprint {
        id: "clause.validator",
        has_validator_refs: true,
        ..hard_contract_clause("clause.base")
    };
    let assessment = assess_hard_clause_mappings(&[clause]);
    let mapped = assessment.ready_for_approval()
        && assessment.has_status(HardClauseMappingStatus::Mapped)
        && assessment.findings()[0]
            .targets()
            .contains(&HardClauseMappingTarget::ValidatorRef);

    if mapped {
        SmokeEvalCaseResult::pass(
            "eval_hard_clause_with_validator_ref_is_mapped",
            "validator ref maps hard clause",
            "validator refs count as declarative hard-clause mapping without running validators",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_hard_clause_with_validator_ref_is_mapped",
            "validator ref maps hard clause",
            format!(
                "validator mapping drifted; findings={:?}",
                assessment.findings()
            ),
        )
    }
}

fn eval_hard_clause_with_required_receipt_field_is_mapped() -> SmokeEvalCaseResult {
    let clause = ContractClauseBlueprint {
        id: "clause.receipt",
        has_required_receipt_fields: true,
        ..hard_contract_clause("clause.base")
    };
    let assessment = assess_hard_clause_mappings(&[clause]);
    let mapped = assessment.ready_for_approval()
        && assessment.has_status(HardClauseMappingStatus::Mapped)
        && assessment.findings()[0]
            .targets()
            .contains(&HardClauseMappingTarget::RequiredReceiptField);

    if mapped {
        SmokeEvalCaseResult::pass(
            "eval_hard_clause_with_required_receipt_field_is_mapped",
            "receipt field maps hard clause",
            "required receipt fields count as declarative hard-clause mapping",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_hard_clause_with_required_receipt_field_is_mapped",
            "receipt field maps hard clause",
            format!(
                "receipt mapping drifted; findings={:?}",
                assessment.findings()
            ),
        )
    }
}

fn eval_hard_clause_with_proof_requirement_ref_is_mapped() -> SmokeEvalCaseResult {
    let clause = ContractClauseBlueprint {
        id: "clause.proof",
        has_proof_requirement_refs: true,
        ..hard_contract_clause("clause.base")
    };
    let assessment = assess_hard_clause_mappings(&[clause]);
    let mapped = assessment.ready_for_approval()
        && assessment.has_status(HardClauseMappingStatus::Mapped)
        && assessment.findings()[0]
            .targets()
            .contains(&HardClauseMappingTarget::ProofRequirementRef);

    if mapped {
        SmokeEvalCaseResult::pass(
            "eval_hard_clause_with_proof_requirement_ref_is_mapped",
            "proof requirement maps hard clause",
            "proof requirement refs count as declarative hard-clause mapping without creating proofpacks",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_hard_clause_with_proof_requirement_ref_is_mapped",
            "proof requirement maps hard clause",
            format!(
                "proof mapping drifted; findings={:?}",
                assessment.findings()
            ),
        )
    }
}

fn eval_hard_clause_with_explicit_human_gate_review_and_reason_is_mapped() -> SmokeEvalCaseResult {
    let clause = ContractClauseBlueprint {
        id: "clause.human",
        gate_review_required: true,
        human_gate_review_reason: Some("requires manual policy check"),
        ..hard_contract_clause("clause.base")
    };
    let assessment = assess_hard_clause_mappings(&[clause]);
    let mapped = assessment.ready_for_approval()
        && assessment.has_status(HardClauseMappingStatus::RequiresHumanGateReview)
        && assessment.findings()[0]
            .targets()
            .contains(&HardClauseMappingTarget::HumanGateReview)
        && assessment.findings()[0].reason().is_some();

    if mapped {
        SmokeEvalCaseResult::pass(
            "eval_hard_clause_with_explicit_human_gate_review_and_reason_is_mapped",
            "explicit human review maps hard clause",
            "human review counts only when a non-empty reason is present and remains a future gate input",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_hard_clause_with_explicit_human_gate_review_and_reason_is_mapped",
            "explicit human review maps hard clause",
            format!(
                "human review mapping drifted; findings={:?}",
                assessment.findings()
            ),
        )
    }
}

fn eval_hard_clause_with_blank_human_review_reason_is_blocking() -> SmokeEvalCaseResult {
    let clause = ContractClauseBlueprint {
        id: "clause.blank-human",
        gate_review_required: true,
        human_gate_review_reason: Some(" "),
        ..hard_contract_clause("clause.base")
    };
    let assessment = assess_hard_clause_mappings(&[clause]);
    let blocking = assessment.approval_blocking()
        && assessment.has_status(HardClauseMappingStatus::Invalid)
        && assessment.findings()[0].reason() == Some("human_gate_review_requires_reason");

    if blocking {
        SmokeEvalCaseResult::pass(
            "eval_hard_clause_with_blank_human_review_reason_is_blocking",
            "blank human review reason blocks",
            "human review without a reason cannot be used as an escape hatch",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_hard_clause_with_blank_human_review_reason_is_blocking",
            "blank human review reason blocks",
            format!(
                "blank human review drifted; findings={:?}",
                assessment.findings()
            ),
        )
    }
}

fn eval_hard_clause_with_no_mapping_is_unsupported_blocking() -> SmokeEvalCaseResult {
    let assessment = assess_hard_clause_mappings(&[hard_contract_clause("clause.unmapped")]);
    let blocking = assessment.approval_blocking()
        && assessment.has_status(HardClauseMappingStatus::UnsupportedBlocking)
        && assessment.findings()[0].targets().is_empty();

    if blocking {
        SmokeEvalCaseResult::pass(
            "eval_hard_clause_with_no_mapping_is_unsupported_blocking",
            "unmapped hard clause blocks",
            "hard clauses without validator, receipt, proof, or explicit human review paths surface as blocking unsupported findings",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_hard_clause_with_no_mapping_is_unsupported_blocking",
            "unmapped hard clause blocks",
            format!(
                "unmapped hard clause drifted; findings={:?}",
                assessment.findings()
            ),
        )
    }
}

fn eval_unsupported_hard_clause_blocks_approved_for_run() -> SmokeEvalCaseResult {
    let clause = ContractClauseBlueprint {
        id: "clause.unsupported",
        unsupported_clause_finding: true,
        ..hard_contract_clause("clause.base")
    };
    let model = ready_user_intent_contract_draft_model().with_contract_clause(clause);
    let result = confirm_contract_draft(&model, &explicit_draft_confirmation());
    let blocked = result.outcome() == ContractDraftConfirmationOutcome::Blocked
        && result.has_blocker(ContractDraftConfirmationBlocker::UnsupportedHardClause)
        && !result.approved_for_run()
        && result
            .hard_clause_mapping_assessment()
            .is_some_and(|assessment| {
                assessment.has_status(HardClauseMappingStatus::UnsupportedBlocking)
            });

    if blocked {
        SmokeEvalCaseResult::pass(
            "eval_unsupported_hard_clause_blocks_approved_for_run",
            "unsupported hard clause blocks approved_for_run",
            "unsupported hard clauses are surfaced and stop approved_for_run model state",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_unsupported_hard_clause_blocks_approved_for_run",
            "unsupported hard clause blocks approved_for_run",
            format!(
                "unsupported approval drifted; outcome={} blockers={}",
                result.outcome().as_str(),
                format_confirmation_blockers(result.blockers())
            ),
        )
    }
}

fn eval_soft_clause_without_mapping_does_not_block_approval() -> SmokeEvalCaseResult {
    let clause = ContractClauseBlueprint {
        id: "clause.soft",
        severity: ContractClauseSeverity::Soft,
        ..hard_contract_clause("clause.base")
    };
    let model = ready_user_intent_contract_draft_model().with_contract_clause(clause);
    let result = confirm_contract_draft(&model, &explicit_draft_confirmation());
    let allowed = result.approved_for_run()
        && result.approved().is_some_and(|approved| {
            approved
                .hard_clause_mapping_assessment()
                .has_status(HardClauseMappingStatus::NotRequiredForSoftOrAdvisory)
        });

    if allowed {
        SmokeEvalCaseResult::pass(
            "eval_soft_clause_without_mapping_does_not_block_approval",
            "soft clause without mapping does not block",
            "soft clauses may remain unmapped without weakening hard-clause checks",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_soft_clause_without_mapping_does_not_block_approval",
            "soft clause without mapping does not block",
            format!(
                "soft clause drifted; outcome={} blockers={}",
                result.outcome().as_str(),
                format_confirmation_blockers(result.blockers())
            ),
        )
    }
}

fn eval_advisory_clause_without_mapping_does_not_block_approval() -> SmokeEvalCaseResult {
    let clause = ContractClauseBlueprint {
        id: "clause.advisory",
        severity: ContractClauseSeverity::Advisory,
        ..hard_contract_clause("clause.base")
    };
    let model = ready_user_intent_contract_draft_model().with_contract_clause(clause);
    let result = confirm_contract_draft(&model, &explicit_draft_confirmation());
    let allowed = result.approved_for_run()
        && result.approved().is_some_and(|approved| {
            approved
                .hard_clause_mapping_assessment()
                .has_status(HardClauseMappingStatus::NotRequiredForSoftOrAdvisory)
        });

    if allowed {
        SmokeEvalCaseResult::pass(
            "eval_advisory_clause_without_mapping_does_not_block_approval",
            "advisory clause without mapping does not block",
            "advisory clauses may remain unmapped while hard clauses still need explicit paths",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_advisory_clause_without_mapping_does_not_block_approval",
            "advisory clause without mapping does not block",
            format!(
                "advisory clause drifted; outcome={} blockers={}",
                result.outcome().as_str(),
                format_confirmation_blockers(result.blockers())
            ),
        )
    }
}

fn eval_hard_rationale_clause_is_invalid_or_blocking() -> SmokeEvalCaseResult {
    let clause = ContractClauseBlueprint {
        id: "clause.rationale",
        kind: ContractClauseKind::Rationale,
        ..hard_contract_clause("clause.base")
    };
    let assessment = assess_hard_clause_mappings(&[clause]);
    let blocking = assessment.approval_blocking()
        && assessment.has_status(HardClauseMappingStatus::Invalid)
        && assessment.findings()[0].reason()
            == Some("hard_rationale_clause_is_not_an_enforcement_clause");

    if blocking {
        SmokeEvalCaseResult::pass(
            "eval_hard_rationale_clause_is_invalid_or_blocking",
            "hard rationale clause blocks",
            "rationale clauses are context and cannot act as hard enforcement clauses",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_hard_rationale_clause_is_invalid_or_blocking",
            "hard rationale clause blocks",
            format!(
                "hard rationale drifted; findings={:?}",
                assessment.findings()
            ),
        )
    }
}

fn eval_ready_confirmed_draft_with_all_hard_clauses_mapped_can_approve() -> SmokeEvalCaseResult {
    let validator = ContractClauseBlueprint {
        id: "clause.validator",
        has_validator_refs: true,
        ..hard_contract_clause("clause.base")
    };
    let receipt = ContractClauseBlueprint {
        id: "clause.receipt",
        has_required_receipt_fields: true,
        required_receipt_fields: &["artifact_hashes"],
        ..hard_contract_clause("clause.base")
    };
    let proof = ContractClauseBlueprint {
        id: "clause.proof",
        has_proof_requirement_refs: true,
        ..hard_contract_clause("clause.base")
    };
    let human = ContractClauseBlueprint {
        id: "clause.human",
        gate_review_required: true,
        human_gate_review_reason: Some("manual review required for hybrid clause"),
        ..hard_contract_clause("clause.base")
    };
    let model = ready_user_intent_contract_draft_model()
        .with_contract_clause(validator)
        .with_contract_clause(receipt)
        .with_contract_clause(proof)
        .with_contract_clause(human)
        .with_receipt_requirement(receipt_requirement_for_hard_clause(
            "artifact_hashes",
            "clause.receipt",
        ));
    let result = confirm_contract_draft(&model, &explicit_draft_confirmation());
    let approved = result.approved().is_some_and(|approved| {
        approved.status() == ContractStatus::ApprovedForRun
            && approved
                .hard_clause_mapping_assessment()
                .ready_for_approval()
            && approved
                .receipt_requirement_assessment()
                .ready_for_approval()
            && approved
                .receipt_requirement_assessment()
                .requires_field("artifact_hashes")
            && approved.contract_clauses().len() == 4
    });

    if result.approved_for_run() && approved {
        SmokeEvalCaseResult::pass(
            "eval_ready_confirmed_draft_with_all_hard_clauses_mapped_can_approve",
            "mapped hard clauses allow approved_for_run",
            "explicit confirmation plus mapped hard clauses can produce approved_for_run model state",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_ready_confirmed_draft_with_all_hard_clauses_mapped_can_approve",
            "mapped hard clauses allow approved_for_run",
            format!(
                "mapped approval drifted; outcome={} blockers={} approved={approved}",
                result.outcome().as_str(),
                format_confirmation_blockers(result.blockers())
            ),
        )
    }
}

fn eval_ready_confirmed_draft_with_unmapped_hard_clause_cannot_approve() -> SmokeEvalCaseResult {
    let model = ready_user_intent_contract_draft_model()
        .with_contract_clause(hard_contract_clause("clause.unmapped"));
    let result = confirm_contract_draft(&model, &explicit_draft_confirmation());
    let blocked = result.outcome() == ContractDraftConfirmationOutcome::Blocked
        && result.has_blocker(ContractDraftConfirmationBlocker::UnsupportedHardClause)
        && !result.approved_for_run();

    if blocked {
        SmokeEvalCaseResult::pass(
            "eval_ready_confirmed_draft_with_unmapped_hard_clause_cannot_approve",
            "unmapped hard clause stops approved_for_run",
            "explicit confirmation cannot bypass hard-clause mapping requirements",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_ready_confirmed_draft_with_unmapped_hard_clause_cannot_approve",
            "unmapped hard clause stops approved_for_run",
            format!(
                "unmapped approval drifted; outcome={} blockers={}",
                result.outcome().as_str(),
                format_confirmation_blockers(result.blockers())
            ),
        )
    }
}

fn eval_hard_clause_mapping_does_not_create_gate_decision() -> SmokeEvalCaseResult {
    let assessment = assess_hard_clause_mappings(&[ContractClauseBlueprint {
        id: "clause.human",
        gate_review_required: true,
        human_gate_review_reason: Some("manual review required"),
        ..hard_contract_clause("clause.base")
    }]);

    if assessment.ready_for_approval() && !assessment.writes_gate_decision() {
        SmokeEvalCaseResult::pass(
            "eval_hard_clause_mapping_does_not_create_gate_decision",
            "hard-clause mapping writes no gate outcome",
            "human review mapping declares a future input only and writes no closure authority",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_hard_clause_mapping_does_not_create_gate_decision",
            "hard-clause mapping writes no gate outcome",
            format!(
                "gate boundary drifted; writes_gate={}",
                assessment.writes_gate_decision()
            ),
        )
    }
}

fn eval_hard_clause_mapping_does_not_create_proofpack() -> SmokeEvalCaseResult {
    let assessment = assess_hard_clause_mappings(&[ContractClauseBlueprint {
        id: "clause.proof",
        has_proof_requirement_refs: true,
        ..hard_contract_clause("clause.base")
    }]);

    if assessment.ready_for_approval() && !assessment.creates_proofpack() {
        SmokeEvalCaseResult::pass(
            "eval_hard_clause_mapping_does_not_create_proofpack",
            "hard-clause mapping creates no proofpack",
            "proof requirement refs are declarative and do not write proof artifacts",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_hard_clause_mapping_does_not_create_proofpack",
            "hard-clause mapping creates no proofpack",
            format!(
                "proofpack boundary drifted; creates_proofpack={}",
                assessment.creates_proofpack()
            ),
        )
    }
}

fn eval_hard_clause_mapping_does_not_invoke_writer() -> SmokeEvalCaseResult {
    let assessment = assess_hard_clause_mappings(&[ContractClauseBlueprint {
        id: "clause.validator",
        has_validator_refs: true,
        ..hard_contract_clause("clause.base")
    }]);

    if assessment.ready_for_approval() && !assessment.invokes_writer() {
        SmokeEvalCaseResult::pass(
            "eval_hard_clause_mapping_does_not_invoke_writer",
            "hard-clause mapping does not invoke Writer",
            "mapping keeps Writer downstream and cannot authorize Writer behavior",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_hard_clause_mapping_does_not_invoke_writer",
            "hard-clause mapping does not invoke Writer",
            format!(
                "Writer boundary drifted; invokes_writer={}",
                assessment.invokes_writer()
            ),
        )
    }
}

fn eval_contract_status_excludes_acceptance_outcomes_after_hard_clause_mapping(
) -> SmokeEvalCaseResult {
    let status_names = CONTRACT_STATUS_VALUES
        .iter()
        .map(|status| status.as_str())
        .collect::<Vec<_>>();
    let ok = !status_names.contains(&"accepted")
        && !status_names.contains(&"rejected")
        && !status_names.contains(&"partially_accepted")
        && status_names.contains(&"approved_for_run");

    if ok {
        SmokeEvalCaseResult::pass(
            "eval_contract_status_excludes_acceptance_outcomes_after_hard_clause_mapping",
            "contract status remains lifecycle-only after mapping",
            "hard-clause mapping did not add closure outcomes to ContractStatus",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_contract_status_excludes_acceptance_outcomes_after_hard_clause_mapping",
            "contract status remains lifecycle-only after mapping",
            format!("status drifted; names={status_names:?}"),
        )
    }
}

fn eval_receipt_requirement_covers_hard_clause_field(
    case_id: &'static str,
    summary: &'static str,
    field: &'static str,
    fields: &'static [&'static str],
    assessment_text: &'static str,
) -> SmokeEvalCaseResult {
    let clause = hard_contract_clause_with_receipt_fields("clause.receipt", fields);
    let requirement = receipt_requirement_for_hard_clause(field, "clause.receipt");
    let assessment = assess_receipt_requirements(&[clause], &[requirement]);
    let covered = assessment.ready_for_approval()
        && assessment.requires_field(field)
        && assessment.has_status(ReceiptRequirementStatus::SatisfiedByRequiredField)
        && assessment.coverage().iter().any(|coverage| {
            coverage.clause_id() == "clause.receipt"
                && coverage.field() == field
                && coverage.status() == ReceiptRequirementStatus::SatisfiedByRequiredField
        });

    if covered {
        SmokeEvalCaseResult::pass(case_id, summary, assessment_text)
    } else {
        SmokeEvalCaseResult::fail(
            case_id,
            summary,
            format!(
                "receipt requirement coverage drifted; field={field} required_fields={:?} findings={:?} coverage={:?}",
                assessment.required_fields(),
                assessment.findings(),
                assessment.coverage()
            ),
        )
    }
}

fn eval_receipt_requirement_covers_artifact_hashes_hard_clause() -> SmokeEvalCaseResult {
    eval_receipt_requirement_covers_hard_clause_field(
        "eval_receipt_requirement_covers_artifact_hashes_hard_clause",
        "artifact hashes cover hard-clause receipt mapping",
        "artifact_hashes",
        &["artifact_hashes"],
        "artifact_hashes is declared as future run receipt evidence for a hard clause",
    )
}

fn eval_receipt_requirement_covers_validator_results_hard_clause() -> SmokeEvalCaseResult {
    eval_receipt_requirement_covers_hard_clause_field(
        "eval_receipt_requirement_covers_validator_results_hard_clause",
        "validator results cover hard-clause receipt mapping",
        "validator_results",
        &["validator_results"],
        "validator_results is declared as future run receipt evidence without executing validators",
    )
}

fn eval_receipt_requirement_covers_deviation_flags_hard_clause() -> SmokeEvalCaseResult {
    eval_receipt_requirement_covers_hard_clause_field(
        "eval_receipt_requirement_covers_deviation_flags_hard_clause",
        "deviation flags cover hard-clause receipt mapping",
        "deviation_flags",
        &["deviation_flags"],
        "deviation_flags is available for later scope/evidence deviation surfacing",
    )
}

fn eval_receipt_requirement_covers_side_effects_hard_clause() -> SmokeEvalCaseResult {
    eval_receipt_requirement_covers_hard_clause_field(
        "eval_receipt_requirement_covers_side_effects_hard_clause",
        "side effects cover hard-clause receipt mapping",
        "side_effects",
        &["side_effects"],
        "side_effects is available for future receipt capture without enforcing runtime policy",
    )
}

fn eval_missing_receipt_requirement_blocks_approved_for_run() -> SmokeEvalCaseResult {
    let clause = hard_contract_clause_with_receipt_fields("clause.artifacts", &["artifact_hashes"]);
    let model = ready_user_intent_contract_draft_model().with_contract_clause(clause);
    let result = confirm_contract_draft(&model, &explicit_draft_confirmation());
    let blocked = result.outcome() == ContractDraftConfirmationOutcome::Blocked
        && result.has_blocker(ContractDraftConfirmationBlocker::MissingRequiredReceiptField)
        && !result.approved_for_run()
        && result
            .receipt_requirement_assessment()
            .is_some_and(|assessment| {
                assessment.has_status(ReceiptRequirementStatus::MissingRequiredField)
            });

    if blocked {
        SmokeEvalCaseResult::pass(
            "eval_missing_receipt_requirement_blocks_approved_for_run",
            "missing hard-clause receipt requirement blocks",
            "a hard clause mapped to artifact_hashes cannot reach approved_for_run without the matching receipt requirement",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_missing_receipt_requirement_blocks_approved_for_run",
            "missing hard-clause receipt requirement blocks",
            format!(
                "missing receipt requirement drifted; outcome={} blockers={}",
                result.outcome().as_str(),
                format_confirmation_blockers(result.blockers())
            ),
        )
    }
}

fn eval_unknown_receipt_requirement_field_is_blocking() -> SmokeEvalCaseResult {
    let clause =
        hard_contract_clause_with_receipt_fields("clause.unknown", &["unknown_receipt_field"]);
    let model = ready_user_intent_contract_draft_model().with_contract_clause(clause);
    let result = confirm_contract_draft(&model, &explicit_draft_confirmation());
    let blocked = result.outcome() == ContractDraftConfirmationOutcome::Blocked
        && result.has_blocker(ContractDraftConfirmationBlocker::UnsupportedReceiptField)
        && !result.approved_for_run()
        && result
            .receipt_requirement_assessment()
            .is_some_and(|assessment| {
                assessment.has_status(ReceiptRequirementStatus::UnsupportedField)
            });

    if blocked {
        SmokeEvalCaseResult::pass(
            "eval_unknown_receipt_requirement_field_is_blocking",
            "unknown receipt field blocks",
            "unknown receipt fields are unsupported and must not be silently treated as evidence",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_unknown_receipt_requirement_field_is_blocking",
            "unknown receipt field blocks",
            format!(
                "unknown receipt field drifted; outcome={} blockers={}",
                result.outcome().as_str(),
                format_confirmation_blockers(result.blockers())
            ),
        )
    }
}

fn eval_duplicate_receipt_requirements_are_non_conflicting() -> SmokeEvalCaseResult {
    let clause = hard_contract_clause_with_receipt_fields("clause.artifacts", &["artifact_hashes"]);
    let first = receipt_requirement_for_hard_clause("artifact_hashes", "clause.artifacts");
    let second = ContractReceiptRequirement::contract_default("artifact_hashes");
    let assessment = assess_receipt_requirements(&[clause], &[first, second]);
    let normalized = assessment.ready_for_approval()
        && assessment.has_status(ReceiptRequirementStatus::DuplicateRequirement)
        && assessment.required_fields() == ["artifact_hashes"]
        && assessment.requires_field("artifact_hashes");

    if normalized {
        SmokeEvalCaseResult::pass(
            "eval_duplicate_receipt_requirements_are_non_conflicting",
            "duplicate receipt requirements normalize",
            "duplicate receipt requirements preserve source findings while keeping one required field",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_duplicate_receipt_requirements_are_non_conflicting",
            "duplicate receipt requirements normalize",
            format!(
                "duplicate handling drifted; required_fields={:?} findings={:?}",
                assessment.required_fields(),
                assessment.findings()
            ),
        )
    }
}

fn eval_soft_clause_receipt_mapping_absent_is_non_blocking() -> SmokeEvalCaseResult {
    let clause = ContractClauseBlueprint {
        id: "clause.soft-receipt",
        severity: ContractClauseSeverity::Soft,
        has_required_receipt_fields: true,
        required_receipt_fields: &["artifact_hashes"],
        ..hard_contract_clause("clause.base")
    };
    let model = ready_user_intent_contract_draft_model().with_contract_clause(clause);
    let result = confirm_contract_draft(&model, &explicit_draft_confirmation());
    let non_blocking = result.approved_for_run()
        && result.approved().is_some_and(|approved| {
            approved
                .receipt_requirement_assessment()
                .has_status(ReceiptRequirementStatus::NotRequiredForClause)
        });

    if non_blocking {
        SmokeEvalCaseResult::pass(
            "eval_soft_clause_receipt_mapping_absent_is_non_blocking",
            "soft receipt mapping is non-blocking when absent",
            "soft-clause receipt fields remain advisory evidence needs unless promoted by a hard clause",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_soft_clause_receipt_mapping_absent_is_non_blocking",
            "soft receipt mapping is non-blocking when absent",
            format!(
                "soft receipt mapping drifted; outcome={} blockers={}",
                result.outcome().as_str(),
                format_confirmation_blockers(result.blockers())
            ),
        )
    }
}

fn eval_advisory_clause_receipt_mapping_absent_is_non_blocking() -> SmokeEvalCaseResult {
    let clause = ContractClauseBlueprint {
        id: "clause.advisory-receipt",
        severity: ContractClauseSeverity::Advisory,
        has_required_receipt_fields: true,
        required_receipt_fields: &["artifact_hashes"],
        ..hard_contract_clause("clause.base")
    };
    let model = ready_user_intent_contract_draft_model().with_contract_clause(clause);
    let result = confirm_contract_draft(&model, &explicit_draft_confirmation());
    let non_blocking = result.approved_for_run()
        && result.approved().is_some_and(|approved| {
            approved
                .receipt_requirement_assessment()
                .has_status(ReceiptRequirementStatus::NotRequiredForClause)
        });

    if non_blocking {
        SmokeEvalCaseResult::pass(
            "eval_advisory_clause_receipt_mapping_absent_is_non_blocking",
            "advisory receipt mapping is non-blocking when absent",
            "advisory receipt fields do not block approved_for_run unless promoted by a hard clause",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_advisory_clause_receipt_mapping_absent_is_non_blocking",
            "advisory receipt mapping is non-blocking when absent",
            format!(
                "advisory receipt mapping drifted; outcome={} blockers={}",
                result.outcome().as_str(),
                format_confirmation_blockers(result.blockers())
            ),
        )
    }
}

fn eval_executor_claims_receipt_field_remains_unverified_evidence() -> SmokeEvalCaseResult {
    let clause = hard_contract_clause_with_receipt_fields("clause.claims", &["executor_claims"]);
    let requirement = ContractReceiptRequirement::new(
        "executor_claims",
        ReceiptRequirementSource::ExecutorClaimPolicy,
    );
    let assessment = assess_receipt_requirements(&[clause], &[requirement]);
    let evidence_only = assessment.ready_for_approval()
        && assessment.requires_field("executor_claims")
        && !assessment.executor_claims_are_proof()
        && !requirement.executor_claims_are_proof();

    if evidence_only {
        SmokeEvalCaseResult::pass(
            "eval_executor_claims_receipt_field_remains_unverified_evidence",
            "executor claims stay unverified evidence",
            "executor_claims may be required as a receipt field but are not proof without linked validation/evidence",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_executor_claims_receipt_field_remains_unverified_evidence",
            "executor claims stay unverified evidence",
            format!(
                "executor claims boundary drifted; required={} claims_are_proof={}",
                assessment.requires_field("executor_claims"),
                assessment.executor_claims_are_proof()
            ),
        )
    }
}

fn eval_receipt_requirements_do_not_create_run_receipt() -> SmokeEvalCaseResult {
    let assessment = assess_receipt_requirements(
        &[],
        &[ContractReceiptRequirement::contract_default(
            "artifact_hashes",
        )],
    );

    if assessment.ready_for_approval()
        && !assessment.writes_run_receipt()
        && !assessment.executes_validators()
    {
        SmokeEvalCaseResult::pass(
            "eval_receipt_requirements_do_not_create_run_receipt",
            "receipt requirements write no run receipt",
            "receipt requirements declare future fields only and do not execute validators or create receipt evidence",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_receipt_requirements_do_not_create_run_receipt",
            "receipt requirements write no run receipt",
            format!(
                "run receipt boundary drifted; writes_receipt={} executes_validators={}",
                assessment.writes_run_receipt(),
                assessment.executes_validators()
            ),
        )
    }
}

fn eval_receipt_requirements_do_not_write_punk_runs_storage() -> SmokeEvalCaseResult {
    let assessment = assess_receipt_requirements(
        &[],
        &[ContractReceiptRequirement::contract_default(
            "artifact_refs",
        )],
    );

    if assessment.ready_for_approval() && !assessment.writes_punk_runs_storage() {
        SmokeEvalCaseResult::pass(
            "eval_receipt_requirements_do_not_write_punk_runs_storage",
            "receipt requirements write no .punk/runs storage",
            "receipt requirements are in-memory model state and do not persist runtime run artifacts",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_receipt_requirements_do_not_write_punk_runs_storage",
            "receipt requirements write no .punk/runs storage",
            format!(
                ".punk/runs boundary drifted; writes_storage={}",
                assessment.writes_punk_runs_storage()
            ),
        )
    }
}

fn eval_receipt_requirements_do_not_create_gate_decision() -> SmokeEvalCaseResult {
    let assessment = assess_receipt_requirements(
        &[],
        &[ContractReceiptRequirement::contract_default(
            "validator_results",
        )],
    );

    if assessment.ready_for_approval() && !assessment.writes_gate_decision() {
        SmokeEvalCaseResult::pass(
            "eval_receipt_requirements_do_not_create_gate_decision",
            "receipt requirements write no gate outcome",
            "receipt requirements can support later gate inputs but do not write closure authority",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_receipt_requirements_do_not_create_gate_decision",
            "receipt requirements write no gate outcome",
            format!(
                "gate outcome boundary drifted; writes_gate={}",
                assessment.writes_gate_decision()
            ),
        )
    }
}

fn eval_receipt_requirements_do_not_create_proofpack() -> SmokeEvalCaseResult {
    let assessment = assess_receipt_requirements(
        &[],
        &[ContractReceiptRequirement::contract_default(
            "artifact_hashes",
        )],
    );

    if assessment.ready_for_approval() && !assessment.creates_proofpack() {
        SmokeEvalCaseResult::pass(
            "eval_receipt_requirements_do_not_create_proofpack",
            "receipt requirements create no proofpack",
            "receipt requirements may point to evidence needs but do not write proof artifacts",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_receipt_requirements_do_not_create_proofpack",
            "receipt requirements create no proofpack",
            format!(
                "proofpack boundary drifted; creates_proofpack={}",
                assessment.creates_proofpack()
            ),
        )
    }
}

fn eval_receipt_requirements_do_not_invoke_writer() -> SmokeEvalCaseResult {
    let assessment = assess_receipt_requirements(
        &[],
        &[ContractReceiptRequirement::contract_default("outputs")],
    );

    if assessment.ready_for_approval() && !assessment.invokes_writer() {
        SmokeEvalCaseResult::pass(
            "eval_receipt_requirements_do_not_invoke_writer",
            "receipt requirements do not invoke Writer",
            "receipt requirements keep Writer downstream of approved work, run evidence, gate, and proof",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_receipt_requirements_do_not_invoke_writer",
            "receipt requirements do not invoke Writer",
            format!(
                "Writer boundary drifted; invokes_writer={}",
                assessment.invokes_writer()
            ),
        )
    }
}

fn eval_contract_status_excludes_acceptance_outcomes_after_receipt_requirements(
) -> SmokeEvalCaseResult {
    let status_names = CONTRACT_STATUS_VALUES
        .iter()
        .map(|status| status.as_str())
        .collect::<Vec<_>>();
    let ok = !status_names.contains(&"accepted")
        && !status_names.contains(&"rejected")
        && !status_names.contains(&"partially_accepted")
        && status_names.contains(&"approved_for_run");

    if ok {
        SmokeEvalCaseResult::pass(
            "eval_contract_status_excludes_acceptance_outcomes_after_receipt_requirements",
            "contract status remains lifecycle-only after receipt requirements",
            "receipt requirements did not add closure outcomes to ContractStatus",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_contract_status_excludes_acceptance_outcomes_after_receipt_requirements",
            "contract status remains lifecycle-only after receipt requirements",
            format!("status drifted; names={status_names:?}"),
        )
    }
}

fn proof_requirements_without(target: &'static str) -> ContractProofRequirements {
    ContractProofRequirements::new(
        contract_proof_requirements_v0_1()
            .requirements()
            .iter()
            .copied()
            .filter(|requirement| requirement.target() != target)
            .collect(),
    )
}

fn eval_proof_requirements_require_target(
    case_id: &'static str,
    summary: &'static str,
    target: &'static str,
    assessment_text: &'static str,
) -> SmokeEvalCaseResult {
    let proof_requirements = contract_proof_requirements_v0_1();
    let assessment = assess_proof_requirements(&proof_requirements);
    let required = proof_requirements.requires_target(target)
        && assessment.proof_requirements_declared()
        && assessment.requires_target(target)
        && assessment.coverage().iter().any(|coverage| {
            coverage.target() == target && coverage.status() == ProofRequirementStatus::Covered
        });

    if required {
        SmokeEvalCaseResult::pass(case_id, summary, assessment_text)
    } else {
        SmokeEvalCaseResult::fail(
            case_id,
            summary,
            format!(
                "proof requirement target drifted; target={target} required_targets={:?} findings={:?} coverage={:?}",
                assessment.required_targets(),
                assessment.findings(),
                assessment.coverage()
            ),
        )
    }
}

fn eval_proof_requirements_require_contract_ref() -> SmokeEvalCaseResult {
    eval_proof_requirements_require_target(
        "eval_proof_requirements_require_contract_ref",
        "proof requirements require contract ref",
        "contract_ref",
        "future proofpack evidence must link contract identity after gate outcome",
    )
}

fn eval_proof_requirements_require_run_receipt_ref() -> SmokeEvalCaseResult {
    eval_proof_requirements_require_target(
        "eval_proof_requirements_require_run_receipt_ref",
        "proof requirements require run receipt ref",
        "run_receipt_ref",
        "future proofpack evidence must link run receipt evidence after gate outcome",
    )
}

fn eval_proof_requirements_require_gate_decision_ref() -> SmokeEvalCaseResult {
    eval_proof_requirements_require_target(
        "eval_proof_requirements_require_gate_decision_ref",
        "proof requirements require gate outcome ref",
        "gate_decision_ref",
        "future proofpack evidence must link the gate outcome after it exists",
    )
}

fn eval_proof_requirements_can_require_eval_report_ref_and_hash() -> SmokeEvalCaseResult {
    let proof_requirements = contract_proof_requirements_v0_1();
    let assessment = assess_proof_requirements(&proof_requirements);
    let required = proof_requirements.requires_target("eval_report_ref")
        && proof_requirements.requires_target("eval_report_hash")
        && assessment.proof_requirements_declared()
        && assessment.requires_target("eval_report_ref")
        && assessment.requires_target("eval_report_hash");

    if required {
        SmokeEvalCaseResult::pass(
            "eval_proof_requirements_can_require_eval_report_ref_and_hash",
            "proof requirements can require eval report refs and hashes",
            "future proofpack evidence may link eval report refs and declared hashes without running evals",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proof_requirements_can_require_eval_report_ref_and_hash",
            "proof requirements can require eval report refs and hashes",
            format!(
                "eval report proof targets drifted; required_targets={:?}",
                assessment.required_targets()
            ),
        )
    }
}

fn eval_proof_requirements_can_require_output_artifact_refs_and_hashes() -> SmokeEvalCaseResult {
    let proof_requirements = contract_proof_requirements_v0_1();
    let assessment = assess_proof_requirements(&proof_requirements);
    let required = proof_requirements.requires_target("output_artifact_refs")
        && proof_requirements.requires_target("output_artifact_hashes")
        && assessment.proof_requirements_declared()
        && assessment.requires_target("output_artifact_refs")
        && assessment.requires_target("output_artifact_hashes");

    if required {
        SmokeEvalCaseResult::pass(
            "eval_proof_requirements_can_require_output_artifact_refs_and_hashes",
            "proof requirements can require output artifact refs and hashes",
            "future proofpack evidence may link output artifact refs and declared hashes without reading files",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proof_requirements_can_require_output_artifact_refs_and_hashes",
            "proof requirements can require output artifact refs and hashes",
            format!(
                "output artifact proof targets drifted; required_targets={:?}",
                assessment.required_targets()
            ),
        )
    }
}

fn eval_missing_proof_target_blocks_declaration(
    case_id: &'static str,
    summary: &'static str,
    target: &'static str,
    assessment_text: &'static str,
) -> SmokeEvalCaseResult {
    let proof_requirements = proof_requirements_without(target);
    let assessment = assess_proof_requirements(&proof_requirements);
    let blocked = assessment.declaration_blocking()
        && !assessment.proof_requirements_declared()
        && assessment.has_status(ProofRequirementStatus::MissingRequiredTarget)
        && assessment.findings().iter().any(|finding| {
            finding.target() == target
                && finding.status() == ProofRequirementStatus::MissingRequiredTarget
        });

    if blocked {
        SmokeEvalCaseResult::pass(case_id, summary, assessment_text)
    } else {
        SmokeEvalCaseResult::fail(
            case_id,
            summary,
            format!(
                "missing proof target drifted; target={target} required_targets={:?} findings={:?}",
                assessment.required_targets(),
                assessment.findings()
            ),
        )
    }
}

fn eval_missing_contract_proof_target_is_blocking() -> SmokeEvalCaseResult {
    eval_missing_proof_target_blocks_declaration(
        "eval_missing_contract_proof_target_is_blocking",
        "missing contract proof target is blocking",
        "contract_ref",
        "proof requirement declaration blocks when contract identity is absent",
    )
}

fn eval_missing_run_receipt_proof_target_is_blocking() -> SmokeEvalCaseResult {
    eval_missing_proof_target_blocks_declaration(
        "eval_missing_run_receipt_proof_target_is_blocking",
        "missing run receipt proof target is blocking",
        "run_receipt_ref",
        "proof requirement declaration blocks when run receipt evidence is absent",
    )
}

fn eval_missing_gate_decision_proof_target_is_blocking() -> SmokeEvalCaseResult {
    eval_missing_proof_target_blocks_declaration(
        "eval_missing_gate_decision_proof_target_is_blocking",
        "missing gate outcome proof target is blocking",
        "gate_decision_ref",
        "proof requirement declaration blocks when gate outcome linkage is absent",
    )
}

fn eval_unsupported_proof_target_is_blocking() -> SmokeEvalCaseResult {
    let mut requirements = contract_proof_requirements_v0_1().requirements().to_vec();
    requirements.push(ProofRequirement::new(
        "unknown_proof_target",
        ProofRequirementSource::ContractPolicy,
    ));
    let proof_requirements = ContractProofRequirements::new(requirements);
    let assessment = assess_proof_requirements(&proof_requirements);
    let blocked = assessment.declaration_blocking()
        && !assessment.proof_requirements_declared()
        && assessment.has_status(ProofRequirementStatus::UnsupportedTarget);

    if blocked {
        SmokeEvalCaseResult::pass(
            "eval_unsupported_proof_target_is_blocking",
            "unsupported proof target is blocking",
            "unknown proof targets fail closed instead of becoming implicit proof evidence",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_unsupported_proof_target_is_blocking",
            "unsupported proof target is blocking",
            format!(
                "unsupported proof target drifted; findings={:?}",
                assessment.findings()
            ),
        )
    }
}

fn eval_duplicate_proof_targets_are_non_conflicting() -> SmokeEvalCaseResult {
    let mut requirements = contract_proof_requirements_v0_1().requirements().to_vec();
    requirements.push(ProofRequirement::contract_policy("contract_ref"));
    let proof_requirements = ContractProofRequirements::new(requirements);
    let assessment = assess_proof_requirements(&proof_requirements);
    let normalized = assessment.proof_requirements_declared()
        && assessment.has_status(ProofRequirementStatus::DuplicateRequirement)
        && assessment
            .required_targets()
            .iter()
            .filter(|target| **target == "contract_ref")
            .count()
            == 1;

    if normalized {
        SmokeEvalCaseResult::pass(
            "eval_duplicate_proof_targets_are_non_conflicting",
            "duplicate proof targets normalize",
            "duplicate proof targets preserve findings without duplicating required coverage",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_duplicate_proof_targets_are_non_conflicting",
            "duplicate proof targets normalize",
            format!(
                "duplicate proof target drifted; required_targets={:?} findings={:?}",
                assessment.required_targets(),
                assessment.findings()
            ),
        )
    }
}

fn eval_proof_requirements_do_not_create_proofpack() -> SmokeEvalCaseResult {
    let blueprint = contract_proof_requirements_blueprint();
    let proof_requirements = contract_proof_requirements_v0_1();
    let assessment = assess_proof_requirements(&proof_requirements);
    let links_ok = blueprint.must_link.contains(&"contract_ref")
        && blueprint.must_link.contains(&"run_receipt_ref")
        && blueprint.must_link.contains(&"gate_decision_ref")
        && blueprint.must_link.contains(&"output_artifact_refs");
    let hashes_ok = blueprint.must_hash.contains(&"contract_hash")
        && blueprint.must_hash.contains(&"run_receipt_hash")
        && blueprint.must_hash.contains(&"gate_decision_hash")
        && blueprint.must_hash.contains(&"output_artifact_hashes");
    let boundary_ok = !blueprint.creates_proofpack
        && !blueprint.proofpack_is_decision_authority
        && assessment.proof_requirements_declared()
        && !proof_requirements.creates_proofpack()
        && !assessment.creates_proofpack();

    if links_ok && hashes_ok && boundary_ok {
        SmokeEvalCaseResult::pass(
            "eval_proof_requirements_do_not_create_proofpack",
            "proof requirements stay declarative",
            "contract proof requirements declare future links and hashes without creating proofpacks or owning closure authority",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proof_requirements_do_not_create_proofpack",
            "proof requirements stay declarative",
            format!(
                "proof requirements drifted; links_ok={links_ok} hashes_ok={hashes_ok} boundary_ok={boundary_ok}"
            ),
        )
    }
}

fn eval_proof_requirements_do_not_write_punk_proofs_storage() -> SmokeEvalCaseResult {
    let proof_requirements = contract_proof_requirements_v0_1();
    let assessment = assess_proof_requirements(&proof_requirements);

    if assessment.proof_requirements_declared()
        && !proof_requirements.writes_punk_proofs_storage()
        && !assessment.writes_punk_proofs_storage()
    {
        SmokeEvalCaseResult::pass(
            "eval_proof_requirements_do_not_write_punk_proofs_storage",
            "proof requirements write no proof storage",
            "proof requirements are declarations only and do not write .punk/proofs",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proof_requirements_do_not_write_punk_proofs_storage",
            "proof requirements write no proof storage",
            format!(
                "proof storage boundary drifted; model_writes={} assessment_writes={}",
                proof_requirements.writes_punk_proofs_storage(),
                assessment.writes_punk_proofs_storage()
            ),
        )
    }
}

fn eval_proof_requirements_do_not_compute_artifact_hashes_from_filesystem() -> SmokeEvalCaseResult {
    let proof_requirements = contract_proof_requirements_v0_1();
    let assessment = assess_proof_requirements(&proof_requirements);

    if assessment.proof_requirements_declared()
        && !proof_requirements.computes_artifact_hashes_from_filesystem()
        && !assessment.computes_artifact_hashes_from_filesystem()
    {
        SmokeEvalCaseResult::pass(
            "eval_proof_requirements_do_not_compute_artifact_hashes_from_filesystem",
            "proof requirements compute no artifact hashes",
            "proof requirements may declare hash obligations but do not read files or compute artifact hashes",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proof_requirements_do_not_compute_artifact_hashes_from_filesystem",
            "proof requirements compute no artifact hashes",
            format!(
                "artifact hash runtime boundary drifted; model_computes={} assessment_computes={}",
                proof_requirements.computes_artifact_hashes_from_filesystem(),
                assessment.computes_artifact_hashes_from_filesystem()
            ),
        )
    }
}

fn eval_proof_requirements_do_not_write_gate_decision() -> SmokeEvalCaseResult {
    let proof_requirements = contract_proof_requirements_v0_1();
    let assessment = assess_proof_requirements(&proof_requirements);

    if assessment.proof_requirements_declared()
        && !proof_requirements.writes_gate_decision()
        && !assessment.writes_gate_decision()
    {
        SmokeEvalCaseResult::pass(
            "eval_proof_requirements_do_not_write_gate_decision",
            "proof requirements write no gate outcome",
            "proof requirements are downstream declarations and cannot write closure outcomes",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proof_requirements_do_not_write_gate_decision",
            "proof requirements write no gate outcome",
            format!(
                "gate writer boundary drifted; model_writes={} assessment_writes={}",
                proof_requirements.writes_gate_decision(),
                assessment.writes_gate_decision()
            ),
        )
    }
}

fn eval_proof_requirements_do_not_create_acceptance_claim() -> SmokeEvalCaseResult {
    let proof_requirements = contract_proof_requirements_v0_1();
    let assessment = assess_proof_requirements(&proof_requirements);

    if assessment.proof_requirements_declared()
        && !proof_requirements.creates_acceptance_claim()
        && !assessment.creates_acceptance_claim()
    {
        SmokeEvalCaseResult::pass(
            "eval_proof_requirements_do_not_create_acceptance_claim",
            "proof requirements create no acceptance claim",
            "acceptance claims remain downstream of accepting gate outcome plus matching proof",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proof_requirements_do_not_create_acceptance_claim",
            "proof requirements create no acceptance claim",
            format!(
                "acceptance claim boundary drifted; model_claim={} assessment_claim={}",
                proof_requirements.creates_acceptance_claim(),
                assessment.creates_acceptance_claim()
            ),
        )
    }
}

fn eval_proof_requirements_do_not_invoke_writer() -> SmokeEvalCaseResult {
    let proof_requirements = contract_proof_requirements_v0_1();
    let assessment = assess_proof_requirements(&proof_requirements);

    if assessment.proof_requirements_declared()
        && !proof_requirements.invokes_writer()
        && !assessment.invokes_writer()
    {
        SmokeEvalCaseResult::pass(
            "eval_proof_requirements_do_not_invoke_writer",
            "proof requirements do not invoke Writer",
            "proof requirements do not invoke Writer or authorize writer behavior",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proof_requirements_do_not_invoke_writer",
            "proof requirements do not invoke Writer",
            format!(
                "Writer boundary drifted; model_invokes={} assessment_invokes={}",
                proof_requirements.invokes_writer(),
                assessment.invokes_writer()
            ),
        )
    }
}

fn eval_proofpack_is_not_required_before_gate_decision() -> SmokeEvalCaseResult {
    let proof_requirements = contract_proof_requirements_v0_1();
    let assessment = assess_proof_requirements(&proof_requirements);
    let no_pre_gate_proofpack = assessment.proof_requirements_declared()
        && !proof_requirements.requires_proofpack_before_gate()
        && !assessment.requires_proofpack_before_gate()
        && !proof_requirements.requires_target("proofpack")
        && !proof_requirements.requires_target("proofpack_ref");

    if no_pre_gate_proofpack {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_is_not_required_before_gate_decision",
            "proofpack is not required before gate outcome",
            "proofpack remains downstream and cannot be a prerequisite for gate readiness",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_is_not_required_before_gate_decision",
            "proofpack is not required before gate outcome",
            format!(
                "proofpack-before-gate drifted; model_requires={} assessment_requires={} required_targets={:?}",
                proof_requirements.requires_proofpack_before_gate(),
                assessment.requires_proofpack_before_gate(),
                assessment.required_targets()
            ),
        )
    }
}

fn eval_contract_status_excludes_acceptance_outcomes_after_proof_requirements(
) -> SmokeEvalCaseResult {
    let status_names = CONTRACT_STATUS_VALUES
        .iter()
        .map(|status| status.as_str())
        .collect::<Vec<_>>();
    let ok = !status_names.contains(&"accepted")
        && !status_names.contains(&"rejected")
        && !status_names.contains(&"partially_accepted")
        && status_names.contains(&"approved_for_run");

    if ok {
        SmokeEvalCaseResult::pass(
            "eval_contract_status_excludes_acceptance_outcomes_after_proof_requirements",
            "contract status remains lifecycle-only after proof requirements",
            "proof requirements did not add closure outcomes to ContractStatus",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_contract_status_excludes_acceptance_outcomes_after_proof_requirements",
            "contract status remains lifecycle-only after proof requirements",
            format!("status drifted; names={status_names:?}"),
        )
    }
}

fn eval_gate_policy_does_not_write_decision() -> SmokeEvalCaseResult {
    let gate_policy = contract_gate_policy_blueprint();
    let inputs_ok = gate_policy.required_inputs.contains(&"contract_ref")
        && gate_policy
            .required_inputs
            .contains(&"approved_for_run_status")
        && gate_policy.required_inputs.contains(&"run_receipt_ref")
        && gate_policy
            .required_inputs
            .contains(&"receipt_requirement_coverage")
        && gate_policy
            .required_inputs
            .contains(&"hard_clause_mapping_assessment")
        && !gate_policy.required_inputs.contains(&"proofpack")
        && !gate_policy.required_inputs.contains(&"proofpack_ref");
    let rules_ok = gate_policy.hard_fail_on.contains(&"unmapped_hard_clause")
        && gate_policy
            .accept_requires
            .contains(&"satisfied_hard_clauses");
    let boundary_ok = gate_policy.gate_only_final_decision && !gate_policy.writes_decision;

    if inputs_ok && rules_ok && boundary_ok {
        SmokeEvalCaseResult::pass(
            "eval_gate_policy_does_not_write_decision",
            "gate policy remains input policy only",
            "contract gate policy declares required inputs and hard-fail conditions without writing authoritative closure",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_gate_policy_does_not_write_decision",
            "gate policy remains input policy only",
            format!(
                "gate policy drifted; inputs_ok={inputs_ok} rules_ok={rules_ok} boundary_ok={boundary_ok}"
            ),
        )
    }
}

fn complete_gate_input_evidence() -> Vec<GateInputEvidence> {
    CONTRACT_GATE_INPUT_POLICY_BASELINE_REQUIREMENTS
        .iter()
        .map(|input| GateInputEvidence::new(*input))
        .collect()
}

fn gate_input_evidence_without(input: &'static str) -> Vec<GateInputEvidence> {
    CONTRACT_GATE_INPUT_POLICY_BASELINE_REQUIREMENTS
        .iter()
        .filter(|required| **required != input)
        .map(|required| GateInputEvidence::new(*required))
        .collect()
}

fn eval_gate_input_policy_requires_input(
    case_id: &'static str,
    summary: &'static str,
    input: &'static str,
    assessment_text: &'static str,
) -> SmokeEvalCaseResult {
    let policy = contract_gate_input_policy_v0_1();
    let assessment = assess_gate_input_policy(&policy, &complete_gate_input_evidence());
    let required = policy.requires_input(input)
        && assessment.ready_for_gate()
        && assessment.requires_input(input)
        && assessment.coverage().iter().any(|coverage| {
            coverage.input() == input && coverage.status() == GateInputRequirementStatus::Covered
        });

    if required {
        SmokeEvalCaseResult::pass(case_id, summary, assessment_text)
    } else {
        SmokeEvalCaseResult::fail(
            case_id,
            summary,
            format!(
                "gate input requirement drifted; input={input} required_inputs={:?} findings={:?} coverage={:?}",
                assessment.required_inputs(),
                assessment.findings(),
                assessment.coverage()
            ),
        )
    }
}

fn eval_gate_input_policy_requires_contract_ref() -> SmokeEvalCaseResult {
    eval_gate_input_policy_requires_input(
        "eval_gate_input_policy_requires_contract_ref",
        "gate input policy requires contract ref",
        "contract_ref",
        "future gate readiness requires contract identity without writing a closure outcome",
    )
}

fn eval_gate_input_policy_requires_approved_for_run_status() -> SmokeEvalCaseResult {
    eval_gate_input_policy_requires_input(
        "eval_gate_input_policy_requires_approved_for_run_status",
        "gate input policy requires approved_for_run status",
        "approved_for_run_status",
        "future gate readiness requires approved_for_run evidence and cannot decide on drafts",
    )
}

fn eval_gate_input_policy_requires_run_receipt_ref() -> SmokeEvalCaseResult {
    eval_gate_input_policy_requires_input(
        "eval_gate_input_policy_requires_run_receipt_ref",
        "gate input policy requires run receipt ref",
        "run_receipt_ref",
        "future gate readiness requires run receipt evidence without writing receipts",
    )
}

fn eval_gate_input_policy_requires_receipt_requirement_coverage() -> SmokeEvalCaseResult {
    eval_gate_input_policy_requires_input(
        "eval_gate_input_policy_requires_receipt_requirement_coverage",
        "gate input policy requires receipt requirement coverage",
        "receipt_requirement_coverage",
        "future gate readiness requires coverage for declared receipt requirements",
    )
}

fn eval_gate_input_policy_requires_hard_clause_mapping_assessment() -> SmokeEvalCaseResult {
    eval_gate_input_policy_requires_input(
        "eval_gate_input_policy_requires_hard_clause_mapping_assessment",
        "gate input policy requires hard-clause mapping assessment",
        "hard_clause_mapping_assessment",
        "future gate readiness requires hard-clause mapping evidence without executing validators",
    )
}

fn eval_gate_input_policy_can_require_validator_or_eval_report() -> SmokeEvalCaseResult {
    eval_gate_input_policy_requires_input(
        "eval_gate_input_policy_can_require_validator_or_eval_report",
        "gate input policy can require validator/eval reports",
        "validator_or_eval_report",
        "future gate readiness can require validator/eval report refs without running validators",
    )
}

fn eval_gate_input_policy_can_require_deviation_status() -> SmokeEvalCaseResult {
    eval_gate_input_policy_requires_input(
        "eval_gate_input_policy_can_require_deviation_status",
        "gate input policy can require deviation status",
        "deviation_status",
        "future gate readiness can require deviation status so scope/evidence drift is visible",
    )
}

fn eval_gate_input_policy_can_require_executor_claim_status_without_proof() -> SmokeEvalCaseResult {
    let policy = contract_gate_input_policy_v0_1();
    let assessment = assess_gate_input_policy(&policy, &complete_gate_input_evidence());
    let evidence_only = policy.requires_input("executor_claim_status")
        && assessment.ready_for_gate()
        && assessment.requires_input("executor_claim_status")
        && !assessment.executor_claims_are_proof();

    if evidence_only {
        SmokeEvalCaseResult::pass(
            "eval_gate_input_policy_can_require_executor_claim_status_without_proof",
            "gate input policy can require executor claim status",
            "executor claim status is gate-visible evidence only and does not make executor claims proof",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_gate_input_policy_can_require_executor_claim_status_without_proof",
            "gate input policy can require executor claim status",
            format!(
                "executor claim boundary drifted; required={} claims_are_proof={}",
                assessment.requires_input("executor_claim_status"),
                assessment.executor_claims_are_proof()
            ),
        )
    }
}

fn eval_missing_gate_input_blocks_readiness(
    case_id: &'static str,
    summary: &'static str,
    input: &'static str,
    assessment_text: &'static str,
) -> SmokeEvalCaseResult {
    let policy = contract_gate_input_policy_v0_1();
    let assessment = assess_gate_input_policy(&policy, &gate_input_evidence_without(input));
    let blocked = assessment.gate_readiness_blocking()
        && !assessment.ready_for_gate()
        && assessment.has_status(GateInputRequirementStatus::MissingRequiredInput)
        && assessment.coverage().iter().any(|coverage| {
            coverage.input() == input
                && coverage.status() == GateInputRequirementStatus::MissingRequiredInput
        });

    if blocked {
        SmokeEvalCaseResult::pass(case_id, summary, assessment_text)
    } else {
        SmokeEvalCaseResult::fail(
            case_id,
            summary,
            format!(
                "missing gate input drifted; input={input} required_inputs={:?} coverage={:?}",
                assessment.required_inputs(),
                assessment.coverage()
            ),
        )
    }
}

fn eval_missing_contract_ref_blocks_gate_readiness() -> SmokeEvalCaseResult {
    eval_missing_gate_input_blocks_readiness(
        "eval_missing_contract_ref_blocks_gate_readiness",
        "missing contract ref blocks gate readiness",
        "contract_ref",
        "gate readiness blocks when contract identity evidence is absent",
    )
}

fn eval_missing_approved_for_run_status_blocks_gate_readiness() -> SmokeEvalCaseResult {
    eval_missing_gate_input_blocks_readiness(
        "eval_missing_approved_for_run_status_blocks_gate_readiness",
        "missing approved_for_run status blocks gate readiness",
        "approved_for_run_status",
        "gate readiness blocks when approved_for_run evidence is absent",
    )
}

fn eval_missing_run_receipt_ref_blocks_gate_readiness() -> SmokeEvalCaseResult {
    eval_missing_gate_input_blocks_readiness(
        "eval_missing_run_receipt_ref_blocks_gate_readiness",
        "missing run receipt ref blocks gate readiness",
        "run_receipt_ref",
        "gate readiness blocks when run receipt evidence is absent",
    )
}

fn eval_missing_receipt_requirement_coverage_blocks_gate_readiness() -> SmokeEvalCaseResult {
    eval_missing_gate_input_blocks_readiness(
        "eval_missing_receipt_requirement_coverage_blocks_gate_readiness",
        "missing receipt requirement coverage blocks gate readiness",
        "receipt_requirement_coverage",
        "gate readiness blocks when receipt requirement coverage is absent",
    )
}

fn eval_missing_hard_clause_mapping_assessment_blocks_gate_readiness() -> SmokeEvalCaseResult {
    eval_missing_gate_input_blocks_readiness(
        "eval_missing_hard_clause_mapping_assessment_blocks_gate_readiness",
        "missing hard-clause mapping assessment blocks gate readiness",
        "hard_clause_mapping_assessment",
        "gate readiness blocks when hard-clause mapping assessment is absent",
    )
}

fn eval_unsupported_gate_input_blocks_gate_readiness() -> SmokeEvalCaseResult {
    let mut requirements = contract_gate_input_policy_v0_1().requirements().to_vec();
    requirements.push(GateInputRequirement::new(
        "unknown_gate_input",
        GateInputRequirementSource::ContractPolicy,
    ));
    let policy = ContractGateInputPolicy::new(requirements);
    let assessment = assess_gate_input_policy(&policy, &complete_gate_input_evidence());
    let blocked = assessment.gate_readiness_blocking()
        && !assessment.ready_for_gate()
        && assessment.has_status(GateInputRequirementStatus::UnsupportedInput);

    if blocked {
        SmokeEvalCaseResult::pass(
            "eval_unsupported_gate_input_blocks_gate_readiness",
            "unsupported gate input blocks readiness",
            "unknown gate inputs fail closed instead of becoming implicit evidence",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_unsupported_gate_input_blocks_gate_readiness",
            "unsupported gate input blocks readiness",
            format!(
                "unsupported gate input drifted; findings={:?}",
                assessment.findings()
            ),
        )
    }
}

fn eval_duplicate_gate_input_requirements_are_non_conflicting() -> SmokeEvalCaseResult {
    let mut requirements = contract_gate_input_policy_v0_1().requirements().to_vec();
    requirements.push(GateInputRequirement::contract_policy("contract_ref"));
    let policy = ContractGateInputPolicy::new(requirements);
    let assessment = assess_gate_input_policy(&policy, &complete_gate_input_evidence());
    let normalized = assessment.ready_for_gate()
        && assessment.has_status(GateInputRequirementStatus::DuplicateRequirement)
        && assessment
            .required_inputs()
            .iter()
            .filter(|input| **input == "contract_ref")
            .count()
            == 1;

    if normalized {
        SmokeEvalCaseResult::pass(
            "eval_duplicate_gate_input_requirements_are_non_conflicting",
            "duplicate gate input requirements normalize",
            "duplicate gate input requirements preserve findings without duplicating required coverage",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_duplicate_gate_input_requirements_are_non_conflicting",
            "duplicate gate input requirements normalize",
            format!(
                "duplicate gate input drifted; required_inputs={:?} findings={:?}",
                assessment.required_inputs(),
                assessment.findings()
            ),
        )
    }
}

fn eval_gate_input_policy_does_not_write_gate_decision() -> SmokeEvalCaseResult {
    let policy = contract_gate_input_policy_v0_1();
    let assessment = assess_gate_input_policy(&policy, &complete_gate_input_evidence());

    if assessment.ready_for_gate()
        && !policy.writes_gate_decision()
        && !assessment.writes_gate_decision()
    {
        SmokeEvalCaseResult::pass(
            "eval_gate_input_policy_does_not_write_gate_decision",
            "gate input policy writes no gate outcome",
            "gate input policy is preflight/evidence modeling only and cannot write closure outcomes",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_gate_input_policy_does_not_write_gate_decision",
            "gate input policy writes no gate outcome",
            format!(
                "gate writer boundary drifted; policy_writes={} assessment_writes={}",
                policy.writes_gate_decision(),
                assessment.writes_gate_decision()
            ),
        )
    }
}

fn eval_gate_input_policy_does_not_create_proofpack() -> SmokeEvalCaseResult {
    let policy = contract_gate_input_policy_v0_1();
    let assessment = assess_gate_input_policy(&policy, &complete_gate_input_evidence());

    if assessment.ready_for_gate() && !policy.creates_proofpack() && !assessment.creates_proofpack()
    {
        SmokeEvalCaseResult::pass(
            "eval_gate_input_policy_does_not_create_proofpack",
            "gate input policy creates no proofpack",
            "proofpack remains downstream of gate outcome and is not created by gate input assessment",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_gate_input_policy_does_not_create_proofpack",
            "gate input policy creates no proofpack",
            format!(
                "proofpack boundary drifted; policy_creates={} assessment_creates={}",
                policy.creates_proofpack(),
                assessment.creates_proofpack()
            ),
        )
    }
}

fn eval_gate_input_policy_does_not_require_existing_proofpack() -> SmokeEvalCaseResult {
    let policy = contract_gate_input_policy_v0_1();
    let assessment = assess_gate_input_policy(&policy, &complete_gate_input_evidence());
    let no_proofpack_input = assessment.ready_for_gate()
        && !policy.requires_existing_proofpack()
        && !assessment.requires_existing_proofpack()
        && policy.post_gate_proof_requirement_pending()
        && !assessment.requires_input("proofpack")
        && !assessment.requires_input("proofpack_ref");

    if no_proofpack_input {
        SmokeEvalCaseResult::pass(
            "eval_gate_input_policy_does_not_require_existing_proofpack",
            "gate input policy does not require proofpack",
            "proofpack is represented only as a post-gate expectation, never a gate prerequisite",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_gate_input_policy_does_not_require_existing_proofpack",
            "gate input policy does not require proofpack",
            format!(
                "proofpack-as-input drifted; policy_requires={} assessment_requires={} required_inputs={:?}",
                policy.requires_existing_proofpack(),
                assessment.requires_existing_proofpack(),
                assessment.required_inputs()
            ),
        )
    }
}

fn eval_gate_input_policy_does_not_create_acceptance_claim() -> SmokeEvalCaseResult {
    let policy = contract_gate_input_policy_v0_1();
    let assessment = assess_gate_input_policy(&policy, &complete_gate_input_evidence());

    if assessment.ready_for_gate()
        && !policy.creates_acceptance_claim()
        && !assessment.creates_acceptance_claim()
    {
        SmokeEvalCaseResult::pass(
            "eval_gate_input_policy_does_not_create_acceptance_claim",
            "gate input policy creates no acceptance claim",
            "gate input readiness is not acceptance authority and does not write acceptance claims",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_gate_input_policy_does_not_create_acceptance_claim",
            "gate input policy creates no acceptance claim",
            format!(
                "acceptance claim boundary drifted; policy_claim={} assessment_claim={}",
                policy.creates_acceptance_claim(),
                assessment.creates_acceptance_claim()
            ),
        )
    }
}

fn eval_gate_input_policy_does_not_invoke_writer() -> SmokeEvalCaseResult {
    let policy = contract_gate_input_policy_v0_1();
    let assessment = assess_gate_input_policy(&policy, &complete_gate_input_evidence());

    if assessment.ready_for_gate()
        && !policy.invokes_writer()
        && !assessment.invokes_writer()
        && !assessment.runs_validators()
        && !assessment.writes_storage()
    {
        SmokeEvalCaseResult::pass(
            "eval_gate_input_policy_does_not_invoke_writer",
            "gate input policy does not invoke Writer",
            "gate input policy does not invoke Writer, run validators, or write runtime storage",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_gate_input_policy_does_not_invoke_writer",
            "gate input policy does not invoke Writer",
            format!(
                "Writer/runtime boundary drifted; invokes_writer={} runs_validators={} writes_storage={}",
                assessment.invokes_writer(),
                assessment.runs_validators(),
                assessment.writes_storage()
            ),
        )
    }
}

fn eval_contract_status_excludes_acceptance_outcomes_after_gate_input_policy() -> SmokeEvalCaseResult
{
    let status_names = CONTRACT_STATUS_VALUES
        .iter()
        .map(|status| status.as_str())
        .collect::<Vec<_>>();
    let ok = !status_names.contains(&"accepted")
        && !status_names.contains(&"rejected")
        && !status_names.contains(&"partially_accepted")
        && status_names.contains(&"approved_for_run");

    if ok {
        SmokeEvalCaseResult::pass(
            "eval_contract_status_excludes_acceptance_outcomes_after_gate_input_policy",
            "contract status remains lifecycle-only after gate input policy",
            "gate input policy did not add closure outcomes to ContractStatus",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_contract_status_excludes_acceptance_outcomes_after_gate_input_policy",
            "contract status remains lifecycle-only after gate input policy",
            format!("status drifted; names={status_names:?}"),
        )
    }
}

fn eval_approved_for_run_is_not_ready_for_gate() -> SmokeEvalCaseResult {
    let policy = contract_gate_input_policy_v0_1();
    let assessment = assess_gate_input_policy(&policy, &[]);
    let separated = ContractStatus::ApprovedForRun.ready_for_bounded_work()
        && !policy.approved_for_run_is_ready_for_gate()
        && !assessment.approved_for_run_is_ready_for_gate()
        && !assessment.ready_for_gate();

    if separated {
        SmokeEvalCaseResult::pass(
            "eval_approved_for_run_is_not_ready_for_gate",
            "approved_for_run is not ready_for_gate",
            "approved_for_run can exist before run receipt/evidence; ready_for_gate requires post-run gate inputs",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_approved_for_run_is_not_ready_for_gate",
            "approved_for_run is not ready_for_gate",
            format!(
                "gate readiness boundary drifted; approved_ready={} policy_equates={} assessment_equates={} ready_for_gate={}",
                ContractStatus::ApprovedForRun.ready_for_bounded_work(),
                policy.approved_for_run_is_ready_for_gate(),
                assessment.approved_for_run_is_ready_for_gate(),
                assessment.ready_for_gate()
            ),
        )
    }
}

fn eval_writer_is_not_upstream_contract_authority() -> SmokeEvalCaseResult {
    let boundary = contract_writer_authority_boundary();
    let boundary_ok = !boundary.writer_is_upstream_planner
        && !boundary.writer_defines_contract
        && !boundary.writer_writes_gate_decision
        && !boundary.writer_claims_acceptance;

    if boundary_ok {
        SmokeEvalCaseResult::pass(
            "eval_writer_is_not_upstream_contract_authority",
            "Writer remains downstream of contract authority",
            "Writer is not allowed to infer intent, define the contract, write gate closure, or claim acceptance in the contract schema blueprint",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_writer_is_not_upstream_contract_authority",
            "Writer remains downstream of contract authority",
            format!(
                "Writer authority boundary drifted; upstream={} defines_contract={} writes_gate={} claims={}",
                boundary.writer_is_upstream_planner,
                boundary.writer_defines_contract,
                boundary.writer_writes_gate_decision,
                boundary.writer_claims_acceptance
            ),
        )
    }
}

fn ready_user_intent_contract_draft_model() -> UserIntentContractDraftModel {
    UserIntentContractDraftModel::new(
        "User wants a bounded upstream contract draft readiness model.",
        "Model whether the request is ready for contract confirmation before execution.",
        UserIntentResearchGate::new(UserIntentResearchGateClassification::R1, true)
            .with_ref("evals/specs/user-intent-to-contract-ux-flow-boundary.v0.1.md"),
        UserIntentDownstreamClosure::default(),
    )
    .with_scope_include("crates/punk-contract/src/lib.rs")
    .with_scope_exclude(".punk/**")
    .with_non_goal("runtime contract storage")
    .with_assumption("Level 0 done is manual closure with evidence")
    .with_acceptance_criterion("readiness is classified without execution")
    .with_evidence_plan_item("cargo test -p punk-contract")
    .with_side_effect_boundary("no schema, CLI, gate, proofpack, or Writer behavior")
    .with_context_pack_ref("ctxpack:advisory-only")
}

fn eval_user_intent_contract_draft_model_ready_for_confirmation() -> SmokeEvalCaseResult {
    let assessment = ready_user_intent_contract_draft_model().assess();

    if assessment.ready_for_user_confirmation()
        && assessment.blockers().is_empty()
        && assessment.readiness().as_str() == "ready_for_user_confirmation"
    {
        SmokeEvalCaseResult::pass(
            "eval_user_intent_contract_draft_model_ready_for_confirmation",
            "ready user intent draft remains pre-execution",
            "complete raw request, intent, scope, acceptance, evidence, research refs, and downstream closure expectations classify as ready for user confirmation",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_user_intent_contract_draft_model_ready_for_confirmation",
            "ready user intent draft remains pre-execution",
            format!(
                "readiness drifted to {} with blockers {}",
                assessment.readiness().as_str(),
                format_user_intent_blockers(assessment.blockers())
            ),
        )
    }
}

fn eval_user_intent_contract_draft_model_requires_clarification() -> SmokeEvalCaseResult {
    let assessment = ready_user_intent_contract_draft_model()
        .with_unknown(UserIntentUnknown::new("target module is ambiguous", true))
        .assess();

    if assessment.readiness() == UserIntentContractDraftReadiness::ClarificationRequired
        && assessment.has_blocker(UserIntentContractDraftBlocker::BlockingUnknown)
    {
        SmokeEvalCaseResult::pass(
            "eval_user_intent_contract_draft_model_requires_clarification",
            "blocking unknowns require clarification",
            "blocking unknowns keep the draft out of execution and classify as clarification_required",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_user_intent_contract_draft_model_requires_clarification",
            "blocking unknowns require clarification",
            format!(
                "clarification classification drifted to {} with blockers {}",
                assessment.readiness().as_str(),
                format_user_intent_blockers(assessment.blockers())
            ),
        )
    }
}

fn eval_user_intent_contract_draft_model_refuses_or_defers() -> SmokeEvalCaseResult {
    let assessment = ready_user_intent_contract_draft_model()
        .with_refusal_or_deferral_reason("request would activate runtime storage outside scope")
        .assess();

    if assessment.readiness() == UserIntentContractDraftReadiness::RefusedOrDeferred
        && assessment.has_blocker(UserIntentContractDraftBlocker::RefusedOrDeferred)
    {
        SmokeEvalCaseResult::pass(
            "eval_user_intent_contract_draft_model_refuses_or_defers",
            "unsafe or out-of-scope requests refuse or defer",
            "explicit stop reasons classify as refused_or_deferred before any execution begins",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_user_intent_contract_draft_model_refuses_or_defers",
            "unsafe or out-of-scope requests refuse or defer",
            format!(
                "refusal classification drifted to {} with blockers {}",
                assessment.readiness().as_str(),
                format_user_intent_blockers(assessment.blockers())
            ),
        )
    }
}

fn eval_user_intent_contract_draft_model_blocks_missing_evidence() -> SmokeEvalCaseResult {
    let assessment = UserIntentContractDraftModel::new(
        "User asks for a bounded model.",
        "Model readiness.",
        UserIntentResearchGate::new(UserIntentResearchGateClassification::R1, true),
        UserIntentDownstreamClosure::default(),
    )
    .with_scope_include("crates/punk-contract/src/lib.rs")
    .with_acceptance_criterion("readiness is classified")
    .assess();

    if assessment.readiness() == UserIntentContractDraftReadiness::Blocked
        && assessment.has_blocker(UserIntentContractDraftBlocker::MissingEvidencePlan)
        && assessment.has_blocker(UserIntentContractDraftBlocker::MissingRequiredResearchRefs)
    {
        SmokeEvalCaseResult::pass(
            "eval_user_intent_contract_draft_model_blocks_missing_evidence",
            "missing evidence or research refs block draft readiness",
            "missing evidence plan and required research refs classify as blocked without treating the raw request as a contract",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_user_intent_contract_draft_model_blocks_missing_evidence",
            "missing evidence or research refs block draft readiness",
            format!(
                "missing-evidence classification drifted to {} with blockers {}",
                assessment.readiness().as_str(),
                format_user_intent_blockers(assessment.blockers())
            ),
        )
    }
}

fn eval_user_intent_contract_draft_model_keeps_writer_downstream() -> SmokeEvalCaseResult {
    let assessment = UserIntentContractDraftModel::new(
        "User asks for Writer behavior.",
        "Keep Writer downstream of intent, contract, execution, evidence, gate, and proof.",
        UserIntentResearchGate::new(UserIntentResearchGateClassification::R0, false),
        UserIntentDownstreamClosure {
            executor_claims_are_proof: true,
            user_confirmation_is_gate_acceptance: true,
            writer_may_infer_intent: true,
            ..UserIntentDownstreamClosure::default()
        },
    )
    .with_scope_include("crates/punk-contract/src/lib.rs")
    .with_acceptance_criterion("authority boundaries stay separate")
    .with_evidence_plan_item("cargo test -p punk-contract")
    .assess();

    if assessment.readiness() == UserIntentContractDraftReadiness::Blocked
        && assessment.has_blocker(UserIntentContractDraftBlocker::ExecutorClaimsTreatedAsProof)
        && assessment
            .has_blocker(UserIntentContractDraftBlocker::UserConfirmationTreatedAsGateAcceptance)
        && assessment.has_blocker(UserIntentContractDraftBlocker::WriterMayInferIntent)
    {
        SmokeEvalCaseResult::pass(
            "eval_user_intent_contract_draft_model_keeps_writer_downstream",
            "Writer and acceptance authority remain downstream",
            "authority drift that treats executor claims as proof, user confirmation as gate acceptance, or Writer as intent authority stays blocked",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_user_intent_contract_draft_model_keeps_writer_downstream",
            "Writer and acceptance authority remain downstream",
            format!(
                "authority classification drifted to {} with blockers {}",
                assessment.readiness().as_str(),
                format_user_intent_blockers(assessment.blockers())
            ),
        )
    }
}

fn explicit_draft_confirmation() -> ContractDraftConfirmation {
    ContractDraftConfirmation::explicit(
        ContractDraftApprovalEvidence::new("vitaly")
            .with_confirmation_ref("work/reports/confirmation.md")
            .with_confirmed_at("2026-04-30T12:00:00Z"),
    )
}

fn eval_ready_draft_with_explicit_confirmation_becomes_approved_for_run() -> SmokeEvalCaseResult {
    let model = ready_user_intent_contract_draft_model();
    let result = confirm_contract_draft(&model, &explicit_draft_confirmation());
    let approved_ok = result.approved().is_some_and(|approved| {
        approved.status() == ContractStatus::ApprovedForRun
            && approved.intent() == model.intent()
            && approved
                .confirmation_evidence()
                .confirmation_ref()
                .is_some()
    });

    if result.outcome() == ContractDraftConfirmationOutcome::ConfirmedForRun
        && result.approved_for_run()
        && approved_ok
    {
        SmokeEvalCaseResult::pass(
            "eval_ready_draft_with_explicit_confirmation_becomes_approved_for_run",
            "explicit confirmation can produce approved_for_run model state",
            "ready draft plus explicit user confirmation produces approved_for_run model state in memory only",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_ready_draft_with_explicit_confirmation_becomes_approved_for_run",
            "explicit confirmation can produce approved_for_run model state",
            format!(
                "confirmation drifted; outcome={} blockers={} approved_ok={approved_ok}",
                result.outcome().as_str(),
                format_confirmation_blockers(result.blockers())
            ),
        )
    }
}

fn eval_ready_draft_without_explicit_confirmation_is_not_approved() -> SmokeEvalCaseResult {
    let result = confirm_contract_draft(
        &ready_user_intent_contract_draft_model(),
        &ContractDraftConfirmation::missing(),
    );

    if result.outcome() == ContractDraftConfirmationOutcome::ConfirmationRequired
        && result.has_blocker(ContractDraftConfirmationBlocker::MissingExplicitConfirmation)
        && result.approved().is_none()
    {
        SmokeEvalCaseResult::pass(
            "eval_ready_draft_without_explicit_confirmation_is_not_approved",
            "ready draft without explicit confirmation stays unapproved",
            "ready draft without explicit user confirmation remains confirmation_required and does not produce approved_for_run model state",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_ready_draft_without_explicit_confirmation_is_not_approved",
            "ready draft without explicit confirmation stays unapproved",
            format!(
                "missing confirmation drifted; outcome={} blockers={} approved_present={}",
                result.outcome().as_str(),
                format_confirmation_blockers(result.blockers()),
                result.approved().is_some()
            ),
        )
    }
}

fn eval_clarification_required_draft_cannot_be_approved() -> SmokeEvalCaseResult {
    let model = ready_user_intent_contract_draft_model()
        .with_unknown(UserIntentUnknown::new("target module is ambiguous", true));
    let result = confirm_contract_draft(&model, &explicit_draft_confirmation());

    if result.outcome() == ContractDraftConfirmationOutcome::ClarificationRequired
        && result.has_blocker(ContractDraftConfirmationBlocker::DraftRequiresClarification)
        && !result.approved_for_run()
    {
        SmokeEvalCaseResult::pass(
            "eval_clarification_required_draft_cannot_be_approved",
            "clarification-required draft cannot approve",
            "drafts that require clarification remain outside approved_for_run model state",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_clarification_required_draft_cannot_be_approved",
            "clarification-required draft cannot approve",
            format!(
                "clarification confirmation drifted; outcome={} blockers={}",
                result.outcome().as_str(),
                format_confirmation_blockers(result.blockers())
            ),
        )
    }
}

fn eval_refused_or_deferred_draft_cannot_be_approved() -> SmokeEvalCaseResult {
    let model = ready_user_intent_contract_draft_model()
        .with_refusal_or_deferral_reason("request would activate runtime storage");
    let result = confirm_contract_draft(&model, &explicit_draft_confirmation());

    if result.outcome() == ContractDraftConfirmationOutcome::RefusedOrDeferred
        && result.has_blocker(ContractDraftConfirmationBlocker::DraftRefusedOrDeferred)
        && !result.approved_for_run()
    {
        SmokeEvalCaseResult::pass(
            "eval_refused_or_deferred_draft_cannot_be_approved",
            "refused or deferred draft cannot approve",
            "refused_or_deferred draft stays outside approved_for_run model state",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_refused_or_deferred_draft_cannot_be_approved",
            "refused or deferred draft cannot approve",
            format!(
                "refused/deferred confirmation drifted; outcome={} blockers={}",
                result.outcome().as_str(),
                format_confirmation_blockers(result.blockers())
            ),
        )
    }
}

fn eval_blocked_draft_cannot_be_approved() -> SmokeEvalCaseResult {
    let model = UserIntentContractDraftModel::new(
        "User asks for bounded work.",
        "Define approval boundary.",
        UserIntentResearchGate::new(UserIntentResearchGateClassification::R1, true),
        UserIntentDownstreamClosure::default(),
    )
    .with_scope_include("crates/punk-contract/src/lib.rs");
    let result = confirm_contract_draft(&model, &explicit_draft_confirmation());

    if result.outcome() == ContractDraftConfirmationOutcome::Blocked
        && result.has_blocker(ContractDraftConfirmationBlocker::DraftBlocked)
        && !result.approved_for_run()
    {
        SmokeEvalCaseResult::pass(
            "eval_blocked_draft_cannot_be_approved",
            "blocked draft cannot approve",
            "blocked draft stays outside approved_for_run model state even when confirmation evidence is present",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_blocked_draft_cannot_be_approved",
            "blocked draft cannot approve",
            format!(
                "blocked confirmation drifted; outcome={} blockers={}",
                result.outcome().as_str(),
                format_confirmation_blockers(result.blockers())
            ),
        )
    }
}

fn eval_unresolved_unknowns_block_approval() -> SmokeEvalCaseResult {
    let model = ready_user_intent_contract_draft_model().with_unknown(UserIntentUnknown::new(
        "deployment target is unresolved",
        false,
    ));
    let result = confirm_contract_draft(&model, &explicit_draft_confirmation());

    if result.outcome() == ContractDraftConfirmationOutcome::Blocked
        && result.has_blocker(ContractDraftConfirmationBlocker::UnhandledUnknown)
        && !result.approved_for_run()
    {
        SmokeEvalCaseResult::pass(
            "eval_unresolved_unknowns_block_approval",
            "unresolved unknowns block approval",
            "non-blocking draft unknowns still require explicit handling before approved_for_run model state",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_unresolved_unknowns_block_approval",
            "unresolved unknowns block approval",
            format!(
                "unknown handling drifted; outcome={} blockers={}",
                result.outcome().as_str(),
                format_confirmation_blockers(result.blockers())
            ),
        )
    }
}

fn eval_unknowns_converted_to_assumptions_allow_approval() -> SmokeEvalCaseResult {
    let unknown = "deployment target is unresolved";
    let model = ready_user_intent_contract_draft_model()
        .with_unknown(UserIntentUnknown::new(unknown, false));
    let confirmation =
        explicit_draft_confirmation().with_unknown_handling(ContractDraftUnknownHandling::new(
            unknown,
            ContractDraftUnknownDisposition::ConvertedToAssumption,
        ));
    let result = confirm_contract_draft(&model, &confirmation);
    let assumption_preserved = result.approved().is_some_and(|approved| {
        approved
            .assumptions()
            .iter()
            .any(|assumption| assumption == unknown)
    });

    if result.approved_for_run() && assumption_preserved {
        SmokeEvalCaseResult::pass(
            "eval_unknowns_converted_to_assumptions_allow_approval",
            "unknowns converted to assumptions can approve",
            "handled unknowns converted to assumptions are preserved in approved_for_run model state",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_unknowns_converted_to_assumptions_allow_approval",
            "unknowns converted to assumptions can approve",
            format!(
                "converted unknown confirmation drifted; outcome={} blockers={} assumption_preserved={assumption_preserved}",
                result.outcome().as_str(),
                format_confirmation_blockers(result.blockers())
            ),
        )
    }
}

fn eval_approved_for_run_preserves_scope_and_non_scope() -> SmokeEvalCaseResult {
    let model = ready_user_intent_contract_draft_model();
    let result = confirm_contract_draft(&model, &explicit_draft_confirmation());
    let preserved = result.approved().is_some_and(|approved| {
        approved.scope_include() == model.scope_include()
            && approved.scope_exclude() == model.scope_exclude()
            && approved.non_goals() == model.non_goals()
    });

    if result.approved_for_run() && preserved {
        SmokeEvalCaseResult::pass(
            "eval_approved_for_run_preserves_scope_and_non_scope",
            "approved_for_run model preserves scope and non-scope",
            "approval preserves include scope, exclude scope, and non-goals without flattening them into prose",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_approved_for_run_preserves_scope_and_non_scope",
            "approved_for_run model preserves scope and non-scope",
            format!(
                "scope preservation drifted; outcome={} preserved={preserved}",
                result.outcome().as_str()
            ),
        )
    }
}

fn eval_approved_for_run_preserves_evidence_plan() -> SmokeEvalCaseResult {
    let model = ready_user_intent_contract_draft_model();
    let result = confirm_contract_draft(&model, &explicit_draft_confirmation());
    let preserved = result
        .approved()
        .is_some_and(|approved| approved.evidence_plan() == model.evidence_plan());

    if result.approved_for_run() && preserved {
        SmokeEvalCaseResult::pass(
            "eval_approved_for_run_preserves_evidence_plan",
            "approved_for_run model preserves evidence plan",
            "approval carries evidence plan forward without converting it into closure authority",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_approved_for_run_preserves_evidence_plan",
            "approved_for_run model preserves evidence plan",
            format!(
                "evidence preservation drifted; outcome={} preserved={preserved}",
                result.outcome().as_str()
            ),
        )
    }
}

fn eval_approved_for_run_preserves_side_effect_boundaries() -> SmokeEvalCaseResult {
    let model = ready_user_intent_contract_draft_model();
    let result = confirm_contract_draft(&model, &explicit_draft_confirmation());
    let preserved = result.approved().is_some_and(|approved| {
        approved.side_effect_boundaries() == model.side_effect_boundaries()
            && approved.downstream_closure() == model.downstream_closure()
    });

    if result.approved_for_run() && preserved {
        SmokeEvalCaseResult::pass(
            "eval_approved_for_run_preserves_side_effect_boundaries",
            "approved_for_run model preserves side-effect boundaries",
            "approval carries side-effect boundaries and downstream closure expectations forward",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_approved_for_run_preserves_side_effect_boundaries",
            "approved_for_run model preserves side-effect boundaries",
            format!(
                "side-effect boundary preservation drifted; outcome={} preserved={preserved}",
                result.outcome().as_str()
            ),
        )
    }
}

fn eval_user_confirmation_does_not_create_gate_decision() -> SmokeEvalCaseResult {
    let result = confirm_contract_draft(
        &ready_user_intent_contract_draft_model(),
        &explicit_draft_confirmation(),
    );
    let boundary = contract_draft_confirmation_boundary();
    let no_gate_writing = result.approved().is_some_and(|approved| {
        !approved.creates_gate_acceptance() && !approved.writes_gate_decision()
    }) && !boundary.user_confirmation_is_gate_acceptance
        && !boundary.writes_gate_decision;

    if result.approved_for_run() && no_gate_writing {
        SmokeEvalCaseResult::pass(
            "eval_user_confirmation_does_not_create_gate_decision",
            "confirmation creates no gate outcome",
            "explicit user confirmation approves only bounded run readiness and leaves gate-writing authority untouched",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_user_confirmation_does_not_create_gate_decision",
            "confirmation creates no gate outcome",
            format!(
                "confirmation gate boundary drifted; outcome={} no_gate_writing={no_gate_writing}",
                result.outcome().as_str()
            ),
        )
    }
}

fn eval_user_confirmation_does_not_create_proofpack() -> SmokeEvalCaseResult {
    let result = confirm_contract_draft(
        &ready_user_intent_contract_draft_model(),
        &explicit_draft_confirmation(),
    );
    let boundary = contract_draft_confirmation_boundary();
    let no_proofpack = result
        .approved()
        .is_some_and(|approved| !approved.creates_proofpack())
        && !boundary.creates_proofpack;

    if result.approved_for_run() && no_proofpack {
        SmokeEvalCaseResult::pass(
            "eval_user_confirmation_does_not_create_proofpack",
            "confirmation creates no proofpack",
            "explicit user confirmation does not create proofpack provenance or proof authority",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_user_confirmation_does_not_create_proofpack",
            "confirmation creates no proofpack",
            format!(
                "confirmation proofpack boundary drifted; outcome={} no_proofpack={no_proofpack}",
                result.outcome().as_str()
            ),
        )
    }
}

fn eval_user_confirmation_does_not_invoke_writer() -> SmokeEvalCaseResult {
    let result = confirm_contract_draft(
        &ready_user_intent_contract_draft_model(),
        &explicit_draft_confirmation(),
    );
    let boundary = contract_draft_confirmation_boundary();
    let writer_downstream = result
        .approved()
        .is_some_and(|approved| !approved.invokes_writer())
        && !boundary.invokes_writer;

    if result.approved_for_run() && writer_downstream {
        SmokeEvalCaseResult::pass(
            "eval_user_confirmation_does_not_invoke_writer",
            "confirmation does not invoke Writer",
            "explicit user confirmation keeps Writer downstream and does not select Writer as an upstream approval mechanism",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_user_confirmation_does_not_invoke_writer",
            "confirmation does not invoke Writer",
            format!(
                "confirmation Writer boundary drifted; outcome={} writer_downstream={writer_downstream}",
                result.outcome().as_str()
            ),
        )
    }
}

fn eval_contract_status_still_excludes_acceptance_outcomes() -> SmokeEvalCaseResult {
    let status_names = CONTRACT_STATUS_VALUES
        .iter()
        .map(|status| status.as_str())
        .collect::<Vec<_>>();
    let lifecycle_ok = status_names == vec!["draft", "approved_for_run", "superseded", "cancelled"];
    let no_closure_outcomes = !status_names.contains(&"accepted")
        && !status_names.contains(&"rejected")
        && !status_names.contains(&"partially_accepted");

    if lifecycle_ok && no_closure_outcomes {
        SmokeEvalCaseResult::pass(
            "eval_contract_status_still_excludes_acceptance_outcomes",
            "ContractStatus stays lifecycle-only after confirmation boundary",
            "lifecycle status names remain draft, approved_for_run, superseded, and cancelled while closure outcome names stay outside ContractStatus",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_contract_status_still_excludes_acceptance_outcomes",
            "ContractStatus stays lifecycle-only after confirmation boundary",
            format!(
                "ContractStatus vocabulary drifted; lifecycle_ok={lifecycle_ok} no_closure_outcomes={no_closure_outcomes} names={status_names:?}"
            ),
        )
    }
}

fn format_confirmation_blockers(blockers: &[ContractDraftConfirmationBlocker]) -> String {
    if blockers.is_empty() {
        return "none".to_owned();
    }

    blockers
        .iter()
        .map(|blocker| blocker.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_user_intent_blockers(blockers: &[UserIntentContractDraftBlocker]) -> String {
    if blockers.is_empty() {
        return "none".to_owned();
    }

    blockers
        .iter()
        .map(|blocker| blocker.as_str())
        .collect::<Vec<_>>()
        .join(",")
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
            ProofArtifactHash::new(PROOF_HASH_RUN_RECEIPT).expect("artifact hash should be valid"),
        ))
        .with_artifact_digest(ProofArtifactDigest::new(
            ProofArtifactKind::Eval,
            ProofArtifactRef::new("eval_smoke_gate_proof").expect("artifact ref should be valid"),
            ProofArtifactHash::new(PROOF_HASH_EVAL).expect("artifact hash should be valid"),
        ))
        .with_artifact_digest(ProofArtifactDigest::new(
            ProofArtifactKind::Event,
            ProofArtifactRef::new("evt_eval_001").expect("artifact ref should be valid"),
            ProofArtifactHash::new(PROOF_HASH_EVENT).expect("artifact hash should be valid"),
        ))
        .with_artifact_digest(ProofArtifactDigest::new(
            ProofArtifactKind::OutputArtifact,
            ProofArtifactRef::new("target/debug/punk").expect("artifact ref should be valid"),
            ProofArtifactHash::new(PROOF_HASH_OUTPUT).expect("artifact hash should be valid"),
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
        ProofArtifactHash::new(PROOF_HASH_GATE_DECISION).expect("artifact hash should be valid"),
    ))
    .with_artifact_digest(ProofArtifactDigest::new(
        ProofArtifactKind::Contract,
        ProofArtifactRef::new("contract_eval_001").expect("artifact ref should be valid"),
        ProofArtifactHash::new(PROOF_HASH_CONTRACT).expect("artifact hash should be valid"),
    ))
}

fn sample_writer_target_artifact_ref(proofpack: &Proofpack) -> ProofpackWriterTargetRef {
    let canonical = ProofpackWriterCanonicalArtifactModel::from_proofpack(proofpack, vec![]);
    let policy = ProofpackWriterTargetArtifactRefPolicyModel::from_canonical_artifact_model(
        &canonical,
        vec![],
    );

    ProofpackWriterTargetRef::from_target_artifact_ref_policy_model(&policy)
        .expect("target artifact ref policy should derive logical ref")
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

fn eval_proofpack_manifest_renderer_is_deterministic_and_side_effect_free() -> SmokeEvalCaseResult {
    let proofpack = sample_proofpack("decision_eval_001");
    let rendered = proofpack.render_manifest_json();
    let rerendered = proofpack.render_manifest_json();
    let boundary = proofpack.boundary();

    let includes_core_fields = rendered.contains("\"proofpack_id\": \"proofpack_eval_001\"")
        && rendered.contains("\"schema_version\": \"punk.proofpack.v0.1\"")
        && rendered.contains("\"gate_decision_ref\": \"decision_eval_001\"")
        && rendered.contains("\"contract_refs\": [\"contract_eval_001\"]")
        && rendered.contains("\"run_receipt_refs\": [\"receipt_eval_001\"]")
        && rendered.contains("\"eval_refs\": [\"eval_smoke_gate_proof\"]")
        && rendered.contains("\"event_refs\": [\"evt_eval_001\"]")
        && rendered.contains("\"output_artifact_refs\": [\"target/debug/punk\"]")
        && rendered.contains("\"artifact_digests\": [")
        && rendered.contains("\"kind\": \"gate_decision\"")
        && rendered.contains(&format!("\"hash\": \"{PROOF_HASH_GATE_DECISION}\""))
        && rendered.contains("\"created_at\": \"2026-04-25T21:01:00Z\"")
        && rendered.contains(
            "\"boundary_notes\": [\"Proofpack references evidence; gate remains the authority.\"]",
        );
    let side_effect_free = !boundary.writes_proofpack
        && !boundary.requires_runtime_storage
        && !boundary.writes_cli_output
        && !boundary.creates_acceptance_claim
        && !boundary.computes_hashes
        && !boundary.normalizes_hashes;

    if rendered == rerendered && includes_core_fields && side_effect_free {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_manifest_renderer_is_deterministic_and_side_effect_free",
            "proofpack manifest renderer is deterministic and side-effect-free",
            "proofpack manifest renderer returns stable in-memory content without writing proofpacks, computing hashes, or activating runtime storage",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_manifest_renderer_is_deterministic_and_side_effect_free",
            "proofpack manifest renderer is deterministic and side-effect-free",
            format!(
                "proofpack manifest renderer drifted; deterministic={} fields={} writes={} storage={} cli={} acceptance={} computes={} normalizes={}",
                rendered == rerendered,
                includes_core_fields,
                boundary.writes_proofpack,
                boundary.requires_runtime_storage,
                boundary.writes_cli_output,
                boundary.creates_acceptance_claim,
                boundary.computes_hashes,
                boundary.normalizes_hashes
            ),
        )
    }
}

fn eval_proofpack_manifest_digest_matches_exact_renderer_bytes() -> SmokeEvalCaseResult {
    let proofpack = sample_proofpack("decision_eval_001");
    let rendered = proofpack.render_manifest_json();
    let digest = compute_proofpack_manifest_digest(&proofpack);
    let expected = compute_artifact_digest(rendered.as_bytes());
    let digest_with_trailing_newline = compute_artifact_digest(format!("{rendered}\n").as_bytes());
    let boundary = proofpack.boundary();
    let partial_proofpack = sample_proofpack_with_partial_integrity("decision_eval_001");
    let before_missing = partial_proofpack
        .link_hash_integrity_report()
        .missing_digest_refs()
        .len();
    let _partial_manifest_digest = compute_proofpack_manifest_digest(&partial_proofpack);
    let after_missing = partial_proofpack
        .link_hash_integrity_report()
        .missing_digest_refs()
        .len();

    let exact_renderer_bytes = digest == expected;
    let canonical = validate_artifact_digest(digest.as_str()).is_ok()
        && is_canonical_artifact_digest(digest.as_str());
    let no_formatting_normalization = digest != digest_with_trailing_newline;
    let no_referenced_artifact_verification = before_missing > 0
        && before_missing == after_missing
        && !partial_proofpack.has_complete_link_hash_integrity();
    let boundary_ok = boundary.computes_manifest_digest
        && !boundary.computes_referenced_artifact_hashes
        && !boundary.computes_hashes
        && !boundary.normalizes_hashes
        && !boundary.writes_proofpack
        && !boundary.requires_runtime_storage
        && !boundary.writes_cli_output
        && !boundary.creates_acceptance_claim
        && !boundary.writes_final_decision;

    if exact_renderer_bytes
        && canonical
        && no_formatting_normalization
        && no_referenced_artifact_verification
        && boundary_ok
    {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_manifest_digest_matches_exact_renderer_bytes",
            "proofpack manifest digest hashes exact renderer bytes",
            "manifest digest matches exact in-memory renderer bytes, stays canonical, and does not verify referenced artifacts or write runtime state",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_manifest_digest_matches_exact_renderer_bytes",
            "proofpack manifest digest hashes exact renderer bytes",
            format!(
                "proofpack manifest digest drifted; exact={} canonical={} no_normalization={} no_ref_verification={} boundary_ok={} writes={} storage={} cli={} acceptance={} computes_manifest={} computes_referenced={}",
                exact_renderer_bytes,
                canonical,
                no_formatting_normalization,
                no_referenced_artifact_verification,
                boundary_ok,
                boundary.writes_proofpack,
                boundary.requires_runtime_storage,
                boundary.writes_cli_output,
                boundary.creates_acceptance_claim,
                boundary.computes_manifest_digest,
                boundary.computes_referenced_artifact_hashes
            ),
        )
    }
}

fn valid_artifact_digest() -> String {
    format!("sha256:{}", "0123456789abcdef".repeat(4))
}

fn eval_proofpack_writer_canonical_artifact_model_is_side_effect_free() -> SmokeEvalCaseResult {
    let proofpack = sample_proofpack("decision_eval_001");
    let rendered = proofpack.render_manifest_json();
    let expected_digest = compute_proofpack_manifest_digest(&proofpack);
    let model = ProofpackWriterCanonicalArtifactModel::from_proofpack(
        &proofpack,
        vec![ProofBoundaryNote::new(
            "Canonical artifact model is manifest-only and side-effect-free.",
        )
        .expect("boundary note should be valid")],
    );
    let boundary = model.boundary();

    let vocabulary_ok = ProofpackWriterCanonicalArtifactLayout::ManifestOnlyJson.as_str()
        == "manifest_only_json"
        && ProofpackWriterNonCanonicalArtifactSurface::ManifestSelfDigestMetadata.as_str()
            == "manifest_self_digest_metadata"
        && ProofpackWriterNonCanonicalArtifactSurface::WrapperMetadata.as_str()
            == "wrapper_metadata"
        && ProofpackWriterNonCanonicalArtifactSurface::IndexView.as_str() == "index_view"
        && ProofpackWriterNonCanonicalArtifactSurface::LatestPointer.as_str() == "latest_pointer";
    let canonical_bytes_ok = model.schema_version()
        == PROOFPACK_WRITER_CANONICAL_ARTIFACT_MODEL_SCHEMA_VERSION
        && model.proofpack_id().as_str() == "proofpack_eval_001"
        && model.is_manifest_only_layout()
        && model.canonical_body_utf8() == rendered
        && model.canonical_body_bytes() == rendered.as_bytes()
        && model.canonical_body_matches_proofpack(&proofpack);
    let digest_ok = model.manifest_self_digest() == &expected_digest
        && model.manifest_self_digest() == &compute_artifact_digest(model.canonical_body_bytes())
        && model.manifest_self_digest_covers_canonical_body()
        && !model.manifest_self_digest_is_embedded_in_canonical_body();
    let metadata_ok = model.surface_is_non_canonical(
        ProofpackWriterNonCanonicalArtifactSurface::ManifestSelfDigestMetadata,
    ) && model
        .surface_is_non_canonical(ProofpackWriterNonCanonicalArtifactSurface::WrapperMetadata)
        && model
            .surface_is_non_canonical(ProofpackWriterNonCanonicalArtifactSurface::StorageRootRef)
        && model.surface_is_non_canonical(
            ProofpackWriterNonCanonicalArtifactSurface::TargetArtifactRef,
        )
        && model
            .surface_is_non_canonical(ProofpackWriterNonCanonicalArtifactSurface::TargetPathRef)
        && model.surface_is_non_canonical(
            ProofpackWriterNonCanonicalArtifactSurface::WriterOperationEvidence,
        )
        && model.surface_is_non_canonical(
            ProofpackWriterNonCanonicalArtifactSurface::SchemaValidationReport,
        )
        && model.surface_is_non_canonical(ProofpackWriterNonCanonicalArtifactSurface::IndexView)
        && model
            .surface_is_non_canonical(ProofpackWriterNonCanonicalArtifactSurface::LatestPointer)
        && model.surface_is_non_canonical(ProofpackWriterNonCanonicalArtifactSurface::CliOutput)
        && model
            .surface_is_non_canonical(ProofpackWriterNonCanonicalArtifactSurface::ServiceMirror);
    let boundary_ok = boundary.models_canonical_artifact_body
        && boundary.canonical_body_is_manifest_renderer_bytes
        && boundary.manifest_self_digest_covers_canonical_body
        && boundary.manifest_self_digest_metadata_outside_body
        && boundary.separates_non_canonical_metadata
        && boundary.evidence_only
        && !boundary.reads_filesystem
        && !boundary.touches_filesystem
        && !boundary.canonicalizes_host_paths
        && !boundary.writes_proofpack
        && !boundary.writes_writer_operation_evidence
        && !boundary.writes_final_decision
        && !boundary.creates_acceptance_claim
        && !boundary.requires_runtime_storage
        && !boundary.writes_cli_output
        && !boundary.writes_schema_files
        && !boundary.verifies_referenced_artifacts
        && !boundary.uses_indexes_or_latest_as_authority
        && !model.reads_filesystem()
        && !model.touches_filesystem()
        && !model.writes_proofpack()
        && !model.writes_writer_operation_evidence()
        && !model.verifies_referenced_artifacts()
        && !model.requires_runtime_storage()
        && !model.writes_cli_output()
        && !model.writes_schema_files()
        && !model.creates_acceptance_claim()
        && !model.uses_indexes_or_latest_as_authority()
        && !model.can_claim_acceptance_by_itself();

    if vocabulary_ok && canonical_bytes_ok && digest_ok && metadata_ok && boundary_ok {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_writer_canonical_artifact_model_is_side_effect_free",
            "proofpack writer canonical artifact model is side-effect-free",
            "writer canonical artifact model keeps exact manifest renderer bytes as canonical, exposes manifest self-digest coverage, and keeps wrapper/index/latest/CLI/service metadata non-canonical without runtime writes",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_writer_canonical_artifact_model_is_side_effect_free",
            "proofpack writer canonical artifact model is side-effect-free",
            format!(
                "writer canonical artifact model drifted; vocabulary={vocabulary_ok} canonical_bytes={canonical_bytes_ok} digest={digest_ok} metadata={metadata_ok} boundary={boundary_ok} reads_fs={} touches_fs={} writes={} writes_evidence={} storage={} cli={} schemas={} verifies={} acceptance={} index_latest={}",
                boundary.reads_filesystem,
                boundary.touches_filesystem,
                boundary.writes_proofpack,
                boundary.writes_writer_operation_evidence,
                boundary.requires_runtime_storage,
                boundary.writes_cli_output,
                boundary.writes_schema_files,
                boundary.verifies_referenced_artifacts,
                boundary.creates_acceptance_claim,
                boundary.uses_indexes_or_latest_as_authority
            ),
        )
    }
}

fn eval_proofpack_writer_target_artifact_ref_policy_model_is_side_effect_free(
) -> SmokeEvalCaseResult {
    let proofpack = sample_proofpack("decision_eval_001");
    let canonical = ProofpackWriterCanonicalArtifactModel::from_proofpack(&proofpack, vec![]);
    let model = ProofpackWriterTargetArtifactRefPolicyModel::from_canonical_artifact_model(
        &canonical,
        vec![
            ProofBoundaryNote::new("Target artifact ref policy renders logical refs only.")
                .expect("boundary note should be valid"),
        ],
    );
    let missing_proofpack_id = ProofpackWriterTargetArtifactRefPolicyModel::evaluate(
        None,
        Some(canonical.manifest_self_digest().clone()),
        ProofpackWriterCanonicalArtifactLayout::ManifestOnlyJson,
        vec![],
    );
    let missing_digest = ProofpackWriterTargetArtifactRefPolicyModel::evaluate(
        Some(ProofpackId::new("proofpack_eval_001").expect("id should be valid")),
        None,
        ProofpackWriterCanonicalArtifactLayout::ManifestOnlyJson,
        vec![],
    );
    let invalid_digest = ProofpackWriterTargetArtifactRefPolicyModel::evaluate_raw(
        Some("proofpack_eval_001"),
        Some("sha256:not-valid"),
        ProofpackWriterCanonicalArtifactLayout::ManifestOnlyJson,
        vec![],
    );
    let boundary = model.boundary();
    let expected_ref = format!(
        "proofpack:{}@{}",
        canonical.proofpack_id().as_str(),
        canonical.manifest_self_digest().as_str()
    );

    let vocabulary_ok = ProofpackWriterTargetArtifactRefPolicyStatus::Accepted.as_str()
        == "accepted"
        && ProofpackWriterTargetArtifactRefPolicyStatus::Rejected.as_str() == "rejected"
        && ProofpackWriterTargetArtifactRefPolicyReason::MissingProofpackId.as_str()
            == "missing_proofpack_id"
        && ProofpackWriterTargetArtifactRefPolicyReason::MissingManifestSelfDigest.as_str()
            == "missing_manifest_self_digest"
        && ProofpackWriterTargetArtifactRefPolicyReason::InvalidManifestSelfDigest.as_str()
            == "invalid_manifest_self_digest";
    let accepted_ok = model.schema_version()
        == PROOFPACK_WRITER_TARGET_ARTIFACT_REF_POLICY_MODEL_SCHEMA_VERSION
        && model.is_accepted()
        && model.status() == ProofpackWriterTargetArtifactRefPolicyStatus::Accepted
        && model.has_complete_identity()
        && model.reasons().is_empty()
        && model.proofpack_id().map(ProofpackId::as_str) == Some("proofpack_eval_001")
        && model.manifest_self_digest() == Some(canonical.manifest_self_digest())
        && model.layout() == Some(ProofpackWriterCanonicalArtifactLayout::ManifestOnlyJson)
        && model.logical_display_ref() == Some(expected_ref.as_str())
        && !model.display_ref_is_filesystem_path();
    let rejection_ok = missing_proofpack_id.is_rejected()
        && missing_proofpack_id.has_missing_precondition()
        && missing_proofpack_id
            .has_reason(ProofpackWriterTargetArtifactRefPolicyReason::MissingProofpackId)
        && missing_proofpack_id.logical_ref().is_none()
        && missing_digest.is_rejected()
        && missing_digest
            .has_reason(ProofpackWriterTargetArtifactRefPolicyReason::MissingManifestSelfDigest)
        && missing_digest.logical_display_ref().is_none()
        && invalid_digest.is_rejected()
        && invalid_digest
            .has_reason(ProofpackWriterTargetArtifactRefPolicyReason::InvalidManifestSelfDigest)
        && invalid_digest.manifest_self_digest().is_none()
        && invalid_digest.logical_ref().is_none();
    let boundary_ok = boundary.models_target_artifact_ref_policy
        && boundary.requires_proofpack_id_and_manifest_self_digest
        && boundary.renders_logical_display_ref
        && boundary.keeps_target_artifact_ref_separate_from_canonical_bytes
        && boundary.keeps_target_artifact_ref_separate_from_target_path
        && boundary.keeps_target_artifact_ref_separate_from_storage_root
        && boundary.evidence_only
        && !boundary.display_ref_is_filesystem_path
        && !boundary.reads_filesystem
        && !boundary.touches_filesystem
        && !boundary.canonicalizes_host_paths
        && !boundary.writes_proofpack
        && !boundary.writes_writer_operation_evidence
        && !boundary.writes_indexes_or_latest
        && !boundary.writes_final_decision
        && !boundary.creates_acceptance_claim
        && !boundary.requires_runtime_storage
        && !boundary.writes_cli_output
        && !boundary.writes_schema_files
        && !boundary.verifies_referenced_artifacts
        && !boundary.uses_indexes_or_latest_as_authority
        && !boundary.uses_service_mirror_as_authority
        && !boundary.executor_claims_are_proof
        && !model.can_claim_acceptance_by_itself();

    if vocabulary_ok && accepted_ok && rejection_ok && boundary_ok {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_writer_target_artifact_ref_policy_model_is_side_effect_free",
            "proofpack writer target artifact ref policy model is side-effect-free",
            "writer target artifact ref policy requires proofpack id plus manifest self-digest, renders a logical non-path ref, and rejects missing or invalid identity parts without runtime writes",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_writer_target_artifact_ref_policy_model_is_side_effect_free",
            "proofpack writer target artifact ref policy model is side-effect-free",
            format!(
                "writer target artifact ref policy drifted; vocabulary={vocabulary_ok} accepted={accepted_ok} rejection={rejection_ok} boundary={boundary_ok} reads_fs={} touches_fs={} canonicalizes={} display_path={} writes={} writes_evidence={} writes_index_latest={} storage={} cli={} schemas={} verifies={} acceptance={} index_latest_authority={} service_mirror_authority={} executor_claims={}",
                boundary.reads_filesystem,
                boundary.touches_filesystem,
                boundary.canonicalizes_host_paths,
                boundary.display_ref_is_filesystem_path,
                boundary.writes_proofpack,
                boundary.writes_writer_operation_evidence,
                boundary.writes_indexes_or_latest,
                boundary.requires_runtime_storage,
                boundary.writes_cli_output,
                boundary.writes_schema_files,
                boundary.verifies_referenced_artifacts,
                boundary.creates_acceptance_claim,
                boundary.uses_indexes_or_latest_as_authority,
                boundary.uses_service_mirror_as_authority,
                boundary.executor_claims_are_proof,
            ),
        )
    }
}

fn eval_proofpack_writer_operation_evidence_model_is_side_effect_free() -> SmokeEvalCaseResult {
    let proofpack = sample_proofpack("decision_eval_001");
    let evidence = ProofpackWriterOperationEvidence::new(
        ProofpackWriterOperationId::new("writer_op_smoke_001")
            .expect("operation id should be valid"),
        ProofpackWriterOperationKind::Write,
        proofpack.id().clone(),
        ProofpackWriterAttemptedAt::new("2026-04-26T13:10:00Z")
            .expect("attempted_at should be valid"),
        sample_writer_target_artifact_ref(&proofpack),
        ProofpackWriterOperationOutcome::IndexUpdateFailed,
        ProofpackWriterCanonicalArtifactStatus::Written,
        ProofpackWriterSideEffectStatus::Failed,
        ProofpackWriterSideEffectStatus::Completed,
        vec![ProofBoundaryNote::new(
            "Operation evidence is evidence-only; gate remains the authority.",
        )
        .expect("boundary note should be valid")],
    )
    .expect("operation evidence should be consistent");
    let boundary = evidence.boundary();

    if evidence.canonical_artifact_available()
        && evidence.has_index_or_latest_pointer_failure()
        && evidence.is_evidence_only()
        && !evidence.is_final_decision_authority()
        && !evidence.creates_acceptance_claim()
        && !evidence.can_claim_acceptance_by_itself()
        && !boundary.writes_proofpack
        && !boundary.writes_writer_operation_evidence
        && !boundary.requires_runtime_storage
        && !boundary.writes_cli_output
        && !boundary.writes_schema_files
        && boundary.separates_canonical_artifact_from_indexes
    {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_writer_operation_evidence_model_is_side_effect_free",
            "proofpack writer operation evidence model is side-effect-free",
            "writer operation evidence models canonical artifact status separately from index/latest side effects without writing proofpacks, decisions, schemas, CLI output, or acceptance claims",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_writer_operation_evidence_model_is_side_effect_free",
            "proofpack writer operation evidence model is side-effect-free",
            format!(
                "writer operation evidence drifted; available={} side_effect_failure={} evidence_only={} decision={} acceptance={} writes={} writes_evidence={} storage={} cli={} schemas={} separates={}",
                evidence.canonical_artifact_available(),
                evidence.has_index_or_latest_pointer_failure(),
                evidence.is_evidence_only(),
                evidence.is_final_decision_authority(),
                evidence.creates_acceptance_claim(),
                boundary.writes_proofpack,
                boundary.writes_writer_operation_evidence,
                boundary.requires_runtime_storage,
                boundary.writes_cli_output,
                boundary.writes_schema_files,
                boundary.separates_canonical_artifact_from_indexes,
            ),
        )
    }
}

fn eval_proofpack_writer_preflight_plan_model_is_side_effect_free() -> SmokeEvalCaseResult {
    let proofpack = sample_proofpack("decision_eval_001");
    let plan = ProofpackWriterPreflightPlan::new(
        &proofpack,
        sample_writer_target_artifact_ref(&proofpack),
        vec![
            ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite,
            ProofpackWriterPlannedSideEffect::IndexUpdate,
        ],
        vec![ProofBoundaryNote::new(
            "Preflight plan is evidence-only and does not attempt writes.",
        )
        .expect("boundary note should be valid")],
    );
    let evidence = plan
        .to_operation_evidence(
            ProofpackWriterOperationId::new("writer_preflight_smoke_001")
                .expect("operation id should be valid"),
            ProofpackWriterAttemptedAt::new("2026-04-26T14:40:00Z")
                .expect("attempted_at should be valid"),
        )
        .expect("planned-only operation evidence should be derivable");
    let boundary = plan.boundary();

    let partial_proofpack = sample_proofpack_with_partial_integrity("decision_eval_001");
    let missing_plan = ProofpackWriterPreflightPlan::new(
        &partial_proofpack,
        sample_writer_target_artifact_ref(&partial_proofpack),
        vec![],
        vec![],
    );
    let missing_preconditions = missing_plan.missing_preconditions();
    let missing_required_digests = missing_preconditions
        .contains(&ProofpackWriterMissingPrecondition::MissingRequiredArtifactDigests);
    let missing_planned_side_effects = missing_preconditions
        .contains(&ProofpackWriterMissingPrecondition::MissingPlannedSideEffects);
    let missing_boundary_notes =
        missing_preconditions.contains(&ProofpackWriterMissingPrecondition::MissingBoundaryNotes);

    let ready_plan_ok = plan.is_writer_ready()
        && plan.status().as_str() == "ready"
        && plan.schema_version() == "punk.proofpack.writer_preflight_plan.v0.1"
        && plan.target_ref().is_aligned_target_artifact_ref()
        && plan
            .target_ref()
            .as_str()
            .starts_with("proofpack:proofpack_eval_001@sha256:")
        && plan.manifest_self_digest() == &compute_proofpack_manifest_digest(&proofpack)
        && plan.plans_side_effect(ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite)
        && plan.plans_side_effect(ProofpackWriterPlannedSideEffect::IndexUpdate)
        && !plan.plans_side_effect(ProofpackWriterPlannedSideEffect::LatestPointerUpdate)
        && plan.missing_preconditions().is_empty();
    let evidence_ok = evidence.outcome() == ProofpackWriterOperationOutcome::PlannedOnly
        && evidence.canonical_artifact_status()
            == ProofpackWriterCanonicalArtifactStatus::NotAttempted
        && !evidence.canonical_artifact_available()
        && evidence.index_status() == ProofpackWriterSideEffectStatus::NotAttempted
        && evidence.latest_pointer_status() == ProofpackWriterSideEffectStatus::NotSelected
        && !evidence.creates_acceptance_claim();
    let missing_plan_ok = !missing_plan.is_writer_ready()
        && missing_plan.operation_outcome() == ProofpackWriterOperationOutcome::PreflightFailed
        && missing_required_digests
        && missing_planned_side_effects
        && missing_boundary_notes;
    let boundary_ok = boundary.models_writer_preflight_plan
        && boundary.computes_manifest_self_digest
        && boundary.evidence_only
        && boundary.separates_preflight_from_artifact_availability
        && !boundary.planned_side_effects_are_attempts
        && !boundary.writes_proofpack
        && !boundary.writes_writer_operation_evidence
        && !boundary.writes_final_decision
        && !boundary.creates_acceptance_claim
        && !boundary.requires_runtime_storage
        && !boundary.writes_cli_output
        && !boundary.writes_schema_files;

    if ready_plan_ok && evidence_ok && missing_plan_ok && boundary_ok {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_writer_preflight_plan_model_is_side_effect_free",
            "proofpack writer preflight plan model is side-effect-free",
            "writer preflight planning exposes target, manifest self-digest, planned side effects, and missing preconditions without writing proofpacks, operation evidence, schemas, CLI output, or acceptance claims",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_writer_preflight_plan_model_is_side_effect_free",
            "proofpack writer preflight plan model is side-effect-free",
            format!(
                "writer preflight plan drifted; ready_plan={ready_plan_ok} evidence={evidence_ok} missing_plan={missing_plan_ok} boundary={boundary_ok} writes={} writes_evidence={} storage={} cli={} schemas={} acceptance={} side_effects_attempted={}",
                boundary.writes_proofpack,
                boundary.writes_writer_operation_evidence,
                boundary.requires_runtime_storage,
                boundary.writes_cli_output,
                boundary.writes_schema_files,
                boundary.creates_acceptance_claim,
                boundary.planned_side_effects_are_attempts,
            ),
        )
    }
}

fn eval_proofpack_writer_file_io_plan_model_is_side_effect_free() -> SmokeEvalCaseResult {
    let proofpack = sample_proofpack("decision_eval_001");
    let preflight_plan = ProofpackWriterPreflightPlan::new(
        &proofpack,
        sample_writer_target_artifact_ref(&proofpack),
        vec![
            ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite,
            ProofpackWriterPlannedSideEffect::IndexUpdate,
            ProofpackWriterPlannedSideEffect::LatestPointerUpdate,
        ],
        vec![ProofBoundaryNote::new(
            "Preflight plan is evidence-only and does not attempt writes.",
        )
        .expect("boundary note should be valid")],
    );
    let plan = ProofpackWriterFileIoPlan::new(
        &preflight_plan,
        ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
            .expect("storage root ref should be valid"),
        ProofpackWriterTargetPathRef::new("future/.punk/proofs/proofpack_eval_001.json")
            .expect("target path ref should be valid"),
        ProofpackWriterWritePolicy::IdempotentIfMatching,
        ProofpackWriterIdempotencyBasis::ManifestSelfDigest,
        ProofpackWriterTempAtomicPolicy::AtomicSiblingTemp,
        vec![
            ProofpackWriterFileIoFailureVisibility::StorageRootMissing,
            ProofpackWriterFileIoFailureVisibility::TargetPathInvalid,
            ProofpackWriterFileIoFailureVisibility::ExistingTargetDifferent,
            ProofpackWriterFileIoFailureVisibility::AtomicMoveFailed,
            ProofpackWriterFileIoFailureVisibility::CleanupFailed,
            ProofpackWriterFileIoFailureVisibility::IndexUpdateFailed,
            ProofpackWriterFileIoFailureVisibility::LatestPointerUpdateFailed,
        ],
        vec![ProofBoundaryNote::new(
            "File IO plan models explicit targets and policies without touching the filesystem.",
        )
        .expect("boundary note should be valid")],
    );
    let evidence = plan
        .to_operation_evidence(
            ProofpackWriterOperationId::new("writer_file_io_plan_smoke_001")
                .expect("operation id should be valid"),
            ProofpackWriterAttemptedAt::new("2026-04-26T15:10:00Z")
                .expect("attempted_at should be valid"),
        )
        .expect("planned-only operation evidence should be derivable");
    let boundary = plan.boundary();

    let partial_proofpack = sample_proofpack_with_partial_integrity("decision_eval_001");
    let blocked_preflight_plan = ProofpackWriterPreflightPlan::new(
        &partial_proofpack,
        sample_writer_target_artifact_ref(&partial_proofpack),
        vec![],
        vec![],
    );
    let blocked_plan = ProofpackWriterFileIoPlan::new(
        &blocked_preflight_plan,
        ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
            .expect("storage root ref should be valid"),
        ProofpackWriterTargetPathRef::new("future/.punk/proofs/proofpack_eval_002.json")
            .expect("target path ref should be valid"),
        ProofpackWriterWritePolicy::IdempotentIfMatching,
        ProofpackWriterIdempotencyBasis::ManifestSelfDigest,
        ProofpackWriterTempAtomicPolicy::AtomicSiblingTemp,
        vec![],
        vec![],
    );

    let ready_plan_ok = plan.is_file_io_ready()
        && plan.status().as_str() == "ready"
        && plan.schema_version() == "punk.proofpack.writer_file_io_plan.v0.1"
        && plan.storage_root_ref().as_str() == "repo_runtime_proofs_root"
        && plan.target_artifact_ref().is_aligned_target_artifact_ref()
        && !plan.target_artifact_ref().is_path_like_ref()
        && plan.target_path_ref().as_str() == "future/.punk/proofs/proofpack_eval_001.json"
        && plan.target_artifact_ref().as_str() != plan.target_path_ref().as_str()
        && plan.manifest_self_digest() == preflight_plan.manifest_self_digest()
        && plan.write_policy() == ProofpackWriterWritePolicy::IdempotentIfMatching
        && plan.write_policy().supports_idempotency()
        && !plan.write_policy().allows_silent_overwrite()
        && plan.idempotency_basis() == ProofpackWriterIdempotencyBasis::ManifestSelfDigest
        && plan.temp_atomic_policy() == ProofpackWriterTempAtomicPolicy::AtomicSiblingTemp
        && plan.temp_atomic_policy().prefers_atomic_move()
        && plan.selects_canonical_artifact_write()
        && plan.selects_index_update()
        && plan.selects_latest_pointer_update()
        && plan.tracks_conflict_visibility()
        && plan.tracks_rollback_visibility();
    let evidence_ok = evidence.outcome() == ProofpackWriterOperationOutcome::PlannedOnly
        && evidence.canonical_artifact_status()
            == ProofpackWriterCanonicalArtifactStatus::NotAttempted
        && !evidence.canonical_artifact_available()
        && evidence.index_status() == ProofpackWriterSideEffectStatus::NotAttempted
        && evidence.latest_pointer_status() == ProofpackWriterSideEffectStatus::NotAttempted
        && !evidence.creates_acceptance_claim();
    let blocked_plan_ok = !blocked_plan.is_file_io_ready()
        && blocked_plan.operation_outcome() == ProofpackWriterOperationOutcome::PreflightFailed
        && blocked_plan
            .blockers()
            .contains(&ProofpackWriterFileIoBlocker::PreflightPlanMissingPreconditions)
        && blocked_plan
            .blockers()
            .contains(&ProofpackWriterFileIoBlocker::MissingCanonicalArtifactWriteSelection)
        && blocked_plan
            .blockers()
            .contains(&ProofpackWriterFileIoBlocker::MissingErrorRollbackVisibility)
        && blocked_plan
            .blockers()
            .contains(&ProofpackWriterFileIoBlocker::MissingBoundaryNotes);
    let boundary_ok = boundary.models_writer_file_io_plan
        && boundary.models_explicit_storage_root
        && boundary.models_target_artifact_ref
        && boundary.models_target_path_ref
        && boundary.models_write_policy
        && boundary.models_idempotency_basis
        && boundary.models_temp_atomic_policy
        && boundary.models_error_rollback_visibility
        && boundary.evidence_only
        && boundary.separates_file_io_plan_from_artifact_availability
        && !boundary.touches_filesystem
        && !boundary.writes_proofpack
        && !boundary.writes_writer_operation_evidence
        && !boundary.writes_final_decision
        && !boundary.creates_acceptance_claim
        && !boundary.requires_runtime_storage
        && !boundary.writes_cli_output
        && !boundary.writes_schema_files
        && !boundary.target_path_is_authority
        && !boundary.index_latest_are_canonical;

    if ready_plan_ok && evidence_ok && blocked_plan_ok && boundary_ok {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_writer_file_io_plan_model_is_side_effect_free",
            "proofpack writer file IO plan model is side-effect-free",
            "writer file IO planning exposes storage root refs, target refs and paths, write/idempotency/temp policies, side-effect selection, and rollback visibility without touching the filesystem or claiming artifact availability",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_writer_file_io_plan_model_is_side_effect_free",
            "proofpack writer file IO plan model is side-effect-free",
            format!(
                "writer file IO plan drifted; ready_plan={ready_plan_ok} evidence={evidence_ok} blocked_plan={blocked_plan_ok} boundary={boundary_ok} touches_fs={} writes={} storage={} cli={} schemas={} acceptance={} target_path_authority={} index_latest_canonical={}",
                boundary.touches_filesystem,
                boundary.writes_proofpack,
                boundary.requires_runtime_storage,
                boundary.writes_cli_output,
                boundary.writes_schema_files,
                boundary.creates_acceptance_claim,
                boundary.target_path_is_authority,
                boundary.index_latest_are_canonical,
            ),
        )
    }
}

fn eval_proofpack_writer_file_io_outcome_model_is_side_effect_free() -> SmokeEvalCaseResult {
    let proofpack = sample_proofpack("decision_eval_001");
    let preflight_plan = ProofpackWriterPreflightPlan::new(
        &proofpack,
        sample_writer_target_artifact_ref(&proofpack),
        vec![
            ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite,
            ProofpackWriterPlannedSideEffect::IndexUpdate,
            ProofpackWriterPlannedSideEffect::LatestPointerUpdate,
        ],
        vec![ProofBoundaryNote::new(
            "Preflight plan is evidence-only and does not attempt writes.",
        )
        .expect("boundary note should be valid")],
    );
    let plan = ProofpackWriterFileIoPlan::new(
        &preflight_plan,
        ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
            .expect("storage root ref should be valid"),
        ProofpackWriterTargetPathRef::new("future/.punk/proofs/proofpack_eval_001.json")
            .expect("target path ref should be valid"),
        ProofpackWriterWritePolicy::IdempotentIfMatching,
        ProofpackWriterIdempotencyBasis::ManifestSelfDigest,
        ProofpackWriterTempAtomicPolicy::AtomicSiblingTemp,
        vec![
            ProofpackWriterFileIoFailureVisibility::ExistingTargetMatching,
            ProofpackWriterFileIoFailureVisibility::ExistingTargetDifferent,
            ProofpackWriterFileIoFailureVisibility::AtomicMoveFailed,
            ProofpackWriterFileIoFailureVisibility::CleanupFailed,
            ProofpackWriterFileIoFailureVisibility::PartialCanonicalArtifactAmbiguous,
            ProofpackWriterFileIoFailureVisibility::IndexUpdateFailed,
            ProofpackWriterFileIoFailureVisibility::LatestPointerUpdateFailed,
        ],
        vec![ProofBoundaryNote::new(
            "File IO plan models explicit targets and policies without touching the filesystem.",
        )
        .expect("boundary note should be valid")],
    );
    let written = ProofpackWriterFileIoOutcomeModel::from_plan_and_observation(
        &plan,
        ProofpackWriterFileIoObservation::target_missing_write_completed(
            ProofpackWriterSideEffectStatus::Completed,
            ProofpackWriterSideEffectStatus::Completed,
            vec![ProofBoundaryNote::new(
                "Explicit caller observation reports completed write and pointers.",
            )
            .expect("boundary note should be valid")],
        ),
    );
    let index_failed = ProofpackWriterFileIoOutcomeModel::from_plan_and_observation(
        &plan,
        ProofpackWriterFileIoObservation::index_failed_after_available(vec![
            ProofBoundaryNote::new("Index update failed after canonical artifact availability.")
                .expect("boundary note should be valid"),
        ]),
    );
    let latest_failed = ProofpackWriterFileIoOutcomeModel::from_plan_and_observation(
        &plan,
        ProofpackWriterFileIoObservation::latest_failed_after_available(vec![
            ProofBoundaryNote::new(
                "Latest pointer update failed after canonical artifact availability.",
            )
            .expect("boundary note should be valid"),
        ]),
    );
    let conflict = ProofpackWriterFileIoOutcomeModel::from_plan_and_observation(
        &plan,
        ProofpackWriterFileIoObservation::target_exists_different(vec![ProofBoundaryNote::new(
            "Existing target differed from the planned manifest.",
        )
        .expect("boundary note should be valid")]),
    );
    let partial = ProofpackWriterFileIoOutcomeModel::from_plan_and_observation(
        &plan,
        ProofpackWriterFileIoObservation::new(
            ProofpackWriterObservedTargetState::AmbiguousPartial,
            ProofpackWriterIdempotencyObservation::NotChecked,
            ProofpackWriterSideEffectStatus::Failed,
            ProofpackWriterObservedWriteResult::PartialWriteDetected,
            ProofpackWriterObservedPartialState::CleanupIncomplete,
            ProofpackWriterSideEffectStatus::Failed,
            ProofpackWriterSideEffectStatus::NotAttempted,
            ProofpackWriterSideEffectStatus::NotAttempted,
            ProofpackWriterAbortState::AbortedAfterPartial,
            vec![
                ProofBoundaryNote::new("Partial write and cleanup failure remain visible.")
                    .expect("boundary note should be valid"),
            ],
        ),
    );
    let aborted = ProofpackWriterFileIoOutcomeModel::from_plan_and_observation(
        &plan,
        ProofpackWriterFileIoObservation::aborted(vec![ProofBoundaryNote::new(
            "Operation was aborted before canonical artifact write.",
        )
        .expect("boundary note should be valid")]),
    );
    let evidence = index_failed
        .to_operation_evidence(
            ProofpackWriterOperationId::new("writer_file_io_outcome_smoke_001")
                .expect("operation id should be valid"),
            ProofpackWriterAttemptedAt::new("2026-04-26T15:40:00Z")
                .expect("attempted_at should be valid"),
        )
        .expect("operation evidence should be derivable from outcome model");
    let boundary = written.boundary();

    let written_ok = written.outcome() == ProofpackWriterOperationOutcome::Written
        && written.canonical_artifact_status() == ProofpackWriterCanonicalArtifactStatus::Written
        && written.canonical_artifact_available()
        && written.observation().target_state() == ProofpackWriterObservedTargetState::Missing
        && written.observation().idempotency_observation()
            == ProofpackWriterIdempotencyObservation::NotApplicable
        && written.observation().write_result() == ProofpackWriterObservedWriteResult::Written
        && !written.has_index_or_latest_pointer_failure();
    let failure_mapping_ok = index_failed.outcome()
        == ProofpackWriterOperationOutcome::IndexUpdateFailed
        && index_failed.operation_kind() == ProofpackWriterOperationKind::IndexUpdate
        && latest_failed.outcome() == ProofpackWriterOperationOutcome::LatestPointerUpdateFailed
        && latest_failed.operation_kind() == ProofpackWriterOperationKind::LatestPointerUpdate
        && conflict.outcome() == ProofpackWriterOperationOutcome::ConflictExistingDifferent
        && conflict.has_conflict()
        && partial.outcome() == ProofpackWriterOperationOutcome::PartialWriteDetected
        && partial.has_partial_or_cleanup_issue()
        && aborted.outcome() == ProofpackWriterOperationOutcome::Aborted
        && aborted.operation_kind() == ProofpackWriterOperationKind::Abort;
    let evidence_ok = evidence.outcome() == ProofpackWriterOperationOutcome::IndexUpdateFailed
        && evidence.canonical_artifact_status() == ProofpackWriterCanonicalArtifactStatus::Written
        && evidence.index_status() == ProofpackWriterSideEffectStatus::Failed
        && evidence.latest_pointer_status() == ProofpackWriterSideEffectStatus::Completed
        && evidence.canonical_artifact_available()
        && !evidence.creates_acceptance_claim();
    let boundary_ok = boundary.models_writer_file_io_outcome
        && boundary.accepts_explicit_observations
        && boundary.maps_observations_to_operation_evidence
        && boundary.evidence_only
        && boundary.separates_observation_from_artifact_availability
        && boundary.preserves_partial_cleanup_visibility
        && !boundary.reads_filesystem
        && !boundary.touches_filesystem
        && !boundary.writes_proofpack
        && !boundary.writes_writer_operation_evidence
        && !boundary.writes_final_decision
        && !boundary.creates_acceptance_claim
        && !boundary.requires_runtime_storage
        && !boundary.writes_cli_output
        && !boundary.writes_schema_files
        && !boundary.target_path_is_authority
        && !boundary.index_latest_are_canonical;

    if written_ok && failure_mapping_ok && evidence_ok && boundary_ok {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_writer_file_io_outcome_model_is_side_effect_free",
            "proofpack writer file IO outcome model is side-effect-free",
            "writer file IO outcome model maps explicit target, idempotency, temp/write, partial/cleanup, index/latest, and abort observations into operation evidence without touching filesystem, runtime storage, schemas, CLI output, or acceptance claims",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_writer_file_io_outcome_model_is_side_effect_free",
            "proofpack writer file IO outcome model is side-effect-free",
            format!(
                "writer file IO outcome model drifted; written={written_ok} failures={failure_mapping_ok} evidence={evidence_ok} boundary={boundary_ok} reads_fs={} touches_fs={} writes={} writes_evidence={} storage={} cli={} schemas={} acceptance={} target_path_authority={} index_latest_canonical={}",
                boundary.reads_filesystem,
                boundary.touches_filesystem,
                boundary.writes_proofpack,
                boundary.writes_writer_operation_evidence,
                boundary.requires_runtime_storage,
                boundary.writes_cli_output,
                boundary.writes_schema_files,
                boundary.creates_acceptance_claim,
                boundary.target_path_is_authority,
                boundary.index_latest_are_canonical,
            ),
        )
    }
}

fn eval_proofpack_writer_file_io_error_reason_model_is_side_effect_free() -> SmokeEvalCaseResult {
    let proofpack = sample_proofpack("decision_eval_001");
    let preflight_plan = ProofpackWriterPreflightPlan::new(
        &proofpack,
        sample_writer_target_artifact_ref(&proofpack),
        vec![
            ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite,
            ProofpackWriterPlannedSideEffect::IndexUpdate,
            ProofpackWriterPlannedSideEffect::LatestPointerUpdate,
        ],
        vec![ProofBoundaryNote::new(
            "Preflight plan is evidence-only and does not attempt writes.",
        )
        .expect("boundary note should be valid")],
    );
    let plan = ProofpackWriterFileIoPlan::new(
        &preflight_plan,
        ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
            .expect("storage root ref should be valid"),
        ProofpackWriterTargetPathRef::new("future/.punk/proofs/proofpack_eval_001.json")
            .expect("target path ref should be valid"),
        ProofpackWriterWritePolicy::IdempotentIfMatching,
        ProofpackWriterIdempotencyBasis::ManifestSelfDigest,
        ProofpackWriterTempAtomicPolicy::AtomicSiblingTemp,
        vec![
            ProofpackWriterFileIoFailureVisibility::TargetPathInvalid,
            ProofpackWriterFileIoFailureVisibility::ExistingTargetDifferent,
            ProofpackWriterFileIoFailureVisibility::TempWriteFailed,
            ProofpackWriterFileIoFailureVisibility::FlushOrSyncFailed,
            ProofpackWriterFileIoFailureVisibility::AtomicMoveFailed,
            ProofpackWriterFileIoFailureVisibility::CleanupFailed,
            ProofpackWriterFileIoFailureVisibility::IndexUpdateFailed,
            ProofpackWriterFileIoFailureVisibility::LatestPointerUpdateFailed,
        ],
        vec![ProofBoundaryNote::new(
            "File IO plan models explicit targets and policies without touching the filesystem.",
        )
        .expect("boundary note should be valid")],
    );
    let outcome = ProofpackWriterFileIoOutcomeModel::from_plan_and_observation(
        &plan,
        ProofpackWriterFileIoObservation::partial_write_detected(
            ProofpackWriterSideEffectStatus::Failed,
            vec![ProofBoundaryNote::new(
                "Partial canonical artifact and cleanup failure remain visible.",
            )
            .expect("boundary note should be valid")],
        ),
    );
    let temp_failure = ProofpackWriterFileIoDiagnostic::for_reason(
        ProofpackWriterFileIoErrorReason::TempWriteFailed,
    )
    .with_target_path_ref(
        ProofpackWriterTargetPathRef::new("future/.punk/proofs/proofpack_eval_001.json")
            .expect("target path ref should be valid"),
    )
    .with_diagnostic_path_ref(
        ProofpackWriterDiagnosticPathRef::new(
            "/diagnostic-only/not-canonical/proofpack_eval_001.tmp",
        )
        .expect("diagnostic path ref should be valid"),
    );
    let executor_claim = ProofpackWriterFileIoDiagnostic::for_reason(
        ProofpackWriterFileIoErrorReason::ExistingTargetMatching,
    )
    .with_source(ProofpackWriterFileIoDiagnosticSource::ExecutorClaim);
    let model = ProofpackWriterFileIoErrorReasonModel::from_outcome_and_diagnostics(
        &outcome,
        vec![
            temp_failure,
            ProofpackWriterFileIoDiagnostic::for_reason(
                ProofpackWriterFileIoErrorReason::CleanupFailed,
            ),
            ProofpackWriterFileIoDiagnostic::for_reason(
                ProofpackWriterFileIoErrorReason::IndexUpdateFailed,
            ),
            ProofpackWriterFileIoDiagnostic::for_reason(
                ProofpackWriterFileIoErrorReason::LatestPointerUpdateFailed,
            ),
            ProofpackWriterFileIoDiagnostic::for_reason(
                ProofpackWriterFileIoErrorReason::OperationEvidencePersistenceFailed,
            ),
            ProofpackWriterFileIoDiagnostic::for_reason(
                ProofpackWriterFileIoErrorReason::OperationAborted,
            ),
            executor_claim,
        ],
        vec![ProofBoundaryNote::new(
            "File IO diagnostics are evidence-only and keep paths diagnostic-only.",
        )
        .expect("boundary note should be valid")],
    );
    let boundary = model.boundary();

    let vocabulary_ok = ProofpackWriterFileIoErrorReason::StorageRootMissing.as_str()
        == "storage_root_missing"
        && ProofpackWriterFileIoErrorReason::TargetPathEscapesStorageRoot.as_str()
            == "target_path_escapes_storage_root"
        && ProofpackWriterFileIoErrorReason::OperationEvidencePersistenceFailed.as_str()
            == "operation_evidence_persistence_failed"
        && ProofpackWriterFileIoErrorReason::OperationAborted.as_str() == "operation_aborted";
    let model_ok = model.schema_version()
        == "punk.proofpack.writer_file_io_error_reason_model.v0.1"
        && model.outcome() == ProofpackWriterOperationOutcome::PartialWriteDetected
        && model.canonical_artifact_status()
            == ProofpackWriterCanonicalArtifactStatus::PartialWriteDetected
        && model.has_reason(ProofpackWriterFileIoErrorReason::TempWriteFailed)
        && model.has_write_reason()
        && model.has_partial_or_cleanup_reason()
        && model.has_index_or_latest_reason()
        && model.has_operation_evidence_persistence_reason()
        && model.has_abort_reason()
        && model.has_diagnostic_path_refs()
        && model.has_executor_claims()
        && !model.target_path_is_authority()
        && !model.diagnostic_paths_are_authority()
        && !model.index_latest_are_canonical()
        && !model.executor_claims_are_proof()
        && !model.can_claim_acceptance_by_itself();
    let boundary_ok = boundary.models_writer_file_io_error_reasons
        && boundary.stable_reason_codes
        && boundary.attaches_to_file_io_outcomes
        && boundary.evidence_only
        && !boundary.reads_filesystem
        && !boundary.touches_filesystem
        && !boundary.writes_proofpack
        && !boundary.writes_writer_operation_evidence
        && !boundary.writes_final_decision
        && !boundary.creates_acceptance_claim
        && !boundary.requires_runtime_storage
        && !boundary.writes_cli_output
        && !boundary.writes_schema_files
        && !boundary.target_path_is_authority
        && !boundary.diagnostic_paths_are_authority
        && !boundary.index_latest_are_canonical
        && !boundary.executor_claims_are_proof;

    if vocabulary_ok && model_ok && boundary_ok {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_writer_file_io_error_reason_model_is_side_effect_free",
            "proofpack writer file IO error reason model is side-effect-free",
            "writer file IO error reason model keeps storage, target path, write, partial/cleanup, index/latest, operation-evidence persistence, abort, diagnostic path, and executor-claim reasons explicit without filesystem, storage, schemas, CLI output, or acceptance claims",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_writer_file_io_error_reason_model_is_side_effect_free",
            "proofpack writer file IO error reason model is side-effect-free",
            format!(
                "writer file IO error reason model drifted; vocabulary={vocabulary_ok} model={model_ok} boundary={boundary_ok} reads_fs={} touches_fs={} writes={} writes_evidence={} storage={} cli={} schemas={} acceptance={} target_path_authority={} diagnostic_path_authority={} index_latest_canonical={} executor_claims_proof={}",
                boundary.reads_filesystem,
                boundary.touches_filesystem,
                boundary.writes_proofpack,
                boundary.writes_writer_operation_evidence,
                boundary.requires_runtime_storage,
                boundary.writes_cli_output,
                boundary.writes_schema_files,
                boundary.creates_acceptance_claim,
                boundary.target_path_is_authority,
                boundary.diagnostic_paths_are_authority,
                boundary.index_latest_are_canonical,
                boundary.executor_claims_are_proof,
            ),
        )
    }
}

fn eval_proofpack_writer_target_path_policy_model_is_side_effect_free() -> SmokeEvalCaseResult {
    let proofpack = sample_proofpack("decision_eval_001");
    let preflight_plan = ProofpackWriterPreflightPlan::new(
        &proofpack,
        sample_writer_target_artifact_ref(&proofpack),
        vec![
            ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite,
            ProofpackWriterPlannedSideEffect::IndexUpdate,
        ],
        vec![ProofBoundaryNote::new(
            "Preflight plan is evidence-only and does not attempt writes.",
        )
        .expect("boundary note should be valid")],
    );
    let plan = ProofpackWriterFileIoPlan::new(
        &preflight_plan,
        ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
            .expect("storage root ref should be valid"),
        ProofpackWriterTargetPathRef::new("future/.punk/proofs/proofpack_eval_001.json")
            .expect("target path ref should be valid"),
        ProofpackWriterWritePolicy::IdempotentIfMatching,
        ProofpackWriterIdempotencyBasis::ManifestSelfDigest,
        ProofpackWriterTempAtomicPolicy::AtomicSiblingTemp,
        vec![
            ProofpackWriterFileIoFailureVisibility::TargetPathInvalid,
            ProofpackWriterFileIoFailureVisibility::ExistingTargetDifferent,
            ProofpackWriterFileIoFailureVisibility::AtomicMoveFailed,
        ],
        vec![ProofBoundaryNote::new(
            "File IO plan models explicit target refs without touching the filesystem.",
        )
        .expect("boundary note should be valid")],
    );
    let accepted = ProofpackWriterTargetPathPolicyModel::from_plan(
        &plan,
        vec![
            ProofBoundaryNote::new("Target path policy is evidence-only and setup-neutral.")
                .expect("boundary note should be valid"),
        ],
    );
    let traversal = ProofpackWriterTargetPathPolicyModel::evaluate(
        ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
            .expect("storage root ref should be valid"),
        sample_writer_target_artifact_ref(&proofpack),
        ProofpackWriterTargetPathRef::new("future/.punk/../proofs/proofpack_eval_001.json")
            .expect("target path ref should be valid"),
        vec![],
    );
    let absolute = ProofpackWriterTargetPathPolicyModel::evaluate(
        ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
            .expect("storage root ref should be valid"),
        sample_writer_target_artifact_ref(&proofpack),
        ProofpackWriterTargetPathRef::new("/tmp/proofpack_eval_001.json")
            .expect("target path ref should be valid"),
        vec![],
    );
    let backslash = ProofpackWriterTargetPathPolicyModel::evaluate(
        ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
            .expect("storage root ref should be valid"),
        sample_writer_target_artifact_ref(&proofpack),
        ProofpackWriterTargetPathRef::new("future\\.punk\\proofpack_eval_001.json")
            .expect("target path ref should be valid"),
        vec![],
    );
    let boundary = accepted.boundary();

    let vocabulary_ok = ProofpackWriterTargetPathPolicyStatus::Accepted.as_str() == "accepted"
        && ProofpackWriterTargetPathPolicyStatus::Rejected.as_str() == "rejected"
        && ProofpackWriterTargetPathPolicyReason::PathTraversal.as_str() == "path_traversal"
        && ProofpackWriterTargetPathPolicyReason::StorageRootEscape.as_str()
            == "storage_root_escape"
        && ProofpackWriterTargetPathPolicyReason::StorageRootEscape.file_io_error_reason()
            == ProofpackWriterFileIoErrorReason::TargetPathEscapesStorageRoot;
    let accepted_ok = accepted.schema_version()
        == "punk.proofpack.writer_target_path_policy_model.v0.1"
        && accepted.is_accepted()
        && accepted.status() == ProofpackWriterTargetPathPolicyStatus::Accepted
        && accepted.reasons().is_empty()
        && accepted.diagnostics().is_empty()
        && accepted.storage_root_ref().as_str() == "repo_runtime_proofs_root"
        && accepted.target_ref().is_aligned_target_artifact_ref()
        && !accepted.target_ref().is_path_like_ref()
        && accepted.target_path_ref().as_str() == "future/.punk/proofs/proofpack_eval_001.json"
        && accepted.target_ref().as_str() != accepted.target_path_ref().as_str()
        && !accepted.target_path_is_authority()
        && !accepted.storage_root_ref_is_authority()
        && !accepted.derives_from_current_working_directory()
        && !accepted.uses_indexes_or_latest_as_authority();
    let rejected_ok = traversal.is_rejected()
        && traversal.has_reason(ProofpackWriterTargetPathPolicyReason::PathTraversal)
        && traversal.has_reason(ProofpackWriterTargetPathPolicyReason::StorageRootEscape)
        && traversal.has_storage_root_escape()
        && traversal
            .diagnostic_reason_codes()
            .contains(&"target_path_escapes_storage_root")
        && absolute.has_reason(ProofpackWriterTargetPathPolicyReason::AbsolutePath)
        && absolute.has_storage_root_escape()
        && backslash.has_reason(ProofpackWriterTargetPathPolicyReason::UnsupportedBackslash)
        && backslash.diagnostics()[0].reason()
            == ProofpackWriterFileIoErrorReason::TargetPathInvalid
        && backslash
            .diagnostics()
            .iter()
            .all(|diagnostic| !diagnostic.target_path_is_authority());
    let boundary_ok = boundary.models_target_path_policy
        && boundary.classifies_explicit_target_path_refs
        && boundary.keeps_storage_root_target_ref_and_path_separate
        && boundary.maps_to_file_io_diagnostics
        && boundary.evidence_only
        && !boundary.reads_filesystem
        && !boundary.touches_filesystem
        && !boundary.canonicalizes_host_paths
        && !boundary.writes_proofpack
        && !boundary.writes_writer_operation_evidence
        && !boundary.writes_final_decision
        && !boundary.creates_acceptance_claim
        && !boundary.requires_runtime_storage
        && !boundary.writes_cli_output
        && !boundary.writes_schema_files
        && !boundary.target_path_is_authority
        && !boundary.storage_root_ref_is_authority
        && !boundary.derives_from_current_working_directory
        && !boundary.uses_indexes_or_latest_as_authority;

    if vocabulary_ok && accepted_ok && rejected_ok && boundary_ok {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_writer_target_path_policy_model_is_side_effect_free",
            "proofpack writer target path policy model is side-effect-free",
            "writer target path policy accepts explicit repo-runtime-style refs and rejects traversal, absolute, and backslash refs without reading, canonicalizing, or writing filesystem paths",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_writer_target_path_policy_model_is_side_effect_free",
            "proofpack writer target path policy model is side-effect-free",
            format!(
                "writer target path policy drifted; vocabulary={vocabulary_ok} accepted={accepted_ok} rejected={rejected_ok} boundary={boundary_ok} reads_fs={} touches_fs={} canonicalizes={} writes={} writes_evidence={} storage={} cli={} schemas={} acceptance={} target_path_authority={} storage_root_authority={} cwd={} index_latest={}",
                boundary.reads_filesystem,
                boundary.touches_filesystem,
                boundary.canonicalizes_host_paths,
                boundary.writes_proofpack,
                boundary.writes_writer_operation_evidence,
                boundary.requires_runtime_storage,
                boundary.writes_cli_output,
                boundary.writes_schema_files,
                boundary.creates_acceptance_claim,
                boundary.target_path_is_authority,
                boundary.storage_root_ref_is_authority,
                boundary.derives_from_current_working_directory,
                boundary.uses_indexes_or_latest_as_authority,
            ),
        )
    }
}

fn eval_proofpack_writer_preflight_integration_model_is_side_effect_free() -> SmokeEvalCaseResult {
    let proofpack = sample_proofpack("decision_eval_001");
    let canonical = ProofpackWriterCanonicalArtifactModel::from_proofpack(&proofpack, vec![]);
    let target_ref_policy =
        ProofpackWriterTargetArtifactRefPolicyModel::from_canonical_artifact_model(
            &canonical,
            vec![],
        );
    let preflight_plan = ProofpackWriterPreflightPlan::new(
        &proofpack,
        ProofpackWriterTargetRef::from_target_artifact_ref_policy_model(&target_ref_policy)
            .expect("target artifact ref policy should derive logical ref"),
        vec![ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite],
        vec![
            ProofBoundaryNote::new("Preflight integration smoke uses explicit preflight inputs.")
                .expect("boundary note should be valid"),
        ],
    );
    let file_io_plan = ProofpackWriterFileIoPlan::new(
        &preflight_plan,
        ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
            .expect("storage root ref should be valid"),
        ProofpackWriterTargetPathRef::new("future/.punk/proofs/proofpack_eval_001.json")
            .expect("target path ref should be valid"),
        ProofpackWriterWritePolicy::IdempotentIfMatching,
        ProofpackWriterIdempotencyBasis::ManifestSelfDigest,
        ProofpackWriterTempAtomicPolicy::AtomicSiblingTemp,
        vec![
            ProofpackWriterFileIoFailureVisibility::ExistingTargetMatching,
            ProofpackWriterFileIoFailureVisibility::ExistingTargetDifferent,
            ProofpackWriterFileIoFailureVisibility::AtomicMoveFailed,
            ProofpackWriterFileIoFailureVisibility::CleanupFailed,
        ],
        vec![ProofBoundaryNote::new(
            "File IO policy inputs are modeled without filesystem access.",
        )
        .expect("boundary note should be valid")],
    );
    let target_path_policy = ProofpackWriterTargetPathPolicyModel::from_plan(
        &file_io_plan,
        vec![
            ProofBoundaryNote::new("Target path policy is explicit and evidence-only.")
                .expect("boundary note should be valid"),
        ],
    );
    let ready = ProofpackWriterPreflightIntegrationModel::evaluate(
        &proofpack,
        Some(&canonical),
        Some(&target_ref_policy),
        Some(&preflight_plan),
        Some(&file_io_plan),
        Some(&target_path_policy),
        vec![
            ProofBoundaryNote::new("Integrated preflight composes explicit model inputs only.")
                .expect("boundary note should be valid"),
        ],
    );

    let partial_proofpack = sample_proofpack_with_partial_integrity("decision_eval_001");
    let blocked_canonical =
        ProofpackWriterCanonicalArtifactModel::from_proofpack(&partial_proofpack, vec![]);
    let blocked_ref_policy =
        ProofpackWriterTargetArtifactRefPolicyModel::from_canonical_artifact_model(
            &blocked_canonical,
            vec![],
        );
    let blocked_preflight = ProofpackWriterPreflightPlan::new(
        &partial_proofpack,
        ProofpackWriterTargetRef::from_target_artifact_ref_policy_model(&blocked_ref_policy)
            .expect("target artifact ref policy should derive logical ref"),
        vec![],
        vec![],
    );
    let blocked_file_io = ProofpackWriterFileIoPlan::new(
        &blocked_preflight,
        ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
            .expect("storage root ref should be valid"),
        ProofpackWriterTargetPathRef::new("/tmp/proofpack_eval_001.json")
            .expect("target path ref should be valid"),
        ProofpackWriterWritePolicy::IdempotentIfMatching,
        ProofpackWriterIdempotencyBasis::ManifestSelfDigest,
        ProofpackWriterTempAtomicPolicy::AtomicSiblingTemp,
        vec![],
        vec![],
    );
    let blocked_path_policy =
        ProofpackWriterTargetPathPolicyModel::from_plan(&blocked_file_io, vec![]);
    let blocked = ProofpackWriterPreflightIntegrationModel::evaluate(
        &partial_proofpack,
        Some(&blocked_canonical),
        Some(&blocked_ref_policy),
        Some(&blocked_preflight),
        Some(&blocked_file_io),
        Some(&blocked_path_policy),
        vec![],
    );
    let not_selected = ProofpackWriterPreflightIntegrationModel::not_selected(
        &proofpack,
        Some(&canonical),
        Some(&target_ref_policy),
        vec![
            ProofBoundaryNote::new("No writer/storage behavior selected in this smoke case.")
                .expect("boundary note should be valid"),
        ],
    );
    let boundary = ready.boundary();

    let vocabulary_ok = ProofpackWriterPreflightIntegrationStatus::Ready.as_str() == "ready"
        && ProofpackWriterPreflightIntegrationStatus::Blocked.as_str() == "blocked"
        && ProofpackWriterPreflightIntegrationStatus::NotSelected.as_str() == "not_selected"
        && ProofpackWriterPreflightIntegrationBlocker::MissingCanonicalArtifactModel.as_str()
            == "missing_canonical_artifact_model"
        && ProofpackWriterPreflightIntegrationBlocker::PreflightPlanMissingPreconditions.as_str()
            == "preflight_plan_missing_preconditions"
        && ProofpackWriterPreflightIntegrationBlocker::RejectedTargetPathPolicy.as_str()
            == "rejected_target_path_policy";
    let ready_ok = ready.schema_version()
        == PROOFPACK_WRITER_PREFLIGHT_INTEGRATION_MODEL_SCHEMA_VERSION
        && ready.status() == ProofpackWriterPreflightIntegrationStatus::Ready
        && ready.writer_selected()
        && ready.is_writer_ready()
        && !ready.has_blockers()
        && ready.proofpack_id().as_str() == "proofpack_eval_001"
        && ready.manifest_self_digest() == Some(canonical.manifest_self_digest())
        && ready.target_artifact_ref() == Some(file_io_plan.target_artifact_ref())
        && ready.storage_root_ref() == Some(file_io_plan.storage_root_ref())
        && ready.target_path_ref() == Some(file_io_plan.target_path_ref())
        && ready.target_path_policy_status()
            == Some(ProofpackWriterTargetPathPolicyStatus::Accepted)
        && ready.write_policy() == Some(ProofpackWriterWritePolicy::IdempotentIfMatching)
        && ready.idempotency_basis() == Some(ProofpackWriterIdempotencyBasis::ManifestSelfDigest)
        && ready.temp_atomic_policy() == Some(ProofpackWriterTempAtomicPolicy::AtomicSiblingTemp)
        && ready.refs_are_separated()
        && ready.operation_outcome() == ProofpackWriterOperationOutcome::PlannedOnly;
    let blocked_ok = blocked.status() == ProofpackWriterPreflightIntegrationStatus::Blocked
        && blocked.is_blocked()
        && blocked.writer_selected()
        && blocked.has_blocker(
            ProofpackWriterPreflightIntegrationBlocker::PreflightPlanMissingPreconditions,
        )
        && blocked.has_blocker(ProofpackWriterPreflightIntegrationBlocker::FileIoPlanBlocked)
        && blocked
            .has_blocker(ProofpackWriterPreflightIntegrationBlocker::RejectedTargetPathPolicy)
        && blocked.has_blocker(ProofpackWriterPreflightIntegrationBlocker::MissingBoundaryNotes)
        && blocked.operation_outcome() == ProofpackWriterOperationOutcome::PreflightFailed
        && blocked.diagnostics().iter().any(|diagnostic| {
            diagnostic.reason() == ProofpackWriterFileIoErrorReason::TargetPathEscapesStorageRoot
        });
    let not_selected_ok = not_selected.status()
        == ProofpackWriterPreflightIntegrationStatus::NotSelected
        && not_selected.is_not_selected()
        && !not_selected.writer_selected()
        && !not_selected.has_blockers()
        && not_selected.operation_outcome() == ProofpackWriterOperationOutcome::PlannedOnly;
    let boundary_ok = boundary.models_preflight_integration
        && boundary.composes_explicit_model_inputs
        && boundary.keeps_storage_root_target_artifact_and_path_separate
        && boundary.blockers_fail_closed
        && boundary.statuses_are_evidence_only
        && boundary.evidence_only
        && boundary.setup_neutral
        && !boundary.reads_filesystem
        && !boundary.touches_filesystem
        && !boundary.canonicalizes_host_paths
        && !boundary.writes_proofpack
        && !boundary.writes_punk_proofs
        && !boundary.writes_writer_operation_evidence
        && !boundary.persists_operation_evidence
        && !boundary.writes_indexes_or_latest
        && !boundary.writes_final_decision
        && !boundary.creates_acceptance_claim
        && !boundary.requires_runtime_storage
        && !boundary.writes_cli_output
        && !boundary.writes_schema_files
        && !boundary.verifies_referenced_artifacts
        && !boundary.uses_indexes_or_latest_as_authority
        && !boundary.uses_service_mirror_as_authority
        && !boundary.executor_claims_are_proof
        && !boundary.target_path_is_authority
        && !boundary.storage_root_ref_is_authority
        && !ready.reads_filesystem()
        && !ready.touches_filesystem()
        && !ready.writes_proofpack()
        && !ready.writes_punk_proofs()
        && !ready.writes_writer_operation_evidence()
        && !ready.persists_operation_evidence()
        && !ready.writes_indexes_or_latest()
        && !ready.requires_runtime_storage()
        && !ready.writes_cli_output()
        && !ready.writes_schema_files()
        && !ready.verifies_referenced_artifacts()
        && !ready.creates_acceptance_claim()
        && !ready.can_claim_acceptance_by_itself()
        && !ready.target_path_is_authority()
        && !ready.storage_root_ref_is_authority()
        && !ready.executor_claims_are_proof();

    if vocabulary_ok && ready_ok && blocked_ok && not_selected_ok && boundary_ok {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_writer_preflight_integration_model_is_side_effect_free",
            "proofpack writer preflight integration model is side-effect-free",
            "writer preflight integration composes explicit canonical artifact, target ref policy, preflight, file IO, and target path policy inputs with ready, blocked, and not-selected evidence-only outcomes",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_writer_preflight_integration_model_is_side_effect_free",
            "proofpack writer preflight integration model is side-effect-free",
            format!(
                "writer preflight integration drifted; vocabulary={vocabulary_ok} ready={ready_ok} blocked={blocked_ok} not_selected={not_selected_ok} boundary={boundary_ok} reads_fs={} touches_fs={} writes={} punk_proofs={} writes_evidence={} persists_evidence={} index_latest={} storage={} cli={} schemas={} verifies={} acceptance={} service_mirror={} executor_claims={}",
                boundary.reads_filesystem,
                boundary.touches_filesystem,
                boundary.writes_proofpack,
                boundary.writes_punk_proofs,
                boundary.writes_writer_operation_evidence,
                boundary.persists_operation_evidence,
                boundary.writes_indexes_or_latest,
                boundary.requires_runtime_storage,
                boundary.writes_cli_output,
                boundary.writes_schema_files,
                boundary.verifies_referenced_artifacts,
                boundary.creates_acceptance_claim,
                boundary.uses_service_mirror_as_authority,
                boundary.executor_claims_are_proof,
            ),
        )
    }
}

fn eval_proofpack_writer_active_behavior_model_is_side_effect_free() -> SmokeEvalCaseResult {
    let proofpack = sample_proofpack("decision_eval_001");
    let canonical = ProofpackWriterCanonicalArtifactModel::from_proofpack(&proofpack, vec![]);
    let target_ref_policy =
        ProofpackWriterTargetArtifactRefPolicyModel::from_canonical_artifact_model(
            &canonical,
            vec![],
        );
    let preflight_plan = ProofpackWriterPreflightPlan::new(
        &proofpack,
        ProofpackWriterTargetRef::from_target_artifact_ref_policy_model(&target_ref_policy)
            .expect("target artifact ref policy should derive logical ref"),
        vec![
            ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite,
            ProofpackWriterPlannedSideEffect::IndexUpdate,
            ProofpackWriterPlannedSideEffect::LatestPointerUpdate,
        ],
        vec![
            ProofBoundaryNote::new("Active behavior smoke uses explicit preflight inputs only.")
                .expect("boundary note should be valid"),
        ],
    );
    let file_io_plan = ProofpackWriterFileIoPlan::new(
        &preflight_plan,
        ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
            .expect("storage root ref should be valid"),
        ProofpackWriterTargetPathRef::new("future/.punk/proofs/proofpack_eval_001.json")
            .expect("target path ref should be valid"),
        ProofpackWriterWritePolicy::IdempotentIfMatching,
        ProofpackWriterIdempotencyBasis::ManifestSelfDigest,
        ProofpackWriterTempAtomicPolicy::AtomicSiblingTemp,
        vec![
            ProofpackWriterFileIoFailureVisibility::ExistingTargetMatching,
            ProofpackWriterFileIoFailureVisibility::ExistingTargetDifferent,
            ProofpackWriterFileIoFailureVisibility::AtomicMoveFailed,
            ProofpackWriterFileIoFailureVisibility::CleanupFailed,
            ProofpackWriterFileIoFailureVisibility::PartialCanonicalArtifactAmbiguous,
            ProofpackWriterFileIoFailureVisibility::IndexUpdateFailed,
            ProofpackWriterFileIoFailureVisibility::LatestPointerUpdateFailed,
        ],
        vec![ProofBoundaryNote::new(
            "File IO plan remains model input and does not touch the filesystem.",
        )
        .expect("boundary note should be valid")],
    );
    let target_path_policy = ProofpackWriterTargetPathPolicyModel::from_plan(
        &file_io_plan,
        vec![
            ProofBoundaryNote::new("Target path policy stays explicit and evidence-only.")
                .expect("boundary note should be valid"),
        ],
    );
    let ready = ProofpackWriterPreflightIntegrationModel::evaluate(
        &proofpack,
        Some(&canonical),
        Some(&target_ref_policy),
        Some(&preflight_plan),
        Some(&file_io_plan),
        Some(&target_path_policy),
        vec![ProofBoundaryNote::new(
            "Ready preflight is required before active behavior can model write observations.",
        )
        .expect("boundary note should be valid")],
    );

    let planned = ProofpackWriterActiveBehaviorModel::planned_only(
        &ready,
        vec![ProofBoundaryNote::new(
            "Planned-only active behavior records selected side effects without attempting them.",
        )
        .expect("boundary note should be valid")],
    );
    let idempotent = ProofpackWriterActiveBehaviorModel::from_preflight_and_observation(
        &ready,
        Some(ProofpackWriterFileIoObservation::target_exists_matching(
            vec![ProofBoundaryNote::new(
                "Existing canonical artifact matched the planned manifest.",
            )
            .expect("boundary note should be valid")],
        )),
        ProofpackWriterSideEffectStatus::NotAttempted,
        vec![],
    );
    let conflict = ProofpackWriterActiveBehaviorModel::from_preflight_and_observation(
        &ready,
        Some(ProofpackWriterFileIoObservation::target_exists_different(
            vec![ProofBoundaryNote::new(
                "Existing canonical artifact differed from the planned manifest.",
            )
            .expect("boundary note should be valid")],
        )),
        ProofpackWriterSideEffectStatus::NotAttempted,
        vec![],
    );
    let write_failed = ProofpackWriterActiveBehaviorModel::from_preflight_and_observation(
        &ready,
        Some(ProofpackWriterFileIoObservation::write_failed(vec![
            ProofBoundaryNote::new("Write failure remains explicit model data.")
                .expect("boundary note should be valid"),
        ])),
        ProofpackWriterSideEffectStatus::Failed,
        vec![],
    );
    let partial = ProofpackWriterActiveBehaviorModel::from_preflight_and_observation(
        &ready,
        Some(ProofpackWriterFileIoObservation::partial_write_detected(
            ProofpackWriterSideEffectStatus::Failed,
            vec![
                ProofBoundaryNote::new("Partial write and cleanup failure remain visible.")
                    .expect("boundary note should be valid"),
            ],
        )),
        ProofpackWriterSideEffectStatus::Failed,
        vec![],
    );
    let index_failed = ProofpackWriterActiveBehaviorModel::from_preflight_and_observation(
        &ready,
        Some(
            ProofpackWriterFileIoObservation::index_failed_after_available(vec![
                ProofBoundaryNote::new(
                    "Index update failed after canonical artifact availability.",
                )
                .expect("boundary note should be valid"),
            ]),
        ),
        ProofpackWriterSideEffectStatus::Completed,
        vec![],
    );
    let latest_failed = ProofpackWriterActiveBehaviorModel::from_preflight_and_observation(
        &ready,
        Some(
            ProofpackWriterFileIoObservation::latest_failed_after_available(vec![
                ProofBoundaryNote::new(
                    "Latest pointer update failed after canonical artifact availability.",
                )
                .expect("boundary note should be valid"),
            ]),
        ),
        ProofpackWriterSideEffectStatus::Completed,
        vec![],
    );
    let evidence_persistence_failed =
        ProofpackWriterActiveBehaviorModel::from_preflight_and_observation(
            &ready,
            Some(
                ProofpackWriterFileIoObservation::target_missing_write_completed(
                    ProofpackWriterSideEffectStatus::Completed,
                    ProofpackWriterSideEffectStatus::Completed,
                    vec![ProofBoundaryNote::new(
                    "Operation evidence persistence failure is modeled as non-authoritative data.",
                )
                .expect("boundary note should be valid")],
                ),
            ),
            ProofpackWriterSideEffectStatus::Failed,
            vec![],
        );

    let partial_proofpack = sample_proofpack_with_partial_integrity("decision_eval_001");
    let blocked_canonical =
        ProofpackWriterCanonicalArtifactModel::from_proofpack(&partial_proofpack, vec![]);
    let blocked_ref_policy =
        ProofpackWriterTargetArtifactRefPolicyModel::from_canonical_artifact_model(
            &blocked_canonical,
            vec![],
        );
    let blocked_preflight = ProofpackWriterPreflightPlan::new(
        &partial_proofpack,
        ProofpackWriterTargetRef::from_target_artifact_ref_policy_model(&blocked_ref_policy)
            .expect("target artifact ref policy should derive logical ref"),
        vec![],
        vec![],
    );
    let blocked_file_io = ProofpackWriterFileIoPlan::new(
        &blocked_preflight,
        ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
            .expect("storage root ref should be valid"),
        ProofpackWriterTargetPathRef::new("/tmp/proofpack_eval_001.json")
            .expect("target path ref should be valid"),
        ProofpackWriterWritePolicy::IdempotentIfMatching,
        ProofpackWriterIdempotencyBasis::ManifestSelfDigest,
        ProofpackWriterTempAtomicPolicy::AtomicSiblingTemp,
        vec![],
        vec![],
    );
    let blocked_path_policy =
        ProofpackWriterTargetPathPolicyModel::from_plan(&blocked_file_io, vec![]);
    let blocked_preflight_model = ProofpackWriterPreflightIntegrationModel::evaluate(
        &partial_proofpack,
        Some(&blocked_canonical),
        Some(&blocked_ref_policy),
        Some(&blocked_preflight),
        Some(&blocked_file_io),
        Some(&blocked_path_policy),
        vec![],
    );
    let blocked = ProofpackWriterActiveBehaviorModel::from_preflight_and_observation(
        &blocked_preflight_model,
        Some(
            ProofpackWriterFileIoObservation::target_missing_write_completed(
                ProofpackWriterSideEffectStatus::Completed,
                ProofpackWriterSideEffectStatus::Completed,
                vec![ProofBoundaryNote::new(
                    "Blocked preflight ignores caller-provided write observations.",
                )
                .expect("boundary note should be valid")],
            ),
        ),
        ProofpackWriterSideEffectStatus::Failed,
        vec![],
    );
    let evidence = index_failed
        .to_operation_evidence(
            ProofpackWriterOperationId::new("writer_active_behavior_smoke_001")
                .expect("operation id should be valid"),
            ProofpackWriterAttemptedAt::new("2026-04-27T06:30:00Z")
                .expect("attempted_at should be valid"),
        )
        .expect("operation evidence should be derivable from active behavior model");
    let boundary = planned.boundary();

    let vocabulary_ok = PROOFPACK_WRITER_ACTIVE_BEHAVIOR_MODEL_SCHEMA_VERSION
        == "punk.proofpack.writer_active_behavior_model.v0.1"
        && ProofpackWriterOperationOutcome::AlreadyExistsMatching.as_str()
            == "already_exists_matching"
        && ProofpackWriterOperationOutcome::ConflictExistingDifferent.as_str()
            == "conflict_existing_different"
        && ProofpackWriterSideEffectStatus::Failed.as_str() == "failed"
        && ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite.as_str()
            == "canonical_artifact_write";
    let planned_ok = planned.schema_version()
        == PROOFPACK_WRITER_ACTIVE_BEHAVIOR_MODEL_SCHEMA_VERSION
        && planned.preflight_status() == ProofpackWriterPreflightIntegrationStatus::Ready
        && planned.is_ready_planned()
        && planned.outcome() == ProofpackWriterOperationOutcome::PlannedOnly
        && planned.proofpack_id().as_str() == "proofpack_eval_001"
        && planned.target_artifact_ref() == Some(file_io_plan.target_artifact_ref())
        && planned.storage_root_ref() == Some(file_io_plan.storage_root_ref())
        && planned.target_path_ref() == Some(file_io_plan.target_path_ref())
        && planned.refs_are_separated()
        && planned.selected_side_effects().len() == 3
        && planned
            .selected_side_effects()
            .contains(&ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite)
        && planned
            .selected_side_effects()
            .contains(&ProofpackWriterPlannedSideEffect::IndexUpdate)
        && planned
            .selected_side_effects()
            .contains(&ProofpackWriterPlannedSideEffect::LatestPointerUpdate)
        && planned.attempted_side_effects().is_empty()
        && planned.completed_side_effects().is_empty()
        && planned.failed_side_effects().is_empty()
        && planned.operation_evidence_persistence_status()
            == ProofpackWriterSideEffectStatus::NotSelected;
    let blocked_ok = blocked.preflight_failed()
        && blocked.outcome() == ProofpackWriterOperationOutcome::PreflightFailed
        && blocked.canonical_artifact_status()
            == ProofpackWriterCanonicalArtifactStatus::NotAttempted
        && blocked.index_status() == ProofpackWriterSideEffectStatus::NotSelected
        && blocked.latest_pointer_status() == ProofpackWriterSideEffectStatus::NotSelected
        && blocked.attempted_side_effects().is_empty()
        && blocked.completed_side_effects().is_empty()
        && blocked.failed_side_effects().is_empty()
        && !blocked.writes_proofpack()
        && !blocked.touches_filesystem();
    let outcome_ok = idempotent.outcome() == ProofpackWriterOperationOutcome::AlreadyExistsMatching
        && idempotent.canonical_artifact_available()
        && idempotent.attempted_side_effects().is_empty()
        && conflict.outcome() == ProofpackWriterOperationOutcome::ConflictExistingDifferent
        && conflict.has_conflict()
        && !conflict.canonical_artifact_available()
        && write_failed.outcome() == ProofpackWriterOperationOutcome::WriteFailed
        && write_failed
            .selected_side_effect_failed(ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite)
        && partial.outcome() == ProofpackWriterOperationOutcome::PartialWriteDetected
        && partial.has_partial_or_cleanup_issue()
        && partial
            .selected_side_effect_failed(ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite);
    let failure_visibility_ok = index_failed.outcome()
        == ProofpackWriterOperationOutcome::IndexUpdateFailed
        && index_failed.operation_kind() == ProofpackWriterOperationKind::IndexUpdate
        && index_failed.selected_side_effect_completed(
            ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite,
        )
        && index_failed.selected_side_effect_failed(ProofpackWriterPlannedSideEffect::IndexUpdate)
        && latest_failed.outcome() == ProofpackWriterOperationOutcome::LatestPointerUpdateFailed
        && latest_failed.operation_kind() == ProofpackWriterOperationKind::LatestPointerUpdate
        && latest_failed.selected_side_effect_completed(
            ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite,
        )
        && latest_failed
            .selected_side_effect_failed(ProofpackWriterPlannedSideEffect::LatestPointerUpdate)
        && evidence_persistence_failed.outcome() == ProofpackWriterOperationOutcome::Written
        && evidence_persistence_failed.has_operation_evidence_persistence_failure()
        && !evidence_persistence_failed.persists_operation_evidence()
        && evidence.outcome() == ProofpackWriterOperationOutcome::IndexUpdateFailed
        && evidence.operation_kind() == ProofpackWriterOperationKind::IndexUpdate
        && evidence.index_status() == ProofpackWriterSideEffectStatus::Failed
        && !evidence.creates_acceptance_claim();
    let boundary_ok = boundary.models_active_writer_behavior
        && boundary.requires_ready_preflight
        && boundary.accepts_explicit_observations
        && boundary.models_selected_attempted_completed_failed_side_effects
        && boundary.keeps_storage_root_target_artifact_and_path_separate
        && boundary.failures_remain_visible
        && boundary.operation_evidence_is_non_authoritative
        && boundary.evidence_only
        && boundary.setup_neutral
        && !boundary.reads_filesystem
        && !boundary.touches_filesystem
        && !boundary.canonicalizes_host_paths
        && !boundary.writes_proofpack
        && !boundary.writes_punk_proofs
        && !boundary.writes_writer_operation_evidence
        && !boundary.persists_operation_evidence
        && !boundary.writes_indexes_or_latest
        && !boundary.writes_final_decision
        && !boundary.creates_acceptance_claim
        && !boundary.requires_runtime_storage
        && !boundary.writes_cli_output
        && !boundary.writes_schema_files
        && !boundary.verifies_referenced_artifacts
        && !boundary.uses_indexes_or_latest_as_authority
        && !boundary.uses_service_mirror_as_authority
        && !boundary.executor_claims_are_proof
        && !boundary.target_path_is_authority
        && !boundary.storage_root_ref_is_authority
        && !boundary.index_latest_are_canonical
        && planned.is_evidence_only()
        && !planned.reads_filesystem()
        && !planned.touches_filesystem()
        && !planned.writes_proofpack()
        && !planned.writes_punk_proofs()
        && !planned.writes_writer_operation_evidence()
        && !planned.persists_operation_evidence()
        && !planned.writes_indexes_or_latest()
        && !planned.requires_runtime_storage()
        && !planned.writes_cli_output()
        && !planned.writes_schema_files()
        && !planned.verifies_referenced_artifacts()
        && !planned.creates_acceptance_claim()
        && !planned.can_claim_acceptance_by_itself()
        && !planned.target_path_is_authority()
        && !planned.storage_root_ref_is_authority()
        && !planned.index_latest_are_canonical()
        && !planned.executor_claims_are_proof();

    if vocabulary_ok
        && planned_ok
        && blocked_ok
        && outcome_ok
        && failure_visibility_ok
        && boundary_ok
    {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_writer_active_behavior_model_is_side_effect_free",
            "proofpack writer active behavior model is side-effect-free",
            "writer active behavior requires ready preflight, models selected/attempted/completed/failed side effects, and keeps operation evidence non-authoritative without filesystem or runtime side effects",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_writer_active_behavior_model_is_side_effect_free",
            "proofpack writer active behavior model is side-effect-free",
            format!(
                "writer active behavior drifted; vocabulary={vocabulary_ok} planned={planned_ok} blocked={blocked_ok} outcome={outcome_ok} failures={failure_visibility_ok} boundary={boundary_ok} reads_fs={} touches_fs={} writes={} punk_proofs={} writes_evidence={} persists_evidence={} index_latest={} storage={} cli={} schemas={} verifies={} acceptance={} service_mirror={} executor_claims={}",
                boundary.reads_filesystem,
                boundary.touches_filesystem,
                boundary.writes_proofpack,
                boundary.writes_punk_proofs,
                boundary.writes_writer_operation_evidence,
                boundary.persists_operation_evidence,
                boundary.writes_indexes_or_latest,
                boundary.requires_runtime_storage,
                boundary.writes_cli_output,
                boundary.writes_schema_files,
                boundary.verifies_referenced_artifacts,
                boundary.creates_acceptance_claim,
                boundary.uses_service_mirror_as_authority,
                boundary.executor_claims_are_proof,
            ),
        )
    }
}

fn eval_proofpack_writer_host_path_resolution_model_is_side_effect_free() -> SmokeEvalCaseResult {
    let proofpack = sample_proofpack("decision_eval_001");
    let canonical = ProofpackWriterCanonicalArtifactModel::from_proofpack(&proofpack, vec![]);
    let target_ref_policy =
        ProofpackWriterTargetArtifactRefPolicyModel::from_canonical_artifact_model(
            &canonical,
            vec![],
        );
    let preflight_plan = ProofpackWriterPreflightPlan::new(
        &proofpack,
        ProofpackWriterTargetRef::from_target_artifact_ref_policy_model(&target_ref_policy)
            .expect("target artifact ref policy should derive logical ref"),
        vec![
            ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite,
            ProofpackWriterPlannedSideEffect::IndexUpdate,
            ProofpackWriterPlannedSideEffect::LatestPointerUpdate,
        ],
        vec![
            ProofBoundaryNote::new("Host path smoke uses explicit preflight inputs only.")
                .expect("boundary note should be valid"),
        ],
    );
    let file_io_plan = ProofpackWriterFileIoPlan::new(
        &preflight_plan,
        ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
            .expect("storage root ref should be valid"),
        ProofpackWriterTargetPathRef::new("future/.punk/proofs/proofpack_eval_001.json")
            .expect("target path ref should be valid"),
        ProofpackWriterWritePolicy::IdempotentIfMatching,
        ProofpackWriterIdempotencyBasis::ManifestSelfDigest,
        ProofpackWriterTempAtomicPolicy::AtomicSiblingTemp,
        vec![
            ProofpackWriterFileIoFailureVisibility::ExistingTargetMatching,
            ProofpackWriterFileIoFailureVisibility::ExistingTargetDifferent,
            ProofpackWriterFileIoFailureVisibility::AtomicMoveFailed,
            ProofpackWriterFileIoFailureVisibility::CleanupFailed,
            ProofpackWriterFileIoFailureVisibility::PartialCanonicalArtifactAmbiguous,
            ProofpackWriterFileIoFailureVisibility::IndexUpdateFailed,
            ProofpackWriterFileIoFailureVisibility::LatestPointerUpdateFailed,
        ],
        vec![ProofBoundaryNote::new(
            "File IO plan remains model input and does not touch the filesystem.",
        )
        .expect("boundary note should be valid")],
    );
    let target_path_policy = ProofpackWriterTargetPathPolicyModel::from_plan(
        &file_io_plan,
        vec![
            ProofBoundaryNote::new("Target path policy stays explicit and evidence-only.")
                .expect("boundary note should be valid"),
        ],
    );
    let ready = ProofpackWriterPreflightIntegrationModel::evaluate(
        &proofpack,
        Some(&canonical),
        Some(&target_ref_policy),
        Some(&preflight_plan),
        Some(&file_io_plan),
        Some(&target_path_policy),
        vec![ProofBoundaryNote::new(
            "Ready preflight is required before host path observations are modeled.",
        )
        .expect("boundary note should be valid")],
    );
    let policy_refs = ProofpackWriterHostPathPolicyRefs::all_selected(
        ProofpackWriterHostPathPolicyRef::new("policy:path-encoding-v0.1")
            .expect("policy ref should be valid"),
        ProofpackWriterHostPathPolicyRef::new("policy:parent-directory-v0.1")
            .expect("policy ref should be valid"),
        ProofpackWriterHostPathPolicyRef::new("policy:symlink-v0.1")
            .expect("policy ref should be valid"),
        ProofpackWriterHostPathPolicyRef::new("policy:canonicalization-v0.1")
            .expect("policy ref should be valid"),
        ProofpackWriterHostPathPolicyRef::new("policy:traversal-v0.1")
            .expect("policy ref should be valid"),
    );
    let observed = ProofpackWriterHostPathResolutionModel::evaluate(
        &ready,
        policy_refs,
        ProofpackWriterHostPathKind::StorageRootRelative,
        Some(
            ProofpackWriterDiagnosticPathRef::new(
                "storage-root-relative:proofpacks/proofpack_eval_001.json",
            )
            .expect("host path observation ref should be valid"),
        ),
        true,
        vec![],
        vec![
            ProofBoundaryNote::new("Host path observation is operational evidence only.")
                .expect("boundary note should be valid"),
        ],
    );

    let unsafe_file_io_plan = ProofpackWriterFileIoPlan::new(
        &preflight_plan,
        ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
            .expect("storage root ref should be valid"),
        ProofpackWriterTargetPathRef::new("future/.punk/../proofs/proofpack_eval_001.json")
            .expect("target path ref should be valid"),
        ProofpackWriterWritePolicy::IdempotentIfMatching,
        ProofpackWriterIdempotencyBasis::ManifestSelfDigest,
        ProofpackWriterTempAtomicPolicy::AtomicSiblingTemp,
        vec![ProofpackWriterFileIoFailureVisibility::ExistingTargetDifferent],
        vec![
            ProofBoundaryNote::new("Unsafe target path remains model data.")
                .expect("boundary note should be valid"),
        ],
    );
    let unsafe_target_path_policy =
        ProofpackWriterTargetPathPolicyModel::from_plan(&unsafe_file_io_plan, vec![]);
    let blocked_preflight = ProofpackWriterPreflightIntegrationModel::evaluate(
        &proofpack,
        Some(&canonical),
        Some(&target_ref_policy),
        Some(&preflight_plan),
        Some(&unsafe_file_io_plan),
        Some(&unsafe_target_path_policy),
        vec![ProofBoundaryNote::new(
            "Blocked preflight must fail closed before host path resolution.",
        )
        .expect("boundary note should be valid")],
    );
    let blocked = ProofpackWriterHostPathResolutionModel::evaluate(
        &blocked_preflight,
        ProofpackWriterHostPathPolicyRefs::new(None, None, None, None, None),
        ProofpackWriterHostPathKind::Absolute,
        Some(
            ProofpackWriterDiagnosticPathRef::new("/redaction-required/proofpack_eval_001.json")
                .expect("host path observation ref should be valid"),
        ),
        false,
        vec![
            ProofpackWriterHostPathBlocker::ParentDirectoryMissing,
            ProofpackWriterHostPathBlocker::SymlinkDisallowed,
            ProofpackWriterHostPathBlocker::CanonicalizationUnavailable,
        ],
        vec![],
    );
    let not_selected_preflight = ProofpackWriterPreflightIntegrationModel::not_selected(
        &proofpack,
        Some(&canonical),
        Some(&target_ref_policy),
        vec![
            ProofBoundaryNote::new("Host path resolution was not selected.")
                .expect("boundary note should be valid"),
        ],
    );
    let not_selected = ProofpackWriterHostPathResolutionModel::not_selected(
        &not_selected_preflight,
        vec![
            ProofBoundaryNote::new("No host path observation is selected.")
                .expect("boundary note should be valid"),
        ],
    );
    let boundary = observed.boundary();

    let vocabulary_ok = PROOFPACK_WRITER_HOST_PATH_RESOLUTION_MODEL_SCHEMA_VERSION
        == "punk.proofpack.writer_host_path_resolution_model.v0.1"
        && ProofpackWriterHostPathObservationStatus::Observed.as_str() == "observed"
        && ProofpackWriterHostPathObservationStatus::Blocked.as_str() == "blocked"
        && ProofpackWriterHostPathKind::StorageRootRelative.as_str() == "storage_root_relative"
        && ProofpackWriterHostPathBlocker::PathEncodingMissing.as_str() == "path_encoding_missing"
        && ProofpackWriterHostPathBlocker::StorageRootEscapeDetected.as_str()
            == "storage_root_escape_detected";
    let observed_ok = observed.schema_version()
        == PROOFPACK_WRITER_HOST_PATH_RESOLUTION_MODEL_SCHEMA_VERSION
        && observed.preflight_status() == ProofpackWriterPreflightIntegrationStatus::Ready
        && observed.is_observed()
        && observed.proofpack_id().as_str() == "proofpack_eval_001"
        && observed.storage_root_ref() == ready.storage_root_ref()
        && observed.logical_target_artifact_ref() == ready.target_artifact_ref()
        && observed.target_path_ref() == ready.target_path_ref()
        && observed.refs_are_separated()
        && observed.policy_refs().all_required_selected()
        && observed.host_path_kind() == ProofpackWriterHostPathKind::StorageRootRelative
        && observed.host_path_ref().is_some()
        && observed.host_path_redacted()
        && observed.blockers().is_empty()
        && observed.operation_outcome() == ProofpackWriterOperationOutcome::PlannedOnly;
    let blocked_ok = blocked.is_blocked()
        && blocked.operation_outcome() == ProofpackWriterOperationOutcome::PreflightFailed
        && blocked.has_blocker(ProofpackWriterHostPathBlocker::PathEncodingMissing)
        && blocked.has_blocker(ProofpackWriterHostPathBlocker::PolicyNotSelected)
        && blocked.has_blocker(ProofpackWriterHostPathBlocker::TargetPathRefInvalid)
        && blocked.has_blocker(ProofpackWriterHostPathBlocker::TraversalDetected)
        && blocked.has_blocker(ProofpackWriterHostPathBlocker::StorageRootEscapeDetected)
        && blocked.has_blocker(ProofpackWriterHostPathBlocker::HostPathRedactionRequired)
        && blocked.has_blocker(ProofpackWriterHostPathBlocker::ParentDirectoryMissing)
        && blocked.has_blocker(ProofpackWriterHostPathBlocker::SymlinkDisallowed)
        && blocked.has_blocker(ProofpackWriterHostPathBlocker::CanonicalizationUnavailable)
        && blocked.blockers_fail_closed()
        && !blocked.policy_refs().all_required_selected()
        && !blocked.host_path_redacted()
        && !blocked.writes_proofpack()
        && !blocked.touches_filesystem();
    let not_selected_ok = not_selected.is_not_selected()
        && not_selected.blockers().is_empty()
        && not_selected.operation_outcome() == ProofpackWriterOperationOutcome::PlannedOnly
        && not_selected.storage_root_ref().is_none()
        && not_selected.target_path_ref().is_none()
        && not_selected.host_path_ref().is_none()
        && not_selected.host_path_redacted();
    let boundary_ok = boundary.models_host_path_resolution
        && boundary.requires_explicit_storage_root_ref
        && boundary.requires_explicit_target_path_ref
        && boundary.requires_explicit_logical_target_artifact_ref
        && boundary.requires_selected_policy_refs
        && boundary.keeps_storage_root_target_artifact_path_and_observation_separate
        && boundary.blockers_fail_closed
        && boundary.host_path_observations_are_operational_evidence
        && !boundary.host_path_observation_implies_proofpack_availability
        && boundary.evidence_only
        && boundary.setup_neutral
        && !boundary.reads_filesystem
        && !boundary.touches_filesystem
        && !boundary.canonicalizes_host_paths
        && !boundary.writes_proofpack
        && !boundary.writes_punk_proofs
        && !boundary.writes_writer_operation_evidence
        && !boundary.persists_operation_evidence
        && !boundary.writes_indexes_or_latest
        && !boundary.writes_final_decision
        && !boundary.creates_acceptance_claim
        && !boundary.requires_runtime_storage
        && !boundary.writes_cli_output
        && !boundary.writes_schema_files
        && !boundary.verifies_referenced_artifacts
        && !boundary.uses_current_working_directory_as_authority
        && !boundary.uses_global_config_as_authority
        && !boundary.uses_ide_state_as_authority
        && !boundary.uses_executor_memory_as_authority
        && !boundary.uses_indexes_or_latest_as_authority
        && !boundary.uses_service_mirror_as_authority
        && !boundary.host_path_is_proof_authority
        && !boundary.target_path_is_authority
        && !boundary.storage_root_ref_is_authority
        && observed.is_evidence_only()
        && !observed.reads_filesystem()
        && !observed.touches_filesystem()
        && !observed.canonicalizes_host_paths()
        && !observed.writes_proofpack()
        && !observed.writes_punk_proofs()
        && !observed.writes_writer_operation_evidence()
        && !observed.persists_operation_evidence()
        && !observed.writes_indexes_or_latest()
        && !observed.writes_final_decision()
        && !observed.creates_acceptance_claim()
        && !observed.requires_runtime_storage()
        && !observed.writes_cli_output()
        && !observed.writes_schema_files()
        && !observed.verifies_referenced_artifacts()
        && !observed.implies_proofpack_availability()
        && !observed.host_path_is_proof_authority()
        && !observed.target_path_is_authority()
        && !observed.storage_root_ref_is_authority()
        && !observed.uses_current_working_directory_as_authority()
        && !observed.uses_global_config_as_authority()
        && !observed.uses_ide_state_as_authority()
        && !observed.uses_executor_memory_as_authority()
        && !observed.uses_indexes_or_latest_as_authority()
        && !observed.uses_service_mirror_as_authority()
        && !observed.can_claim_acceptance_by_itself();

    if vocabulary_ok && observed_ok && blocked_ok && not_selected_ok && boundary_ok {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_writer_host_path_resolution_model_is_side_effect_free",
            "proofpack writer host path resolution model is side-effect-free",
            "writer host path resolution composes explicit refs, selected policies, observed/blocked/not-selected evidence, and fail-closed blockers without filesystem or runtime side effects",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_writer_host_path_resolution_model_is_side_effect_free",
            "proofpack writer host path resolution model is side-effect-free",
            format!(
                "writer host path resolution drifted; vocabulary={vocabulary_ok} observed={observed_ok} blocked={blocked_ok} not_selected={not_selected_ok} boundary={boundary_ok} reads_fs={} touches_fs={} canonicalizes={} writes={} punk_proofs={} writes_evidence={} persists_evidence={} index_latest={} storage={} cli={} schemas={} verifies={} acceptance={} cwd={} global_config={} ide={} executor_memory={} service_mirror={} host_path_authority={}",
                boundary.reads_filesystem,
                boundary.touches_filesystem,
                boundary.canonicalizes_host_paths,
                boundary.writes_proofpack,
                boundary.writes_punk_proofs,
                boundary.writes_writer_operation_evidence,
                boundary.persists_operation_evidence,
                boundary.writes_indexes_or_latest,
                boundary.requires_runtime_storage,
                boundary.writes_cli_output,
                boundary.writes_schema_files,
                boundary.verifies_referenced_artifacts,
                boundary.creates_acceptance_claim,
                boundary.uses_current_working_directory_as_authority,
                boundary.uses_global_config_as_authority,
                boundary.uses_ide_state_as_authority,
                boundary.uses_executor_memory_as_authority,
                boundary.uses_service_mirror_as_authority,
                boundary.host_path_is_proof_authority,
            ),
        )
    }
}

fn eval_proofpack_writer_concrete_path_storage_policy_model_is_side_effect_free(
) -> SmokeEvalCaseResult {
    let proofpack = sample_proofpack("decision_eval_001");
    let canonical = ProofpackWriterCanonicalArtifactModel::from_proofpack(&proofpack, vec![]);
    let target_ref_policy =
        ProofpackWriterTargetArtifactRefPolicyModel::from_canonical_artifact_model(
            &canonical,
            vec![],
        );
    let preflight_plan = ProofpackWriterPreflightPlan::new(
        &proofpack,
        ProofpackWriterTargetRef::from_target_artifact_ref_policy_model(&target_ref_policy)
            .expect("target artifact ref policy should derive logical ref"),
        vec![
            ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite,
            ProofpackWriterPlannedSideEffect::IndexUpdate,
            ProofpackWriterPlannedSideEffect::LatestPointerUpdate,
        ],
        vec![
            ProofBoundaryNote::new("Concrete path/storage smoke uses explicit preflight inputs.")
                .expect("boundary note should be valid"),
        ],
    );
    let file_io_plan = ProofpackWriterFileIoPlan::new(
        &preflight_plan,
        ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
            .expect("storage root ref should be valid"),
        ProofpackWriterTargetPathRef::new("future/.punk/proofs/proofpack_eval_001.json")
            .expect("target path ref should be valid"),
        ProofpackWriterWritePolicy::IdempotentIfMatching,
        ProofpackWriterIdempotencyBasis::ManifestSelfDigest,
        ProofpackWriterTempAtomicPolicy::AtomicSiblingTemp,
        vec![
            ProofpackWriterFileIoFailureVisibility::ExistingTargetMatching,
            ProofpackWriterFileIoFailureVisibility::ExistingTargetDifferent,
            ProofpackWriterFileIoFailureVisibility::AtomicMoveFailed,
            ProofpackWriterFileIoFailureVisibility::CleanupFailed,
            ProofpackWriterFileIoFailureVisibility::PartialCanonicalArtifactAmbiguous,
            ProofpackWriterFileIoFailureVisibility::IndexUpdateFailed,
            ProofpackWriterFileIoFailureVisibility::LatestPointerUpdateFailed,
        ],
        vec![ProofBoundaryNote::new(
            "File IO plan remains model input and does not touch the filesystem.",
        )
        .expect("boundary note should be valid")],
    );
    let target_path_policy = ProofpackWriterTargetPathPolicyModel::from_plan(
        &file_io_plan,
        vec![
            ProofBoundaryNote::new("Target path policy stays explicit and evidence-only.")
                .expect("boundary note should be valid"),
        ],
    );
    let ready = ProofpackWriterPreflightIntegrationModel::evaluate(
        &proofpack,
        Some(&canonical),
        Some(&target_ref_policy),
        Some(&preflight_plan),
        Some(&file_io_plan),
        Some(&target_path_policy),
        vec![ProofBoundaryNote::new(
            "Ready preflight is required before concrete path/storage policy evidence is modeled.",
        )
        .expect("boundary note should be valid")],
    );
    let observed_host_path = ProofpackWriterHostPathResolutionModel::evaluate(
        &ready,
        ProofpackWriterHostPathPolicyRefs::all_selected(
            ProofpackWriterHostPathPolicyRef::new("policy:path-encoding-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:parent-directory-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:symlink-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:canonicalization-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:traversal-v0.1")
                .expect("policy ref should be valid"),
        ),
        ProofpackWriterHostPathKind::StorageRootRelative,
        Some(
            ProofpackWriterDiagnosticPathRef::new(
                "storage-root-relative:proofpacks/proofpack_eval_001.json",
            )
            .expect("host path observation ref should be valid"),
        ),
        true,
        vec![],
        vec![
            ProofBoundaryNote::new("Host path observation is operational evidence only.")
                .expect("boundary note should be valid"),
        ],
    );
    let policy_refs = ProofpackWriterConcretePathStoragePolicyRefs::all_selected(
        ProofpackWriterHostPathPolicyRef::new("policy:storage-root-selection-v0.1")
            .expect("policy ref should be valid"),
        ProofpackWriterHostPathPolicyRef::new("policy:logical-target-artifact-v0.1")
            .expect("policy ref should be valid"),
        ProofpackWriterHostPathPolicyRef::new("policy:target-path-derivation-v0.1")
            .expect("policy ref should be valid"),
        ProofpackWriterHostPathPolicyRef::new("policy:path-encoding-v0.1")
            .expect("policy ref should be valid"),
        ProofpackWriterHostPathPolicyRef::new("policy:parent-directory-v0.1")
            .expect("policy ref should be valid"),
        ProofpackWriterHostPathPolicyRef::new("policy:symlink-v0.1")
            .expect("policy ref should be valid"),
        ProofpackWriterHostPathPolicyRef::new("policy:canonicalization-v0.1")
            .expect("policy ref should be valid"),
        ProofpackWriterHostPathPolicyRef::new("policy:traversal-v0.1")
            .expect("policy ref should be valid"),
        ProofpackWriterHostPathPolicyRef::new("policy:storage-root-escape-v0.1")
            .expect("policy ref should be valid"),
        ProofpackWriterHostPathPolicyRef::new("policy:redaction-v0.1")
            .expect("policy ref should be valid"),
        ProofpackWriterHostPathPolicyRef::new("policy:idempotency-conflict-v0.1")
            .expect("policy ref should be valid"),
        ProofpackWriterHostPathPolicyRef::new("policy:temp-atomic-v0.1")
            .expect("policy ref should be valid"),
        ProofpackWriterHostPathPolicyRef::new("policy:index-latest-non-authority-v0.1")
            .expect("policy ref should be valid"),
    );
    let ready_model = ProofpackWriterConcretePathStoragePolicyModel::evaluate(
        &ready,
        Some(&observed_host_path),
        policy_refs,
        vec![ProofBoundaryNote::new(
            "Concrete path/storage policy evidence does not authorize writer side effects.",
        )
        .expect("boundary note should be valid")],
    );

    let unsafe_file_io_plan = ProofpackWriterFileIoPlan::new(
        &preflight_plan,
        ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
            .expect("storage root ref should be valid"),
        ProofpackWriterTargetPathRef::new("future/.punk/../proofs/proofpack_eval_001.json")
            .expect("target path ref should be valid"),
        ProofpackWriterWritePolicy::IdempotentIfMatching,
        ProofpackWriterIdempotencyBasis::ManifestSelfDigest,
        ProofpackWriterTempAtomicPolicy::AtomicSiblingTemp,
        vec![ProofpackWriterFileIoFailureVisibility::ExistingTargetDifferent],
        vec![
            ProofBoundaryNote::new("Unsafe target path remains model data.")
                .expect("boundary note should be valid"),
        ],
    );
    let unsafe_target_path_policy =
        ProofpackWriterTargetPathPolicyModel::from_plan(&unsafe_file_io_plan, vec![]);
    let blocked_preflight = ProofpackWriterPreflightIntegrationModel::evaluate(
        &proofpack,
        Some(&canonical),
        Some(&target_ref_policy),
        Some(&preflight_plan),
        Some(&unsafe_file_io_plan),
        Some(&unsafe_target_path_policy),
        vec![ProofBoundaryNote::new(
            "Blocked preflight must fail closed before concrete path/storage policy readiness.",
        )
        .expect("boundary note should be valid")],
    );
    let blocked_host_path = ProofpackWriterHostPathResolutionModel::evaluate(
        &blocked_preflight,
        ProofpackWriterHostPathPolicyRefs::new(None, None, None, None, None),
        ProofpackWriterHostPathKind::Absolute,
        Some(
            ProofpackWriterDiagnosticPathRef::new("/redaction-required/proofpack_eval_001.json")
                .expect("host path observation ref should be valid"),
        ),
        false,
        vec![
            ProofpackWriterHostPathBlocker::StorageRootRefDisallowed,
            ProofpackWriterHostPathBlocker::ParentDirectoryMissing,
            ProofpackWriterHostPathBlocker::SymlinkDisallowed,
            ProofpackWriterHostPathBlocker::CanonicalizationUnavailable,
        ],
        vec![],
    );
    let blocked_model = ProofpackWriterConcretePathStoragePolicyModel::evaluate(
        &blocked_preflight,
        Some(&blocked_host_path),
        ProofpackWriterConcretePathStoragePolicyRefs::new(
            None, None, None, None, None, None, None, None, None, None, None, None, None,
        ),
        vec![],
    );
    let not_selected_preflight = ProofpackWriterPreflightIntegrationModel::not_selected(
        &proofpack,
        Some(&canonical),
        Some(&target_ref_policy),
        vec![
            ProofBoundaryNote::new("Concrete path/storage policy was not selected.")
                .expect("boundary note should be valid"),
        ],
    );
    let not_selected_model = ProofpackWriterConcretePathStoragePolicyModel::not_selected(
        &not_selected_preflight,
        vec![
            ProofBoundaryNote::new("No concrete path/storage policy evidence is selected.")
                .expect("boundary note should be valid"),
        ],
    );
    let boundary = ready_model.boundary();

    let vocabulary_ok = PROOFPACK_WRITER_CONCRETE_PATH_STORAGE_POLICY_MODEL_SCHEMA_VERSION
        == "punk.proofpack.writer_concrete_path_storage_policy_model.v0.1"
        && ProofpackWriterConcretePathStoragePolicyStatus::Ready.as_str() == "ready"
        && ProofpackWriterConcretePathStoragePolicyStatus::Blocked.as_str() == "blocked"
        && ProofpackWriterConcretePathStoragePolicyBlocker::StorageRootPolicyMissing.as_str()
            == "storage_root_policy_missing"
        && ProofpackWriterConcretePathStoragePolicyBlocker::PathEncodingRejected.as_str()
            == "path_encoding_rejected"
        && ProofpackWriterConcretePathStoragePolicyBlocker::IndexLatestPolicyMissing.as_str()
            == "index_latest_policy_missing";
    let ready_ok = ready_model.schema_version()
        == PROOFPACK_WRITER_CONCRETE_PATH_STORAGE_POLICY_MODEL_SCHEMA_VERSION
        && ready_model.preflight_status() == ProofpackWriterPreflightIntegrationStatus::Ready
        && ready_model.host_path_status()
            == Some(ProofpackWriterHostPathObservationStatus::Observed)
        && ready_model.is_ready()
        && ready_model.proofpack_id().as_str() == "proofpack_eval_001"
        && ready_model.manifest_self_digest().is_some()
        && ready_model.storage_root_ref() == ready.storage_root_ref()
        && ready_model.logical_target_artifact_ref() == ready.target_artifact_ref()
        && ready_model.target_path_ref() == ready.target_path_ref()
        && ready_model.refs_are_separated()
        && ready_model
            .policy_refs()
            .all_required_selected(ready.planned_side_effects())
        && ready_model.host_path_kind() == Some(ProofpackWriterHostPathKind::StorageRootRelative)
        && ready_model.host_path_ref().is_some()
        && ready_model.host_path_redacted() == Some(true)
        && ready_model.blockers().is_empty()
        && ready_model.operation_outcome() == ProofpackWriterOperationOutcome::PlannedOnly;
    let blocked_ok = blocked_model.is_blocked()
        && blocked_model.operation_outcome() == ProofpackWriterOperationOutcome::PreflightFailed
        && blocked_model
            .has_blocker(ProofpackWriterConcretePathStoragePolicyBlocker::PreflightNotReady)
        && blocked_model
            .has_blocker(ProofpackWriterConcretePathStoragePolicyBlocker::StorageRootPolicyMissing)
        && blocked_model
            .has_blocker(ProofpackWriterConcretePathStoragePolicyBlocker::StorageRootRefDisallowed)
        && blocked_model.has_blocker(
            ProofpackWriterConcretePathStoragePolicyBlocker::TargetPathDerivationPolicyMissing,
        )
        && blocked_model
            .has_blocker(ProofpackWriterConcretePathStoragePolicyBlocker::RejectedTargetPathPolicy)
        && blocked_model.has_blocker(
            ProofpackWriterConcretePathStoragePolicyBlocker::PathEncodingPolicyMissing,
        )
        && blocked_model
            .has_blocker(ProofpackWriterConcretePathStoragePolicyBlocker::PathEncodingRejected)
        && blocked_model
            .has_blocker(ProofpackWriterConcretePathStoragePolicyBlocker::ParentDirectoryMissing)
        && blocked_model
            .has_blocker(ProofpackWriterConcretePathStoragePolicyBlocker::SymlinkDisallowed)
        && blocked_model.has_blocker(
            ProofpackWriterConcretePathStoragePolicyBlocker::CanonicalizationUnavailable,
        )
        && blocked_model
            .has_blocker(ProofpackWriterConcretePathStoragePolicyBlocker::TraversalDetected)
        && blocked_model.has_blocker(
            ProofpackWriterConcretePathStoragePolicyBlocker::StorageRootEscapeDetected,
        )
        && blocked_model.has_blocker(
            ProofpackWriterConcretePathStoragePolicyBlocker::HostPathRedactionRequired,
        )
        && blocked_model.has_blocker(
            ProofpackWriterConcretePathStoragePolicyBlocker::IdempotencyConflictPolicyMissing,
        )
        && blocked_model
            .has_blocker(ProofpackWriterConcretePathStoragePolicyBlocker::TempAtomicPolicyMissing)
        && blocked_model
            .has_blocker(ProofpackWriterConcretePathStoragePolicyBlocker::IndexLatestPolicyMissing)
        && blocked_model.blockers_fail_closed()
        && blocked_model.host_path_redacted() == Some(false)
        && !blocked_model.writes_proofpack()
        && !blocked_model.touches_filesystem();
    let not_selected_ok = not_selected_model.is_not_selected()
        && not_selected_model.blockers().is_empty()
        && not_selected_model.operation_outcome() == ProofpackWriterOperationOutcome::PlannedOnly
        && not_selected_model.storage_root_ref().is_none()
        && not_selected_model.target_path_ref().is_none()
        && not_selected_model.host_path_ref().is_none()
        && not_selected_model.host_path_redacted().is_none();
    let boundary_ok = boundary.models_concrete_path_storage_policy
        && boundary.requires_explicit_storage_root_ref
        && boundary.requires_explicit_logical_target_artifact_ref
        && boundary.requires_explicit_target_path_ref
        && boundary.requires_selected_storage_and_path_policy_refs
        && boundary
            .keeps_storage_root_target_artifact_path_host_observation_policy_and_canonical_artifact_separate
        && boundary.blockers_fail_closed
        && boundary.policy_refs_are_operational_evidence
        && boundary.host_path_observations_are_operational_evidence
        && boundary.evidence_only
        && boundary.setup_neutral
        && !boundary.activates_writer
        && !boundary.reads_filesystem
        && !boundary.inspects_filesystem
        && !boundary.touches_filesystem
        && !boundary.canonicalizes_host_paths
        && !boundary.normalizes_host_paths
        && !boundary.writes_proofpack
        && !boundary.writes_punk_proofs
        && !boundary.writes_writer_operation_evidence
        && !boundary.persists_operation_evidence
        && !boundary.writes_indexes_or_latest
        && !boundary.writes_final_decision
        && !boundary.creates_acceptance_claim
        && !boundary.requires_runtime_storage
        && !boundary.writes_cli_output
        && !boundary.exposes_cli_behavior
        && !boundary.writes_schema_files
        && !boundary.verifies_referenced_artifacts
        && !boundary.invokes_adapter
        && !boundary.invokes_automation
        && !boundary.invokes_provider_or_model_runner
        && !boundary.compiles_context
        && !boundary.implements_knowledge_vault
        && !boundary.implements_compiled_wiki
        && !boundary.implements_punk_init
        && !boundary.policy_refs_are_proof_authority
        && !boundary.host_path_is_proof_authority
        && !boundary.target_path_is_authority
        && !boundary.storage_root_ref_is_authority
        && !boundary.uses_current_working_directory_as_authority
        && !boundary.uses_global_config_as_authority
        && !boundary.uses_ide_state_as_authority
        && !boundary.uses_executor_memory_as_authority
        && !boundary.uses_indexes_or_latest_as_authority
        && !boundary.uses_service_mirror_as_authority
        && !ready_model.activates_writer()
        && !ready_model.reads_filesystem()
        && !ready_model.inspects_filesystem()
        && !ready_model.touches_filesystem()
        && !ready_model.canonicalizes_host_paths()
        && !ready_model.normalizes_host_paths()
        && !ready_model.writes_proofpack()
        && !ready_model.writes_punk_proofs()
        && !ready_model.writes_writer_operation_evidence()
        && !ready_model.persists_operation_evidence()
        && !ready_model.writes_indexes_or_latest()
        && !ready_model.writes_final_decision()
        && !ready_model.creates_acceptance_claim()
        && !ready_model.requires_runtime_storage()
        && !ready_model.writes_cli_output()
        && !ready_model.exposes_cli_behavior()
        && !ready_model.writes_schema_files()
        && !ready_model.verifies_referenced_artifacts()
        && !ready_model.invokes_adapter()
        && !ready_model.invokes_automation()
        && !ready_model.invokes_provider_or_model_runner()
        && !ready_model.compiles_context()
        && !ready_model.implements_knowledge_vault()
        && !ready_model.implements_compiled_wiki()
        && !ready_model.implements_punk_init()
        && !ready_model.policy_refs_are_proof_authority()
        && !ready_model.host_path_is_proof_authority()
        && !ready_model.target_path_is_authority()
        && !ready_model.storage_root_ref_is_authority()
        && !ready_model.uses_current_working_directory_as_authority()
        && !ready_model.uses_global_config_as_authority()
        && !ready_model.uses_ide_state_as_authority()
        && !ready_model.uses_executor_memory_as_authority()
        && !ready_model.uses_indexes_or_latest_as_authority()
        && !ready_model.uses_service_mirror_as_authority()
        && !ready_model.can_claim_acceptance_by_itself();

    if vocabulary_ok && ready_ok && blocked_ok && not_selected_ok && boundary_ok {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_writer_concrete_path_storage_policy_model_is_side_effect_free",
            "proofpack writer concrete path/storage policy model is side-effect-free",
            "writer concrete path/storage policy composes explicit refs, selected policies, host path evidence, and fail-closed blockers without filesystem or runtime side effects",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_writer_concrete_path_storage_policy_model_is_side_effect_free",
            "proofpack writer concrete path/storage policy model is side-effect-free",
            format!(
                "writer concrete path/storage policy drifted; vocabulary={vocabulary_ok} ready={ready_ok} blocked={blocked_ok} not_selected={not_selected_ok} boundary={boundary_ok} activates={} reads_fs={} inspects_fs={} touches_fs={} canonicalizes={} normalizes={} writes={} punk_proofs={} writes_evidence={} persists_evidence={} index_latest={} storage={} cli_output={} cli_behavior={} schemas={} verifies={} acceptance={} adapter={} automation={} provider={} context={} knowledge_vault={} compiled_wiki={} punk_init={} policy_authority={} host_path_authority={}",
                boundary.activates_writer,
                boundary.reads_filesystem,
                boundary.inspects_filesystem,
                boundary.touches_filesystem,
                boundary.canonicalizes_host_paths,
                boundary.normalizes_host_paths,
                boundary.writes_proofpack,
                boundary.writes_punk_proofs,
                boundary.writes_writer_operation_evidence,
                boundary.persists_operation_evidence,
                boundary.writes_indexes_or_latest,
                boundary.requires_runtime_storage,
                boundary.writes_cli_output,
                boundary.exposes_cli_behavior,
                boundary.writes_schema_files,
                boundary.verifies_referenced_artifacts,
                boundary.creates_acceptance_claim,
                boundary.invokes_adapter,
                boundary.invokes_automation,
                boundary.invokes_provider_or_model_runner,
                boundary.compiles_context,
                boundary.implements_knowledge_vault,
                boundary.implements_compiled_wiki,
                boundary.implements_punk_init,
                boundary.policy_refs_are_proof_authority,
                boundary.host_path_is_proof_authority,
            ),
        )
    }
}

fn sample_writer_first_active_write_slice_inputs() -> (
    ProofpackWriterCanonicalArtifactModel,
    ProofpackWriterPreflightIntegrationModel,
    ProofpackWriterConcretePathStoragePolicyModel,
) {
    let proofpack = sample_proofpack("decision_eval_001");
    let canonical = ProofpackWriterCanonicalArtifactModel::from_proofpack(
        &proofpack,
        vec![
            ProofBoundaryNote::new("First active smoke uses explicit canonical artifact bytes.")
                .expect("boundary note should be valid"),
        ],
    );
    let target_ref_policy =
        ProofpackWriterTargetArtifactRefPolicyModel::from_canonical_artifact_model(
            &canonical,
            vec![
                ProofBoundaryNote::new("First active smoke target artifact ref is explicit.")
                    .expect("boundary note should be valid"),
            ],
        );
    let preflight_plan = ProofpackWriterPreflightPlan::new(
        &proofpack,
        ProofpackWriterTargetRef::from_target_artifact_ref_policy_model(&target_ref_policy)
            .expect("target artifact ref policy should derive logical ref"),
        vec![ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite],
        vec![
            ProofBoundaryNote::new("First active smoke selects only canonical artifact writing.")
                .expect("boundary note should be valid"),
        ],
    );
    let file_io_plan = ProofpackWriterFileIoPlan::new(
        &preflight_plan,
        ProofpackWriterStorageRootRef::new("explicit_eval_storage_root")
            .expect("storage root ref should be valid"),
        ProofpackWriterTargetPathRef::new("proofpacks/proofpack_eval_001.json")
            .expect("target path ref should be valid"),
        ProofpackWriterWritePolicy::IdempotentIfMatching,
        ProofpackWriterIdempotencyBasis::ExactManifestBytes,
        ProofpackWriterTempAtomicPolicy::ExplicitNonAtomic,
        vec![
            ProofpackWriterFileIoFailureVisibility::ExistingTargetMatching,
            ProofpackWriterFileIoFailureVisibility::ExistingTargetDifferent,
            ProofpackWriterFileIoFailureVisibility::TempWriteFailed,
            ProofpackWriterFileIoFailureVisibility::PartialCanonicalArtifactAmbiguous,
        ],
        vec![ProofBoundaryNote::new(
            "First active smoke uses explicit non-atomic create-new behavior without platform guarantees.",
        )
        .expect("boundary note should be valid")],
    );
    let target_path_policy = ProofpackWriterTargetPathPolicyModel::from_plan(
        &file_io_plan,
        vec![ProofBoundaryNote::new(
            "First active smoke target path policy is accepted before writing.",
        )
        .expect("boundary note should be valid")],
    );
    let preflight = ProofpackWriterPreflightIntegrationModel::evaluate(
        &proofpack,
        Some(&canonical),
        Some(&target_ref_policy),
        Some(&preflight_plan),
        Some(&file_io_plan),
        Some(&target_path_policy),
        vec![
            ProofBoundaryNote::new("First active smoke preflight composes explicit inputs only.")
                .expect("boundary note should be valid"),
        ],
    );
    let host_path = ProofpackWriterHostPathResolutionModel::evaluate(
        &preflight,
        ProofpackWriterHostPathPolicyRefs::all_selected(
            ProofpackWriterHostPathPolicyRef::new("policy:path-encoding-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:parent-directory-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:symlink-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:canonicalization-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:traversal-v0.1")
                .expect("policy ref should be valid"),
        ),
        ProofpackWriterHostPathKind::StorageRootRelative,
        Some(
            ProofpackWriterDiagnosticPathRef::new(
                "storage-root-relative:proofpacks/proofpack_eval_001.json",
            )
            .expect("host path observation ref should be valid"),
        ),
        true,
        vec![],
        vec![ProofBoundaryNote::new(
            "First active smoke host path observation remains operational evidence only.",
        )
        .expect("boundary note should be valid")],
    );
    let concrete_policy = ProofpackWriterConcretePathStoragePolicyModel::evaluate(
        &preflight,
        Some(&host_path),
        ProofpackWriterConcretePathStoragePolicyRefs::all_selected(
            ProofpackWriterHostPathPolicyRef::new("policy:storage-root-selection-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:logical-target-artifact-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:target-path-derivation-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:path-encoding-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:parent-directory-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:symlink-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:canonicalization-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:traversal-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:storage-root-escape-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:redaction-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:idempotency-conflict-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:temp-atomic-v0.1")
                .expect("policy ref should be valid"),
            ProofpackWriterHostPathPolicyRef::new("policy:index-latest-non-authority-v0.1")
                .expect("policy ref should be valid"),
        ),
        vec![ProofBoundaryNote::new(
            "First active smoke concrete path/storage policy is ready before writing.",
        )
        .expect("boundary note should be valid")],
    );

    (canonical, preflight, concrete_policy)
}

fn eval_proofpack_writer_first_active_write_slice_writes_exact_bytes() -> SmokeEvalCaseResult {
    let (canonical, preflight, concrete_policy) = sample_writer_first_active_write_slice_inputs();
    let storage_root = unique_smoke_temp_path();
    let target_relative = std::path::Path::new("proofpacks/proofpack_eval_001.json");
    let target_path = storage_root.join(target_relative);

    if let Err(error) = fs::create_dir_all(storage_root.join("proofpacks")) {
        return SmokeEvalCaseResult::fail(
            "eval_proofpack_writer_first_active_write_slice_writes_exact_bytes",
            "proofpack writer first active write slice writes exact bytes",
            format!("test setup could not create explicit parent directory: {error:?}"),
        );
    }

    let result = proofpack_writer_write_first_active_slice(
        &preflight,
        &concrete_policy,
        &canonical,
        &storage_root,
        target_relative,
        vec![ProofBoundaryNote::new(
            "First active smoke writes exact bytes to an explicit temporary target only.",
        )
        .expect("boundary note should be valid")],
    );
    let written = fs::read(&target_path);
    let punk_runtime_exists = storage_root.join(".punk").exists();
    let cleanup_ok = fs::remove_dir_all(&storage_root).is_ok();
    let boundary = proofpack_writer_first_active_write_slice_boundary();

    let vocabulary_ok = PROOFPACK_WRITER_FIRST_ACTIVE_WRITE_SLICE_SCHEMA_VERSION
        == "punk.proofpack.writer_first_active_write_slice.v0.1"
        && ProofpackWriterFirstActiveWriteSliceBlocker::PreflightNotReady.as_str()
            == "preflight_not_ready"
        && ProofpackWriterFirstActiveWriteSliceBlocker::ExistingTargetDifferent.as_str()
            == "existing_target_different";
    let write_ok = result.schema_version()
        == PROOFPACK_WRITER_FIRST_ACTIVE_WRITE_SLICE_SCHEMA_VERSION
        && result.outcome() == ProofpackWriterOperationOutcome::Written
        && result.canonical_artifact_status() == ProofpackWriterCanonicalArtifactStatus::Written
        && result.represents_new_canonical_artifact_write()
        && result.canonical_artifact_available()
        && result.blockers().is_empty()
        && matches!(&written, Ok(bytes) if bytes.as_slice() == canonical.canonical_body_bytes())
        && result.index_status() == ProofpackWriterSideEffectStatus::NotSelected
        && result.latest_pointer_status() == ProofpackWriterSideEffectStatus::NotSelected
        && result.cleanup_status() == ProofpackWriterSideEffectStatus::NotSelected;
    let boundary_ok = boundary.implements_first_active_write_slice
        && boundary.requires_ready_preflight
        && boundary.requires_concrete_path_storage_policy_ready
        && boundary.requires_explicit_storage_root_path
        && boundary.requires_explicit_target_relative_path
        && boundary.writes_exact_canonical_bytes
        && boundary.uses_create_new_no_overwrite
        && boundary.reads_filesystem
        && boundary.touches_filesystem
        && boundary.writes_proofpack
        && !boundary.writes_punk_proofs
        && !boundary.writes_writer_operation_evidence
        && !boundary.persists_operation_evidence
        && !boundary.writes_indexes_or_latest
        && !boundary.writes_final_decision
        && !boundary.creates_acceptance_claim
        && !boundary.requires_runtime_storage
        && !boundary.writes_cli_output
        && !boundary.exposes_cli_behavior
        && !boundary.writes_schema_files
        && !boundary.verifies_referenced_artifacts
        && !boundary.creates_parent_directories
        && !boundary.canonicalizes_host_paths
        && !boundary.normalizes_host_paths
        && !boundary.claims_platform_atomicity
        && !boundary.claims_crash_durability
        && !boundary.uses_current_working_directory_as_authority
        && !boundary.uses_global_config_as_authority
        && !boundary.uses_ide_state_as_authority
        && !boundary.uses_executor_memory_as_authority
        && !boundary.uses_indexes_or_latest_as_authority
        && !boundary.uses_service_mirror_as_authority
        && !boundary.target_path_is_authority
        && !boundary.storage_root_path_is_authority
        && boundary.operation_evidence_is_non_authoritative
        && boundary.setup_neutral
        && result.writes_exact_canonical_bytes()
        && result.writes_proofpack()
        && !result.writes_punk_proofs()
        && !result.persists_operation_evidence()
        && !result.writes_indexes_or_latest()
        && !result.requires_runtime_storage()
        && !result.writes_cli_output()
        && !result.exposes_cli_behavior()
        && !result.writes_schema_files()
        && !result.verifies_referenced_artifacts()
        && !result.creates_acceptance_claim()
        && !result.claims_platform_atomicity()
        && !result.claims_crash_durability()
        && !result.canonicalizes_host_paths()
        && !result.normalizes_host_paths()
        && !result.uses_current_working_directory_as_authority()
        && !result.uses_global_config_as_authority()
        && !result.uses_ide_state_as_authority()
        && !result.uses_executor_memory_as_authority()
        && !result.uses_indexes_or_latest_as_authority()
        && !result.target_path_is_authority()
        && !result.storage_root_path_is_authority()
        && !result.can_claim_acceptance_by_itself();
    let no_forbidden_side_effects = !punk_runtime_exists && cleanup_ok;

    if vocabulary_ok && write_ok && boundary_ok && no_forbidden_side_effects {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_writer_first_active_write_slice_writes_exact_bytes",
            "proofpack writer first active write slice writes exact bytes",
            "writer first active write slice wrote exact canonical bytes to an explicit temporary target while leaving runtime storage, indexes, CLI, schema, persisted evidence, gate, and acceptance inactive",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_writer_first_active_write_slice_writes_exact_bytes",
            "proofpack writer first active write slice writes exact bytes",
            format!(
                "writer first active write slice drifted; vocabulary={vocabulary_ok} write={write_ok} boundary={boundary_ok} no_forbidden_side_effects={no_forbidden_side_effects} outcome={:?} canonical={:?} blockers={:?} read={:?} cleanup_ok={cleanup_ok} punk_runtime_exists={punk_runtime_exists}",
                result.outcome(),
                result.canonical_artifact_status(),
                result.blockers(),
                written.as_ref().map(|bytes| bytes.len()),
            ),
        )
    }
}

fn eval_proofpack_writer_hash_reference_integration_model_is_side_effect_free(
) -> SmokeEvalCaseResult {
    let proofpack = sample_proofpack("decision_eval_001");
    let canonical = ProofpackWriterCanonicalArtifactModel::from_proofpack(&proofpack, vec![]);
    let requirements = proofpack.required_artifact_digest_refs();
    let declared = requirements
        .iter()
        .cloned()
        .map(|requirement| {
            let digest = proofpack
                .artifact_digests()
                .iter()
                .find(|digest| digest.satisfies_requirement(&requirement))
                .expect("sample proofpack should have digest for requirement");
            ProofpackWriterDeclaredArtifactDigestEvidence::from_declared_digest(requirement, digest)
        })
        .collect::<Vec<_>>();
    let verification = requirements
        .iter()
        .cloned()
        .map(|requirement| {
            let digest = proofpack
                .artifact_digests()
                .iter()
                .find(|digest| digest.satisfies_requirement(&requirement))
                .expect("sample proofpack should have digest for requirement");
            ProofpackWriterReferencedArtifactVerificationEvidence::verified(
                requirement,
                ArtifactDigest::new(digest.artifact_hash().as_str())
                    .expect("sample proofpack digest should satisfy core policy"),
            )
        })
        .collect::<Vec<_>>();
    let ready = ProofpackWriterHashReferenceIntegrationModel::evaluate(
        &proofpack,
        Some(&canonical),
        declared.clone(),
        verification,
        vec![
            ProofBoundaryNote::new("Hash/reference smoke composes explicit evidence only.")
                .expect("boundary note should be valid"),
        ],
    );
    let blocked = ProofpackWriterHashReferenceIntegrationModel::evaluate(
        &proofpack,
        Some(&canonical),
        vec![
            ProofpackWriterDeclaredArtifactDigestEvidence::missing(requirements[0].clone()),
            ProofpackWriterDeclaredArtifactDigestEvidence::invalid_format(
                requirements[1].clone(),
                "sha256:NOT-LOWERCASE-HEX",
            ),
        ],
        vec![
            ProofpackWriterReferencedArtifactVerificationEvidence::required(
                requirements[2].clone(),
                ProofpackWriterReferencedArtifactVerificationStatus::DigestMismatch,
                Some(
                    ArtifactDigest::new(PROOF_HASH_RUN_RECEIPT)
                        .expect("expected digest should be valid"),
                ),
                Some(
                    ArtifactDigest::new(PROOF_HASH_CONTRACT)
                        .expect("observed digest should be valid"),
                ),
                vec![],
            ),
        ],
        vec![],
    );
    let optional = ProofpackWriterHashReferenceIntegrationModel::evaluate(
        &proofpack,
        Some(&canonical),
        declared,
        vec![
            ProofpackWriterReferencedArtifactVerificationEvidence::optional(
                requirements[0].clone(),
                ProofpackWriterReferencedArtifactVerificationStatus::Unverified,
                Some(
                    ArtifactDigest::new(PROOF_HASH_GATE_DECISION)
                        .expect("expected digest should be valid"),
                ),
                None,
                vec![],
            ),
            ProofpackWriterReferencedArtifactVerificationEvidence::optional(
                requirements[1].clone(),
                ProofpackWriterReferencedArtifactVerificationStatus::NotRequired,
                None,
                None,
                vec![],
            ),
        ],
        vec![],
    );
    let boundary = proofpack_writer_hash_reference_integration_model_boundary();

    let vocabulary_ok = PROOFPACK_WRITER_HASH_REFERENCE_INTEGRATION_MODEL_SCHEMA_VERSION
        == "punk.proofpack.writer_hash_reference_integration_model.v0.1"
        && ProofpackWriterHashReferenceIntegrationStatus::Ready.as_str() == "ready"
        && ProofpackWriterDeclaredArtifactDigestStatus::DeclaredInvalidFormat.as_str()
            == "declared_invalid_format"
        && ProofpackWriterReferencedArtifactVerificationStatus::Unverified.as_str() == "unverified"
        && ProofpackWriterHashReferenceIntegrationBlocker::VerificationDigestMismatch.as_str()
            == "verification_digest_mismatch";
    let ready_ok = ready.schema_version()
        == PROOFPACK_WRITER_HASH_REFERENCE_INTEGRATION_MODEL_SCHEMA_VERSION
        && ready.is_ready()
        && ready.blockers().is_empty()
        && ready.declared_digest_surface_is_complete()
        && ready.structural_link_hash_integrity_is_complete()
        && ready.has_referenced_artifact_verification_evidence()
        && ready.has_manifest_self_digest_ready()
        && ready.keeps_evidence_surfaces_separate()
        && ready.operation_outcome() == ProofpackWriterOperationOutcome::PlannedOnly;
    let blocked_ok = blocked.is_blocked()
        && blocked.operation_outcome() == ProofpackWriterOperationOutcome::PreflightFailed
        && blocked.has_blocker(
            ProofpackWriterHashReferenceIntegrationBlocker::MissingDeclaredArtifactDigest,
        )
        && blocked.has_blocker(
            ProofpackWriterHashReferenceIntegrationBlocker::InvalidDeclaredArtifactDigest,
        )
        && blocked.has_blocker(
            ProofpackWriterHashReferenceIntegrationBlocker::VerificationDigestMismatch,
        )
        && blocked.blockers_fail_closed();
    let optional_ok = optional.is_ready()
        && optional.has_optional_or_not_required_verification_evidence()
        && optional.referenced_artifact_verification_evidence()[0].status()
            == ProofpackWriterReferencedArtifactVerificationStatus::Unverified
        && optional.referenced_artifact_verification_evidence()[1].status()
            == ProofpackWriterReferencedArtifactVerificationStatus::NotRequired
        && optional.blockers().is_empty();
    let boundary_ok = boundary.models_hash_reference_integration
        && boundary.consumes_explicit_proofpack_refs
        && boundary.checks_declared_digest_policy
        && boundary.checks_structural_link_hash_integrity
        && boundary.consumes_optional_referenced_artifact_verification_outcomes
        && boundary.keeps_evidence_surfaces_separate
        && boundary.manifest_self_digest_is_readiness_metadata
        && boundary.evidence_only
        && boundary.setup_neutral
        && !boundary.reads_filesystem
        && !boundary.touches_filesystem
        && !boundary.computes_referenced_artifact_hashes
        && !boundary.verifies_referenced_artifacts
        && !boundary.broad_file_hashing
        && !boundary.canonicalizes_host_paths
        && !ready.reads_filesystem()
        && !ready.touches_filesystem()
        && !ready.computes_referenced_artifact_hashes()
        && !ready.verifies_referenced_artifacts()
        && !ready.broad_file_hashing()
        && !ready.writes_proofpack()
        && !ready.writes_punk_proofs()
        && !ready.writes_writer_operation_evidence()
        && !ready.persists_operation_evidence()
        && !ready.writes_indexes_or_latest()
        && !ready.writes_final_decision()
        && !ready.creates_acceptance_claim()
        && !ready.requires_runtime_storage()
        && !ready.writes_cli_output()
        && !ready.exposes_cli_behavior()
        && !ready.writes_schema_files()
        && !ready.can_claim_acceptance_by_itself();

    if vocabulary_ok && ready_ok && blocked_ok && optional_ok && boundary_ok {
        SmokeEvalCaseResult::pass(
            "eval_proofpack_writer_hash_reference_integration_model_is_side_effect_free",
            "proofpack writer hash/reference integration model stays side-effect free",
            "hash/reference integration composes declared digest, structural integrity, optional verification, and manifest self-digest evidence without reading files, writing proofpacks, or claiming acceptance",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_proofpack_writer_hash_reference_integration_model_is_side_effect_free",
            "proofpack writer hash/reference integration model stays side-effect free",
            format!(
                "hash/reference integration drifted; vocabulary={vocabulary_ok} ready={ready_ok} blocked={blocked_ok} optional={optional_ok} boundary={boundary_ok} ready_status={:?} ready_blockers={:?} blocked_status={:?} blocked_blockers={:?}",
                ready.status(),
                ready.blockers(),
                blocked.status(),
                blocked.blockers(),
            ),
        )
    }
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

fn eval_artifact_hash_computation_matches_known_vectors() -> SmokeEvalCaseResult {
    let empty = compute_artifact_digest(b"");
    let abc = compute_artifact_digest(b"abc");

    let empty_matches =
        empty.as_str() == "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    let abc_matches =
        abc.as_str() == "sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
    let both_canonical = validate_artifact_digest(empty.as_str()).is_ok()
        && validate_artifact_digest(abc.as_str()).is_ok()
        && is_canonical_artifact_digest(empty.as_str())
        && is_canonical_artifact_digest(abc.as_str());

    if empty_matches && abc_matches && both_canonical {
        SmokeEvalCaseResult::pass(
            "eval_artifact_hash_computation_matches_known_vectors",
            "exact-byte artifact hash computation matches known vectors",
            "empty bytes and abc compute canonical sha256-prefixed lowercase digest metadata",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_artifact_hash_computation_matches_known_vectors",
            "exact-byte artifact hash computation matches known vectors",
            format!(
                "hash computation drifted; empty={} empty_ok={} abc={} abc_ok={} canonical={both_canonical}",
                empty.as_str(),
                empty_matches,
                abc.as_str(),
                abc_matches
            ),
        )
    }
}

fn eval_artifact_hash_computation_preserves_exact_bytes() -> SmokeEvalCaseResult {
    let unix_line = compute_artifact_digest(b"line\n");
    let windows_line = compute_artifact_digest(b"line\r\n");
    let trailing_space = compute_artifact_digest(b"line\n ");
    let capabilities = ARTIFACT_HASH_POLICY_CAPABILITIES;

    let distinct =
        unix_line != windows_line && unix_line != trailing_space && windows_line != trailing_space;

    if distinct
        && capabilities.computes_hashes
        && !capabilities.normalizes_artifact_bytes
        && !capabilities.writes_runtime_state
    {
        SmokeEvalCaseResult::pass(
            "eval_artifact_hash_computation_preserves_exact_bytes",
            "artifact hash computation preserves exact bytes",
            "different newline and whitespace bytes produce different digests without normalization or runtime writes",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_artifact_hash_computation_preserves_exact_bytes",
            "artifact hash computation preserves exact bytes",
            format!(
                "exact-byte hash boundary drifted; distinct={} computes={} normalizes={} writes={}",
                distinct,
                capabilities.computes_hashes,
                capabilities.normalizes_artifact_bytes,
                capabilities.writes_runtime_state
            ),
        )
    }
}

fn eval_file_artifact_hashing_helper_hashes_explicit_regular_file() -> SmokeEvalCaseResult {
    let temp_path = unique_smoke_temp_path();
    let artifacts_dir = temp_path.join("artifacts");
    let file_path = artifacts_dir.join("result.txt");
    let dir_path = artifacts_dir.join("dir");

    let setup_result = fs::create_dir_all(&dir_path)
        .and_then(|_| fs::write(&file_path, b"line\n"))
        .map(|_| ());

    if let Err(error) = setup_result {
        let _ = fs::remove_dir_all(&temp_path);
        return SmokeEvalCaseResult::fail(
            "eval_file_artifact_hashing_helper_hashes_explicit_regular_file",
            "file IO artifact hashing helper hashes explicit regular files",
            format!("temporary artifact setup failed with {error:?}"),
        );
    }

    let repo_root = match RepoRoot::new(temp_path.clone()) {
        Ok(repo_root) => repo_root,
        Err(error) => {
            let _ = fs::remove_dir_all(&temp_path);
            return SmokeEvalCaseResult::fail(
                "eval_file_artifact_hashing_helper_hashes_explicit_regular_file",
                "file IO artifact hashing helper hashes explicit regular files",
                format!("temporary repo root was rejected with {error:?}"),
            );
        }
    };
    let file_ref =
        RepoRelativeArtifactRef::new("artifacts/result.txt").expect("file ref should be valid");
    let missing_ref =
        RepoRelativeArtifactRef::new("artifacts/missing.txt").expect("missing ref should be valid");
    let dir_ref = RepoRelativeArtifactRef::new("artifacts/dir").expect("dir ref should be valid");

    let digest = compute_artifact_file_digest(&repo_root, &file_ref);
    let expected = compute_artifact_digest(b"line\n");
    let digest_with_normalized_line = compute_artifact_digest(b"line\r\n");
    let missing = compute_artifact_file_digest(&repo_root, &missing_ref);
    let directory = compute_artifact_file_digest(&repo_root, &dir_ref);
    let invalid_ref_rejected = RepoRelativeArtifactRef::new("../outside.txt")
        == Err(ArtifactHashPolicyError::InvalidArtifactRefSegment);
    let relative_root_rejected =
        RepoRoot::new("relative/repo") == Err(FileArtifactHashError::RelativeRepoRoot);
    let capabilities = FILE_ARTIFACT_HASHING_CAPABILITIES;
    let cleanup_ok = fs::remove_dir_all(&temp_path).is_ok();

    let digest_ok = digest.as_ref().is_ok_and(|digest| {
        digest == &expected
            && digest != &digest_with_normalized_line
            && validate_artifact_digest(digest.as_str()).is_ok()
            && is_canonical_artifact_digest(digest.as_str())
    });
    let missing_explicit = missing == Err(FileArtifactHashError::Missing);
    let directory_rejected = directory == Err(FileArtifactHashError::NotRegularFile);
    let boundary_ok = capabilities.computes_file_artifact_digests
        && capabilities.reads_regular_files
        && !capabilities.follows_symlinks
        && !capabilities.scans_directories
        && !capabilities.verifies_referenced_artifact_bytes
        && !capabilities.normalizes_artifact_bytes
        && !capabilities.writes_runtime_state
        && !capabilities.requires_runtime_storage
        && !capabilities.writes_proofpack
        && !capabilities.writes_cli_output
        && !capabilities.creates_acceptance_claim;

    if digest_ok
        && missing_explicit
        && directory_rejected
        && invalid_ref_rejected
        && relative_root_rejected
        && boundary_ok
        && cleanup_ok
    {
        SmokeEvalCaseResult::pass(
            "eval_file_artifact_hashing_helper_hashes_explicit_regular_file",
            "file IO artifact hashing helper hashes explicit regular files",
            "explicit repo root and repo-relative ref produced canonical exact-byte digest evidence while missing files and directories stayed non-passing",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_file_artifact_hashing_helper_hashes_explicit_regular_file",
            "file IO artifact hashing helper hashes explicit regular files",
            format!(
                "file IO hash helper drifted; digest_ok={} missing={} directory={} invalid_ref={} relative_root={} boundary_ok={} cleanup_ok={} digest={:?} missing_result={:?} directory_result={:?}",
                digest_ok,
                missing_explicit,
                directory_rejected,
                invalid_ref_rejected,
                relative_root_rejected,
                boundary_ok,
                cleanup_ok,
                digest.as_ref().map(ArtifactDigest::as_str),
                missing,
                directory
            ),
        )
    }
}

fn eval_referenced_artifact_verification_helper_compares_expected_digest() -> SmokeEvalCaseResult {
    let temp_path = unique_smoke_temp_path();
    let artifacts_dir = temp_path.join("artifacts");
    let file_path = artifacts_dir.join("result.txt");
    let dir_path = artifacts_dir.join("dir");

    let setup_result = fs::create_dir_all(&dir_path)
        .and_then(|_| fs::write(&file_path, b"line\n"))
        .map(|_| ());

    if let Err(error) = setup_result {
        let _ = fs::remove_dir_all(&temp_path);
        return SmokeEvalCaseResult::fail(
            "eval_referenced_artifact_verification_helper_compares_expected_digest",
            "referenced artifact verification helper compares expected and observed digests",
            format!("temporary artifact setup failed with {error:?}"),
        );
    }

    let repo_root = match RepoRoot::new(temp_path.clone()) {
        Ok(repo_root) => repo_root,
        Err(error) => {
            let _ = fs::remove_dir_all(&temp_path);
            return SmokeEvalCaseResult::fail(
                "eval_referenced_artifact_verification_helper_compares_expected_digest",
                "referenced artifact verification helper compares expected and observed digests",
                format!("temporary repo root was rejected with {error:?}"),
            );
        }
    };
    let file_ref =
        RepoRelativeArtifactRef::new("artifacts/result.txt").expect("file ref should be valid");
    let missing_ref =
        RepoRelativeArtifactRef::new("artifacts/missing.txt").expect("missing ref should be valid");
    let dir_ref = RepoRelativeArtifactRef::new("artifacts/dir").expect("dir ref should be valid");

    let expected = compute_artifact_digest(b"line\n");
    let different = compute_artifact_digest(b"line\r\n");
    let verified = verify_referenced_artifact_digest(&repo_root, &file_ref, &expected);
    let mismatch = verify_referenced_artifact_digest(&repo_root, &file_ref, &different);
    let missing = verify_referenced_artifact_digest(&repo_root, &missing_ref, &expected);
    let directory = verify_referenced_artifact_digest(&repo_root, &dir_ref, &expected);
    let invalid_ref_rejected = RepoRelativeArtifactRef::new("../outside.txt")
        == Err(ArtifactHashPolicyError::InvalidArtifactRefSegment);
    let invalid_digest_rejected = ArtifactDigest::new("sha256:ABCDEF")
        == Err(ArtifactHashPolicyError::InvalidDigestLength {
            expected: 64,
            actual: 6,
        });
    let capabilities = REFERENCED_ARTIFACT_VERIFICATION_CAPABILITIES;
    let cleanup_ok = fs::remove_dir_all(&temp_path).is_ok();

    let verified_ok = verified.is_verified()
        && verified.outcome() == ReferencedArtifactVerificationOutcome::Verified
        && verified.outcome().as_str() == "verified"
        && verified.artifact_ref() == &file_ref
        && verified.expected_digest() == &expected
        && verified.observed_digest() == Some(&expected);
    let mismatch_ok = mismatch.outcome() == ReferencedArtifactVerificationOutcome::DigestMismatch
        && mismatch.outcome().as_str() == "digest_mismatch"
        && mismatch.expected_digest() == &different
        && mismatch.observed_digest() == Some(&expected);
    let missing_ok = missing.outcome() == ReferencedArtifactVerificationOutcome::Missing
        && missing.observed_digest().is_none();
    let directory_ok = directory.outcome() == ReferencedArtifactVerificationOutcome::NotRegularFile
        && directory.observed_digest().is_none();
    let boundary_ok = capabilities.verifies_referenced_artifact_bytes
        && capabilities.reads_regular_files
        && !capabilities.follows_symlinks
        && !capabilities.scans_directories
        && !capabilities.normalizes_artifact_bytes
        && !capabilities.writes_runtime_state
        && !capabilities.requires_runtime_storage
        && !capabilities.writes_proofpack
        && !capabilities.writes_gate_decision
        && !capabilities.writes_cli_output
        && !capabilities.creates_acceptance_claim;

    if verified_ok
        && mismatch_ok
        && missing_ok
        && directory_ok
        && invalid_ref_rejected
        && invalid_digest_rejected
        && boundary_ok
        && cleanup_ok
    {
        SmokeEvalCaseResult::pass(
            "eval_referenced_artifact_verification_helper_compares_expected_digest",
            "referenced artifact verification helper compares expected and observed digests",
            "explicit repo root, repo-relative ref, and canonical expected digest produced evidence-only verified, mismatch, missing, and non-regular outcomes without proof or runtime authority",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_referenced_artifact_verification_helper_compares_expected_digest",
            "referenced artifact verification helper compares expected and observed digests",
            format!(
                "referenced verification drifted; verified_ok={} mismatch_ok={} missing_ok={} directory_ok={} invalid_ref={} invalid_digest={} boundary_ok={} cleanup_ok={} verified={:?} mismatch={:?} missing={:?} directory={:?}",
                verified_ok,
                mismatch_ok,
                missing_ok,
                directory_ok,
                invalid_ref_rejected,
                invalid_digest_rejected,
                boundary_ok,
                cleanup_ok,
                verified,
                mismatch,
                missing,
                directory
            ),
        )
    }
}

fn eval_artifact_hash_policy_helper_boundary_flags() -> SmokeEvalCaseResult {
    let capabilities = ARTIFACT_HASH_POLICY_CAPABILITIES;
    let file_capabilities = FILE_ARTIFACT_HASHING_CAPABILITIES;
    let verification_capabilities = REFERENCED_ARTIFACT_VERIFICATION_CAPABILITIES;

    if capabilities.validates_digest_format
        && capabilities.validates_repo_relative_refs
        && capabilities.computes_hashes
        && !capabilities.normalizes_artifact_bytes
        && !capabilities.writes_runtime_state
        && file_capabilities.computes_file_artifact_digests
        && !file_capabilities.verifies_referenced_artifact_bytes
        && verification_capabilities.verifies_referenced_artifact_bytes
        && !verification_capabilities.normalizes_artifact_bytes
        && !verification_capabilities.writes_runtime_state
        && !verification_capabilities.writes_proofpack
        && !verification_capabilities.writes_gate_decision
        && !verification_capabilities.writes_cli_output
        && !verification_capabilities.creates_acceptance_claim
        && !file_capabilities.normalizes_artifact_bytes
        && !file_capabilities.writes_runtime_state
        && !file_capabilities.writes_proofpack
        && !file_capabilities.writes_cli_output
        && !file_capabilities.creates_acceptance_claim
    {
        SmokeEvalCaseResult::pass(
            "eval_artifact_hash_policy_helper_boundary_flags",
            "artifact hash helpers stay bounded",
            "helper capabilities validate digest/ref shape, compute exact-byte and file artifact hashes, and compare referenced artifact digests without byte normalization, proofpack writes, final-decision writes, or runtime writes",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_artifact_hash_policy_helper_boundary_flags",
            "artifact hash helpers stay bounded",
            format!(
                "artifact hash helper boundary drifted; digest={} refs={} computes={} file_computes={} file_verifies_refs={} verifies_refs={} normalizes={} writes={} proofpack={} gate={} cli={} acceptance={}",
                capabilities.validates_digest_format,
                capabilities.validates_repo_relative_refs,
                capabilities.computes_hashes,
                file_capabilities.computes_file_artifact_digests,
                file_capabilities.verifies_referenced_artifact_bytes,
                verification_capabilities.verifies_referenced_artifact_bytes,
                capabilities.normalizes_artifact_bytes,
                capabilities.writes_runtime_state,
                verification_capabilities.writes_proofpack,
                verification_capabilities.writes_gate_decision,
                verification_capabilities.writes_cli_output,
                verification_capabilities.creates_acceptance_claim
            ),
        )
    }
}

fn unique_smoke_temp_path() -> std::path::PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();
    let counter = SMOKE_TEMP_PATH_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "punk-eval-file-hash-smoke-{}-{}-{}",
        process::id(),
        unique,
        counter
    ))
}

fn count_regular_files(root: &std::path::Path) -> usize {
    let mut count = 0;
    let mut stack = vec![root.to_path_buf()];
    while let Some(path) = stack.pop() {
        let Ok(metadata) = fs::symlink_metadata(&path) else {
            continue;
        };
        if metadata.file_type().is_dir() {
            let Ok(entries) = fs::read_dir(&path) else {
                continue;
            };
            for entry in entries.flatten() {
                stack.push(entry.path());
            }
        } else if metadata.file_type().is_file() {
            count += 1;
        }
    }
    count
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
        assert_eq!(report.runtime_persistence(), "local-event-log-writer");
        assert_eq!(report.report_storage(), "inactive");
        assert_eq!(report.cases().len(), 158);
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
        assert!(rendered.contains("runtime_persistence: local-event-log-writer"));
        assert!(rendered.contains("report_storage: inactive"));
        assert!(rendered.contains("smoke_result: pass"));
        assert!(rendered.contains(
            "assessment: local deterministic smoke harness passed over current contract, contract schema blueprint model, user intent-to-contract draft model, contract draft confirmation boundary model, hard clause mapping model, contract receipt requirements model, contract gate input policy model, contract proof requirements model, flow, receipt, event, local event writer, instruction page-index model, publishing locate resolver, PubPunk inventory assessment model, module-host invocation envelope, module-host receipt proposal model, module-host side-effect request proposal model, module-host policy gate preflight model, module-host side-effect receipt writer preflight model, module-host side-effect receipt writer active behavior model, greenfield and brownfield project init scaffolds, brownfield source corpus manifest side-effect-free model, brownfield source corpus manifest writer preflight model, brownfield source corpus manifest writer first slice, gate, proof, proofpack manifest renderer, proofpack manifest digest helper, proofpack writer canonical artifact model, proofpack writer target artifact ref policy model, proofpack writer operation evidence model, proofpack writer preflight plan model, proofpack writer file IO plan model, proofpack writer file IO outcome model, proofpack writer file IO error reason model, proofpack writer target path policy model, proofpack writer preflight integration model, proofpack writer active behavior model, proofpack writer host path resolution model, proofpack writer concrete path/storage policy model, proofpack writer first active write slice, proofpack writer hash/reference integration model, artifact hash policy, exact-byte hash computation helper, file IO artifact hashing helper, and referenced artifact verification helper kernels"
        ));
        assert!(rendered.contains("case_results:"));
        assert!(rendered.contains("  - id: eval_flow_allows_approval_transition"));
        assert!(rendered.contains("  - id: eval_local_flow_event_writer_appends_project_event"));
        assert!(rendered
            .contains("  - id: eval_instruction_page_index_model_is_deterministic_and_advisory"));
        assert!(rendered.contains(
            "  - id: eval_publishing_locate_resolves_local_pointer_without_side_effects"
        ));
        assert!(rendered
            .contains("  - id: eval_pubpunk_inventory_assessment_model_is_side_effect_free"));
        assert!(rendered
            .contains("  - id: eval_module_host_wraps_pubpunk_assessment_without_side_effects"));
        assert!(rendered
            .contains("  - id: eval_module_host_receipt_proposal_model_is_side_effect_free"));
        assert!(rendered.contains(
            "  - id: eval_module_host_side_effect_request_proposal_model_is_side_effect_free"
        ));
        assert!(rendered
            .contains("  - id: eval_module_host_policy_gate_preflight_model_is_side_effect_free"));
        assert!(rendered.contains(
            "  - id: eval_module_host_side_effect_receipt_writer_preflight_model_is_side_effect_free"
        ));
        assert!(rendered.contains(
            "  - id: eval_module_host_side_effect_receipt_writer_active_behavior_model_is_side_effect_free"
        ));
        assert!(
            rendered.contains("  - id: eval_project_init_creates_level0_manual_memory_scaffold")
        );
        assert!(rendered.contains("  - id: eval_project_init_brownfield_scaffold_shape"));
        assert!(rendered.contains("  - id: eval_project_init_refuses_to_overwrite_existing_memory"));
        assert!(rendered.contains("  - id: eval_project_init_conflict_is_atomic_noop"));
        assert!(rendered.contains("  - id: eval_source_corpus_manifest_model_is_side_effect_free"));
        assert!(rendered.contains(
            "  - id: eval_source_corpus_manifest_writer_preflight_model_is_side_effect_free"
        ));
        assert!(rendered.contains(
            "  - id: eval_source_corpus_manifest_writer_first_slice_writes_prepared_manifest"
        ));
        assert!(rendered.contains("  - id: eval_contract_ready_for_bounded_work_allows_start_run"));
        assert!(rendered.contains("  - id: eval_contract_receipt_allowed_path_produces_evidence"));
        assert!(rendered
            .contains("  - id: eval_contract_schema_blueprint_has_required_top_level_sections"));
        assert!(rendered.contains(
            "  - id: eval_contract_schema_blueprint_marks_status_required_now_deferred_parked_future"
        ));
        assert!(
            rendered.contains("  - id: eval_contract_status_does_not_include_acceptance_decisions")
        );
        assert!(rendered.contains(
            "  - id: eval_hard_clause_requires_validator_receipt_proof_or_gate_review_mapping"
        ));
        assert!(rendered.contains("  - id: eval_hard_clause_with_validator_ref_is_mapped"));
        assert!(
            rendered.contains("  - id: eval_hard_clause_with_no_mapping_is_unsupported_blocking")
        );
        assert!(rendered.contains("  - id: eval_unsupported_hard_clause_blocks_approved_for_run"));
        assert!(rendered.contains(
            "  - id: eval_ready_confirmed_draft_with_unmapped_hard_clause_cannot_approve"
        ));
        assert!(rendered.contains("  - id: eval_hard_clause_mapping_does_not_invoke_writer"));
        assert!(rendered
            .contains("  - id: eval_receipt_requirement_covers_artifact_hashes_hard_clause"));
        assert!(rendered
            .contains("  - id: eval_receipt_requirement_covers_validator_results_hard_clause"));
        assert!(
            rendered.contains("  - id: eval_missing_receipt_requirement_blocks_approved_for_run")
        );
        assert!(rendered.contains("  - id: eval_unknown_receipt_requirement_field_is_blocking"));
        assert!(
            rendered.contains("  - id: eval_duplicate_receipt_requirements_are_non_conflicting")
        );
        assert!(rendered
            .contains("  - id: eval_executor_claims_receipt_field_remains_unverified_evidence"));
        assert!(rendered.contains("  - id: eval_receipt_requirements_do_not_create_run_receipt"));
        assert!(rendered.contains("  - id: eval_receipt_requirements_do_not_invoke_writer"));
        assert!(rendered.contains(
            "  - id: eval_contract_status_excludes_acceptance_outcomes_after_receipt_requirements"
        ));
        assert!(rendered.contains("  - id: eval_proof_requirements_require_contract_ref"));
        assert!(rendered.contains("  - id: eval_proof_requirements_require_run_receipt_ref"));
        assert!(rendered.contains("  - id: eval_proof_requirements_require_gate_decision_ref"));
        assert!(rendered.contains("  - id: eval_unsupported_proof_target_is_blocking"));
        assert!(rendered.contains(
            "  - id: eval_proof_requirements_do_not_compute_artifact_hashes_from_filesystem"
        ));
        assert!(rendered.contains("  - id: eval_proof_requirements_do_not_create_proofpack"));
        assert!(rendered.contains("  - id: eval_proofpack_is_not_required_before_gate_decision"));
        assert!(rendered.contains(
            "  - id: eval_contract_status_excludes_acceptance_outcomes_after_proof_requirements"
        ));
        assert!(rendered.contains("  - id: eval_gate_policy_does_not_write_decision"));
        assert!(rendered.contains("  - id: eval_gate_input_policy_requires_contract_ref"));
        assert!(
            rendered.contains("  - id: eval_gate_input_policy_requires_approved_for_run_status")
        );
        assert!(rendered.contains("  - id: eval_gate_input_policy_requires_run_receipt_ref"));
        assert!(rendered
            .contains("  - id: eval_gate_input_policy_requires_receipt_requirement_coverage"));
        assert!(rendered
            .contains("  - id: eval_gate_input_policy_requires_hard_clause_mapping_assessment"));
        assert!(rendered.contains("  - id: eval_missing_contract_ref_blocks_gate_readiness"));
        assert!(rendered.contains("  - id: eval_missing_run_receipt_ref_blocks_gate_readiness"));
        assert!(rendered.contains("  - id: eval_unsupported_gate_input_blocks_gate_readiness"));
        assert!(
            rendered.contains("  - id: eval_gate_input_policy_does_not_require_existing_proofpack")
        );
        assert!(rendered.contains("  - id: eval_approved_for_run_is_not_ready_for_gate"));
        assert!(rendered.contains("  - id: eval_writer_is_not_upstream_contract_authority"));
        assert!(rendered
            .contains("  - id: eval_user_intent_contract_draft_model_ready_for_confirmation"));
        assert!(rendered
            .contains("  - id: eval_user_intent_contract_draft_model_requires_clarification"));
        assert!(
            rendered.contains("  - id: eval_user_intent_contract_draft_model_refuses_or_defers")
        );
        assert!(rendered
            .contains("  - id: eval_user_intent_contract_draft_model_blocks_missing_evidence"));
        assert!(rendered
            .contains("  - id: eval_user_intent_contract_draft_model_keeps_writer_downstream"));
        assert!(rendered.contains(
            "  - id: eval_ready_draft_with_explicit_confirmation_becomes_approved_for_run"
        ));
        assert!(rendered
            .contains("  - id: eval_ready_draft_without_explicit_confirmation_is_not_approved"));
        assert!(rendered.contains("  - id: eval_clarification_required_draft_cannot_be_approved"));
        assert!(rendered.contains("  - id: eval_refused_or_deferred_draft_cannot_be_approved"));
        assert!(rendered.contains("  - id: eval_blocked_draft_cannot_be_approved"));
        assert!(rendered.contains("  - id: eval_unresolved_unknowns_block_approval"));
        assert!(rendered.contains("  - id: eval_unknowns_converted_to_assumptions_allow_approval"));
        assert!(rendered.contains("  - id: eval_approved_for_run_preserves_scope_and_non_scope"));
        assert!(rendered.contains("  - id: eval_approved_for_run_preserves_evidence_plan"));
        assert!(rendered.contains("  - id: eval_approved_for_run_preserves_side_effect_boundaries"));
        assert!(rendered.contains("  - id: eval_user_confirmation_does_not_create_gate_decision"));
        assert!(rendered.contains("  - id: eval_user_confirmation_does_not_create_proofpack"));
        assert!(rendered.contains("  - id: eval_user_confirmation_does_not_invoke_writer"));
        assert!(
            rendered.contains("  - id: eval_contract_status_still_excludes_acceptance_outcomes")
        );
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
        assert!(rendered.contains(
            "  - id: eval_proofpack_manifest_renderer_is_deterministic_and_side_effect_free"
        ));
        assert!(rendered
            .contains("  - id: eval_proofpack_manifest_digest_matches_exact_renderer_bytes"));
        assert!(rendered.contains(
            "  - id: eval_proofpack_writer_canonical_artifact_model_is_side_effect_free"
        ));
        assert!(rendered.contains(
            "  - id: eval_proofpack_writer_target_artifact_ref_policy_model_is_side_effect_free"
        ));
        assert!(rendered.contains(
            "  - id: eval_proofpack_writer_operation_evidence_model_is_side_effect_free"
        ));
        assert!(rendered
            .contains("  - id: eval_proofpack_writer_preflight_plan_model_is_side_effect_free"));
        assert!(rendered
            .contains("  - id: eval_proofpack_writer_file_io_plan_model_is_side_effect_free"));
        assert!(rendered
            .contains("  - id: eval_proofpack_writer_file_io_outcome_model_is_side_effect_free"));
        assert!(rendered.contains(
            "  - id: eval_proofpack_writer_file_io_error_reason_model_is_side_effect_free"
        ));
        assert!(rendered.contains(
            "  - id: eval_proofpack_writer_target_path_policy_model_is_side_effect_free"
        ));
        assert!(rendered.contains(
            "  - id: eval_proofpack_writer_preflight_integration_model_is_side_effect_free"
        ));
        assert!(rendered
            .contains("  - id: eval_proofpack_writer_active_behavior_model_is_side_effect_free"));
        assert!(rendered.contains(
            "  - id: eval_proofpack_writer_host_path_resolution_model_is_side_effect_free"
        ));
        assert!(rendered.contains(
            "  - id: eval_proofpack_writer_concrete_path_storage_policy_model_is_side_effect_free"
        ));
        assert!(rendered
            .contains("  - id: eval_proofpack_writer_first_active_write_slice_writes_exact_bytes"));
        assert!(rendered.contains(
            "  - id: eval_proofpack_writer_hash_reference_integration_model_is_side_effect_free"
        ));
        assert!(rendered.contains("  - id: eval_artifact_hash_policy_accepts_canonical_digest"));
        assert!(rendered.contains("  - id: eval_artifact_hash_policy_rejects_invalid_digest"));
        assert!(rendered.contains("  - id: eval_artifact_hash_policy_accepts_repo_relative_ref"));
        assert!(rendered.contains("  - id: eval_artifact_hash_policy_rejects_invalid_ref"));
        assert!(rendered.contains("  - id: eval_artifact_hash_computation_matches_known_vectors"));
        assert!(rendered.contains("  - id: eval_artifact_hash_computation_preserves_exact_bytes"));
        assert!(rendered
            .contains("  - id: eval_file_artifact_hashing_helper_hashes_explicit_regular_file"));
        assert!(rendered.contains(
            "  - id: eval_referenced_artifact_verification_helper_compares_expected_digest"
        ));
        assert!(rendered.contains("  - id: eval_artifact_hash_policy_helper_boundary_flags"));
        assert!(rendered.contains("    status: pass"));
        assert!(rendered.contains("notes:"));
        assert!(rendered.contains("local assessment only; no authority is written here"));
        assert!(rendered.contains(
            "instruction page-index smoke case renders a deterministic advisory tree from source instruction refs without creating .punk/views, summaries, vector DBs, hidden truth stores, LLM calls, or runtime side effects"
        ));
        assert!(rendered.contains(
            "publishing locate smoke case reads only .punk/publishing.toml and .punk/publishing.local.toml under an explicit temporary project root and invokes no publishing, browser, network API, credential, adapter, bot, or file-write behavior"
        ));
        assert!(rendered.contains(
            "PubPunk inventory assessment smoke case is module-owned, side-effect-free, no-IO, no-CLI, advisory-only, and does not publish, create receipts, read credentials, invoke adapters, or write gate/proof authority"
        ));
        assert!(rendered.contains(
            "module-host invocation envelope smoke case wraps advisory module output only and does not load plugins, invoke modules, expose CLI behavior, read or write files, create receipts, mutate event logs, call APIs, read credentials, invoke adapters, publish, or write gate/proof authority"
        ));
        assert!(rendered.contains(
            "module-host receipt proposal smoke case models future module receipt fields only and does not write receipts, mutate event logs, read or write files, call APIs, read credentials, invoke modules or adapters, publish, or write gate/proof authority"
        ));
        assert!(rendered.contains(
            "module-host side-effect request proposal smoke case models future external action preconditions only and does not invoke adapters, publish, comment, create pull requests, write receipts, mutate event logs, call APIs, read credentials, or write gate/proof authority"
        ));
        assert!(rendered.contains(
            "module-host policy gate preflight smoke case models future policy evidence readiness only and does not invoke policy engines, invoke gate, write decisions, invoke adapters, publish, comment, create pull requests, write receipts, mutate event logs, call APIs, read credentials, or write proof authority"
        ));
        assert!(rendered.contains(
            "module-host side-effect receipt writer preflight smoke case models future receipt writer readiness only and does not write receipts, mutate event logs, read or write files, invoke adapters, invoke policy engines, invoke gate, publish, comment, create pull requests, call APIs, read credentials, write proofpacks, or claim acceptance"
        ));
        assert!(rendered.contains(
            "module-host side-effect receipt writer active behavior smoke case models planned, written, idempotent, conflict, write-failed, and partial receipt outcomes without writing receipts, mutating event logs, persisting operation evidence, reading or writing files, invoking adapters, invoking policy engines, invoking gate, publishing, commenting, creating pull requests, calling APIs, reading credentials, writing proofpacks, or claiming acceptance"
        ));
        assert!(rendered.contains(
            "greenfield init smoke cases create compact .punk/memory project-memory scaffold files plus thin .punk/instructions entrypoints with project_id, entry_mode, and .punk marker files without root-level Punk memory dirs, brownfield reconstruction, grayfield reconciliation, network behavior, .punk runtime stores, .punk/views, contracts, receipts, gate artifacts, proofpacks, or acceptance claims"
        ));
        assert!(rendered.contains(
            "brownfield init smoke case creates only an advisory .punk/memory/reconstruction workspace plus thin .punk/instructions entrypoints with reconstruction_status not_started and no repo scan, AI summary, contracts, claims, runtime stores, .punk/views, gate artifacts, proofpacks, or acceptance claims"
        ));
        assert!(rendered.contains(
            "brownfield source corpus manifest model smoke case is side-effect-free and does not scan repositories, walk files, read file contents, compute file hashes, write manifests, create claims, infer intent, use network, or use remote AI"
        ));
        assert!(rendered.contains(
            "brownfield source corpus manifest writer preflight model smoke case is side-effect-free and models target/path/conflict/claim/runtime blockers without scanning repositories, walking files, reading contents, computing filesystem hashes, writing manifests, generating manifests, creating claims, using network, or using remote AI"
        ));
        assert!(rendered.contains(
            "brownfield source corpus manifest writer first slice smoke case writes deterministic canonical bytes from an already-constructed model to one explicit safe target after preflight, without scanning repositories, walking files, reading source contents, computing source file hashes, creating claims, activating runtime storage, or promoting authority"
        ));
        assert!(rendered.contains(
            "contract schema blueprint smoke cases preserve target shape, field status split, clause mapping, gate input policy, proof requirements, and Writer authority boundaries without runtime activation"
        ));
        assert!(rendered.contains(
            "user intent-to-contract draft model smoke cases classify readiness in memory only and do not create contracts, runtime storage, CLI behavior, gate-writing behavior, proofpacks, or Writer behavior"
        ));
        assert!(rendered.contains(
            "contract draft confirmation smoke cases require explicit user confirmation before approved_for_run model state without runtime storage, CLI behavior, gate-writing behavior, proofpacks, or Writer behavior"
        ));
        assert!(rendered.contains(
            "hard clause mapping smoke cases require hard clauses to have validator, receipt, proof, or explicit human review paths before approved_for_run model state without runtime validator execution, storage, CLI, proofpacks, or Writer behavior"
        ));
        assert!(rendered.contains(
            "contract receipt requirements smoke cases connect hard-clause receipt-field mappings to future run receipt fields without writing receipts, runtime storage, validator execution, gate-writing behavior, proofpacks, or Writer behavior"
        ));
        assert!(rendered.contains(
            "contract gate input policy smoke cases distinguish approved_for_run from ready_for_gate and require gate inputs without proofpack-as-input, runtime gate writing, proofpack creation, acceptance claims, validator execution, storage, CLI, or Writer behavior"
        ));
        assert!(rendered.contains(
            "contract proof requirements smoke cases declare future proofpack links and hashes after gate outcome without proofpack writing, artifact hash runtime, .punk/proofs storage, gate-writing behavior, acceptance claims, or Writer behavior"
        ));
        assert!(rendered
            .contains("run receipt evidence remains pre-gate and does not imply final acceptance"));
        assert!(rendered.contains(
            "gate/proof smoke cases remain local assessment and do not claim acceptance"
        ));
        assert!(rendered.contains(
            "proofpack manifest renderer smoke case renders in memory only and does not write proofpacks"
        ));
        assert!(rendered.contains(
            "proofpack manifest digest smoke case hashes exact in-memory renderer bytes only and does not verify referenced artifacts"
        ));
        assert!(rendered.contains(
            "proofpack writer canonical artifact model smoke case keeps exact manifest bytes canonical and surrounding metadata non-canonical without writer side effects"
        ));
        assert!(rendered.contains(
            "proofpack writer target artifact ref policy smoke case requires proofpack id plus manifest self-digest and renders logical non-path refs without writer side effects"
        ));
        assert!(rendered.contains(
            "proofpack writer operation evidence smoke case models writer outcomes without writing proofpacks or acceptance claims"
        ));
        assert!(rendered.contains(
            "proofpack writer preflight plan smoke case models writer-ready plans without attempting side effects"
        ));
        assert!(rendered.contains(
            "proofpack writer file IO plan smoke case models storage roots, target paths, write policy, idempotency, and rollback visibility without filesystem side effects"
        ));
        assert!(rendered.contains(
            "proofpack writer file IO outcome smoke case maps explicit observations to operation evidence without filesystem, storage, CLI, schema, or acceptance side effects"
        ));
        assert!(rendered.contains(
            "proofpack writer file IO error reason smoke case models stable diagnostic reason codes without filesystem, storage, CLI, schema, or acceptance side effects"
        ));
        assert!(rendered.contains(
            "proofpack writer target path policy smoke case rejects path injection and escape refs without filesystem, storage, CLI, schema, or acceptance side effects"
        ));
        assert!(rendered.contains(
            "proofpack writer preflight integration smoke case composes explicit model inputs and fail-closed blockers without filesystem, storage, CLI, schema, or acceptance side effects"
        ));
        assert!(rendered.contains(
            "proofpack writer active behavior smoke case models selected, attempted, completed, and failed side effects without filesystem, storage, CLI, schema, or acceptance side effects"
        ));
        assert!(rendered.contains(
            "proofpack writer host path resolution smoke case models explicit host path observations and fail-closed blockers without filesystem, storage, CLI, schema, or acceptance side effects"
        ));
        assert!(rendered.contains(
            "proofpack writer concrete path/storage policy smoke case models selected policy refs and host path evidence without filesystem, storage, CLI, schema, or acceptance side effects"
        ));
        assert!(rendered.contains(
            "proofpack writer first active write slice smoke case writes exact bytes only to an explicit temporary target without runtime storage, CLI, schema, persisted evidence, gate, or acceptance side effects"
        ));
        assert!(rendered.contains(
            "proofpack writer hash/reference integration smoke case composes declared digest, structural integrity, optional verification, and manifest self-digest evidence without filesystem, storage, CLI, schema, or acceptance side effects"
        ));
        assert!(rendered.contains(
            "artifact hash smoke cases validate helper shape, exact-byte computation, explicit file IO hashing, and referenced artifact verification without runtime writes"
        ));
        assert!(rendered
            .contains("JSON output is opt-in only and does not imply a stable public contract"));
        assert!(rendered.contains("deferred:"));
        assert!(rendered.contains("baseline, waiver, and stored eval reports are not active"));
        assert!(rendered.contains(
            "runtime contract storage, CLI contract commands, Writer activation, gate writers, and proof writers remain inactive"
        ));
        assert!(rendered.contains(
            "runtime proof storage and .punk/proofs remain inactive; only the explicit first active test-target write slice is active"
        ));
        assert!(rendered.contains(
            "byte normalization and active proofpack writer referenced artifact verification file reads remain inactive; hash/reference integration is evidence-only"
        ));
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
        assert!(rendered.contains("\"runtime_persistence\": \"local-event-log-writer\""));
        assert!(rendered.contains("\"report_storage\": \"inactive\""));
        assert!(rendered.contains("\"case_results\": ["));
        assert!(rendered.contains("\"case_id\": \"eval_flow_allows_approval_transition\""));
        assert!(rendered
            .contains("\"case_id\": \"eval_local_flow_event_writer_appends_project_event\""));
        assert!(rendered
            .contains("\"case_id\": \"eval_source_corpus_manifest_model_is_side_effect_free\""));
        assert!(rendered.contains(
            "\"case_id\": \"eval_source_corpus_manifest_writer_preflight_model_is_side_effect_free\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_source_corpus_manifest_writer_first_slice_writes_prepared_manifest\""
        ));
        assert!(rendered
            .contains("\"case_id\": \"eval_contract_ready_for_bounded_work_allows_start_run\""));
        assert!(rendered
            .contains("\"case_id\": \"eval_contract_receipt_allowed_path_produces_evidence\""));
        assert!(rendered.contains(
            "\"case_id\": \"eval_contract_schema_blueprint_has_required_top_level_sections\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_contract_schema_blueprint_marks_status_required_now_deferred_parked_future\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_contract_status_does_not_include_acceptance_decisions\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_hard_clause_requires_validator_receipt_proof_or_gate_review_mapping\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_receipt_requirement_covers_artifact_hashes_hard_clause\""
        ));
        assert!(rendered
            .contains("\"case_id\": \"eval_missing_receipt_requirement_blocks_approved_for_run\""));
        assert!(rendered
            .contains("\"case_id\": \"eval_unknown_receipt_requirement_field_is_blocking\""));
        assert!(rendered
            .contains("\"case_id\": \"eval_receipt_requirements_do_not_create_run_receipt\""));
        assert!(
            rendered.contains("\"case_id\": \"eval_receipt_requirements_do_not_invoke_writer\"")
        );
        assert!(
            rendered.contains("\"case_id\": \"eval_proof_requirements_do_not_create_proofpack\"")
        );
        assert!(rendered.contains("\"case_id\": \"eval_proof_requirements_require_contract_ref\""));
        assert!(rendered.contains("\"case_id\": \"eval_unsupported_proof_target_is_blocking\""));
        assert!(rendered
            .contains("\"case_id\": \"eval_proofpack_is_not_required_before_gate_decision\""));
        assert!(rendered.contains("\"case_id\": \"eval_gate_policy_does_not_write_decision\""));
        assert!(rendered.contains("\"case_id\": \"eval_gate_input_policy_requires_contract_ref\""));
        assert!(
            rendered.contains("\"case_id\": \"eval_missing_contract_ref_blocks_gate_readiness\"")
        );
        assert!(rendered.contains(
            "\"case_id\": \"eval_gate_input_policy_does_not_require_existing_proofpack\""
        ));
        assert!(rendered.contains("\"case_id\": \"eval_approved_for_run_is_not_ready_for_gate\""));
        assert!(
            rendered.contains("\"case_id\": \"eval_writer_is_not_upstream_contract_authority\"")
        );
        assert!(rendered.contains(
            "\"case_id\": \"eval_user_intent_contract_draft_model_ready_for_confirmation\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_user_intent_contract_draft_model_requires_clarification\""
        ));
        assert!(rendered
            .contains("\"case_id\": \"eval_user_intent_contract_draft_model_refuses_or_defers\""));
        assert!(rendered.contains(
            "\"case_id\": \"eval_user_intent_contract_draft_model_blocks_missing_evidence\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_user_intent_contract_draft_model_keeps_writer_downstream\""
        ));
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
        assert!(rendered.contains(
            "\"case_id\": \"eval_proofpack_manifest_renderer_is_deterministic_and_side_effect_free\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_proofpack_manifest_digest_matches_exact_renderer_bytes\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_proofpack_writer_canonical_artifact_model_is_side_effect_free\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_proofpack_writer_target_artifact_ref_policy_model_is_side_effect_free\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_proofpack_writer_operation_evidence_model_is_side_effect_free\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_proofpack_writer_preflight_plan_model_is_side_effect_free\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_proofpack_writer_file_io_plan_model_is_side_effect_free\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_proofpack_writer_file_io_outcome_model_is_side_effect_free\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_proofpack_writer_file_io_error_reason_model_is_side_effect_free\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_proofpack_writer_target_path_policy_model_is_side_effect_free\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_proofpack_writer_preflight_integration_model_is_side_effect_free\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_proofpack_writer_active_behavior_model_is_side_effect_free\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_proofpack_writer_host_path_resolution_model_is_side_effect_free\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_proofpack_writer_concrete_path_storage_policy_model_is_side_effect_free\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_proofpack_writer_first_active_write_slice_writes_exact_bytes\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_proofpack_writer_hash_reference_integration_model_is_side_effect_free\""
        ));
        assert!(rendered
            .contains("\"case_id\": \"eval_artifact_hash_policy_accepts_canonical_digest\""));
        assert!(
            rendered.contains("\"case_id\": \"eval_artifact_hash_policy_rejects_invalid_digest\"")
        );
        assert!(rendered
            .contains("\"case_id\": \"eval_artifact_hash_policy_accepts_repo_relative_ref\""));
        assert!(rendered.contains("\"case_id\": \"eval_artifact_hash_policy_rejects_invalid_ref\""));
        assert!(rendered.contains(
            "\"case_id\": \"eval_file_artifact_hashing_helper_hashes_explicit_regular_file\""
        ));
        assert!(rendered.contains(
            "\"case_id\": \"eval_referenced_artifact_verification_helper_compares_expected_digest\""
        ));
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
