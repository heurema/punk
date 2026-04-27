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
pub const PROOFPACK_WRITER_FILE_IO_OUTCOME_MODEL_SCHEMA_VERSION: &str =
    "punk.proofpack.writer_file_io_outcome_model.v0.1";
pub const PROOFPACK_WRITER_FILE_IO_ERROR_REASON_MODEL_SCHEMA_VERSION: &str =
    "punk.proofpack.writer_file_io_error_reason_model.v0.1";
pub const PROOFPACK_WRITER_TARGET_PATH_POLICY_MODEL_SCHEMA_VERSION: &str =
    "punk.proofpack.writer_target_path_policy_model.v0.1";
pub const PROOFPACK_WRITER_CANONICAL_ARTIFACT_MODEL_SCHEMA_VERSION: &str =
    "punk.proofpack.writer_canonical_artifact_model.v0.1";
pub const PROOFPACK_WRITER_TARGET_ARTIFACT_REF_POLICY_MODEL_SCHEMA_VERSION: &str =
    "punk.proofpack.writer_target_artifact_ref_policy_model.v0.1";
pub const PROOFPACK_WRITER_PREFLIGHT_INTEGRATION_MODEL_SCHEMA_VERSION: &str =
    "punk.proofpack.writer_preflight_integration_model.v0.1";
pub const PROOFPACK_WRITER_ACTIVE_BEHAVIOR_MODEL_SCHEMA_VERSION: &str =
    "punk.proofpack.writer_active_behavior_model.v0.1";

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterCanonicalArtifactLayout {
    ManifestOnlyJson,
}

impl ProofpackWriterCanonicalArtifactLayout {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ManifestOnlyJson => "manifest_only_json",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterNonCanonicalArtifactSurface {
    ManifestSelfDigestMetadata,
    WrapperMetadata,
    StorageRootRef,
    TargetArtifactRef,
    TargetPathRef,
    WriterOperationEvidence,
    SchemaValidationReport,
    IndexView,
    LatestPointer,
    CliOutput,
    ServiceMirror,
}

impl ProofpackWriterNonCanonicalArtifactSurface {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ManifestSelfDigestMetadata => "manifest_self_digest_metadata",
            Self::WrapperMetadata => "wrapper_metadata",
            Self::StorageRootRef => "storage_root_ref",
            Self::TargetArtifactRef => "target_artifact_ref",
            Self::TargetPathRef => "target_path_ref",
            Self::WriterOperationEvidence => "writer_operation_evidence",
            Self::SchemaValidationReport => "schema_validation_report",
            Self::IndexView => "index_view",
            Self::LatestPointer => "latest_pointer",
            Self::CliOutput => "cli_output",
            Self::ServiceMirror => "service_mirror",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofpackWriterCanonicalArtifactModel {
    schema_version: &'static str,
    proofpack_id: ProofpackId,
    layout: ProofpackWriterCanonicalArtifactLayout,
    canonical_body: String,
    manifest_self_digest: ArtifactDigest,
    non_canonical_surfaces: Vec<ProofpackWriterNonCanonicalArtifactSurface>,
    boundary_notes: Vec<ProofBoundaryNote>,
}

impl ProofpackWriterCanonicalArtifactModel {
    pub fn from_proofpack(proofpack: &Proofpack, boundary_notes: Vec<ProofBoundaryNote>) -> Self {
        let canonical_body = proofpack.render_manifest_json();
        let manifest_self_digest = compute_artifact_digest(canonical_body.as_bytes());

        Self {
            schema_version: PROOFPACK_WRITER_CANONICAL_ARTIFACT_MODEL_SCHEMA_VERSION,
            proofpack_id: proofpack.id().clone(),
            layout: ProofpackWriterCanonicalArtifactLayout::ManifestOnlyJson,
            canonical_body,
            manifest_self_digest,
            non_canonical_surfaces: proofpack_writer_non_canonical_artifact_surfaces(),
            boundary_notes: proofpack_writer_canonical_artifact_boundary_notes(boundary_notes),
        }
    }

    pub fn schema_version(&self) -> &str {
        self.schema_version
    }

    pub fn proofpack_id(&self) -> &ProofpackId {
        &self.proofpack_id
    }

    pub fn layout(&self) -> ProofpackWriterCanonicalArtifactLayout {
        self.layout
    }

    pub fn canonical_body_utf8(&self) -> &str {
        &self.canonical_body
    }

    pub fn canonical_body_bytes(&self) -> &[u8] {
        self.canonical_body.as_bytes()
    }

    pub fn manifest_self_digest(&self) -> &ArtifactDigest {
        &self.manifest_self_digest
    }

    pub fn non_canonical_surfaces(&self) -> &[ProofpackWriterNonCanonicalArtifactSurface] {
        &self.non_canonical_surfaces
    }

    pub fn boundary_notes(&self) -> &[ProofBoundaryNote] {
        &self.boundary_notes
    }

    pub fn is_manifest_only_layout(&self) -> bool {
        self.layout == ProofpackWriterCanonicalArtifactLayout::ManifestOnlyJson
    }

    pub fn canonical_body_matches_proofpack(&self, proofpack: &Proofpack) -> bool {
        self.canonical_body == proofpack.render_manifest_json()
    }

    pub fn manifest_self_digest_covers_canonical_body(&self) -> bool {
        compute_artifact_digest(self.canonical_body_bytes()) == self.manifest_self_digest
    }

    pub fn manifest_self_digest_is_embedded_in_canonical_body(&self) -> bool {
        self.canonical_body
            .contains(self.manifest_self_digest.as_str())
    }

    pub fn surface_is_non_canonical(
        &self,
        surface: ProofpackWriterNonCanonicalArtifactSurface,
    ) -> bool {
        self.non_canonical_surfaces.contains(&surface)
    }

    pub fn boundary(&self) -> ProofpackWriterCanonicalArtifactModelBoundary {
        proofpack_writer_canonical_artifact_model_boundary()
    }

    pub fn reads_filesystem(&self) -> bool {
        self.boundary().reads_filesystem
    }

    pub fn touches_filesystem(&self) -> bool {
        self.boundary().touches_filesystem
    }

    pub fn writes_proofpack(&self) -> bool {
        self.boundary().writes_proofpack
    }

    pub fn writes_writer_operation_evidence(&self) -> bool {
        self.boundary().writes_writer_operation_evidence
    }

    pub fn verifies_referenced_artifacts(&self) -> bool {
        self.boundary().verifies_referenced_artifacts
    }

    pub fn requires_runtime_storage(&self) -> bool {
        self.boundary().requires_runtime_storage
    }

    pub fn writes_cli_output(&self) -> bool {
        self.boundary().writes_cli_output
    }

    pub fn writes_schema_files(&self) -> bool {
        self.boundary().writes_schema_files
    }

    pub fn creates_acceptance_claim(&self) -> bool {
        self.boundary().creates_acceptance_claim
    }

    pub fn uses_indexes_or_latest_as_authority(&self) -> bool {
        self.boundary().uses_indexes_or_latest_as_authority
    }

