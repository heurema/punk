//! Minimal side-effect-free proofpack kernel for Punk Phase 3.
//!
//! This crate models post-gate provenance as data only. It does not write
//! `.punk/proofs`, expose CLI behavior, write gate decisions, claim
//! acceptance, run validators, or require runtime storage.

use std::fmt::Write as _;

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const PROOFPACK_SCHEMA_VERSION: &str = "punk.proofpack.v0.1";

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
    InvalidArtifactHash(ArtifactHashPolicyError),
    MissingContractRefs,
    MissingRunReceiptRefs,
    MissingBoundaryNotes,
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
