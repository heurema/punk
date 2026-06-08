//! Minimal contract lifecycle kernel for Punk Phase 3.
//!
//! This crate intentionally stays deterministic and side-effect free in this
//! first implementation step. It models a bounded contract, explicit scope,
//! minimal approval-for-run state, and validation only. No `.punk/` storage,
//! CLI behavior, gate decisions, proofpacks, or run execution are introduced
//! here. Contract Context Pack helpers are also side-effect free and do not
//! activate retrieval, executor briefs, runtime storage, or authority changes.

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContractId(String);

impl ContractId {
    pub fn new(value: impl Into<String>) -> Result<Self, ContractError> {
        let value = value.into().trim().to_string();

        if value.is_empty() {
            return Err(ContractError::EmptyContractId);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ContractScope {
    refs: Vec<String>,
}

impl ContractScope {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_ref(mut self, artifact_ref: impl Into<String>) -> Self {
        let artifact_ref = artifact_ref.into().trim().to_string();

        if !artifact_ref.is_empty() {
            self.refs.push(artifact_ref);
        }

        self
    }

    pub fn refs(&self) -> &[String] {
        &self.refs
    }

    pub fn is_empty(&self) -> bool {
        self.refs.is_empty()
    }

    pub fn refs_are_repo_relative(&self) -> bool {
        !self.is_empty()
            && self
                .refs
                .iter()
                .all(|artifact_ref| is_safe_contract_scope_ref(artifact_ref))
    }
}

fn is_safe_contract_scope_ref(artifact_ref: &str) -> bool {
    let artifact_ref = artifact_ref.trim();
    !artifact_ref.is_empty()
        && !artifact_ref.starts_with('/')
        && !artifact_ref.starts_with('~')
        && !artifact_ref.contains('\\')
        && !artifact_ref.contains(':')
        && !artifact_ref.split('/').any(|segment| {
            segment.is_empty()
                || matches!(segment, "." | "..")
                || segment.chars().any(char::is_control)
        })
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextPackId(String);

impl ContextPackId {
    pub fn new(value: impl Into<String>) -> Result<Self, ContextPackError> {
        let value = value.into().trim().to_string();

        if value.is_empty() {
            return Err(ContextPackError::EmptyContextPackId);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextPackStatus {
    Draft,
    Frozen,
    Superseded,
}

impl ContextPackStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Frozen => "frozen",
            Self::Superseded => "superseded",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextAuthority {
    Canonical,
    Advisory,
    Speculative,
}

impl ContextAuthority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canonical => "canonical",
            Self::Advisory => "advisory",
            Self::Speculative => "speculative",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextDocStatus {
    Active,
    Stale,
    Superseded,
    Archived,
    Retired,
    Unknown,
}

impl ContextDocStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Stale => "stale",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
            Self::Retired => "retired",
            Self::Unknown => "unknown",
        }
    }

    pub fn requires_explicit_flag(self) -> bool {
        !matches!(self, Self::Active | Self::Unknown)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextCoverageType {
    Source,
    Clarification,
    Assumption,
    Unknown,
}

impl ContextCoverageType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Source => "source",
            Self::Clarification => "clarification",
            Self::Assumption => "assumption",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextPackItem {
    item_id: String,
    artifact_ref: String,
    authority: ContextAuthority,
    doc_status: ContextDocStatus,
    supports: Vec<String>,
}

impl ContextPackItem {
    pub fn new(
        item_id: impl Into<String>,
        artifact_ref: impl Into<String>,
        authority: ContextAuthority,
        doc_status: ContextDocStatus,
    ) -> Self {
        Self {
            item_id: item_id.into().trim().to_string(),
            artifact_ref: artifact_ref.into().trim().to_string(),
            authority,
            doc_status,
            supports: Vec::new(),
        }
    }

    pub fn with_support(mut self, clause_id: impl Into<String>) -> Self {
        let clause_id = clause_id.into().trim().to_string();
        if !clause_id.is_empty() {
            self.supports.push(clause_id);
        }
        self
    }

    pub fn item_id(&self) -> &str {
        &self.item_id
    }
    pub fn artifact_ref(&self) -> &str {
        &self.artifact_ref
    }
    pub fn authority(&self) -> ContextAuthority {
        self.authority
    }
    pub fn doc_status(&self) -> ContextDocStatus {
        self.doc_status
    }
    pub fn supports(&self) -> &[String] {
        &self.supports
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextPackClauseCoverage {
    clause_id: String,
    covered_by: Vec<String>,
    coverage_type: ContextCoverageType,
}

impl ContextPackClauseCoverage {
    pub fn new(clause_id: impl Into<String>, coverage_type: ContextCoverageType) -> Self {
        Self {
            clause_id: clause_id.into().trim().to_string(),
            covered_by: Vec::new(),
            coverage_type,
        }
    }

    pub fn with_covered_by(mut self, ref_id: impl Into<String>) -> Self {
        let ref_id = ref_id.into().trim().to_string();
        if !ref_id.is_empty() {
            self.covered_by.push(ref_id);
        }
        self
    }

    pub fn clause_id(&self) -> &str {
        &self.clause_id
    }
    pub fn covered_by(&self) -> &[String] {
        &self.covered_by
    }
    pub fn coverage_type(&self) -> ContextCoverageType {
        self.coverage_type
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextPackExclusion {
    artifact_ref: String,
    reason: String,
}

impl ContextPackExclusion {
    pub fn new(artifact_ref: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            artifact_ref: artifact_ref.into().trim().to_string(),
            reason: reason.into().trim().to_string(),
        }
    }

    pub fn artifact_ref(&self) -> &str {
        &self.artifact_ref
    }
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextPackContradiction {
    refs: Vec<String>,
    note: String,
}

impl ContextPackContradiction {
    pub fn new(refs: Vec<String>, note: impl Into<String>) -> Self {
        Self {
            refs: refs
                .into_iter()
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty())
                .collect(),
            note: note.into().trim().to_string(),
        }
    }

    pub fn refs(&self) -> &[String] {
        &self.refs
    }
    pub fn note(&self) -> &str {
        &self.note
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextPackUnknown {
    statement: String,
    blocking: bool,
}

impl ContextPackUnknown {
    pub fn new(statement: impl Into<String>, blocking: bool) -> Self {
        Self {
            statement: statement.into().trim().to_string(),
            blocking,
        }
    }

    pub fn statement(&self) -> &str {
        &self.statement
    }
    pub fn blocking(&self) -> bool {
        self.blocking
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractContextPack {
    id: ContextPackId,
    contract_ref: String,
    status: ContextPackStatus,
    items: Vec<ContextPackItem>,
    clause_coverage: Vec<ContextPackClauseCoverage>,
    exclusions: Vec<ContextPackExclusion>,
    contradictions: Vec<ContextPackContradiction>,
    unknowns: Vec<ContextPackUnknown>,
    retrieval_receipt_refs: Vec<String>,
}

impl ContractContextPack {
    pub fn new(
        id: ContextPackId,
        contract_ref: impl Into<String>,
        status: ContextPackStatus,
    ) -> Self {
        Self {
            id,
            contract_ref: contract_ref.into().trim().to_string(),
            status,
            items: Vec::new(),
            clause_coverage: Vec::new(),
            exclusions: Vec::new(),
            contradictions: Vec::new(),
            unknowns: Vec::new(),
            retrieval_receipt_refs: Vec::new(),
        }
    }

    pub fn with_item(mut self, item: ContextPackItem) -> Self {
        self.items.push(item);
        self
    }
    pub fn with_clause_coverage(mut self, coverage: ContextPackClauseCoverage) -> Self {
        self.clause_coverage.push(coverage);
        self
    }
    pub fn with_exclusion(mut self, exclusion: ContextPackExclusion) -> Self {
        self.exclusions.push(exclusion);
        self
    }
    pub fn with_contradiction(mut self, contradiction: ContextPackContradiction) -> Self {
        self.contradictions.push(contradiction);
        self
    }
    pub fn with_unknown(mut self, unknown: ContextPackUnknown) -> Self {
        self.unknowns.push(unknown);
        self
    }

    pub fn with_retrieval_receipt_ref(mut self, receipt_ref: impl Into<String>) -> Self {
        let receipt_ref = receipt_ref.into().trim().to_string();
        if !receipt_ref.is_empty() {
            self.retrieval_receipt_refs.push(receipt_ref);
        }
        self
    }

    pub fn id(&self) -> &ContextPackId {
        &self.id
    }
    pub fn contract_ref(&self) -> &str {
        &self.contract_ref
    }
    pub fn status(&self) -> ContextPackStatus {
        self.status
    }
    pub fn items(&self) -> &[ContextPackItem] {
        &self.items
    }
    pub fn clause_coverage(&self) -> &[ContextPackClauseCoverage] {
        &self.clause_coverage
    }
    pub fn exclusions(&self) -> &[ContextPackExclusion] {
        &self.exclusions
    }
    pub fn contradictions(&self) -> &[ContextPackContradiction] {
        &self.contradictions
    }
    pub fn unknowns(&self) -> &[ContextPackUnknown] {
        &self.unknowns
    }
    pub fn retrieval_receipt_refs(&self) -> &[String] {
        &self.retrieval_receipt_refs
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractStatus {
    Draft,
    ApprovedForRun,
    Superseded,
    Cancelled,
}

impl ContractStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::ApprovedForRun => "approved_for_run",
            Self::Superseded => "superseded",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn ready_for_bounded_work(self) -> bool {
        matches!(self, Self::ApprovedForRun)
    }

    pub fn is_gate_acceptance_decision(self) -> bool {
        false
    }
}

pub const CONTRACT_STATUS_VALUES: [ContractStatus; 4] = [
    ContractStatus::Draft,
    ContractStatus::ApprovedForRun,
    ContractStatus::Superseded,
    ContractStatus::Cancelled,
];

pub const CONTRACT_SCHEMA_BLUEPRINT_VERSION: &str = "contract-schema-blueprint.v0.1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractSchemaFieldStatus {
    RequiredNow,
    Deferred,
    Parked,
    Future,
}

impl ContractSchemaFieldStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RequiredNow => "required_now",
            Self::Deferred => "deferred",
            Self::Parked => "parked",
            Self::Future => "future",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractSchemaSection {
    Identity,
    Authority,
    Lifecycle,
    Work,
    Boundaries,
    Clauses,
    Validation,
    Evidence,
    ReceiptRequirements,
    GatePolicy,
    ProofRequirements,
    ChangeControl,
    MemoryLinks,
}

impl ContractSchemaSection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::Authority => "authority",
            Self::Lifecycle => "lifecycle",
            Self::Work => "work",
            Self::Boundaries => "boundaries",
            Self::Clauses => "clauses",
            Self::Validation => "validation",
            Self::Evidence => "evidence",
            Self::ReceiptRequirements => "receipt_requirements",
            Self::GatePolicy => "gate_policy",
            Self::ProofRequirements => "proof_requirements",
            Self::ChangeControl => "change_control",
            Self::MemoryLinks => "memory_links",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContractSchemaField {
    pub section: ContractSchemaSection,
    pub name: &'static str,
    pub status: ContractSchemaFieldStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContractSchemaAvoidPattern {
    pub name: &'static str,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContractSchemaBlueprintV0_1 {
    sections: &'static [ContractSchemaSection],
    fields: &'static [ContractSchemaField],
    avoid_patterns: &'static [ContractSchemaAvoidPattern],
}

impl ContractSchemaBlueprintV0_1 {
    pub fn version(&self) -> &'static str {
        CONTRACT_SCHEMA_BLUEPRINT_VERSION
    }

    pub fn sections(&self) -> &'static [ContractSchemaSection] {
        self.sections
    }

    pub fn fields(&self) -> &'static [ContractSchemaField] {
        self.fields
    }

    pub fn avoid_patterns(&self) -> &'static [ContractSchemaAvoidPattern] {
        self.avoid_patterns
    }

    pub fn has_required_sections(&self) -> bool {
        CONTRACT_SCHEMA_REQUIRED_SECTIONS
            .iter()
            .all(|section| self.sections.contains(section))
    }

    pub fn has_all_status_classes(&self) -> bool {
        [
            ContractSchemaFieldStatus::RequiredNow,
            ContractSchemaFieldStatus::Deferred,
            ContractSchemaFieldStatus::Parked,
            ContractSchemaFieldStatus::Future,
        ]
        .iter()
        .all(|status| self.fields.iter().any(|field| field.status == *status))
    }

    pub fn has_field(&self, section: ContractSchemaSection, name: &str) -> bool {
        self.fields
            .iter()
            .any(|field| field.section == section && field.name == name)
    }

    pub fn fields_for_section(
        &self,
        section: ContractSchemaSection,
    ) -> impl Iterator<Item = &'static ContractSchemaField> {
        self.fields
            .iter()
            .filter(move |field| field.section == section)
    }
}

pub fn contract_schema_blueprint_v0_1() -> ContractSchemaBlueprintV0_1 {
    ContractSchemaBlueprintV0_1 {
        sections: &CONTRACT_SCHEMA_REQUIRED_SECTIONS,
        fields: CONTRACT_SCHEMA_FIELDS,
        avoid_patterns: CONTRACT_SCHEMA_AVOID_PATTERNS,
    }
}

pub const CONTRACT_SCHEMA_REQUIRED_SECTIONS: [ContractSchemaSection; 13] = [
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
];

pub const CONTRACT_SCHEMA_FIELDS: &[ContractSchemaField] = &[
    ContractSchemaField {
        section: ContractSchemaSection::Identity,
        name: "kind",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Identity,
        name: "schema_version",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Identity,
        name: "id",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Identity,
        name: "title",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Identity,
        name: "created_at",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Identity,
        name: "digest_metadata",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Authority,
        name: "core_rules_ref",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Authority,
        name: "project_rules_ref",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Authority,
        name: "roadmap_phase_ref",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Authority,
        name: "research_gate_ref",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Authority,
        name: "authority_level",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Authority,
        name: "run_contract_can_narrow_upper_rules",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Authority,
        name: "run_contract_can_weaken_upper_rules_false",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Authority,
        name: "pki_signature",
        status: ContractSchemaFieldStatus::Parked,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Lifecycle,
        name: "status",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Lifecycle,
        name: "approved_by",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Lifecycle,
        name: "approved_at",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Lifecycle,
        name: "supersedes",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Lifecycle,
        name: "superseded_by",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Lifecycle,
        name: "closed_by_gate_ref",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Work,
        name: "goal_ref",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Work,
        name: "intent",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Work,
        name: "rationale",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Work,
        name: "assumptions",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Work,
        name: "unknowns",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Work,
        name: "research_refs",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Work,
        name: "decision_refs",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Work,
        name: "acceptance_criteria",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Work,
        name: "semantic_intent_recovery",
        status: ContractSchemaFieldStatus::Future,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Work,
        name: "contract_as_source_generation",
        status: ContractSchemaFieldStatus::Future,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Boundaries,
        name: "scope.include",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Boundaries,
        name: "scope.exclude",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Boundaries,
        name: "non_scope",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Boundaries,
        name: "forbidden_effects",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Boundaries,
        name: "side_effect_boundaries",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Boundaries,
        name: "executor_boundary",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Boundaries,
        name: "module_boundary",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Boundaries,
        name: "adapter_boundary",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Boundaries,
        name: "module_adapter_boundary_manifests",
        status: ContractSchemaFieldStatus::Deferred,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Boundaries,
        name: "full_capability_manifests",
        status: ContractSchemaFieldStatus::Parked,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Clauses,
        name: "clause.id",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Clauses,
        name: "clause.kind",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Clauses,
        name: "clause.text",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Clauses,
        name: "clause.mode",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Clauses,
        name: "clause.severity",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Clauses,
        name: "clause.source_refs",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Clauses,
        name: "clause.validator_refs",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Clauses,
        name: "clause.required_receipt_fields",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Clauses,
        name: "clause.proof_requirement_refs",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Clauses,
        name: "clause.gate_review_required",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Clauses,
        name: "context_clause_coverage_matrix",
        status: ContractSchemaFieldStatus::Deferred,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Clauses,
        name: "contract_negotiation_workflow",
        status: ContractSchemaFieldStatus::Parked,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Clauses,
        name: "spec_native_takeover",
        status: ContractSchemaFieldStatus::Future,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Validation,
        name: "validator_plan",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Validation,
        name: "eval_refs",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Validation,
        name: "unsupported_clause_policy",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Validation,
        name: "richer_compatibility_version_checks",
        status: ContractSchemaFieldStatus::Deferred,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Validation,
        name: "policy_engine_integration",
        status: ContractSchemaFieldStatus::Parked,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Evidence,
        name: "evidence_plan",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Evidence,
        name: "required_artifacts",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Evidence,
        name: "allowed_sources",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Evidence,
        name: "excluded_sources",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Evidence,
        name: "freshness_policy",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Evidence,
        name: "stale_refs_must_be_flagged",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Evidence,
        name: "superseded_refs_must_be_flagged",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Evidence,
        name: "contradictions_must_be_surfaced",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::Evidence,
        name: "stale_superseded_source_automation",
        status: ContractSchemaFieldStatus::Deferred,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ReceiptRequirements,
        name: "contract_id",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ReceiptRequirements,
        name: "executor_id",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ReceiptRequirements,
        name: "started_at",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ReceiptRequirements,
        name: "ended_at",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ReceiptRequirements,
        name: "inputs",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ReceiptRequirements,
        name: "outputs",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ReceiptRequirements,
        name: "artifact_refs",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ReceiptRequirements,
        name: "artifact_hashes",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ReceiptRequirements,
        name: "validator_results",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ReceiptRequirements,
        name: "side_effects",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ReceiptRequirements,
        name: "deviation_flags",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ReceiptRequirements,
        name: "executor_claims",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::GatePolicy,
        name: "gate_only_final_decision",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::GatePolicy,
        name: "required_inputs",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::GatePolicy,
        name: "hard_fail_on",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::GatePolicy,
        name: "accept_requires",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::GatePolicy,
        name: "partial_acceptance_policy",
        status: ContractSchemaFieldStatus::Deferred,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ProofRequirements,
        name: "must_link.contract",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ProofRequirements,
        name: "must_link.run_receipt",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ProofRequirements,
        name: "must_link.gate_outcome",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ProofRequirements,
        name: "must_link.output_artifacts",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ProofRequirements,
        name: "must_link.eval_reports_when_applicable",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ProofRequirements,
        name: "must_hash.contract",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ProofRequirements,
        name: "must_hash.run_receipt",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ProofRequirements,
        name: "must_hash.gate_outcome",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ProofRequirements,
        name: "must_hash.output_artifacts",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ProofRequirements,
        name: "remote_transparency_log",
        status: ContractSchemaFieldStatus::Parked,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ChangeControl,
        name: "amendment_policy",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ChangeControl,
        name: "supersession_policy",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ChangeControl,
        name: "deviation_policy",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ChangeControl,
        name: "waiver_policy",
        status: ContractSchemaFieldStatus::Deferred,
    },
    ContractSchemaField {
        section: ContractSchemaSection::ChangeControl,
        name: "waiver_ledger",
        status: ContractSchemaFieldStatus::Deferred,
    },
    ContractSchemaField {
        section: ContractSchemaSection::MemoryLinks,
        name: "goal_ref",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::MemoryLinks,
        name: "contract_ref",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::MemoryLinks,
        name: "context_pack_ref",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::MemoryLinks,
        name: "report_refs",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::MemoryLinks,
        name: "research_refs",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::MemoryLinks,
        name: "adr_refs",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::MemoryLinks,
        name: "eval_refs",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::MemoryLinks,
        name: "decision_refs",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::MemoryLinks,
        name: "proof_refs",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::MemoryLinks,
        name: "doc_impact",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::MemoryLinks,
        name: "public_narrative_refs",
        status: ContractSchemaFieldStatus::RequiredNow,
    },
    ContractSchemaField {
        section: ContractSchemaSection::MemoryLinks,
        name: "multi_agent_coordinated_contracts",
        status: ContractSchemaFieldStatus::Future,
    },
    ContractSchemaField {
        section: ContractSchemaSection::MemoryLinks,
        name: "semantic_merge_of_contracts_specs_code",
        status: ContractSchemaFieldStatus::Future,
    },
];

pub const CONTRACT_SCHEMA_AVOID_PATTERNS: &[ContractSchemaAvoidPattern] = &[
    ContractSchemaAvoidPattern {
        name: "smart_contract_style_automatic_finality",
        reason: "Contract does not own acceptance authority.",
    },
    ContractSchemaAvoidPattern {
        name: "oracle_driven_truth",
        reason: "Evidence sources inform review; they do not become truth authority.",
    },
    ContractSchemaAvoidPattern {
        name: "on_chain_execution",
        reason: "Punk contracts are local work contracts, not execution ledgers.",
    },
    ContractSchemaAvoidPattern {
        name: "proofpack_as_decision_authority",
        reason: "Proofpack links evidence after review; it is not an authority surface.",
    },
    ContractSchemaAvoidPattern {
        name: "writer_as_upstream_planner",
        reason: "Writer remains downstream of intent, contract, execution, receipt, and gate.",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractClauseKind {
    Precondition,
    Invariant,
    Acceptance,
    Evidence,
    Prohibition,
    Rationale,
}

impl ContractClauseKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Precondition => "precondition",
            Self::Invariant => "invariant",
            Self::Acceptance => "acceptance",
            Self::Evidence => "evidence",
            Self::Prohibition => "prohibition",
            Self::Rationale => "rationale",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractClauseMode {
    Human,
    Machine,
    Hybrid,
}

impl ContractClauseMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Human => "human",
            Self::Machine => "machine",
            Self::Hybrid => "hybrid",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractClauseSeverity {
    Hard,
    Soft,
    Advisory,
}

impl ContractClauseSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Hard => "hard",
            Self::Soft => "soft",
            Self::Advisory => "advisory",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContractClauseBlueprint {
    pub id: &'static str,
    pub kind: ContractClauseKind,
    pub mode: ContractClauseMode,
    pub severity: ContractClauseSeverity,
    pub has_validator_refs: bool,
    pub has_required_receipt_fields: bool,
    pub required_receipt_fields: &'static [&'static str],
    pub has_proof_requirement_refs: bool,
    pub gate_review_required: bool,
    pub human_gate_review_reason: Option<&'static str>,
    pub unsupported_clause_finding: bool,
}

impl ContractClauseBlueprint {
    pub fn hard_clause_mapping_satisfied(&self) -> bool {
        !assess_hard_clause_mapping(self).blocks_approval()
    }

    pub fn human_gate_review_reason(&self) -> Option<&'static str> {
        self.human_gate_review_reason
            .map(str::trim)
            .filter(|reason| !reason.is_empty())
    }

    pub fn required_receipt_fields(&self) -> &[&'static str] {
        // Empty hard receipt-field mappings default to artifact hashes so the
        // hard-clause check stays fail-closed instead of silently unmapped.
        if self.has_required_receipt_fields && self.required_receipt_fields.is_empty() {
            &["artifact_hashes"]
        } else {
            self.required_receipt_fields
        }
    }

    fn mapping_targets(&self) -> Vec<HardClauseMappingTarget> {
        let mut targets = Vec::new();

        if self.has_validator_refs {
            targets.push(HardClauseMappingTarget::ValidatorRef);
        }
        if self.has_required_receipt_fields {
            targets.push(HardClauseMappingTarget::RequiredReceiptField);
        }
        if self.has_proof_requirement_refs {
            targets.push(HardClauseMappingTarget::ProofRequirementRef);
        }
        if self.gate_review_required && self.human_gate_review_reason().is_some() {
            targets.push(HardClauseMappingTarget::HumanGateReview);
        }

        targets
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HardClauseMappingTarget {
    ValidatorRef,
    RequiredReceiptField,
    ProofRequirementRef,
    HumanGateReview,
}

impl HardClauseMappingTarget {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ValidatorRef => "validator_ref",
            Self::RequiredReceiptField => "required_receipt_field",
            Self::ProofRequirementRef => "proof_requirement_ref",
            Self::HumanGateReview => "human_gate_review",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HardClauseMappingStatus {
    Mapped,
    RequiresHumanGateReview,
    UnsupportedBlocking,
    Invalid,
    NotRequiredForSoftOrAdvisory,
}

impl HardClauseMappingStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Mapped => "mapped",
            Self::RequiresHumanGateReview => "requires_human_gate_review",
            Self::UnsupportedBlocking => "unsupported_blocking",
            Self::Invalid => "invalid",
            Self::NotRequiredForSoftOrAdvisory => "not_required_for_soft_or_advisory",
        }
    }

    pub fn blocks_approval(self) -> bool {
        matches!(self, Self::UnsupportedBlocking | Self::Invalid)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardClauseMappingFinding {
    clause_id: &'static str,
    status: HardClauseMappingStatus,
    targets: Vec<HardClauseMappingTarget>,
    reason: Option<&'static str>,
}

impl HardClauseMappingFinding {
    fn new(
        clause_id: &'static str,
        status: HardClauseMappingStatus,
        targets: Vec<HardClauseMappingTarget>,
        reason: Option<&'static str>,
    ) -> Self {
        Self {
            clause_id,
            status,
            targets,
            reason,
        }
    }

    pub fn clause_id(&self) -> &'static str {
        self.clause_id
    }

    pub fn status(&self) -> HardClauseMappingStatus {
        self.status
    }

    pub fn targets(&self) -> &[HardClauseMappingTarget] {
        &self.targets
    }

    pub fn reason(&self) -> Option<&'static str> {
        self.reason
    }

    pub fn blocks_approval(&self) -> bool {
        self.status.blocks_approval()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardClauseMappingAssessment {
    findings: Vec<HardClauseMappingFinding>,
}

impl HardClauseMappingAssessment {
    pub fn findings(&self) -> &[HardClauseMappingFinding] {
        &self.findings
    }

    pub fn ready_for_approval(&self) -> bool {
        !self.approval_blocking()
    }

    pub fn approval_blocking(&self) -> bool {
        self.findings
            .iter()
            .any(HardClauseMappingFinding::blocks_approval)
    }

    pub fn has_status(&self, status: HardClauseMappingStatus) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.status() == status)
    }

    pub fn executes_validators(&self) -> bool {
        false
    }

    pub fn writes_gate_decision(&self) -> bool {
        false
    }

    pub fn creates_proofpack(&self) -> bool {
        false
    }

    pub fn invokes_writer(&self) -> bool {
        false
    }
}

pub fn assess_hard_clause_mappings(
    clauses: &[ContractClauseBlueprint],
) -> HardClauseMappingAssessment {
    let findings = clauses
        .iter()
        .map(assess_hard_clause_mapping)
        .collect::<Vec<_>>();

    HardClauseMappingAssessment { findings }
}

fn assess_hard_clause_mapping(clause: &ContractClauseBlueprint) -> HardClauseMappingFinding {
    let targets = clause.mapping_targets();

    if clause.severity != ContractClauseSeverity::Hard {
        return HardClauseMappingFinding::new(
            clause.id,
            HardClauseMappingStatus::NotRequiredForSoftOrAdvisory,
            targets,
            Some("soft_or_advisory_clause_does_not_require_hard_mapping"),
        );
    }

    if clause.kind == ContractClauseKind::Rationale {
        return HardClauseMappingFinding::new(
            clause.id,
            HardClauseMappingStatus::Invalid,
            targets,
            Some("hard_rationale_clause_is_not_an_enforcement_clause"),
        );
    }

    if clause.unsupported_clause_finding {
        return HardClauseMappingFinding::new(
            clause.id,
            HardClauseMappingStatus::UnsupportedBlocking,
            targets,
            Some("unsupported_hard_clause_blocks_approval"),
        );
    }

    if clause.gate_review_required && clause.human_gate_review_reason().is_none() {
        return HardClauseMappingFinding::new(
            clause.id,
            HardClauseMappingStatus::Invalid,
            targets,
            Some("human_gate_review_requires_reason"),
        );
    }

    if targets.is_empty() {
        return HardClauseMappingFinding::new(
            clause.id,
            HardClauseMappingStatus::UnsupportedBlocking,
            targets,
            Some("hard_clause_has_no_mapping_target"),
        );
    }

    let status = if targets.as_slice() == [HardClauseMappingTarget::HumanGateReview] {
        HardClauseMappingStatus::RequiresHumanGateReview
    } else {
        HardClauseMappingStatus::Mapped
    };

    HardClauseMappingFinding::new(
        clause.id,
        status,
        targets,
        clause.human_gate_review_reason(),
    )
}

pub const CONTRACT_RECEIPT_REQUIREMENTS_MODEL_SCHEMA_VERSION: &str =
    "contract-receipt-requirements.v0.1";

pub const CONTRACT_RECEIPT_KNOWN_FIELDS: [&str; 12] = [
    "contract_id",
    "executor_id",
    "started_at",
    "ended_at",
    "inputs",
    "outputs",
    "artifact_refs",
    "artifact_hashes",
    "validator_results",
    "side_effects",
    "deviation_flags",
    "executor_claims",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReceiptRequirementSource {
    ContractDefault,
    HardClause(&'static str),
    SoftClause(&'static str),
    AdvisoryClause(&'static str),
    SideEffectBoundary,
    DeviationPolicy,
    ExecutorClaimPolicy,
}

impl ReceiptRequirementSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ContractDefault => "contract_default",
            Self::HardClause(_) => "hard_clause",
            Self::SoftClause(_) => "soft_clause",
            Self::AdvisoryClause(_) => "advisory_clause",
            Self::SideEffectBoundary => "side_effect_boundary",
            Self::DeviationPolicy => "deviation_policy",
            Self::ExecutorClaimPolicy => "executor_claim_policy",
        }
    }

    pub fn clause_id(self) -> Option<&'static str> {
        match self {
            Self::HardClause(clause_id)
            | Self::SoftClause(clause_id)
            | Self::AdvisoryClause(clause_id) => Some(clause_id),
            Self::ContractDefault
            | Self::SideEffectBoundary
            | Self::DeviationPolicy
            | Self::ExecutorClaimPolicy => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContractReceiptRequirement {
    field: &'static str,
    source: ReceiptRequirementSource,
}

impl ContractReceiptRequirement {
    pub const fn new(field: &'static str, source: ReceiptRequirementSource) -> Self {
        Self { field, source }
    }

    pub const fn contract_default(field: &'static str) -> Self {
        Self::new(field, ReceiptRequirementSource::ContractDefault)
    }

    pub const fn for_clause(field: &'static str, clause: &ContractClauseBlueprint) -> Self {
        let source = match clause.severity {
            ContractClauseSeverity::Hard => ReceiptRequirementSource::HardClause(clause.id),
            ContractClauseSeverity::Soft => ReceiptRequirementSource::SoftClause(clause.id),
            ContractClauseSeverity::Advisory => ReceiptRequirementSource::AdvisoryClause(clause.id),
        };
        Self::new(field, source)
    }

    pub fn field(&self) -> &'static str {
        self.field
    }

    pub fn source(&self) -> ReceiptRequirementSource {
        self.source
    }

    pub fn known_field(&self) -> bool {
        is_known_receipt_requirement_field(self.field)
    }

    pub fn executor_claims_are_proof(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReceiptRequirementStatus {
    SatisfiedByRequiredField,
    MissingRequiredField,
    UnsupportedField,
    DuplicateRequirement,
    NotRequiredForClause,
}

impl ReceiptRequirementStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SatisfiedByRequiredField => "satisfied_by_required_field",
            Self::MissingRequiredField => "missing_required_field",
            Self::UnsupportedField => "unsupported_field",
            Self::DuplicateRequirement => "duplicate_requirement",
            Self::NotRequiredForClause => "not_required_for_clause",
        }
    }

    pub fn blocks_approval(self) -> bool {
        matches!(self, Self::MissingRequiredField | Self::UnsupportedField)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptRequirementFinding {
    field: &'static str,
    status: ReceiptRequirementStatus,
    source: ReceiptRequirementSource,
    reason: &'static str,
}

impl ReceiptRequirementFinding {
    fn new(
        field: &'static str,
        status: ReceiptRequirementStatus,
        source: ReceiptRequirementSource,
        reason: &'static str,
    ) -> Self {
        Self {
            field,
            status,
            source,
            reason,
        }
    }

    pub fn field(&self) -> &'static str {
        self.field
    }

    pub fn status(&self) -> ReceiptRequirementStatus {
        self.status
    }

    pub fn source(&self) -> ReceiptRequirementSource {
        self.source
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }

    pub fn blocks_approval(&self) -> bool {
        self.status.blocks_approval()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptRequirementCoverage {
    clause_id: &'static str,
    field: &'static str,
    status: ReceiptRequirementStatus,
}

impl ReceiptRequirementCoverage {
    fn new(clause_id: &'static str, field: &'static str, status: ReceiptRequirementStatus) -> Self {
        Self {
            clause_id,
            field,
            status,
        }
    }

    pub fn clause_id(&self) -> &'static str {
        self.clause_id
    }

    pub fn field(&self) -> &'static str {
        self.field
    }

    pub fn status(&self) -> ReceiptRequirementStatus {
        self.status
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptRequirementAssessment {
    required_fields: Vec<&'static str>,
    findings: Vec<ReceiptRequirementFinding>,
    coverage: Vec<ReceiptRequirementCoverage>,
}

impl ReceiptRequirementAssessment {
    pub fn required_fields(&self) -> &[&'static str] {
        &self.required_fields
    }

    pub fn findings(&self) -> &[ReceiptRequirementFinding] {
        &self.findings
    }

    pub fn coverage(&self) -> &[ReceiptRequirementCoverage] {
        &self.coverage
    }

    pub fn approval_blocking(&self) -> bool {
        self.findings
            .iter()
            .any(ReceiptRequirementFinding::blocks_approval)
            || self
                .coverage
                .iter()
                .any(|coverage| coverage.status().blocks_approval())
    }

    pub fn ready_for_approval(&self) -> bool {
        !self.approval_blocking()
    }

    pub fn has_status(&self, status: ReceiptRequirementStatus) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.status() == status)
            || self
                .coverage
                .iter()
                .any(|coverage| coverage.status() == status)
    }

    pub fn requires_field(&self, field: &str) -> bool {
        self.required_fields.iter().any(|known| *known == field)
    }

    pub fn executor_claims_are_proof(&self) -> bool {
        false
    }

    pub fn executes_validators(&self) -> bool {
        false
    }

    pub fn writes_run_receipt(&self) -> bool {
        false
    }

    pub fn writes_punk_runs_storage(&self) -> bool {
        false
    }

    pub fn writes_gate_decision(&self) -> bool {
        false
    }

    pub fn creates_proofpack(&self) -> bool {
        false
    }

    pub fn invokes_writer(&self) -> bool {
        false
    }
}

pub fn is_known_receipt_requirement_field(field: &str) -> bool {
    CONTRACT_RECEIPT_KNOWN_FIELDS.contains(&field)
}

pub fn assess_receipt_requirements(
    clauses: &[ContractClauseBlueprint],
    requirements: &[ContractReceiptRequirement],
) -> ReceiptRequirementAssessment {
    let mut required_fields = Vec::new();
    let mut findings = Vec::new();
    let mut coverage = Vec::new();

    for requirement in requirements {
        if !requirement.known_field() {
            findings.push(ReceiptRequirementFinding::new(
                requirement.field(),
                ReceiptRequirementStatus::UnsupportedField,
                requirement.source(),
                "receipt_requirement_field_is_not_known",
            ));
            continue;
        }

        if required_fields.contains(&requirement.field()) {
            findings.push(ReceiptRequirementFinding::new(
                requirement.field(),
                ReceiptRequirementStatus::DuplicateRequirement,
                requirement.source(),
                "duplicate_receipt_requirement_is_non_blocking_and_normalized",
            ));
        } else {
            required_fields.push(requirement.field());
            findings.push(ReceiptRequirementFinding::new(
                requirement.field(),
                ReceiptRequirementStatus::SatisfiedByRequiredField,
                requirement.source(),
                "receipt_requirement_field_is_declared",
            ));
        }
    }

    for clause in clauses {
        if clause.required_receipt_fields().is_empty() {
            if clause.severity != ContractClauseSeverity::Hard && clause.has_required_receipt_fields
            {
                coverage.push(ReceiptRequirementCoverage::new(
                    clause.id,
                    "",
                    ReceiptRequirementStatus::NotRequiredForClause,
                ));
            }
            continue;
        }

        for field in clause.required_receipt_fields() {
            if !is_known_receipt_requirement_field(field) {
                coverage.push(ReceiptRequirementCoverage::new(
                    clause.id,
                    field,
                    ReceiptRequirementStatus::UnsupportedField,
                ));
                continue;
            }

            let field_declared = required_fields.contains(field);
            match clause.severity {
                ContractClauseSeverity::Hard => {
                    if field_declared {
                        coverage.push(ReceiptRequirementCoverage::new(
                            clause.id,
                            field,
                            ReceiptRequirementStatus::SatisfiedByRequiredField,
                        ));
                    } else {
                        coverage.push(ReceiptRequirementCoverage::new(
                            clause.id,
                            field,
                            ReceiptRequirementStatus::MissingRequiredField,
                        ));
                    }
                }
                ContractClauseSeverity::Soft | ContractClauseSeverity::Advisory => {
                    let status = if field_declared {
                        ReceiptRequirementStatus::SatisfiedByRequiredField
                    } else {
                        ReceiptRequirementStatus::NotRequiredForClause
                    };
                    coverage.push(ReceiptRequirementCoverage::new(clause.id, field, status));
                }
            }
        }
    }

    ReceiptRequirementAssessment {
        required_fields,
        findings,
        coverage,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContractGatePolicyBlueprint {
    pub gate_only_final_decision: bool,
    pub writes_decision: bool,
    pub required_inputs: &'static [&'static str],
    pub hard_fail_on: &'static [&'static str],
    pub accept_requires: &'static [&'static str],
}

pub const CONTRACT_GATE_POLICY_REQUIRED_INPUTS: [&str; 9] = [
    "contract_ref",
    "approved_for_run_status",
    "run_receipt_ref",
    "receipt_requirement_coverage",
    "hard_clause_mapping_assessment",
    "validator_or_eval_report",
    "scope_status",
    "deviation_status",
    "executor_claim_status",
];

pub const CONTRACT_GATE_POLICY_HARD_FAIL_ON: [&str; 7] = [
    "missing_contract_ref",
    "missing_approved_for_run_status",
    "missing_run_receipt_ref",
    "missing_receipt_requirement_coverage",
    "missing_hard_clause_mapping_assessment",
    "unmapped_hard_clause",
    "unsupported_gate_input",
];

pub const CONTRACT_GATE_POLICY_ACCEPT_REQUIRES: [&str; 7] = [
    "contract_ref",
    "approved_for_run_status",
    "run_receipt_ref",
    "receipt_requirement_coverage",
    "hard_clause_mapping_assessment",
    "satisfied_hard_clauses",
    "deviation_status",
];

pub const CONTRACT_GATE_POLICY_BLUEPRINT: ContractGatePolicyBlueprint =
    ContractGatePolicyBlueprint {
        gate_only_final_decision: true,
        writes_decision: false,
        required_inputs: &CONTRACT_GATE_POLICY_REQUIRED_INPUTS,
        hard_fail_on: &CONTRACT_GATE_POLICY_HARD_FAIL_ON,
        accept_requires: &CONTRACT_GATE_POLICY_ACCEPT_REQUIRES,
    };

pub const fn contract_gate_policy_blueprint() -> &'static ContractGatePolicyBlueprint {
    &CONTRACT_GATE_POLICY_BLUEPRINT
}

pub const CONTRACT_GATE_INPUT_POLICY_MODEL_SCHEMA_VERSION: &str = "contract-gate-input-policy.v0.1";

pub const CONTRACT_GATE_INPUT_KNOWN_INPUTS: [&str; 12] = [
    "contract_ref",
    "approved_for_run_status",
    "run_receipt_ref",
    "receipt_requirement_coverage",
    "hard_clause_mapping_assessment",
    "validator_or_eval_report",
    "module_or_semantic_assessment",
    "scope_status",
    "deviation_status",
    "executor_claim_status",
    "artifact_refs",
    "artifact_hashes",
];

pub const CONTRACT_GATE_INPUT_POLICY_BASELINE_REQUIREMENTS: [&str; 9] = [
    "contract_ref",
    "approved_for_run_status",
    "run_receipt_ref",
    "receipt_requirement_coverage",
    "hard_clause_mapping_assessment",
    "validator_or_eval_report",
    "scope_status",
    "deviation_status",
    "executor_claim_status",
];

pub const CONTRACT_GATE_INPUT_POST_GATE_EXPECTATIONS: [&str; 1] =
    ["post_gate_proof_requirement_pending"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GateInputRequirementSource {
    ContractPolicy,
    ApprovedContract,
    RunReceipt,
    ReceiptRequirements,
    HardClauseMapping,
    EvalPolicy,
    ModuleAssessment,
    ScopePolicy,
    DeviationPolicy,
    ExecutorClaimPolicy,
    ArtifactPolicy,
}

impl GateInputRequirementSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ContractPolicy => "contract_policy",
            Self::ApprovedContract => "approved_contract",
            Self::RunReceipt => "run_receipt",
            Self::ReceiptRequirements => "receipt_requirements",
            Self::HardClauseMapping => "hard_clause_mapping",
            Self::EvalPolicy => "eval_policy",
            Self::ModuleAssessment => "module_assessment",
            Self::ScopePolicy => "scope_policy",
            Self::DeviationPolicy => "deviation_policy",
            Self::ExecutorClaimPolicy => "executor_claim_policy",
            Self::ArtifactPolicy => "artifact_policy",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GateInputRequirement {
    input: &'static str,
    source: GateInputRequirementSource,
}

impl GateInputRequirement {
    pub const fn new(input: &'static str, source: GateInputRequirementSource) -> Self {
        Self { input, source }
    }

    pub const fn contract_policy(input: &'static str) -> Self {
        Self::new(input, GateInputRequirementSource::ContractPolicy)
    }

    pub fn input(&self) -> &'static str {
        self.input
    }

    pub fn source(&self) -> GateInputRequirementSource {
        self.source
    }

    pub fn known_input(&self) -> bool {
        is_known_gate_input(self.input)
    }

    pub fn is_proofpack_input(&self) -> bool {
        is_forbidden_gate_input(self.input)
    }

    pub fn executor_claims_are_proof(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GateInputEvidence {
    input: &'static str,
}

impl GateInputEvidence {
    pub const fn new(input: &'static str) -> Self {
        Self { input }
    }

    pub fn input(&self) -> &'static str {
        self.input
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractGateInputPolicy {
    requirements: Vec<GateInputRequirement>,
    post_gate_expectations: Vec<&'static str>,
}

impl ContractGateInputPolicy {
    pub fn new(requirements: Vec<GateInputRequirement>) -> Self {
        Self {
            requirements,
            post_gate_expectations: CONTRACT_GATE_INPUT_POST_GATE_EXPECTATIONS.to_vec(),
        }
    }

    pub fn requirements(&self) -> &[GateInputRequirement] {
        &self.requirements
    }

    pub fn post_gate_expectations(&self) -> &[&'static str] {
        &self.post_gate_expectations
    }

    pub fn requires_input(&self, input: &str) -> bool {
        self.requirements
            .iter()
            .any(|requirement| requirement.input() == input)
    }

    pub fn requires_existing_proofpack(&self) -> bool {
        self.requirements
            .iter()
            .any(GateInputRequirement::is_proofpack_input)
    }

    pub fn post_gate_proof_requirement_pending(&self) -> bool {
        self.post_gate_expectations
            .contains(&"post_gate_proof_requirement_pending")
    }

    pub fn approved_for_run_is_ready_for_gate(&self) -> bool {
        false
    }

    pub fn writes_gate_decision(&self) -> bool {
        false
    }

    pub fn creates_proofpack(&self) -> bool {
        false
    }

    pub fn creates_acceptance_claim(&self) -> bool {
        false
    }

    pub fn invokes_writer(&self) -> bool {
        false
    }

    pub fn runs_validators(&self) -> bool {
        false
    }

    pub fn writes_storage(&self) -> bool {
        false
    }
}

pub fn contract_gate_input_policy_v0_1() -> ContractGateInputPolicy {
    ContractGateInputPolicy::new(
        CONTRACT_GATE_INPUT_POLICY_BASELINE_REQUIREMENTS
            .iter()
            .map(|input| {
                let source = match *input {
                    "contract_ref" => GateInputRequirementSource::ContractPolicy,
                    "approved_for_run_status" => GateInputRequirementSource::ApprovedContract,
                    "run_receipt_ref" => GateInputRequirementSource::RunReceipt,
                    "receipt_requirement_coverage" => {
                        GateInputRequirementSource::ReceiptRequirements
                    }
                    "hard_clause_mapping_assessment" => {
                        GateInputRequirementSource::HardClauseMapping
                    }
                    "validator_or_eval_report" => GateInputRequirementSource::EvalPolicy,
                    "scope_status" => GateInputRequirementSource::ScopePolicy,
                    "deviation_status" => GateInputRequirementSource::DeviationPolicy,
                    "executor_claim_status" => GateInputRequirementSource::ExecutorClaimPolicy,
                    _ => GateInputRequirementSource::ContractPolicy,
                };
                GateInputRequirement::new(input, source)
            })
            .collect(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GateInputRequirementStatus {
    Covered,
    MissingRequiredInput,
    UnsupportedInput,
    DuplicateRequirement,
    NotRequired,
}

impl GateInputRequirementStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Covered => "covered",
            Self::MissingRequiredInput => "missing_required_input",
            Self::UnsupportedInput => "unsupported_input",
            Self::DuplicateRequirement => "duplicate_requirement",
            Self::NotRequired => "not_required",
        }
    }

    pub fn blocks_gate_readiness(self) -> bool {
        matches!(self, Self::MissingRequiredInput | Self::UnsupportedInput)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateInputPolicyFinding {
    input: &'static str,
    status: GateInputRequirementStatus,
    source: GateInputRequirementSource,
    reason: &'static str,
}

impl GateInputPolicyFinding {
    fn new(
        input: &'static str,
        status: GateInputRequirementStatus,
        source: GateInputRequirementSource,
        reason: &'static str,
    ) -> Self {
        Self {
            input,
            status,
            source,
            reason,
        }
    }

    pub fn input(&self) -> &'static str {
        self.input
    }

    pub fn status(&self) -> GateInputRequirementStatus {
        self.status
    }

    pub fn source(&self) -> GateInputRequirementSource {
        self.source
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }

    pub fn blocks_gate_readiness(&self) -> bool {
        self.status.blocks_gate_readiness()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateInputCoverage {
    input: &'static str,
    status: GateInputRequirementStatus,
}

impl GateInputCoverage {
    fn new(input: &'static str, status: GateInputRequirementStatus) -> Self {
        Self { input, status }
    }

    pub fn input(&self) -> &'static str {
        self.input
    }

    pub fn status(&self) -> GateInputRequirementStatus {
        self.status
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateInputPolicyAssessment {
    required_inputs: Vec<&'static str>,
    findings: Vec<GateInputPolicyFinding>,
    coverage: Vec<GateInputCoverage>,
}

impl GateInputPolicyAssessment {
    pub fn required_inputs(&self) -> &[&'static str] {
        &self.required_inputs
    }

    pub fn findings(&self) -> &[GateInputPolicyFinding] {
        &self.findings
    }

    pub fn coverage(&self) -> &[GateInputCoverage] {
        &self.coverage
    }

    pub fn has_status(&self, status: GateInputRequirementStatus) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.status() == status)
            || self
                .coverage
                .iter()
                .any(|coverage| coverage.status() == status)
    }

    pub fn requires_input(&self, input: &str) -> bool {
        self.required_inputs
            .iter()
            .any(|required| *required == input)
    }

    pub fn gate_readiness_blocking(&self) -> bool {
        self.findings
            .iter()
            .any(GateInputPolicyFinding::blocks_gate_readiness)
            || self
                .coverage
                .iter()
                .any(|coverage| coverage.status().blocks_gate_readiness())
    }

    pub fn ready_for_gate(&self) -> bool {
        !self.gate_readiness_blocking()
    }

    pub fn approved_for_run_is_ready_for_gate(&self) -> bool {
        false
    }

    pub fn requires_existing_proofpack(&self) -> bool {
        self.required_inputs
            .iter()
            .any(|input| is_forbidden_gate_input(input))
    }

    pub fn executor_claims_are_proof(&self) -> bool {
        false
    }

    pub fn writes_gate_decision(&self) -> bool {
        false
    }

    pub fn creates_proofpack(&self) -> bool {
        false
    }

    pub fn creates_acceptance_claim(&self) -> bool {
        false
    }

    pub fn invokes_writer(&self) -> bool {
        false
    }

    pub fn runs_validators(&self) -> bool {
        false
    }

    pub fn writes_storage(&self) -> bool {
        false
    }
}

pub fn is_known_gate_input(input: &str) -> bool {
    CONTRACT_GATE_INPUT_KNOWN_INPUTS.contains(&input)
}

pub fn is_forbidden_gate_input(input: &str) -> bool {
    matches!(
        input,
        "proofpack"
            | "proofpack_ref"
            | "existing_proofpack"
            | "proofpack_writer_output"
            | "proof_requirements"
            | "proofpack_requirements"
            | "acceptance_claim"
            | "writer_output"
    )
}

pub fn assess_gate_input_policy(
    policy: &ContractGateInputPolicy,
    available_inputs: &[GateInputEvidence],
) -> GateInputPolicyAssessment {
    let mut required_inputs = Vec::new();
    let mut findings = Vec::new();
    let mut coverage = Vec::new();

    for requirement in policy.requirements() {
        if requirement.is_proofpack_input() {
            findings.push(GateInputPolicyFinding::new(
                requirement.input(),
                GateInputRequirementStatus::UnsupportedInput,
                requirement.source(),
                "proofpack_is_post_gate_and_cannot_be_gate_input",
            ));
            continue;
        }

        if !requirement.known_input() {
            findings.push(GateInputPolicyFinding::new(
                requirement.input(),
                GateInputRequirementStatus::UnsupportedInput,
                requirement.source(),
                "gate_input_is_not_known",
            ));
            continue;
        }

        if required_inputs.contains(&requirement.input()) {
            findings.push(GateInputPolicyFinding::new(
                requirement.input(),
                GateInputRequirementStatus::DuplicateRequirement,
                requirement.source(),
                "duplicate_gate_input_requirement_is_non_blocking_and_normalized",
            ));
        } else {
            required_inputs.push(requirement.input());
        }
    }

    for baseline_input in CONTRACT_GATE_INPUT_POLICY_BASELINE_REQUIREMENTS {
        if !required_inputs.contains(&baseline_input) {
            findings.push(GateInputPolicyFinding::new(
                baseline_input,
                GateInputRequirementStatus::MissingRequiredInput,
                GateInputRequirementSource::ContractPolicy,
                "gate_input_policy_missing_baseline_requirement",
            ));
        }
    }

    for input in &required_inputs {
        let covered = available_inputs
            .iter()
            .any(|evidence| evidence.input() == *input);
        let status = if covered {
            GateInputRequirementStatus::Covered
        } else {
            GateInputRequirementStatus::MissingRequiredInput
        };
        coverage.push(GateInputCoverage::new(*input, status));
    }

    GateInputPolicyAssessment {
        required_inputs,
        findings,
        coverage,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContractProofRequirementsBlueprint {
    pub must_link: &'static [&'static str],
    pub must_hash: &'static [&'static str],
    pub creates_proofpack: bool,
    pub proofpack_is_decision_authority: bool,
}

pub const CONTRACT_PROOF_REQUIREMENTS_MUST_LINK: [&str; 5] = [
    "contract_ref",
    "run_receipt_ref",
    "gate_decision_ref",
    "output_artifact_refs",
    "eval_report_ref",
];

pub const CONTRACT_PROOF_REQUIREMENTS_MUST_HASH: [&str; 4] = [
    "contract_hash",
    "run_receipt_hash",
    "gate_decision_hash",
    "output_artifact_hashes",
];

pub const CONTRACT_PROOF_REQUIREMENTS_BLUEPRINT: ContractProofRequirementsBlueprint =
    ContractProofRequirementsBlueprint {
        must_link: &CONTRACT_PROOF_REQUIREMENTS_MUST_LINK,
        must_hash: &CONTRACT_PROOF_REQUIREMENTS_MUST_HASH,
        creates_proofpack: false,
        proofpack_is_decision_authority: false,
    };

pub const fn contract_proof_requirements_blueprint() -> &'static ContractProofRequirementsBlueprint
{
    &CONTRACT_PROOF_REQUIREMENTS_BLUEPRINT
}

pub const CONTRACT_PROOF_REQUIREMENTS_MODEL_SCHEMA_VERSION: &str =
    "contract-proof-requirements.v0.1";

pub const CONTRACT_PROOF_KNOWN_TARGETS: [&str; 12] = [
    "contract_ref",
    "contract_hash",
    "run_receipt_ref",
    "run_receipt_hash",
    "gate_decision_ref",
    "gate_decision_hash",
    "eval_report_ref",
    "eval_report_hash",
    "output_artifact_refs",
    "output_artifact_hashes",
    "event_range_or_root",
    "rule_or_guard_version_refs",
];

pub const CONTRACT_PROOF_BASELINE_REQUIRED_TARGETS: [&str; 3] =
    ["contract_ref", "run_receipt_ref", "gate_decision_ref"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofRequirementSource {
    ContractPolicy,
    RunReceipt,
    GateOutcome,
    EvalPolicy,
    ArtifactPolicy,
    EventPolicy,
    RuleOrGuardPolicy,
}

impl ProofRequirementSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ContractPolicy => "contract_policy",
            Self::RunReceipt => "run_receipt",
            Self::GateOutcome => "gate_outcome",
            Self::EvalPolicy => "eval_policy",
            Self::ArtifactPolicy => "artifact_policy",
            Self::EventPolicy => "event_policy",
            Self::RuleOrGuardPolicy => "rule_or_guard_policy",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProofRequirement {
    target: &'static str,
    source: ProofRequirementSource,
}

impl ProofRequirement {
    pub const fn new(target: &'static str, source: ProofRequirementSource) -> Self {
        Self { target, source }
    }

    pub const fn contract_policy(target: &'static str) -> Self {
        Self::new(target, ProofRequirementSource::ContractPolicy)
    }

    pub fn target(&self) -> &'static str {
        self.target
    }

    pub fn source(&self) -> ProofRequirementSource {
        self.source
    }

    pub fn known_target(&self) -> bool {
        is_known_proof_requirement_target(self.target)
    }

    pub fn forbidden_before_gate(&self) -> bool {
        is_forbidden_proof_requirement_target(self.target)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractProofRequirements {
    requirements: Vec<ProofRequirement>,
}

impl ContractProofRequirements {
    pub fn new(requirements: Vec<ProofRequirement>) -> Self {
        Self { requirements }
    }

    pub fn requirements(&self) -> &[ProofRequirement] {
        &self.requirements
    }

    pub fn requires_target(&self, target: &str) -> bool {
        self.requirements
            .iter()
            .any(|requirement| requirement.target() == target)
    }

    pub fn requires_proofpack_before_gate(&self) -> bool {
        self.requirements
            .iter()
            .any(ProofRequirement::forbidden_before_gate)
    }

    pub fn creates_proofpack(&self) -> bool {
        false
    }

    pub fn writes_punk_proofs_storage(&self) -> bool {
        false
    }

    pub fn computes_artifact_hashes_from_filesystem(&self) -> bool {
        false
    }

    pub fn writes_gate_decision(&self) -> bool {
        false
    }

    pub fn creates_acceptance_claim(&self) -> bool {
        false
    }

    pub fn invokes_writer(&self) -> bool {
        false
    }
}

pub fn contract_proof_requirements_v0_1() -> ContractProofRequirements {
    ContractProofRequirements::new(vec![
        ProofRequirement::new("contract_ref", ProofRequirementSource::ContractPolicy),
        ProofRequirement::new("contract_hash", ProofRequirementSource::ContractPolicy),
        ProofRequirement::new("run_receipt_ref", ProofRequirementSource::RunReceipt),
        ProofRequirement::new("run_receipt_hash", ProofRequirementSource::RunReceipt),
        ProofRequirement::new("gate_decision_ref", ProofRequirementSource::GateOutcome),
        ProofRequirement::new("gate_decision_hash", ProofRequirementSource::GateOutcome),
        ProofRequirement::new("eval_report_ref", ProofRequirementSource::EvalPolicy),
        ProofRequirement::new("eval_report_hash", ProofRequirementSource::EvalPolicy),
        ProofRequirement::new(
            "output_artifact_refs",
            ProofRequirementSource::ArtifactPolicy,
        ),
        ProofRequirement::new(
            "output_artifact_hashes",
            ProofRequirementSource::ArtifactPolicy,
        ),
    ])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofRequirementStatus {
    Covered,
    MissingRequiredTarget,
    UnsupportedTarget,
    DuplicateRequirement,
    NotRequired,
}

impl ProofRequirementStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Covered => "covered",
            Self::MissingRequiredTarget => "missing_required_target",
            Self::UnsupportedTarget => "unsupported_target",
            Self::DuplicateRequirement => "duplicate_requirement",
            Self::NotRequired => "not_required",
        }
    }

    pub fn blocks_proof_requirement_declaration(self) -> bool {
        matches!(self, Self::MissingRequiredTarget | Self::UnsupportedTarget)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofRequirementFinding {
    target: &'static str,
    status: ProofRequirementStatus,
    source: ProofRequirementSource,
    reason: &'static str,
}

impl ProofRequirementFinding {
    fn new(
        target: &'static str,
        status: ProofRequirementStatus,
        source: ProofRequirementSource,
        reason: &'static str,
    ) -> Self {
        Self {
            target,
            status,
            source,
            reason,
        }
    }

    pub fn target(&self) -> &'static str {
        self.target
    }

    pub fn status(&self) -> ProofRequirementStatus {
        self.status
    }

    pub fn source(&self) -> ProofRequirementSource {
        self.source
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }

    pub fn blocks_proof_requirement_declaration(&self) -> bool {
        self.status.blocks_proof_requirement_declaration()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofRequirementCoverage {
    target: &'static str,
    status: ProofRequirementStatus,
}

impl ProofRequirementCoverage {
    fn new(target: &'static str, status: ProofRequirementStatus) -> Self {
        Self { target, status }
    }

    pub fn target(&self) -> &'static str {
        self.target
    }

    pub fn status(&self) -> ProofRequirementStatus {
        self.status
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofRequirementAssessment {
    required_targets: Vec<&'static str>,
    findings: Vec<ProofRequirementFinding>,
    coverage: Vec<ProofRequirementCoverage>,
}

impl ProofRequirementAssessment {
    pub fn required_targets(&self) -> &[&'static str] {
        &self.required_targets
    }

    pub fn findings(&self) -> &[ProofRequirementFinding] {
        &self.findings
    }

    pub fn coverage(&self) -> &[ProofRequirementCoverage] {
        &self.coverage
    }

    pub fn has_status(&self, status: ProofRequirementStatus) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.status() == status)
            || self
                .coverage
                .iter()
                .any(|coverage| coverage.status() == status)
    }

    pub fn requires_target(&self, target: &str) -> bool {
        self.required_targets
            .iter()
            .any(|required| *required == target)
    }

    pub fn declaration_blocking(&self) -> bool {
        self.findings
            .iter()
            .any(ProofRequirementFinding::blocks_proof_requirement_declaration)
            || self
                .coverage
                .iter()
                .any(|coverage| coverage.status().blocks_proof_requirement_declaration())
    }

    pub fn proof_requirements_declared(&self) -> bool {
        !self.declaration_blocking()
    }

    pub fn proofpack_ready_after_gate(&self) -> bool {
        false
    }

    pub fn requires_proofpack_before_gate(&self) -> bool {
        self.required_targets
            .iter()
            .any(|target| is_forbidden_proof_requirement_target(target))
    }

    pub fn creates_proofpack(&self) -> bool {
        false
    }

    pub fn writes_punk_proofs_storage(&self) -> bool {
        false
    }

    pub fn computes_artifact_hashes_from_filesystem(&self) -> bool {
        false
    }

    pub fn writes_gate_decision(&self) -> bool {
        false
    }

    pub fn creates_acceptance_claim(&self) -> bool {
        false
    }

    pub fn invokes_writer(&self) -> bool {
        false
    }
}

pub fn is_known_proof_requirement_target(target: &str) -> bool {
    CONTRACT_PROOF_KNOWN_TARGETS.contains(&target)
}

pub fn is_forbidden_proof_requirement_target(target: &str) -> bool {
    matches!(
        target,
        "proofpack"
            | "proofpack_ref"
            | "existing_proofpack"
            | "proofpack_writer_output"
            | "acceptance_claim"
            | "writer_output"
    )
}

pub fn assess_proof_requirements(
    requirements: &ContractProofRequirements,
) -> ProofRequirementAssessment {
    let mut required_targets = Vec::new();
    let mut findings = Vec::new();
    let mut coverage = Vec::new();

    for requirement in requirements.requirements() {
        if requirement.forbidden_before_gate() {
            findings.push(ProofRequirementFinding::new(
                requirement.target(),
                ProofRequirementStatus::UnsupportedTarget,
                requirement.source(),
                "proofpack_or_acceptance_claim_is_not_a_proof_requirement_target",
            ));
            continue;
        }

        if !requirement.known_target() {
            findings.push(ProofRequirementFinding::new(
                requirement.target(),
                ProofRequirementStatus::UnsupportedTarget,
                requirement.source(),
                "proof_requirement_target_is_not_known",
            ));
            continue;
        }

        if required_targets.contains(&requirement.target()) {
            findings.push(ProofRequirementFinding::new(
                requirement.target(),
                ProofRequirementStatus::DuplicateRequirement,
                requirement.source(),
                "duplicate_proof_requirement_is_non_blocking_and_normalized",
            ));
        } else {
            required_targets.push(requirement.target());
        }
    }

    for baseline_target in CONTRACT_PROOF_BASELINE_REQUIRED_TARGETS {
        if !required_targets.contains(&baseline_target) {
            findings.push(ProofRequirementFinding::new(
                baseline_target,
                ProofRequirementStatus::MissingRequiredTarget,
                ProofRequirementSource::ContractPolicy,
                "proof_requirements_missing_baseline_target",
            ));
        }
    }

    for target in &required_targets {
        coverage.push(ProofRequirementCoverage::new(
            *target,
            ProofRequirementStatus::Covered,
        ));
    }

    ProofRequirementAssessment {
        required_targets,
        findings,
        coverage,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContractWriterAuthorityBoundary {
    pub writer_is_upstream_planner: bool,
    pub writer_defines_contract: bool,
    pub writer_writes_gate_decision: bool,
    pub writer_claims_acceptance: bool,
}

pub const fn contract_writer_authority_boundary() -> ContractWriterAuthorityBoundary {
    ContractWriterAuthorityBoundary {
        writer_is_upstream_planner: false,
        writer_defines_contract: false,
        writer_writes_gate_decision: false,
        writer_claims_acceptance: false,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractDraft {
    id: ContractId,
    title: String,
    scope: ContractScope,
}

impl ContractDraft {
    pub fn new(id: ContractId, title: impl Into<String>, scope: ContractScope) -> Self {
        Self {
            id,
            title: title.into(),
            scope,
        }
    }

    pub fn id(&self) -> &ContractId {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn scope(&self) -> &ContractScope {
        &self.scope
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contract {
    id: ContractId,
    title: String,
    scope: ContractScope,
    status: ContractStatus,
}

impl Contract {
    pub fn id(&self) -> &ContractId {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn scope(&self) -> &ContractScope {
        &self.scope
    }

    pub fn status(&self) -> ContractStatus {
        self.status
    }

    pub fn contract_valid(&self) -> bool {
        !self.title.trim().is_empty() && self.scope_valid()
    }

    pub fn scope_valid(&self) -> bool {
        self.scope.refs_are_repo_relative()
    }

    pub fn ready_for_bounded_work(&self) -> bool {
        self.contract_valid() && self.status.ready_for_bounded_work()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractError {
    EmptyContractId,
    EmptyTitle,
    EmptyScope,
    UnsafeScopeRef,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextPackError {
    EmptyContextPackId,
    EmptyContractRef,
    EmptyContextItemId,
    EmptyContextItemRef,
    ContextItemWithoutSupport,
    SpeculativeContextItemIncluded,
    EmptyClauseCoverageId,
    CoverageWithoutBacking,
    CoverageReferencesMissingContextItem,
    CoverageBackedByNonSupportingItem,
    ContextItemSupportWithoutCoverage,
    UnsupportedRequiredClause,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContractLifecycleBoundary {
    pub implies_final_acceptance: bool,
    pub writes_gate_decision: bool,
    pub creates_proofpack: bool,
    pub requires_runtime_storage: bool,
}

pub const fn contract_lifecycle_boundary() -> ContractLifecycleBoundary {
    ContractLifecycleBoundary {
        implies_final_acceptance: false,
        writes_gate_decision: false,
        creates_proofpack: false,
        requires_runtime_storage: false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContextPackBoundary {
    pub writes_gate_decision: bool,
    pub owns_project_truth: bool,
    pub requires_runtime_storage: bool,
    pub allows_executor_override: bool,
    pub activates_retrieval_adapter: bool,
}

pub const fn context_pack_boundary() -> ContextPackBoundary {
    ContextPackBoundary {
        writes_gate_decision: false,
        owns_project_truth: false,
        requires_runtime_storage: false,
        allows_executor_override: false,
        activates_retrieval_adapter: false,
    }
}

pub const USER_INTENT_CONTRACT_DRAFT_MODEL_SCHEMA_VERSION: &str =
    "user-intent-contract-draft-model.v0.1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserIntentResearchGateClassification {
    R0,
    R1,
    R2,
    R3,
}

impl UserIntentResearchGateClassification {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::R0 => "R0",
            Self::R1 => "R1",
            Self::R2 => "R2",
            Self::R3 => "R3",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserIntentResearchGate {
    classification: UserIntentResearchGateClassification,
    required: bool,
    refs: Vec<String>,
}

impl UserIntentResearchGate {
    pub fn new(classification: UserIntentResearchGateClassification, required: bool) -> Self {
        Self {
            classification,
            required,
            refs: Vec::new(),
        }
    }

    pub fn with_ref(mut self, artifact_ref: impl Into<String>) -> Self {
        push_non_empty(&mut self.refs, artifact_ref);
        self
    }

    pub fn classification(&self) -> UserIntentResearchGateClassification {
        self.classification
    }

    pub fn required(&self) -> bool {
        self.required
    }

    pub fn refs(&self) -> &[String] {
        &self.refs
    }

    pub fn satisfied(&self) -> bool {
        !self.required || !self.refs.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserIntentDownstreamClosure {
    pub receipt_required: bool,
    pub gate_decision_required_for_acceptance: bool,
    pub proofpack_required_for_positive_acceptance: bool,
    pub executor_claims_are_proof: bool,
    pub user_confirmation_is_gate_acceptance: bool,
    pub writer_may_infer_intent: bool,
}

impl Default for UserIntentDownstreamClosure {
    fn default() -> Self {
        Self {
            receipt_required: true,
            gate_decision_required_for_acceptance: true,
            proofpack_required_for_positive_acceptance: true,
            executor_claims_are_proof: false,
            user_confirmation_is_gate_acceptance: false,
            writer_may_infer_intent: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserIntentContractDraftBoundary {
    pub raw_request_is_contract: bool,
    pub intent_is_execution_permission: bool,
    pub user_confirmation_is_gate_acceptance: bool,
    pub executor_claims_are_proof: bool,
    pub context_pack_defines_acceptance: bool,
    pub writer_is_downstream_only: bool,
    pub writes_runtime_storage: bool,
    pub writes_gate_decision: bool,
    pub writes_proofpack: bool,
}

pub const fn user_intent_contract_draft_boundary() -> UserIntentContractDraftBoundary {
    UserIntentContractDraftBoundary {
        raw_request_is_contract: false,
        intent_is_execution_permission: false,
        user_confirmation_is_gate_acceptance: false,
        executor_claims_are_proof: false,
        context_pack_defines_acceptance: false,
        writer_is_downstream_only: true,
        writes_runtime_storage: false,
        writes_gate_decision: false,
        writes_proofpack: false,
    }
}

pub const PLOT_INTAKE_ROUTING_MODEL_SCHEMA_VERSION: &str = "plot-intake-routing-model.v0.1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlotIntakeRequest {
    raw_text: String,
    harness_id: Option<String>,
    harness_metadata: Vec<String>,
}

impl PlotIntakeRequest {
    pub fn new(raw_text: impl Into<String>) -> Self {
        Self {
            raw_text: raw_text.into(),
            harness_id: None,
            harness_metadata: Vec::new(),
        }
    }

    pub fn with_harness_id(mut self, harness_id: impl Into<String>) -> Self {
        let harness_id = harness_id.into().trim().to_string();
        if !harness_id.is_empty() {
            self.harness_id = Some(harness_id);
        }
        self
    }

    pub fn with_harness_metadata(mut self, metadata_ref: impl Into<String>) -> Self {
        push_non_empty(&mut self.harness_metadata, metadata_ref);
        self
    }

    pub fn raw_text(&self) -> &str {
        &self.raw_text
    }

    pub fn harness_id(&self) -> Option<&str> {
        self.harness_id.as_deref()
    }

    pub fn harness_metadata(&self) -> &[String] {
        &self.harness_metadata
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlotIntakeRouteStatus {
    Routed,
    ClarificationRequired,
    NotCaptured,
}

impl PlotIntakeRouteStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Routed => "routed",
            Self::ClarificationRequired => "clarification_required",
            Self::NotCaptured => "not_captured",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlotIntakeRouteAuthority {
    NonAuthoritative,
}

impl PlotIntakeRouteAuthority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NonAuthoritative => "non_authoritative",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlotIntakePhaseRoute {
    Plot,
    Cut,
    Gate,
    Blocked,
    Clarify,
}

impl PlotIntakePhaseRoute {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Plot => "plot",
            Self::Cut => "cut",
            Self::Gate => "gate",
            Self::Blocked => "blocked",
            Self::Clarify => "clarify",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlotIntakeModuleRoute {
    Core,
    PubPunk,
    DevPunk,
    FutureModule,
    External,
}

impl PlotIntakeModuleRoute {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Core => "core",
            Self::PubPunk => "pubpunk",
            Self::DevPunk => "devpunk",
            Self::FutureModule => "future_module",
            Self::External => "external",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlotIntakeRouteConfidence {
    Low,
    Medium,
    High,
}

impl PlotIntakeRouteConfidence {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlotIntakeRouteBlocker {
    MissingRequestBody,
    UnclearRequest,
}

impl PlotIntakeRouteBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingRequestBody => "missing_request_body",
            Self::UnclearRequest => "unclear_request",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlotIntakeRouteBoundary {
    pub side_effects_active: bool,
    pub module_activated: bool,
    pub gate_decision_written: bool,
    pub proofpack_written: bool,
    pub acceptance_claimed: bool,
    pub route_hint_is_authority: bool,
}

impl PlotIntakeRouteBoundary {
    pub const fn advisory_only() -> Self {
        Self {
            side_effects_active: false,
            module_activated: false,
            gate_decision_written: false,
            proofpack_written: false,
            acceptance_claimed: false,
            route_hint_is_authority: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlotIntakeRouteResult {
    status: PlotIntakeRouteStatus,
    authority: PlotIntakeRouteAuthority,
    phase_route: Option<PlotIntakePhaseRoute>,
    module_route: Option<PlotIntakeModuleRoute>,
    route_confidence: PlotIntakeRouteConfidence,
    normalized_request: String,
    rationale: Vec<String>,
    blockers: Vec<PlotIntakeRouteBlocker>,
    next_handoff: Option<&'static str>,
    boundary: PlotIntakeRouteBoundary,
}

impl PlotIntakeRouteResult {
    pub fn status(&self) -> PlotIntakeRouteStatus {
        self.status
    }

    pub fn authority(&self) -> PlotIntakeRouteAuthority {
        self.authority
    }

    pub fn phase_route(&self) -> Option<PlotIntakePhaseRoute> {
        self.phase_route
    }

    pub fn module_route(&self) -> Option<PlotIntakeModuleRoute> {
        self.module_route
    }

    pub fn route_confidence(&self) -> PlotIntakeRouteConfidence {
        self.route_confidence
    }

    pub fn confidence(&self) -> PlotIntakeRouteConfidence {
        self.route_confidence
    }

    pub fn normalized_request(&self) -> &str {
        &self.normalized_request
    }

    pub fn rationale(&self) -> &[String] {
        &self.rationale
    }

    pub fn blockers(&self) -> &[PlotIntakeRouteBlocker] {
        &self.blockers
    }

    pub fn has_blocker(&self, blocker: PlotIntakeRouteBlocker) -> bool {
        self.blockers.contains(&blocker)
    }

    pub fn next_handoff(&self) -> Option<&'static str> {
        self.next_handoff
    }

    pub fn boundary(&self) -> &PlotIntakeRouteBoundary {
        &self.boundary
    }
}

pub fn route_plot_intake_request(request: &PlotIntakeRequest) -> PlotIntakeRouteResult {
    let trimmed = request.raw_text().trim();

    match split_plot_intake_command(trimmed) {
        None => not_captured_result(trimmed),
        Some((PlotIntakeCommand::Punk, body)) => route_punk_command(body),
        Some((PlotIntakeCommand::PubHint, body)) => route_hint_command(
            body,
            PlotIntakeModuleRoute::PubPunk,
            "pubpunk.contract_intake",
            "PubPunk route hint is advisory and non-authoritative.",
        ),
        Some((PlotIntakeCommand::DevHint, body)) => route_hint_command(
            body,
            PlotIntakeModuleRoute::DevPunk,
            "devpunk.contract_intake",
            "DevPunk route hint is advisory and non-authoritative.",
        ),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlotIntakeCommand {
    Punk,
    PubHint,
    DevHint,
}

fn split_plot_intake_command(input: &str) -> Option<(PlotIntakeCommand, &str)> {
    supported_plot_intake_commands()
        .iter()
        .find_map(|(prefix, command)| {
            if input == *prefix {
                Some((*command, ""))
            } else {
                input
                    .strip_prefix(prefix)
                    .and_then(|rest| rest.chars().next().filter(|next| next.is_whitespace()))
                    .map(|_| (*command, input[prefix.len()..].trim()))
            }
        })
}

fn supported_plot_intake_commands() -> &'static [(&'static str, PlotIntakeCommand)] {
    &[
        ("/punk", PlotIntakeCommand::Punk),
        ("/pub", PlotIntakeCommand::PubHint),
        ("/dev", PlotIntakeCommand::DevHint),
    ]
}

fn route_punk_command(body: &str) -> PlotIntakeRouteResult {
    if body.trim().is_empty() {
        return clarification_result(body, PlotIntakeRouteBlocker::MissingRequestBody);
    }

    if looks_like_content_request(body) {
        return routed_result(
            body,
            PlotIntakeModuleRoute::PubPunk,
            PlotIntakeRouteConfidence::High,
            "pubpunk.contract_intake",
            "content request matched PubPunk-shaped contract intake.",
        );
    }

    clarification_result(body, PlotIntakeRouteBlocker::UnclearRequest)
}

fn route_hint_command(
    body: &str,
    module_route: PlotIntakeModuleRoute,
    next_handoff: &'static str,
    rationale: impl Into<String>,
) -> PlotIntakeRouteResult {
    if body.trim().is_empty() {
        return clarification_result(body, PlotIntakeRouteBlocker::MissingRequestBody);
    }

    routed_result(
        body,
        module_route,
        PlotIntakeRouteConfidence::Medium,
        next_handoff,
        rationale,
    )
}

fn routed_result(
    body: &str,
    module_route: PlotIntakeModuleRoute,
    confidence: PlotIntakeRouteConfidence,
    next_handoff: &'static str,
    rationale: impl Into<String>,
) -> PlotIntakeRouteResult {
    PlotIntakeRouteResult {
        status: PlotIntakeRouteStatus::Routed,
        authority: PlotIntakeRouteAuthority::NonAuthoritative,
        phase_route: Some(PlotIntakePhaseRoute::Plot),
        module_route: Some(module_route),
        route_confidence: confidence,
        normalized_request: body.trim().to_string(),
        rationale: vec![rationale.into()],
        blockers: Vec::new(),
        next_handoff: Some(next_handoff),
        boundary: PlotIntakeRouteBoundary::advisory_only(),
    }
}

fn clarification_result(body: &str, blocker: PlotIntakeRouteBlocker) -> PlotIntakeRouteResult {
    PlotIntakeRouteResult {
        status: PlotIntakeRouteStatus::ClarificationRequired,
        authority: PlotIntakeRouteAuthority::NonAuthoritative,
        phase_route: Some(PlotIntakePhaseRoute::Clarify),
        module_route: None,
        route_confidence: PlotIntakeRouteConfidence::Low,
        normalized_request: body.trim().to_string(),
        rationale: vec!["request needs clarification before routing.".to_string()],
        blockers: vec![blocker],
        next_handoff: None,
        boundary: PlotIntakeRouteBoundary::advisory_only(),
    }
}

fn not_captured_result(trimmed: &str) -> PlotIntakeRouteResult {
    PlotIntakeRouteResult {
        status: PlotIntakeRouteStatus::NotCaptured,
        authority: PlotIntakeRouteAuthority::NonAuthoritative,
        phase_route: None,
        module_route: None,
        route_confidence: PlotIntakeRouteConfidence::Low,
        normalized_request: trimmed.to_string(),
        rationale: vec!["raw text did not use a supported Punk intake command.".to_string()],
        blockers: Vec::new(),
        next_handoff: None,
        boundary: PlotIntakeRouteBoundary::advisory_only(),
    }
}

fn looks_like_content_request(body: &str) -> bool {
    let normalized = body.to_lowercase();
    [
        "article",
        "blog",
        "content",
        "draft",
        "post",
        "publication",
        "text",
        "write",
        "блог",
        "напиши",
        "написать",
        "пост",
        "публикац",
        "стат",
        "текст",
    ]
    .iter()
    .any(|keyword| normalized.contains(keyword))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserIntentUnknown {
    statement: String,
    blocking: bool,
}

impl UserIntentUnknown {
    pub fn new(statement: impl Into<String>, blocking: bool) -> Self {
        Self {
            statement: statement.into().trim().to_string(),
            blocking,
        }
    }

    pub fn statement(&self) -> &str {
        &self.statement
    }

    pub fn blocking(&self) -> bool {
        self.blocking
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserIntentContractDraftReadiness {
    ReadyForUserConfirmation,
    ClarificationRequired,
    RefusedOrDeferred,
    Blocked,
}

impl UserIntentContractDraftReadiness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReadyForUserConfirmation => "ready_for_user_confirmation",
            Self::ClarificationRequired => "clarification_required",
            Self::RefusedOrDeferred => "refused_or_deferred",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserIntentContractDraftBlocker {
    EmptyRawRequestSummary,
    EmptyIntent,
    BlockingUnknown,
    MissingIncludedScope,
    MissingAcceptanceCriteria,
    MissingEvidencePlan,
    MissingRequiredResearchRefs,
    MissingReceiptExpectation,
    MissingGateDecisionExpectation,
    MissingProofpackExpectation,
    ExecutorClaimsTreatedAsProof,
    UserConfirmationTreatedAsGateAcceptance,
    WriterMayInferIntent,
    RefusedOrDeferred,
}

impl UserIntentContractDraftBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptyRawRequestSummary => "empty_raw_request_summary",
            Self::EmptyIntent => "empty_intent",
            Self::BlockingUnknown => "blocking_unknown",
            Self::MissingIncludedScope => "missing_included_scope",
            Self::MissingAcceptanceCriteria => "missing_acceptance_criteria",
            Self::MissingEvidencePlan => "missing_evidence_plan",
            Self::MissingRequiredResearchRefs => "missing_required_research_refs",
            Self::MissingReceiptExpectation => "missing_receipt_expectation",
            Self::MissingGateDecisionExpectation => "missing_gate_decision_expectation",
            Self::MissingProofpackExpectation => "missing_proofpack_expectation",
            Self::ExecutorClaimsTreatedAsProof => "executor_claims_treated_as_proof",
            Self::UserConfirmationTreatedAsGateAcceptance => {
                "user_confirmation_treated_as_gate_acceptance"
            }
            Self::WriterMayInferIntent => "writer_may_infer_intent",
            Self::RefusedOrDeferred => "refused_or_deferred",
        }
    }

    pub fn requires_clarification(self) -> bool {
        matches!(
            self,
            Self::EmptyRawRequestSummary | Self::EmptyIntent | Self::BlockingUnknown
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserIntentContractDraftAssessment {
    readiness: UserIntentContractDraftReadiness,
    blockers: Vec<UserIntentContractDraftBlocker>,
}

impl UserIntentContractDraftAssessment {
    pub fn readiness(&self) -> UserIntentContractDraftReadiness {
        self.readiness
    }

    pub fn blockers(&self) -> &[UserIntentContractDraftBlocker] {
        &self.blockers
    }

    pub fn ready_for_user_confirmation(&self) -> bool {
        self.readiness == UserIntentContractDraftReadiness::ReadyForUserConfirmation
    }

    pub fn has_blocker(&self, blocker: UserIntentContractDraftBlocker) -> bool {
        self.blockers.contains(&blocker)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserIntentContractDraftModel {
    raw_request_summary: String,
    intent: String,
    scope_include: Vec<String>,
    scope_exclude: Vec<String>,
    non_goals: Vec<String>,
    assumptions: Vec<String>,
    unknowns: Vec<UserIntentUnknown>,
    acceptance_criteria: Vec<String>,
    evidence_plan: Vec<String>,
    research_gate: UserIntentResearchGate,
    side_effect_boundaries: Vec<String>,
    contract_clauses: Vec<ContractClauseBlueprint>,
    receipt_requirements: Vec<ContractReceiptRequirement>,
    gate_input_policy: ContractGateInputPolicy,
    proof_requirements: ContractProofRequirements,
    downstream_closure: UserIntentDownstreamClosure,
    context_pack_refs: Vec<String>,
    refusal_or_deferral_reasons: Vec<String>,
}

impl UserIntentContractDraftModel {
    pub fn new(
        raw_request_summary: impl Into<String>,
        intent: impl Into<String>,
        research_gate: UserIntentResearchGate,
        downstream_closure: UserIntentDownstreamClosure,
    ) -> Self {
        Self {
            raw_request_summary: raw_request_summary.into().trim().to_string(),
            intent: intent.into().trim().to_string(),
            scope_include: Vec::new(),
            scope_exclude: Vec::new(),
            non_goals: Vec::new(),
            assumptions: Vec::new(),
            unknowns: Vec::new(),
            acceptance_criteria: Vec::new(),
            evidence_plan: Vec::new(),
            research_gate,
            side_effect_boundaries: Vec::new(),
            contract_clauses: Vec::new(),
            receipt_requirements: Vec::new(),
            gate_input_policy: contract_gate_input_policy_v0_1(),
            proof_requirements: contract_proof_requirements_v0_1(),
            downstream_closure,
            context_pack_refs: Vec::new(),
            refusal_or_deferral_reasons: Vec::new(),
        }
    }

    pub fn with_scope_include(mut self, scope_ref: impl Into<String>) -> Self {
        push_non_empty(&mut self.scope_include, scope_ref);
        self
    }

    pub fn with_scope_exclude(mut self, scope_ref: impl Into<String>) -> Self {
        push_non_empty(&mut self.scope_exclude, scope_ref);
        self
    }

    pub fn with_non_goal(mut self, non_goal: impl Into<String>) -> Self {
        push_non_empty(&mut self.non_goals, non_goal);
        self
    }

    pub fn with_assumption(mut self, assumption: impl Into<String>) -> Self {
        push_non_empty(&mut self.assumptions, assumption);
        self
    }

    pub fn with_unknown(mut self, unknown: UserIntentUnknown) -> Self {
        if !unknown.statement().is_empty() {
            self.unknowns.push(unknown);
        }
        self
    }

    pub fn with_acceptance_criterion(mut self, criterion: impl Into<String>) -> Self {
        push_non_empty(&mut self.acceptance_criteria, criterion);
        self
    }

    pub fn with_evidence_plan_item(mut self, evidence: impl Into<String>) -> Self {
        push_non_empty(&mut self.evidence_plan, evidence);
        self
    }

    pub fn with_side_effect_boundary(mut self, boundary: impl Into<String>) -> Self {
        push_non_empty(&mut self.side_effect_boundaries, boundary);
        self
    }

    pub fn with_contract_clause(mut self, clause: ContractClauseBlueprint) -> Self {
        if !clause.id.trim().is_empty() {
            self.contract_clauses.push(clause);
        }
        self
    }

    pub fn with_receipt_requirement(mut self, requirement: ContractReceiptRequirement) -> Self {
        if !requirement.field().trim().is_empty() {
            self.receipt_requirements.push(requirement);
        }
        self
    }

    pub fn with_receipt_field_requirement(
        self,
        field: &'static str,
        source: ReceiptRequirementSource,
    ) -> Self {
        self.with_receipt_requirement(ContractReceiptRequirement::new(field, source))
    }

    pub fn with_gate_input_policy(mut self, policy: ContractGateInputPolicy) -> Self {
        self.gate_input_policy = policy;
        self
    }

    pub fn with_proof_requirements(mut self, requirements: ContractProofRequirements) -> Self {
        self.proof_requirements = requirements;
        self
    }

    pub fn with_context_pack_ref(mut self, context_pack_ref: impl Into<String>) -> Self {
        push_non_empty(&mut self.context_pack_refs, context_pack_ref);
        self
    }

    pub fn with_refusal_or_deferral_reason(mut self, reason: impl Into<String>) -> Self {
        push_non_empty(&mut self.refusal_or_deferral_reasons, reason);
        self
    }

    pub fn raw_request_summary(&self) -> &str {
        &self.raw_request_summary
    }

    pub fn intent(&self) -> &str {
        &self.intent
    }

    pub fn scope_include(&self) -> &[String] {
        &self.scope_include
    }

    pub fn scope_exclude(&self) -> &[String] {
        &self.scope_exclude
    }

    pub fn non_goals(&self) -> &[String] {
        &self.non_goals
    }

    pub fn assumptions(&self) -> &[String] {
        &self.assumptions
    }

    pub fn unknowns(&self) -> &[UserIntentUnknown] {
        &self.unknowns
    }

    pub fn acceptance_criteria(&self) -> &[String] {
        &self.acceptance_criteria
    }

    pub fn evidence_plan(&self) -> &[String] {
        &self.evidence_plan
    }

    pub fn research_gate(&self) -> &UserIntentResearchGate {
        &self.research_gate
    }

    pub fn side_effect_boundaries(&self) -> &[String] {
        &self.side_effect_boundaries
    }

    pub fn contract_clauses(&self) -> &[ContractClauseBlueprint] {
        &self.contract_clauses
    }

    pub fn receipt_requirements(&self) -> &[ContractReceiptRequirement] {
        &self.receipt_requirements
    }

    pub fn gate_input_policy(&self) -> &ContractGateInputPolicy {
        &self.gate_input_policy
    }

    pub fn proof_requirements(&self) -> &ContractProofRequirements {
        &self.proof_requirements
    }

    pub fn downstream_closure(&self) -> UserIntentDownstreamClosure {
        self.downstream_closure
    }

    pub fn context_pack_refs(&self) -> &[String] {
        &self.context_pack_refs
    }

    pub fn refusal_or_deferral_reasons(&self) -> &[String] {
        &self.refusal_or_deferral_reasons
    }

    pub fn assess(&self) -> UserIntentContractDraftAssessment {
        let mut blockers = Vec::new();

        if self.raw_request_summary.trim().is_empty() {
            blockers.push(UserIntentContractDraftBlocker::EmptyRawRequestSummary);
        }
        if self.intent.trim().is_empty() {
            blockers.push(UserIntentContractDraftBlocker::EmptyIntent);
        }
        if self.unknowns.iter().any(UserIntentUnknown::blocking) {
            blockers.push(UserIntentContractDraftBlocker::BlockingUnknown);
        }
        if self.scope_include.is_empty() {
            blockers.push(UserIntentContractDraftBlocker::MissingIncludedScope);
        }
        if self.acceptance_criteria.is_empty() {
            blockers.push(UserIntentContractDraftBlocker::MissingAcceptanceCriteria);
        }
        if self.evidence_plan.is_empty() {
            blockers.push(UserIntentContractDraftBlocker::MissingEvidencePlan);
        }
        if !self.research_gate.satisfied() {
            blockers.push(UserIntentContractDraftBlocker::MissingRequiredResearchRefs);
        }
        if !self.downstream_closure.receipt_required {
            blockers.push(UserIntentContractDraftBlocker::MissingReceiptExpectation);
        }
        if !self
            .downstream_closure
            .gate_decision_required_for_acceptance
        {
            blockers.push(UserIntentContractDraftBlocker::MissingGateDecisionExpectation);
        }
        if !self
            .downstream_closure
            .proofpack_required_for_positive_acceptance
        {
            blockers.push(UserIntentContractDraftBlocker::MissingProofpackExpectation);
        }
        if self.downstream_closure.executor_claims_are_proof {
            blockers.push(UserIntentContractDraftBlocker::ExecutorClaimsTreatedAsProof);
        }
        if self.downstream_closure.user_confirmation_is_gate_acceptance {
            blockers.push(UserIntentContractDraftBlocker::UserConfirmationTreatedAsGateAcceptance);
        }
        if self.downstream_closure.writer_may_infer_intent {
            blockers.push(UserIntentContractDraftBlocker::WriterMayInferIntent);
        }
        if !self.refusal_or_deferral_reasons.is_empty() {
            blockers.push(UserIntentContractDraftBlocker::RefusedOrDeferred);
        }

        let readiness = if !self.refusal_or_deferral_reasons.is_empty() {
            UserIntentContractDraftReadiness::RefusedOrDeferred
        } else if blockers
            .iter()
            .any(|blocker| blocker.requires_clarification())
        {
            UserIntentContractDraftReadiness::ClarificationRequired
        } else if blockers.is_empty() {
            UserIntentContractDraftReadiness::ReadyForUserConfirmation
        } else {
            UserIntentContractDraftReadiness::Blocked
        };

        UserIntentContractDraftAssessment {
            readiness,
            blockers,
        }
    }
}

pub const CONTRACT_DRAFT_CONFIRMATION_BOUNDARY_SCHEMA_VERSION: &str =
    "contract-draft-confirmation-boundary.v0.1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContractDraftConfirmationBoundary {
    pub requires_explicit_confirmation: bool,
    pub user_confirmation_is_gate_acceptance: bool,
    pub writes_gate_decision: bool,
    pub creates_proofpack: bool,
    pub invokes_writer: bool,
    pub requires_runtime_storage: bool,
    pub writes_cli_output: bool,
}

pub const fn contract_draft_confirmation_boundary() -> ContractDraftConfirmationBoundary {
    ContractDraftConfirmationBoundary {
        requires_explicit_confirmation: true,
        user_confirmation_is_gate_acceptance: false,
        writes_gate_decision: false,
        creates_proofpack: false,
        invokes_writer: false,
        requires_runtime_storage: false,
        writes_cli_output: false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractDraftUnknownDisposition {
    Resolved,
    ConvertedToAssumption,
    ExplicitlyAcceptedRisk,
}

impl ContractDraftUnknownDisposition {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
            Self::ConvertedToAssumption => "converted_to_assumption",
            Self::ExplicitlyAcceptedRisk => "explicitly_accepted_risk",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractDraftUnknownHandling {
    statement: String,
    disposition: ContractDraftUnknownDisposition,
}

impl ContractDraftUnknownHandling {
    pub fn new(statement: impl Into<String>, disposition: ContractDraftUnknownDisposition) -> Self {
        Self {
            statement: statement.into().trim().to_string(),
            disposition,
        }
    }

    pub fn statement(&self) -> &str {
        &self.statement
    }

    pub fn disposition(&self) -> ContractDraftUnknownDisposition {
        self.disposition
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractDraftApprovalEvidence {
    confirmed_by: String,
    confirmation_ref: Option<String>,
    confirmation_reason: Option<String>,
    confirmed_at: Option<String>,
}

impl ContractDraftApprovalEvidence {
    pub fn new(confirmed_by: impl Into<String>) -> Self {
        Self {
            confirmed_by: confirmed_by.into().trim().to_string(),
            confirmation_ref: None,
            confirmation_reason: None,
            confirmed_at: None,
        }
    }

    pub fn with_confirmation_ref(mut self, confirmation_ref: impl Into<String>) -> Self {
        let confirmation_ref = confirmation_ref.into().trim().to_string();
        if !confirmation_ref.is_empty() {
            self.confirmation_ref = Some(confirmation_ref);
        }
        self
    }

    pub fn with_confirmation_reason(mut self, confirmation_reason: impl Into<String>) -> Self {
        let confirmation_reason = confirmation_reason.into().trim().to_string();
        if !confirmation_reason.is_empty() {
            self.confirmation_reason = Some(confirmation_reason);
        }
        self
    }

    pub fn with_confirmed_at(mut self, confirmed_at: impl Into<String>) -> Self {
        let confirmed_at = confirmed_at.into().trim().to_string();
        if !confirmed_at.is_empty() {
            self.confirmed_at = Some(confirmed_at);
        }
        self
    }

    pub fn confirmed_by(&self) -> &str {
        &self.confirmed_by
    }

    pub fn confirmation_ref(&self) -> Option<&str> {
        self.confirmation_ref.as_deref()
    }

    pub fn confirmation_reason(&self) -> Option<&str> {
        self.confirmation_reason.as_deref()
    }

    pub fn confirmed_at(&self) -> Option<&str> {
        self.confirmed_at.as_deref()
    }

    pub fn explicit(&self) -> bool {
        !self.confirmed_by.trim().is_empty()
            && (self.confirmation_ref.is_some() || self.confirmation_reason.is_some())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ContractDraftConfirmation {
    approval_evidence: Option<ContractDraftApprovalEvidence>,
    unknown_handling: Vec<ContractDraftUnknownHandling>,
}

impl ContractDraftConfirmation {
    pub fn missing() -> Self {
        Self::default()
    }

    pub fn explicit(approval_evidence: ContractDraftApprovalEvidence) -> Self {
        Self {
            approval_evidence: Some(approval_evidence),
            unknown_handling: Vec::new(),
        }
    }

    pub fn with_unknown_handling(mut self, handling: ContractDraftUnknownHandling) -> Self {
        if !handling.statement().is_empty() {
            self.unknown_handling.push(handling);
        }
        self
    }

    pub fn approval_evidence(&self) -> Option<&ContractDraftApprovalEvidence> {
        self.approval_evidence.as_ref()
    }

    pub fn unknown_handling(&self) -> &[ContractDraftUnknownHandling] {
        &self.unknown_handling
    }

    pub fn explicit_confirmation(&self) -> bool {
        self.approval_evidence
            .as_ref()
            .is_some_and(ContractDraftApprovalEvidence::explicit)
    }

    pub fn handling_for(
        &self,
        unknown: &UserIntentUnknown,
    ) -> Option<&ContractDraftUnknownHandling> {
        self.unknown_handling
            .iter()
            .find(|handling| handling.statement() == unknown.statement())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractDraftConfirmationOutcome {
    ConfirmedForRun,
    ConfirmationRequired,
    ClarificationRequired,
    RefusedOrDeferred,
    Blocked,
}

impl ContractDraftConfirmationOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ConfirmedForRun => "confirmed_for_run",
            Self::ConfirmationRequired => "confirmation_required",
            Self::ClarificationRequired => "clarification_required",
            Self::RefusedOrDeferred => "refused_or_deferred",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractDraftConfirmationBlocker {
    MissingExplicitConfirmation,
    DraftRequiresClarification,
    DraftRefusedOrDeferred,
    DraftBlocked,
    UnhandledUnknown,
    UnsupportedHardClause,
    InvalidHardClause,
    MissingRequiredReceiptField,
    UnsupportedReceiptField,
}

impl ContractDraftConfirmationBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingExplicitConfirmation => "missing_explicit_confirmation",
            Self::DraftRequiresClarification => "draft_requires_clarification",
            Self::DraftRefusedOrDeferred => "draft_refused_or_deferred",
            Self::DraftBlocked => "draft_blocked",
            Self::UnhandledUnknown => "unhandled_unknown",
            Self::UnsupportedHardClause => "unsupported_hard_clause",
            Self::InvalidHardClause => "invalid_hard_clause",
            Self::MissingRequiredReceiptField => "missing_required_receipt_field",
            Self::UnsupportedReceiptField => "unsupported_receipt_field",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovedForRunContractDraft {
    status: ContractStatus,
    confirmation_evidence: ContractDraftApprovalEvidence,
    intent: String,
    scope_include: Vec<String>,
    scope_exclude: Vec<String>,
    non_goals: Vec<String>,
    assumptions: Vec<String>,
    resolved_unknowns: Vec<String>,
    accepted_risks: Vec<String>,
    acceptance_criteria: Vec<String>,
    evidence_plan: Vec<String>,
    research_gate: UserIntentResearchGate,
    side_effect_boundaries: Vec<String>,
    contract_clauses: Vec<ContractClauseBlueprint>,
    receipt_requirements: Vec<ContractReceiptRequirement>,
    gate_input_policy: ContractGateInputPolicy,
    proof_requirements: ContractProofRequirements,
    hard_clause_mapping_assessment: HardClauseMappingAssessment,
    receipt_requirement_assessment: ReceiptRequirementAssessment,
    downstream_closure: UserIntentDownstreamClosure,
    context_pack_refs: Vec<String>,
}

impl ApprovedForRunContractDraft {
    fn from_ready_draft(
        draft: &UserIntentContractDraftModel,
        confirmation: &ContractDraftConfirmation,
        hard_clause_mapping_assessment: HardClauseMappingAssessment,
        receipt_requirement_assessment: ReceiptRequirementAssessment,
    ) -> Option<Self> {
        let confirmation_evidence = confirmation.approval_evidence()?.clone();
        let mut assumptions = draft.assumptions().to_vec();
        let mut resolved_unknowns = Vec::new();
        let mut accepted_risks = Vec::new();

        for handling in confirmation.unknown_handling() {
            match handling.disposition() {
                ContractDraftUnknownDisposition::Resolved => {
                    resolved_unknowns.push(handling.statement().to_string());
                }
                ContractDraftUnknownDisposition::ConvertedToAssumption => {
                    assumptions.push(handling.statement().to_string());
                }
                ContractDraftUnknownDisposition::ExplicitlyAcceptedRisk => {
                    accepted_risks.push(handling.statement().to_string());
                }
            }
        }

        Some(Self {
            status: ContractStatus::ApprovedForRun,
            confirmation_evidence,
            intent: draft.intent().to_string(),
            scope_include: draft.scope_include().to_vec(),
            scope_exclude: draft.scope_exclude().to_vec(),
            non_goals: draft.non_goals().to_vec(),
            assumptions,
            resolved_unknowns,
            accepted_risks,
            acceptance_criteria: draft.acceptance_criteria().to_vec(),
            evidence_plan: draft.evidence_plan().to_vec(),
            research_gate: draft.research_gate().clone(),
            side_effect_boundaries: draft.side_effect_boundaries().to_vec(),
            contract_clauses: draft.contract_clauses().to_vec(),
            receipt_requirements: draft.receipt_requirements().to_vec(),
            gate_input_policy: draft.gate_input_policy().clone(),
            proof_requirements: draft.proof_requirements().clone(),
            hard_clause_mapping_assessment,
            receipt_requirement_assessment,
            downstream_closure: draft.downstream_closure(),
            context_pack_refs: draft.context_pack_refs().to_vec(),
        })
    }

    pub fn status(&self) -> ContractStatus {
        self.status
    }

    pub fn confirmation_evidence(&self) -> &ContractDraftApprovalEvidence {
        &self.confirmation_evidence
    }

    pub fn intent(&self) -> &str {
        &self.intent
    }

    pub fn scope_include(&self) -> &[String] {
        &self.scope_include
    }

    pub fn scope_exclude(&self) -> &[String] {
        &self.scope_exclude
    }

    pub fn non_goals(&self) -> &[String] {
        &self.non_goals
    }

    pub fn assumptions(&self) -> &[String] {
        &self.assumptions
    }

    pub fn resolved_unknowns(&self) -> &[String] {
        &self.resolved_unknowns
    }

    pub fn accepted_risks(&self) -> &[String] {
        &self.accepted_risks
    }

    pub fn acceptance_criteria(&self) -> &[String] {
        &self.acceptance_criteria
    }

    pub fn evidence_plan(&self) -> &[String] {
        &self.evidence_plan
    }

    pub fn research_gate(&self) -> &UserIntentResearchGate {
        &self.research_gate
    }

    pub fn side_effect_boundaries(&self) -> &[String] {
        &self.side_effect_boundaries
    }

    pub fn contract_clauses(&self) -> &[ContractClauseBlueprint] {
        &self.contract_clauses
    }

    pub fn receipt_requirements(&self) -> &[ContractReceiptRequirement] {
        &self.receipt_requirements
    }

    pub fn receipt_requirement_assessment(&self) -> &ReceiptRequirementAssessment {
        &self.receipt_requirement_assessment
    }

    pub fn gate_input_policy(&self) -> &ContractGateInputPolicy {
        &self.gate_input_policy
    }

    pub fn proof_requirements(&self) -> &ContractProofRequirements {
        &self.proof_requirements
    }

    pub fn hard_clause_mapping_assessment(&self) -> &HardClauseMappingAssessment {
        &self.hard_clause_mapping_assessment
    }

    pub fn downstream_closure(&self) -> UserIntentDownstreamClosure {
        self.downstream_closure
    }

    pub fn context_pack_refs(&self) -> &[String] {
        &self.context_pack_refs
    }

    pub fn creates_gate_acceptance(&self) -> bool {
        false
    }

    pub fn writes_gate_decision(&self) -> bool {
        false
    }

    pub fn creates_proofpack(&self) -> bool {
        false
    }

    pub fn creates_acceptance_claim(&self) -> bool {
        false
    }

    pub fn invokes_writer(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractDraftConfirmationResult {
    outcome: ContractDraftConfirmationOutcome,
    blockers: Vec<ContractDraftConfirmationBlocker>,
    approved: Option<ApprovedForRunContractDraft>,
    hard_clause_mapping_assessment: Option<HardClauseMappingAssessment>,
    receipt_requirement_assessment: Option<ReceiptRequirementAssessment>,
}

impl ContractDraftConfirmationResult {
    pub fn outcome(&self) -> ContractDraftConfirmationOutcome {
        self.outcome
    }

    pub fn blockers(&self) -> &[ContractDraftConfirmationBlocker] {
        &self.blockers
    }

    pub fn approved(&self) -> Option<&ApprovedForRunContractDraft> {
        self.approved.as_ref()
    }

    pub fn hard_clause_mapping_assessment(&self) -> Option<&HardClauseMappingAssessment> {
        self.hard_clause_mapping_assessment.as_ref()
    }

    pub fn receipt_requirement_assessment(&self) -> Option<&ReceiptRequirementAssessment> {
        self.receipt_requirement_assessment.as_ref()
    }

    pub fn approved_for_run(&self) -> bool {
        self.outcome == ContractDraftConfirmationOutcome::ConfirmedForRun
            && self
                .approved
                .as_ref()
                .is_some_and(|approved| approved.status() == ContractStatus::ApprovedForRun)
    }

    pub fn has_blocker(&self, blocker: ContractDraftConfirmationBlocker) -> bool {
        self.blockers.contains(&blocker)
    }
}

pub fn confirm_contract_draft(
    draft: &UserIntentContractDraftModel,
    confirmation: &ContractDraftConfirmation,
) -> ContractDraftConfirmationResult {
    let assessment = draft.assess();

    match assessment.readiness() {
        UserIntentContractDraftReadiness::ClarificationRequired => {
            return ContractDraftConfirmationResult {
                outcome: ContractDraftConfirmationOutcome::ClarificationRequired,
                blockers: vec![ContractDraftConfirmationBlocker::DraftRequiresClarification],
                approved: None,
                hard_clause_mapping_assessment: None,
                receipt_requirement_assessment: None,
            };
        }
        UserIntentContractDraftReadiness::RefusedOrDeferred => {
            return ContractDraftConfirmationResult {
                outcome: ContractDraftConfirmationOutcome::RefusedOrDeferred,
                blockers: vec![ContractDraftConfirmationBlocker::DraftRefusedOrDeferred],
                approved: None,
                hard_clause_mapping_assessment: None,
                receipt_requirement_assessment: None,
            };
        }
        UserIntentContractDraftReadiness::Blocked => {
            return ContractDraftConfirmationResult {
                outcome: ContractDraftConfirmationOutcome::Blocked,
                blockers: vec![ContractDraftConfirmationBlocker::DraftBlocked],
                approved: None,
                hard_clause_mapping_assessment: None,
                receipt_requirement_assessment: None,
            };
        }
        UserIntentContractDraftReadiness::ReadyForUserConfirmation => {}
    }

    if !confirmation.explicit_confirmation() {
        return ContractDraftConfirmationResult {
            outcome: ContractDraftConfirmationOutcome::ConfirmationRequired,
            blockers: vec![ContractDraftConfirmationBlocker::MissingExplicitConfirmation],
            approved: None,
            hard_clause_mapping_assessment: None,
            receipt_requirement_assessment: None,
        };
    }

    if draft
        .unknowns()
        .iter()
        .any(|unknown| confirmation.handling_for(unknown).is_none())
    {
        return ContractDraftConfirmationResult {
            outcome: ContractDraftConfirmationOutcome::Blocked,
            blockers: vec![ContractDraftConfirmationBlocker::UnhandledUnknown],
            approved: None,
            hard_clause_mapping_assessment: None,
            receipt_requirement_assessment: None,
        };
    }

    let hard_clause_mapping_assessment = assess_hard_clause_mappings(draft.contract_clauses());
    if hard_clause_mapping_assessment.approval_blocking() {
        let mut blockers = Vec::new();
        if hard_clause_mapping_assessment.has_status(HardClauseMappingStatus::UnsupportedBlocking) {
            blockers.push(ContractDraftConfirmationBlocker::UnsupportedHardClause);
        }
        if hard_clause_mapping_assessment.has_status(HardClauseMappingStatus::Invalid) {
            blockers.push(ContractDraftConfirmationBlocker::InvalidHardClause);
        }

        return ContractDraftConfirmationResult {
            outcome: ContractDraftConfirmationOutcome::Blocked,
            blockers,
            approved: None,
            hard_clause_mapping_assessment: Some(hard_clause_mapping_assessment),
            receipt_requirement_assessment: None,
        };
    }

    let receipt_requirement_assessment =
        assess_receipt_requirements(draft.contract_clauses(), draft.receipt_requirements());
    if receipt_requirement_assessment.approval_blocking() {
        let mut blockers = Vec::new();
        if receipt_requirement_assessment.has_status(ReceiptRequirementStatus::MissingRequiredField)
        {
            blockers.push(ContractDraftConfirmationBlocker::MissingRequiredReceiptField);
        }
        if receipt_requirement_assessment.has_status(ReceiptRequirementStatus::UnsupportedField) {
            blockers.push(ContractDraftConfirmationBlocker::UnsupportedReceiptField);
        }

        return ContractDraftConfirmationResult {
            outcome: ContractDraftConfirmationOutcome::Blocked,
            blockers,
            approved: None,
            hard_clause_mapping_assessment: Some(hard_clause_mapping_assessment),
            receipt_requirement_assessment: Some(receipt_requirement_assessment),
        };
    }

    ContractDraftConfirmationResult {
        outcome: ContractDraftConfirmationOutcome::ConfirmedForRun,
        blockers: Vec::new(),
        approved: ApprovedForRunContractDraft::from_ready_draft(
            draft,
            confirmation,
            hard_clause_mapping_assessment.clone(),
            receipt_requirement_assessment.clone(),
        ),
        hard_clause_mapping_assessment: Some(hard_clause_mapping_assessment),
        receipt_requirement_assessment: Some(receipt_requirement_assessment),
    }
}

fn push_non_empty(target: &mut Vec<String>, value: impl Into<String>) {
    let value = value.into().trim().to_string();
    if !value.is_empty() {
        target.push(value);
    }
}

pub fn validate_contract(draft: &ContractDraft) -> Result<(), ContractError> {
    if draft.title.trim().is_empty() {
        return Err(ContractError::EmptyTitle);
    }

    if draft.scope.is_empty() {
        return Err(ContractError::EmptyScope);
    }
    if !draft.scope.refs_are_repo_relative() {
        return Err(ContractError::UnsafeScopeRef);
    }

    Ok(())
}

/// Build the minimal approved-for-run contract value from an already concrete
/// draft.
///
/// This does not replace `confirm_contract_draft`, which owns the full user
/// intent, research gate, hard-clause, and explicit-confirmation closure.
pub fn approve_contract(draft: ContractDraft) -> Result<Contract, ContractError> {
    validate_contract(&draft)?;

    Ok(Contract {
        id: draft.id,
        title: draft.title,
        scope: draft.scope,
        status: ContractStatus::ApprovedForRun,
    })
}

pub fn validate_context_pack_boundary(
    pack: &ContractContextPack,
    required_clause_ids: &[&str],
) -> Result<(), ContextPackError> {
    if pack.contract_ref().trim().is_empty() {
        return Err(ContextPackError::EmptyContractRef);
    }

    for item in pack.items() {
        if item.item_id().trim().is_empty() {
            return Err(ContextPackError::EmptyContextItemId);
        }

        if item.artifact_ref().trim().is_empty() {
            return Err(ContextPackError::EmptyContextItemRef);
        }

        if item.supports().is_empty() {
            return Err(ContextPackError::ContextItemWithoutSupport);
        }

        if item.authority() == ContextAuthority::Speculative {
            return Err(ContextPackError::SpeculativeContextItemIncluded);
        }
    }

    for coverage in pack.clause_coverage() {
        if coverage.clause_id().trim().is_empty() {
            return Err(ContextPackError::EmptyClauseCoverageId);
        }

        if coverage.coverage_type() != ContextCoverageType::Unknown
            && coverage.covered_by().is_empty()
        {
            return Err(ContextPackError::CoverageWithoutBacking);
        }

        for covered_by in coverage.covered_by() {
            let Some(item) = pack
                .items()
                .iter()
                .find(|item| item.item_id() == covered_by.as_str())
            else {
                return Err(ContextPackError::CoverageReferencesMissingContextItem);
            };

            if !item
                .supports()
                .iter()
                .any(|support| support.as_str() == coverage.clause_id())
            {
                return Err(ContextPackError::CoverageBackedByNonSupportingItem);
            }
        }
    }

    for item in pack.items() {
        for support in item.supports() {
            if !pack
                .clause_coverage()
                .iter()
                .any(|coverage| coverage.clause_id() == support.as_str())
            {
                return Err(ContextPackError::ContextItemSupportWithoutCoverage);
            }
        }
    }

    for required_clause_id in required_clause_ids {
        if !pack
            .clause_coverage()
            .iter()
            .any(|coverage| coverage.clause_id() == *required_clause_id)
        {
            return Err(ContextPackError::UnsupportedRequiredClause);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_scope() -> ContractScope {
        ContractScope::new()
            .with_ref("work/goals/goal_add_contract_lifecycle_minimal.md")
            .with_ref("docs/product/CONTRACT-TRACKER.md")
    }

    fn valid_draft() -> ContractDraft {
        ContractDraft::new(
            ContractId::new("contract_local_001").expect("contract id should be valid"),
            "Minimal contract lifecycle kernel",
            valid_scope(),
        )
    }

    fn valid_context_pack() -> ContractContextPack {
        ContractContextPack::new(
            ContextPackId::new("ctxpack_contract_local_001")
                .expect("context pack id should be valid"),
            "contract_local_001",
            ContextPackStatus::Draft,
        )
        .with_item(
            ContextPackItem::new(
                "item_laws",
                "docs/product/PUNK-LAWS.md",
                ContextAuthority::Canonical,
                ContextDocStatus::Active,
            )
            .with_support("rules.only_gate_decides")
            .with_support("rules.proof_before_acceptance"),
        )
        .with_clause_coverage(
            ContextPackClauseCoverage::new("rules.only_gate_decides", ContextCoverageType::Source)
                .with_covered_by("item_laws"),
        )
        .with_clause_coverage(
            ContextPackClauseCoverage::new(
                "rules.proof_before_acceptance",
                ContextCoverageType::Source,
            )
            .with_covered_by("item_laws"),
        )
        .with_clause_coverage(ContextPackClauseCoverage::new(
            "scope.output",
            ContextCoverageType::Unknown,
        ))
        .with_unknown(ContextPackUnknown::new(
            "exact output artifact shape is not selected yet",
            true,
        ))
    }

    fn ready_intent_contract_draft_model() -> UserIntentContractDraftModel {
        UserIntentContractDraftModel::new(
            "User wants a bounded intent-to-contract model.",
            "Add a side-effect-free draft readiness model before runtime UX.",
            UserIntentResearchGate::new(UserIntentResearchGateClassification::R1, true)
                .with_ref("evals/specs/user-intent-to-contract-ux-flow-boundary.v0.1.md"),
            UserIntentDownstreamClosure::default(),
        )
        .with_scope_include("crates/punk-contract/src/lib.rs")
        .with_scope_exclude(".punk/**")
        .with_non_goal("runtime `.punk/contracts` storage")
        .with_assumption("Level 0 done is manual closure with evidence")
        .with_acceptance_criterion("draft readiness is classified without execution")
        .with_evidence_plan_item("cargo test -p punk-contract")
        .with_side_effect_boundary("no runtime storage or CLI behavior")
        .with_context_pack_ref("context-pack:advisory-only")
    }

    fn explicit_confirmation() -> ContractDraftConfirmation {
        ContractDraftConfirmation::explicit(
            ContractDraftApprovalEvidence::new("vitaly")
                .with_confirmation_ref("work/reports/confirmation.md")
                .with_confirmed_at("2026-04-30T12:00:00Z"),
        )
    }

    fn hard_clause(id: &'static str) -> ContractClauseBlueprint {
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

    #[test]
    fn contract_with_empty_scope_is_invalid() {
        let draft = ContractDraft::new(
            ContractId::new("contract_local_002").expect("contract id should be valid"),
            "Scope must be explicit",
            ContractScope::new(),
        );

        assert_eq!(validate_contract(&draft), Err(ContractError::EmptyScope));
    }

    #[test]
    fn contract_with_explicit_scope_is_valid() {
        let draft = valid_draft();

        assert_eq!(validate_contract(&draft), Ok(()));
    }

    #[test]
    fn contract_scope_refs_must_be_repo_relative_for_bounded_work() {
        let draft = ContractDraft::new(
            ContractId::new("contract_local_unsafe_scope").expect("contract id should be valid"),
            "Unsafe scope is not bounded work",
            ContractScope::new()
                .with_ref("work/goals/goal_contract_scope.md")
                .with_ref("../secrets.txt"),
        );

        assert_eq!(
            validate_contract(&draft),
            Err(ContractError::UnsafeScopeRef)
        );
        assert!(!draft.scope().refs_are_repo_relative());
    }

    #[test]
    fn approved_contract_is_ready_for_bounded_work_but_not_final_acceptance() {
        let contract = approve_contract(valid_draft()).expect("contract should approve");
        let boundary = contract_lifecycle_boundary();

        assert_eq!(contract.status(), ContractStatus::ApprovedForRun);
        assert_eq!(contract.status().as_str(), "approved_for_run");
        assert!(contract.ready_for_bounded_work());
        assert!(!boundary.implies_final_acceptance);
    }

    #[test]
    fn contract_lifecycle_does_not_write_gate_decisions_or_create_proofpacks() {
        let boundary = contract_lifecycle_boundary();

        assert!(!boundary.writes_gate_decision);
        assert!(!boundary.creates_proofpack);
    }

    #[test]
    fn contract_lifecycle_requires_no_runtime_storage() {
        let boundary = contract_lifecycle_boundary();

        assert!(!boundary.requires_runtime_storage);
    }

    #[test]
    fn contract_draft_confirmation_boundary_is_side_effect_free() {
        let boundary = contract_draft_confirmation_boundary();

        assert_eq!(
            CONTRACT_DRAFT_CONFIRMATION_BOUNDARY_SCHEMA_VERSION,
            "contract-draft-confirmation-boundary.v0.1"
        );
        assert!(boundary.requires_explicit_confirmation);
        assert!(!boundary.user_confirmation_is_gate_acceptance);
        assert!(!boundary.writes_gate_decision);
        assert!(!boundary.creates_proofpack);
        assert!(!boundary.invokes_writer);
        assert!(!boundary.requires_runtime_storage);
        assert!(!boundary.writes_cli_output);
    }

    #[test]
    fn ready_contract_draft_requires_explicit_confirmation_before_approval() {
        let model = ready_intent_contract_draft_model();
        let result = confirm_contract_draft(&model, &ContractDraftConfirmation::missing());

        assert_eq!(
            result.outcome(),
            ContractDraftConfirmationOutcome::ConfirmationRequired
        );
        assert!(result.has_blocker(ContractDraftConfirmationBlocker::MissingExplicitConfirmation));
        assert!(!result.approved_for_run());
        assert_eq!(
            ContractDraftConfirmationOutcome::ConfirmationRequired.as_str(),
            "confirmation_required"
        );
    }

    #[test]
    fn ready_contract_draft_with_explicit_confirmation_becomes_approved_for_run() {
        let model = ready_intent_contract_draft_model();
        let result = confirm_contract_draft(&model, &explicit_confirmation());
        let approved = result
            .approved()
            .expect("ready draft with explicit confirmation should approve");

        assert_eq!(
            result.outcome(),
            ContractDraftConfirmationOutcome::ConfirmedForRun
        );
        assert!(result.approved_for_run());
        assert_eq!(approved.status(), ContractStatus::ApprovedForRun);
        assert_eq!(approved.intent(), model.intent());
        assert_eq!(approved.scope_include(), model.scope_include());
        assert_eq!(approved.scope_exclude(), model.scope_exclude());
        assert_eq!(approved.non_goals(), model.non_goals());
        assert_eq!(approved.assumptions(), model.assumptions());
        assert_eq!(approved.acceptance_criteria(), model.acceptance_criteria());
        assert_eq!(approved.evidence_plan(), model.evidence_plan());
        assert_eq!(
            approved.research_gate().classification(),
            model.research_gate().classification()
        );
        assert_eq!(
            approved.side_effect_boundaries(),
            model.side_effect_boundaries()
        );
        assert_eq!(approved.downstream_closure(), model.downstream_closure());
        assert_eq!(approved.context_pack_refs(), model.context_pack_refs());
        assert_eq!(
            approved.confirmation_evidence().confirmation_ref(),
            Some("work/reports/confirmation.md")
        );
    }

    #[test]
    fn non_ready_contract_drafts_cannot_be_confirmed_for_run() {
        let clarification = ready_intent_contract_draft_model()
            .with_unknown(UserIntentUnknown::new("target module is unclear", true));
        let refused = ready_intent_contract_draft_model()
            .with_refusal_or_deferral_reason("runtime storage is out of scope");
        let blocked = UserIntentContractDraftModel::new(
            "User asks for bounded work.",
            "Define a boundary.",
            UserIntentResearchGate::new(UserIntentResearchGateClassification::R0, false),
            UserIntentDownstreamClosure::default(),
        );

        let clarification_result = confirm_contract_draft(&clarification, &explicit_confirmation());
        let refused_result = confirm_contract_draft(&refused, &explicit_confirmation());
        let blocked_result = confirm_contract_draft(&blocked, &explicit_confirmation());

        assert_eq!(
            clarification_result.outcome(),
            ContractDraftConfirmationOutcome::ClarificationRequired
        );
        assert_eq!(
            refused_result.outcome(),
            ContractDraftConfirmationOutcome::RefusedOrDeferred
        );
        assert_eq!(
            blocked_result.outcome(),
            ContractDraftConfirmationOutcome::Blocked
        );
        assert!(!clarification_result.approved_for_run());
        assert!(!refused_result.approved_for_run());
        assert!(!blocked_result.approved_for_run());
    }

    #[test]
    fn unresolved_unknowns_block_contract_draft_confirmation() {
        let model = ready_intent_contract_draft_model().with_unknown(UserIntentUnknown::new(
            "deployment target is not final",
            false,
        ));
        let result = confirm_contract_draft(&model, &explicit_confirmation());

        assert_eq!(result.outcome(), ContractDraftConfirmationOutcome::Blocked);
        assert!(result.has_blocker(ContractDraftConfirmationBlocker::UnhandledUnknown));
        assert!(!result.approved_for_run());
        assert_eq!(
            ContractDraftConfirmationBlocker::UnhandledUnknown.as_str(),
            "unhandled_unknown"
        );
    }

    #[test]
    fn unknowns_converted_to_assumptions_allow_contract_draft_confirmation() {
        let unknown = "deployment target is not final";
        let model = ready_intent_contract_draft_model()
            .with_unknown(UserIntentUnknown::new(unknown, false));
        let confirmation =
            explicit_confirmation().with_unknown_handling(ContractDraftUnknownHandling::new(
                unknown,
                ContractDraftUnknownDisposition::ConvertedToAssumption,
            ));
        let result = confirm_contract_draft(&model, &confirmation);
        let approved = result
            .approved()
            .expect("handled unknown should allow confirmation");

        assert!(result.approved_for_run());
        assert!(approved
            .assumptions()
            .iter()
            .any(|assumption| assumption == unknown));
        assert_eq!(
            ContractDraftUnknownDisposition::ConvertedToAssumption.as_str(),
            "converted_to_assumption"
        );
    }

    #[test]
    fn confirmed_contract_draft_does_not_create_gate_proof_or_writer_outputs() {
        let result = confirm_contract_draft(
            &ready_intent_contract_draft_model(),
            &explicit_confirmation(),
        );
        let approved = result
            .approved()
            .expect("ready draft with explicit confirmation should approve");

        assert!(!approved.creates_gate_acceptance());
        assert!(!approved.writes_gate_decision());
        assert!(!approved.creates_proofpack());
        assert!(!approved.creates_acceptance_claim());
        assert!(!approved.invokes_writer());
    }

    #[test]
    fn confirmed_contract_draft_with_mapped_hard_clauses_can_be_approved_for_run() {
        let clause = ContractClauseBlueprint {
            id: "clause.validator",
            has_validator_refs: true,
            ..hard_clause("clause.base")
        };
        let model = ready_intent_contract_draft_model().with_contract_clause(clause);
        let result = confirm_contract_draft(&model, &explicit_confirmation());
        let approved = result
            .approved()
            .expect("mapped hard clause should allow approval");

        assert!(result.approved_for_run());
        assert_eq!(approved.contract_clauses(), model.contract_clauses());
        assert!(approved
            .hard_clause_mapping_assessment()
            .has_status(HardClauseMappingStatus::Mapped));
        assert!(approved
            .hard_clause_mapping_assessment()
            .ready_for_approval());
        assert!(!approved
            .hard_clause_mapping_assessment()
            .executes_validators());
        assert!(!approved
            .hard_clause_mapping_assessment()
            .writes_gate_decision());
        assert!(!approved
            .hard_clause_mapping_assessment()
            .creates_proofpack());
        assert!(!approved.hard_clause_mapping_assessment().invokes_writer());
    }

    #[test]
    fn unmapped_or_invalid_hard_clauses_block_contract_draft_confirmation() {
        let unsupported = ContractClauseBlueprint {
            id: "clause.unsupported",
            unsupported_clause_finding: true,
            ..hard_clause("clause.base")
        };
        let invalid = ContractClauseBlueprint {
            id: "clause.invalid",
            kind: ContractClauseKind::Rationale,
            ..hard_clause("clause.base")
        };
        let model = ready_intent_contract_draft_model()
            .with_contract_clause(unsupported)
            .with_contract_clause(invalid);
        let result = confirm_contract_draft(&model, &explicit_confirmation());
        let assessment = result
            .hard_clause_mapping_assessment()
            .expect("blocked confirmation should surface hard-clause findings");

        assert_eq!(result.outcome(), ContractDraftConfirmationOutcome::Blocked);
        assert!(result.has_blocker(ContractDraftConfirmationBlocker::UnsupportedHardClause));
        assert!(result.has_blocker(ContractDraftConfirmationBlocker::InvalidHardClause));
        assert!(!result.approved_for_run());
        assert!(assessment.approval_blocking());
        assert!(assessment.has_status(HardClauseMappingStatus::UnsupportedBlocking));
        assert!(assessment.has_status(HardClauseMappingStatus::Invalid));
        assert_eq!(
            ContractDraftConfirmationBlocker::UnsupportedHardClause.as_str(),
            "unsupported_hard_clause"
        );
    }

    #[test]
    fn contract_schema_blueprint_has_required_top_level_sections() {
        let blueprint = contract_schema_blueprint_v0_1();

        assert_eq!(blueprint.version(), "contract-schema-blueprint.v0.1");
        assert!(blueprint.has_required_sections());
        for section in CONTRACT_SCHEMA_REQUIRED_SECTIONS {
            assert!(
                blueprint.sections().contains(&section),
                "missing section {}",
                section.as_str()
            );
            assert!(
                blueprint.fields_for_section(section).next().is_some(),
                "section {} should have fields",
                section.as_str()
            );
        }
    }

    #[test]
    fn contract_schema_blueprint_marks_status_required_now_deferred_parked_future() {
        let blueprint = contract_schema_blueprint_v0_1();

        assert!(blueprint.has_all_status_classes());
        assert!(blueprint.has_field(ContractSchemaSection::Lifecycle, "status"));
        assert!(blueprint.has_field(
            ContractSchemaSection::GatePolicy,
            "partial_acceptance_policy"
        ));
        assert!(blueprint.has_field(ContractSchemaSection::Authority, "pki_signature"));
        assert!(blueprint.has_field(ContractSchemaSection::Work, "semantic_intent_recovery"));
    }

    #[test]
    fn contract_status_does_not_include_acceptance_decisions() {
        let status_names = CONTRACT_STATUS_VALUES
            .iter()
            .map(|status| status.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            status_names,
            vec!["draft", "approved_for_run", "superseded", "cancelled"]
        );
        assert!(!status_names.contains(&"accepted"));
        assert!(!status_names.contains(&"rejected"));
        assert!(!status_names.contains(&"partially_accepted"));
        assert!(CONTRACT_STATUS_VALUES
            .iter()
            .all(|status| !status.is_gate_acceptance_decision()));
        assert!(ContractStatus::ApprovedForRun.ready_for_bounded_work());
        assert!(!ContractStatus::Superseded.ready_for_bounded_work());
        assert!(!ContractStatus::Cancelled.ready_for_bounded_work());
    }

    #[test]
    fn plot_intake_punk_blog_request_routes_to_pubpunk_advisory_contract_intake() {
        let request = PlotIntakeRequest::new("/punk напиши пост для блога про X")
            .with_harness_id("codex-cli")
            .with_harness_metadata("opaque-session-ref");

        let route = route_plot_intake_request(&request);

        assert_eq!(route.status(), PlotIntakeRouteStatus::Routed);
        assert_eq!(
            route.authority(),
            PlotIntakeRouteAuthority::NonAuthoritative
        );
        assert_eq!(route.phase_route(), Some(PlotIntakePhaseRoute::Plot));
        assert_eq!(route.module_route(), Some(PlotIntakeModuleRoute::PubPunk));
        assert_eq!(route.route_confidence(), PlotIntakeRouteConfidence::High);
        assert_eq!(route.normalized_request(), "напиши пост для блога про X");
        assert_eq!(route.next_handoff(), Some("pubpunk.contract_intake"));
        assert!(route
            .rationale()
            .iter()
            .any(|reason| reason.contains("content")));
        assert!(!route.boundary().side_effects_active);
        assert!(!route.boundary().module_activated);
        assert!(!route.boundary().gate_decision_written);
        assert!(!route.boundary().proofpack_written);
        assert!(!route.boundary().acceptance_claimed);
    }

    #[test]
    fn plot_intake_empty_punk_command_requires_clarification() {
        let route = route_plot_intake_request(&PlotIntakeRequest::new("/punk   "));

        assert_eq!(route.status(), PlotIntakeRouteStatus::ClarificationRequired);
        assert_eq!(route.phase_route(), Some(PlotIntakePhaseRoute::Clarify));
        assert_eq!(route.module_route(), None);
        assert_eq!(route.normalized_request(), "");
        assert!(route.has_blocker(PlotIntakeRouteBlocker::MissingRequestBody));
        assert!(!route.boundary().side_effects_active);
        assert!(!route.boundary().module_activated);
    }

    #[test]
    fn plot_intake_plain_text_is_not_captured() {
        let route = route_plot_intake_request(&PlotIntakeRequest::new("напиши пост для блога"));

        assert_eq!(route.status(), PlotIntakeRouteStatus::NotCaptured);
        assert_eq!(route.phase_route(), None);
        assert_eq!(route.module_route(), None);
        assert_eq!(route.normalized_request(), "напиши пост для блога");
        assert!(!route.boundary().side_effects_active);
        assert!(!route.boundary().module_activated);
    }

    #[test]
    fn plot_intake_pub_and_dev_shortcuts_are_non_authoritative_hints() {
        let pub_route = route_plot_intake_request(&PlotIntakeRequest::new("/pub напиши пост"));
        let dev_route = route_plot_intake_request(&PlotIntakeRequest::new("/dev поправь тест"));

        assert_eq!(pub_route.status(), PlotIntakeRouteStatus::Routed);
        assert_eq!(pub_route.phase_route(), Some(PlotIntakePhaseRoute::Plot));
        assert_eq!(
            pub_route.module_route(),
            Some(PlotIntakeModuleRoute::PubPunk)
        );
        assert_eq!(
            pub_route.route_confidence(),
            PlotIntakeRouteConfidence::Medium
        );
        assert!(!pub_route.boundary().route_hint_is_authority);
        assert_eq!(pub_route.next_handoff(), Some("pubpunk.contract_intake"));

        assert_eq!(dev_route.status(), PlotIntakeRouteStatus::Routed);
        assert_eq!(dev_route.phase_route(), Some(PlotIntakePhaseRoute::Plot));
        assert_eq!(
            dev_route.module_route(),
            Some(PlotIntakeModuleRoute::DevPunk)
        );
        assert_eq!(
            dev_route.route_confidence(),
            PlotIntakeRouteConfidence::Medium
        );
        assert!(!dev_route.boundary().route_hint_is_authority);
        assert_eq!(dev_route.next_handoff(), Some("devpunk.contract_intake"));
    }

    #[test]
    fn hard_clause_requires_validator_receipt_proof_or_gate_review_mapping() {
        let unmapped = hard_clause("clause.unmapped");
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
            human_gate_review_reason: Some("manual review required for non-machine clause"),
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

        assert!(!unmapped.hard_clause_mapping_satisfied());
        assert!(validator_mapped.hard_clause_mapping_satisfied());
        assert!(receipt_mapped.hard_clause_mapping_satisfied());
        assert!(proof_mapped.hard_clause_mapping_satisfied());
        assert!(gate_mapped.hard_clause_mapping_satisfied());
        assert!(!unsupported_mapped.hard_clause_mapping_satisfied());
        assert!(advisory_unmapped.hard_clause_mapping_satisfied());

        for clause in [
            unmapped,
            validator_mapped,
            receipt_mapped,
            proof_mapped,
            gate_mapped,
            unsupported_mapped,
            advisory_unmapped,
        ] {
            assert_eq!(
                clause.hard_clause_mapping_satisfied(),
                !assess_hard_clause_mapping(&clause).blocks_approval(),
                "hard clause mapping predicate drifted for {}",
                clause.id
            );
        }
    }

    #[test]
    fn required_receipt_field_flag_defaults_to_artifact_hashes_when_empty() {
        let clause = ContractClauseBlueprint {
            id: "clause.receipt-default",
            has_required_receipt_fields: true,
            required_receipt_fields: &[],
            ..hard_clause("clause.base")
        };

        assert_eq!(clause.required_receipt_fields(), &["artifact_hashes"]);
        assert!(clause.hard_clause_mapping_satisfied());
    }

    #[test]
    fn hard_clause_mapping_assessment_blocks_unsupported_or_invalid_hard_clauses() {
        let validator_mapped = ContractClauseBlueprint {
            id: "clause.validator",
            has_validator_refs: true,
            ..hard_clause("clause.base")
        };
        let receipt_mapped = ContractClauseBlueprint {
            id: "clause.receipt",
            has_required_receipt_fields: true,
            ..hard_clause("clause.base")
        };
        let proof_mapped = ContractClauseBlueprint {
            id: "clause.proof",
            has_proof_requirement_refs: true,
            ..hard_clause("clause.base")
        };
        let human_review = ContractClauseBlueprint {
            id: "clause.human",
            gate_review_required: true,
            human_gate_review_reason: Some("requires manual policy review"),
            ..hard_clause("clause.base")
        };
        let blank_human_review = ContractClauseBlueprint {
            id: "clause.blank-human",
            gate_review_required: true,
            human_gate_review_reason: Some("   "),
            ..hard_clause("clause.base")
        };
        let unsupported = ContractClauseBlueprint {
            id: "clause.unsupported",
            unsupported_clause_finding: true,
            ..hard_clause("clause.base")
        };
        let rationale = ContractClauseBlueprint {
            id: "clause.rationale",
            kind: ContractClauseKind::Rationale,
            ..hard_clause("clause.base")
        };
        let soft = ContractClauseBlueprint {
            id: "clause.soft",
            severity: ContractClauseSeverity::Soft,
            ..hard_clause("clause.base")
        };

        let assessment = assess_hard_clause_mappings(&[
            validator_mapped,
            receipt_mapped,
            proof_mapped,
            human_review,
            blank_human_review,
            unsupported,
            rationale,
            soft,
        ]);

        assert!(assessment.approval_blocking());
        assert!(assessment.has_status(HardClauseMappingStatus::Mapped));
        assert!(assessment.has_status(HardClauseMappingStatus::RequiresHumanGateReview));
        assert!(assessment.has_status(HardClauseMappingStatus::UnsupportedBlocking));
        assert!(assessment.has_status(HardClauseMappingStatus::Invalid));
        assert!(assessment.has_status(HardClauseMappingStatus::NotRequiredForSoftOrAdvisory));
        assert!(!assessment.executes_validators());
        assert!(!assessment.writes_gate_decision());
        assert!(!assessment.creates_proofpack());
        assert!(!assessment.invokes_writer());
        assert_eq!(
            HardClauseMappingStatus::UnsupportedBlocking.as_str(),
            "unsupported_blocking"
        );
        assert_eq!(
            HardClauseMappingTarget::ValidatorRef.as_str(),
            "validator_ref"
        );
    }

    #[test]
    fn proof_requirements_do_not_create_proofpack() {
        let blueprint = contract_proof_requirements_blueprint();
        let proof_requirements = contract_proof_requirements_v0_1();
        let assessment = assess_proof_requirements(&proof_requirements);

        assert!(blueprint.must_link.contains(&"contract_ref"));
        assert!(blueprint.must_link.contains(&"run_receipt_ref"));
        assert!(blueprint.must_link.contains(&"gate_decision_ref"));
        assert!(blueprint.must_hash.contains(&"contract_hash"));
        assert!(blueprint.must_hash.contains(&"output_artifact_hashes"));
        assert!(!blueprint.creates_proofpack);
        assert!(!blueprint.proofpack_is_decision_authority);
        assert!(proof_requirements.requires_target("contract_ref"));
        assert!(proof_requirements.requires_target("run_receipt_ref"));
        assert!(proof_requirements.requires_target("gate_decision_ref"));
        assert!(proof_requirements.requires_target("output_artifact_refs"));
        assert!(proof_requirements.requires_target("output_artifact_hashes"));
        assert!(assessment.proof_requirements_declared());
        assert!(!assessment.proofpack_ready_after_gate());
        assert!(!proof_requirements.creates_proofpack());
        assert!(!proof_requirements.writes_punk_proofs_storage());
        assert!(!proof_requirements.computes_artifact_hashes_from_filesystem());
        assert!(!proof_requirements.writes_gate_decision());
        assert!(!proof_requirements.creates_acceptance_claim());
        assert!(!proof_requirements.invokes_writer());
        assert!(!proof_requirements.requires_proofpack_before_gate());
        assert!(!assessment.creates_proofpack());
        assert!(!assessment.writes_punk_proofs_storage());
        assert!(!assessment.computes_artifact_hashes_from_filesystem());
        assert!(!assessment.writes_gate_decision());
        assert!(!assessment.creates_acceptance_claim());
        assert!(!assessment.invokes_writer());
        assert!(!assessment.requires_proofpack_before_gate());
    }

    #[test]
    fn gate_policy_does_not_write_decision() {
        let gate_policy = contract_gate_policy_blueprint();

        assert!(gate_policy.gate_only_final_decision);
        assert!(!gate_policy.writes_decision);
        assert!(gate_policy.required_inputs.contains(&"contract_ref"));
        assert!(gate_policy
            .required_inputs
            .contains(&"approved_for_run_status"));
        assert!(gate_policy.required_inputs.contains(&"run_receipt_ref"));
        assert!(!gate_policy.required_inputs.contains(&"proofpack"));
        assert!(!gate_policy.required_inputs.contains(&"proofpack_ref"));
        assert!(gate_policy.hard_fail_on.contains(&"unmapped_hard_clause"));
        assert!(gate_policy
            .accept_requires
            .contains(&"satisfied_hard_clauses"));
    }

    #[test]
    fn gate_input_policy_assesses_readiness_without_proofpack_input() {
        let policy = contract_gate_input_policy_v0_1();
        let evidence = CONTRACT_GATE_INPUT_POLICY_BASELINE_REQUIREMENTS
            .iter()
            .map(|input| GateInputEvidence::new(*input))
            .collect::<Vec<_>>();
        let assessment = assess_gate_input_policy(&policy, &evidence);

        assert!(policy.requires_input("contract_ref"));
        assert!(policy.requires_input("approved_for_run_status"));
        assert!(policy.requires_input("run_receipt_ref"));
        assert!(!policy.requires_existing_proofpack());
        assert!(policy.post_gate_proof_requirement_pending());
        assert!(assessment.ready_for_gate());
        assert!(!assessment.requires_existing_proofpack());
        assert!(!assessment.writes_gate_decision());
        assert!(!assessment.creates_proofpack());
        assert!(!assessment.creates_acceptance_claim());
        assert!(!assessment.invokes_writer());
    }

    #[test]
    fn writer_is_not_upstream_contract_authority() {
        let boundary = contract_writer_authority_boundary();

        assert!(!boundary.writer_is_upstream_planner);
        assert!(!boundary.writer_defines_contract);
        assert!(!boundary.writer_writes_gate_decision);
        assert!(!boundary.writer_claims_acceptance);
    }

    #[test]
    fn context_pack_boundary_is_side_effect_free_and_advisory() {
        let boundary = context_pack_boundary();

        assert!(!boundary.writes_gate_decision);
        assert!(!boundary.owns_project_truth);
        assert!(!boundary.requires_runtime_storage);
        assert!(!boundary.allows_executor_override);
        assert!(!boundary.activates_retrieval_adapter);
    }

    #[test]
    fn context_pack_with_clause_coverage_is_valid() {
        let pack = valid_context_pack();

        assert_eq!(
            validate_context_pack_boundary(&pack, &["rules.only_gate_decides", "scope.output"]),
            Ok(())
        );
    }

    #[test]
    fn context_pack_accepts_valid_bidirectional_support_coverage_mapping() {
        let pack = valid_context_pack();

        assert_eq!(
            validate_context_pack_boundary(
                &pack,
                &[
                    "rules.only_gate_decides",
                    "rules.proof_before_acceptance",
                    "scope.output"
                ]
            ),
            Ok(())
        );
    }

    #[test]
    fn context_pack_rejects_orphan_context_item() {
        let pack = ContractContextPack::new(
            ContextPackId::new("ctxpack_contract_local_002")
                .expect("context pack id should be valid"),
            "contract_local_002",
            ContextPackStatus::Draft,
        )
        .with_item(ContextPackItem::new(
            "item_orphan",
            "docs/product/ARCHITECTURE.md",
            ContextAuthority::Canonical,
            ContextDocStatus::Active,
        ));

        assert_eq!(
            validate_context_pack_boundary(&pack, &["architecture.boundary"]),
            Err(ContextPackError::ContextItemWithoutSupport)
        );
    }

    #[test]
    fn context_pack_rejects_speculative_included_item_by_default() {
        let pack = ContractContextPack::new(
            ContextPackId::new("ctxpack_contract_local_003")
                .expect("context pack id should be valid"),
            "contract_local_003",
            ContextPackStatus::Draft,
        )
        .with_item(
            ContextPackItem::new(
                "item_speculative",
                "knowledge/ideas/future-agent.md",
                ContextAuthority::Speculative,
                ContextDocStatus::Unknown,
            )
            .with_support("scope.future"),
        );

        assert_eq!(
            validate_context_pack_boundary(&pack, &["scope.future"]),
            Err(ContextPackError::SpeculativeContextItemIncluded)
        );
    }

    #[test]
    fn context_pack_rejects_coverage_referencing_missing_item_id() {
        let pack = valid_context_pack().with_clause_coverage(
            ContextPackClauseCoverage::new("validators.required", ContextCoverageType::Source)
                .with_covered_by("item_missing"),
        );

        assert_eq!(
            validate_context_pack_boundary(
                &pack,
                &[
                    "rules.only_gate_decides",
                    "scope.output",
                    "validators.required"
                ]
            ),
            Err(ContextPackError::CoverageReferencesMissingContextItem)
        );
    }

    #[test]
    fn context_pack_rejects_item_support_without_clause_coverage() {
        let pack = valid_context_pack().with_item(
            ContextPackItem::new(
                "item_architecture",
                "docs/product/ARCHITECTURE.md",
                ContextAuthority::Canonical,
                ContextDocStatus::Active,
            )
            .with_support("architecture.boundary"),
        );

        assert_eq!(
            validate_context_pack_boundary(&pack, &["rules.only_gate_decides", "scope.output"]),
            Err(ContextPackError::ContextItemSupportWithoutCoverage)
        );
    }

    #[test]
    fn context_pack_rejects_coverage_backed_by_non_supporting_item() {
        let pack = valid_context_pack().with_clause_coverage(
            ContextPackClauseCoverage::new("validators.required", ContextCoverageType::Source)
                .with_covered_by("item_laws"),
        );

        assert_eq!(
            validate_context_pack_boundary(
                &pack,
                &[
                    "rules.only_gate_decides",
                    "scope.output",
                    "validators.required"
                ]
            ),
            Err(ContextPackError::CoverageBackedByNonSupportingItem)
        );
    }

    #[test]
    fn context_pack_rejects_unsupported_required_clause() {
        let pack = valid_context_pack();

        assert_eq!(
            validate_context_pack_boundary(
                &pack,
                &["rules.only_gate_decides", "validators.required"]
            ),
            Err(ContextPackError::UnsupportedRequiredClause)
        );
    }

    #[test]
    fn user_intent_contract_draft_model_can_be_ready_for_confirmation() {
        let model = ready_intent_contract_draft_model();
        let assessment = model.assess();

        assert_eq!(
            USER_INTENT_CONTRACT_DRAFT_MODEL_SCHEMA_VERSION,
            "user-intent-contract-draft-model.v0.1"
        );
        assert_eq!(model.research_gate().classification().as_str(), "R1");
        assert_eq!(
            assessment.readiness(),
            UserIntentContractDraftReadiness::ReadyForUserConfirmation
        );
        assert_eq!(
            assessment.readiness().as_str(),
            "ready_for_user_confirmation"
        );
        assert!(assessment.ready_for_user_confirmation());
        assert!(assessment.blockers().is_empty());
    }

    #[test]
    fn user_intent_contract_draft_model_preserves_boundary_flags() {
        let boundary = user_intent_contract_draft_boundary();

        assert!(!boundary.raw_request_is_contract);
        assert!(!boundary.intent_is_execution_permission);
        assert!(!boundary.user_confirmation_is_gate_acceptance);
        assert!(!boundary.executor_claims_are_proof);
        assert!(!boundary.context_pack_defines_acceptance);
        assert!(boundary.writer_is_downstream_only);
        assert!(!boundary.writes_runtime_storage);
        assert!(!boundary.writes_gate_decision);
        assert!(!boundary.writes_proofpack);
    }

    #[test]
    fn user_intent_contract_draft_model_requires_clarification_for_blocking_unknowns() {
        let model = ready_intent_contract_draft_model()
            .with_unknown(UserIntentUnknown::new("target module is unclear", true));
        let assessment = model.assess();

        assert_eq!(
            assessment.readiness(),
            UserIntentContractDraftReadiness::ClarificationRequired
        );
        assert!(assessment.has_blocker(UserIntentContractDraftBlocker::BlockingUnknown));
        assert_eq!(
            UserIntentContractDraftBlocker::BlockingUnknown.as_str(),
            "blocking_unknown"
        );
    }

    #[test]
    fn user_intent_contract_draft_model_refuses_or_defers_explicit_stop_reasons() {
        let model = ready_intent_contract_draft_model()
            .with_refusal_or_deferral_reason("request requires runtime storage outside scope");
        let assessment = model.assess();

        assert_eq!(
            assessment.readiness(),
            UserIntentContractDraftReadiness::RefusedOrDeferred
        );
        assert!(assessment.has_blocker(UserIntentContractDraftBlocker::RefusedOrDeferred));
    }

    #[test]
    fn user_intent_contract_draft_model_blocks_missing_evidence_research_or_scope() {
        let model = UserIntentContractDraftModel::new(
            "User asks for implementation.",
            "Implement a bounded model.",
            UserIntentResearchGate::new(UserIntentResearchGateClassification::R1, true),
            UserIntentDownstreamClosure::default(),
        )
        .with_acceptance_criterion("model classifies readiness");
        let assessment = model.assess();

        assert_eq!(
            assessment.readiness(),
            UserIntentContractDraftReadiness::Blocked
        );
        assert!(assessment.has_blocker(UserIntentContractDraftBlocker::MissingIncludedScope));
        assert!(assessment.has_blocker(UserIntentContractDraftBlocker::MissingEvidencePlan));
        assert!(assessment.has_blocker(UserIntentContractDraftBlocker::MissingRequiredResearchRefs));
    }

    #[test]
    fn context_pack_refs_do_not_define_acceptance_criteria() {
        let model = UserIntentContractDraftModel::new(
            "User asks for a contract draft.",
            "Draft bounded work.",
            UserIntentResearchGate::new(UserIntentResearchGateClassification::R0, false),
            UserIntentDownstreamClosure::default(),
        )
        .with_scope_include("work/goals/example.md")
        .with_evidence_plan_item("manual report")
        .with_context_pack_ref("ctxpack:advisory-only");
        let assessment = model.assess();

        assert_eq!(
            assessment.readiness(),
            UserIntentContractDraftReadiness::Blocked
        );
        assert!(assessment.has_blocker(UserIntentContractDraftBlocker::MissingAcceptanceCriteria));
    }

    #[test]
    fn user_intent_contract_draft_model_rejects_downstream_authority_drift() {
        let model = UserIntentContractDraftModel::new(
            "User asks for proof artifact behavior.",
            "Keep Writer downstream.",
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
        .with_evidence_plan_item("cargo test -p punk-contract");
        let assessment = model.assess();

        assert_eq!(
            assessment.readiness(),
            UserIntentContractDraftReadiness::Blocked
        );
        assert!(
            assessment.has_blocker(UserIntentContractDraftBlocker::ExecutorClaimsTreatedAsProof)
        );
        assert!(assessment
            .has_blocker(UserIntentContractDraftBlocker::UserConfirmationTreatedAsGateAcceptance));
        assert!(assessment.has_blocker(UserIntentContractDraftBlocker::WriterMayInferIntent));
    }
}