    pub fn can_claim_acceptance_by_itself(&self) -> bool {
        false
    }
}

fn proofpack_writer_non_canonical_artifact_surfaces(
) -> Vec<ProofpackWriterNonCanonicalArtifactSurface> {
    vec![
        ProofpackWriterNonCanonicalArtifactSurface::ManifestSelfDigestMetadata,
        ProofpackWriterNonCanonicalArtifactSurface::WrapperMetadata,
        ProofpackWriterNonCanonicalArtifactSurface::StorageRootRef,
        ProofpackWriterNonCanonicalArtifactSurface::TargetArtifactRef,
        ProofpackWriterNonCanonicalArtifactSurface::TargetPathRef,
        ProofpackWriterNonCanonicalArtifactSurface::WriterOperationEvidence,
        ProofpackWriterNonCanonicalArtifactSurface::SchemaValidationReport,
        ProofpackWriterNonCanonicalArtifactSurface::IndexView,
        ProofpackWriterNonCanonicalArtifactSurface::LatestPointer,
        ProofpackWriterNonCanonicalArtifactSurface::CliOutput,
        ProofpackWriterNonCanonicalArtifactSurface::ServiceMirror,
    ]
}

fn proofpack_writer_canonical_artifact_boundary_notes(
    boundary_notes: Vec<ProofBoundaryNote>,
) -> Vec<ProofBoundaryNote> {
    if boundary_notes.is_empty() {
        return vec![ProofBoundaryNote::new(
            "Writer canonical artifact model is side-effect-free; canonical bytes are exact manifest renderer bytes and surrounding metadata stays non-canonical.",
        )
        .expect("fallback boundary note should be valid")];
    }

    boundary_notes
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProofpackWriterCanonicalArtifactModelBoundary {
    pub models_canonical_artifact_body: bool,
    pub canonical_body_is_manifest_renderer_bytes: bool,
    pub manifest_self_digest_covers_canonical_body: bool,
    pub manifest_self_digest_metadata_outside_body: bool,
    pub separates_non_canonical_metadata: bool,
    pub reads_filesystem: bool,
    pub touches_filesystem: bool,
    pub canonicalizes_host_paths: bool,
    pub writes_proofpack: bool,
    pub writes_writer_operation_evidence: bool,
    pub writes_final_decision: bool,
    pub creates_acceptance_claim: bool,
    pub requires_runtime_storage: bool,
    pub writes_cli_output: bool,
    pub writes_schema_files: bool,
    pub verifies_referenced_artifacts: bool,
    pub uses_indexes_or_latest_as_authority: bool,
    pub evidence_only: bool,
}

pub const fn proofpack_writer_canonical_artifact_model_boundary(
) -> ProofpackWriterCanonicalArtifactModelBoundary {
    ProofpackWriterCanonicalArtifactModelBoundary {
        models_canonical_artifact_body: true,
        canonical_body_is_manifest_renderer_bytes: true,
        manifest_self_digest_covers_canonical_body: true,
        manifest_self_digest_metadata_outside_body: true,
        separates_non_canonical_metadata: true,
        reads_filesystem: false,
        touches_filesystem: false,
        canonicalizes_host_paths: false,
        writes_proofpack: false,
        writes_writer_operation_evidence: false,
        writes_final_decision: false,
        creates_acceptance_claim: false,
        requires_runtime_storage: false,
        writes_cli_output: false,
        writes_schema_files: false,
        verifies_referenced_artifacts: false,
        uses_indexes_or_latest_as_authority: false,
        evidence_only: true,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterTargetArtifactRefPolicyStatus {
    Accepted,
    Rejected,
}

impl ProofpackWriterTargetArtifactRefPolicyStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterTargetArtifactRefPolicyReason {
    MissingProofpackId,
    MissingManifestSelfDigest,
    InvalidManifestSelfDigest,
}

impl ProofpackWriterTargetArtifactRefPolicyReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingProofpackId => "missing_proofpack_id",
            Self::MissingManifestSelfDigest => "missing_manifest_self_digest",
            Self::InvalidManifestSelfDigest => "invalid_manifest_self_digest",
        }
    }

    pub fn is_missing_precondition(self) -> bool {
        matches!(
            self,
            Self::MissingProofpackId | Self::MissingManifestSelfDigest
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofpackWriterTargetArtifactIdentity {
    proofpack_id: ProofpackId,
    manifest_self_digest: ArtifactDigest,
    layout: ProofpackWriterCanonicalArtifactLayout,
}

impl ProofpackWriterTargetArtifactIdentity {
    pub fn new(
        proofpack_id: ProofpackId,
        manifest_self_digest: ArtifactDigest,
        layout: ProofpackWriterCanonicalArtifactLayout,
    ) -> Self {
        Self {
            proofpack_id,
            manifest_self_digest,
            layout,
        }
    }

    pub fn proofpack_id(&self) -> &ProofpackId {
        &self.proofpack_id
    }

    pub fn manifest_self_digest(&self) -> &ArtifactDigest {
        &self.manifest_self_digest
    }

    pub fn layout(&self) -> ProofpackWriterCanonicalArtifactLayout {
        self.layout
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofpackWriterTargetArtifactRef(String);

impl ProofpackWriterTargetArtifactRef {
    pub fn from_identity(identity: &ProofpackWriterTargetArtifactIdentity) -> Self {
        let mut value = String::new();
        write!(
            &mut value,
            "proofpack:{}@{}",
            identity.proofpack_id().as_str(),
            identity.manifest_self_digest().as_str()
        )
        .expect("writing to String should succeed");
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofpackWriterTargetArtifactRefPolicyModel {
    schema_version: &'static str,
    identity: Option<ProofpackWriterTargetArtifactIdentity>,
    logical_ref: Option<ProofpackWriterTargetArtifactRef>,
    status: ProofpackWriterTargetArtifactRefPolicyStatus,
    reasons: Vec<ProofpackWriterTargetArtifactRefPolicyReason>,
    boundary_notes: Vec<ProofBoundaryNote>,
}

impl ProofpackWriterTargetArtifactRefPolicyModel {
    pub fn evaluate(
        proofpack_id: Option<ProofpackId>,
        manifest_self_digest: Option<ArtifactDigest>,
        layout: ProofpackWriterCanonicalArtifactLayout,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> Self {
        Self::from_parts(
            proofpack_id,
            manifest_self_digest,
            layout,
            Vec::new(),
            boundary_notes,
        )
    }

    pub fn evaluate_raw(
        proofpack_id: Option<&str>,
        manifest_self_digest: Option<&str>,
        layout: ProofpackWriterCanonicalArtifactLayout,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> Self {
        let mut reasons = Vec::new();
        let proofpack_id = match proofpack_id {
            Some(value) => match ProofpackId::new(value) {
                Ok(proofpack_id) => Some(proofpack_id),
                Err(ProofpackError::EmptyProofpackId) => {
                    reasons.push(ProofpackWriterTargetArtifactRefPolicyReason::MissingProofpackId);
                    None
                }
                Err(_) => None,
            },
            None => {
                reasons.push(ProofpackWriterTargetArtifactRefPolicyReason::MissingProofpackId);
                None
            }
        };
        let manifest_self_digest = match manifest_self_digest {
            Some(value) if value.trim().is_empty() => {
                reasons
                    .push(ProofpackWriterTargetArtifactRefPolicyReason::MissingManifestSelfDigest);
                None
            }
            Some(value) => match ArtifactDigest::new(value.trim()) {
                Ok(digest) => Some(digest),
                Err(_) => {
                    reasons.push(
                        ProofpackWriterTargetArtifactRefPolicyReason::InvalidManifestSelfDigest,
                    );
                    None
                }
            },
            None => {
                reasons
                    .push(ProofpackWriterTargetArtifactRefPolicyReason::MissingManifestSelfDigest);
                None
            }
        };

        Self::from_parts(
            proofpack_id,
            manifest_self_digest,
            layout,
            reasons,
            boundary_notes,
        )
    }

    pub fn from_canonical_artifact_model(
        canonical_artifact: &ProofpackWriterCanonicalArtifactModel,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> Self {
        Self::evaluate(
            Some(canonical_artifact.proofpack_id().clone()),
            Some(canonical_artifact.manifest_self_digest().clone()),
            canonical_artifact.layout(),
            boundary_notes,
        )
    }

    fn from_parts(
        proofpack_id: Option<ProofpackId>,
        manifest_self_digest: Option<ArtifactDigest>,
        layout: ProofpackWriterCanonicalArtifactLayout,
        mut reasons: Vec<ProofpackWriterTargetArtifactRefPolicyReason>,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> Self {
        if proofpack_id.is_none()
            && !reasons.contains(&ProofpackWriterTargetArtifactRefPolicyReason::MissingProofpackId)
        {
            reasons.push(ProofpackWriterTargetArtifactRefPolicyReason::MissingProofpackId);
        }
        if manifest_self_digest.is_none()
            && !reasons
                .contains(&ProofpackWriterTargetArtifactRefPolicyReason::MissingManifestSelfDigest)
            && !reasons
                .contains(&ProofpackWriterTargetArtifactRefPolicyReason::InvalidManifestSelfDigest)
        {
            reasons.push(ProofpackWriterTargetArtifactRefPolicyReason::MissingManifestSelfDigest);
        }

        let identity = match (proofpack_id, manifest_self_digest) {
            (Some(proofpack_id), Some(manifest_self_digest)) => {
                Some(ProofpackWriterTargetArtifactIdentity::new(
                    proofpack_id,
                    manifest_self_digest,
                    layout,
                ))
            }
            _ => None,
        };
        let logical_ref = identity
            .as_ref()
            .map(ProofpackWriterTargetArtifactRef::from_identity);
        let status = if identity.is_some() && reasons.is_empty() {
            ProofpackWriterTargetArtifactRefPolicyStatus::Accepted
        } else {
            ProofpackWriterTargetArtifactRefPolicyStatus::Rejected
        };

        Self {
            schema_version: PROOFPACK_WRITER_TARGET_ARTIFACT_REF_POLICY_MODEL_SCHEMA_VERSION,
            identity,
            logical_ref,
            status,
            reasons,
            boundary_notes: proofpack_writer_target_artifact_ref_policy_boundary_notes(
                boundary_notes,
            ),
        }
    }

    pub fn schema_version(&self) -> &str {
        self.schema_version
    }

    pub fn identity(&self) -> Option<&ProofpackWriterTargetArtifactIdentity> {
        self.identity.as_ref()
    }

    pub fn proofpack_id(&self) -> Option<&ProofpackId> {
        self.identity()
            .map(ProofpackWriterTargetArtifactIdentity::proofpack_id)
    }

    pub fn manifest_self_digest(&self) -> Option<&ArtifactDigest> {
        self.identity()
            .map(ProofpackWriterTargetArtifactIdentity::manifest_self_digest)
    }

    pub fn layout(&self) -> Option<ProofpackWriterCanonicalArtifactLayout> {
        self.identity()
            .map(ProofpackWriterTargetArtifactIdentity::layout)
    }

    pub fn logical_ref(&self) -> Option<&ProofpackWriterTargetArtifactRef> {
        self.logical_ref.as_ref()
    }

    pub fn logical_display_ref(&self) -> Option<&str> {
        self.logical_ref()
            .map(ProofpackWriterTargetArtifactRef::as_str)
    }

    pub fn status(&self) -> ProofpackWriterTargetArtifactRefPolicyStatus {
        self.status
    }

    pub fn reasons(&self) -> &[ProofpackWriterTargetArtifactRefPolicyReason] {
        &self.reasons
    }

    pub fn boundary_notes(&self) -> &[ProofBoundaryNote] {
        &self.boundary_notes
    }

    pub fn is_accepted(&self) -> bool {
        self.status == ProofpackWriterTargetArtifactRefPolicyStatus::Accepted
    }

    pub fn is_rejected(&self) -> bool {
        self.status == ProofpackWriterTargetArtifactRefPolicyStatus::Rejected
    }

    pub fn has_complete_identity(&self) -> bool {
        self.identity.is_some()
    }

    pub fn has_reason(&self, reason: ProofpackWriterTargetArtifactRefPolicyReason) -> bool {
        self.reasons.contains(&reason)
    }

    pub fn has_missing_precondition(&self) -> bool {
        self.reasons
            .iter()
            .any(|reason| reason.is_missing_precondition())
    }

    pub fn boundary(&self) -> ProofpackWriterTargetArtifactRefPolicyModelBoundary {
        proofpack_writer_target_artifact_ref_policy_model_boundary()
    }

    pub fn is_evidence_only(&self) -> bool {
        self.boundary().evidence_only
    }

    pub fn reads_filesystem(&self) -> bool {
        self.boundary().reads_filesystem
    }

    pub fn touches_filesystem(&self) -> bool {
        self.boundary().touches_filesystem
    }

    pub fn canonicalizes_host_paths(&self) -> bool {
        self.boundary().canonicalizes_host_paths
    }

    pub fn display_ref_is_filesystem_path(&self) -> bool {
        self.boundary().display_ref_is_filesystem_path
    }

    pub fn writes_proofpack(&self) -> bool {
        self.boundary().writes_proofpack
    }

    pub fn writes_writer_operation_evidence(&self) -> bool {
        self.boundary().writes_writer_operation_evidence
    }

    pub fn writes_indexes_or_latest(&self) -> bool {
        self.boundary().writes_indexes_or_latest
    }

    pub fn verifies_referenced_artifacts(&self) -> bool {
        self.boundary().verifies_referenced_artifacts
    }

    pub fn requires_runtime_storage(&self) -> bool {
        self.boundary().requires_runtime_storage
    }

    pub fn writes_cli_output(&self) -> bool {
        self.boundary().writes_cli_output
    }

    pub fn writes_schema_files(&self) -> bool {
        self.boundary().writes_schema_files
    }

    pub fn creates_acceptance_claim(&self) -> bool {
        self.boundary().creates_acceptance_claim
    }

    pub fn uses_indexes_or_latest_as_authority(&self) -> bool {
        self.boundary().uses_indexes_or_latest_as_authority
    }

    pub fn uses_service_mirror_as_authority(&self) -> bool {
        self.boundary().uses_service_mirror_as_authority
    }

    pub fn executor_claims_are_proof(&self) -> bool {
        self.boundary().executor_claims_are_proof
    }

    pub fn can_claim_acceptance_by_itself(&self) -> bool {
        false
    }
}

fn proofpack_writer_target_artifact_ref_policy_boundary_notes(
    boundary_notes: Vec<ProofBoundaryNote>,
) -> Vec<ProofBoundaryNote> {
    if boundary_notes.is_empty() {
        return vec![ProofBoundaryNote::new(
            "Writer target artifact ref policy is side-effect-free; it requires proofpack id plus manifest self-digest and renders only a logical non-path ref.",
        )
        .expect("fallback boundary note should be valid")];
    }

    boundary_notes
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProofpackWriterTargetArtifactRefPolicyModelBoundary {
    pub models_target_artifact_ref_policy: bool,
    pub requires_proofpack_id_and_manifest_self_digest: bool,
    pub renders_logical_display_ref: bool,
    pub display_ref_is_filesystem_path: bool,
    pub keeps_target_artifact_ref_separate_from_canonical_bytes: bool,
    pub keeps_target_artifact_ref_separate_from_target_path: bool,
    pub keeps_target_artifact_ref_separate_from_storage_root: bool,
    pub reads_filesystem: bool,
    pub touches_filesystem: bool,
    pub canonicalizes_host_paths: bool,
    pub writes_proofpack: bool,
    pub writes_writer_operation_evidence: bool,
    pub writes_indexes_or_latest: bool,
    pub writes_final_decision: bool,
    pub creates_acceptance_claim: bool,
    pub requires_runtime_storage: bool,
    pub writes_cli_output: bool,
    pub writes_schema_files: bool,
    pub verifies_referenced_artifacts: bool,
    pub uses_indexes_or_latest_as_authority: bool,
    pub uses_service_mirror_as_authority: bool,
    pub executor_claims_are_proof: bool,
    pub evidence_only: bool,
}

pub const fn proofpack_writer_target_artifact_ref_policy_model_boundary(
) -> ProofpackWriterTargetArtifactRefPolicyModelBoundary {
    ProofpackWriterTargetArtifactRefPolicyModelBoundary {
        models_target_artifact_ref_policy: true,
        requires_proofpack_id_and_manifest_self_digest: true,
        renders_logical_display_ref: true,
        display_ref_is_filesystem_path: false,
        keeps_target_artifact_ref_separate_from_canonical_bytes: true,
        keeps_target_artifact_ref_separate_from_target_path: true,
        keeps_target_artifact_ref_separate_from_storage_root: true,
        reads_filesystem: false,
        touches_filesystem: false,
        canonicalizes_host_paths: false,
        writes_proofpack: false,
        writes_writer_operation_evidence: false,
        writes_indexes_or_latest: false,
        writes_final_decision: false,
        creates_acceptance_claim: false,
        requires_runtime_storage: false,
        writes_cli_output: false,
        writes_schema_files: false,
        verifies_referenced_artifacts: false,
        uses_indexes_or_latest_as_authority: false,
        uses_service_mirror_as_authority: false,
        executor_claims_are_proof: false,
        evidence_only: true,
    }
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

    pub fn from_target_artifact_ref(
        target_artifact_ref: &ProofpackWriterTargetArtifactRef,
    ) -> Self {
        Self(target_artifact_ref.as_str().to_string())
    }

    pub fn from_target_artifact_ref_policy_model(
        model: &ProofpackWriterTargetArtifactRefPolicyModel,
    ) -> Option<Self> {
        model.logical_ref().map(Self::from_target_artifact_ref)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_logical_proofpack_ref(&self) -> bool {
        self.0.starts_with("proofpack:") && self.0.contains("@sha256:")
    }

    pub fn is_path_like_ref(&self) -> bool {
        self.0.contains('/')
            || self.0.contains('\\')
            || self.0.starts_with('~')
            || self.0.contains("://")
    }

    pub fn is_aligned_target_artifact_ref(&self) -> bool {
        self.is_logical_proofpack_ref() && !self.is_path_like_ref()
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofpackWriterDiagnosticPathRef(String);

impl ProofpackWriterDiagnosticPathRef {
    pub fn new(value: impl Into<String>) -> Result<Self, ProofpackError> {
        Ok(Self(non_empty(
            value,
            ProofpackError::EmptyWriterDiagnosticPathRef,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterObservedTargetState {
    NotChecked,
    Missing,
    ExistsMatching,
    ExistsDifferent,
    AmbiguousPartial,
}

impl ProofpackWriterObservedTargetState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotChecked => "not_checked",
            Self::Missing => "missing",
            Self::ExistsMatching => "exists_matching",
            Self::ExistsDifferent => "exists_different",
            Self::AmbiguousPartial => "ambiguous_partial",
        }
    }

    pub fn is_matching(self) -> bool {
        self == Self::ExistsMatching
    }

    pub fn is_conflict(self) -> bool {
        self == Self::ExistsDifferent
    }

    pub fn is_partial_or_ambiguous(self) -> bool {
        self == Self::AmbiguousPartial
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterIdempotencyObservation {
    NotChecked,
    NotApplicable,
    Matching,
    Different,
}

impl ProofpackWriterIdempotencyObservation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotChecked => "not_checked",
            Self::NotApplicable => "not_applicable",
            Self::Matching => "matching",
            Self::Different => "different",
        }
    }

    pub fn is_matching(self) -> bool {
        self == Self::Matching
    }

    pub fn is_conflict(self) -> bool {
        self == Self::Different
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterObservedWriteResult {
    NotAttempted,
    Written,
    WriteFailed,
    PartialWriteDetected,
}

impl ProofpackWriterObservedWriteResult {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotAttempted => "not_attempted",
            Self::Written => "written",
            Self::WriteFailed => "write_failed",
            Self::PartialWriteDetected => "partial_write_detected",
        }
    }

    pub fn is_written(self) -> bool {
        self == Self::Written
    }

    pub fn is_failed(self) -> bool {
        self == Self::WriteFailed
    }

    pub fn is_partial(self) -> bool {
        self == Self::PartialWriteDetected
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterObservedPartialState {
    NotObserved,
    AmbiguousCanonicalArtifact,
    CleanupIncomplete,
}

impl ProofpackWriterObservedPartialState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotObserved => "not_observed",
            Self::AmbiguousCanonicalArtifact => "ambiguous_canonical_artifact",
            Self::CleanupIncomplete => "cleanup_incomplete",
        }
    }

    pub fn is_partial_or_ambiguous(self) -> bool {
        self != Self::NotObserved
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterAbortState {
    NotAborted,
    AbortedBeforeWrite,
    AbortedAfterPartial,
}

impl ProofpackWriterAbortState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotAborted => "not_aborted",
            Self::AbortedBeforeWrite => "aborted_before_write",
            Self::AbortedAfterPartial => "aborted_after_partial",
        }
    }

    pub fn is_aborted(self) -> bool {
        self != Self::NotAborted
    }

    pub fn has_partial_visibility(self) -> bool {
        self == Self::AbortedAfterPartial
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofpackWriterFileIoObservation {
    target_state: ProofpackWriterObservedTargetState,
    idempotency_observation: ProofpackWriterIdempotencyObservation,
    temp_atomic_status: ProofpackWriterSideEffectStatus,
    write_result: ProofpackWriterObservedWriteResult,
    partial_state: ProofpackWriterObservedPartialState,
    cleanup_status: ProofpackWriterSideEffectStatus,
    index_status: ProofpackWriterSideEffectStatus,
    latest_pointer_status: ProofpackWriterSideEffectStatus,
    abort_state: ProofpackWriterAbortState,
    boundary_notes: Vec<ProofBoundaryNote>,
}

impl ProofpackWriterFileIoObservation {
    pub fn new(
        target_state: ProofpackWriterObservedTargetState,
        idempotency_observation: ProofpackWriterIdempotencyObservation,
        temp_atomic_status: ProofpackWriterSideEffectStatus,
        write_result: ProofpackWriterObservedWriteResult,
        partial_state: ProofpackWriterObservedPartialState,
        cleanup_status: ProofpackWriterSideEffectStatus,
        index_status: ProofpackWriterSideEffectStatus,
        latest_pointer_status: ProofpackWriterSideEffectStatus,
        abort_state: ProofpackWriterAbortState,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> Self {
        Self {
            target_state,
            idempotency_observation,
            temp_atomic_status,
            write_result,
            partial_state,
            cleanup_status,
            index_status,
            latest_pointer_status,
            abort_state,
            boundary_notes,
        }
    }

    pub fn planned_only() -> Self {
        Self::new(
            ProofpackWriterObservedTargetState::NotChecked,
            ProofpackWriterIdempotencyObservation::NotChecked,
            ProofpackWriterSideEffectStatus::NotAttempted,
            ProofpackWriterObservedWriteResult::NotAttempted,
            ProofpackWriterObservedPartialState::NotObserved,
            ProofpackWriterSideEffectStatus::NotAttempted,
            ProofpackWriterSideEffectStatus::NotAttempted,
            ProofpackWriterSideEffectStatus::NotAttempted,
            ProofpackWriterAbortState::NotAborted,
            vec![ProofBoundaryNote::new(
                "Writer file IO outcome model is observation-only and does not touch the filesystem.",
            )
            .expect("fallback boundary note should be valid")],
        )
    }

    pub fn target_missing_write_completed(
        index_status: ProofpackWriterSideEffectStatus,
        latest_pointer_status: ProofpackWriterSideEffectStatus,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> Self {
        Self::new(
            ProofpackWriterObservedTargetState::Missing,
            ProofpackWriterIdempotencyObservation::NotApplicable,
            ProofpackWriterSideEffectStatus::Completed,
            ProofpackWriterObservedWriteResult::Written,
            ProofpackWriterObservedPartialState::NotObserved,
            ProofpackWriterSideEffectStatus::Completed,
            index_status,
            latest_pointer_status,
            ProofpackWriterAbortState::NotAborted,
            boundary_notes,
        )
    }

    pub fn target_exists_matching(boundary_notes: Vec<ProofBoundaryNote>) -> Self {
        Self::new(
            ProofpackWriterObservedTargetState::ExistsMatching,
            ProofpackWriterIdempotencyObservation::Matching,
            ProofpackWriterSideEffectStatus::NotAttempted,
            ProofpackWriterObservedWriteResult::NotAttempted,
            ProofpackWriterObservedPartialState::NotObserved,
            ProofpackWriterSideEffectStatus::NotAttempted,
            ProofpackWriterSideEffectStatus::Skipped,
            ProofpackWriterSideEffectStatus::Skipped,
            ProofpackWriterAbortState::NotAborted,
            boundary_notes,
        )
    }

    pub fn target_exists_different(boundary_notes: Vec<ProofBoundaryNote>) -> Self {
        Self::new(
            ProofpackWriterObservedTargetState::ExistsDifferent,
            ProofpackWriterIdempotencyObservation::Different,
            ProofpackWriterSideEffectStatus::NotAttempted,
            ProofpackWriterObservedWriteResult::NotAttempted,
            ProofpackWriterObservedPartialState::NotObserved,
            ProofpackWriterSideEffectStatus::NotAttempted,
            ProofpackWriterSideEffectStatus::Skipped,
            ProofpackWriterSideEffectStatus::Skipped,
            ProofpackWriterAbortState::NotAborted,
            boundary_notes,
        )
    }

    pub fn write_failed(boundary_notes: Vec<ProofBoundaryNote>) -> Self {
        Self::new(
            ProofpackWriterObservedTargetState::Missing,
            ProofpackWriterIdempotencyObservation::NotApplicable,
            ProofpackWriterSideEffectStatus::Failed,
            ProofpackWriterObservedWriteResult::WriteFailed,
            ProofpackWriterObservedPartialState::NotObserved,
            ProofpackWriterSideEffectStatus::NotAttempted,
            ProofpackWriterSideEffectStatus::NotAttempted,
            ProofpackWriterSideEffectStatus::NotAttempted,
            ProofpackWriterAbortState::NotAborted,
            boundary_notes,
        )
    }

    pub fn partial_write_detected(
        cleanup_status: ProofpackWriterSideEffectStatus,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> Self {
        Self::new(
            ProofpackWriterObservedTargetState::AmbiguousPartial,
            ProofpackWriterIdempotencyObservation::NotChecked,
            ProofpackWriterSideEffectStatus::Failed,
            ProofpackWriterObservedWriteResult::PartialWriteDetected,
            ProofpackWriterObservedPartialState::AmbiguousCanonicalArtifact,
            cleanup_status,
            ProofpackWriterSideEffectStatus::NotAttempted,
            ProofpackWriterSideEffectStatus::NotAttempted,
            ProofpackWriterAbortState::NotAborted,
            boundary_notes,
        )
    }

    pub fn index_failed_after_available(boundary_notes: Vec<ProofBoundaryNote>) -> Self {
        Self::target_missing_write_completed(
            ProofpackWriterSideEffectStatus::Failed,
            ProofpackWriterSideEffectStatus::Completed,
            boundary_notes,
        )
    }

    pub fn latest_failed_after_available(boundary_notes: Vec<ProofBoundaryNote>) -> Self {
        Self::target_missing_write_completed(
            ProofpackWriterSideEffectStatus::Completed,
            ProofpackWriterSideEffectStatus::Failed,
            boundary_notes,
        )
    }

    pub fn aborted(boundary_notes: Vec<ProofBoundaryNote>) -> Self {
        Self::new(
            ProofpackWriterObservedTargetState::NotChecked,
            ProofpackWriterIdempotencyObservation::NotChecked,
            ProofpackWriterSideEffectStatus::NotAttempted,
            ProofpackWriterObservedWriteResult::NotAttempted,
            ProofpackWriterObservedPartialState::NotObserved,
            ProofpackWriterSideEffectStatus::NotAttempted,
            ProofpackWriterSideEffectStatus::NotAttempted,
            ProofpackWriterSideEffectStatus::NotAttempted,
            ProofpackWriterAbortState::AbortedBeforeWrite,
            boundary_notes,
        )
    }

    pub fn target_state(&self) -> ProofpackWriterObservedTargetState {
        self.target_state
    }

    pub fn idempotency_observation(&self) -> ProofpackWriterIdempotencyObservation {
        self.idempotency_observation
    }

    pub fn temp_atomic_status(&self) -> ProofpackWriterSideEffectStatus {
        self.temp_atomic_status
    }

    pub fn write_result(&self) -> ProofpackWriterObservedWriteResult {
        self.write_result
    }

    pub fn partial_state(&self) -> ProofpackWriterObservedPartialState {
        self.partial_state
    }

    pub fn cleanup_status(&self) -> ProofpackWriterSideEffectStatus {
        self.cleanup_status
    }

    pub fn index_status(&self) -> ProofpackWriterSideEffectStatus {
        self.index_status
    }

    pub fn latest_pointer_status(&self) -> ProofpackWriterSideEffectStatus {
        self.latest_pointer_status
    }

    pub fn abort_state(&self) -> ProofpackWriterAbortState {
        self.abort_state
    }

    pub fn boundary_notes(&self) -> &[ProofBoundaryNote] {
        &self.boundary_notes
    }

    pub fn has_idempotent_match(&self) -> bool {
        self.target_state.is_matching() || self.idempotency_observation.is_matching()
    }

    pub fn has_conflict(&self) -> bool {
        self.target_state.is_conflict() || self.idempotency_observation.is_conflict()
    }

    pub fn has_cleanup_failure(&self) -> bool {
        self.cleanup_status.is_failed()
            || self.partial_state == ProofpackWriterObservedPartialState::CleanupIncomplete
    }

    pub fn has_partial_or_ambiguous_state(&self) -> bool {
        self.target_state.is_partial_or_ambiguous()
            || self.write_result.is_partial()
            || self.partial_state.is_partial_or_ambiguous()
            || self.has_cleanup_failure()
            || self.abort_state.has_partial_visibility()
    }

    pub fn is_aborted(&self) -> bool {
        self.abort_state.is_aborted()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofpackWriterFileIoOutcomeModel {
    proofpack_id: ProofpackId,
    schema_version: &'static str,
    target_artifact_ref: ProofpackWriterTargetRef,
    target_path_ref: ProofpackWriterTargetPathRef,
    operation_kind: ProofpackWriterOperationKind,
    outcome: ProofpackWriterOperationOutcome,
    canonical_artifact_status: ProofpackWriterCanonicalArtifactStatus,
    index_status: ProofpackWriterSideEffectStatus,
    latest_pointer_status: ProofpackWriterSideEffectStatus,
    cleanup_status: ProofpackWriterSideEffectStatus,
    observation: ProofpackWriterFileIoObservation,
    boundary_notes: Vec<ProofBoundaryNote>,
}

impl ProofpackWriterFileIoOutcomeModel {
    pub fn from_plan_and_observation(
        plan: &ProofpackWriterFileIoPlan,
        observation: ProofpackWriterFileIoObservation,
    ) -> Self {
        let outcome = writer_file_io_outcome(plan, &observation);
        let canonical_artifact_status =
            writer_file_io_canonical_artifact_status(outcome, &observation);
        let operation_kind = writer_file_io_operation_kind(outcome);
        let index_status = writer_file_io_side_effect_status(
            plan,
            ProofpackWriterPlannedSideEffect::IndexUpdate,
            observation.index_status(),
        );
        let latest_pointer_status = writer_file_io_side_effect_status(
            plan,
            ProofpackWriterPlannedSideEffect::LatestPointerUpdate,
            observation.latest_pointer_status(),
        );
        let cleanup_status = observation.cleanup_status();
        let boundary_notes = writer_file_io_outcome_boundary_notes(plan, &observation);

        Self {
            proofpack_id: plan.proofpack_id().clone(),
            schema_version: PROOFPACK_WRITER_FILE_IO_OUTCOME_MODEL_SCHEMA_VERSION,
            target_artifact_ref: plan.target_artifact_ref().clone(),
            target_path_ref: plan.target_path_ref().clone(),
            operation_kind,
            outcome,
            canonical_artifact_status,
            index_status,
            latest_pointer_status,
            cleanup_status,
            observation,
            boundary_notes,
        }
    }

    pub fn proofpack_id(&self) -> &ProofpackId {
        &self.proofpack_id
    }

    pub fn schema_version(&self) -> &str {
        self.schema_version
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

    pub fn operation_kind(&self) -> ProofpackWriterOperationKind {
        self.operation_kind
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

    pub fn cleanup_status(&self) -> ProofpackWriterSideEffectStatus {
        self.cleanup_status
    }

    pub fn observation(&self) -> &ProofpackWriterFileIoObservation {
        &self.observation
    }

    pub fn boundary_notes(&self) -> &[ProofBoundaryNote] {
        &self.boundary_notes
    }

    pub fn to_operation_evidence(
        &self,
        operation_id: ProofpackWriterOperationId,
        attempted_at: ProofpackWriterAttemptedAt,
    ) -> Result<ProofpackWriterOperationEvidence, ProofpackError> {
        ProofpackWriterOperationEvidence::new(
            operation_id,
            self.operation_kind,
            self.proofpack_id.clone(),
            attempted_at,
            self.target_artifact_ref.clone(),
            self.outcome,
            self.canonical_artifact_status,
            self.index_status,
            self.latest_pointer_status,
            self.boundary_notes.clone(),
        )
    }

    pub fn boundary(&self) -> ProofpackWriterFileIoOutcomeModelBoundary {
        proofpack_writer_file_io_outcome_model_boundary()
    }

    pub fn canonical_artifact_available(&self) -> bool {
        self.canonical_artifact_status.is_available()
    }

    pub fn has_conflict(&self) -> bool {
        self.canonical_artifact_status.is_conflict()
    }

    pub fn has_partial_or_cleanup_issue(&self) -> bool {
        self.canonical_artifact_status.is_partial() || self.cleanup_status.is_failed()
    }

    pub fn has_index_or_latest_pointer_failure(&self) -> bool {
        self.index_status.is_failed() || self.latest_pointer_status.is_failed()
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

    pub fn writes_writer_operation_evidence(&self) -> bool {
        self.boundary().writes_writer_operation_evidence
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

    pub fn writes_schema_files(&self) -> bool {
        self.boundary().writes_schema_files
    }

    pub fn target_path_is_authority(&self) -> bool {
        self.boundary().target_path_is_authority
    }

    pub fn index_latest_are_canonical(&self) -> bool {
        self.boundary().index_latest_are_canonical
    }
}

fn writer_file_io_outcome(
    plan: &ProofpackWriterFileIoPlan,
    observation: &ProofpackWriterFileIoObservation,
) -> ProofpackWriterOperationOutcome {
    if !plan.is_file_io_ready() {
        return ProofpackWriterOperationOutcome::PreflightFailed;
    }

    if observation.has_partial_or_ambiguous_state() {
        return ProofpackWriterOperationOutcome::PartialWriteDetected;
    }

    if observation.abort_state() == ProofpackWriterAbortState::AbortedBeforeWrite {
        return ProofpackWriterOperationOutcome::Aborted;
    }

    if observation.has_conflict() {
        return ProofpackWriterOperationOutcome::ConflictExistingDifferent;
    }

    if observation.has_idempotent_match() {
        return ProofpackWriterOperationOutcome::AlreadyExistsMatching;
    }

    if observation.write_result().is_failed() {
        return ProofpackWriterOperationOutcome::WriteFailed;
    }

    if observation.write_result().is_written()
        && observation.index_status().is_failed()
        && plan.selects_index_update()
    {
        return ProofpackWriterOperationOutcome::IndexUpdateFailed;
    }

    if observation.write_result().is_written()
        && observation.latest_pointer_status().is_failed()
        && plan.selects_latest_pointer_update()
    {
        return ProofpackWriterOperationOutcome::LatestPointerUpdateFailed;
    }

    if observation.write_result().is_written() {
        return ProofpackWriterOperationOutcome::Written;
    }

    ProofpackWriterOperationOutcome::PlannedOnly
}

fn writer_file_io_canonical_artifact_status(
    outcome: ProofpackWriterOperationOutcome,
    observation: &ProofpackWriterFileIoObservation,
) -> ProofpackWriterCanonicalArtifactStatus {
    match outcome {
        ProofpackWriterOperationOutcome::PlannedOnly
        | ProofpackWriterOperationOutcome::PreflightFailed
        | ProofpackWriterOperationOutcome::Aborted => {
            ProofpackWriterCanonicalArtifactStatus::NotAttempted
        }
        ProofpackWriterOperationOutcome::Written
        | ProofpackWriterOperationOutcome::IndexUpdateFailed
        | ProofpackWriterOperationOutcome::LatestPointerUpdateFailed => {
            if observation.has_idempotent_match() {
                ProofpackWriterCanonicalArtifactStatus::AlreadyExistsMatching
            } else {
                ProofpackWriterCanonicalArtifactStatus::Written
            }
        }
        ProofpackWriterOperationOutcome::AlreadyExistsMatching => {
            ProofpackWriterCanonicalArtifactStatus::AlreadyExistsMatching
        }
        ProofpackWriterOperationOutcome::ConflictExistingDifferent => {
            ProofpackWriterCanonicalArtifactStatus::ConflictExistingDifferent
        }
        ProofpackWriterOperationOutcome::WriteFailed => {
            ProofpackWriterCanonicalArtifactStatus::WriteFailed
        }
        ProofpackWriterOperationOutcome::PartialWriteDetected => {
            ProofpackWriterCanonicalArtifactStatus::PartialWriteDetected
        }
    }
}

fn writer_file_io_operation_kind(
    outcome: ProofpackWriterOperationOutcome,
) -> ProofpackWriterOperationKind {
    match outcome {
        ProofpackWriterOperationOutcome::PlannedOnly
        | ProofpackWriterOperationOutcome::PreflightFailed => {
            ProofpackWriterOperationKind::PlannedOnly
        }
        ProofpackWriterOperationOutcome::Written
        | ProofpackWriterOperationOutcome::WriteFailed
        | ProofpackWriterOperationOutcome::PartialWriteDetected => {
            ProofpackWriterOperationKind::Write
        }
        ProofpackWriterOperationOutcome::AlreadyExistsMatching => {
            ProofpackWriterOperationKind::IdempotencyCheck
        }
        ProofpackWriterOperationOutcome::ConflictExistingDifferent => {
            ProofpackWriterOperationKind::ConflictCheck
        }
        ProofpackWriterOperationOutcome::IndexUpdateFailed => {
            ProofpackWriterOperationKind::IndexUpdate
        }
        ProofpackWriterOperationOutcome::LatestPointerUpdateFailed => {
            ProofpackWriterOperationKind::LatestPointerUpdate
        }
        ProofpackWriterOperationOutcome::Aborted => ProofpackWriterOperationKind::Abort,
    }
}

fn writer_file_io_side_effect_status(
    plan: &ProofpackWriterFileIoPlan,
    side_effect: ProofpackWriterPlannedSideEffect,
    observed_status: ProofpackWriterSideEffectStatus,
) -> ProofpackWriterSideEffectStatus {
    if !plan.plans_side_effect(side_effect) {
        return ProofpackWriterSideEffectStatus::NotSelected;
    }

    if !plan.is_file_io_ready() {
        return ProofpackWriterSideEffectStatus::NotAttempted;
    }

    if observed_status == ProofpackWriterSideEffectStatus::NotSelected {
        ProofpackWriterSideEffectStatus::NotAttempted
    } else {
        observed_status
    }
}

fn writer_file_io_outcome_boundary_notes(
    plan: &ProofpackWriterFileIoPlan,
    observation: &ProofpackWriterFileIoObservation,
) -> Vec<ProofBoundaryNote> {
    let mut boundary_notes = plan.boundary_notes().to_vec();
    boundary_notes.extend(observation.boundary_notes().iter().cloned());

    if boundary_notes.is_empty() {
        return vec![ProofBoundaryNote::new(
            "Writer file IO outcome model is evidence-only; missing notes are explicit observation data.",
        )
        .expect("fallback boundary note should be valid")];
    }

    boundary_notes
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProofpackWriterFileIoOutcomeModelBoundary {
    pub models_writer_file_io_outcome: bool,
    pub accepts_explicit_observations: bool,
    pub maps_observations_to_operation_evidence: bool,
    pub reads_filesystem: bool,
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
    pub separates_observation_from_artifact_availability: bool,
    pub preserves_partial_cleanup_visibility: bool,
}

pub const fn proofpack_writer_file_io_outcome_model_boundary(
) -> ProofpackWriterFileIoOutcomeModelBoundary {
    ProofpackWriterFileIoOutcomeModelBoundary {
        models_writer_file_io_outcome: true,
        accepts_explicit_observations: true,
        maps_observations_to_operation_evidence: true,
        reads_filesystem: false,
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
        separates_observation_from_artifact_availability: true,
        preserves_partial_cleanup_visibility: true,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterFileIoDiagnosticSurface {
    StorageRoot,
    TargetPath,
    ExistingTarget,
    TempWrite,
    Durability,
    AtomicMove,
    Cleanup,
    PartialCanonicalArtifact,
    Index,
    LatestPointer,
    OperationEvidencePersistence,
    Abort,
}

impl ProofpackWriterFileIoDiagnosticSurface {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::StorageRoot => "storage_root",
            Self::TargetPath => "target_path",
            Self::ExistingTarget => "existing_target",
            Self::TempWrite => "temp_write",
            Self::Durability => "durability",
            Self::AtomicMove => "atomic_move",
            Self::Cleanup => "cleanup",
            Self::PartialCanonicalArtifact => "partial_canonical_artifact",
            Self::Index => "index",
            Self::LatestPointer => "latest_pointer",
            Self::OperationEvidencePersistence => "operation_evidence_persistence",
            Self::Abort => "abort",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterFileIoErrorReason {
    StorageRootMissing,
    StorageRootDisallowed,
    TargetPathInvalid,
    TargetPathEscapesStorageRoot,
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
    OperationEvidencePersistenceFailed,
    OperationAborted,
}

impl ProofpackWriterFileIoErrorReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::StorageRootMissing => "storage_root_missing",
            Self::StorageRootDisallowed => "storage_root_disallowed",
            Self::TargetPathInvalid => "target_path_invalid",
            Self::TargetPathEscapesStorageRoot => "target_path_escapes_storage_root",
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
            Self::OperationEvidencePersistenceFailed => "operation_evidence_persistence_failed",
            Self::OperationAborted => "operation_aborted",
        }
    }

    pub fn surface(self) -> ProofpackWriterFileIoDiagnosticSurface {
        match self {
            Self::StorageRootMissing | Self::StorageRootDisallowed => {
                ProofpackWriterFileIoDiagnosticSurface::StorageRoot
            }
            Self::TargetPathInvalid
            | Self::TargetPathEscapesStorageRoot
            | Self::ParentDirectoryMissing => ProofpackWriterFileIoDiagnosticSurface::TargetPath,
            Self::ExistingTargetMatching | Self::ExistingTargetDifferent => {
                ProofpackWriterFileIoDiagnosticSurface::ExistingTarget
            }
            Self::TempWriteDenied | Self::TempWriteFailed => {
                ProofpackWriterFileIoDiagnosticSurface::TempWrite
            }
            Self::FlushOrSyncFailed => ProofpackWriterFileIoDiagnosticSurface::Durability,
            Self::AtomicMoveUnsupported | Self::AtomicMoveFailed => {
                ProofpackWriterFileIoDiagnosticSurface::AtomicMove
            }
            Self::CleanupFailed => ProofpackWriterFileIoDiagnosticSurface::Cleanup,
            Self::PartialCanonicalArtifactAmbiguous => {
                ProofpackWriterFileIoDiagnosticSurface::PartialCanonicalArtifact
            }
            Self::IndexUpdateFailed => ProofpackWriterFileIoDiagnosticSurface::Index,
            Self::LatestPointerUpdateFailed => {
                ProofpackWriterFileIoDiagnosticSurface::LatestPointer
            }
            Self::OperationEvidencePersistenceFailed => {
                ProofpackWriterFileIoDiagnosticSurface::OperationEvidencePersistence
            }
            Self::OperationAborted => ProofpackWriterFileIoDiagnosticSurface::Abort,
        }
    }

    pub fn is_storage_or_target_reason(self) -> bool {
        matches!(
            self.surface(),
            ProofpackWriterFileIoDiagnosticSurface::StorageRoot
                | ProofpackWriterFileIoDiagnosticSurface::TargetPath
        )
    }

    pub fn is_write_reason(self) -> bool {
        matches!(
            self,
            Self::TempWriteDenied
                | Self::TempWriteFailed
                | Self::FlushOrSyncFailed
                | Self::AtomicMoveUnsupported
                | Self::AtomicMoveFailed
        )
    }

    pub fn is_index_or_latest_reason(self) -> bool {
        matches!(
            self,
            Self::IndexUpdateFailed | Self::LatestPointerUpdateFailed
        )
    }

    pub fn is_partial_or_cleanup_reason(self) -> bool {
        matches!(
            self,
            Self::CleanupFailed | Self::PartialCanonicalArtifactAmbiguous
        )
    }

    pub fn is_operation_evidence_persistence_reason(self) -> bool {
        self == Self::OperationEvidencePersistenceFailed
    }

    pub fn is_abort_reason(self) -> bool {
        self == Self::OperationAborted
    }

    pub fn is_conflict_reason(self) -> bool {
        self == Self::ExistingTargetDifferent
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterFileIoDiagnosticSource {
    CallerObservation,
    WriterDiagnostic,
    ExecutorClaim,
}

impl ProofpackWriterFileIoDiagnosticSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CallerObservation => "caller_observation",
            Self::WriterDiagnostic => "writer_diagnostic",
            Self::ExecutorClaim => "executor_claim",
        }
    }

    pub fn is_proof_authority(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofpackWriterFileIoDiagnostic {
    reason: ProofpackWriterFileIoErrorReason,
    source: ProofpackWriterFileIoDiagnosticSource,
    target_path_ref: Option<ProofpackWriterTargetPathRef>,
    diagnostic_path_ref: Option<ProofpackWriterDiagnosticPathRef>,
    boundary_notes: Vec<ProofBoundaryNote>,
}

impl ProofpackWriterFileIoDiagnostic {
    pub fn new(
        reason: ProofpackWriterFileIoErrorReason,
        source: ProofpackWriterFileIoDiagnosticSource,
        target_path_ref: Option<ProofpackWriterTargetPathRef>,
        diagnostic_path_ref: Option<ProofpackWriterDiagnosticPathRef>,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> Self {
        Self {
            reason,
            source,
            target_path_ref,
            diagnostic_path_ref,
            boundary_notes,
        }
    }

    pub fn for_reason(reason: ProofpackWriterFileIoErrorReason) -> Self {
        Self::new(
            reason,
            ProofpackWriterFileIoDiagnosticSource::CallerObservation,
            None,
            None,
            vec![ProofBoundaryNote::new(
                "Writer file IO diagnostic is evidence-only and does not touch the filesystem.",
            )
            .expect("fallback boundary note should be valid")],
        )
    }

    pub fn with_target_path_ref(mut self, target_path_ref: ProofpackWriterTargetPathRef) -> Self {
        self.target_path_ref = Some(target_path_ref);
        self
    }

    pub fn with_diagnostic_path_ref(
        mut self,
        diagnostic_path_ref: ProofpackWriterDiagnosticPathRef,
    ) -> Self {
        self.diagnostic_path_ref = Some(diagnostic_path_ref);
        self
    }

    pub fn with_source(mut self, source: ProofpackWriterFileIoDiagnosticSource) -> Self {
        self.source = source;
        self
    }

    pub fn with_boundary_notes(mut self, boundary_notes: Vec<ProofBoundaryNote>) -> Self {
        self.boundary_notes = boundary_notes;
        self
    }

    pub fn reason(&self) -> ProofpackWriterFileIoErrorReason {
        self.reason
    }

    pub fn reason_code(&self) -> &'static str {
        self.reason.as_str()
    }

    pub fn surface(&self) -> ProofpackWriterFileIoDiagnosticSurface {
        self.reason.surface()
    }

    pub fn source(&self) -> ProofpackWriterFileIoDiagnosticSource {
        self.source
    }

    pub fn target_path_ref(&self) -> Option<&ProofpackWriterTargetPathRef> {
        self.target_path_ref.as_ref()
    }

    pub fn diagnostic_path_ref(&self) -> Option<&ProofpackWriterDiagnosticPathRef> {
        self.diagnostic_path_ref.as_ref()
    }

    pub fn boundary_notes(&self) -> &[ProofBoundaryNote] {
        &self.boundary_notes
    }

    pub fn has_diagnostic_path_ref(&self) -> bool {
        self.diagnostic_path_ref.is_some()
    }

    pub fn diagnostic_path_is_authority(&self) -> bool {
        false
    }

    pub fn target_path_is_authority(&self) -> bool {
        false
    }

    pub fn source_is_proof_authority(&self) -> bool {
        self.source.is_proof_authority()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofpackWriterFileIoErrorReasonModel {
    proofpack_id: ProofpackId,
    schema_version: &'static str,
    target_artifact_ref: ProofpackWriterTargetRef,
    target_path_ref: ProofpackWriterTargetPathRef,
    outcome: ProofpackWriterOperationOutcome,
    canonical_artifact_status: ProofpackWriterCanonicalArtifactStatus,
    diagnostics: Vec<ProofpackWriterFileIoDiagnostic>,
    boundary_notes: Vec<ProofBoundaryNote>,
}

impl ProofpackWriterFileIoErrorReasonModel {
    pub fn from_outcome_and_diagnostics(
        outcome: &ProofpackWriterFileIoOutcomeModel,
        diagnostics: Vec<ProofpackWriterFileIoDiagnostic>,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> Self {
        Self {
            proofpack_id: outcome.proofpack_id().clone(),
            schema_version: PROOFPACK_WRITER_FILE_IO_ERROR_REASON_MODEL_SCHEMA_VERSION,
            target_artifact_ref: outcome.target_artifact_ref().clone(),
            target_path_ref: outcome.target_path_ref().clone(),
            outcome: outcome.outcome(),
            canonical_artifact_status: outcome.canonical_artifact_status(),
            diagnostics,
            boundary_notes: writer_file_io_error_reason_boundary_notes(outcome, boundary_notes),
        }
    }

    pub fn proofpack_id(&self) -> &ProofpackId {
        &self.proofpack_id
    }

    pub fn schema_version(&self) -> &str {
        self.schema_version
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

    pub fn outcome(&self) -> ProofpackWriterOperationOutcome {
        self.outcome
    }

    pub fn canonical_artifact_status(&self) -> ProofpackWriterCanonicalArtifactStatus {
        self.canonical_artifact_status
    }

    pub fn diagnostics(&self) -> &[ProofpackWriterFileIoDiagnostic] {
        &self.diagnostics
    }

    pub fn boundary_notes(&self) -> &[ProofBoundaryNote] {
        &self.boundary_notes
    }

    pub fn reasons(&self) -> Vec<ProofpackWriterFileIoErrorReason> {
        self.diagnostics
            .iter()
            .map(ProofpackWriterFileIoDiagnostic::reason)
            .collect()
    }

    pub fn has_reason(&self, reason: ProofpackWriterFileIoErrorReason) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.reason() == reason)
    }

    pub fn has_write_reason(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.reason().is_write_reason())
    }

    pub fn has_index_or_latest_reason(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.reason().is_index_or_latest_reason())
    }

    pub fn has_partial_or_cleanup_reason(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.reason().is_partial_or_cleanup_reason())
    }

    pub fn has_operation_evidence_persistence_reason(&self) -> bool {
        self.diagnostics.iter().any(|diagnostic| {
            diagnostic
                .reason()
                .is_operation_evidence_persistence_reason()
        })
    }

    pub fn has_abort_reason(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.reason().is_abort_reason())
    }

    pub fn has_diagnostic_path_refs(&self) -> bool {
        self.diagnostics
            .iter()
            .any(ProofpackWriterFileIoDiagnostic::has_diagnostic_path_ref)
    }

    pub fn has_executor_claims(&self) -> bool {
        self.diagnostics.iter().any(|diagnostic| {
            diagnostic.source() == ProofpackWriterFileIoDiagnosticSource::ExecutorClaim
        })
    }

    pub fn boundary(&self) -> ProofpackWriterFileIoErrorReasonModelBoundary {
        proofpack_writer_file_io_error_reason_model_boundary()
    }

    pub fn is_evidence_only(&self) -> bool {
        self.boundary().evidence_only
    }

    pub fn touches_filesystem(&self) -> bool {
        self.boundary().touches_filesystem
    }

    pub fn reads_filesystem(&self) -> bool {
        self.boundary().reads_filesystem
    }

    pub fn writes_proofpack(&self) -> bool {
        self.boundary().writes_proofpack
    }

    pub fn writes_writer_operation_evidence(&self) -> bool {
        self.boundary().writes_writer_operation_evidence
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

    pub fn writes_schema_files(&self) -> bool {
        self.boundary().writes_schema_files
    }

    pub fn target_path_is_authority(&self) -> bool {
        self.boundary().target_path_is_authority
    }

    pub fn diagnostic_paths_are_authority(&self) -> bool {
        self.boundary().diagnostic_paths_are_authority
    }

    pub fn index_latest_are_canonical(&self) -> bool {
        self.boundary().index_latest_are_canonical
    }

    pub fn executor_claims_are_proof(&self) -> bool {
        self.boundary().executor_claims_are_proof
    }

    pub fn can_claim_acceptance_by_itself(&self) -> bool {
        false
    }
}

fn writer_file_io_error_reason_boundary_notes(
    outcome: &ProofpackWriterFileIoOutcomeModel,
    boundary_notes: Vec<ProofBoundaryNote>,
) -> Vec<ProofBoundaryNote> {
    if boundary_notes.is_empty() {
        return outcome.boundary_notes().to_vec();
    }

    boundary_notes
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProofpackWriterFileIoErrorReasonModelBoundary {
    pub models_writer_file_io_error_reasons: bool,
    pub stable_reason_codes: bool,
    pub attaches_to_file_io_outcomes: bool,
    pub reads_filesystem: bool,
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
    pub diagnostic_paths_are_authority: bool,
    pub index_latest_are_canonical: bool,
    pub executor_claims_are_proof: bool,
}

pub const fn proofpack_writer_file_io_error_reason_model_boundary(
) -> ProofpackWriterFileIoErrorReasonModelBoundary {
    ProofpackWriterFileIoErrorReasonModelBoundary {
        models_writer_file_io_error_reasons: true,
        stable_reason_codes: true,
        attaches_to_file_io_outcomes: true,
        reads_filesystem: false,
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
        diagnostic_paths_are_authority: false,
        index_latest_are_canonical: false,
        executor_claims_are_proof: false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterTargetPathPolicyStatus {
    Accepted,
    Rejected,
}

impl ProofpackWriterTargetPathPolicyStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterTargetPathPolicyReason {
    AbsolutePath,
    HomeRelativePath,
    UrlRef,
    PathTraversal,
    AmbiguousDotSegment,
    EmptySegment,
    UnsupportedBackslash,
    StorageRootEscape,
}

impl ProofpackWriterTargetPathPolicyReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AbsolutePath => "absolute_path",
            Self::HomeRelativePath => "home_relative_path",
            Self::UrlRef => "url_ref",
            Self::PathTraversal => "path_traversal",
            Self::AmbiguousDotSegment => "ambiguous_dot_segment",
            Self::EmptySegment => "empty_segment",
            Self::UnsupportedBackslash => "unsupported_backslash",
            Self::StorageRootEscape => "storage_root_escape",
        }
    }

    pub fn file_io_error_reason(self) -> ProofpackWriterFileIoErrorReason {
        match self {
            Self::StorageRootEscape => {
                ProofpackWriterFileIoErrorReason::TargetPathEscapesStorageRoot
            }
            Self::AbsolutePath
            | Self::HomeRelativePath
            | Self::UrlRef
            | Self::PathTraversal
            | Self::AmbiguousDotSegment
            | Self::EmptySegment
            | Self::UnsupportedBackslash => ProofpackWriterFileIoErrorReason::TargetPathInvalid,
        }
    }

    pub fn is_storage_root_escape(self) -> bool {
        self == Self::StorageRootEscape
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofpackWriterTargetPathPolicyModel {
    schema_version: &'static str,
    storage_root_ref: ProofpackWriterStorageRootRef,
    target_artifact_ref: ProofpackWriterTargetRef,
    target_path_ref: ProofpackWriterTargetPathRef,
    status: ProofpackWriterTargetPathPolicyStatus,
    reasons: Vec<ProofpackWriterTargetPathPolicyReason>,
    diagnostics: Vec<ProofpackWriterFileIoDiagnostic>,
    boundary_notes: Vec<ProofBoundaryNote>,
}

impl ProofpackWriterTargetPathPolicyModel {
    pub fn evaluate(
        storage_root_ref: ProofpackWriterStorageRootRef,
        target_artifact_ref: ProofpackWriterTargetRef,
        target_path_ref: ProofpackWriterTargetPathRef,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> Self {
        let reasons = writer_target_path_policy_reasons(target_path_ref.as_str());
        let diagnostics = writer_target_path_policy_diagnostics(&target_path_ref, &reasons);
        let status = if reasons.is_empty() {
            ProofpackWriterTargetPathPolicyStatus::Accepted
        } else {
            ProofpackWriterTargetPathPolicyStatus::Rejected
        };

        Self {
            schema_version: PROOFPACK_WRITER_TARGET_PATH_POLICY_MODEL_SCHEMA_VERSION,
            storage_root_ref,
            target_artifact_ref,
            target_path_ref,
            status,
            reasons,
            diagnostics,
            boundary_notes: writer_target_path_policy_boundary_notes(boundary_notes),
        }
    }

    pub fn from_plan(
        plan: &ProofpackWriterFileIoPlan,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> Self {
        Self::evaluate(
            plan.storage_root_ref().clone(),
            plan.target_artifact_ref().clone(),
            plan.target_path_ref().clone(),
            boundary_notes,
        )
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

    pub fn status(&self) -> ProofpackWriterTargetPathPolicyStatus {
        self.status
    }

    pub fn reasons(&self) -> &[ProofpackWriterTargetPathPolicyReason] {
        &self.reasons
    }

    pub fn diagnostics(&self) -> &[ProofpackWriterFileIoDiagnostic] {
        &self.diagnostics
    }

    pub fn boundary_notes(&self) -> &[ProofBoundaryNote] {
        &self.boundary_notes
    }

    pub fn is_accepted(&self) -> bool {
        self.status == ProofpackWriterTargetPathPolicyStatus::Accepted
    }

    pub fn is_rejected(&self) -> bool {
        self.status == ProofpackWriterTargetPathPolicyStatus::Rejected
    }

    pub fn has_reason(&self, reason: ProofpackWriterTargetPathPolicyReason) -> bool {
        self.reasons.contains(&reason)
    }

    pub fn has_storage_root_escape(&self) -> bool {
        self.reasons
            .iter()
            .any(|reason| reason.is_storage_root_escape())
    }

    pub fn diagnostic_reason_codes(&self) -> Vec<&'static str> {
        self.diagnostics
            .iter()
            .map(ProofpackWriterFileIoDiagnostic::reason_code)
            .collect()
    }

    pub fn boundary(&self) -> ProofpackWriterTargetPathPolicyModelBoundary {
        proofpack_writer_target_path_policy_model_boundary()
    }

    pub fn is_evidence_only(&self) -> bool {
        self.boundary().evidence_only
    }

    pub fn reads_filesystem(&self) -> bool {
        self.boundary().reads_filesystem
    }

    pub fn touches_filesystem(&self) -> bool {
        self.boundary().touches_filesystem
    }

    pub fn canonicalizes_host_paths(&self) -> bool {
        self.boundary().canonicalizes_host_paths
    }

    pub fn writes_proofpack(&self) -> bool {
        self.boundary().writes_proofpack
    }

    pub fn writes_writer_operation_evidence(&self) -> bool {
        self.boundary().writes_writer_operation_evidence
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

    pub fn writes_schema_files(&self) -> bool {
        self.boundary().writes_schema_files
    }

    pub fn target_path_is_authority(&self) -> bool {
        self.boundary().target_path_is_authority
    }

    pub fn storage_root_ref_is_authority(&self) -> bool {
        self.boundary().storage_root_ref_is_authority
    }

    pub fn derives_from_current_working_directory(&self) -> bool {
        self.boundary().derives_from_current_working_directory
    }

    pub fn uses_indexes_or_latest_as_authority(&self) -> bool {
        self.boundary().uses_indexes_or_latest_as_authority
    }
}

fn writer_target_path_policy_reasons(
    target_path: &str,
) -> Vec<ProofpackWriterTargetPathPolicyReason> {
    let mut reasons = Vec::new();

    let absolute_path = writer_target_path_is_absolute(target_path);
    let home_relative = target_path.starts_with('~');
    let url_ref = writer_target_path_is_url_ref(target_path);
    let unsupported_backslash = target_path.contains('\\');
    let mut path_traversal = false;
    let mut ambiguous_dot_segment = false;
    let mut empty_segment = false;

    for segment in target_path.split('/') {
        if segment.is_empty() {
            empty_segment = true;
        } else if segment == "." {
            ambiguous_dot_segment = true;
        } else if segment == ".." {
            path_traversal = true;
        }
    }

    if absolute_path {
        reasons.push(ProofpackWriterTargetPathPolicyReason::AbsolutePath);
    }
    if home_relative {
        reasons.push(ProofpackWriterTargetPathPolicyReason::HomeRelativePath);
    }
    if url_ref {
        reasons.push(ProofpackWriterTargetPathPolicyReason::UrlRef);
    }
    if path_traversal {
        reasons.push(ProofpackWriterTargetPathPolicyReason::PathTraversal);
    }
    if ambiguous_dot_segment {
        reasons.push(ProofpackWriterTargetPathPolicyReason::AmbiguousDotSegment);
    }
    if empty_segment {
        reasons.push(ProofpackWriterTargetPathPolicyReason::EmptySegment);
    }
    if unsupported_backslash {
        reasons.push(ProofpackWriterTargetPathPolicyReason::UnsupportedBackslash);
    }
    if absolute_path || home_relative || url_ref || path_traversal {
        reasons.push(ProofpackWriterTargetPathPolicyReason::StorageRootEscape);
    }

    reasons
}

fn writer_target_path_is_absolute(target_path: &str) -> bool {
    if target_path.starts_with('/') {
        return true;
    }

    let bytes = target_path.as_bytes();
    bytes.len() >= 3
        && bytes[1] == b':'
        && (bytes[2] == b'/' || bytes[2] == b'\\')
        && bytes[0].is_ascii_alphabetic()
}

fn writer_target_path_is_url_ref(target_path: &str) -> bool {
    target_path.contains("://")
        || target_path.starts_with("file:")
        || target_path.starts_with("http:")
        || target_path.starts_with("https:")
        || target_path.starts_with("ssh:")
}

fn writer_target_path_policy_diagnostics(
    target_path_ref: &ProofpackWriterTargetPathRef,
    reasons: &[ProofpackWriterTargetPathPolicyReason],
) -> Vec<ProofpackWriterFileIoDiagnostic> {
    reasons
        .iter()
        .map(|reason| {
            ProofpackWriterFileIoDiagnostic::for_reason(reason.file_io_error_reason())
                .with_target_path_ref(target_path_ref.clone())
                .with_source(ProofpackWriterFileIoDiagnosticSource::WriterDiagnostic)
        })
        .collect()
}

fn writer_target_path_policy_boundary_notes(
    boundary_notes: Vec<ProofBoundaryNote>,
) -> Vec<ProofBoundaryNote> {
    if boundary_notes.is_empty() {
        return vec![ProofBoundaryNote::new(
            "Writer target path policy is evidence-only; it classifies explicit refs without resolving host paths or touching the filesystem.",
        )
        .expect("fallback boundary note should be valid")];
    }

    boundary_notes
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProofpackWriterTargetPathPolicyModelBoundary {
    pub models_target_path_policy: bool,
    pub classifies_explicit_target_path_refs: bool,
    pub keeps_storage_root_target_ref_and_path_separate: bool,
    pub maps_to_file_io_diagnostics: bool,
    pub reads_filesystem: bool,
    pub touches_filesystem: bool,
    pub canonicalizes_host_paths: bool,
    pub writes_proofpack: bool,
    pub writes_writer_operation_evidence: bool,
    pub writes_final_decision: bool,
    pub creates_acceptance_claim: bool,
    pub requires_runtime_storage: bool,
    pub writes_cli_output: bool,
    pub writes_schema_files: bool,
    pub evidence_only: bool,
    pub target_path_is_authority: bool,
    pub storage_root_ref_is_authority: bool,
    pub derives_from_current_working_directory: bool,
    pub uses_indexes_or_latest_as_authority: bool,
}

pub const fn proofpack_writer_target_path_policy_model_boundary(
) -> ProofpackWriterTargetPathPolicyModelBoundary {
    ProofpackWriterTargetPathPolicyModelBoundary {
        models_target_path_policy: true,
        classifies_explicit_target_path_refs: true,
        keeps_storage_root_target_ref_and_path_separate: true,
        maps_to_file_io_diagnostics: true,
        reads_filesystem: false,
        touches_filesystem: false,
        canonicalizes_host_paths: false,
        writes_proofpack: false,
        writes_writer_operation_evidence: false,
        writes_final_decision: false,
        creates_acceptance_claim: false,
        requires_runtime_storage: false,
        writes_cli_output: false,
        writes_schema_files: false,
        evidence_only: true,
        target_path_is_authority: false,
        storage_root_ref_is_authority: false,
        derives_from_current_working_directory: false,
        uses_indexes_or_latest_as_authority: false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterPreflightIntegrationStatus {
    Ready,
    Blocked,
    NotSelected,
}

impl ProofpackWriterPreflightIntegrationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Blocked => "blocked",
            Self::NotSelected => "not_selected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofpackWriterPreflightIntegrationBlocker {
    MissingCanonicalArtifactModel,
    CanonicalArtifactProofpackMismatch,
    CanonicalArtifactDigestMismatch,
    MissingTargetArtifactRefPolicy,
    RejectedTargetArtifactRefPolicy,
    MissingLogicalTargetArtifactRef,
    TargetArtifactRefNotLogical,
    TargetArtifactRefMismatch,
    MissingPreflightPlan,
    PreflightPlanMissingPreconditions,
    MissingFileIoPlan,
    FileIoPlanBlocked,
    MissingTargetPathPolicy,
    RejectedTargetPathPolicy,
    TargetPathPolicyMismatch,
    RefsNotSeparated,
    MissingBoundaryNotes,
}

impl ProofpackWriterPreflightIntegrationBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingCanonicalArtifactModel => "missing_canonical_artifact_model",
            Self::CanonicalArtifactProofpackMismatch => "canonical_artifact_proofpack_mismatch",
            Self::CanonicalArtifactDigestMismatch => "canonical_artifact_digest_mismatch",
            Self::MissingTargetArtifactRefPolicy => "missing_target_artifact_ref_policy",
            Self::RejectedTargetArtifactRefPolicy => "rejected_target_artifact_ref_policy",
            Self::MissingLogicalTargetArtifactRef => "missing_logical_target_artifact_ref",
            Self::TargetArtifactRefNotLogical => "target_artifact_ref_not_logical",
            Self::TargetArtifactRefMismatch => "target_artifact_ref_mismatch",
            Self::MissingPreflightPlan => "missing_preflight_plan",
            Self::PreflightPlanMissingPreconditions => "preflight_plan_missing_preconditions",
            Self::MissingFileIoPlan => "missing_file_io_plan",
            Self::FileIoPlanBlocked => "file_io_plan_blocked",
            Self::MissingTargetPathPolicy => "missing_target_path_policy",
            Self::RejectedTargetPathPolicy => "rejected_target_path_policy",
            Self::TargetPathPolicyMismatch => "target_path_policy_mismatch",
            Self::RefsNotSeparated => "refs_not_separated",
            Self::MissingBoundaryNotes => "missing_boundary_notes",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofpackWriterPreflightIntegrationModel {
    proofpack_id: ProofpackId,
    schema_version: &'static str,
    writer_selected: bool,
    canonical_layout: Option<ProofpackWriterCanonicalArtifactLayout>,
    manifest_self_digest: Option<ArtifactDigest>,
    target_artifact_ref: Option<ProofpackWriterTargetRef>,
    storage_root_ref: Option<ProofpackWriterStorageRootRef>,
    target_path_ref: Option<ProofpackWriterTargetPathRef>,
    target_path_policy_status: Option<ProofpackWriterTargetPathPolicyStatus>,
    write_policy: Option<ProofpackWriterWritePolicy>,
    idempotency_basis: Option<ProofpackWriterIdempotencyBasis>,
    temp_atomic_policy: Option<ProofpackWriterTempAtomicPolicy>,
    planned_side_effects: Vec<ProofpackWriterPlannedSideEffect>,
    failure_visibility: Vec<ProofpackWriterFileIoFailureVisibility>,
    blockers: Vec<ProofpackWriterPreflightIntegrationBlocker>,
    diagnostics: Vec<ProofpackWriterFileIoDiagnostic>,
    boundary_notes: Vec<ProofBoundaryNote>,
}

impl ProofpackWriterPreflightIntegrationModel {
    pub fn evaluate(
        proofpack: &Proofpack,
        canonical_artifact: Option<&ProofpackWriterCanonicalArtifactModel>,
        target_artifact_ref_policy: Option<&ProofpackWriterTargetArtifactRefPolicyModel>,
        preflight_plan: Option<&ProofpackWriterPreflightPlan>,
        file_io_plan: Option<&ProofpackWriterFileIoPlan>,
        target_path_policy: Option<&ProofpackWriterTargetPathPolicyModel>,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> Self {
        let writer_selected =
            preflight_plan.is_some() || file_io_plan.is_some() || target_path_policy.is_some();
        let target_artifact_ref = writer_integration_target_artifact_ref(
            target_artifact_ref_policy,
            preflight_plan,
            file_io_plan,
            target_path_policy,
        );
        let storage_root_ref = file_io_plan
            .map(|plan| plan.storage_root_ref().clone())
            .or_else(|| target_path_policy.map(|policy| policy.storage_root_ref().clone()));
        let target_path_ref = file_io_plan
            .map(|plan| plan.target_path_ref().clone())
            .or_else(|| target_path_policy.map(|policy| policy.target_path_ref().clone()));
        let planned_side_effects = file_io_plan
            .map(|plan| plan.planned_side_effects().to_vec())
            .or_else(|| preflight_plan.map(|plan| plan.planned_side_effects().to_vec()))
            .unwrap_or_default();
        let failure_visibility = file_io_plan
            .map(|plan| plan.failure_visibility().to_vec())
            .unwrap_or_default();
        let diagnostics = target_path_policy
            .map(|policy| policy.diagnostics().to_vec())
            .unwrap_or_default();
        let manifest_self_digest = canonical_artifact
            .map(|canonical| canonical.manifest_self_digest().clone())
            .or_else(|| preflight_plan.map(|plan| plan.manifest_self_digest().clone()))
            .or_else(|| file_io_plan.map(|plan| plan.manifest_self_digest().clone()));
        let canonical_layout =
            canonical_artifact.map(ProofpackWriterCanonicalArtifactModel::layout);

        let blockers = if writer_selected {
            writer_preflight_integration_blockers(
                proofpack,
                canonical_artifact,
                target_artifact_ref_policy,
                preflight_plan,
                file_io_plan,
                target_path_policy,
                &target_artifact_ref,
                storage_root_ref.as_ref(),
                target_path_ref.as_ref(),
                &boundary_notes,
            )
        } else {
            Vec::new()
        };

        Self {
            proofpack_id: proofpack.id().clone(),
            schema_version: PROOFPACK_WRITER_PREFLIGHT_INTEGRATION_MODEL_SCHEMA_VERSION,
            writer_selected,
            canonical_layout,
            manifest_self_digest,
            target_artifact_ref,
            storage_root_ref,
            target_path_ref,
            target_path_policy_status: target_path_policy
                .map(ProofpackWriterTargetPathPolicyModel::status),
            write_policy: file_io_plan.map(ProofpackWriterFileIoPlan::write_policy),
            idempotency_basis: file_io_plan.map(ProofpackWriterFileIoPlan::idempotency_basis),
            temp_atomic_policy: file_io_plan.map(ProofpackWriterFileIoPlan::temp_atomic_policy),
            planned_side_effects,
            failure_visibility,
            blockers,
            diagnostics,
            boundary_notes: writer_preflight_integration_boundary_notes(boundary_notes),
        }
    }

    pub fn not_selected(
        proofpack: &Proofpack,
        canonical_artifact: Option<&ProofpackWriterCanonicalArtifactModel>,
        target_artifact_ref_policy: Option<&ProofpackWriterTargetArtifactRefPolicyModel>,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> Self {
        Self::evaluate(
            proofpack,
            canonical_artifact,
            target_artifact_ref_policy,
            None,
            None,
            None,
            boundary_notes,
        )
    }

    pub fn proofpack_id(&self) -> &ProofpackId {
        &self.proofpack_id
    }

    pub fn schema_version(&self) -> &str {
        self.schema_version
    }

    pub fn writer_selected(&self) -> bool {
        self.writer_selected
    }

    pub fn canonical_layout(&self) -> Option<ProofpackWriterCanonicalArtifactLayout> {
        self.canonical_layout
    }

    pub fn manifest_self_digest(&self) -> Option<&ArtifactDigest> {
        self.manifest_self_digest.as_ref()
    }

    pub fn target_artifact_ref(&self) -> Option<&ProofpackWriterTargetRef> {
        self.target_artifact_ref.as_ref()
    }

    pub fn target_ref(&self) -> Option<&ProofpackWriterTargetRef> {
        self.target_artifact_ref()
    }

    pub fn storage_root_ref(&self) -> Option<&ProofpackWriterStorageRootRef> {
        self.storage_root_ref.as_ref()
    }

    pub fn target_path_ref(&self) -> Option<&ProofpackWriterTargetPathRef> {
        self.target_path_ref.as_ref()
    }

    pub fn target_path_policy_status(&self) -> Option<ProofpackWriterTargetPathPolicyStatus> {
        self.target_path_policy_status
    }

    pub fn write_policy(&self) -> Option<ProofpackWriterWritePolicy> {
        self.write_policy
    }

    pub fn idempotency_basis(&self) -> Option<ProofpackWriterIdempotencyBasis> {
        self.idempotency_basis
    }

    pub fn temp_atomic_policy(&self) -> Option<ProofpackWriterTempAtomicPolicy> {
        self.temp_atomic_policy
    }

    pub fn planned_side_effects(&self) -> &[ProofpackWriterPlannedSideEffect] {
        &self.planned_side_effects
    }

    pub fn failure_visibility(&self) -> &[ProofpackWriterFileIoFailureVisibility] {
        &self.failure_visibility
    }

    pub fn blockers(&self) -> &[ProofpackWriterPreflightIntegrationBlocker] {
        &self.blockers
    }

    pub fn diagnostics(&self) -> &[ProofpackWriterFileIoDiagnostic] {
        &self.diagnostics
    }

    pub fn boundary_notes(&self) -> &[ProofBoundaryNote] {
        &self.boundary_notes
    }

    pub fn status(&self) -> ProofpackWriterPreflightIntegrationStatus {
        if self.is_not_selected() {
            ProofpackWriterPreflightIntegrationStatus::NotSelected
        } else if self.blockers.is_empty() {
            ProofpackWriterPreflightIntegrationStatus::Ready
        } else {
            ProofpackWriterPreflightIntegrationStatus::Blocked
        }
    }

    pub fn is_writer_ready(&self) -> bool {
        self.status() == ProofpackWriterPreflightIntegrationStatus::Ready
    }

    pub fn is_blocked(&self) -> bool {
        self.status() == ProofpackWriterPreflightIntegrationStatus::Blocked
    }

    pub fn is_not_selected(&self) -> bool {
        !self.writer_selected
    }

    pub fn has_blockers(&self) -> bool {
        !self.blockers.is_empty()
    }

    pub fn has_blocker(&self, blocker: ProofpackWriterPreflightIntegrationBlocker) -> bool {
        self.blockers.contains(&blocker)
    }

    pub fn refs_are_separated(&self) -> bool {
        writer_preflight_integration_refs_are_separated(
            self.storage_root_ref.as_ref(),
            self.target_artifact_ref.as_ref(),
            self.target_path_ref.as_ref(),
        )
    }

    pub fn operation_kind(&self) -> ProofpackWriterOperationKind {
        ProofpackWriterOperationKind::PlannedOnly
    }

    pub fn operation_outcome(&self) -> ProofpackWriterOperationOutcome {
        if self.is_blocked() {
            ProofpackWriterOperationOutcome::PreflightFailed
        } else {
            ProofpackWriterOperationOutcome::PlannedOnly
        }
    }

    pub fn boundary(&self) -> ProofpackWriterPreflightIntegrationModelBoundary {
        proofpack_writer_preflight_integration_model_boundary()
    }

    pub fn is_evidence_only(&self) -> bool {
        self.boundary().evidence_only
    }

    pub fn reads_filesystem(&self) -> bool {
        self.boundary().reads_filesystem
    }

    pub fn touches_filesystem(&self) -> bool {
        self.boundary().touches_filesystem
    }

    pub fn writes_proofpack(&self) -> bool {
        self.boundary().writes_proofpack
    }

    pub fn writes_punk_proofs(&self) -> bool {
        self.boundary().writes_punk_proofs
    }

    pub fn writes_writer_operation_evidence(&self) -> bool {
        self.boundary().writes_writer_operation_evidence
    }

    pub fn persists_operation_evidence(&self) -> bool {
        self.boundary().persists_operation_evidence
    }

    pub fn writes_indexes_or_latest(&self) -> bool {
        self.boundary().writes_indexes_or_latest
    }

    pub fn requires_runtime_storage(&self) -> bool {
        self.boundary().requires_runtime_storage
    }

    pub fn writes_cli_output(&self) -> bool {
        self.boundary().writes_cli_output
    }

    pub fn writes_schema_files(&self) -> bool {
        self.boundary().writes_schema_files
    }

    pub fn verifies_referenced_artifacts(&self) -> bool {
        self.boundary().verifies_referenced_artifacts
    }

    pub fn creates_acceptance_claim(&self) -> bool {
        self.boundary().creates_acceptance_claim
    }

    pub fn can_claim_acceptance_by_itself(&self) -> bool {
        false
    }

    pub fn target_path_is_authority(&self) -> bool {
        self.boundary().target_path_is_authority
    }

    pub fn storage_root_ref_is_authority(&self) -> bool {
        self.boundary().storage_root_ref_is_authority
    }

    pub fn executor_claims_are_proof(&self) -> bool {
        self.boundary().executor_claims_are_proof
    }
}

fn writer_integration_target_artifact_ref(
    target_artifact_ref_policy: Option<&ProofpackWriterTargetArtifactRefPolicyModel>,
    preflight_plan: Option<&ProofpackWriterPreflightPlan>,
    file_io_plan: Option<&ProofpackWriterFileIoPlan>,
    target_path_policy: Option<&ProofpackWriterTargetPathPolicyModel>,
) -> Option<ProofpackWriterTargetRef> {
    target_artifact_ref_policy
        .and_then(ProofpackWriterTargetRef::from_target_artifact_ref_policy_model)
        .or_else(|| preflight_plan.map(|plan| plan.target_ref().clone()))
        .or_else(|| file_io_plan.map(|plan| plan.target_artifact_ref().clone()))
        .or_else(|| target_path_policy.map(|policy| policy.target_artifact_ref().clone()))
}

fn writer_preflight_integration_blockers(
    proofpack: &Proofpack,
    canonical_artifact: Option<&ProofpackWriterCanonicalArtifactModel>,
    target_artifact_ref_policy: Option<&ProofpackWriterTargetArtifactRefPolicyModel>,
    preflight_plan: Option<&ProofpackWriterPreflightPlan>,
    file_io_plan: Option<&ProofpackWriterFileIoPlan>,
    target_path_policy: Option<&ProofpackWriterTargetPathPolicyModel>,
    target_artifact_ref: &Option<ProofpackWriterTargetRef>,
    storage_root_ref: Option<&ProofpackWriterStorageRootRef>,
    target_path_ref: Option<&ProofpackWriterTargetPathRef>,
    boundary_notes: &[ProofBoundaryNote],
) -> Vec<ProofpackWriterPreflightIntegrationBlocker> {
    let mut blockers = Vec::new();

    match canonical_artifact {
        Some(canonical) => {
            if canonical.proofpack_id() != proofpack.id() {
                blockers.push(
                    ProofpackWriterPreflightIntegrationBlocker::CanonicalArtifactProofpackMismatch,
                );
            }
            if canonical.manifest_self_digest() != &compute_proofpack_manifest_digest(proofpack)
                || !canonical.manifest_self_digest_covers_canonical_body()
            {
                blockers.push(
                    ProofpackWriterPreflightIntegrationBlocker::CanonicalArtifactDigestMismatch,
                );
            }
        }
        None => {
            blockers.push(ProofpackWriterPreflightIntegrationBlocker::MissingCanonicalArtifactModel)
        }
    }

    match target_artifact_ref_policy {
        Some(policy) => {
            if policy.is_rejected() {
                blockers.push(
                    ProofpackWriterPreflightIntegrationBlocker::RejectedTargetArtifactRefPolicy,
                );
            }
            if policy.logical_ref().is_none() {
                blockers.push(
                    ProofpackWriterPreflightIntegrationBlocker::MissingLogicalTargetArtifactRef,
                );
            }
        }
        None => blockers
            .push(ProofpackWriterPreflightIntegrationBlocker::MissingTargetArtifactRefPolicy),
    }

    match target_artifact_ref {
        Some(target_ref) if !target_ref.is_aligned_target_artifact_ref() => {
            blockers.push(ProofpackWriterPreflightIntegrationBlocker::TargetArtifactRefNotLogical)
        }
        None => blockers
            .push(ProofpackWriterPreflightIntegrationBlocker::MissingLogicalTargetArtifactRef),
        Some(_) => {}
    }

    if writer_preflight_integration_target_refs_mismatch(
        target_artifact_ref_policy,
        preflight_plan,
        file_io_plan,
        target_path_policy,
    ) {
        blockers.push(ProofpackWriterPreflightIntegrationBlocker::TargetArtifactRefMismatch);
    }

    match preflight_plan {
        Some(plan) => {
            if plan.has_missing_preconditions() {
                blockers.push(
                    ProofpackWriterPreflightIntegrationBlocker::PreflightPlanMissingPreconditions,
                );
            }
            if plan.proofpack_id() != proofpack.id() {
                blockers.push(
                    ProofpackWriterPreflightIntegrationBlocker::CanonicalArtifactProofpackMismatch,
                );
            }
        }
        None => blockers.push(ProofpackWriterPreflightIntegrationBlocker::MissingPreflightPlan),
    }

    match file_io_plan {
        Some(plan) => {
            if plan.has_file_io_blockers() {
                blockers.push(ProofpackWriterPreflightIntegrationBlocker::FileIoPlanBlocked);
            }
            if plan.proofpack_id() != proofpack.id() {
                blockers.push(
                    ProofpackWriterPreflightIntegrationBlocker::CanonicalArtifactProofpackMismatch,
                );
            }
        }
        None => blockers.push(ProofpackWriterPreflightIntegrationBlocker::MissingFileIoPlan),
    }

    match target_path_policy {
        Some(policy) => {
            if policy.is_rejected() {
                blockers.push(ProofpackWriterPreflightIntegrationBlocker::RejectedTargetPathPolicy);
            }
        }
        None => blockers.push(ProofpackWriterPreflightIntegrationBlocker::MissingTargetPathPolicy),
    }

    if writer_preflight_integration_target_path_policy_mismatch(file_io_plan, target_path_policy) {
        blockers.push(ProofpackWriterPreflightIntegrationBlocker::TargetPathPolicyMismatch);
    }

    if !writer_preflight_integration_refs_are_separated(
        storage_root_ref,
        target_artifact_ref.as_ref(),
        target_path_ref,
    ) {
        blockers.push(ProofpackWriterPreflightIntegrationBlocker::RefsNotSeparated);
    }

    if boundary_notes.is_empty() {
        blockers.push(ProofpackWriterPreflightIntegrationBlocker::MissingBoundaryNotes);
    }

    blockers
}

fn writer_preflight_integration_target_refs_mismatch(
    target_artifact_ref_policy: Option<&ProofpackWriterTargetArtifactRefPolicyModel>,
    preflight_plan: Option<&ProofpackWriterPreflightPlan>,
    file_io_plan: Option<&ProofpackWriterFileIoPlan>,
    target_path_policy: Option<&ProofpackWriterTargetPathPolicyModel>,
) -> bool {
    let expected = target_artifact_ref_policy
        .and_then(ProofpackWriterTargetRef::from_target_artifact_ref_policy_model);

    let Some(expected) = expected else {
        return false;
    };

    preflight_plan
        .map(|plan| plan.target_ref() != &expected)
        .unwrap_or(false)
        || file_io_plan
            .map(|plan| plan.target_artifact_ref() != &expected)
            .unwrap_or(false)
        || target_path_policy
            .map(|policy| policy.target_artifact_ref() != &expected)
            .unwrap_or(false)
}

fn writer_preflight_integration_target_path_policy_mismatch(
    file_io_plan: Option<&ProofpackWriterFileIoPlan>,
    target_path_policy: Option<&ProofpackWriterTargetPathPolicyModel>,
) -> bool {
    match (file_io_plan, target_path_policy) {
        (Some(plan), Some(policy)) => {
            plan.storage_root_ref() != policy.storage_root_ref()
                || plan.target_artifact_ref() != policy.target_artifact_ref()
                || plan.target_path_ref() != policy.target_path_ref()
        }
        _ => false,
    }
}

fn writer_preflight_integration_refs_are_separated(
    storage_root_ref: Option<&ProofpackWriterStorageRootRef>,
    target_artifact_ref: Option<&ProofpackWriterTargetRef>,
    target_path_ref: Option<&ProofpackWriterTargetPathRef>,
) -> bool {
    match (storage_root_ref, target_artifact_ref, target_path_ref) {
        (Some(storage_root_ref), Some(target_artifact_ref), Some(target_path_ref)) => {
            target_artifact_ref.is_aligned_target_artifact_ref()
                && storage_root_ref.as_str() != target_artifact_ref.as_str()
                && storage_root_ref.as_str() != target_path_ref.as_str()
                && target_artifact_ref.as_str() != target_path_ref.as_str()
        }
        (None, Some(target_artifact_ref), None) => {
            target_artifact_ref.is_aligned_target_artifact_ref()
        }
        _ => false,
    }
}

fn writer_preflight_integration_boundary_notes(
    boundary_notes: Vec<ProofBoundaryNote>,
) -> Vec<ProofBoundaryNote> {
    if boundary_notes.is_empty() {
        return vec![ProofBoundaryNote::new(
            "Writer preflight integration model is evidence-only; missing boundary notes remain visible blockers when writer behavior is selected.",
        )
        .expect("fallback boundary note should be valid")];
    }

    boundary_notes
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProofpackWriterPreflightIntegrationModelBoundary {
    pub models_preflight_integration: bool,
    pub composes_explicit_model_inputs: bool,
    pub keeps_storage_root_target_artifact_and_path_separate: bool,
    pub blockers_fail_closed: bool,
    pub statuses_are_evidence_only: bool,
    pub reads_filesystem: bool,
    pub touches_filesystem: bool,
    pub canonicalizes_host_paths: bool,
    pub writes_proofpack: bool,
    pub writes_punk_proofs: bool,
    pub writes_writer_operation_evidence: bool,
    pub persists_operation_evidence: bool,
    pub writes_indexes_or_latest: bool,
    pub writes_final_decision: bool,
    pub creates_acceptance_claim: bool,
    pub requires_runtime_storage: bool,
    pub writes_cli_output: bool,
    pub writes_schema_files: bool,
    pub verifies_referenced_artifacts: bool,
    pub uses_indexes_or_latest_as_authority: bool,
    pub uses_service_mirror_as_authority: bool,
    pub executor_claims_are_proof: bool,
    pub evidence_only: bool,
    pub setup_neutral: bool,
    pub target_path_is_authority: bool,
    pub storage_root_ref_is_authority: bool,
}

pub const fn proofpack_writer_preflight_integration_model_boundary(
) -> ProofpackWriterPreflightIntegrationModelBoundary {
    ProofpackWriterPreflightIntegrationModelBoundary {
        models_preflight_integration: true,
        composes_explicit_model_inputs: true,
        keeps_storage_root_target_artifact_and_path_separate: true,
        blockers_fail_closed: true,
        statuses_are_evidence_only: true,
        reads_filesystem: false,
        touches_filesystem: false,
        canonicalizes_host_paths: false,
        writes_proofpack: false,
        writes_punk_proofs: false,
        writes_writer_operation_evidence: false,
        persists_operation_evidence: false,
        writes_indexes_or_latest: false,
        writes_final_decision: false,
        creates_acceptance_claim: false,
        requires_runtime_storage: false,
        writes_cli_output: false,
        writes_schema_files: false,
        verifies_referenced_artifacts: false,
        uses_indexes_or_latest_as_authority: false,
        uses_service_mirror_as_authority: false,
        executor_claims_are_proof: false,
        evidence_only: true,
        setup_neutral: true,
        target_path_is_authority: false,
        storage_root_ref_is_authority: false,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofpackWriterActiveBehaviorModel {
    proofpack_id: ProofpackId,
    schema_version: &'static str,
    preflight_status: ProofpackWriterPreflightIntegrationStatus,
    target_artifact_ref: Option<ProofpackWriterTargetRef>,
    storage_root_ref: Option<ProofpackWriterStorageRootRef>,
    target_path_ref: Option<ProofpackWriterTargetPathRef>,
    selected_side_effects: Vec<ProofpackWriterPlannedSideEffect>,
    attempted_side_effects: Vec<ProofpackWriterPlannedSideEffect>,
    completed_side_effects: Vec<ProofpackWriterPlannedSideEffect>,
    failed_side_effects: Vec<ProofpackWriterPlannedSideEffect>,
    operation_evidence_persistence_status: ProofpackWriterSideEffectStatus,
    outcome: ProofpackWriterOperationOutcome,
    operation_kind: ProofpackWriterOperationKind,
    canonical_artifact_status: ProofpackWriterCanonicalArtifactStatus,
    index_status: ProofpackWriterSideEffectStatus,
    latest_pointer_status: ProofpackWriterSideEffectStatus,
    cleanup_status: ProofpackWriterSideEffectStatus,
    observation: Option<ProofpackWriterFileIoObservation>,
    boundary_notes: Vec<ProofBoundaryNote>,
}

impl ProofpackWriterActiveBehaviorModel {
    pub fn from_preflight_and_observation(
        preflight: &ProofpackWriterPreflightIntegrationModel,
        observation: Option<ProofpackWriterFileIoObservation>,
        operation_evidence_persistence_status: ProofpackWriterSideEffectStatus,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> Self {
        let outcome = writer_active_behavior_outcome(preflight, observation.as_ref());
        let canonical_artifact_status =
            writer_active_behavior_canonical_artifact_status(outcome, observation.as_ref());
        let index_status = writer_active_behavior_side_effect_status(
            preflight,
            observation.as_ref(),
            ProofpackWriterPlannedSideEffect::IndexUpdate,
        );
        let latest_pointer_status = writer_active_behavior_side_effect_status(
            preflight,
            observation.as_ref(),
            ProofpackWriterPlannedSideEffect::LatestPointerUpdate,
        );
        let cleanup_status = observation
            .as_ref()
            .map(ProofpackWriterFileIoObservation::cleanup_status)
            .unwrap_or(ProofpackWriterSideEffectStatus::NotAttempted);
        let operation_kind = writer_file_io_operation_kind(outcome);
        let selected_side_effects = preflight.planned_side_effects().to_vec();
        let attempted_side_effects = writer_active_behavior_side_effects_by_status(
            &selected_side_effects,
            canonical_artifact_status,
            index_status,
            latest_pointer_status,
            ProofpackWriterSideEffectStatus::Failed,
            ProofpackWriterSideEffectStatus::Completed,
        );
        let completed_side_effects = writer_active_behavior_side_effects_by_status(
            &selected_side_effects,
            canonical_artifact_status,
            index_status,
            latest_pointer_status,
            ProofpackWriterSideEffectStatus::Completed,
            ProofpackWriterSideEffectStatus::Completed,
        );
        let failed_side_effects = writer_active_behavior_side_effects_by_status(
            &selected_side_effects,
            canonical_artifact_status,
            index_status,
            latest_pointer_status,
            ProofpackWriterSideEffectStatus::Failed,
            ProofpackWriterSideEffectStatus::Failed,
        );
        let boundary_notes =
            writer_active_behavior_boundary_notes(preflight, &observation, boundary_notes);

        Self {
            proofpack_id: preflight.proofpack_id().clone(),
            schema_version: PROOFPACK_WRITER_ACTIVE_BEHAVIOR_MODEL_SCHEMA_VERSION,
            preflight_status: preflight.status(),
            target_artifact_ref: preflight.target_artifact_ref().cloned(),
            storage_root_ref: preflight.storage_root_ref().cloned(),
            target_path_ref: preflight.target_path_ref().cloned(),
            selected_side_effects,
            attempted_side_effects,
            completed_side_effects,
            failed_side_effects,
            operation_evidence_persistence_status,
            outcome,
            operation_kind,
            canonical_artifact_status,
            index_status,
            latest_pointer_status,
            cleanup_status,
            observation,
            boundary_notes,
        }
    }

    pub fn planned_only(
        preflight: &ProofpackWriterPreflightIntegrationModel,
        boundary_notes: Vec<ProofBoundaryNote>,
    ) -> Self {
        Self::from_preflight_and_observation(
            preflight,
            None,
            ProofpackWriterSideEffectStatus::NotSelected,
            boundary_notes,
        )
    }

    pub fn proofpack_id(&self) -> &ProofpackId {
        &self.proofpack_id
    }

    pub fn schema_version(&self) -> &str {
        self.schema_version
    }

    pub fn preflight_status(&self) -> ProofpackWriterPreflightIntegrationStatus {
        self.preflight_status
    }

    pub fn target_artifact_ref(&self) -> Option<&ProofpackWriterTargetRef> {
        self.target_artifact_ref.as_ref()
    }

    pub fn storage_root_ref(&self) -> Option<&ProofpackWriterStorageRootRef> {
        self.storage_root_ref.as_ref()
    }

    pub fn target_path_ref(&self) -> Option<&ProofpackWriterTargetPathRef> {
        self.target_path_ref.as_ref()
    }

    pub fn selected_side_effects(&self) -> &[ProofpackWriterPlannedSideEffect] {
        &self.selected_side_effects
    }

    pub fn attempted_side_effects(&self) -> &[ProofpackWriterPlannedSideEffect] {
        &self.attempted_side_effects
    }

    pub fn completed_side_effects(&self) -> &[ProofpackWriterPlannedSideEffect] {
        &self.completed_side_effects
    }

    pub fn failed_side_effects(&self) -> &[ProofpackWriterPlannedSideEffect] {
        &self.failed_side_effects
    }

    pub fn operation_evidence_persistence_status(&self) -> ProofpackWriterSideEffectStatus {
        self.operation_evidence_persistence_status
    }

    pub fn outcome(&self) -> ProofpackWriterOperationOutcome {
        self.outcome
    }

    pub fn operation_kind(&self) -> ProofpackWriterOperationKind {
        self.operation_kind
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

    pub fn cleanup_status(&self) -> ProofpackWriterSideEffectStatus {
        self.cleanup_status
    }

    pub fn observation(&self) -> Option<&ProofpackWriterFileIoObservation> {
        self.observation.as_ref()
    }

    pub fn boundary_notes(&self) -> &[ProofBoundaryNote] {
        &self.boundary_notes
    }

    pub fn to_operation_evidence(
        &self,
        operation_id: ProofpackWriterOperationId,
        attempted_at: ProofpackWriterAttemptedAt,
    ) -> Result<ProofpackWriterOperationEvidence, ProofpackError> {
        let Some(target_ref) = self.target_artifact_ref.clone() else {
            return Err(ProofpackError::EmptyWriterTargetRef);
        };

        ProofpackWriterOperationEvidence::new(
            operation_id,
            self.operation_kind,
            self.proofpack_id.clone(),
            attempted_at,
            target_ref,
            self.outcome,
            self.canonical_artifact_status,
            self.index_status,
            self.latest_pointer_status,
            self.boundary_notes.clone(),
        )
    }

    pub fn boundary(&self) -> ProofpackWriterActiveBehaviorModelBoundary {
        proofpack_writer_active_behavior_model_boundary()
    }

    pub fn is_ready_planned(&self) -> bool {
        self.preflight_status == ProofpackWriterPreflightIntegrationStatus::Ready
            && self.outcome == ProofpackWriterOperationOutcome::PlannedOnly
    }

    pub fn preflight_failed(&self) -> bool {
        self.outcome == ProofpackWriterOperationOutcome::PreflightFailed
    }

    pub fn canonical_artifact_available(&self) -> bool {
        self.canonical_artifact_status.is_available()
    }

    pub fn has_conflict(&self) -> bool {
        self.canonical_artifact_status.is_conflict()
    }

    pub fn has_partial_or_cleanup_issue(&self) -> bool {
        self.canonical_artifact_status.is_partial() || self.cleanup_status.is_failed()
    }

    pub fn has_index_or_latest_pointer_failure(&self) -> bool {
        self.index_status.is_failed() || self.latest_pointer_status.is_failed()
    }

    pub fn has_operation_evidence_persistence_failure(&self) -> bool {
        self.operation_evidence_persistence_status.is_failed()
    }

    pub fn selected_side_effect_was_attempted(
        &self,
        side_effect: ProofpackWriterPlannedSideEffect,
    ) -> bool {
        self.attempted_side_effects.contains(&side_effect)
    }

    pub fn selected_side_effect_completed(
        &self,
        side_effect: ProofpackWriterPlannedSideEffect,
    ) -> bool {
        self.completed_side_effects.contains(&side_effect)
    }

    pub fn selected_side_effect_failed(
        &self,
        side_effect: ProofpackWriterPlannedSideEffect,
    ) -> bool {
        self.failed_side_effects.contains(&side_effect)
    }

    pub fn refs_are_separated(&self) -> bool {
        writer_preflight_integration_refs_are_separated(
            self.storage_root_ref.as_ref(),
            self.target_artifact_ref.as_ref(),
            self.target_path_ref.as_ref(),
        )
    }

    pub fn is_evidence_only(&self) -> bool {
        self.boundary().evidence_only
    }

    pub fn reads_filesystem(&self) -> bool {
        self.boundary().reads_filesystem
    }

    pub fn touches_filesystem(&self) -> bool {
        self.boundary().touches_filesystem
    }

    pub fn writes_proofpack(&self) -> bool {
        self.boundary().writes_proofpack
    }

    pub fn writes_punk_proofs(&self) -> bool {
        self.boundary().writes_punk_proofs
    }

    pub fn writes_writer_operation_evidence(&self) -> bool {
        self.boundary().writes_writer_operation_evidence
    }

    pub fn persists_operation_evidence(&self) -> bool {
        self.boundary().persists_operation_evidence
    }

    pub fn writes_indexes_or_latest(&self) -> bool {
        self.boundary().writes_indexes_or_latest
    }

    pub fn requires_runtime_storage(&self) -> bool {
        self.boundary().requires_runtime_storage
    }

    pub fn writes_cli_output(&self) -> bool {
        self.boundary().writes_cli_output
    }

    pub fn writes_schema_files(&self) -> bool {
        self.boundary().writes_schema_files
    }

    pub fn verifies_referenced_artifacts(&self) -> bool {
        self.boundary().verifies_referenced_artifacts
    }

    pub fn creates_acceptance_claim(&self) -> bool {
        self.boundary().creates_acceptance_claim
    }

    pub fn can_claim_acceptance_by_itself(&self) -> bool {
        false
    }

    pub fn target_path_is_authority(&self) -> bool {
        self.boundary().target_path_is_authority
    }

    pub fn storage_root_ref_is_authority(&self) -> bool {
        self.boundary().storage_root_ref_is_authority
    }

    pub fn index_latest_are_canonical(&self) -> bool {
        self.boundary().index_latest_are_canonical
    }

    pub fn executor_claims_are_proof(&self) -> bool {
        self.boundary().executor_claims_are_proof
    }
}

fn writer_active_behavior_outcome(
    preflight: &ProofpackWriterPreflightIntegrationModel,
    observation: Option<&ProofpackWriterFileIoObservation>,
) -> ProofpackWriterOperationOutcome {
    if !preflight.is_writer_ready() {
        return ProofpackWriterOperationOutcome::PreflightFailed;
    }

    let Some(observation) = observation else {
        return ProofpackWriterOperationOutcome::PlannedOnly;
    };

    if observation.has_partial_or_ambiguous_state() {
        return ProofpackWriterOperationOutcome::PartialWriteDetected;
    }

    if observation.abort_state() == ProofpackWriterAbortState::AbortedBeforeWrite {
        return ProofpackWriterOperationOutcome::Aborted;
    }

    if observation.has_conflict() {
        return ProofpackWriterOperationOutcome::ConflictExistingDifferent;
    }

    if observation.has_idempotent_match() {
        return ProofpackWriterOperationOutcome::AlreadyExistsMatching;
    }

    if observation.write_result().is_failed() {
        return ProofpackWriterOperationOutcome::WriteFailed;
    }

    if observation.write_result().is_written()
        && observation.index_status().is_failed()
        && preflight
            .planned_side_effects()
            .contains(&ProofpackWriterPlannedSideEffect::IndexUpdate)
    {
        return ProofpackWriterOperationOutcome::IndexUpdateFailed;
    }

    if observation.write_result().is_written()
        && observation.latest_pointer_status().is_failed()
        && preflight
            .planned_side_effects()
            .contains(&ProofpackWriterPlannedSideEffect::LatestPointerUpdate)
    {
        return ProofpackWriterOperationOutcome::LatestPointerUpdateFailed;
    }

    if observation.write_result().is_written() {
        return ProofpackWriterOperationOutcome::Written;
    }

    ProofpackWriterOperationOutcome::PlannedOnly
}

fn writer_active_behavior_canonical_artifact_status(
    outcome: ProofpackWriterOperationOutcome,
    observation: Option<&ProofpackWriterFileIoObservation>,
) -> ProofpackWriterCanonicalArtifactStatus {
    match observation {
        Some(observation) => writer_file_io_canonical_artifact_status(outcome, observation),
        None => ProofpackWriterCanonicalArtifactStatus::NotAttempted,
    }
}

fn writer_active_behavior_side_effect_status(
    preflight: &ProofpackWriterPreflightIntegrationModel,
    observation: Option<&ProofpackWriterFileIoObservation>,
    side_effect: ProofpackWriterPlannedSideEffect,
) -> ProofpackWriterSideEffectStatus {
    if !preflight.planned_side_effects().contains(&side_effect) {
        return ProofpackWriterSideEffectStatus::NotSelected;
    }

    if !preflight.is_writer_ready() {
        return ProofpackWriterSideEffectStatus::NotAttempted;
    }

    let Some(observation) = observation else {
        return ProofpackWriterSideEffectStatus::NotAttempted;
    };

    let observed_status = match side_effect {
        ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite => {
            match observation.write_result() {
                ProofpackWriterObservedWriteResult::Written => {
                    ProofpackWriterSideEffectStatus::Completed
                }
                ProofpackWriterObservedWriteResult::WriteFailed
                | ProofpackWriterObservedWriteResult::PartialWriteDetected => {
                    ProofpackWriterSideEffectStatus::Failed
                }
                ProofpackWriterObservedWriteResult::NotAttempted => {
                    ProofpackWriterSideEffectStatus::NotAttempted
                }
            }
        }
        ProofpackWriterPlannedSideEffect::IndexUpdate => observation.index_status(),
        ProofpackWriterPlannedSideEffect::LatestPointerUpdate => {
            observation.latest_pointer_status()
        }
    };

    if observed_status == ProofpackWriterSideEffectStatus::NotSelected {
        ProofpackWriterSideEffectStatus::NotAttempted
    } else {
        observed_status
    }
}

fn writer_active_behavior_side_effects_by_status(
    selected_side_effects: &[ProofpackWriterPlannedSideEffect],
    canonical_artifact_status: ProofpackWriterCanonicalArtifactStatus,
    index_status: ProofpackWriterSideEffectStatus,
    latest_pointer_status: ProofpackWriterSideEffectStatus,
    primary_status: ProofpackWriterSideEffectStatus,
    secondary_status: ProofpackWriterSideEffectStatus,
) -> Vec<ProofpackWriterPlannedSideEffect> {
    selected_side_effects
        .iter()
        .copied()
        .filter(|side_effect| match side_effect {
            ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite => {
                writer_active_behavior_canonical_side_effect_status(canonical_artifact_status)
                    == primary_status
                    || writer_active_behavior_canonical_side_effect_status(
                        canonical_artifact_status,
                    ) == secondary_status
            }
            ProofpackWriterPlannedSideEffect::IndexUpdate => {
                index_status == primary_status || index_status == secondary_status
            }
            ProofpackWriterPlannedSideEffect::LatestPointerUpdate => {
                latest_pointer_status == primary_status || latest_pointer_status == secondary_status
            }
        })
        .collect()
}

fn writer_active_behavior_canonical_side_effect_status(
    status: ProofpackWriterCanonicalArtifactStatus,
) -> ProofpackWriterSideEffectStatus {
    match status {
        ProofpackWriterCanonicalArtifactStatus::Written => {
            ProofpackWriterSideEffectStatus::Completed
        }
        ProofpackWriterCanonicalArtifactStatus::WriteFailed
        | ProofpackWriterCanonicalArtifactStatus::PartialWriteDetected => {
            ProofpackWriterSideEffectStatus::Failed
        }
        ProofpackWriterCanonicalArtifactStatus::NotAttempted
        | ProofpackWriterCanonicalArtifactStatus::AlreadyExistsMatching
        | ProofpackWriterCanonicalArtifactStatus::ConflictExistingDifferent => {
            ProofpackWriterSideEffectStatus::NotAttempted
        }
    }
}

fn writer_active_behavior_boundary_notes(
    preflight: &ProofpackWriterPreflightIntegrationModel,
    observation: &Option<ProofpackWriterFileIoObservation>,
    boundary_notes: Vec<ProofBoundaryNote>,
) -> Vec<ProofBoundaryNote> {
    let mut notes = preflight.boundary_notes().to_vec();
    if let Some(observation) = observation {
        notes.extend(observation.boundary_notes().iter().cloned());
    }
    notes.extend(boundary_notes);

    if notes.is_empty() {
        return vec![ProofBoundaryNote::new(
            "Writer active behavior model is evidence-only; missing boundary notes remain model data and do not authorize side effects.",
        )
        .expect("fallback boundary note should be valid")];
    }

    notes
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProofpackWriterActiveBehaviorModelBoundary {
    pub models_active_writer_behavior: bool,
    pub requires_ready_preflight: bool,
    pub accepts_explicit_observations: bool,
    pub models_selected_attempted_completed_failed_side_effects: bool,
    pub keeps_storage_root_target_artifact_and_path_separate: bool,
    pub failures_remain_visible: bool,
    pub operation_evidence_is_non_authoritative: bool,
    pub reads_filesystem: bool,
    pub touches_filesystem: bool,
    pub canonicalizes_host_paths: bool,
    pub writes_proofpack: bool,
    pub writes_punk_proofs: bool,
    pub writes_writer_operation_evidence: bool,
    pub persists_operation_evidence: bool,
    pub writes_indexes_or_latest: bool,
    pub writes_final_decision: bool,
    pub creates_acceptance_claim: bool,
    pub requires_runtime_storage: bool,
    pub writes_cli_output: bool,
    pub writes_schema_files: bool,
    pub verifies_referenced_artifacts: bool,
    pub uses_indexes_or_latest_as_authority: bool,
    pub uses_service_mirror_as_authority: bool,
    pub executor_claims_are_proof: bool,
    pub evidence_only: bool,
    pub setup_neutral: bool,
    pub target_path_is_authority: bool,
    pub storage_root_ref_is_authority: bool,
    pub index_latest_are_canonical: bool,
}

pub const fn proofpack_writer_active_behavior_model_boundary(
) -> ProofpackWriterActiveBehaviorModelBoundary {
    ProofpackWriterActiveBehaviorModelBoundary {
        models_active_writer_behavior: true,
        requires_ready_preflight: true,
        accepts_explicit_observations: true,
        models_selected_attempted_completed_failed_side_effects: true,
        keeps_storage_root_target_artifact_and_path_separate: true,
        failures_remain_visible: true,
        operation_evidence_is_non_authoritative: true,
        reads_filesystem: false,
        touches_filesystem: false,
        canonicalizes_host_paths: false,
        writes_proofpack: false,
        writes_punk_proofs: false,
        writes_writer_operation_evidence: false,
        persists_operation_evidence: false,
        writes_indexes_or_latest: false,
        writes_final_decision: false,
        creates_acceptance_claim: false,
        requires_runtime_storage: false,
        writes_cli_output: false,
        writes_schema_files: false,
        verifies_referenced_artifacts: false,
        uses_indexes_or_latest_as_authority: false,
        uses_service_mirror_as_authority: false,
        executor_claims_are_proof: false,
        evidence_only: true,
        setup_neutral: true,
        target_path_is_authority: false,
        storage_root_ref_is_authority: false,
        index_latest_are_canonical: false,
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
    EmptyWriterDiagnosticPathRef,
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

    fn sample_writer_target_artifact_ref(proofpack: &Proofpack) -> ProofpackWriterTargetRef {
        let canonical = ProofpackWriterCanonicalArtifactModel::from_proofpack(proofpack, vec![]);
        let policy = ProofpackWriterTargetArtifactRefPolicyModel::from_canonical_artifact_model(
            &canonical,
            vec![],
        );

        ProofpackWriterTargetRef::from_target_artifact_ref_policy_model(&policy)
            .expect("target artifact ref policy should derive logical ref")
    }

    fn sample_writer_operation_evidence(
        outcome: ProofpackWriterOperationOutcome,
        canonical_artifact_status: ProofpackWriterCanonicalArtifactStatus,
        index_status: ProofpackWriterSideEffectStatus,
        latest_pointer_status: ProofpackWriterSideEffectStatus,
    ) -> ProofpackWriterOperationEvidence {
        let proofpack = sample_proofpack();

        ProofpackWriterOperationEvidence::new(
            ProofpackWriterOperationId::new("writer_op_local_001")
                .expect("writer operation id should be valid"),
            ProofpackWriterOperationKind::Write,
            proofpack.id().clone(),
            ProofpackWriterAttemptedAt::new("2026-04-26T13:00:00Z")
                .expect("attempted_at should be valid"),
            sample_writer_target_artifact_ref(&proofpack),
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
            sample_writer_target_artifact_ref(proofpack),
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

    fn sample_writer_file_io_ready_plan() -> ProofpackWriterFileIoPlan {
        let proofpack = sample_proofpack();
        let preflight_plan = sample_writer_preflight_plan(
            &proofpack,
            vec![
                ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite,
                ProofpackWriterPlannedSideEffect::IndexUpdate,
                ProofpackWriterPlannedSideEffect::LatestPointerUpdate,
            ],
            vec![ProofBoundaryNote::new("Preflight is complete.")
                .expect("boundary note should be valid")],
        );

        sample_writer_file_io_plan(
            &preflight_plan,
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
                "File IO plan is model-only and keeps runtime writes deferred.",
            )
            .expect("boundary note should be valid")],
        )
    }

    fn sample_writer_preflight_integration_ready_model() -> ProofpackWriterPreflightIntegrationModel
    {
        let proofpack = sample_proofpack();
        let canonical = ProofpackWriterCanonicalArtifactModel::from_proofpack(
            &proofpack,
            vec![
                ProofBoundaryNote::new("Canonical artifact input is explicit.")
                    .expect("boundary note should be valid"),
            ],
        );
        let target_ref_policy =
            ProofpackWriterTargetArtifactRefPolicyModel::from_canonical_artifact_model(
                &canonical,
                vec![
                    ProofBoundaryNote::new("Target artifact ref policy is explicit.")
                        .expect("boundary note should be valid"),
                ],
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
                ProofBoundaryNote::new("Preflight plan is writer-ready evidence.")
                    .expect("boundary note should be valid"),
            ],
        );
        let file_io_plan = sample_writer_file_io_plan(
            &preflight_plan,
            vec![
                ProofpackWriterFileIoFailureVisibility::ExistingTargetMatching,
                ProofpackWriterFileIoFailureVisibility::ExistingTargetDifferent,
                ProofpackWriterFileIoFailureVisibility::AtomicMoveFailed,
                ProofpackWriterFileIoFailureVisibility::CleanupFailed,
                ProofpackWriterFileIoFailureVisibility::PartialCanonicalArtifactAmbiguous,
                ProofpackWriterFileIoFailureVisibility::IndexUpdateFailed,
                ProofpackWriterFileIoFailureVisibility::LatestPointerUpdateFailed,
            ],
            vec![
                ProofBoundaryNote::new("File IO policy is explicit and side-effect-free.")
                    .expect("boundary note should be valid"),
            ],
        );
        let target_path_policy = ProofpackWriterTargetPathPolicyModel::from_plan(
            &file_io_plan,
            vec![ProofBoundaryNote::new("Target path policy is explicit.")
                .expect("boundary note should be valid")],
        );

        ProofpackWriterPreflightIntegrationModel::evaluate(
            &proofpack,
            Some(&canonical),
            Some(&target_ref_policy),
            Some(&preflight_plan),
            Some(&file_io_plan),
            Some(&target_path_policy),
            vec![
                ProofBoundaryNote::new("Integrated preflight is ready and evidence-only.")
                    .expect("boundary note should be valid"),
            ],
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
    fn proofpack_writer_canonical_artifact_layout_vocabulary_is_stable() {
        assert_eq!(
            ProofpackWriterCanonicalArtifactLayout::ManifestOnlyJson.as_str(),
            "manifest_only_json"
        );
        assert_eq!(
            ProofpackWriterNonCanonicalArtifactSurface::ManifestSelfDigestMetadata.as_str(),
            "manifest_self_digest_metadata"
        );
        assert_eq!(
            ProofpackWriterNonCanonicalArtifactSurface::WrapperMetadata.as_str(),
            "wrapper_metadata"
        );
        assert_eq!(
            ProofpackWriterNonCanonicalArtifactSurface::StorageRootRef.as_str(),
            "storage_root_ref"
        );
        assert_eq!(
            ProofpackWriterNonCanonicalArtifactSurface::TargetArtifactRef.as_str(),
            "target_artifact_ref"
        );
        assert_eq!(
            ProofpackWriterNonCanonicalArtifactSurface::TargetPathRef.as_str(),
            "target_path_ref"
        );
        assert_eq!(
            ProofpackWriterNonCanonicalArtifactSurface::WriterOperationEvidence.as_str(),
            "writer_operation_evidence"
        );
        assert_eq!(
            ProofpackWriterNonCanonicalArtifactSurface::SchemaValidationReport.as_str(),
            "schema_validation_report"
        );
        assert_eq!(
            ProofpackWriterNonCanonicalArtifactSurface::IndexView.as_str(),
            "index_view"
        );
        assert_eq!(
            ProofpackWriterNonCanonicalArtifactSurface::LatestPointer.as_str(),
            "latest_pointer"
        );
        assert_eq!(
            ProofpackWriterNonCanonicalArtifactSurface::CliOutput.as_str(),
            "cli_output"
        );
        assert_eq!(
            ProofpackWriterNonCanonicalArtifactSurface::ServiceMirror.as_str(),
            "service_mirror"
        );
    }

    #[test]
    fn proofpack_writer_canonical_artifact_model_uses_exact_manifest_bytes() {
        let proofpack = sample_proofpack();
        let rendered = proofpack.render_manifest_json();
        let model = ProofpackWriterCanonicalArtifactModel::from_proofpack(
            &proofpack,
            vec![ProofBoundaryNote::new(
                "Canonical artifact model is manifest-only and side-effect-free.",
            )
            .expect("boundary note should be valid")],
        );

        assert_eq!(
            model.schema_version(),
            PROOFPACK_WRITER_CANONICAL_ARTIFACT_MODEL_SCHEMA_VERSION
        );
        assert_eq!(model.proofpack_id().as_str(), proofpack.id().as_str());
        assert_eq!(model.layout().as_str(), "manifest_only_json");
        assert!(model.is_manifest_only_layout());
        assert_eq!(model.canonical_body_utf8(), rendered);
        assert_eq!(model.canonical_body_bytes(), rendered.as_bytes());
        assert!(model.canonical_body_matches_proofpack(&proofpack));
        assert_eq!(
            model.manifest_self_digest(),
            &compute_proofpack_manifest_digest(&proofpack)
        );
        assert_eq!(
            model.manifest_self_digest(),
            &compute_artifact_digest(model.canonical_body_bytes())
        );
        assert!(model.manifest_self_digest_covers_canonical_body());
        assert!(!model.manifest_self_digest_is_embedded_in_canonical_body());
        assert_eq!(model.boundary_notes().len(), 1);
    }

    #[test]
    fn proofpack_writer_canonical_artifact_model_separates_non_canonical_metadata() {
        let proofpack = sample_proofpack();
        let model = ProofpackWriterCanonicalArtifactModel::from_proofpack(&proofpack, vec![]);
        let boundary = model.boundary();

        assert_eq!(model.non_canonical_surfaces().len(), 11);
        for surface in [
            ProofpackWriterNonCanonicalArtifactSurface::ManifestSelfDigestMetadata,
            ProofpackWriterNonCanonicalArtifactSurface::WrapperMetadata,
            ProofpackWriterNonCanonicalArtifactSurface::StorageRootRef,
            ProofpackWriterNonCanonicalArtifactSurface::TargetArtifactRef,
            ProofpackWriterNonCanonicalArtifactSurface::TargetPathRef,
            ProofpackWriterNonCanonicalArtifactSurface::WriterOperationEvidence,
            ProofpackWriterNonCanonicalArtifactSurface::SchemaValidationReport,
            ProofpackWriterNonCanonicalArtifactSurface::IndexView,
            ProofpackWriterNonCanonicalArtifactSurface::LatestPointer,
            ProofpackWriterNonCanonicalArtifactSurface::CliOutput,
            ProofpackWriterNonCanonicalArtifactSurface::ServiceMirror,
        ] {
            assert!(model.surface_is_non_canonical(surface));
        }

        assert!(boundary.models_canonical_artifact_body);
        assert!(boundary.canonical_body_is_manifest_renderer_bytes);
        assert!(boundary.manifest_self_digest_covers_canonical_body);
        assert!(boundary.manifest_self_digest_metadata_outside_body);
        assert!(boundary.separates_non_canonical_metadata);
        assert!(boundary.evidence_only);
        assert!(!boundary.uses_indexes_or_latest_as_authority);
        assert_eq!(model.boundary_notes().len(), 1);
    }

    #[test]
    fn proofpack_writer_canonical_artifact_model_is_side_effect_free() {
        let proofpack = sample_proofpack();
        let model = ProofpackWriterCanonicalArtifactModel::from_proofpack(&proofpack, vec![]);
        let boundary = model.boundary();

        assert!(!model.reads_filesystem());
        assert!(!model.touches_filesystem());
        assert!(!model.writes_proofpack());
        assert!(!model.writes_writer_operation_evidence());
        assert!(!model.verifies_referenced_artifacts());
        assert!(!model.requires_runtime_storage());
        assert!(!model.writes_cli_output());
        assert!(!model.writes_schema_files());
        assert!(!model.creates_acceptance_claim());
        assert!(!model.uses_indexes_or_latest_as_authority());
        assert!(!model.can_claim_acceptance_by_itself());
        assert!(!boundary.canonicalizes_host_paths);
        assert!(!boundary.writes_final_decision);
        assert!(!boundary.creates_acceptance_claim);
    }

    #[test]
    fn proofpack_writer_target_artifact_ref_policy_vocabulary_is_stable() {
        assert_eq!(
            PROOFPACK_WRITER_TARGET_ARTIFACT_REF_POLICY_MODEL_SCHEMA_VERSION,
            "punk.proofpack.writer_target_artifact_ref_policy_model.v0.1"
        );
        assert_eq!(
            ProofpackWriterTargetArtifactRefPolicyStatus::Accepted.as_str(),
            "accepted"
        );
        assert_eq!(
            ProofpackWriterTargetArtifactRefPolicyStatus::Rejected.as_str(),
            "rejected"
        );
        assert_eq!(
            ProofpackWriterTargetArtifactRefPolicyReason::MissingProofpackId.as_str(),
            "missing_proofpack_id"
        );
        assert_eq!(
            ProofpackWriterTargetArtifactRefPolicyReason::MissingManifestSelfDigest.as_str(),
            "missing_manifest_self_digest"
        );
        assert_eq!(
            ProofpackWriterTargetArtifactRefPolicyReason::InvalidManifestSelfDigest.as_str(),
            "invalid_manifest_self_digest"
        );
        assert!(
            ProofpackWriterTargetArtifactRefPolicyReason::MissingProofpackId
                .is_missing_precondition()
        );
        assert!(
            !ProofpackWriterTargetArtifactRefPolicyReason::InvalidManifestSelfDigest
                .is_missing_precondition()
        );
    }

    #[test]
    fn proofpack_writer_target_artifact_ref_policy_accepts_explicit_identity_pair() {
        let proofpack = sample_proofpack();
        let canonical = ProofpackWriterCanonicalArtifactModel::from_proofpack(&proofpack, vec![]);
        let model = ProofpackWriterTargetArtifactRefPolicyModel::from_canonical_artifact_model(
            &canonical,
            vec![ProofBoundaryNote::new(
                "Target artifact ref policy renders a logical non-path ref.",
            )
            .expect("boundary note should be valid")],
        );
        let expected_ref = format!(
            "proofpack:{}@{}",
            canonical.proofpack_id().as_str(),
            canonical.manifest_self_digest().as_str()
        );

        assert_eq!(
            model.schema_version(),
            PROOFPACK_WRITER_TARGET_ARTIFACT_REF_POLICY_MODEL_SCHEMA_VERSION
        );
        assert!(model.is_accepted());
        assert!(!model.is_rejected());
        assert!(model.has_complete_identity());
        assert_eq!(
            model.status(),
            ProofpackWriterTargetArtifactRefPolicyStatus::Accepted
        );
        assert_eq!(model.reasons().len(), 0);
        assert_eq!(
            model
                .proofpack_id()
                .expect("identity should exist")
                .as_str(),
            proofpack.id().as_str()
        );
        assert_eq!(
            model.manifest_self_digest(),
            Some(canonical.manifest_self_digest())
        );
        assert_eq!(
            model.layout(),
            Some(ProofpackWriterCanonicalArtifactLayout::ManifestOnlyJson)
        );
        assert_eq!(model.logical_display_ref(), Some(expected_ref.as_str()));
        assert_eq!(
            model
                .logical_ref()
                .expect("logical ref should exist")
                .as_str(),
            expected_ref
        );
        let target_ref = ProofpackWriterTargetRef::from_target_artifact_ref_policy_model(&model)
            .expect("accepted target artifact ref policy should derive target ref");
        assert_eq!(target_ref.as_str(), expected_ref);
        assert!(target_ref.is_logical_proofpack_ref());
        assert!(!target_ref.is_path_like_ref());
        assert!(target_ref.is_aligned_target_artifact_ref());
        assert!(!model.display_ref_is_filesystem_path());
        assert_eq!(model.boundary_notes().len(), 1);
    }

    #[test]
    fn proofpack_writer_target_artifact_ref_policy_rejects_missing_or_invalid_identity_parts() {
        let valid_digest =
            ArtifactDigest::new(PROOF_HASH_GATE_DECISION).expect("digest should be valid");

        let missing_proofpack_id = ProofpackWriterTargetArtifactRefPolicyModel::evaluate(
            None,
            Some(valid_digest.clone()),
            ProofpackWriterCanonicalArtifactLayout::ManifestOnlyJson,
            vec![],
        );
        let missing_digest = ProofpackWriterTargetArtifactRefPolicyModel::evaluate(
            Some(ProofpackId::new("proofpack_local_001").expect("id should be valid")),
            None,
            ProofpackWriterCanonicalArtifactLayout::ManifestOnlyJson,
            vec![],
        );
        let invalid_digest = ProofpackWriterTargetArtifactRefPolicyModel::evaluate_raw(
            Some("proofpack_local_001"),
            Some("sha256:not-valid"),
            ProofpackWriterCanonicalArtifactLayout::ManifestOnlyJson,
            vec![],
        );

        assert!(missing_proofpack_id.is_rejected());
        assert!(missing_proofpack_id.has_missing_precondition());
        assert!(missing_proofpack_id
            .has_reason(ProofpackWriterTargetArtifactRefPolicyReason::MissingProofpackId));
        assert_eq!(missing_proofpack_id.logical_display_ref(), None);

        assert!(missing_digest.is_rejected());
        assert!(missing_digest.has_missing_precondition());
        assert!(missing_digest
            .has_reason(ProofpackWriterTargetArtifactRefPolicyReason::MissingManifestSelfDigest));
        assert_eq!(missing_digest.logical_ref(), None);

        assert!(invalid_digest.is_rejected());
        assert!(!invalid_digest.has_missing_precondition());
        assert!(invalid_digest
            .has_reason(ProofpackWriterTargetArtifactRefPolicyReason::InvalidManifestSelfDigest));
        assert_eq!(invalid_digest.manifest_self_digest(), None);
        assert_eq!(invalid_digest.logical_display_ref(), None);
        assert_eq!(invalid_digest.boundary_notes().len(), 1);
    }

    #[test]
    fn proofpack_writer_target_artifact_ref_policy_is_side_effect_free_and_non_authoritative() {
        let proofpack = sample_proofpack();
        let canonical = ProofpackWriterCanonicalArtifactModel::from_proofpack(&proofpack, vec![]);
        let model = ProofpackWriterTargetArtifactRefPolicyModel::from_canonical_artifact_model(
            &canonical,
            vec![],
        );
        let boundary = model.boundary();

        assert!(model.is_evidence_only());
        assert!(boundary.models_target_artifact_ref_policy);
        assert!(boundary.requires_proofpack_id_and_manifest_self_digest);
        assert!(boundary.renders_logical_display_ref);
        assert!(boundary.keeps_target_artifact_ref_separate_from_canonical_bytes);
        assert!(boundary.keeps_target_artifact_ref_separate_from_target_path);
        assert!(boundary.keeps_target_artifact_ref_separate_from_storage_root);
        assert!(!model.reads_filesystem());
        assert!(!model.touches_filesystem());
        assert!(!model.canonicalizes_host_paths());
        assert!(!model.display_ref_is_filesystem_path());
        assert!(!model.writes_proofpack());
        assert!(!model.writes_writer_operation_evidence());
        assert!(!model.writes_indexes_or_latest());
        assert!(!boundary.writes_final_decision);
        assert!(!model.creates_acceptance_claim());
        assert!(!model.requires_runtime_storage());
        assert!(!model.writes_cli_output());
        assert!(!model.writes_schema_files());
        assert!(!model.verifies_referenced_artifacts());
        assert!(!model.uses_indexes_or_latest_as_authority());
        assert!(!model.uses_service_mirror_as_authority());
        assert!(!model.executor_claims_are_proof());
        assert!(!model.can_claim_acceptance_by_itself());
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
        assert!(evidence.target_ref().is_aligned_target_artifact_ref());
        assert!(evidence
            .target_ref()
            .as_str()
            .starts_with("proofpack:proofpack_local_001@sha256:"));
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
        assert!(plan.target_ref().is_aligned_target_artifact_ref());
        assert!(plan
            .target_ref()
            .as_str()
            .starts_with("proofpack:proofpack_local_001@sha256:"));
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
        assert!(plan.target_artifact_ref().is_aligned_target_artifact_ref());
        assert!(!plan.target_artifact_ref().is_path_like_ref());
        assert_eq!(
            plan.target_path_ref().as_str(),
            "future/.punk/proofs/proofpack_local_001.json"
        );
        assert_ne!(
            plan.target_artifact_ref().as_str(),
            plan.target_path_ref().as_str()
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
    fn proofpack_writer_file_io_outcome_model_maps_blocked_plan_without_artifact_availability() {
        let incomplete_proofpack =
            Proofpack::new(
                ProofpackId::new("proofpack_outcome_blocked_001")
                    .expect("proofpack id should be valid"),
                ProofGateDecisionRef::new("decision_local_001")
                    .expect("gate decision ref should be valid"),
                vec![ProofContractRef::new("contract_local_001")
                    .expect("contract ref should be valid")],
                vec![ProofRunReceiptRef::new("receipt_local_001")
                    .expect("run receipt ref should be valid")],
                ProofCreatedAt::new("2026-04-26T15:30:00Z").expect("created_at should be valid"),
                vec![
                    ProofBoundaryNote::new("Incomplete proofpack keeps file IO blocked.")
                        .expect("boundary note should be valid"),
                ],
            )
            .expect("proofpack should be structurally valid");
        let preflight_plan = sample_writer_preflight_plan(&incomplete_proofpack, vec![], vec![]);
        let plan = sample_writer_file_io_plan(&preflight_plan, vec![], vec![]);
        let outcome = ProofpackWriterFileIoOutcomeModel::from_plan_and_observation(
            &plan,
            ProofpackWriterFileIoObservation::target_missing_write_completed(
                ProofpackWriterSideEffectStatus::Completed,
                ProofpackWriterSideEffectStatus::Completed,
                vec![ProofBoundaryNote::new(
                    "Caller observation is ignored for artifact availability while plan is blocked.",
                )
                .expect("boundary note should be valid")],
            ),
        );
        let evidence = outcome
            .to_operation_evidence(
                ProofpackWriterOperationId::new("writer_file_io_outcome_blocked_001")
                    .expect("operation id should be valid"),
                ProofpackWriterAttemptedAt::new("2026-04-26T15:31:00Z")
                    .expect("attempted_at should be valid"),
            )
            .expect("blocked outcome evidence should be derivable");

        assert_eq!(
            outcome.schema_version(),
            PROOFPACK_WRITER_FILE_IO_OUTCOME_MODEL_SCHEMA_VERSION
        );
        assert_eq!(
            outcome.outcome(),
            ProofpackWriterOperationOutcome::PreflightFailed
        );
        assert_eq!(
            outcome.canonical_artifact_status(),
            ProofpackWriterCanonicalArtifactStatus::NotAttempted
        );
        assert_eq!(
            outcome.index_status(),
            ProofpackWriterSideEffectStatus::NotSelected
        );
        assert_eq!(
            outcome.latest_pointer_status(),
            ProofpackWriterSideEffectStatus::NotSelected
        );
        assert!(!outcome.canonical_artifact_available());
        assert_eq!(
            evidence.outcome(),
            ProofpackWriterOperationOutcome::PreflightFailed
        );
        assert!(!evidence.canonical_artifact_available());
        assert!(!evidence.creates_acceptance_claim());
    }

    #[test]
    fn proofpack_writer_file_io_outcome_model_maps_idempotent_match_and_conflict() {
        let plan = sample_writer_file_io_ready_plan();
        let matching = ProofpackWriterFileIoOutcomeModel::from_plan_and_observation(
            &plan,
            ProofpackWriterFileIoObservation::target_exists_matching(vec![ProofBoundaryNote::new(
                "Existing target matched the manifest self-digest.",
            )
            .expect("boundary note should be valid")]),
        );
        let conflict = ProofpackWriterFileIoOutcomeModel::from_plan_and_observation(
            &plan,
            ProofpackWriterFileIoObservation::target_exists_different(vec![
                ProofBoundaryNote::new("Existing target differed from the planned manifest.")
                    .expect("boundary note should be valid"),
            ]),
        );

        assert_eq!(
            matching.outcome(),
            ProofpackWriterOperationOutcome::AlreadyExistsMatching
        );
        assert_eq!(
            matching.operation_kind(),
            ProofpackWriterOperationKind::IdempotencyCheck
        );
        assert_eq!(
            matching.canonical_artifact_status(),
            ProofpackWriterCanonicalArtifactStatus::AlreadyExistsMatching
        );
        assert!(matching.canonical_artifact_available());
        assert!(!matching.has_conflict());

        assert_eq!(
            conflict.outcome(),
            ProofpackWriterOperationOutcome::ConflictExistingDifferent
        );
        assert_eq!(
            conflict.operation_kind(),
            ProofpackWriterOperationKind::ConflictCheck
        );
        assert_eq!(
            conflict.canonical_artifact_status(),
            ProofpackWriterCanonicalArtifactStatus::ConflictExistingDifferent
        );
        assert!(!conflict.canonical_artifact_available());
        assert!(conflict.has_conflict());
    }

    #[test]
    fn proofpack_writer_file_io_outcome_model_maps_write_failure_and_partial_cleanup() {
        let plan = sample_writer_file_io_ready_plan();
        let write_failed = ProofpackWriterFileIoOutcomeModel::from_plan_and_observation(
            &plan,
            ProofpackWriterFileIoObservation::write_failed(vec![ProofBoundaryNote::new(
                "Temp write failed before canonical artifact availability.",
            )
            .expect("boundary note should be valid")]),
        );
        let partial = ProofpackWriterFileIoOutcomeModel::from_plan_and_observation(
            &plan,
            ProofpackWriterFileIoObservation::partial_write_detected(
                ProofpackWriterSideEffectStatus::Failed,
                vec![ProofBoundaryNote::new(
                    "Partial canonical artifact and cleanup failure remain visible.",
                )
                .expect("boundary note should be valid")],
            ),
        );

        assert_eq!(
            write_failed.outcome(),
            ProofpackWriterOperationOutcome::WriteFailed
        );
        assert_eq!(
            write_failed.canonical_artifact_status(),
            ProofpackWriterCanonicalArtifactStatus::WriteFailed
        );
        assert_eq!(
            write_failed.observation().write_result().as_str(),
            "write_failed"
        );
        assert!(!write_failed.canonical_artifact_available());

        assert_eq!(
            partial.outcome(),
            ProofpackWriterOperationOutcome::PartialWriteDetected
        );
        assert_eq!(
            partial.canonical_artifact_status(),
            ProofpackWriterCanonicalArtifactStatus::PartialWriteDetected
        );
        assert!(partial.has_partial_or_cleanup_issue());
        assert!(partial.observation().has_cleanup_failure());
        assert_eq!(
            partial.observation().partial_state().as_str(),
            "ambiguous_canonical_artifact"
        );
        assert!(!partial.canonical_artifact_available());
    }

    #[test]
    fn proofpack_writer_file_io_outcome_model_separates_index_latest_failures() {
        let plan = sample_writer_file_io_ready_plan();
        let written = ProofpackWriterFileIoOutcomeModel::from_plan_and_observation(
            &plan,
            ProofpackWriterFileIoObservation::target_missing_write_completed(
                ProofpackWriterSideEffectStatus::Completed,
                ProofpackWriterSideEffectStatus::Completed,
                vec![ProofBoundaryNote::new(
                    "Canonical artifact write completed before index/latest side effects.",
                )
                .expect("boundary note should be valid")],
            ),
        );
        let index_failed = ProofpackWriterFileIoOutcomeModel::from_plan_and_observation(
            &plan,
            ProofpackWriterFileIoObservation::index_failed_after_available(vec![
                ProofBoundaryNote::new(
                    "Index update failed after canonical artifact availability.",
                )
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

        assert_eq!(written.outcome(), ProofpackWriterOperationOutcome::Written);
        assert_eq!(
            written.canonical_artifact_status(),
            ProofpackWriterCanonicalArtifactStatus::Written
        );
        assert!(written.canonical_artifact_available());
        assert!(!written.has_index_or_latest_pointer_failure());

        assert_eq!(
            index_failed.outcome(),
            ProofpackWriterOperationOutcome::IndexUpdateFailed
        );
        assert_eq!(
            index_failed.operation_kind(),
            ProofpackWriterOperationKind::IndexUpdate
        );
        assert_eq!(
            index_failed.index_status(),
            ProofpackWriterSideEffectStatus::Failed
        );
        assert_eq!(
            index_failed.latest_pointer_status(),
            ProofpackWriterSideEffectStatus::Completed
        );
        assert!(index_failed.canonical_artifact_available());
        assert!(index_failed.has_index_or_latest_pointer_failure());

        assert_eq!(
            latest_failed.outcome(),
            ProofpackWriterOperationOutcome::LatestPointerUpdateFailed
        );
        assert_eq!(
            latest_failed.operation_kind(),
            ProofpackWriterOperationKind::LatestPointerUpdate
        );
        assert_eq!(
            latest_failed.index_status(),
            ProofpackWriterSideEffectStatus::Completed
        );
        assert_eq!(
            latest_failed.latest_pointer_status(),
            ProofpackWriterSideEffectStatus::Failed
        );
        assert!(latest_failed.canonical_artifact_available());
        assert!(latest_failed.has_index_or_latest_pointer_failure());
    }

    #[test]
    fn proofpack_writer_file_io_outcome_model_is_evidence_only_and_setup_neutral() {
        let plan = sample_writer_file_io_ready_plan();
        let planned = ProofpackWriterFileIoOutcomeModel::from_plan_and_observation(
            &plan,
            ProofpackWriterFileIoObservation::planned_only(),
        );
        let aborted = ProofpackWriterFileIoOutcomeModel::from_plan_and_observation(
            &plan,
            ProofpackWriterFileIoObservation::aborted(vec![ProofBoundaryNote::new(
                "Operation was aborted before canonical artifact write.",
            )
            .expect("boundary note should be valid")]),
        );
        let evidence = aborted
            .to_operation_evidence(
                ProofpackWriterOperationId::new("writer_file_io_outcome_aborted_001")
                    .expect("operation id should be valid"),
                ProofpackWriterAttemptedAt::new("2026-04-26T15:32:00Z")
                    .expect("attempted_at should be valid"),
            )
            .expect("aborted operation evidence should be derivable");
        let boundary = planned.boundary();

        assert_eq!(
            planned.outcome(),
            ProofpackWriterOperationOutcome::PlannedOnly
        );
        assert_eq!(
            planned.operation_kind(),
            ProofpackWriterOperationKind::PlannedOnly
        );
        assert_eq!(aborted.outcome(), ProofpackWriterOperationOutcome::Aborted);
        assert_eq!(
            aborted.operation_kind(),
            ProofpackWriterOperationKind::Abort
        );
        assert_eq!(evidence.outcome(), ProofpackWriterOperationOutcome::Aborted);
        assert!(!aborted.canonical_artifact_available());
        assert!(aborted.observation().is_aborted());

        assert!(planned.is_evidence_only());
        assert!(boundary.models_writer_file_io_outcome);
        assert!(boundary.accepts_explicit_observations);
        assert!(boundary.maps_observations_to_operation_evidence);
        assert!(boundary.separates_observation_from_artifact_availability);
        assert!(boundary.preserves_partial_cleanup_visibility);
        assert!(!boundary.reads_filesystem);
        assert!(!planned.touches_filesystem());
        assert!(!planned.writes_proofpack());
        assert!(!planned.writes_writer_operation_evidence());
        assert!(!boundary.writes_final_decision);
        assert!(!planned.creates_acceptance_claim());
        assert!(!planned.requires_runtime_storage());
        assert!(!planned.writes_cli_output());
        assert!(!planned.writes_schema_files());
        assert!(!planned.target_path_is_authority());
        assert!(!planned.index_latest_are_canonical());
        assert!(!evidence.creates_acceptance_claim());
    }

    #[test]
    fn proofpack_writer_file_io_observation_vocabulary_is_stable() {
        assert_eq!(
            ProofpackWriterObservedTargetState::Missing.as_str(),
            "missing"
        );
        assert_eq!(
            ProofpackWriterObservedTargetState::ExistsMatching.as_str(),
            "exists_matching"
        );
        assert_eq!(
            ProofpackWriterIdempotencyObservation::Different.as_str(),
            "different"
        );
        assert_eq!(
            ProofpackWriterObservedWriteResult::Written.as_str(),
            "written"
        );
        assert_eq!(
            ProofpackWriterObservedPartialState::CleanupIncomplete.as_str(),
            "cleanup_incomplete"
        );
        assert_eq!(
            ProofpackWriterAbortState::AbortedAfterPartial.as_str(),
            "aborted_after_partial"
        );
    }

    #[test]
    fn proofpack_writer_file_io_error_reason_vocabulary_is_stable() {
        assert_eq!(
            PROOFPACK_WRITER_FILE_IO_ERROR_REASON_MODEL_SCHEMA_VERSION,
            "punk.proofpack.writer_file_io_error_reason_model.v0.1"
        );
        assert_eq!(
            ProofpackWriterFileIoErrorReason::StorageRootMissing.as_str(),
            "storage_root_missing"
        );
        assert_eq!(
            ProofpackWriterFileIoErrorReason::TargetPathEscapesStorageRoot.as_str(),
            "target_path_escapes_storage_root"
        );
        assert_eq!(
            ProofpackWriterFileIoErrorReason::ExistingTargetDifferent.as_str(),
            "existing_target_different"
        );
        assert_eq!(
            ProofpackWriterFileIoErrorReason::FlushOrSyncFailed.as_str(),
            "flush_or_sync_failed"
        );
        assert_eq!(
            ProofpackWriterFileIoErrorReason::OperationEvidencePersistenceFailed.as_str(),
            "operation_evidence_persistence_failed"
        );
        assert_eq!(
            ProofpackWriterFileIoErrorReason::OperationAborted.as_str(),
            "operation_aborted"
        );
        assert_eq!(
            ProofpackWriterFileIoErrorReason::IndexUpdateFailed.surface(),
            ProofpackWriterFileIoDiagnosticSurface::Index
        );
        assert_eq!(
            ProofpackWriterFileIoDiagnosticSurface::OperationEvidencePersistence.as_str(),
            "operation_evidence_persistence"
        );
        assert!(ProofpackWriterFileIoErrorReason::AtomicMoveFailed.is_write_reason());
        assert!(
            ProofpackWriterFileIoErrorReason::LatestPointerUpdateFailed.is_index_or_latest_reason()
        );
        assert!(ProofpackWriterFileIoErrorReason::CleanupFailed.is_partial_or_cleanup_reason());
        assert!(
            ProofpackWriterFileIoErrorReason::OperationEvidencePersistenceFailed
                .is_operation_evidence_persistence_reason()
        );
        assert!(ProofpackWriterFileIoErrorReason::OperationAborted.is_abort_reason());
        assert!(
            ProofpackWriterFileIoErrorReason::StorageRootDisallowed.is_storage_or_target_reason()
        );
    }

    #[test]
    fn proofpack_writer_file_io_error_reason_model_attaches_to_outcome_without_authority() {
        let plan = sample_writer_file_io_ready_plan();
        let outcome = ProofpackWriterFileIoOutcomeModel::from_plan_and_observation(
            &plan,
            ProofpackWriterFileIoObservation::write_failed(vec![ProofBoundaryNote::new(
                "Temp write failed before canonical artifact availability.",
            )
            .expect("boundary note should be valid")]),
        );
        let diagnostic = ProofpackWriterFileIoDiagnostic::for_reason(
            ProofpackWriterFileIoErrorReason::TempWriteFailed,
        )
        .with_target_path_ref(
            ProofpackWriterTargetPathRef::new("future/.punk/proofs/proofpack_local_001.json")
                .expect("target path ref should be valid"),
        )
        .with_diagnostic_path_ref(
            ProofpackWriterDiagnosticPathRef::new(
                "/diagnostic-only/not-canonical/proofpack_local_001.tmp",
            )
            .expect("diagnostic path ref should be valid"),
        );
        let model = ProofpackWriterFileIoErrorReasonModel::from_outcome_and_diagnostics(
            &outcome,
            vec![diagnostic.clone()],
            vec![ProofBoundaryNote::new(
                "Diagnostic paths are debug evidence only, not proof authority.",
            )
            .expect("boundary note should be valid")],
        );
        let boundary = model.boundary();

        assert_eq!(
            model.schema_version(),
            PROOFPACK_WRITER_FILE_IO_ERROR_REASON_MODEL_SCHEMA_VERSION
        );
        assert_eq!(model.proofpack_id(), outcome.proofpack_id());
        assert_eq!(model.target_artifact_ref(), outcome.target_artifact_ref());
        assert_eq!(model.target_path_ref(), outcome.target_path_ref());
        assert_eq!(
            model.outcome(),
            ProofpackWriterOperationOutcome::WriteFailed
        );
        assert_eq!(
            model.canonical_artifact_status(),
            ProofpackWriterCanonicalArtifactStatus::WriteFailed
        );
        assert_eq!(model.diagnostics().len(), 1);
        assert_eq!(model.diagnostics()[0].reason_code(), "temp_write_failed");
        assert_eq!(model.diagnostics()[0].surface().as_str(), "temp_write");
        assert!(model.has_reason(ProofpackWriterFileIoErrorReason::TempWriteFailed));
        assert!(model.has_write_reason());
        assert!(model.has_diagnostic_path_refs());
        assert!(!diagnostic.target_path_is_authority());
        assert!(!diagnostic.diagnostic_path_is_authority());
        assert!(!diagnostic.source_is_proof_authority());

        assert!(model.is_evidence_only());
        assert!(boundary.models_writer_file_io_error_reasons);
        assert!(boundary.stable_reason_codes);
        assert!(boundary.attaches_to_file_io_outcomes);
        assert!(!model.reads_filesystem());
        assert!(!model.touches_filesystem());
        assert!(!model.writes_proofpack());
        assert!(!model.writes_writer_operation_evidence());
        assert!(!boundary.writes_final_decision);
        assert!(!model.creates_acceptance_claim());
        assert!(!model.requires_runtime_storage());
        assert!(!model.writes_cli_output());
        assert!(!model.writes_schema_files());
        assert!(!model.target_path_is_authority());
        assert!(!model.diagnostic_paths_are_authority());
        assert!(!model.index_latest_are_canonical());
        assert!(!model.can_claim_acceptance_by_itself());
    }

    #[test]
    fn proofpack_writer_file_io_error_reason_model_separates_side_effect_reasons() {
        let plan = sample_writer_file_io_ready_plan();
        let outcome = ProofpackWriterFileIoOutcomeModel::from_plan_and_observation(
            &plan,
            ProofpackWriterFileIoObservation::partial_write_detected(
                ProofpackWriterSideEffectStatus::Failed,
                vec![
                    ProofBoundaryNote::new("Partial write and cleanup failure remain visible.")
                        .expect("boundary note should be valid"),
                ],
            ),
        );
        let model = ProofpackWriterFileIoErrorReasonModel::from_outcome_and_diagnostics(
            &outcome,
            vec![
                ProofpackWriterFileIoDiagnostic::for_reason(
                    ProofpackWriterFileIoErrorReason::PartialCanonicalArtifactAmbiguous,
                ),
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
            ],
            vec![ProofBoundaryNote::new(
                "Side-effect reason codes stay separate from canonical artifact authority.",
            )
            .expect("boundary note should be valid")],
        );

        assert_eq!(
            model.outcome(),
            ProofpackWriterOperationOutcome::PartialWriteDetected
        );
        assert!(model.has_partial_or_cleanup_reason());
        assert!(model.has_index_or_latest_reason());
        assert!(model.has_operation_evidence_persistence_reason());
        assert!(model.has_abort_reason());
        assert!(model
            .reasons()
            .contains(&ProofpackWriterFileIoErrorReason::IndexUpdateFailed));
        assert!(model
            .reasons()
            .contains(&ProofpackWriterFileIoErrorReason::LatestPointerUpdateFailed));
        assert!(model
            .reasons()
            .contains(&ProofpackWriterFileIoErrorReason::CleanupFailed));
        assert!(!model.index_latest_are_canonical());
        assert!(!model.creates_acceptance_claim());
    }

    #[test]
    fn proofpack_writer_file_io_error_reason_model_keeps_executor_claims_non_proof() {
        let plan = sample_writer_file_io_ready_plan();
        let outcome = ProofpackWriterFileIoOutcomeModel::from_plan_and_observation(
            &plan,
            ProofpackWriterFileIoObservation::target_exists_different(vec![
                ProofBoundaryNote::new("Existing target differed from the planned manifest.")
                    .expect("boundary note should be valid"),
            ]),
        );
        let executor_claim = ProofpackWriterFileIoDiagnostic::for_reason(
            ProofpackWriterFileIoErrorReason::ExistingTargetMatching,
        )
        .with_source(ProofpackWriterFileIoDiagnosticSource::ExecutorClaim)
        .with_boundary_notes(vec![ProofBoundaryNote::new(
            "Executor claims are diagnostic input only and never proof authority.",
        )
        .expect("boundary note should be valid")]);
        let model = ProofpackWriterFileIoErrorReasonModel::from_outcome_and_diagnostics(
            &outcome,
            vec![executor_claim],
            vec![],
        );

        assert_eq!(
            model.outcome(),
            ProofpackWriterOperationOutcome::ConflictExistingDifferent
        );
        assert!(model.has_executor_claims());
        assert!(!model.executor_claims_are_proof());
        assert_eq!(model.diagnostics()[0].source().as_str(), "executor_claim");
        assert!(!model.diagnostics()[0].source_is_proof_authority());
        assert_eq!(model.boundary_notes().len(), outcome.boundary_notes().len());
        assert!(!model.can_claim_acceptance_by_itself());
    }

    #[test]
    fn proofpack_writer_target_path_policy_vocabulary_is_stable() {
        assert_eq!(
            PROOFPACK_WRITER_TARGET_PATH_POLICY_MODEL_SCHEMA_VERSION,
            "punk.proofpack.writer_target_path_policy_model.v0.1"
        );
        assert_eq!(
            ProofpackWriterTargetPathPolicyStatus::Accepted.as_str(),
            "accepted"
        );
        assert_eq!(
            ProofpackWriterTargetPathPolicyStatus::Rejected.as_str(),
            "rejected"
        );
        assert_eq!(
            ProofpackWriterTargetPathPolicyReason::AbsolutePath.as_str(),
            "absolute_path"
        );
        assert_eq!(
            ProofpackWriterTargetPathPolicyReason::HomeRelativePath.as_str(),
            "home_relative_path"
        );
        assert_eq!(
            ProofpackWriterTargetPathPolicyReason::UrlRef.as_str(),
            "url_ref"
        );
        assert_eq!(
            ProofpackWriterTargetPathPolicyReason::PathTraversal.as_str(),
            "path_traversal"
        );
        assert_eq!(
            ProofpackWriterTargetPathPolicyReason::AmbiguousDotSegment.as_str(),
            "ambiguous_dot_segment"
        );
        assert_eq!(
            ProofpackWriterTargetPathPolicyReason::EmptySegment.as_str(),
            "empty_segment"
        );
        assert_eq!(
            ProofpackWriterTargetPathPolicyReason::UnsupportedBackslash.as_str(),
            "unsupported_backslash"
        );
        assert_eq!(
            ProofpackWriterTargetPathPolicyReason::StorageRootEscape.as_str(),
            "storage_root_escape"
        );
        assert_eq!(
            ProofpackWriterTargetPathPolicyReason::StorageRootEscape.file_io_error_reason(),
            ProofpackWriterFileIoErrorReason::TargetPathEscapesStorageRoot
        );
        assert_eq!(
            ProofpackWriterTargetPathPolicyReason::PathTraversal.file_io_error_reason(),
            ProofpackWriterFileIoErrorReason::TargetPathInvalid
        );
    }

    #[test]
    fn proofpack_writer_target_path_policy_accepts_repo_runtime_style_refs() {
        let plan = sample_writer_file_io_ready_plan();
        let model = ProofpackWriterTargetPathPolicyModel::from_plan(
            &plan,
            vec![ProofBoundaryNote::new(
                "Target path policy classifies explicit refs without filesystem access.",
            )
            .expect("boundary note should be valid")],
        );
        let boundary = model.boundary();

        assert_eq!(
            model.schema_version(),
            PROOFPACK_WRITER_TARGET_PATH_POLICY_MODEL_SCHEMA_VERSION
        );
        assert_eq!(
            model.status(),
            ProofpackWriterTargetPathPolicyStatus::Accepted
        );
        assert!(model.is_accepted());
        assert!(!model.is_rejected());
        assert_eq!(model.reasons().len(), 0);
        assert_eq!(model.diagnostics().len(), 0);
        assert_eq!(model.storage_root_ref(), plan.storage_root_ref());
        assert_eq!(model.target_artifact_ref(), plan.target_artifact_ref());
        assert_eq!(model.target_path_ref(), plan.target_path_ref());
        assert_ne!(
            model.storage_root_ref().as_str(),
            model.target_path_ref().as_str()
        );
        assert!(model.target_artifact_ref().is_aligned_target_artifact_ref());
        assert!(!model.target_artifact_ref().is_path_like_ref());
        assert_eq!(
            model.target_path_ref().as_str(),
            "future/.punk/proofs/proofpack_local_001.json"
        );
        assert_ne!(
            model.target_artifact_ref().as_str(),
            model.target_path_ref().as_str()
        );

        assert!(model.is_evidence_only());
        assert!(boundary.models_target_path_policy);
        assert!(boundary.classifies_explicit_target_path_refs);
        assert!(boundary.keeps_storage_root_target_ref_and_path_separate);
        assert!(boundary.maps_to_file_io_diagnostics);
        assert!(!model.reads_filesystem());
        assert!(!model.touches_filesystem());
        assert!(!model.canonicalizes_host_paths());
        assert!(!model.target_path_is_authority());
        assert!(!model.storage_root_ref_is_authority());
        assert!(!model.derives_from_current_working_directory());
        assert!(!model.uses_indexes_or_latest_as_authority());
    }

    #[test]
    fn proofpack_writer_target_path_policy_rejects_path_injection_and_escape_cases() {
        let storage_root_ref = ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
            .expect("storage root ref should be valid");
        let proofpack = sample_proofpack();
        let target_ref = sample_writer_target_artifact_ref(&proofpack);
        let model_for = |target_path: &str| {
            ProofpackWriterTargetPathPolicyModel::evaluate(
                storage_root_ref.clone(),
                target_ref.clone(),
                ProofpackWriterTargetPathRef::new(target_path)
                    .expect("target path ref should be non-empty"),
                vec![],
            )
        };

        let absolute = model_for("/tmp/proofpack.json");
        let windows_absolute = model_for("C:\\tmp\\proofpack.json");
        let home = model_for("~/proofpack.json");
        let url = model_for("file:///tmp/proofpack.json");
        let traversal = model_for("future/.punk/../proofs/proofpack.json");
        let dot = model_for("future/./proofpack.json");
        let empty_segment = model_for("future//proofpack.json");
        let backslash = model_for("future\\.punk\\proofpack.json");

        assert!(absolute.is_rejected());
        assert!(absolute.has_reason(ProofpackWriterTargetPathPolicyReason::AbsolutePath));
        assert!(absolute.has_reason(ProofpackWriterTargetPathPolicyReason::StorageRootEscape));
        assert!(absolute.has_storage_root_escape());
        assert!(absolute
            .diagnostic_reason_codes()
            .contains(&"target_path_escapes_storage_root"));

        assert!(windows_absolute.has_reason(ProofpackWriterTargetPathPolicyReason::AbsolutePath));
        assert!(windows_absolute
            .has_reason(ProofpackWriterTargetPathPolicyReason::UnsupportedBackslash));
        assert!(windows_absolute.has_storage_root_escape());

        assert!(home.has_reason(ProofpackWriterTargetPathPolicyReason::HomeRelativePath));
        assert!(home.has_storage_root_escape());
        assert!(url.has_reason(ProofpackWriterTargetPathPolicyReason::UrlRef));
        assert!(url.has_storage_root_escape());
        assert!(traversal.has_reason(ProofpackWriterTargetPathPolicyReason::PathTraversal));
        assert!(traversal.has_storage_root_escape());
        assert!(dot.has_reason(ProofpackWriterTargetPathPolicyReason::AmbiguousDotSegment));
        assert!(empty_segment.has_reason(ProofpackWriterTargetPathPolicyReason::EmptySegment));
        assert!(backslash.has_reason(ProofpackWriterTargetPathPolicyReason::UnsupportedBackslash));
        assert!(backslash.diagnostics().iter().all(|diagnostic| {
            diagnostic.surface() == ProofpackWriterFileIoDiagnosticSurface::TargetPath
                && !diagnostic.target_path_is_authority()
                && !diagnostic.source_is_proof_authority()
        }));
        assert!(traversal.diagnostics().iter().any(|diagnostic| {
            diagnostic.reason() == ProofpackWriterFileIoErrorReason::TargetPathEscapesStorageRoot
        }));
    }

    #[test]
    fn proofpack_writer_target_path_policy_is_setup_neutral_and_side_effect_free() {
        let proofpack = sample_proofpack();
        let model = ProofpackWriterTargetPathPolicyModel::evaluate(
            ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
                .expect("storage root ref should be valid"),
            sample_writer_target_artifact_ref(&proofpack),
            ProofpackWriterTargetPathRef::new("future/./proofpack_local_001.json")
                .expect("target path ref should be valid"),
            vec![],
        );
        let boundary = model.boundary();

        assert_eq!(model.status().as_str(), "rejected");
        assert!(model.has_reason(ProofpackWriterTargetPathPolicyReason::AmbiguousDotSegment));
        assert_eq!(model.diagnostics().len(), 1);
        assert_eq!(model.diagnostics()[0].reason_code(), "target_path_invalid");
        assert_eq!(
            model.diagnostics()[0].source(),
            ProofpackWriterFileIoDiagnosticSource::WriterDiagnostic
        );
        assert_eq!(model.boundary_notes().len(), 1);
        assert!(model.is_evidence_only());
        assert!(!model.reads_filesystem());
        assert!(!model.touches_filesystem());
        assert!(!model.canonicalizes_host_paths());
        assert!(!model.writes_proofpack());
        assert!(!model.writes_writer_operation_evidence());
        assert!(!boundary.writes_final_decision);
        assert!(!model.creates_acceptance_claim());
        assert!(!model.requires_runtime_storage());
        assert!(!model.writes_cli_output());
        assert!(!model.writes_schema_files());
        assert!(!model.target_path_is_authority());
        assert!(!model.storage_root_ref_is_authority());
        assert!(!model.derives_from_current_working_directory());
        assert!(!model.uses_indexes_or_latest_as_authority());
    }

    #[test]
    fn proofpack_writer_preflight_integration_model_ready_composes_explicit_inputs() {
        let proofpack = sample_proofpack();
        let canonical = ProofpackWriterCanonicalArtifactModel::from_proofpack(
            &proofpack,
            vec![
                ProofBoundaryNote::new("Canonical artifact model is explicit preflight input.")
                    .expect("boundary note should be valid"),
            ],
        );
        let target_ref_policy =
            ProofpackWriterTargetArtifactRefPolicyModel::from_canonical_artifact_model(
                &canonical,
                vec![ProofBoundaryNote::new(
                    "Target artifact ref policy is explicit preflight input.",
                )
                .expect("boundary note should be valid")],
            );
        let preflight_plan = ProofpackWriterPreflightPlan::new(
            &proofpack,
            ProofpackWriterTargetRef::from_target_artifact_ref_policy_model(&target_ref_policy)
                .expect("target artifact ref policy should derive logical ref"),
            vec![ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite],
            vec![
                ProofBoundaryNote::new("Preflight plan is explicit and side-effect-free.")
                    .expect("boundary note should be valid"),
            ],
        );
        let file_io_plan = ProofpackWriterFileIoPlan::new(
            &preflight_plan,
            ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
                .expect("storage root ref should be valid"),
            ProofpackWriterTargetPathRef::new("future/.punk/proofs/proofpack_local_001.json")
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
                "File IO plan is explicit and does not touch the filesystem.",
            )
            .expect("boundary note should be valid")],
        );
        let target_path_policy = ProofpackWriterTargetPathPolicyModel::from_plan(
            &file_io_plan,
            vec![
                ProofBoundaryNote::new("Target path policy classifies explicit refs only.")
                    .expect("boundary note should be valid"),
            ],
        );

        let integration = ProofpackWriterPreflightIntegrationModel::evaluate(
            &proofpack,
            Some(&canonical),
            Some(&target_ref_policy),
            Some(&preflight_plan),
            Some(&file_io_plan),
            Some(&target_path_policy),
            vec![
                ProofBoundaryNote::new("Integrated preflight is evidence-only and writer-ready.")
                    .expect("boundary note should be valid"),
            ],
        );

        assert_eq!(
            integration.schema_version(),
            PROOFPACK_WRITER_PREFLIGHT_INTEGRATION_MODEL_SCHEMA_VERSION
        );
        assert_eq!(
            integration.status(),
            ProofpackWriterPreflightIntegrationStatus::Ready
        );
        assert!(integration.writer_selected());
        assert!(integration.is_writer_ready());
        assert!(!integration.has_blockers());
        assert_eq!(integration.proofpack_id(), proofpack.id());
        assert_eq!(
            integration.canonical_layout(),
            Some(ProofpackWriterCanonicalArtifactLayout::ManifestOnlyJson)
        );
        assert_eq!(
            integration.manifest_self_digest(),
            Some(canonical.manifest_self_digest())
        );
        assert_eq!(
            integration.target_artifact_ref(),
            Some(file_io_plan.target_artifact_ref())
        );
        assert_eq!(
            integration.storage_root_ref(),
            Some(file_io_plan.storage_root_ref())
        );
        assert_eq!(
            integration.target_path_ref(),
            Some(file_io_plan.target_path_ref())
        );
        assert_eq!(
            integration.target_path_policy_status(),
            Some(ProofpackWriterTargetPathPolicyStatus::Accepted)
        );
        assert_eq!(
            integration.write_policy(),
            Some(ProofpackWriterWritePolicy::IdempotentIfMatching)
        );
        assert_eq!(
            integration.idempotency_basis(),
            Some(ProofpackWriterIdempotencyBasis::ManifestSelfDigest)
        );
        assert_eq!(
            integration.temp_atomic_policy(),
            Some(ProofpackWriterTempAtomicPolicy::AtomicSiblingTemp)
        );
        assert!(integration
            .planned_side_effects()
            .contains(&ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite));
        assert!(integration
            .failure_visibility()
            .contains(&ProofpackWriterFileIoFailureVisibility::ExistingTargetDifferent));
        assert_eq!(integration.diagnostics().len(), 0);
        assert!(integration.refs_are_separated());
        assert_eq!(
            integration.operation_outcome(),
            ProofpackWriterOperationOutcome::PlannedOnly
        );
    }

    #[test]
    fn proofpack_writer_preflight_integration_model_blocks_missing_or_rejected_inputs() {
        let proofpack = sample_proofpack_without_digests();
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
            vec![],
            vec![],
        );
        let file_io_plan = ProofpackWriterFileIoPlan::new(
            &preflight_plan,
            ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
                .expect("storage root ref should be valid"),
            ProofpackWriterTargetPathRef::new("/tmp/proofpack_local_001.json")
                .expect("target path ref should be valid"),
            ProofpackWriterWritePolicy::IdempotentIfMatching,
            ProofpackWriterIdempotencyBasis::ManifestSelfDigest,
            ProofpackWriterTempAtomicPolicy::AtomicSiblingTemp,
            vec![],
            vec![],
        );
        let target_path_policy =
            ProofpackWriterTargetPathPolicyModel::from_plan(&file_io_plan, vec![]);

        let integration = ProofpackWriterPreflightIntegrationModel::evaluate(
            &proofpack,
            Some(&canonical),
            Some(&target_ref_policy),
            Some(&preflight_plan),
            Some(&file_io_plan),
            Some(&target_path_policy),
            vec![],
        );

        assert_eq!(
            integration.status(),
            ProofpackWriterPreflightIntegrationStatus::Blocked
        );
        assert!(integration.writer_selected());
        assert!(integration.is_blocked());
        assert!(integration.has_blockers());
        assert!(integration.has_blocker(
            ProofpackWriterPreflightIntegrationBlocker::PreflightPlanMissingPreconditions
        ));
        assert!(
            integration.has_blocker(ProofpackWriterPreflightIntegrationBlocker::FileIoPlanBlocked)
        );
        assert!(integration
            .has_blocker(ProofpackWriterPreflightIntegrationBlocker::RejectedTargetPathPolicy));
        assert!(integration
            .has_blocker(ProofpackWriterPreflightIntegrationBlocker::MissingBoundaryNotes));
        assert_eq!(
            integration.operation_outcome(),
            ProofpackWriterOperationOutcome::PreflightFailed
        );
        assert!(integration.diagnostics().iter().any(|diagnostic| {
            diagnostic.reason() == ProofpackWriterFileIoErrorReason::TargetPathEscapesStorageRoot
        }));
        assert!(!integration.target_path_is_authority());
        assert!(!integration.executor_claims_are_proof());
    }

    #[test]
    fn proofpack_writer_preflight_integration_model_not_selected_is_evidence_only() {
        let proofpack = sample_proofpack();
        let canonical = ProofpackWriterCanonicalArtifactModel::from_proofpack(&proofpack, vec![]);
        let target_ref_policy =
            ProofpackWriterTargetArtifactRefPolicyModel::from_canonical_artifact_model(
                &canonical,
                vec![],
            );
        let integration = ProofpackWriterPreflightIntegrationModel::not_selected(
            &proofpack,
            Some(&canonical),
            Some(&target_ref_policy),
            vec![
                ProofBoundaryNote::new("Writer/storage behavior is intentionally not selected.")
                    .expect("boundary note should be valid"),
            ],
        );
        let boundary = integration.boundary();

        assert_eq!(
            ProofpackWriterPreflightIntegrationStatus::Ready.as_str(),
            "ready"
        );
        assert_eq!(
            ProofpackWriterPreflightIntegrationStatus::Blocked.as_str(),
            "blocked"
        );
        assert_eq!(
            ProofpackWriterPreflightIntegrationStatus::NotSelected.as_str(),
            "not_selected"
        );
        assert_eq!(
            ProofpackWriterPreflightIntegrationBlocker::MissingFileIoPlan.as_str(),
            "missing_file_io_plan"
        );
        assert_eq!(
            integration.status(),
            ProofpackWriterPreflightIntegrationStatus::NotSelected
        );
        assert!(!integration.writer_selected());
        assert!(integration.is_not_selected());
        assert!(!integration.has_blockers());
        assert_eq!(
            integration.operation_outcome(),
            ProofpackWriterOperationOutcome::PlannedOnly
        );
        assert!(integration.is_evidence_only());
        assert!(boundary.models_preflight_integration);
        assert!(boundary.composes_explicit_model_inputs);
        assert!(boundary.keeps_storage_root_target_artifact_and_path_separate);
        assert!(boundary.blockers_fail_closed);
        assert!(boundary.statuses_are_evidence_only);
        assert!(boundary.setup_neutral);
        assert!(!integration.reads_filesystem());
        assert!(!integration.touches_filesystem());
        assert!(!integration.writes_proofpack());
        assert!(!integration.writes_punk_proofs());
        assert!(!integration.writes_writer_operation_evidence());
        assert!(!integration.persists_operation_evidence());
        assert!(!integration.writes_indexes_or_latest());
        assert!(!integration.requires_runtime_storage());
        assert!(!integration.writes_cli_output());
        assert!(!integration.writes_schema_files());
        assert!(!integration.verifies_referenced_artifacts());
        assert!(!integration.creates_acceptance_claim());
        assert!(!integration.can_claim_acceptance_by_itself());
        assert!(!integration.storage_root_ref_is_authority());
    }

    #[test]
    fn proofpack_writer_active_behavior_model_ready_planned_requires_ready_preflight() {
        let preflight = sample_writer_preflight_integration_ready_model();
        let model = ProofpackWriterActiveBehaviorModel::planned_only(
            &preflight,
            vec![ProofBoundaryNote::new(
                "Active behavior model is side-effect-free planning evidence.",
            )
            .expect("boundary note should be valid")],
        );
        let boundary = model.boundary();

        assert_eq!(
            model.schema_version(),
            PROOFPACK_WRITER_ACTIVE_BEHAVIOR_MODEL_SCHEMA_VERSION
        );
        assert_eq!(
            model.preflight_status(),
            ProofpackWriterPreflightIntegrationStatus::Ready
        );
        assert_eq!(
            model.outcome(),
            ProofpackWriterOperationOutcome::PlannedOnly
        );
        assert!(model.is_ready_planned());
        assert!(model.refs_are_separated());
        assert_eq!(model.proofpack_id().as_str(), "proofpack_local_001");
        assert!(model.target_artifact_ref().is_some());
        assert!(model.storage_root_ref().is_some());
        assert!(model.target_path_ref().is_some());
        assert!(model
            .selected_side_effects()
            .contains(&ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite));
        assert!(model.attempted_side_effects().is_empty());
        assert!(model.completed_side_effects().is_empty());
        assert!(model.failed_side_effects().is_empty());
        assert_eq!(
            model.operation_evidence_persistence_status(),
            ProofpackWriterSideEffectStatus::NotSelected
        );
        assert_eq!(
            model.canonical_artifact_status(),
            ProofpackWriterCanonicalArtifactStatus::NotAttempted
        );
        assert!(!model.canonical_artifact_available());

        assert!(model.is_evidence_only());
        assert!(boundary.models_active_writer_behavior);
        assert!(boundary.requires_ready_preflight);
        assert!(boundary.accepts_explicit_observations);
        assert!(boundary.models_selected_attempted_completed_failed_side_effects);
        assert!(boundary.keeps_storage_root_target_artifact_and_path_separate);
        assert!(boundary.failures_remain_visible);
        assert!(boundary.operation_evidence_is_non_authoritative);
        assert!(boundary.setup_neutral);
        assert!(!model.reads_filesystem());
        assert!(!model.touches_filesystem());
        assert!(!boundary.canonicalizes_host_paths);
        assert!(!model.writes_proofpack());
        assert!(!model.writes_punk_proofs());
        assert!(!model.writes_writer_operation_evidence());
        assert!(!model.persists_operation_evidence());
        assert!(!model.writes_indexes_or_latest());
        assert!(!boundary.writes_final_decision);
        assert!(!model.creates_acceptance_claim());
        assert!(!model.requires_runtime_storage());
        assert!(!model.writes_cli_output());
        assert!(!model.writes_schema_files());
        assert!(!model.verifies_referenced_artifacts());
        assert!(!boundary.uses_indexes_or_latest_as_authority);
        assert!(!boundary.uses_service_mirror_as_authority);
        assert!(!model.executor_claims_are_proof());
        assert!(!model.target_path_is_authority());
        assert!(!model.storage_root_ref_is_authority());
        assert!(!model.index_latest_are_canonical());
        assert!(!model.can_claim_acceptance_by_itself());
    }

    #[test]
    fn proofpack_writer_active_behavior_model_fails_closed_before_ready_preflight() {
        let proofpack = sample_proofpack_without_digests();
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
            vec![],
            vec![],
        );
        let file_io_plan = ProofpackWriterFileIoPlan::new(
            &preflight_plan,
            ProofpackWriterStorageRootRef::new("repo_runtime_proofs_root")
                .expect("storage root ref should be valid"),
            ProofpackWriterTargetPathRef::new("/tmp/proofpack_local_001.json")
                .expect("target path ref should be valid"),
            ProofpackWriterWritePolicy::IdempotentIfMatching,
            ProofpackWriterIdempotencyBasis::ManifestSelfDigest,
            ProofpackWriterTempAtomicPolicy::AtomicSiblingTemp,
            vec![],
            vec![],
        );
        let target_path_policy =
            ProofpackWriterTargetPathPolicyModel::from_plan(&file_io_plan, vec![]);
        let blocked_preflight = ProofpackWriterPreflightIntegrationModel::evaluate(
            &proofpack,
            Some(&canonical),
            Some(&target_ref_policy),
            Some(&preflight_plan),
            Some(&file_io_plan),
            Some(&target_path_policy),
            vec![],
        );
        let blocked_model = ProofpackWriterActiveBehaviorModel::from_preflight_and_observation(
            &blocked_preflight,
            Some(
                ProofpackWriterFileIoObservation::target_missing_write_completed(
                    ProofpackWriterSideEffectStatus::Completed,
                    ProofpackWriterSideEffectStatus::Completed,
                    vec![ProofBoundaryNote::new(
                        "Caller observation must not override blocked preflight.",
                    )
                    .expect("boundary note should be valid")],
                ),
            ),
            ProofpackWriterSideEffectStatus::NotSelected,
            vec![],
        );
        let ready_proofpack = sample_proofpack();
        let not_selected = ProofpackWriterPreflightIntegrationModel::not_selected(
            &ready_proofpack,
            None,
            None,
            vec![ProofBoundaryNote::new("Writer behavior was not selected.")
                .expect("boundary note should be valid")],
        );
        let not_selected_model = ProofpackWriterActiveBehaviorModel::planned_only(
            &not_selected,
            vec![
                ProofBoundaryNote::new("No active writer behavior selected.")
                    .expect("boundary note should be valid"),
            ],
        );

        assert_eq!(
            blocked_model.preflight_status(),
            ProofpackWriterPreflightIntegrationStatus::Blocked
        );
        assert!(blocked_model.preflight_failed());
        assert_eq!(
            blocked_model.outcome(),
            ProofpackWriterOperationOutcome::PreflightFailed
        );
        assert_eq!(
            blocked_model.canonical_artifact_status(),
            ProofpackWriterCanonicalArtifactStatus::NotAttempted
        );
        assert!(blocked_model.attempted_side_effects().is_empty());
        assert!(blocked_model.completed_side_effects().is_empty());
        assert!(blocked_model.failed_side_effects().is_empty());
        assert!(!blocked_model.canonical_artifact_available());
        assert!(!blocked_model.writes_proofpack());
        assert!(!blocked_model.touches_filesystem());

        assert_eq!(
            not_selected_model.preflight_status(),
            ProofpackWriterPreflightIntegrationStatus::NotSelected
        );
        assert!(not_selected_model.preflight_failed());
        assert_eq!(
            not_selected_model.outcome(),
            ProofpackWriterOperationOutcome::PreflightFailed
        );
        assert!(not_selected_model.selected_side_effects().is_empty());
        assert!(not_selected_model.attempted_side_effects().is_empty());
        assert!(!not_selected_model.writes_proofpack());
    }

    #[test]
    fn proofpack_writer_active_behavior_model_maps_idempotency_conflict_and_failures() {
        let preflight = sample_writer_preflight_integration_ready_model();
        let idempotent = ProofpackWriterActiveBehaviorModel::from_preflight_and_observation(
            &preflight,
            Some(ProofpackWriterFileIoObservation::target_exists_matching(
                vec![ProofBoundaryNote::new(
                    "Existing canonical artifact matched planned identity.",
                )
                .expect("boundary note should be valid")],
            )),
            ProofpackWriterSideEffectStatus::NotSelected,
            vec![ProofBoundaryNote::new("Idempotent match is evidence only.")
                .expect("boundary note should be valid")],
        );
        let conflict = ProofpackWriterActiveBehaviorModel::from_preflight_and_observation(
            &preflight,
            Some(ProofpackWriterFileIoObservation::target_exists_different(
                vec![
                    ProofBoundaryNote::new("Existing canonical artifact differed.")
                        .expect("boundary note should be valid"),
                ],
            )),
            ProofpackWriterSideEffectStatus::NotSelected,
            vec![],
        );
        let write_failed = ProofpackWriterActiveBehaviorModel::from_preflight_and_observation(
            &preflight,
            Some(ProofpackWriterFileIoObservation::write_failed(vec![
                ProofBoundaryNote::new("Temp write failed before artifact availability.")
                    .expect("boundary note should be valid"),
            ])),
            ProofpackWriterSideEffectStatus::NotSelected,
            vec![],
        );
        let partial = ProofpackWriterActiveBehaviorModel::from_preflight_and_observation(
            &preflight,
            Some(ProofpackWriterFileIoObservation::partial_write_detected(
                ProofpackWriterSideEffectStatus::Failed,
                vec![
                    ProofBoundaryNote::new("Partial artifact and cleanup failure visible.")
                        .expect("boundary note should be valid"),
                ],
            )),
            ProofpackWriterSideEffectStatus::NotSelected,
            vec![],
        );

        assert_eq!(
            idempotent.outcome(),
            ProofpackWriterOperationOutcome::AlreadyExistsMatching
        );
        assert_eq!(
            idempotent.canonical_artifact_status(),
            ProofpackWriterCanonicalArtifactStatus::AlreadyExistsMatching
        );
        assert!(idempotent.canonical_artifact_available());
        assert!(idempotent.attempted_side_effects().is_empty());
        assert!(!idempotent.has_conflict());

        assert_eq!(
            conflict.outcome(),
            ProofpackWriterOperationOutcome::ConflictExistingDifferent
        );
        assert!(conflict.has_conflict());
        assert!(!conflict.canonical_artifact_available());
        assert!(conflict.attempted_side_effects().is_empty());

        assert_eq!(
            write_failed.outcome(),
            ProofpackWriterOperationOutcome::WriteFailed
        );
        assert!(write_failed.selected_side_effect_was_attempted(
            ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite
        ));
        assert!(write_failed
            .selected_side_effect_failed(ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite));
        assert!(!write_failed.canonical_artifact_available());

        assert_eq!(
            partial.outcome(),
            ProofpackWriterOperationOutcome::PartialWriteDetected
        );
        assert!(partial.has_partial_or_cleanup_issue());
        assert!(partial
            .selected_side_effect_failed(ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite));
        assert_eq!(
            partial.cleanup_status(),
            ProofpackWriterSideEffectStatus::Failed
        );
        assert!(!partial.creates_acceptance_claim());
    }

    #[test]
    fn proofpack_writer_active_behavior_model_keeps_index_latest_and_evidence_persistence_visible()
    {
        let preflight = sample_writer_preflight_integration_ready_model();
        let written = ProofpackWriterActiveBehaviorModel::from_preflight_and_observation(
            &preflight,
            Some(
                ProofpackWriterFileIoObservation::target_missing_write_completed(
                    ProofpackWriterSideEffectStatus::Completed,
                    ProofpackWriterSideEffectStatus::Completed,
                    vec![
                        ProofBoundaryNote::new("Canonical artifact write completed.")
                            .expect("boundary note should be valid"),
                    ],
                ),
            ),
            ProofpackWriterSideEffectStatus::Failed,
            vec![ProofBoundaryNote::new(
                "Operation evidence persistence failure remains visible and non-authoritative.",
            )
            .expect("boundary note should be valid")],
        );
        let index_failed = ProofpackWriterActiveBehaviorModel::from_preflight_and_observation(
            &preflight,
            Some(
                ProofpackWriterFileIoObservation::index_failed_after_available(vec![
                    ProofBoundaryNote::new(
                        "Index update failed after canonical artifact availability.",
                    )
                    .expect("boundary note should be valid"),
                ]),
            ),
            ProofpackWriterSideEffectStatus::NotSelected,
            vec![],
        );
        let latest_failed = ProofpackWriterActiveBehaviorModel::from_preflight_and_observation(
            &preflight,
            Some(
                ProofpackWriterFileIoObservation::latest_failed_after_available(vec![
                    ProofBoundaryNote::new(
                        "Latest pointer update failed after canonical artifact availability.",
                    )
                    .expect("boundary note should be valid"),
                ]),
            ),
            ProofpackWriterSideEffectStatus::NotSelected,
            vec![],
        );
        let evidence = written
            .to_operation_evidence(
                ProofpackWriterOperationId::new("writer_active_behavior_written_001")
                    .expect("operation id should be valid"),
                ProofpackWriterAttemptedAt::new("2026-04-27T06:40:00Z")
                    .expect("attempted_at should be valid"),
            )
            .expect("active behavior model should map to operation evidence");

        assert_eq!(written.outcome(), ProofpackWriterOperationOutcome::Written);
        assert!(written.canonical_artifact_available());
        assert!(written.selected_side_effect_completed(
            ProofpackWriterPlannedSideEffect::CanonicalArtifactWrite
        ));
        assert!(written.has_operation_evidence_persistence_failure());
        assert!(!written.persists_operation_evidence());
        assert_eq!(evidence.outcome(), ProofpackWriterOperationOutcome::Written);
        assert!(!evidence.creates_acceptance_claim());

        assert_eq!(
            index_failed.outcome(),
            ProofpackWriterOperationOutcome::IndexUpdateFailed
        );
        assert_eq!(
            index_failed.index_status(),
            ProofpackWriterSideEffectStatus::Failed
        );
        assert!(
            index_failed.selected_side_effect_failed(ProofpackWriterPlannedSideEffect::IndexUpdate)
        );
        assert!(index_failed.has_index_or_latest_pointer_failure());
        assert!(index_failed.canonical_artifact_available());
        assert!(!index_failed.index_latest_are_canonical());

        assert_eq!(
            latest_failed.outcome(),
            ProofpackWriterOperationOutcome::LatestPointerUpdateFailed
        );
        assert_eq!(
            latest_failed.latest_pointer_status(),
            ProofpackWriterSideEffectStatus::Failed
        );
        assert!(latest_failed
            .selected_side_effect_failed(ProofpackWriterPlannedSideEffect::LatestPointerUpdate));
        assert!(latest_failed.has_index_or_latest_pointer_failure());
        assert!(latest_failed.canonical_artifact_available());
        assert!(!latest_failed.index_latest_are_canonical());
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
        let proofpack_id =
            ProofpackId::new("proofpack_local_002").expect("proofpack id should be valid");
        let target_ref_policy = ProofpackWriterTargetArtifactRefPolicyModel::evaluate(
            Some(proofpack_id.clone()),
            Some(ArtifactDigest::new(PROOF_HASH_GATE_DECISION).expect("digest should be valid")),
            ProofpackWriterCanonicalArtifactLayout::ManifestOnlyJson,
            vec![],
        );

        let inconsistent = ProofpackWriterOperationEvidence::new(
            ProofpackWriterOperationId::new("writer_op_local_002")
                .expect("operation id should be valid"),
            ProofpackWriterOperationKind::Write,
            proofpack_id,
            ProofpackWriterAttemptedAt::new("2026-04-26T13:01:00Z")
                .expect("attempted_at should be valid"),
            ProofpackWriterTargetRef::from_target_artifact_ref_policy_model(&target_ref_policy)
                .expect("target artifact ref policy should derive logical ref"),
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
        assert_eq!(
            ProofpackWriterStorageRootRef::new(" "),
            Err(ProofpackError::EmptyWriterStorageRootRef)
        );
        assert_eq!(
            ProofpackWriterTargetPathRef::new(" "),
            Err(ProofpackError::EmptyWriterTargetPathRef)
        );
        assert_eq!(
            ProofpackWriterDiagnosticPathRef::new(" "),
            Err(ProofpackError::EmptyWriterDiagnosticPathRef)
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
