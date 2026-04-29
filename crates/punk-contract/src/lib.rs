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
}

impl ContractStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::ApprovedForRun => "approved_for_run",
        }
    }

    pub fn ready_for_bounded_work(self) -> bool {
        matches!(self, Self::ApprovedForRun)
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
        !self.title.trim().is_empty() && !self.scope.is_empty()
    }

    pub fn scope_valid(&self) -> bool {
        !self.scope.is_empty()
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

pub fn validate_contract(draft: &ContractDraft) -> Result<(), ContractError> {
    if draft.title.trim().is_empty() {
        return Err(ContractError::EmptyTitle);
    }

    if draft.scope.is_empty() {
        return Err(ContractError::EmptyScope);
    }

    Ok(())
}

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
}
