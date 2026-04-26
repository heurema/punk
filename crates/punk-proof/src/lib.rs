//! Minimal side-effect-free proofpack kernel for Punk Phase 3.
//!
//! This crate models post-gate provenance and writer-operation evidence as data
//! only. It does not write `.punk/proofs`, expose CLI behavior, write gate
//! decisions, claim acceptance, run validators, or require runtime storage.

use std::fmt::Write as _;

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const PROOFPACK_SCHEMA_VERSION: &str = "punk.proofpack.v0.1";
pub const PROOFPACK_WRITER_OPERATION_EVIDENCE_SCHEMA_VERSION: &str =
    "punk.proofpack.writer_operation_evidence.v0.1";
pub const PROOFPACK_WRITER_PREFLIGHT_PLAN_SCHEMA_VERSION: &str =
    "punk.proofpack.writer_preflight_plan.v0.1";
pub const PROOFPACK_WRITER_FILE_IO_PLAN_SCHEMA_VERSION: &str =
    "punk.proofpack.writer_file_io_plan.v0.1";

use punk_core::{
    compute_artifact_digest, validate_artifact_digest, ArtifactDigest, ArtifactHashPolicyError,
};

fn non_empty(value: impl Into<String>, error: ProofpackError) -> Result<String, ProofpackError> {
    let value = value.into().trim().to_string();

    if value.is_empty() {
        return Err(error);
    }

    Ok(value)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofpackId(String);

impl ProofpackId {
    pub fn new(value: impl Into<String>) -> Result<Self, ProofpackError> {
        Ok(Self(non_empty(value, ProofpackError::EmptyProofpackId)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofCreatedAt(String);

impl ProofCreatedAt {
    pub fn new(value: impl Into<String>) -> Result<Self, ProofpackError> {
        Ok(Self(non_empty(value, ProofpackError::EmptyCreatedAt)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofGateDecisionRef(String);

impl ProofGateDecisionRef {
    pub fn new(value: impl Into<String>) -> Result<Self, ProofpackError> {
        Ok(Self(non_empty(
            value,
            ProofpackError::EmptyGateDecisionRef,
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofContractRef(String);

impl ProofContractRef {
    pub fn new(value: impl Into<String>) -> Result<Self, ProofpackError> {
        Ok(Self(non_empty(value, ProofpackError::EmptyContractRef)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofRunReceiptRef(String);

impl ProofRunReceiptRef {
    pub fn new(value: impl Into<String>) -> Result<Self, ProofpackError> {
        Ok(Self(non_empty(value, ProofpackError::EmptyRunReceiptRef)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofEvalRef(String);

impl ProofEvalRef {
    pub fn new(value: impl Into<String>) -> Result<Self, ProofpackError> {
        Ok(Self(non_empty(value, ProofpackError::EmptyEvalRef)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofEventRef(String);

impl ProofEventRef {
    pub fn new(value: impl Into<String>) -> Result<Self, ProofpackError> {
        Ok(Self(non_empty(value, ProofpackError::EmptyEventRef)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofOutputArtifactRef(String);

impl ProofOutputArtifactRef {
    pub fn new(value: impl Into<String>) -> Result<Self, ProofpackError> {
        Ok(Self(non_empty(
            value,
            ProofpackError::EmptyOutputArtifactRef,
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofBoundaryNote(String);

impl ProofBoundaryNote {
    pub fn new(value: impl Into<String>) -> Result<Self, ProofpackError> {
        Ok(Self(non_empty(value, ProofpackError::EmptyBoundaryNote)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofArtifactRef(String);

impl ProofArtifactRef {
    pub fn new(value: impl Into<String>) -> Result<Self, ProofpackError> {
        Ok(Self(non_empty(value, ProofpackError::EmptyArtifactRef)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofArtifactHash(String);

impl ProofArtifactHash {
    pub fn new(value: impl Into<String>) -> Result<Self, ProofpackError> {
        let value = non_empty(value, ProofpackError::EmptyArtifactHash)?;
        validate_artifact_digest(&value).map_err(ProofpackError::InvalidArtifactHash)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofArtifactKind {
    GateDecision,
    Contract,
    RunReceipt,
    Eval,
    Event,
    OutputArtifact,
}

impl ProofArtifactKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GateDecision => "gate_decision",
            Self::Contract => "contract",
            Self::RunReceipt => "run_receipt",
            Self::Eval => "eval",
            Self::Event => "event",
            Self::OutputArtifact => "output_artifact",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofArtifactDigest {
    kind: ProofArtifactKind,
    artifact_ref: ProofArtifactRef,
    artifact_hash: ProofArtifactHash,
}

impl ProofArtifactDigest {
    pub fn new(
        kind: ProofArtifactKind,
        artifact_ref: ProofArtifactRef,
        artifact_hash: ProofArtifactHash,
    ) -> Self {
        Self {
            kind,
            artifact_ref,
            artifact_hash,
        }
    }

    pub fn kind(&self) -> ProofArtifactKind {
        self.kind
    }

    pub fn artifact_ref(&self) -> &ProofArtifactRef {
        &self.artifact_ref
    }

    pub fn artifact_hash(&self) -> &ProofArtifactHash {
        &self.artifact_hash
    }

    pub fn satisfies_requirement(&self, requirement: &ProofArtifactDigestRequirement) -> bool {
        self.kind == requirement.kind && self.artifact_ref == requirement.artifact_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofArtifactDigestRequirement {
    kind: ProofArtifactKind,
    artifact_ref: ProofArtifactRef,
}

impl ProofArtifactDigestRequirement {
    pub fn new(
        kind: ProofArtifactKind,
        artifact_ref: impl Into<String>,
    ) -> Result<Self, ProofpackError> {
        Ok(Self {
            kind,
            artifact_ref: ProofArtifactRef::new(artifact_ref)?,
        })
    }

    pub fn kind(&self) -> ProofArtifactKind {
        self.kind
    }

    pub fn artifact_ref(&self) -> &ProofArtifactRef {
        &self.artifact_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofpackIntegrityReport {
    required_digest_refs: Vec<ProofArtifactDigestRequirement>,
    missing_digest_refs: Vec<ProofArtifactDigestRequirement>,
}

impl ProofpackIntegrityReport {
    pub fn new(
        required_digest_refs: Vec<ProofArtifactDigestRequirement>,
        missing_digest_refs: Vec<ProofArtifactDigestRequirement>,
    ) -> Self {
        Self {
            required_digest_refs,
            missing_digest_refs,
        }
    }

    pub fn required_digest_refs(&self) -> &[ProofArtifactDigestRequirement] {
        &self.required_digest_refs
    }

    pub fn missing_digest_refs(&self) -> &[ProofArtifactDigestRequirement] {
        &self.missing_digest_refs
    }

    pub fn is_complete(&self) -> bool {
        self.missing_digest_refs.is_empty()
    }

    pub fn has_missing_required_digests(&self) -> bool {
        !self.is_complete()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proofpack {
    id: ProofpackId,
    schema_version: &'static str,
    gate_decision_ref: ProofGateDecisionRef,
    contract_refs: Vec<ProofContractRef>,
    run_receipt_refs: Vec<ProofRunReceiptRef>,
    eval_refs: Vec<ProofEvalRef>,
    event_refs: Vec<ProofEventRef>,
    output_artifact_refs: Vec<ProofOutputArtifactRef>,
    artifact_digests: Vec<ProofArtifactDigest>,
    created_at: ProofCreatedAt,
    boundary_notes: Vec<ProofBoundaryNote>,
}

impl Proofpack {
    pub fn new(
        id: ProofpackId,
        gate_decision_ref: ProofGateDecisionRef,
        contract_refs: Vec<ProofContractRef>,
        run_receipt_refs: Vec<ProofRunReceiptRef>,
        created_at: ProofCreatedAt,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> Result<Self, ProofpackError> {
        if contract_refs.is_empty() {
            return Err(ProofpackError::MissingContractRefs);
        }

        if run_receipt_refs.is_empty() {
            return Err(ProofpackError::MissingRunReceiptRefs);
        }

        if boundary_notes.is_empty() {
            return Err(ProofpackError::MissingBoundaryNotes);
        }

        Ok(Self {
            id,
            schema_version: PROOFPACK_SCHEMA_VERSION,
            gate_decision_ref,
            contract_refs,
            run_receipt_refs,
            eval_refs: Vec::new(),
            event_refs: Vec::new(),
            output_artifact_refs: Vec::new(),
            artifact_digests: Vec::new(),
            created_at,
            boundary_notes,
        })
    }

    pub fn with_eval_ref(mut self, eval_ref: ProofEvalRef) -> Self {
        self.eval_refs.push(eval_ref);
        self
    }

    pub fn with_event_ref(mut self, event_ref: ProofEventRef) -> Self {
        self.event_refs.push(event_ref);
        self
    }

    pub fn with_output_artifact_ref(mut self, output_artifact_ref: ProofOutputArtifactRef) -> Self {
        self.output_artifact_refs.push(output_artifact_ref);
        self
    }

    pub fn with_artifact_digest(mut self, artifact_digest: ProofArtifactDigest) -> Self {
        self.artifact_digests.push(artifact_digest);
        self
    }

    pub fn id(&self) -> &ProofpackId {
        &self.id
    }

    pub fn schema_version(&self) -> &str {
        self.schema_version
    }

    pub fn gate_decision_ref(&self) -> &ProofGateDecisionRef {
        &self.gate_decision_ref
    }

    pub fn contract_refs(&self) -> &[ProofContractRef] {
        &self.contract_refs
    }

    pub fn run_receipt_refs(&self) -> &[ProofRunReceiptRef] {
        &self.run_receipt_refs
    }

    pub fn eval_refs(&self) -> &[ProofEvalRef] {
        &self.eval_refs
    }

    pub fn event_refs(&self) -> &[ProofEventRef] {
        &self.event_refs
    }

    pub fn output_artifact_refs(&self) -> &[ProofOutputArtifactRef] {
        &self.output_artifact_refs
    }

    pub fn artifact_digests(&self) -> &[ProofArtifactDigest] {
        &self.artifact_digests
    }

    pub fn created_at(&self) -> &ProofCreatedAt {
        &self.created_at
    }

    pub fn boundary_notes(&self) -> &[ProofBoundaryNote] {
        &self.boundary_notes
    }

    pub fn boundary(&self) -> ProofpackBoundary {
        proofpack_boundary()
    }

    pub fn is_post_gate_provenance_bundle(&self) -> bool {
        self.boundary().post_gate_only
    }

    pub fn is_final_decision_authority(&self) -> bool {
        self.boundary().writes_final_decision
    }

    pub fn creates_acceptance_claim(&self) -> bool {
        self.boundary().creates_acceptance_claim
    }

    pub fn can_claim_acceptance_by_itself(&self) -> bool {
        false
    }

    pub fn references_evidence_without_absorbing(&self) -> bool {
        !self.boundary().absorbs_evidence
            && !self.contract_refs.is_empty()
            && !self.run_receipt_refs.is_empty()
    }

    pub fn matches_gate_decision_ref(&self, gate_decision_ref: &ProofGateDecisionRef) -> bool {
        &self.gate_decision_ref == gate_decision_ref
    }

    pub fn required_artifact_digest_refs(&self) -> Vec<ProofArtifactDigestRequirement> {
        let mut required = Vec::new();

        required.push(proof_artifact_digest_requirement(
            ProofArtifactKind::GateDecision,
            self.gate_decision_ref.as_str(),
        ));

        required.extend(self.contract_refs.iter().map(|contract_ref| {
            proof_artifact_digest_requirement(ProofArtifactKind::Contract, contract_ref.as_str())
        }));

        required.extend(self.run_receipt_refs.iter().map(|run_receipt_ref| {
            proof_artifact_digest_requirement(
                ProofArtifactKind::RunReceipt,
                run_receipt_ref.as_str(),
            )
        }));

        required.extend(self.eval_refs.iter().map(|eval_ref| {
            proof_artifact_digest_requirement(ProofArtifactKind::Eval, eval_ref.as_str())
        }));

        required.extend(self.event_refs.iter().map(|event_ref| {
            proof_artifact_digest_requirement(ProofArtifactKind::Event, event_ref.as_str())
        }));

        required.extend(self.output_artifact_refs.iter().map(|output_artifact_ref| {
            proof_artifact_digest_requirement(
                ProofArtifactKind::OutputArtifact,
                output_artifact_ref.as_str(),
            )
        }));

        required
    }

    pub fn has_artifact_digest_for(&self, requirement: &ProofArtifactDigestRequirement) -> bool {
        self.artifact_digests
            .iter()
            .any(|digest| digest.satisfies_requirement(requirement))
    }

    pub fn link_hash_integrity_report(&self) -> ProofpackIntegrityReport {
        let required_digest_refs = self.required_artifact_digest_refs();
        let missing_digest_refs = required_digest_refs
            .iter()
            .filter(|requirement| !self.has_artifact_digest_for(requirement))
            .cloned()
            .collect();

        ProofpackIntegrityReport::new(required_digest_refs, missing_digest_refs)
    }

    pub fn has_complete_link_hash_integrity(&self) -> bool {
        self.link_hash_integrity_report().is_complete()
    }

    pub fn is_matching_proof_ready_for_acceptance(
        &self,
        gate_decision_ref: &ProofGateDecisionRef,
    ) -> bool {
        self.matches_gate_decision_ref(gate_decision_ref) && self.has_complete_link_hash_integrity()
    }

    pub fn render_manifest_json(&self) -> String {
        let mut output = String::new();
        output.push_str("{\n");

        write_manifest_string_field(&mut output, 1, "proofpack_id", self.id().as_str(), true);
        write_manifest_string_field(
            &mut output,
            1,
            "schema_version",
            self.schema_version(),
            true,
        );
        write_manifest_string_field(
            &mut output,
            1,
            "gate_decision_ref",
            self.gate_decision_ref().as_str(),
            true,
        );
        write_manifest_string_array_field(
            &mut output,
            1,
            "contract_refs",
            self.contract_refs().iter().map(ProofContractRef::as_str),
            true,
        );
        write_manifest_string_array_field(
            &mut output,
            1,
            "run_receipt_refs",
            self.run_receipt_refs()
                .iter()
                .map(ProofRunReceiptRef::as_str),
            true,
        );
        write_manifest_string_array_field(
            &mut output,
            1,
            "eval_refs",
            self.eval_refs().iter().map(ProofEvalRef::as_str),
            true,
        );
        write_manifest_string_array_field(
            &mut output,
            1,
            "event_refs",
            self.event_refs().iter().map(ProofEventRef::as_str),
            true,
        );
        write_manifest_string_array_field(
            &mut output,
            1,
            "output_artifact_refs",
            self.output_artifact_refs()
                .iter()
                .map(ProofOutputArtifactRef::as_str),
            true,
        );
        write_manifest_artifact_digests(&mut output, self.artifact_digests());
        write_manifest_string_field(
            &mut output,
            1,
            "created_at",
            self.created_at().as_str(),
            true,
        );
        write_manifest_string_array_field(
            &mut output,
            1,
            "boundary_notes",
            self.boundary_notes().iter().map(ProofBoundaryNote::as_str),
            false,
        );

        output.push_str("}");
        output
    }
}

pub fn compute_proofpack_manifest_digest(proofpack: &Proofpack) -> ArtifactDigest {
    compute_artifact_digest(proofpack.render_manifest_json().as_bytes())
}

fn proof_artifact_digest_requirement(
    kind: ProofArtifactKind,
    artifact_ref: &str,
) -> ProofArtifactDigestRequirement {
    ProofArtifactDigestRequirement::new(kind, artifact_ref)
        .expect("proofpack refs are validated before integrity checks")
}

fn write_manifest_indent(output: &mut String, indent_level: usize) {
    output.push_str(&"  ".repeat(indent_level));
}

fn write_manifest_string_field(
    output: &mut String,
    indent_level: usize,
    key: &str,
    value: &str,
    trailing_comma: bool,
) {
    write_manifest_indent(output, indent_level);
    push_manifest_json_string(output, key);
    output.push_str(": ");
    push_manifest_json_string(output, value);
    if trailing_comma {
        output.push(',');
    }
    output.push('\n');
}

fn write_manifest_string_array_field<'a, I>(
    output: &mut String,
    indent_level: usize,
    key: &str,
    values: I,
    trailing_comma: bool,
) where
    I: IntoIterator<Item = &'a str>,
{
    write_manifest_indent(output, indent_level);
    push_manifest_json_string(output, key);
    output.push_str(": [");
    for (index, value) in values.into_iter().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        push_manifest_json_string(output, value);
    }
    output.push(']');
    if trailing_comma {
        output.push(',');
    }
    output.push('\n');
}

fn write_manifest_artifact_digests(output: &mut String, artifact_digests: &[ProofArtifactDigest]) {
    write_manifest_indent(output, 1);
    push_manifest_json_string(output, "artifact_digests");
    output.push_str(": [\n");

    for (index, digest) in artifact_digests.iter().enumerate() {
        write_manifest_indent(output, 2);
        output.push_str("{\n");
        write_manifest_string_field(output, 3, "kind", digest.kind().as_str(), true);
        write_manifest_string_field(output, 3, "ref", digest.artifact_ref().as_str(), true);
        write_manifest_string_field(output, 3, "hash", digest.artifact_hash().as_str(), false);
        write_manifest_indent(output, 2);
        output.push('}');
        if index + 1 != artifact_digests.len() {
            output.push(',');
        }
        output.push('\n');
    }

    write_manifest_indent(output, 1);
    output.push_str("],\n");
}

fn push_manifest_json_string(output: &mut String, value: &str) {
    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            character if character.is_control() => {
                write!(output, "\\u{:04x}", character as u32)
                    .expect("writing to String should succeed");
            }
            character => output.push(character),
        }
    }
    output.push('"');
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofpackWriterOperationId(String);

impl ProofpackWriterOperationId {
    pub fn new(value: impl Into<String>) -> Result<Self, ProofpackError> {
        Ok(Self(non_empty(
            value,
            ProofpackError::EmptyWriterOperationId,
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofpackWriterAttemptedAt(String);

impl ProofpackWriterAttemptedAt {
    pub fn new(value: impl Into<String>) -> Result<Self, ProofpackError> {
        Ok(Self(non_empty(
            value,
            ProofpackError::EmptyWriterAttemptedAt,
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofpackWriterTargetRef(String);

impl ProofpackWriterTargetRef {
    pub fn new(value: impl Into<String>) -> Result<Self, ProofpackError> {
        Ok(Self(non_empty(
            value,
            ProofpackError::EmptyWriterTargetRef,
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofpackWriterStorageRootRef(String);

impl ProofpackWriterStorageRootRef {
    pub fn new(value: impl Into<String>) -> Result<Self, ProofpackError> {
        Ok(Self(non_empty(
            value,
            ProofpackError::EmptyWriterStorageRootRef,
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofpackWriterTargetPathRef(String);

impl ProofpackWriterTargetPathRef {
    pub fn new(value: impl Into<String>) -> Result<Self, ProofpackError> {
        Ok(Self(non_empty(
            value,
            ProofpackError::EmptyWriterTargetPathRef,
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterOperationKind {
    PlannedOnly,
    Write,
    IdempotencyCheck,
    ConflictCheck,
    Repair,
    IndexUpdate,
    LatestPointerUpdate,
    Abort,
}

impl ProofpackWriterOperationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PlannedOnly => "planned_only",
            Self::Write => "write",
            Self::IdempotencyCheck => "idempotency_check",
            Self::ConflictCheck => "conflict_check",
            Self::Repair => "repair",
            Self::IndexUpdate => "index_update",
            Self::LatestPointerUpdate => "latest_pointer_update",
            Self::Abort => "abort",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterOperationOutcome {
    PlannedOnly,
    Written,
    AlreadyExistsMatching,
    ConflictExistingDifferent,
    PreflightFailed,
    WriteFailed,
    PartialWriteDetected,
    IndexUpdateFailed,
    LatestPointerUpdateFailed,
    Aborted,
}

impl ProofpackWriterOperationOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PlannedOnly => "planned_only",
            Self::Written => "written",
            Self::AlreadyExistsMatching => "already_exists_matching",
            Self::ConflictExistingDifferent => "conflict_existing_different",
            Self::PreflightFailed => "preflight_failed",
            Self::WriteFailed => "write_failed",
            Self::PartialWriteDetected => "partial_write_detected",
            Self::IndexUpdateFailed => "index_update_failed",
            Self::LatestPointerUpdateFailed => "latest_pointer_update_failed",
            Self::Aborted => "aborted",
        }
    }

    pub fn can_represent_canonical_artifact_availability(self) -> bool {
        matches!(self, Self::Written | Self::AlreadyExistsMatching)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterCanonicalArtifactStatus {
    NotAttempted,
    Written,
    AlreadyExistsMatching,
    ConflictExistingDifferent,
    WriteFailed,
    PartialWriteDetected,
}

impl ProofpackWriterCanonicalArtifactStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotAttempted => "not_attempted",
            Self::Written => "written",
            Self::AlreadyExistsMatching => "already_exists_matching",
            Self::ConflictExistingDifferent => "conflict_existing_different",
            Self::WriteFailed => "write_failed",
            Self::PartialWriteDetected => "partial_write_detected",
        }
    }

    pub fn is_available(self) -> bool {
        matches!(self, Self::Written | Self::AlreadyExistsMatching)
    }

    pub fn is_conflict(self) -> bool {
        self == Self::ConflictExistingDifferent
    }

    pub fn is_partial(self) -> bool {
        self == Self::PartialWriteDetected
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterSideEffectStatus {
    NotSelected,
    NotAttempted,
    Completed,
    Failed,
    Skipped,
}

impl ProofpackWriterSideEffectStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotSelected => "not_selected",
            Self::NotAttempted => "not_attempted",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Skipped => "skipped",
        }
    }

    pub fn is_failed(self) -> bool {
        self == Self::Failed
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofpackWriterOperationEvidence {
    operation_id: ProofpackWriterOperationId,
    operation_kind: ProofpackWriterOperationKind,
    proofpack_id: ProofpackId,
    schema_version: &'static str,
    attempted_at: ProofpackWriterAttemptedAt,
    target_ref: ProofpackWriterTargetRef,
    outcome: ProofpackWriterOperationOutcome,
    canonical_artifact_status: ProofpackWriterCanonicalArtifactStatus,
    index_status: ProofpackWriterSideEffectStatus,
    latest_pointer_status: ProofpackWriterSideEffectStatus,
    boundary_notes: Vec<ProofBoundaryNote>,
}

impl ProofpackWriterOperationEvidence {
    pub fn new(
        operation_id: ProofpackWriterOperationId,
        operation_kind: ProofpackWriterOperationKind,
        proofpack_id: ProofpackId,
        attempted_at: ProofpackWriterAttemptedAt,
        target_ref: ProofpackWriterTargetRef,
        outcome: ProofpackWriterOperationOutcome,
        canonical_artifact_status: ProofpackWriterCanonicalArtifactStatus,
        index_status: ProofpackWriterSideEffectStatus,
        latest_pointer_status: ProofpackWriterSideEffectStatus,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> Result<Self, ProofpackError> {
        if boundary_notes.is_empty() {
            return Err(ProofpackError::MissingWriterBoundaryNotes);
        }

        let evidence = Self {
            operation_id,
            operation_kind,
            proofpack_id,
            schema_version: PROOFPACK_WRITER_OPERATION_EVIDENCE_SCHEMA_VERSION,
            attempted_at,
            target_ref,
            outcome,
            canonical_artifact_status,
            index_status,
            latest_pointer_status,
            boundary_notes,
        };

        if !evidence.outcome_matches_reported_status() {
            return Err(ProofpackError::InconsistentWriterOperationEvidence);
        }

        Ok(evidence)
    }

    pub fn operation_id(&self) -> &ProofpackWriterOperationId {
        &self.operation_id
    }

    pub fn operation_kind(&self) -> ProofpackWriterOperationKind {
        self.operation_kind
    }

    pub fn proofpack_id(&self) -> &ProofpackId {
        &self.proofpack_id
    }

    pub fn schema_version(&self) -> &str {
        self.schema_version
    }

    pub fn attempted_at(&self) -> &ProofpackWriterAttemptedAt {
        &self.attempted_at
    }

    pub fn target_ref(&self) -> &ProofpackWriterTargetRef {
        &self.target_ref
    }

    pub fn outcome(&self) -> ProofpackWriterOperationOutcome {
        self.outcome
    }

    pub fn canonical_artifact_status(&self) -> ProofpackWriterCanonicalArtifactStatus {
        self.canonical_artifact_status
    }

    pub fn index_status(&self) -> ProofpackWriterSideEffectStatus {
        self.index_status
    }

    pub fn latest_pointer_status(&self) -> ProofpackWriterSideEffectStatus {
        self.latest_pointer_status
    }

    pub fn boundary_notes(&self) -> &[ProofBoundaryNote] {
        &self.boundary_notes
    }

    pub fn boundary(&self) -> ProofpackWriterOperationEvidenceBoundary {
        proofpack_writer_operation_evidence_boundary()
    }

    pub fn canonical_artifact_available(&self) -> bool {
        self.canonical_artifact_status.is_available()
    }

    pub fn represents_new_canonical_artifact_write(&self) -> bool {
        self.outcome == ProofpackWriterOperationOutcome::Written
            && self.canonical_artifact_status == ProofpackWriterCanonicalArtifactStatus::Written
    }

    pub fn has_conflict(&self) -> bool {
        self.canonical_artifact_status.is_conflict()
    }

    pub fn has_partial_write(&self) -> bool {
        self.canonical_artifact_status.is_partial()
    }

    pub fn has_index_or_latest_pointer_failure(&self) -> bool {
        self.index_status.is_failed() || self.latest_pointer_status.is_failed()
    }

    pub fn is_evidence_only(&self) -> bool {
        self.boundary().evidence_only
    }

    pub fn is_final_decision_authority(&self) -> bool {
        self.boundary().writes_final_decision
    }

    pub fn creates_acceptance_claim(&self) -> bool {
        self.boundary().creates_acceptance_claim
    }

    pub fn requires_runtime_storage(&self) -> bool {
        self.boundary().requires_runtime_storage
    }

    pub fn writes_cli_output(&self) -> bool {
        self.boundary().writes_cli_output
    }

    pub fn is_proofpack_artifact(&self) -> bool {
        self.boundary().is_proofpack_artifact
    }

    pub fn is_run_receipt(&self) -> bool {
        self.boundary().is_run_receipt
    }

    pub fn is_schema_validation(&self) -> bool {
        self.boundary().is_schema_validation
    }

    pub fn can_claim_acceptance_by_itself(&self) -> bool {
        false
    }

    fn outcome_matches_reported_status(&self) -> bool {
        match self.outcome {
            ProofpackWriterOperationOutcome::PlannedOnly
            | ProofpackWriterOperationOutcome::PreflightFailed
            | ProofpackWriterOperationOutcome::Aborted => {
                self.canonical_artifact_status
                    == ProofpackWriterCanonicalArtifactStatus::NotAttempted
            }
            ProofpackWriterOperationOutcome::Written => {
                self.canonical_artifact_status == ProofpackWriterCanonicalArtifactStatus::Written
            }
            ProofpackWriterOperationOutcome::AlreadyExistsMatching => {
                self.canonical_artifact_status
                    == ProofpackWriterCanonicalArtifactStatus::AlreadyExistsMatching
            }
            ProofpackWriterOperationOutcome::ConflictExistingDifferent => {
                self.canonical_artifact_status
                    == ProofpackWriterCanonicalArtifactStatus::ConflictExistingDifferent
            }
            ProofpackWriterOperationOutcome::WriteFailed => {
                self.canonical_artifact_status
                    == ProofpackWriterCanonicalArtifactStatus::WriteFailed
            }
            ProofpackWriterOperationOutcome::PartialWriteDetected => {
                self.canonical_artifact_status
                    == ProofpackWriterCanonicalArtifactStatus::PartialWriteDetected
            }
            ProofpackWriterOperationOutcome::IndexUpdateFailed => {
                self.canonical_artifact_status.is_available()
                    && self.index_status == ProofpackWriterSideEffectStatus::Failed
            }
            ProofpackWriterOperationOutcome::LatestPointerUpdateFailed => {
                self.canonical_artifact_status.is_available()
                    && self.latest_pointer_status == ProofpackWriterSideEffectStatus::Failed
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProofpackWriterOperationEvidenceBoundary {
    pub models_writer_operation_evidence: bool,
    pub writes_proofpack: bool,
    pub writes_writer_operation_evidence: bool,
    pub writes_final_decision: bool,
    pub creates_acceptance_claim: bool,
    pub requires_runtime_storage: bool,
    pub writes_cli_output: bool,
    pub writes_schema_files: bool,
    pub is_proofpack_artifact: bool,
    pub is_run_receipt: bool,
    pub is_schema_validation: bool,
    pub evidence_only: bool,
    pub separates_canonical_artifact_from_indexes: bool,
}

pub const fn proofpack_writer_operation_evidence_boundary(
) -> ProofpackWriterOperationEvidenceBoundary {
    ProofpackWriterOperationEvidenceBoundary {
        models_writer_operation_evidence: true,
        writes_proofpack: false,
        writes_writer_operation_evidence: false,
        writes_final_decision: false,
        creates_acceptance_claim: false,
        requires_runtime_storage: false,
        writes_cli_output: false,
        writes_schema_files: false,
        is_proofpack_artifact: false,
        is_run_receipt: false,
        is_schema_validation: false,
        evidence_only: true,
        separates_canonical_artifact_from_indexes: true,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterPreflightStatus {
    Ready,
    MissingPreconditions,
}

impl ProofpackWriterPreflightStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::MissingPreconditions => "missing_preconditions",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterMissingPrecondition {
    MissingRequiredArtifactDigests,
    MissingPlannedSideEffects,
    MissingBoundaryNotes,
}

impl ProofpackWriterMissingPrecondition {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingRequiredArtifactDigests => "missing_required_artifact_digests",
            Self::MissingPlannedSideEffects => "missing_planned_side_effects",
            Self::MissingBoundaryNotes => "missing_boundary_notes",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterPlannedSideEffect {
    CanonicalArtifactWrite,
    IndexUpdate,
    LatestPointerUpdate,
}

impl ProofpackWriterPlannedSideEffect {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalArtifactWrite => "canonical_artifact_write",
            Self::IndexUpdate => "index_update",
            Self::LatestPointerUpdate => "latest_pointer_update",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofpackWriterPreflightPlan {
    proofpack_id: ProofpackId,
    schema_version: &'static str,
    target_ref: ProofpackWriterTargetRef,
    manifest_self_digest: ArtifactDigest,
    planned_side_effects: Vec<ProofpackWriterPlannedSideEffect>,
    missing_preconditions: Vec<ProofpackWriterMissingPrecondition>,
    boundary_notes: Vec<ProofBoundaryNote>,
}

impl ProofpackWriterPreflightPlan {
    pub fn new(
        proofpack: &Proofpack,
        target_ref: ProofpackWriterTargetRef,
        planned_side_effects: Vec<ProofpackWriterPlannedSideEffect>,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> Self {
        let manifest_self_digest = compute_proofpack_manifest_digest(proofpack);
        let missing_preconditions = writer_preflight_missing_preconditions(
            proofpack,
            &planned_side_effects,
            &boundary_notes,
        );

        Self {
            proofpack_id: proofpack.id().clone(),
            schema_version: PROOFPACK_WRITER_PREFLIGHT_PLAN_SCHEMA_VERSION,
            target_ref,
            manifest_self_digest,
            planned_side_effects,
            missing_preconditions,
            boundary_notes,
        }
    }

    pub fn proofpack_id(&self) -> &ProofpackId {
        &self.proofpack_id
    }

    pub fn schema_version(&self) -> &str {
        self.schema_version
    }

    pub fn target_ref(&self) -> &ProofpackWriterTargetRef {
        &self.target_ref
    }

    pub fn manifest_self_digest(&self) -> &ArtifactDigest {
        &self.manifest_self_digest
    }

    pub fn planned_side_effects(&self) -> &[ProofpackWriterPlannedSideEffect] {
        &self.planned_side_effects
    }

    pub fn missing_preconditions(&self) -> &[ProofpackWriterMissingPrecondition] {
        &self.missing_preconditions
    }

    pub fn boundary_notes(&self) -> &[ProofBoundaryNote] {
        &self.boundary_notes
    }

    pub fn status(&self) -> ProofpackWriterPreflightStatus {
        if self.missing_preconditions.is_empty() {
            ProofpackWriterPreflightStatus::Ready
        } else {
            ProofpackWriterPreflightStatus::MissingPreconditions
        }
    }

    pub fn is_writer_ready(&self) -> bool {
        self.status() == ProofpackWriterPreflightStatus::Ready
    }

    pub fn has_missing_preconditions(&self) -> bool {
        !self.missing_preconditions.is_empty()
    }

    pub fn plans_side_effect(&self, side_effect: ProofpackWriterPlannedSideEffect) -> bool {
        self.planned_side_effects.contains(&side_effect)
    }

    pub fn operation_outcome(&self) -> ProofpackWriterOperationOutcome {
        if self.is_writer_ready() {
            ProofpackWriterOperationOutcome::PlannedOnly
        } else {
            ProofpackWriterOperationOutcome::PreflightFailed
        }
    }

    pub fn operation_kind(&self) -> ProofpackWriterOperationKind {
        ProofpackWriterOperationKind::PlannedOnly
    }

    pub fn to_operation_evidence(
        &self,
        operation_id: ProofpackWriterOperationId,
        attempted_at: ProofpackWriterAttemptedAt,
    ) -> Result<ProofpackWriterOperationEvidence, ProofpackError> {
        ProofpackWriterOperationEvidence::new(
            operation_id,
            self.operation_kind(),
            self.proofpack_id.clone(),
            attempted_at,
            self.target_ref.clone(),
            self.operation_outcome(),
            ProofpackWriterCanonicalArtifactStatus::NotAttempted,
            self.side_effect_status(ProofpackWriterPlannedSideEffect::IndexUpdate),
            self.side_effect_status(ProofpackWriterPlannedSideEffect::LatestPointerUpdate),
            self.operation_evidence_boundary_notes(),
        )
    }

    pub fn boundary(&self) -> ProofpackWriterPreflightPlanBoundary {
        proofpack_writer_preflight_plan_boundary()
    }

    pub fn is_evidence_only(&self) -> bool {
        self.boundary().evidence_only
    }

    pub fn writes_proofpack(&self) -> bool {
        self.boundary().writes_proofpack
    }

    pub fn requires_runtime_storage(&self) -> bool {
        self.boundary().requires_runtime_storage
    }

    pub fn writes_cli_output(&self) -> bool {
        self.boundary().writes_cli_output
    }

    pub fn creates_acceptance_claim(&self) -> bool {
        self.boundary().creates_acceptance_claim
    }

    fn side_effect_status(
        &self,
        side_effect: ProofpackWriterPlannedSideEffect,
    ) -> ProofpackWriterSideEffectStatus {
        if self.plans_side_effect(side_effect) {
            ProofpackWriterSideEffectStatus::NotAttempted
        } else {
            ProofpackWriterSideEffectStatus::NotSelected
        }
    }

    fn operation_evidence_boundary_notes(&self) -> Vec<ProofBoundaryNote> {
        if self.boundary_notes.is_empty() {
            return vec![ProofBoundaryNote::new(
                "Writer preflight plan is evidence-only; missing plan boundary notes are explicit precondition data.",
            )
            .expect("fallback boundary note should be valid")];
        }

        self.boundary_notes.clone()
    }
}

fn writer_preflight_missing_preconditions(
    proofpack: &Proofpack,
    planned_side_effects: &[ProofpackWriterPlannedSideEffect],
    boundary_notes: &[ProofBoundaryNote],
) -> Vec<ProofpackWriterMissingPrecondition> {
    let mut missing = Vec::new();

    if !proofpack.has_complete_link_hash_integrity() {
        missing.push(ProofpackWriterMissingPrecondition::MissingRequiredArtifactDigests);
    }

    if planned_side_effects.is_empty() {
        missing.push(ProofpackWriterMissingPrecondition::MissingPlannedSideEffects);
    }

    if boundary_notes.is_empty() {
        missing.push(ProofpackWriterMissingPrecondition::MissingBoundaryNotes);
    }

    missing
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProofpackWriterPreflightPlanBoundary {
    pub models_writer_preflight_plan: bool,
    pub computes_manifest_self_digest: bool,
    pub writes_proofpack: bool,
    pub writes_writer_operation_evidence: bool,
    pub writes_final_decision: bool,
    pub creates_acceptance_claim: bool,
    pub requires_runtime_storage: bool,
    pub writes_cli_output: bool,
    pub writes_schema_files: bool,
    pub evidence_only: bool,
    pub planned_side_effects_are_attempts: bool,
    pub separates_preflight_from_artifact_availability: bool,
}

pub const fn proofpack_writer_preflight_plan_boundary() -> ProofpackWriterPreflightPlanBoundary {
    ProofpackWriterPreflightPlanBoundary {
        models_writer_preflight_plan: true,
        computes_manifest_self_digest: true,
        writes_proofpack: false,
        writes_writer_operation_evidence: false,
        writes_final_decision: false,
        creates_acceptance_claim: false,
        requires_runtime_storage: false,
        writes_cli_output: false,
        writes_schema_files: false,
        evidence_only: true,
        planned_side_effects_are_attempts: false,
        separates_preflight_from_artifact_availability: true,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterFileIoPlanStatus {
    Ready,
    FileIoBlocked,
}

impl ProofpackWriterFileIoPlanStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::FileIoBlocked => "file_io_blocked",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterFileIoBlocker {
    PreflightPlanMissingPreconditions,
    MissingCanonicalArtifactWriteSelection,
    MissingErrorRollbackVisibility,
    MissingBoundaryNotes,
}

impl ProofpackWriterFileIoBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PreflightPlanMissingPreconditions => "preflight_plan_missing_preconditions",
            Self::MissingCanonicalArtifactWriteSelection => {
                "missing_canonical_artifact_write_selection"
            }
            Self::MissingErrorRollbackVisibility => "missing_error_rollback_visibility",
            Self::MissingBoundaryNotes => "missing_boundary_notes",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterWritePolicy {
    AppendOnlyCreateNew,
    IdempotentIfMatching,
    FailIfExists,
}

impl ProofpackWriterWritePolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AppendOnlyCreateNew => "append_only_create_new",
            Self::IdempotentIfMatching => "idempotent_if_matching",
            Self::FailIfExists => "fail_if_exists",
        }
    }

    pub fn supports_idempotency(self) -> bool {
        self == Self::IdempotentIfMatching
    }

    pub fn allows_silent_overwrite(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterIdempotencyBasis {
    ExactManifestBytes,
    ManifestSelfDigest,
    ContentIdentity,
}

impl ProofpackWriterIdempotencyBasis {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExactManifestBytes => "exact_manifest_bytes",
            Self::ManifestSelfDigest => "manifest_self_digest",
            Self::ContentIdentity => "content_identity",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterTempAtomicPolicy {
    AtomicSiblingTemp,
    ExplicitNonAtomic,
    FailClosedIfAtomicUnavailable,
}

impl ProofpackWriterTempAtomicPolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AtomicSiblingTemp => "atomic_sibling_temp",
            Self::ExplicitNonAtomic => "explicit_non_atomic",
            Self::FailClosedIfAtomicUnavailable => "fail_closed_if_atomic_unavailable",
        }
    }

    pub fn prefers_atomic_move(self) -> bool {
        matches!(
            self,
            Self::AtomicSiblingTemp | Self::FailClosedIfAtomicUnavailable
        )
    }

    pub fn fails_closed_when_atomic_unavailable(self) -> bool {
        self == Self::FailClosedIfAtomicUnavailable
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterFileIoFailureVisibility {
    StorageRootMissing,
    StorageRootDisallowed,
    TargetPathInvalid,
    ParentDirectoryMissing,
    ExistingTargetMatching,
    ExistingTargetDifferent,
    TempWriteDenied,
    TempWriteFailed,
    FlushOrSyncFailed,
    AtomicMoveUnsupported,
    AtomicMoveFailed,
    CleanupFailed,
    PartialCanonicalArtifactAmbiguous,
    IndexUpdateFailed,
    LatestPointerUpdateFailed,
}

impl ProofpackWriterFileIoFailureVisibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::StorageRootMissing => "storage_root_missing",
            Self::StorageRootDisallowed => "storage_root_disallowed",
            Self::TargetPathInvalid => "target_path_invalid",
            Self::ParentDirectoryMissing => "parent_directory_missing",
            Self::ExistingTargetMatching => "existing_target_matching",
            Self::ExistingTargetDifferent => "existing_target_different",
            Self::TempWriteDenied => "temp_write_denied",
            Self::TempWriteFailed => "temp_write_failed",
            Self::FlushOrSyncFailed => "flush_or_sync_failed",
            Self::AtomicMoveUnsupported => "atomic_move_unsupported",
            Self::AtomicMoveFailed => "atomic_move_failed",
            Self::CleanupFailed => "cleanup_failed",
            Self::PartialCanonicalArtifactAmbiguous => "partial_canonical_artifact_ambiguous",
            Self::IndexUpdateFailed => "index_update_failed",
            Self::LatestPointerUpdateFailed => "latest_pointer_update_failed",
        }
    }

    pub fn is_conflict_related(self) -> bool {
        self == Self::ExistingTargetDifferent
    }

    pub fn is_rollback_related(self) -> bool {
        matches!(
            self,
            Self::CleanupFailed | Self::PartialCanonicalArtifactAmbiguous
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofpackWriterFileIoPlan {
    proofpack_id: ProofpackId,
    schema_version: &'static str,
    storage_root_ref: ProofpackWriterStorageRootRef,
    target_artifact_ref: ProofpackWriterTargetRef,
    target_path_ref: ProofpackWriterTargetPathRef,
    manifest_self_digest: ArtifactDigest,
    write_policy: ProofpackWriterWritePolicy,
    idempotency_basis: ProofpackWriterIdempotencyBasis,
    temp_atomic_policy: ProofpackWriterTempAtomicPolicy,
    planned_side_effects: Vec<ProofpackWriterPlannedSideEffect>,
    failure_visibility: Vec<ProofpackWriterFileIoFailureVisibility>,
    blockers: Vec<ProofpackWriterFileIoBlocker>,
    boundary_notes: Vec<ProofBoundaryNote>,
}

impl ProofpackWriterFileIoPlan {
    pub fn new(
        preflight_plan: &ProofpackWriterPreflightPlan,
        storage_root_ref: ProofpackWriterStorageRootRef,
        target_path_ref: ProofpackWriterTargetPathRef,
        write_policy: ProofpackWriterWritePolicy,
        idempotency_basis: ProofpackWriterIdempotencyBasis,
        temp_atomic_policy: ProofpackWriterTempAtomicPolicy,
        failure_visibility: Vec<ProofpackWriterFileIoFailureVisibility>,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> Self {
        let blockers =
            writer_file_io_plan_blockers(preflight_plan, &failure_visibility, &boundary_notes);

        Self {
            proofpack_id: preflight_plan.proofpack_id().clone(),
            schema_version: PROOFPACK_WRITER_FILE_IO_PLAN_SCHEMA_VERSION,
            storage_root_ref,
            target_artifact_ref: preflight_plan.target_ref().clone(),
            target_path_ref,
            manifest_self_digest: preflight_plan.manifest_self_digest().clone(),
            write_policy,
            idempotency_basis,
            temp_atomic_policy,
            planned_side_effects: preflight_plan.planned_side_effects().to_vec(),
            failure_visibility,
            blockers,
            boundary_notes,
        }
    }

    pub fn proofpack_id(&self) -> &ProofpackId {
        &self.proofpack_id
    }

    pub fn schema_version(&self) -> &str {
        self.schema_version
    }

    pub fn storage_root_ref(&self) -> &ProofpackWriterStorageRootRef {
        &self.storage_root_ref
    }

    pub fn target_artifact_ref(&self) -> &ProofpackWriterTargetRef {
        &self.target_artifact_ref
    }

    pub fn target_ref(&self) -> &ProofpackWriterTargetRef {
        self.target_artifact_ref()
    }

    pub fn target_path_ref(&self) -> &ProofpackWriterTargetPathRef {
        &self.target_path_ref
    }

    pub fn manifest_self_digest(&self) -> &ArtifactDigest {
        &self.manifest_self_digest
    }

    pub fn write_policy(&self) -> ProofpackWriterWritePolicy {
        self.write_policy
    }

    pub fn idempotency_basis(&self) -> ProofpackWriterIdempotencyBasis {
        self.idempotency_basis
    }

    pub fn temp_atomic_policy(&self) -> ProofpackWriterTempAtomicPolicy {
        self.temp_atomic_policy
    }

    pub fn planned_side_effects(&self) -> &[ProofpackWriterPlannedSideEffect] {
        &self.planned_side_effects
    }

    pub fn failure_visibility(&self) -> &[ProofpackWriterFileIoFailureVisibility] {
        &self.failure_visibility
    }

    pub fn blockers(&self) -> &[ProofpackWriterFileIoBlocker] {
        &self.blockers
    }

    pub fn boundary_notes(&self) -> &[ProofBoundaryNote] {
        &self.boundary_notes
    }

    pub fn status(&self) -> ProofpackWriterFileIoPlanStatus {
        if self.blockers.is_empty() {
            ProofpackWriterFileIoPlanStatus::Ready
        } else {
            ProofpackWriterFileIoPlanStatus::FileIoBlocked
        }
    }

    pub fn is_file_io_ready(&self) -> bool {
        self.status() == ProofpackWriterFileIoPlanStatus::Ready
    }

    pub fn has_file_io_blockers(&self) -> bool {
        !self.blockers.is_empty()
    }

    pub fn plans_side_effect(&self, side_effect: ProofpackWriterPlannedSideEffect) -> bool {
        self.planned_side_effects.contains(&side_effect)
    }

    pub fn selects_canonical_artifact_write(&self) -> bool {
        self.plans_side_effect(ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite)
    }

    pub fn selects_index_update(&self) -> bool {
        self.plans_side_effect(ProofpackWriterPlannedSideEffect::IndexUpdate)
    }

    pub fn selects_latest_pointer_update(&self) -> bool {
        self.plans_side_effect(ProofpackWriterPlannedSideEffect::LatestPointerUpdate)
    }

    pub fn tracks_failure(&self, failure: ProofpackWriterFileIoFailureVisibility) -> bool {
        self.failure_visibility.contains(&failure)
    }

    pub fn tracks_conflict_visibility(&self) -> bool {
        self.failure_visibility
            .iter()
            .any(|failure| failure.is_conflict_related())
    }

    pub fn tracks_rollback_visibility(&self) -> bool {
        self.failure_visibility
            .iter()
            .any(|failure| failure.is_rollback_related())
    }

    pub fn operation_outcome(&self) -> ProofpackWriterOperationOutcome {
        if self.is_file_io_ready() {
            ProofpackWriterOperationOutcome::PlannedOnly
        } else {
            ProofpackWriterOperationOutcome::PreflightFailed
        }
    }

    pub fn operation_kind(&self) -> ProofpackWriterOperationKind {
        ProofpackWriterOperationKind::PlannedOnly
    }

    pub fn to_operation_evidence(
        &self,
        operation_id: ProofpackWriterOperationId,
        attempted_at: ProofpackWriterAttemptedAt,
    ) -> Result<ProofpackWriterOperationEvidence, ProofpackError> {
        ProofpackWriterOperationEvidence::new(
            operation_id,
            self.operation_kind(),
            self.proofpack_id.clone(),
            attempted_at,
            self.target_artifact_ref.clone(),
            self.operation_outcome(),
            ProofpackWriterCanonicalArtifactStatus::NotAttempted,
            self.side_effect_status(ProofpackWriterPlannedSideEffect::IndexUpdate),
            self.side_effect_status(ProofpackWriterPlannedSideEffect::LatestPointerUpdate),
            self.operation_evidence_boundary_notes(),
        )
    }

    pub fn boundary(&self) -> ProofpackWriterFileIoPlanBoundary {
        proofpack_writer_file_io_plan_boundary()
    }

    pub fn is_evidence_only(&self) -> bool {
        self.boundary().evidence_only
    }

    pub fn touches_filesystem(&self) -> bool {
        self.boundary().touches_filesystem
    }

    pub fn writes_proofpack(&self) -> bool {
        self.boundary().writes_proofpack
    }

    pub fn requires_runtime_storage(&self) -> bool {
        self.boundary().requires_runtime_storage
    }

    pub fn writes_cli_output(&self) -> bool {
        self.boundary().writes_cli_output
    }

    pub fn creates_acceptance_claim(&self) -> bool {
        self.boundary().creates_acceptance_claim
    }

    pub fn target_path_is_authority(&self) -> bool {
        self.boundary().target_path_is_authority
    }

    pub fn index_latest_are_canonical(&self) -> bool {
        self.boundary().index_latest_are_canonical
    }

    fn side_effect_status(
        &self,
        side_effect: ProofpackWriterPlannedSideEffect,
    ) -> ProofpackWriterSideEffectStatus {
        if self.plans_side_effect(side_effect) {
            ProofpackWriterSideEffectStatus::NotAttempted
        } else {
            ProofpackWriterSideEffectStatus::NotSelected
        }
    }

    fn operation_evidence_boundary_notes(&self) -> Vec<ProofBoundaryNote> {
        if self.boundary_notes.is_empty() {
            return vec![ProofBoundaryNote::new(
                "Writer file IO plan is evidence-only; missing plan boundary notes are explicit blocker data.",
            )
            .expect("fallback boundary note should be valid")];
        }

        self.boundary_notes.clone()
    }
}

fn writer_file_io_plan_blockers(
    preflight_plan: &ProofpackWriterPreflightPlan,
    failure_visibility: &[ProofpackWriterFileIoFailureVisibility],
    boundary_notes: &[ProofBoundaryNote],
) -> Vec<ProofpackWriterFileIoBlocker> {
    let mut blockers = Vec::new();

    if preflight_plan.has_missing_preconditions() {
        blockers.push(ProofpackWriterFileIoBlocker::PreflightPlanMissingPreconditions);
    }

    if !preflight_plan.plans_side_effect(ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite) {
        blockers.push(ProofpackWriterFileIoBlocker::MissingCanonicalArtifactWriteSelection);
    }

    if failure_visibility.is_empty() {
        blockers.push(ProofpackWriterFileIoBlocker::MissingErrorRollbackVisibility);
    }

    if boundary_notes.is_empty() {
        blockers.push(ProofpackWriterFileIoBlocker::MissingBoundaryNotes);
    }

    blockers
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProofpackWriterFileIoPlanBoundary {
    pub models_writer_file_io_plan: bool,
    pub models_explicit_storage_root: bool,
    pub models_target_artifact_ref: bool,
    pub models_target_path_ref: bool,
    pub models_write_policy: bool,
    pub models_idempotency_basis: bool,
    pub models_temp_atomic_policy: bool,
    pub models_error_rollback_visibility: bool,
    pub writes_proofpack: bool,
    pub touches_filesystem: bool,
    pub writes_writer_operation_evidence: bool,
    pub writes_final_decision: bool,
    pub creates_acceptance_claim: bool,
    pub requires_runtime_storage: bool,
    pub writes_cli_output: bool,
    pub writes_schema_files: bool,
    pub evidence_only: bool,
    pub target_path_is_authority: bool,
    pub index_latest_are_canonical: bool,
    pub separates_file_io_plan_from_artifact_availability: bool,
}

pub const fn proofpack_writer_file_io_plan_boundary() -> ProofpackWriterFileIoPlanBoundary {
    ProofpackWriterFileIoPlanBoundary {
        models_writer_file_io_plan: true,
        models_explicit_storage_root: true,
        models_target_artifact_ref: true,
        models_target_path_ref: true,
        models_write_policy: true,
        models_idempotency_basis: true,
        models_temp_atomic_policy: true,
        models_error_rollback_visibility: true,
        writes_proofpack: false,
        touches_filesystem: false,
        writes_writer_operation_evidence: false,
        writes_final_decision: false,
        creates_acceptance_claim: false,
        requires_runtime_storage: false,
        writes_cli_output: false,
        writes_schema_files: false,
        evidence_only: true,
        target_path_is_authority: false,
        index_latest_are_canonical: false,
        separates_file_io_plan_from_artifact_availability: true,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProofpackError {
    EmptyProofpackId,
    EmptyCreatedAt,
    EmptyGateDecisionRef,
    EmptyContractRef,
    EmptyRunReceiptRef,
    EmptyEvalRef,
    EmptyEventRef,
    EmptyOutputArtifactRef,
    EmptyBoundaryNote,
    EmptyArtifactRef,
    EmptyArtifactHash,
    EmptyWriterOperationId,
    EmptyWriterAttemptedAt,
    EmptyWriterTargetRef,
    EmptyWriterStorageRootRef,
    EmptyWriterTargetPathRef,
    InvalidArtifactHash(ArtifactHashPolicyError),
    MissingContractRefs,
    MissingRunReceiptRefs,
    MissingBoundaryNotes,
    MissingWriterBoundaryNotes,
    InconsistentWriterOperationEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProofpackBoundary {
    pub models_proofpack: bool,
    pub writes_proofpack: bool,
    pub writes_final_decision: bool,
    pub implies_acceptance_by_itself: bool,
    pub requires_accepting_gate_decision_for_acceptance: bool,
    pub requires_matching_proofpack_for_acceptance: bool,
    pub requires_runtime_storage: bool,
    pub writes_cli_output: bool,
    pub creates_acceptance_claim: bool,
    pub absorbs_evidence: bool,
    pub post_gate_only: bool,
    pub checks_structural_link_hash_integrity: bool,
    pub computes_manifest_digest: bool,
    pub computes_referenced_artifact_hashes: bool,
    pub computes_hashes: bool,
    pub normalizes_hashes: bool,
}

pub const fn proofpack_boundary() -> ProofpackBoundary {
    ProofpackBoundary {
        models_proofpack: true,
        writes_proofpack: false,
        writes_final_decision: false,
        implies_acceptance_by_itself: false,
        requires_accepting_gate_decision_for_acceptance: true,
        requires_matching_proofpack_for_acceptance: true,
        requires_runtime_storage: false,
        writes_cli_output: false,
        creates_acceptance_claim: false,
        absorbs_evidence: false,
        post_gate_only: true,
        checks_structural_link_hash_integrity: true,
        computes_manifest_digest: true,
        computes_referenced_artifact_hashes: false,
        computes_hashes: false,
        normalizes_hashes: false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PositiveAcceptanceInputs {
    pub accepting_gate_decision: bool,
    pub matching_proofpack: bool,
}

pub fn positive_acceptance_preconditions_met(inputs: PositiveAcceptanceInputs) -> bool {
    inputs.accepting_gate_decision && inputs.matching_proofpack
}

#[cfg(test)]
mod tests {
    use super::*;

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
    const PROOF_HASH_OTHER: &str =
        "sha256:0000000000000000000000000000000000000000000000000000000000000007";

    fn sample_proofpack_without_digests() -> Proofpack {
        Proofpack::new(
            ProofpackId::new("proofpack_local_001").expect("proofpack id should be valid"),
            ProofGateDecisionRef::new("decision_local_001")
                .expect("gate decision ref should be valid"),
            vec![ProofContractRef::new("contract_local_001")
                .expect("contract ref should be valid")],
            vec![ProofRunReceiptRef::new("receipt_local_001")
                .expect("run receipt ref should be valid")],
            ProofCreatedAt::new("2026-04-25T20:00:00Z")
                .expect("created_at should be valid"),
            vec![ProofBoundaryNote::new(
                "Proofpack references evidence; gate remains the decision authority.",
            )
            .expect("boundary note should be valid")],
        )
        .expect("proofpack should be valid")
        .with_eval_ref(
            ProofEvalRef::new("work/reports/2026-04-25-gate-decision-kernel-minimal-v0-1.md")
                .expect("eval ref should be valid"),
        )
        .with_event_ref(ProofEventRef::new("evt_0000000000000001").expect("event ref should be valid"))
        .with_output_artifact_ref(
            ProofOutputArtifactRef::new("target/debug/punk")
                .expect("output artifact ref should be valid"),
        )
    }

    fn sample_proofpack() -> Proofpack {
        sample_proofpack_without_digests()
            .with_artifact_digest(ProofArtifactDigest::new(
                ProofArtifactKind::GateDecision,
                ProofArtifactRef::new("decision_local_001").expect("artifact ref should be valid"),
                ProofArtifactHash::new(PROOF_HASH_GATE_DECISION)
                    .expect("artifact hash should be valid"),
            ))
            .with_artifact_digest(ProofArtifactDigest::new(
                ProofArtifactKind::Contract,
                ProofArtifactRef::new("contract_local_001").expect("artifact ref should be valid"),
                ProofArtifactHash::new(PROOF_HASH_CONTRACT).expect("artifact hash should be valid"),
            ))
            .with_artifact_digest(ProofArtifactDigest::new(
                ProofArtifactKind::RunReceipt,
                ProofArtifactRef::new("receipt_local_001").expect("artifact ref should be valid"),
                ProofArtifactHash::new(PROOF_HASH_RUN_RECEIPT)
                    .expect("artifact hash should be valid"),
            ))
            .with_artifact_digest(ProofArtifactDigest::new(
                ProofArtifactKind::Eval,
                ProofArtifactRef::new(
                    "work/reports/2026-04-25-gate-decision-kernel-minimal-v0-1.md",
                )
                .expect("artifact ref should be valid"),
                ProofArtifactHash::new(PROOF_HASH_EVAL).expect("artifact hash should be valid"),
            ))
            .with_artifact_digest(ProofArtifactDigest::new(
                ProofArtifactKind::Event,
                ProofArtifactRef::new("evt_0000000000000001")
                    .expect("artifact ref should be valid"),
                ProofArtifactHash::new(PROOF_HASH_EVENT).expect("artifact hash should be valid"),
            ))
            .with_artifact_digest(ProofArtifactDigest::new(
                ProofArtifactKind::OutputArtifact,
                ProofArtifactRef::new("target/debug/punk").expect("artifact ref should be valid"),
                ProofArtifactHash::new(PROOF_HASH_OUTPUT).expect("artifact hash should be valid"),
            ))
    }

    fn digest_requirement(
        kind: ProofArtifactKind,
        artifact_ref: &str,
    ) -> ProofArtifactDigestRequirement {
        ProofArtifactDigestRequirement::new(kind, artifact_ref)
            .expect("digest requirement should be valid")
    }

    fn sample_writer_operation_evidence(
        outcome: ProofpackWriterOperationOutcome,
        canonical_artifact_status: ProofpackWriterCanonicalArtifactStatus,
        index_status: ProofpackWriterSideEffectStatus,
        latest_pointer_status: ProofpackWriterSideEffectStatus,
    ) -> ProofpackWriterOperationEvidence {
        ProofpackWriterOperationEvidence::new(
            ProofpackWriterOperationId::new("writer_op_local_001")
                .expect("writer operation id should be valid"),
            ProofpackWriterOperationKind::Write,
            ProofpackId::new("proofpack_local_001").expect("proofpack id should be valid"),
            ProofpackWriterAttemptedAt::new("2026-04-26T13:00:00Z")
                .expect("attempted_at should be valid"),
            ProofpackWriterTargetRef::new("future/.punk/proofs/proofpack_local_001.json")
                .expect("target ref should be valid"),
            outcome,
            canonical_artifact_status,
            index_status,
            latest_pointer_status,
            vec![ProofBoundaryNote::new(
                "Writer operation evidence is evidence-only and does not claim acceptance.",
            )
            .expect("boundary note should be valid")],
        )
        .expect("writer operation evidence should be consistent")
    }

    fn sample_writer_preflight_plan(
        proofpack: &Proofpack,
        planned_side_effects: Vec<ProofpackWriterPlannedSideEffect>,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> ProofpackWriterPreflightPlan {
        ProofpackWriterPreflightPlan::new(
            proofpack,
            ProofpackWriterTargetRef::new("future/.punk/proofs/proofpack_local_001.json")
                .expect("target ref should be valid"),
            planned_side_effects,
            boundary_notes,
        )
    }

    fn sample_writer_file_io_plan(
        preflight_plan: &ProofpackWriterPreflightPlan,
        failure_visibility: Vec<ProofpackWriterFileIoFailureVisibility>,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> ProofpackWriterFileIoPlan {
        ProofpackWriterFileIoPlan::new(
            preflight_plan,
            ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
                .expect("storage root ref should be valid"),
            ProofpackWriterTargetPathRef::new("future/.punk/proofs/proofpack_local_001.json")
                .expect("target path ref should be valid"),
            ProofpackWriterWritePolicy::IdempotentIfMatching,
            ProofpackWriterIdempotencyBasis::ManifestSelfDigest,
            ProofpackWriterTempAtomicPolicy::AtomicSiblingTemp,
            failure_visibility,
            boundary_notes,
        )
    }

    #[test]
    fn proofpack_references_evidence_and_hashes_without_absorbing_them() {
        let proofpack = sample_proofpack();

        assert_eq!(proofpack.id().as_str(), "proofpack_local_001");
        assert_eq!(proofpack.schema_version(), PROOFPACK_SCHEMA_VERSION);
        assert_eq!(proofpack.gate_decision_ref().as_str(), "decision_local_001");
        assert_eq!(proofpack.contract_refs()[0].as_str(), "contract_local_001");
        assert_eq!(
            proofpack.run_receipt_refs()[0].as_str(),
            "receipt_local_001"
        );
        assert_eq!(proofpack.eval_refs().len(), 1);
        assert_eq!(proofpack.event_refs().len(), 1);
        assert_eq!(proofpack.output_artifact_refs().len(), 1);
        assert_eq!(proofpack.artifact_digests().len(), 6);
        assert_eq!(
            proofpack.artifact_digests()[0].kind().as_str(),
            "gate_decision"
        );
        assert_eq!(
            proofpack.artifact_digests()[0].artifact_hash().as_str(),
            PROOF_HASH_GATE_DECISION
        );
        assert_eq!(proofpack.created_at().as_str(), "2026-04-25T20:00:00Z");
        assert_eq!(proofpack.boundary_notes().len(), 1);
        assert!(proofpack.references_evidence_without_absorbing());
        assert!(!proofpack.boundary().absorbs_evidence);
    }

    #[test]
    fn proofpack_manifest_renderer_is_deterministic_and_complete() {
        let proofpack = sample_proofpack();
        let rendered = proofpack.render_manifest_json();

        assert_eq!(rendered, proofpack.render_manifest_json());
        assert!(rendered.starts_with("{\n"));
        assert!(rendered.contains("\"proofpack_id\": \"proofpack_local_001\""));
        assert!(rendered.contains(&format!(
            "\"schema_version\": \"{PROOFPACK_SCHEMA_VERSION}\""
        )));
        assert!(rendered.contains("\"gate_decision_ref\": \"decision_local_001\""));
        assert!(rendered.contains("\"contract_refs\": [\"contract_local_001\"]"));
        assert!(rendered.contains("\"run_receipt_refs\": [\"receipt_local_001\"]"));
        assert!(rendered.contains(
            "\"eval_refs\": [\"work/reports/2026-04-25-gate-decision-kernel-minimal-v0-1.md\"]"
        ));
        assert!(rendered.contains("\"event_refs\": [\"evt_0000000000000001\"]"));
        assert!(rendered.contains("\"output_artifact_refs\": [\"target/debug/punk\"]"));
        assert!(rendered.contains("\"artifact_digests\": ["));
        assert!(rendered.contains("\"kind\": \"gate_decision\""));
        assert!(rendered.contains("\"ref\": \"decision_local_001\""));
        assert!(rendered.contains(&format!("\"hash\": \"{PROOF_HASH_GATE_DECISION}\"")));
        assert!(rendered.contains("\"created_at\": \"2026-04-25T20:00:00Z\""));
        assert!(rendered.contains(
            "\"boundary_notes\": [\"Proofpack references evidence; gate remains the decision authority.\"]"
        ));
    }

    #[test]
    fn proofpack_manifest_renderer_escapes_json_strings() {
        let proofpack = Proofpack::new(
            ProofpackId::new("proofpack_\"quoted\"").expect("proofpack id should be valid"),
            ProofGateDecisionRef::new("decision\\local").expect("decision ref should be valid"),
            vec![ProofContractRef::new("contract\nlocal").expect("contract ref should be valid")],
            vec![ProofRunReceiptRef::new("receipt\tlocal").expect("receipt ref should be valid")],
            ProofCreatedAt::new("2026-04-25T20:00:00Z").expect("created_at should be valid"),
            vec![
                ProofBoundaryNote::new("note with \"quote\", slash\\ and\nnewline")
                    .expect("boundary note should be valid"),
            ],
        )
        .expect("proofpack should be valid");

        let rendered = proofpack.render_manifest_json();

        assert!(rendered.contains("\"proofpack_id\": \"proofpack_\\\"quoted\\\"\""));
        assert!(rendered.contains("\"gate_decision_ref\": \"decision\\\\local\""));
        assert!(rendered.contains("\"contract_refs\": [\"contract\\nlocal\"]"));
        assert!(rendered.contains("\"run_receipt_refs\": [\"receipt\\tlocal\"]"));
        assert!(rendered.contains(
            "\"boundary_notes\": [\"note with \\\"quote\\\", slash\\\\ and\\nnewline\"]"
        ));
    }

    #[test]
    fn proofpack_manifest_renderer_has_no_writer_or_hashing_side_effects() {
        let proofpack = sample_proofpack();
        let boundary = proofpack.boundary();
        let rendered = proofpack.render_manifest_json();

        assert!(!rendered.is_empty());
        assert!(!boundary.writes_proofpack);
        assert!(!boundary.requires_runtime_storage);
        assert!(!boundary.writes_cli_output);
        assert!(!boundary.creates_acceptance_claim);
        assert!(!boundary.computes_hashes);
        assert!(!boundary.normalizes_hashes);
    }

    #[test]
    fn proofpack_manifest_digest_is_deterministic_and_canonical() {
        let proofpack = sample_proofpack();
        let digest = compute_proofpack_manifest_digest(&proofpack);

        assert_eq!(digest, compute_proofpack_manifest_digest(&proofpack));
        assert!(digest.as_str().starts_with("sha256:"));
        assert_eq!(digest.as_str().len(), "sha256:".len() + 64);
        assert!(validate_artifact_digest(digest.as_str()).is_ok());
    }

    #[test]
    fn proofpack_manifest_digest_uses_exact_renderer_bytes() {
        let proofpack = sample_proofpack();
        let rendered = proofpack.render_manifest_json();
        let digest = compute_proofpack_manifest_digest(&proofpack);
        let expected = compute_artifact_digest(rendered.as_bytes());

        assert_eq!(digest, expected);
    }

    #[test]
    fn proofpack_manifest_digest_preserves_renderer_byte_identity() {
        let proofpack = sample_proofpack();
        let rendered = proofpack.render_manifest_json();
        let digest = compute_proofpack_manifest_digest(&proofpack);
        let digest_with_trailing_newline =
            compute_artifact_digest(format!("{rendered}\n").as_bytes());

        assert_ne!(digest, digest_with_trailing_newline);
        assert!(!rendered.ends_with('\n'));
    }

    #[test]
    fn proofpack_manifest_digest_has_no_recursive_self_inclusion() {
        let proofpack = sample_proofpack();
        let rendered_before = proofpack.render_manifest_json();
        let digest = compute_proofpack_manifest_digest(&proofpack);
        let rendered_after = proofpack.render_manifest_json();

        assert_eq!(rendered_before, rendered_after);
        assert!(!rendered_after.contains(digest.as_str()));
    }

    #[test]
    fn proofpack_manifest_digest_does_not_satisfy_referenced_artifact_integrity() {
        let proofpack = sample_proofpack_without_digests();
        let before = proofpack.link_hash_integrity_report();
        let digest = compute_proofpack_manifest_digest(&proofpack);
        let after = proofpack.link_hash_integrity_report();

        assert!(validate_artifact_digest(digest.as_str()).is_ok());
        assert!(before.has_missing_required_digests());
        assert!(after.has_missing_required_digests());
        assert_eq!(
            before.required_digest_refs().len(),
            after.required_digest_refs().len()
        );
        assert_eq!(
            before.missing_digest_refs().len(),
            after.missing_digest_refs().len()
        );
        assert_eq!(proofpack.artifact_digests().len(), 0);
        assert!(!proofpack.has_complete_link_hash_integrity());
    }

    #[test]
    fn proofpack_manifest_digest_has_no_writer_or_runtime_side_effects() {
        let proofpack = sample_proofpack();
        let boundary = proofpack.boundary();
        let digest = compute_proofpack_manifest_digest(&proofpack);

        assert!(validate_artifact_digest(digest.as_str()).is_ok());
        assert!(boundary.computes_manifest_digest);
        assert!(!boundary.computes_referenced_artifact_hashes);
        assert!(!boundary.computes_hashes);
        assert!(!boundary.normalizes_hashes);
        assert!(!boundary.writes_proofpack);
        assert!(!boundary.requires_runtime_storage);
        assert!(!boundary.writes_cli_output);
        assert!(!boundary.creates_acceptance_claim);
        assert!(!boundary.writes_final_decision);
    }

    #[test]
    fn link_hash_integrity_report_requires_digests_for_declared_refs() {
        let proofpack = sample_proofpack_without_digests()
            .with_artifact_digest(ProofArtifactDigest::new(
                ProofArtifactKind::GateDecision,
                ProofArtifactRef::new("decision_local_001").expect("artifact ref should be valid"),
                ProofArtifactHash::new(PROOF_HASH_GATE_DECISION)
                    .expect("artifact hash should be valid"),
            ))
            .with_artifact_digest(ProofArtifactDigest::new(
                ProofArtifactKind::Contract,
                ProofArtifactRef::new("contract_local_001").expect("artifact ref should be valid"),
                ProofArtifactHash::new(PROOF_HASH_CONTRACT).expect("artifact hash should be valid"),
            ));

        let report = proofpack.link_hash_integrity_report();
        let missing_receipt =
            digest_requirement(ProofArtifactKind::RunReceipt, "receipt_local_001");
        let missing_eval = digest_requirement(
            ProofArtifactKind::Eval,
            "work/reports/2026-04-25-gate-decision-kernel-minimal-v0-1.md",
        );
        let missing_event = digest_requirement(ProofArtifactKind::Event, "evt_0000000000000001");
        let missing_output =
            digest_requirement(ProofArtifactKind::OutputArtifact, "target/debug/punk");
        let matching_decision =
            ProofGateDecisionRef::new("decision_local_001").expect("decision ref should be valid");

        assert_eq!(report.required_digest_refs().len(), 6);
        assert_eq!(report.missing_digest_refs().len(), 4);
        assert!(report.has_missing_required_digests());
        assert!(!report.is_complete());
        assert!(report.missing_digest_refs().contains(&missing_receipt));
        assert!(report.missing_digest_refs().contains(&missing_eval));
        assert!(report.missing_digest_refs().contains(&missing_event));
        assert!(report.missing_digest_refs().contains(&missing_output));
        assert!(!proofpack.has_complete_link_hash_integrity());
        assert!(!proofpack.is_matching_proof_ready_for_acceptance(&matching_decision));
    }

    #[test]
    fn link_hash_integrity_accepts_declared_optional_refs_when_digest_entries_match() {
        let proofpack = sample_proofpack();
        let report = proofpack.link_hash_integrity_report();
        let matching_decision =
            ProofGateDecisionRef::new("decision_local_001").expect("decision ref should be valid");

        assert_eq!(report.required_digest_refs().len(), 6);
        assert!(report.missing_digest_refs().is_empty());
        assert!(report.is_complete());
        assert!(proofpack.has_complete_link_hash_integrity());
        assert!(proofpack.is_matching_proof_ready_for_acceptance(&matching_decision));
    }

    #[test]
    fn minimal_link_hash_integrity_requires_only_declared_core_refs() {
        let proofpack = Proofpack::new(
            ProofpackId::new("proofpack_minimal_001").expect("proofpack id should be valid"),
            ProofGateDecisionRef::new("decision_minimal_001")
                .expect("decision ref should be valid"),
            vec![ProofContractRef::new("contract_minimal_001")
                .expect("contract ref should be valid")],
            vec![ProofRunReceiptRef::new("receipt_minimal_001")
                .expect("receipt ref should be valid")],
            ProofCreatedAt::new("2026-04-25T20:02:00Z").expect("created_at should be valid"),
            vec![
                ProofBoundaryNote::new("Minimal proofpack still needs core ref digests.")
                    .expect("boundary note should be valid"),
            ],
        )
        .expect("minimal proofpack should be valid")
        .with_artifact_digest(ProofArtifactDigest::new(
            ProofArtifactKind::GateDecision,
            ProofArtifactRef::new("decision_minimal_001").expect("artifact ref should be valid"),
            ProofArtifactHash::new(PROOF_HASH_GATE_DECISION)
                .expect("artifact hash should be valid"),
        ))
        .with_artifact_digest(ProofArtifactDigest::new(
            ProofArtifactKind::Contract,
            ProofArtifactRef::new("contract_minimal_001").expect("artifact ref should be valid"),
            ProofArtifactHash::new(PROOF_HASH_CONTRACT).expect("artifact hash should be valid"),
        ))
        .with_artifact_digest(ProofArtifactDigest::new(
            ProofArtifactKind::RunReceipt,
            ProofArtifactRef::new("receipt_minimal_001").expect("artifact ref should be valid"),
            ProofArtifactHash::new(PROOF_HASH_RUN_RECEIPT).expect("artifact hash should be valid"),
        ));

        let report = proofpack.link_hash_integrity_report();

        assert_eq!(report.required_digest_refs().len(), 3);
        assert!(report.is_complete());
        assert!(proofpack.has_complete_link_hash_integrity());
    }

    #[test]
    fn link_hash_integrity_matches_kind_and_ref_without_hash_normalization() {
        let proofpack = sample_proofpack_without_digests()
            .with_artifact_digest(ProofArtifactDigest::new(
                ProofArtifactKind::Contract,
                ProofArtifactRef::new("decision_local_001").expect("artifact ref should be valid"),
                ProofArtifactHash::new(PROOF_HASH_CONTRACT).expect("artifact hash should be valid"),
            ))
            .with_artifact_digest(ProofArtifactDigest::new(
                ProofArtifactKind::GateDecision,
                ProofArtifactRef::new("decision_local_other")
                    .expect("artifact ref should be valid"),
                ProofArtifactHash::new(PROOF_HASH_OTHER).expect("artifact hash should be valid"),
            ));

        let required_decision =
            digest_requirement(ProofArtifactKind::GateDecision, "decision_local_001");
        let wrong_kind_same_ref =
            digest_requirement(ProofArtifactKind::Contract, "decision_local_001");
        let report = proofpack.link_hash_integrity_report();

        assert!(!proofpack.has_artifact_digest_for(&required_decision));
        assert!(proofpack.has_artifact_digest_for(&wrong_kind_same_ref));
        assert!(report.missing_digest_refs().contains(&required_decision));
        assert!(report.has_missing_required_digests());
        assert!(!proofpack.boundary().computes_hashes);
        assert!(!proofpack.boundary().normalizes_hashes);
    }

    #[test]
    fn proofpack_requires_post_gate_refs_and_boundary_notes() {
        let id = ProofpackId::new("proofpack_local_002").expect("proofpack id should be valid");
        let decision =
            ProofGateDecisionRef::new("decision_local_002").expect("decision ref should be valid");
        let created_at =
            ProofCreatedAt::new("2026-04-25T20:01:00Z").expect("created_at should be valid");
        let note =
            ProofBoundaryNote::new("Post-gate provenance only").expect("note should be valid");

        assert_eq!(
            Proofpack::new(
                id.clone(),
                decision.clone(),
                Vec::new(),
                vec![ProofRunReceiptRef::new("receipt").expect("receipt ref should be valid")],
                created_at.clone(),
                vec![note.clone()],
            ),
            Err(ProofpackError::MissingContractRefs)
        );
        assert_eq!(
            Proofpack::new(
                id.clone(),
                decision.clone(),
                vec![ProofContractRef::new("contract").expect("contract ref should be valid")],
                Vec::new(),
                created_at.clone(),
                vec![note],
            ),
            Err(ProofpackError::MissingRunReceiptRefs)
        );
        assert_eq!(
            Proofpack::new(
                id,
                decision,
                vec![ProofContractRef::new("contract").expect("contract ref should be valid")],
                vec![ProofRunReceiptRef::new("receipt").expect("receipt ref should be valid")],
                created_at,
                Vec::new(),
            ),
            Err(ProofpackError::MissingBoundaryNotes)
        );
    }

    #[test]
    fn proofpack_is_post_gate_but_not_decision_authority() {
        let proofpack = sample_proofpack();
        let boundary = proofpack.boundary();

        assert!(proofpack.is_post_gate_provenance_bundle());
        assert!(!proofpack.is_final_decision_authority());
        assert!(boundary.models_proofpack);
        assert!(!boundary.writes_proofpack);
        assert!(!boundary.writes_final_decision);
        assert!(boundary.post_gate_only);
        assert!(boundary.checks_structural_link_hash_integrity);
        assert!(!boundary.computes_hashes);
        assert!(!boundary.normalizes_hashes);
    }

    #[test]
    fn proofpack_has_no_runtime_writer_or_acceptance_side_effects() {
        let proofpack = sample_proofpack();
        let boundary = proofpack.boundary();

        assert!(!boundary.requires_runtime_storage);
        assert!(!boundary.writes_cli_output);
        assert!(!boundary.creates_acceptance_claim);
        assert!(!boundary.implies_acceptance_by_itself);
        assert!(!proofpack.creates_acceptance_claim());
        assert!(!proofpack.can_claim_acceptance_by_itself());
    }

    #[test]
    fn positive_acceptance_requires_accepting_decision_and_matching_proofpack() {
        let proofpack = sample_proofpack();
        let matching_decision =
            ProofGateDecisionRef::new("decision_local_001").expect("decision ref should be valid");
        let other_decision = ProofGateDecisionRef::new("decision_local_other")
            .expect("decision ref should be valid");

        assert!(proofpack.matches_gate_decision_ref(&matching_decision));
        assert!(proofpack.is_matching_proof_ready_for_acceptance(&matching_decision));
        assert!(!proofpack.matches_gate_decision_ref(&other_decision));
        assert!(!proofpack.is_matching_proof_ready_for_acceptance(&other_decision));
        assert!(positive_acceptance_preconditions_met(
            PositiveAcceptanceInputs {
                accepting_gate_decision: true,
                matching_proofpack: proofpack
                    .is_matching_proof_ready_for_acceptance(&matching_decision),
            },
        ));
        assert!(!positive_acceptance_preconditions_met(
            PositiveAcceptanceInputs {
                accepting_gate_decision: false,
                matching_proofpack: true,
            },
        ));
        assert!(!positive_acceptance_preconditions_met(
            PositiveAcceptanceInputs {
                accepting_gate_decision: true,
                matching_proofpack: false,
            },
        ));
        assert!(!positive_acceptance_preconditions_met(
            PositiveAcceptanceInputs {
                accepting_gate_decision: false,
                matching_proofpack: false,
            },
        ));
    }

    #[test]
    fn proofpack_writer_operation_outcome_vocabulary_is_stable() {
        assert_eq!(
            ProofpackWriterOperationOutcome::PlannedOnly.as_str(),
            "planned_only"
        );
        assert_eq!(ProofpackWriterOperationOutcome::Written.as_str(), "written");
        assert_eq!(
            ProofpackWriterOperationOutcome::AlreadyExistsMatching.as_str(),
            "already_exists_matching"
        );
        assert_eq!(
            ProofpackWriterOperationOutcome::ConflictExistingDifferent.as_str(),
            "conflict_existing_different"
        );
        assert_eq!(
            ProofpackWriterOperationOutcome::PreflightFailed.as_str(),
            "preflight_failed"
        );
        assert_eq!(
            ProofpackWriterOperationOutcome::WriteFailed.as_str(),
            "write_failed"
        );
        assert_eq!(
            ProofpackWriterOperationOutcome::PartialWriteDetected.as_str(),
            "partial_write_detected"
        );
        assert_eq!(
            ProofpackWriterOperationOutcome::IndexUpdateFailed.as_str(),
            "index_update_failed"
        );
        assert_eq!(
            ProofpackWriterOperationOutcome::LatestPointerUpdateFailed.as_str(),
            "latest_pointer_update_failed"
        );
        assert_eq!(ProofpackWriterOperationOutcome::Aborted.as_str(), "aborted");

        assert!(ProofpackWriterOperationOutcome::Written
            .can_represent_canonical_artifact_availability());
        assert!(ProofpackWriterOperationOutcome::AlreadyExistsMatching
            .can_represent_canonical_artifact_availability());
        assert!(!ProofpackWriterOperationOutcome::IndexUpdateFailed
            .can_represent_canonical_artifact_availability());
        assert!(!ProofpackWriterOperationOutcome::LatestPointerUpdateFailed
            .can_represent_canonical_artifact_availability());
    }

    #[test]
    fn proofpack_writer_operation_evidence_is_evidence_only() {
        let evidence = sample_writer_operation_evidence(
            ProofpackWriterOperationOutcome::Written,
            ProofpackWriterCanonicalArtifactStatus::Written,
            ProofpackWriterSideEffectStatus::NotSelected,
            ProofpackWriterSideEffectStatus::NotSelected,
        );
        let boundary = evidence.boundary();

        assert_eq!(
            evidence.schema_version(),
            PROOFPACK_WRITER_OPERATION_EVIDENCE_SCHEMA_VERSION
        );
        assert_eq!(evidence.operation_id().as_str(), "writer_op_local_001");
        assert_eq!(evidence.operation_kind().as_str(), "write");
        assert_eq!(evidence.proofpack_id().as_str(), "proofpack_local_001");
        assert_eq!(evidence.attempted_at().as_str(), "2026-04-26T13:00:00Z");
        assert_eq!(
            evidence.target_ref().as_str(),
            "future/.punk/proofs/proofpack_local_001.json"
        );
        assert!(evidence.canonical_artifact_available());
        assert!(evidence.represents_new_canonical_artifact_write());
        assert!(evidence.is_evidence_only());
        assert!(boundary.models_writer_operation_evidence);
        assert!(boundary.separates_canonical_artifact_from_indexes);
        assert!(!boundary.writes_proofpack);
        assert!(!boundary.writes_writer_operation_evidence);
        assert!(!boundary.writes_final_decision);
        assert!(!boundary.creates_acceptance_claim);
        assert!(!boundary.requires_runtime_storage);
        assert!(!boundary.writes_cli_output);
        assert!(!boundary.writes_schema_files);
        assert!(!boundary.is_proofpack_artifact);
        assert!(!boundary.is_run_receipt);
        assert!(!boundary.is_schema_validation);
    }

    #[test]
    fn proofpack_writer_preflight_plan_success_is_planned_only() {
        let proofpack = sample_proofpack();
        let plan = sample_writer_preflight_plan(
            &proofpack,
            vec![
                ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite,
                ProofpackWriterPlannedSideEffect::IndexUpdate,
                ProofpackWriterPlannedSideEffect::LatestPointerUpdate,
            ],
            vec![ProofBoundaryNote::new(
                "Preflight plan is side-effect-free and does not write proofpacks.",
            )
            .expect("boundary note should be valid")],
        );
        let evidence = plan
            .to_operation_evidence(
                ProofpackWriterOperationId::new("writer_preflight_local_001")
                    .expect("operation id should be valid"),
                ProofpackWriterAttemptedAt::new("2026-04-26T14:30:00Z")
                    .expect("attempted_at should be valid"),
            )
            .expect("operation evidence should be derivable");

        assert_eq!(
            plan.schema_version(),
            PROOFPACK_WRITER_PREFLIGHT_PLAN_SCHEMA_VERSION
        );
        assert_eq!(plan.proofpack_id(), proofpack.id());
        assert_eq!(
            plan.target_ref().as_str(),
            "future/.punk/proofs/proofpack_local_001.json"
        );
        assert_eq!(
            plan.manifest_self_digest(),
            &compute_proofpack_manifest_digest(&proofpack)
        );
        assert_eq!(plan.status().as_str(), "ready");
        assert!(plan.is_writer_ready());
        assert!(plan.missing_preconditions().is_empty());
        assert!(plan.plans_side_effect(ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite));
        assert!(plan.plans_side_effect(ProofpackWriterPlannedSideEffect::IndexUpdate));
        assert!(plan.plans_side_effect(ProofpackWriterPlannedSideEffect::LatestPointerUpdate));

        assert_eq!(evidence.operation_kind().as_str(), "planned_only");
        assert_eq!(evidence.outcome().as_str(), "planned_only");
        assert_eq!(
            evidence.canonical_artifact_status(),
            ProofpackWriterCanonicalArtifactStatus::NotAttempted
        );
        assert_eq!(
            evidence.index_status(),
            ProofpackWriterSideEffectStatus::NotAttempted
        );
        assert_eq!(
            evidence.latest_pointer_status(),
            ProofpackWriterSideEffectStatus::NotAttempted
        );
        assert!(!evidence.canonical_artifact_available());
        assert!(!evidence.creates_acceptance_claim());
    }

    #[test]
    fn proofpack_writer_preflight_plan_records_missing_preconditions() {
        let incomplete_proofpack =
            Proofpack::new(
                ProofpackId::new("proofpack_incomplete_001").expect("proofpack id should be valid"),
                ProofGateDecisionRef::new("decision_local_001")
                    .expect("gate decision ref should be valid"),
                vec![ProofContractRef::new("contract_local_001")
                    .expect("contract ref should be valid")],
                vec![ProofRunReceiptRef::new("receipt_local_001")
                    .expect("run receipt ref should be valid")],
                ProofCreatedAt::new("2026-04-25T21:00:00Z").expect("created_at should be valid"),
                vec![
                    ProofBoundaryNote::new("Incomplete proofpack lacks required digest entries.")
                        .expect("boundary note should be valid"),
                ],
            )
            .expect("proofpack should be structurally valid");
        let plan = sample_writer_preflight_plan(&incomplete_proofpack, vec![], vec![]);
        let evidence = plan
            .to_operation_evidence(
                ProofpackWriterOperationId::new("writer_preflight_local_002")
                    .expect("operation id should be valid"),
                ProofpackWriterAttemptedAt::new("2026-04-26T14:31:00Z")
                    .expect("attempted_at should be valid"),
            )
            .expect("preflight failed evidence should be derivable");

        assert_eq!(plan.status().as_str(), "missing_preconditions");
        assert!(!plan.is_writer_ready());
        assert!(plan.has_missing_preconditions());
        assert_eq!(
            plan.missing_preconditions(),
            &[
                ProofpackWriterMissingPrecondition::MissingRequiredArtifactDigests,
                ProofpackWriterMissingPrecondition::MissingPlannedSideEffects,
                ProofpackWriterMissingPrecondition::MissingBoundaryNotes,
            ]
        );
        assert_eq!(
            plan.missing_preconditions()[0].as_str(),
            "missing_required_artifact_digests"
        );
        assert_eq!(plan.planned_side_effects().len(), 0);
        assert_eq!(plan.boundary_notes().len(), 0);
        assert_eq!(evidence.outcome().as_str(), "preflight_failed");
        assert_eq!(
            evidence.canonical_artifact_status(),
            ProofpackWriterCanonicalArtifactStatus::NotAttempted
        );
        assert_eq!(
            evidence.index_status(),
            ProofpackWriterSideEffectStatus::NotSelected
        );
        assert_eq!(
            evidence.latest_pointer_status(),
            ProofpackWriterSideEffectStatus::NotSelected
        );
        assert!(!evidence.canonical_artifact_available());
        assert!(!evidence.can_claim_acceptance_by_itself());
    }

    #[test]
    fn proofpack_writer_preflight_plan_is_evidence_only_and_setup_neutral() {
        let proofpack = sample_proofpack();
        let plan = sample_writer_preflight_plan(
            &proofpack,
            vec![ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite],
            vec![ProofBoundaryNote::new(
                "Preflight planning preserves the gate/proof authority boundary.",
            )
            .expect("boundary note should be valid")],
        );
        let boundary = plan.boundary();

        assert!(plan.is_evidence_only());
        assert!(boundary.models_writer_preflight_plan);
        assert!(boundary.computes_manifest_self_digest);
        assert!(boundary.separates_preflight_from_artifact_availability);
        assert!(!boundary.planned_side_effects_are_attempts);
        assert!(!plan.writes_proofpack());
        assert!(!boundary.writes_writer_operation_evidence);
        assert!(!boundary.writes_final_decision);
        assert!(!plan.creates_acceptance_claim());
        assert!(!plan.requires_runtime_storage());
        assert!(!plan.writes_cli_output());
        assert!(!boundary.writes_schema_files);
    }

    #[test]
    fn proofpack_writer_file_io_plan_records_explicit_targets_without_writing() {
        let proofpack = sample_proofpack();
        let preflight_plan = sample_writer_preflight_plan(
            &proofpack,
            vec![
                ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite,
                ProofpackWriterPlannedSideEffect::IndexUpdate,
                ProofpackWriterPlannedSideEffect::LatestPointerUpdate,
            ],
            vec![ProofBoundaryNote::new(
                "Preflight plan is side-effect-free and does not write proofpacks.",
            )
            .expect("boundary note should be valid")],
        );
        let plan = sample_writer_file_io_plan(
            &preflight_plan,
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
                ProofpackWriterOperationId::new("writer_file_io_plan_local_001")
                    .expect("operation id should be valid"),
                ProofpackWriterAttemptedAt::new("2026-04-26T15:00:00Z")
                    .expect("attempted_at should be valid"),
            )
            .expect("planned-only operation evidence should be derivable");

        assert_eq!(
            plan.schema_version(),
            PROOFPACK_WRITER_FILE_IO_PLAN_SCHEMA_VERSION
        );
        assert_eq!(plan.proofpack_id(), proofpack.id());
        assert_eq!(plan.storage_root_ref().as_str(), "repo_runtime_proofs_root");
        assert_eq!(
            plan.target_artifact_ref().as_str(),
            "future/.punk/proofs/proofpack_local_001.json"
        );
        assert_eq!(
            plan.target_path_ref().as_str(),
            "future/.punk/proofs/proofpack_local_001.json"
        );
        assert_eq!(
            plan.manifest_self_digest(),
            preflight_plan.manifest_self_digest()
        );
        assert_eq!(plan.status().as_str(), "ready");
        assert!(plan.is_file_io_ready());
        assert!(plan.selects_canonical_artifact_write());
        assert!(plan.selects_index_update());
        assert!(plan.selects_latest_pointer_update());
        assert!(plan.tracks_conflict_visibility());
        assert!(plan.tracks_rollback_visibility());
        assert_eq!(evidence.operation_kind().as_str(), "planned_only");
        assert_eq!(evidence.outcome().as_str(), "planned_only");
        assert_eq!(
            evidence.canonical_artifact_status(),
            ProofpackWriterCanonicalArtifactStatus::NotAttempted
        );
        assert!(!evidence.canonical_artifact_available());
        assert!(!evidence.creates_acceptance_claim());
    }

    #[test]
    fn proofpack_writer_file_io_plan_exposes_idempotency_and_conflict_policy() {
        let proofpack = sample_proofpack();
        let preflight_plan = sample_writer_preflight_plan(
            &proofpack,
            vec![ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite],
            vec![ProofBoundaryNote::new("Preflight is complete.")
                .expect("boundary note should be valid")],
        );
        let plan = sample_writer_file_io_plan(
            &preflight_plan,
            vec![
                ProofpackWriterFileIoFailureVisibility::ExistingTargetMatching,
                ProofpackWriterFileIoFailureVisibility::ExistingTargetDifferent,
                ProofpackWriterFileIoFailureVisibility::PartialCanonicalArtifactAmbiguous,
            ],
            vec![ProofBoundaryNote::new(
                "Idempotency and conflict handling are modeled as policy, not inferred from indexes.",
            )
            .expect("boundary note should be valid")],
        );

        assert_eq!(plan.write_policy().as_str(), "idempotent_if_matching");
        assert!(plan.write_policy().supports_idempotency());
        assert!(!plan.write_policy().allows_silent_overwrite());
        assert_eq!(plan.idempotency_basis().as_str(), "manifest_self_digest");
        assert_eq!(plan.temp_atomic_policy().as_str(), "atomic_sibling_temp");
        assert!(plan.temp_atomic_policy().prefers_atomic_move());
        assert!(
            plan.tracks_failure(ProofpackWriterFileIoFailureVisibility::ExistingTargetDifferent)
        );
        assert_eq!(
            ProofpackWriterFileIoFailureVisibility::ExistingTargetDifferent.as_str(),
            "existing_target_different"
        );
        assert!(!plan.target_path_is_authority());
        assert!(!plan.index_latest_are_canonical());
    }

    #[test]
    fn proofpack_writer_file_io_plan_records_blockers_without_artifact_availability() {
        let incomplete_proofpack =
            Proofpack::new(
                ProofpackId::new("proofpack_file_io_blocked_001")
                    .expect("proofpack id should be valid"),
                ProofGateDecisionRef::new("decision_local_001")
                    .expect("gate decision ref should be valid"),
                vec![ProofContractRef::new("contract_local_001")
                    .expect("contract ref should be valid")],
                vec![ProofRunReceiptRef::new("receipt_local_001")
                    .expect("run receipt ref should be valid")],
                ProofCreatedAt::new("2026-04-25T22:00:00Z").expect("created_at should be valid"),
                vec![
                    ProofBoundaryNote::new("Incomplete proofpack lacks digest entries.")
                        .expect("boundary note should be valid"),
                ],
            )
            .expect("proofpack should be structurally valid");
        let preflight_plan = sample_writer_preflight_plan(&incomplete_proofpack, vec![], vec![]);
        let plan = sample_writer_file_io_plan(&preflight_plan, vec![], vec![]);
        let evidence = plan
            .to_operation_evidence(
                ProofpackWriterOperationId::new("writer_file_io_plan_blocked_001")
                    .expect("operation id should be valid"),
                ProofpackWriterAttemptedAt::new("2026-04-26T15:01:00Z")
                    .expect("attempted_at should be valid"),
            )
            .expect("blocked operation evidence should be derivable");

        assert_eq!(plan.status().as_str(), "file_io_blocked");
        assert!(!plan.is_file_io_ready());
        assert!(plan.has_file_io_blockers());
        assert_eq!(
            plan.blockers(),
            &[
                ProofpackWriterFileIoBlocker::PreflightPlanMissingPreconditions,
                ProofpackWriterFileIoBlocker::MissingCanonicalArtifactWriteSelection,
                ProofpackWriterFileIoBlocker::MissingErrorRollbackVisibility,
                ProofpackWriterFileIoBlocker::MissingBoundaryNotes,
            ]
        );
        assert_eq!(
            plan.blockers()[0].as_str(),
            "preflight_plan_missing_preconditions"
        );
        assert_eq!(evidence.outcome().as_str(), "preflight_failed");
        assert_eq!(
            evidence.canonical_artifact_status(),
            ProofpackWriterCanonicalArtifactStatus::NotAttempted
        );
        assert!(!evidence.canonical_artifact_available());
        assert!(!evidence.can_claim_acceptance_by_itself());
    }

    #[test]
    fn proofpack_writer_file_io_plan_is_evidence_only_and_setup_neutral() {
        let proofpack = sample_proofpack();
        let preflight_plan = sample_writer_preflight_plan(
            &proofpack,
            vec![ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite],
            vec![ProofBoundaryNote::new("Preflight is complete.")
                .expect("boundary note should be valid")],
        );
        let plan = sample_writer_file_io_plan(
            &preflight_plan,
            vec![ProofpackWriterFileIoFailureVisibility::StorageRootDisallowed],
            vec![
                ProofBoundaryNote::new("File IO plan is model-only and setup-neutral.")
                    .expect("boundary note should be valid"),
            ],
        );
        let boundary = plan.boundary();

        assert!(plan.is_evidence_only());
        assert!(boundary.models_writer_file_io_plan);
        assert!(boundary.models_explicit_storage_root);
        assert!(boundary.models_target_artifact_ref);
        assert!(boundary.models_target_path_ref);
        assert!(boundary.models_write_policy);
        assert!(boundary.models_idempotency_basis);
        assert!(boundary.models_temp_atomic_policy);
        assert!(boundary.models_error_rollback_visibility);
        assert!(boundary.separates_file_io_plan_from_artifact_availability);
        assert!(!plan.touches_filesystem());
        assert!(!plan.writes_proofpack());
        assert!(!boundary.writes_writer_operation_evidence);
        assert!(!boundary.writes_final_decision);
        assert!(!plan.creates_acceptance_claim());
        assert!(!plan.requires_runtime_storage());
        assert!(!plan.writes_cli_output());
        assert!(!boundary.writes_schema_files);
        assert!(!boundary.target_path_is_authority);
        assert!(!boundary.index_latest_are_canonical);
    }

    #[test]
    fn writer_operation_write_success_does_not_imply_acceptance() {
        let evidence = sample_writer_operation_evidence(
            ProofpackWriterOperationOutcome::Written,
            ProofpackWriterCanonicalArtifactStatus::Written,
            ProofpackWriterSideEffectStatus::Completed,
            ProofpackWriterSideEffectStatus::Completed,
        );

        assert!(evidence.canonical_artifact_available());
        assert!(!evidence.is_final_decision_authority());
        assert!(!evidence.creates_acceptance_claim());
        assert!(!evidence.can_claim_acceptance_by_itself());
        assert!(!positive_acceptance_preconditions_met(
            PositiveAcceptanceInputs {
                accepting_gate_decision: false,
                matching_proofpack: evidence.canonical_artifact_available(),
            },
        ));
    }

    #[test]
    fn writer_operation_idempotent_existing_artifact_is_explicit() {
        let evidence = sample_writer_operation_evidence(
            ProofpackWriterOperationOutcome::AlreadyExistsMatching,
            ProofpackWriterCanonicalArtifactStatus::AlreadyExistsMatching,
            ProofpackWriterSideEffectStatus::NotSelected,
            ProofpackWriterSideEffectStatus::NotSelected,
        );

        assert!(evidence.canonical_artifact_available());
        assert_eq!(
            evidence.canonical_artifact_status().as_str(),
            "already_exists_matching"
        );
        assert!(!evidence.represents_new_canonical_artifact_write());
        assert!(!evidence.has_conflict());
        assert!(!evidence.has_index_or_latest_pointer_failure());
    }

    #[test]
    fn writer_operation_conflict_existing_artifact_is_non_available() {
        let evidence = sample_writer_operation_evidence(
            ProofpackWriterOperationOutcome::ConflictExistingDifferent,
            ProofpackWriterCanonicalArtifactStatus::ConflictExistingDifferent,
            ProofpackWriterSideEffectStatus::NotSelected,
            ProofpackWriterSideEffectStatus::NotSelected,
        );

        assert!(!evidence.canonical_artifact_available());
        assert!(evidence.has_conflict());
        assert!(!evidence.represents_new_canonical_artifact_write());
        assert!(!evidence.creates_acceptance_claim());
    }

    #[test]
    fn writer_operation_partial_and_side_effect_failures_remain_visible() {
        let partial = sample_writer_operation_evidence(
            ProofpackWriterOperationOutcome::PartialWriteDetected,
            ProofpackWriterCanonicalArtifactStatus::PartialWriteDetected,
            ProofpackWriterSideEffectStatus::NotSelected,
            ProofpackWriterSideEffectStatus::NotSelected,
        );
        let index_failed = sample_writer_operation_evidence(
            ProofpackWriterOperationOutcome::IndexUpdateFailed,
            ProofpackWriterCanonicalArtifactStatus::Written,
            ProofpackWriterSideEffectStatus::Failed,
            ProofpackWriterSideEffectStatus::Completed,
        );
        let latest_failed = sample_writer_operation_evidence(
            ProofpackWriterOperationOutcome::LatestPointerUpdateFailed,
            ProofpackWriterCanonicalArtifactStatus::AlreadyExistsMatching,
            ProofpackWriterSideEffectStatus::Completed,
            ProofpackWriterSideEffectStatus::Failed,
        );

        assert!(partial.has_partial_write());
        assert!(!partial.canonical_artifact_available());
        assert!(index_failed.canonical_artifact_available());
        assert!(index_failed.has_index_or_latest_pointer_failure());
        assert_eq!(index_failed.index_status().as_str(), "failed");
        assert_eq!(index_failed.latest_pointer_status().as_str(), "completed");
        assert!(latest_failed.canonical_artifact_available());
        assert!(latest_failed.has_index_or_latest_pointer_failure());
        assert_eq!(latest_failed.index_status().as_str(), "completed");
        assert_eq!(latest_failed.latest_pointer_status().as_str(), "failed");
    }

    #[test]
    fn writer_operation_evidence_rejects_inconsistent_status() {
        let inconsistent = ProofpackWriterOperationEvidence::new(
            ProofpackWriterOperationId::new("writer_op_local_002")
                .expect("operation id should be valid"),
            ProofpackWriterOperationKind::Write,
            ProofpackId::new("proofpack_local_002").expect("proofpack id should be valid"),
            ProofpackWriterAttemptedAt::new("2026-04-26T13:01:00Z")
                .expect("attempted_at should be valid"),
            ProofpackWriterTargetRef::new("future/.punk/proofs/proofpack_local_002.json")
                .expect("target ref should be valid"),
            ProofpackWriterOperationOutcome::Written,
            ProofpackWriterCanonicalArtifactStatus::NotAttempted,
            ProofpackWriterSideEffectStatus::NotSelected,
            ProofpackWriterSideEffectStatus::NotSelected,
            vec![ProofBoundaryNote::new("status mismatch must stay visible")
                .expect("boundary note should be valid")],
        );

        assert_eq!(
            inconsistent,
            Err(ProofpackError::InconsistentWriterOperationEvidence)
        );
    }

    #[test]
    fn writer_operation_evidence_stays_setup_neutral() {
        let evidence = sample_writer_operation_evidence(
            ProofpackWriterOperationOutcome::PlannedOnly,
            ProofpackWriterCanonicalArtifactStatus::NotAttempted,
            ProofpackWriterSideEffectStatus::NotSelected,
            ProofpackWriterSideEffectStatus::NotSelected,
        );

        assert!(!evidence.requires_runtime_storage());
        assert!(!evidence.writes_cli_output());
        assert!(!evidence.is_proofpack_artifact());
        assert!(!evidence.is_run_receipt());
        assert!(!evidence.is_schema_validation());
        assert!(evidence.is_evidence_only());
        assert_eq!(evidence.boundary_notes().len(), 1);
    }

    #[test]
    fn artifact_kind_vocabulary_is_stable() {
        assert_eq!(ProofArtifactKind::GateDecision.as_str(), "gate_decision");
        assert_eq!(ProofArtifactKind::Contract.as_str(), "contract");
        assert_eq!(ProofArtifactKind::RunReceipt.as_str(), "run_receipt");
        assert_eq!(ProofArtifactKind::Eval.as_str(), "eval");
        assert_eq!(ProofArtifactKind::Event.as_str(), "event");
        assert_eq!(
            ProofArtifactKind::OutputArtifact.as_str(),
            "output_artifact"
        );
    }

    #[test]
    fn refs_reject_empty_values() {
        assert_eq!(ProofpackId::new(" "), Err(ProofpackError::EmptyProofpackId));
        assert_eq!(
            ProofCreatedAt::new(" "),
            Err(ProofpackError::EmptyCreatedAt)
        );
        assert_eq!(
            ProofGateDecisionRef::new(" "),
            Err(ProofpackError::EmptyGateDecisionRef)
        );
        assert_eq!(
            ProofContractRef::new(" "),
            Err(ProofpackError::EmptyContractRef)
        );
        assert_eq!(
            ProofRunReceiptRef::new(" "),
            Err(ProofpackError::EmptyRunReceiptRef)
        );
        assert_eq!(ProofEvalRef::new(" "), Err(ProofpackError::EmptyEvalRef));
        assert_eq!(ProofEventRef::new(" "), Err(ProofpackError::EmptyEventRef));
        assert_eq!(
            ProofOutputArtifactRef::new(" "),
            Err(ProofpackError::EmptyOutputArtifactRef)
        );
        assert_eq!(
            ProofBoundaryNote::new(" "),
            Err(ProofpackError::EmptyBoundaryNote)
        );
        assert_eq!(
            ProofArtifactRef::new(" "),
            Err(ProofpackError::EmptyArtifactRef)
        );
        assert_eq!(
            ProofArtifactHash::new(" "),
            Err(ProofpackError::EmptyArtifactHash)
        );
        assert_eq!(
            ProofpackWriterOperationId::new(" "),
            Err(ProofpackError::EmptyWriterOperationId)
        );
        assert_eq!(
            ProofpackWriterAttemptedAt::new(" "),
            Err(ProofpackError::EmptyWriterAttemptedAt)
        );
        assert_eq!(
            ProofpackWriterTargetRef::new(" "),
            Err(ProofpackError::EmptyWriterTargetRef)
        );
    }

    #[test]
    fn artifact_hashes_validate_artifact_hash_policy_shape() {
        let lowercase_bare = "0123456789abcdef".repeat(4);
        let uppercase_digest = format!("sha256:{}", "ABCDEF0123456789".repeat(4));

        assert_eq!(
            ProofArtifactHash::new(PROOF_HASH_GATE_DECISION)
                .expect("canonical digest should be valid")
                .as_str(),
            PROOF_HASH_GATE_DECISION
        );
        assert_eq!(
            ProofArtifactHash::new("unknown"),
            Err(ProofpackError::InvalidArtifactHash(
                ArtifactHashPolicyError::PlaceholderDigest
            ))
        );
        assert_eq!(
            ProofArtifactHash::new(lowercase_bare),
            Err(ProofpackError::InvalidArtifactHash(
                ArtifactHashPolicyError::BareDigest
            ))
        );
        assert_eq!(
            ProofArtifactHash::new(uppercase_digest),
            Err(ProofpackError::InvalidArtifactHash(
                ArtifactHashPolicyError::InvalidDigestHex
            ))
        );
        assert_eq!(
            ProofArtifactHash::new("sha256:abc"),
            Err(ProofpackError::InvalidArtifactHash(
                ArtifactHashPolicyError::InvalidDigestLength {
                    expected: 64,
                    actual: 3
                }
            ))
        );
        assert_eq!(
            ProofArtifactHash::new(
                "sha512:0000000000000000000000000000000000000000000000000000000000000001"
            ),
            Err(ProofpackError::InvalidArtifactHash(
                ArtifactHashPolicyError::UnsupportedDigestAlgorithm
            ))
        );
    }
}
