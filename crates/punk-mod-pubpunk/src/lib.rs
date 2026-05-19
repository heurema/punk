//! Incubating side-effect-free PubPunk module models.
//!
//! This crate models publication inventory assessment from caller-provided
//! metadata only. It does not read files, write receipts, call external APIs,
//! read credentials, invoke adapters, expose CLI behavior, write gate
//! decisions, write proofpacks, or claim acceptance.

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const PUBPUNK_MODULE_ID: &str = "pubpunk";
pub const PUBPUNK_INVENTORY_ASSESSMENT_SCHEMA_VERSION: &str =
    "punk.pubpunk.inventory_assessment.v0.1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkAssessmentAuthority {
    Advisory,
}

impl PubPunkAssessmentAuthority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Advisory => "advisory",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkAssessmentStatus {
    Ready,
    Blocked,
}

impl PubPunkAssessmentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkInventoryOperation {
    AssessInventory,
}

impl PubPunkInventoryOperation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AssessInventory => "assess_inventory",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkInventoryItemKind {
    PostDraft,
    PreviewDraft,
    PublicationReceipt,
    MetricsSnapshot,
    Other,
}

impl PubPunkInventoryItemKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PostDraft => "post_draft",
            Self::PreviewDraft => "preview_draft",
            Self::PublicationReceipt => "publication_receipt",
            Self::MetricsSnapshot => "metrics_snapshot",
            Self::Other => "other",
        }
    }

    pub fn is_publication_candidate(self) -> bool {
        matches!(self, Self::PostDraft | Self::PreviewDraft)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkInventoryItemStatus {
    Draft,
    ReadyForReview,
    PreviewOnly,
    Published,
    History,
    Unknown,
}

impl PubPunkInventoryItemStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::ReadyForReview => "ready_for_review",
            Self::PreviewOnly => "preview_only",
            Self::Published => "published",
            Self::History => "history",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unpublished_candidate(self) -> bool {
        matches!(self, Self::Draft | Self::ReadyForReview | Self::PreviewOnly)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkCapabilityGrant {
    AssessProvidedInventory,
    ReadWorkspaceMetadata,
    ReadDraftFile,
    WriteDraftArtifact,
    WriteReceiptProposal,
    RequestExternalPublish,
    RequestMetricsCollection,
    ReadCredentials,
    InvokeAdapter,
    WriteGateDecision,
    WriteProofpack,
    CreateAcceptanceClaim,
}

impl PubPunkCapabilityGrant {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AssessProvidedInventory => "assess_provided_inventory",
            Self::ReadWorkspaceMetadata => "read_workspace_metadata",
            Self::ReadDraftFile => "read_draft_file",
            Self::WriteDraftArtifact => "write_draft_artifact",
            Self::WriteReceiptProposal => "write_receipt_proposal",
            Self::RequestExternalPublish => "request_external_publish",
            Self::RequestMetricsCollection => "request_metrics_collection",
            Self::ReadCredentials => "read_credentials",
            Self::InvokeAdapter => "invoke_adapter",
            Self::WriteGateDecision => "write_gate_decision",
            Self::WriteProofpack => "write_proofpack",
            Self::CreateAcceptanceClaim => "create_acceptance_claim",
        }
    }

    pub fn supported_by_side_effect_free_assessment(self) -> bool {
        matches!(self, Self::AssessProvidedInventory)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PubPunkPrivacyPolicy {
    pub raw_post_bodies: bool,
    pub raw_external_payloads: bool,
    pub secrets_or_credentials: bool,
    pub customer_data: bool,
    pub sensitive_code: bool,
    pub raw_prompts: bool,
}

impl PubPunkPrivacyPolicy {
    pub const fn safe_metadata_only() -> Self {
        Self {
            raw_post_bodies: false,
            raw_external_payloads: false,
            secrets_or_credentials: false,
            customer_data: false,
            sensitive_code: false,
            raw_prompts: false,
        }
    }

    pub fn allows_private_or_raw_payloads(self) -> bool {
        self.raw_post_bodies
            || self.raw_external_payloads
            || self.secrets_or_credentials
            || self.customer_data
            || self.sensitive_code
            || self.raw_prompts
    }
}

impl Default for PubPunkPrivacyPolicy {
    fn default() -> Self {
        Self::safe_metadata_only()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PubPunkInventoryBoundaryFlags {
    pub public_cli: bool,
    pub reads_files: bool,
    pub writes_files: bool,
    pub writes_publication_receipts: bool,
    pub calls_external_apis: bool,
    pub opens_browser: bool,
    pub reads_credentials: bool,
    pub invokes_adapters: bool,
    pub writes_gate_decision: bool,
    pub writes_proofpack: bool,
    pub creates_acceptance_claim: bool,
    pub external_side_effects: bool,
}

impl PubPunkInventoryBoundaryFlags {
    pub const fn side_effect_free_assessment() -> Self {
        Self {
            public_cli: false,
            reads_files: false,
            writes_files: false,
            writes_publication_receipts: false,
            calls_external_apis: false,
            opens_browser: false,
            reads_credentials: false,
            invokes_adapters: false,
            writes_gate_decision: false,
            writes_proofpack: false,
            creates_acceptance_claim: false,
            external_side_effects: false,
        }
    }

    pub fn all_side_effect_flags_false(self) -> bool {
        !self.public_cli
            && !self.reads_files
            && !self.writes_files
            && !self.writes_publication_receipts
            && !self.calls_external_apis
            && !self.opens_browser
            && !self.reads_credentials
            && !self.invokes_adapters
            && !self.writes_gate_decision
            && !self.writes_proofpack
            && !self.creates_acceptance_claim
            && !self.external_side_effects
    }
}

pub const PUBPUNK_INVENTORY_ASSESSMENT_BOUNDARY_FLAGS: PubPunkInventoryBoundaryFlags =
    PubPunkInventoryBoundaryFlags::side_effect_free_assessment();

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkInventoryItemInput {
    pub source_ref: String,
    pub kind: PubPunkInventoryItemKind,
    pub status: PubPunkInventoryItemStatus,
    pub channel: Option<String>,
    pub has_publication_receipt: bool,
    pub raw_body_provided: bool,
}

impl PubPunkInventoryItemInput {
    pub fn new(
        source_ref: impl Into<String>,
        kind: PubPunkInventoryItemKind,
        status: PubPunkInventoryItemStatus,
    ) -> Self {
        Self {
            source_ref: source_ref.into(),
            kind,
            status,
            channel: None,
            has_publication_receipt: false,
            raw_body_provided: false,
        }
    }

    pub fn with_channel(mut self, channel: impl Into<String>) -> Self {
        self.channel = Some(channel.into());
        self
    }

    pub fn with_publication_receipt(mut self, has_publication_receipt: bool) -> Self {
        self.has_publication_receipt = has_publication_receipt;
        self
    }

    pub fn with_raw_body_provided(mut self, raw_body_provided: bool) -> Self {
        self.raw_body_provided = raw_body_provided;
        self
    }

    pub fn is_publication_candidate(&self) -> bool {
        self.kind.is_publication_candidate() && self.status.is_unpublished_candidate()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkInventoryInput {
    pub module_id: String,
    pub module_version: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub publishing_workspace_ref: String,
    pub requested_operation: PubPunkInventoryOperation,
    pub items: Vec<PubPunkInventoryItemInput>,
    pub granted_capabilities: Vec<PubPunkCapabilityGrant>,
    pub privacy_policy: PubPunkPrivacyPolicy,
    pub expected_receipt_fields: Vec<String>,
}

impl PubPunkInventoryInput {
    pub fn new(
        module_version: impl Into<String>,
        contract_ref: impl Into<String>,
        run_ref: impl Into<String>,
        project_ref: impl Into<String>,
        publishing_workspace_ref: impl Into<String>,
    ) -> Self {
        Self {
            module_id: PUBPUNK_MODULE_ID.to_owned(),
            module_version: module_version.into(),
            contract_ref: contract_ref.into(),
            run_ref: run_ref.into(),
            project_ref: project_ref.into(),
            publishing_workspace_ref: publishing_workspace_ref.into(),
            requested_operation: PubPunkInventoryOperation::AssessInventory,
            items: Vec::new(),
            granted_capabilities: Vec::new(),
            privacy_policy: PubPunkPrivacyPolicy::safe_metadata_only(),
            expected_receipt_fields: Vec::new(),
        }
    }

    pub fn with_items(mut self, items: Vec<PubPunkInventoryItemInput>) -> Self {
        self.items = items;
        self
    }

    pub fn with_granted_capabilities(
        mut self,
        granted_capabilities: Vec<PubPunkCapabilityGrant>,
    ) -> Self {
        self.granted_capabilities = granted_capabilities;
        self
    }

    pub fn with_privacy_policy(mut self, privacy_policy: PubPunkPrivacyPolicy) -> Self {
        self.privacy_policy = privacy_policy;
        self
    }

    pub fn with_expected_receipt_fields(
        mut self,
        expected_receipt_fields: Vec<impl Into<String>>,
    ) -> Self {
        self.expected_receipt_fields = expected_receipt_fields
            .into_iter()
            .map(Into::into)
            .collect();
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkInventoryFindingCode {
    MissingModuleId,
    MissingModuleVersion,
    MissingContractRef,
    MissingRunRef,
    MissingProjectRef,
    MissingPublishingWorkspaceRef,
    MissingExpectedReceiptFields,
    UnsafeSourceRef,
    RawBodyProvided,
    UnsafePrivacyPolicy,
    UnsupportedCapabilityGrant,
    CandidateNeedsReceiptAfterPublication,
}

impl PubPunkInventoryFindingCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingModuleId => "missing_module_id",
            Self::MissingModuleVersion => "missing_module_version",
            Self::MissingContractRef => "missing_contract_ref",
            Self::MissingRunRef => "missing_run_ref",
            Self::MissingProjectRef => "missing_project_ref",
            Self::MissingPublishingWorkspaceRef => "missing_publishing_workspace_ref",
            Self::MissingExpectedReceiptFields => "missing_expected_receipt_fields",
            Self::UnsafeSourceRef => "unsafe_source_ref",
            Self::RawBodyProvided => "raw_body_provided",
            Self::UnsafePrivacyPolicy => "unsafe_privacy_policy",
            Self::UnsupportedCapabilityGrant => "unsupported_capability_grant",
            Self::CandidateNeedsReceiptAfterPublication => {
                "candidate_needs_receipt_after_publication"
            }
        }
    }

    pub fn is_blocking(self) -> bool {
        !matches!(self, Self::CandidateNeedsReceiptAfterPublication)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkInventoryFinding {
    pub code: PubPunkInventoryFindingCode,
    pub source_ref: Option<String>,
    pub capability: Option<PubPunkCapabilityGrant>,
    pub message: &'static str,
}

impl PubPunkInventoryFinding {
    fn new(code: PubPunkInventoryFindingCode, message: &'static str) -> Self {
        Self {
            code,
            source_ref: None,
            capability: None,
            message,
        }
    }

    fn for_source(
        code: PubPunkInventoryFindingCode,
        source_ref: impl Into<String>,
        message: &'static str,
    ) -> Self {
        Self {
            code,
            source_ref: Some(source_ref.into()),
            capability: None,
            message,
        }
    }

    fn for_capability(capability: PubPunkCapabilityGrant) -> Self {
        Self {
            code: PubPunkInventoryFindingCode::UnsupportedCapabilityGrant,
            source_ref: None,
            capability: Some(capability),
            message: "capability is not available in the side-effect-free assessment slice",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkInventoryAssessmentRefs {
    pub module_id: String,
    pub module_version: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub publishing_workspace_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkInventoryAssessment {
    pub schema_version: &'static str,
    pub status: PubPunkAssessmentStatus,
    pub authority: PubPunkAssessmentAuthority,
    pub requested_operation: PubPunkInventoryOperation,
    pub candidate_count: usize,
    pub receipt_gap_count: usize,
    pub findings: Vec<PubPunkInventoryFinding>,
    pub boundary_flags: PubPunkInventoryBoundaryFlags,
    pub refs: PubPunkInventoryAssessmentRefs,
}

impl PubPunkInventoryAssessment {
    pub fn blocking_findings(&self) -> impl Iterator<Item = &PubPunkInventoryFinding> {
        self.findings
            .iter()
            .filter(|finding| finding.code.is_blocking())
    }

    pub fn has_blockers(&self) -> bool {
        self.blocking_findings().next().is_some()
    }
}

pub fn assess_pubpunk_inventory(input: &PubPunkInventoryInput) -> PubPunkInventoryAssessment {
    let mut findings = Vec::new();

    push_required_ref_finding(
        &mut findings,
        input.module_id.as_str(),
        PubPunkInventoryFindingCode::MissingModuleId,
        "module id is required",
    );
    push_required_ref_finding(
        &mut findings,
        input.module_version.as_str(),
        PubPunkInventoryFindingCode::MissingModuleVersion,
        "module version is required",
    );
    push_required_ref_finding(
        &mut findings,
        input.contract_ref.as_str(),
        PubPunkInventoryFindingCode::MissingContractRef,
        "contract ref is required",
    );
    push_required_ref_finding(
        &mut findings,
        input.run_ref.as_str(),
        PubPunkInventoryFindingCode::MissingRunRef,
        "run ref is required",
    );
    push_required_ref_finding(
        &mut findings,
        input.project_ref.as_str(),
        PubPunkInventoryFindingCode::MissingProjectRef,
        "project ref is required",
    );
    push_required_ref_finding(
        &mut findings,
        input.publishing_workspace_ref.as_str(),
        PubPunkInventoryFindingCode::MissingPublishingWorkspaceRef,
        "publishing workspace ref is required",
    );

    if input.expected_receipt_fields.is_empty() {
        findings.push(PubPunkInventoryFinding::new(
            PubPunkInventoryFindingCode::MissingExpectedReceiptFields,
            "expected receipt fields are required even though this slice does not write receipts",
        ));
    }

    for capability in &input.granted_capabilities {
        if !capability.supported_by_side_effect_free_assessment() {
            findings.push(PubPunkInventoryFinding::for_capability(*capability));
        }
    }

    if input.privacy_policy.allows_private_or_raw_payloads() {
        findings.push(PubPunkInventoryFinding::new(
            PubPunkInventoryFindingCode::UnsafePrivacyPolicy,
            "privacy policy must disallow raw/private payloads for this assessment slice",
        ));
    }

    let mut candidate_count = 0;
    let mut receipt_gap_count = 0;

    for item in &input.items {
        if !is_safe_source_ref(&item.source_ref) {
            findings.push(PubPunkInventoryFinding::for_source(
                PubPunkInventoryFindingCode::UnsafeSourceRef,
                item.source_ref.clone(),
                "source refs must be explicit repo-relative metadata refs",
            ));
        }

        if item.raw_body_provided {
            findings.push(PubPunkInventoryFinding::for_source(
                PubPunkInventoryFindingCode::RawBodyProvided,
                item.source_ref.clone(),
                "raw post bodies are not accepted by the inventory assessment model",
            ));
        }

        if item.is_publication_candidate() {
            candidate_count += 1;
            if !item.has_publication_receipt {
                receipt_gap_count += 1;
                findings.push(PubPunkInventoryFinding::for_source(
                    PubPunkInventoryFindingCode::CandidateNeedsReceiptAfterPublication,
                    item.source_ref.clone(),
                    "publication receipt must be created only after an actual publication event",
                ));
            }
        }
    }

    let status = if findings.iter().any(|finding| finding.code.is_blocking()) {
        PubPunkAssessmentStatus::Blocked
    } else {
        PubPunkAssessmentStatus::Ready
    };

    PubPunkInventoryAssessment {
        schema_version: PUBPUNK_INVENTORY_ASSESSMENT_SCHEMA_VERSION,
        status,
        authority: PubPunkAssessmentAuthority::Advisory,
        requested_operation: input.requested_operation,
        candidate_count,
        receipt_gap_count,
        findings,
        boundary_flags: PUBPUNK_INVENTORY_ASSESSMENT_BOUNDARY_FLAGS,
        refs: PubPunkInventoryAssessmentRefs {
            module_id: input.module_id.clone(),
            module_version: input.module_version.clone(),
            contract_ref: input.contract_ref.clone(),
            run_ref: input.run_ref.clone(),
            project_ref: input.project_ref.clone(),
            publishing_workspace_ref: input.publishing_workspace_ref.clone(),
        },
    }
}

fn push_required_ref_finding(
    findings: &mut Vec<PubPunkInventoryFinding>,
    value: &str,
    code: PubPunkInventoryFindingCode,
    message: &'static str,
) {
    if value.trim().is_empty() {
        findings.push(PubPunkInventoryFinding::new(code, message));
    }
}

fn is_safe_source_ref(value: &str) -> bool {
    let value = value.trim();
    !value.is_empty()
        && !value.starts_with('/')
        && !value.starts_with('~')
        && !value.contains('\\')
        && !value.contains("://")
        && !value.split('/').any(|segment| {
            segment.is_empty()
                || matches!(segment, "." | "..")
                || segment.chars().any(char::is_control)
        })
}

#[cfg(test)]
mod tests {
    use super::{
        assess_pubpunk_inventory, PubPunkAssessmentAuthority, PubPunkAssessmentStatus,
        PubPunkCapabilityGrant, PubPunkInventoryFindingCode, PubPunkInventoryInput,
        PubPunkInventoryItemInput, PubPunkInventoryItemKind, PubPunkInventoryItemStatus,
        PubPunkPrivacyPolicy,
    };

    fn valid_input() -> PubPunkInventoryInput {
        PubPunkInventoryInput::new(
            "v0.1",
            "contracts/publish-cycle-0",
            "runs/local-pubpunk-inventory",
            "project/punk",
            "punk-publishing://project/punk",
        )
        .with_expected_receipt_fields(vec![
            "module_id",
            "operation",
            "source_refs",
            "capability_grants",
            "side_effects",
        ])
    }

    #[test]
    fn assessment_counts_candidates_and_receipt_gaps_without_side_effects() {
        let input = valid_input().with_items(vec![
            PubPunkInventoryItemInput::new(
                "publishing/posts/example.md",
                PubPunkInventoryItemKind::PostDraft,
                PubPunkInventoryItemStatus::ReadyForReview,
            ),
            PubPunkInventoryItemInput::new(
                "publishing/publications/example.md",
                PubPunkInventoryItemKind::PublicationReceipt,
                PubPunkInventoryItemStatus::Published,
            )
            .with_publication_receipt(true),
        ]);

        let assessment = assess_pubpunk_inventory(&input);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Ready);
        assert_eq!(assessment.authority, PubPunkAssessmentAuthority::Advisory);
        assert_eq!(assessment.candidate_count, 1);
        assert_eq!(assessment.receipt_gap_count, 1);
        assert!(assessment.boundary_flags.all_side_effect_flags_false());
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkInventoryFindingCode::CandidateNeedsReceiptAfterPublication));
        assert!(!assessment.has_blockers());
    }

    #[test]
    fn assessment_blocks_unsupported_capability_grants() {
        let input = valid_input().with_granted_capabilities(vec![
            PubPunkCapabilityGrant::AssessProvidedInventory,
            PubPunkCapabilityGrant::RequestExternalPublish,
        ]);

        let assessment = assess_pubpunk_inventory(&input);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkInventoryFindingCode::UnsupportedCapabilityGrant
            && finding.capability == Some(PubPunkCapabilityGrant::RequestExternalPublish)));
        assert!(assessment.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn assessment_blocks_raw_body_payloads() {
        let input = valid_input()
            .with_privacy_policy(PubPunkPrivacyPolicy {
                raw_post_bodies: true,
                ..PubPunkPrivacyPolicy::safe_metadata_only()
            })
            .with_items(vec![PubPunkInventoryItemInput::new(
                "publishing/posts/example.md",
                PubPunkInventoryItemKind::PostDraft,
                PubPunkInventoryItemStatus::Draft,
            )
            .with_raw_body_provided(true)]);

        let assessment = assess_pubpunk_inventory(&input);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment
            .findings
            .iter()
            .any(|finding| finding.code == PubPunkInventoryFindingCode::UnsafePrivacyPolicy));
        assert!(assessment
            .findings
            .iter()
            .any(|finding| finding.code == PubPunkInventoryFindingCode::RawBodyProvided));
        assert!(assessment.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn assessment_blocks_unsafe_source_refs() {
        let input = valid_input().with_items(vec![PubPunkInventoryItemInput::new(
            "../publishing/posts/example.md",
            PubPunkInventoryItemKind::PostDraft,
            PubPunkInventoryItemStatus::Draft,
        )]);

        let assessment = assess_pubpunk_inventory(&input);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment
            .findings
            .iter()
            .any(|finding| finding.code == PubPunkInventoryFindingCode::UnsafeSourceRef));
        assert!(assessment.boundary_flags.all_side_effect_flags_false());
    }
}
