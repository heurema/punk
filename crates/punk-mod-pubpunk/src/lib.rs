//! Incubating side-effect-free PubPunk module models.
//!
//! This crate models publication inventory and connector handoff packets from
//! caller-provided metadata only. It does not read files, write receipts, call
//! external APIs, read credentials, invoke adapters, expose CLI behavior, write
//! gate decisions, write proofpacks, or claim acceptance.

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const PUBPUNK_MODULE_ID: &str = "pubpunk";
pub const PUBPUNK_INVENTORY_READER_SCHEMA_VERSION: &str = "punk.pubpunk.inventory_reader.v0.1";
pub const PUBPUNK_INVENTORY_INPUT_PACKET_SCHEMA_VERSION: &str =
    "punk.pubpunk.inventory_input_packet.v0.1";
pub const PUBPUNK_INVENTORY_ASSESSMENT_SCHEMA_VERSION: &str =
    "punk.pubpunk.inventory_assessment.v0.1";
pub const PUBPUNK_CHANNEL_CONNECTOR_PROFILE_RESOLUTION_SCHEMA_VERSION: &str =
    "punk.pubpunk.channel_connector_profile_resolution.v0.1";
pub const PUBPUNK_PUBLISH_REQUEST_PACKET_SCHEMA_VERSION: &str =
    "punk.pubpunk.publish_request_packet.v0.1";
pub const PUBPUNK_PUBLISH_RECEIPT_PREFLIGHT_PACKET_SCHEMA_VERSION: &str =
    "punk.pubpunk.publish_receipt_preflight_packet.v0.1";
pub const PUBPUNK_PUBLISH_RECEIPT_WRITE_HANDOFF_PACKET_SCHEMA_VERSION: &str =
    "punk.pubpunk.publish_receipt_write_handoff_packet.v0.1";
pub const PUBPUNK_PUBLISH_OPERATION_EVIDENCE_HANDOFF_PACKET_SCHEMA_VERSION: &str =
    "punk.pubpunk.publish_operation_evidence_handoff_packet.v0.1";
pub const PUBPUNK_PUBLISH_RECEIPT_EVIDENCE_EVENT_HANDOFF_PACKET_SCHEMA_VERSION: &str =
    "punk.pubpunk.publish_receipt_evidence_event_handoff_packet.v0.1";
pub const PUBPUNK_REQUIRED_INSTRUCTION_REFS: &[&str] = &[
    "docs/modules/pubpunk.md",
    "docs/modules/pubpunk-workspace-instructions.md",
    "docs/product/MODULE-AUTHORING.md",
    "docs/product/MODULE-CONFORMANCE.md",
    "docs/product/MODULE-HOST-CONTRACT.md",
    "docs/product/INSTRUCTION-SOURCES.md",
    "publishing/README.md",
];

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
    ResolveConnectorProfile,
    ReadDraftFile,
    WriteDraftArtifact,
    WriteReceiptProposal,
    RequestPublicationReceiptWrite,
    RequestOperationEvidenceWrite,
    RequestReceiptEvidenceEventHandoff,
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
            Self::ResolveConnectorProfile => "resolve_connector_profile",
            Self::ReadDraftFile => "read_draft_file",
            Self::WriteDraftArtifact => "write_draft_artifact",
            Self::WriteReceiptProposal => "write_receipt_proposal",
            Self::RequestPublicationReceiptWrite => "request_publication_receipt_write",
            Self::RequestOperationEvidenceWrite => "request_operation_evidence_write",
            Self::RequestReceiptEvidenceEventHandoff => "request_receipt_evidence_event_handoff",
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

    pub fn supported_by_side_effect_free_reader(self) -> bool {
        matches!(self, Self::ReadWorkspaceMetadata)
    }

    pub fn supported_by_side_effect_free_connector_profile_resolution(self) -> bool {
        matches!(self, Self::ResolveConnectorProfile)
    }

    pub fn supported_by_side_effect_free_publish_request(self) -> bool {
        matches!(self, Self::RequestExternalPublish)
    }

    pub fn supported_by_side_effect_free_publish_receipt_preflight(self) -> bool {
        matches!(self, Self::RequestExternalPublish)
    }

    pub fn supported_by_side_effect_free_publish_receipt_write_handoff(self) -> bool {
        matches!(self, Self::RequestPublicationReceiptWrite)
    }

    pub fn supported_by_side_effect_free_publish_operation_evidence_handoff(self) -> bool {
        matches!(self, Self::RequestOperationEvidenceWrite)
    }

    pub fn supported_by_side_effect_free_publish_receipt_evidence_event_handoff(self) -> bool {
        matches!(self, Self::RequestReceiptEvidenceEventHandoff)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkWorkspacePolicy {
    SplitExplicitRefs,
    RepoNativeOnly,
    ExternalWorkspaceOnly,
    GlobalPunk,
    ProjectPunk,
}

impl PubPunkWorkspacePolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SplitExplicitRefs => "split_explicit_refs",
            Self::RepoNativeOnly => "repo_native_only",
            Self::ExternalWorkspaceOnly => "external_workspace_only",
            Self::GlobalPunk => "global_punk",
            Self::ProjectPunk => "project_punk",
        }
    }

    pub fn selected_for_first_slice(self) -> bool {
        matches!(self, Self::SplitExplicitRefs)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkInventoryReaderInput {
    pub module_id: String,
    pub module_version_ref: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub workspace_policy: PubPunkWorkspacePolicy,
    pub publishing_workspace_ref: String,
    pub allowed_source_refs: Vec<String>,
    pub instruction_refs: Vec<String>,
    pub observed_items: Vec<PubPunkInventoryItemInput>,
    pub granted_capabilities: Vec<PubPunkCapabilityGrant>,
    pub privacy_policy: PubPunkPrivacyPolicy,
    pub expected_receipt_fields: Vec<String>,
    pub token_cost_ref: Option<String>,
}

impl PubPunkInventoryReaderInput {
    pub fn new(
        module_version_ref: impl Into<String>,
        contract_ref: impl Into<String>,
        run_ref: impl Into<String>,
        project_ref: impl Into<String>,
        publishing_workspace_ref: impl Into<String>,
    ) -> Self {
        Self {
            module_id: PUBPUNK_MODULE_ID.to_owned(),
            module_version_ref: module_version_ref.into(),
            contract_ref: contract_ref.into(),
            run_ref: run_ref.into(),
            project_ref: project_ref.into(),
            workspace_policy: PubPunkWorkspacePolicy::SplitExplicitRefs,
            publishing_workspace_ref: publishing_workspace_ref.into(),
            allowed_source_refs: Vec::new(),
            instruction_refs: Vec::new(),
            observed_items: Vec::new(),
            granted_capabilities: Vec::new(),
            privacy_policy: PubPunkPrivacyPolicy::safe_metadata_only(),
            expected_receipt_fields: Vec::new(),
            token_cost_ref: None,
        }
    }

    pub fn with_workspace_policy(mut self, workspace_policy: PubPunkWorkspacePolicy) -> Self {
        self.workspace_policy = workspace_policy;
        self
    }

    pub fn with_allowed_source_refs(mut self, allowed_source_refs: Vec<impl Into<String>>) -> Self {
        self.allowed_source_refs = allowed_source_refs.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_instruction_refs(mut self, instruction_refs: Vec<impl Into<String>>) -> Self {
        self.instruction_refs = instruction_refs.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_observed_items(mut self, observed_items: Vec<PubPunkInventoryItemInput>) -> Self {
        self.observed_items = observed_items;
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

    pub fn with_token_cost_ref(mut self, token_cost_ref: impl Into<String>) -> Self {
        self.token_cost_ref = Some(token_cost_ref.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkInventoryReaderFindingCode {
    MissingModuleId,
    NonCanonicalModuleId,
    MissingModuleVersionRef,
    MissingContractRef,
    MissingRunRef,
    MissingProjectRef,
    UnsupportedWorkspacePolicy,
    MissingPublishingWorkspaceRef,
    UnsafePublishingWorkspaceRef,
    MissingInstructionRefs,
    MissingRequiredInstructionRef,
    UnsafeInstructionRef,
    UnsafeAllowedSourceRef,
    MissingReadWorkspaceMetadataGrant,
    UnsupportedCapabilityGrant,
    MissingExpectedReceiptFields,
    UnsafePrivacyPolicy,
    UnsafeObservedItemSourceRef,
    ObservedItemSourceRefNotAllowed,
    RawBodyProvided,
    UnsafeTokenCostRef,
}

impl PubPunkInventoryReaderFindingCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingModuleId => "missing_module_id",
            Self::NonCanonicalModuleId => "non_canonical_module_id",
            Self::MissingModuleVersionRef => "missing_module_version_ref",
            Self::MissingContractRef => "missing_contract_ref",
            Self::MissingRunRef => "missing_run_ref",
            Self::MissingProjectRef => "missing_project_ref",
            Self::UnsupportedWorkspacePolicy => "unsupported_workspace_policy",
            Self::MissingPublishingWorkspaceRef => "missing_publishing_workspace_ref",
            Self::UnsafePublishingWorkspaceRef => "unsafe_publishing_workspace_ref",
            Self::MissingInstructionRefs => "missing_instruction_refs",
            Self::MissingRequiredInstructionRef => "missing_required_instruction_ref",
            Self::UnsafeInstructionRef => "unsafe_instruction_ref",
            Self::UnsafeAllowedSourceRef => "unsafe_allowed_source_ref",
            Self::MissingReadWorkspaceMetadataGrant => "missing_read_workspace_metadata_grant",
            Self::UnsupportedCapabilityGrant => "unsupported_capability_grant",
            Self::MissingExpectedReceiptFields => "missing_expected_receipt_fields",
            Self::UnsafePrivacyPolicy => "unsafe_privacy_policy",
            Self::UnsafeObservedItemSourceRef => "unsafe_observed_item_source_ref",
            Self::ObservedItemSourceRefNotAllowed => "observed_item_source_ref_not_allowed",
            Self::RawBodyProvided => "raw_body_provided",
            Self::UnsafeTokenCostRef => "unsafe_token_cost_ref",
        }
    }

    pub fn is_blocking(self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkInventoryReaderFinding {
    pub code: PubPunkInventoryReaderFindingCode,
    pub ref_value: Option<String>,
    pub capability: Option<PubPunkCapabilityGrant>,
    pub message: &'static str,
}

impl PubPunkInventoryReaderFinding {
    fn new(code: PubPunkInventoryReaderFindingCode, message: &'static str) -> Self {
        Self {
            code,
            ref_value: None,
            capability: None,
            message,
        }
    }

    fn for_ref(
        code: PubPunkInventoryReaderFindingCode,
        ref_value: impl Into<String>,
        message: &'static str,
    ) -> Self {
        Self {
            code,
            ref_value: Some(ref_value.into()),
            capability: None,
            message,
        }
    }

    fn for_capability(capability: PubPunkCapabilityGrant) -> Self {
        Self {
            code: PubPunkInventoryReaderFindingCode::UnsupportedCapabilityGrant,
            ref_value: None,
            capability: Some(capability),
            message: "capability is not available in the side-effect-free reader slice",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkInventoryReaderRefs {
    pub module_id: String,
    pub module_version_ref: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub workspace_policy: PubPunkWorkspacePolicy,
    pub publishing_workspace_ref: String,
    pub token_cost_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkInventoryReaderAssessment {
    pub schema_version: &'static str,
    pub status: PubPunkAssessmentStatus,
    pub authority: PubPunkAssessmentAuthority,
    pub observed_item_count: usize,
    pub candidate_count: usize,
    pub receipt_count: usize,
    pub metrics_snapshot_count: usize,
    pub unclear_item_count: usize,
    pub findings: Vec<PubPunkInventoryReaderFinding>,
    pub boundary_flags: PubPunkInventoryBoundaryFlags,
    pub refs: PubPunkInventoryReaderRefs,
}

impl PubPunkInventoryReaderAssessment {
    pub fn blocking_findings(&self) -> impl Iterator<Item = &PubPunkInventoryReaderFinding> {
        self.findings
            .iter()
            .filter(|finding| finding.code.is_blocking())
    }

    pub fn has_blockers(&self) -> bool {
        self.blocking_findings().next().is_some()
    }
}

pub fn assess_pubpunk_inventory_reader_input(
    input: &PubPunkInventoryReaderInput,
) -> PubPunkInventoryReaderAssessment {
    let mut findings = Vec::new();

    push_reader_required_ref_finding(
        &mut findings,
        input.module_id.as_str(),
        PubPunkInventoryReaderFindingCode::MissingModuleId,
        "module id is required",
    );
    if !input.module_id.trim().is_empty() && input.module_id != PUBPUNK_MODULE_ID {
        findings.push(PubPunkInventoryReaderFinding::for_ref(
            PubPunkInventoryReaderFindingCode::NonCanonicalModuleId,
            input.module_id.clone(),
            "PubPunk inventory readers must use the canonical pubpunk module id",
        ));
    }
    push_reader_required_ref_finding(
        &mut findings,
        input.module_version_ref.as_str(),
        PubPunkInventoryReaderFindingCode::MissingModuleVersionRef,
        "module version ref is required",
    );
    push_reader_required_ref_finding(
        &mut findings,
        input.contract_ref.as_str(),
        PubPunkInventoryReaderFindingCode::MissingContractRef,
        "contract ref is required",
    );
    push_reader_required_ref_finding(
        &mut findings,
        input.run_ref.as_str(),
        PubPunkInventoryReaderFindingCode::MissingRunRef,
        "run ref is required",
    );
    push_reader_required_ref_finding(
        &mut findings,
        input.project_ref.as_str(),
        PubPunkInventoryReaderFindingCode::MissingProjectRef,
        "project ref is required",
    );

    if !input.workspace_policy.selected_for_first_slice() {
        findings.push(PubPunkInventoryReaderFinding::for_ref(
            PubPunkInventoryReaderFindingCode::UnsupportedWorkspacePolicy,
            input.workspace_policy.as_str(),
            "the first PubPunk reader slice only supports split explicit refs",
        ));
    }

    push_reader_required_ref_finding(
        &mut findings,
        input.publishing_workspace_ref.as_str(),
        PubPunkInventoryReaderFindingCode::MissingPublishingWorkspaceRef,
        "publishing workspace ref is required",
    );
    if !input.publishing_workspace_ref.trim().is_empty()
        && !is_safe_workspace_ref(&input.publishing_workspace_ref)
    {
        findings.push(PubPunkInventoryReaderFinding::for_ref(
            PubPunkInventoryReaderFindingCode::UnsafePublishingWorkspaceRef,
            input.publishing_workspace_ref.clone(),
            "publishing workspace ref must be an explicit safe logical or repo-relative ref",
        ));
    }

    if input.instruction_refs.is_empty() {
        findings.push(PubPunkInventoryReaderFinding::new(
            PubPunkInventoryReaderFindingCode::MissingInstructionRefs,
            "instruction refs are required for PubPunk reader inputs",
        ));
    }

    for required_ref in PUBPUNK_REQUIRED_INSTRUCTION_REFS {
        if !input
            .instruction_refs
            .iter()
            .any(|instruction_ref| instruction_ref == required_ref)
        {
            findings.push(PubPunkInventoryReaderFinding::for_ref(
                PubPunkInventoryReaderFindingCode::MissingRequiredInstructionRef,
                *required_ref,
                "required PubPunk instruction ref is missing",
            ));
        }
    }

    for instruction_ref in &input.instruction_refs {
        if !is_safe_source_ref(instruction_ref) {
            findings.push(PubPunkInventoryReaderFinding::for_ref(
                PubPunkInventoryReaderFindingCode::UnsafeInstructionRef,
                instruction_ref.clone(),
                "instruction refs must be explicit repo-relative refs",
            ));
        }
    }

    for source_ref in &input.allowed_source_refs {
        if !is_safe_source_ref(source_ref) {
            findings.push(PubPunkInventoryReaderFinding::for_ref(
                PubPunkInventoryReaderFindingCode::UnsafeAllowedSourceRef,
                source_ref.clone(),
                "allowed source refs must be explicit repo-relative refs",
            ));
        }
    }

    if !input
        .granted_capabilities
        .contains(&PubPunkCapabilityGrant::ReadWorkspaceMetadata)
    {
        findings.push(PubPunkInventoryReaderFinding::new(
            PubPunkInventoryReaderFindingCode::MissingReadWorkspaceMetadataGrant,
            "read_workspace_metadata must be explicitly granted for this reader input",
        ));
    }
    for capability in &input.granted_capabilities {
        if !capability.supported_by_side_effect_free_reader() {
            findings.push(PubPunkInventoryReaderFinding::for_capability(*capability));
        }
    }

    if input.expected_receipt_fields.is_empty() {
        findings.push(PubPunkInventoryReaderFinding::new(
            PubPunkInventoryReaderFindingCode::MissingExpectedReceiptFields,
            "expected receipt fields are required even though this reader does not write receipts",
        ));
    }

    if input.privacy_policy.allows_private_or_raw_payloads() {
        findings.push(PubPunkInventoryReaderFinding::new(
            PubPunkInventoryReaderFindingCode::UnsafePrivacyPolicy,
            "privacy policy must disallow raw/private payloads for this reader slice",
        ));
    }

    let mut candidate_count = 0;
    let mut receipt_count = 0;
    let mut metrics_snapshot_count = 0;
    let mut unclear_item_count = 0;

    for item in &input.observed_items {
        if !is_safe_source_ref(&item.source_ref) {
            findings.push(PubPunkInventoryReaderFinding::for_ref(
                PubPunkInventoryReaderFindingCode::UnsafeObservedItemSourceRef,
                item.source_ref.clone(),
                "observed item refs must be explicit repo-relative metadata refs",
            ));
        }
        if !input.allowed_source_refs.contains(&item.source_ref) {
            findings.push(PubPunkInventoryReaderFinding::for_ref(
                PubPunkInventoryReaderFindingCode::ObservedItemSourceRefNotAllowed,
                item.source_ref.clone(),
                "observed item refs must be included in allowed source refs",
            ));
        }
        if item.raw_body_provided {
            findings.push(PubPunkInventoryReaderFinding::for_ref(
                PubPunkInventoryReaderFindingCode::RawBodyProvided,
                item.source_ref.clone(),
                "raw post bodies are not accepted by the PubPunk reader model",
            ));
        }

        match item.kind {
            PubPunkInventoryItemKind::PostDraft | PubPunkInventoryItemKind::PreviewDraft => {
                candidate_count += 1;
            }
            PubPunkInventoryItemKind::PublicationReceipt => {
                receipt_count += 1;
            }
            PubPunkInventoryItemKind::MetricsSnapshot => {
                metrics_snapshot_count += 1;
            }
            PubPunkInventoryItemKind::Other => {
                unclear_item_count += 1;
            }
        }
    }

    if let Some(token_cost_ref) = &input.token_cost_ref {
        if !is_safe_source_ref(token_cost_ref) {
            findings.push(PubPunkInventoryReaderFinding::for_ref(
                PubPunkInventoryReaderFindingCode::UnsafeTokenCostRef,
                token_cost_ref.clone(),
                "token cost ref must be an explicit repo-relative ref when provided",
            ));
        }
    }

    let status = if findings.iter().any(|finding| finding.code.is_blocking()) {
        PubPunkAssessmentStatus::Blocked
    } else {
        PubPunkAssessmentStatus::Ready
    };

    PubPunkInventoryReaderAssessment {
        schema_version: PUBPUNK_INVENTORY_READER_SCHEMA_VERSION,
        status,
        authority: PubPunkAssessmentAuthority::Advisory,
        observed_item_count: input.observed_items.len(),
        candidate_count,
        receipt_count,
        metrics_snapshot_count,
        unclear_item_count,
        findings,
        boundary_flags: PUBPUNK_INVENTORY_ASSESSMENT_BOUNDARY_FLAGS,
        refs: PubPunkInventoryReaderRefs {
            module_id: input.module_id.clone(),
            module_version_ref: input.module_version_ref.clone(),
            contract_ref: input.contract_ref.clone(),
            run_ref: input.run_ref.clone(),
            project_ref: input.project_ref.clone(),
            workspace_policy: input.workspace_policy,
            publishing_workspace_ref: input.publishing_workspace_ref.clone(),
            token_cost_ref: input.token_cost_ref.clone(),
        },
    }
}

pub fn build_pubpunk_inventory_input_packet_from_reader_input(
    input: &PubPunkInventoryReaderInput,
) -> Result<PubPunkInventoryInputPacket, PubPunkInventoryReaderAssessment> {
    let assessment = assess_pubpunk_inventory_reader_input(input);
    if assessment.has_blockers() {
        return Err(assessment);
    }

    let mut packet = PubPunkInventoryInputPacket::new(
        input.module_version_ref.clone(),
        input.contract_ref.clone(),
        input.run_ref.clone(),
        input.project_ref.clone(),
        input.publishing_workspace_ref.clone(),
    )
    .with_workspace_policy(input.workspace_policy)
    .with_allowed_source_refs(input.allowed_source_refs.clone())
    .with_instruction_refs(input.instruction_refs.clone())
    .with_items(input.observed_items.clone())
    .with_granted_capabilities(vec![PubPunkCapabilityGrant::AssessProvidedInventory])
    .with_privacy_policy(input.privacy_policy)
    .with_expected_receipt_fields(input.expected_receipt_fields.clone());

    if let Some(token_cost_ref) = &input.token_cost_ref {
        packet = packet.with_token_cost_ref(token_cost_ref.clone());
    }

    Ok(packet)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkInventoryInputPacket {
    pub module_id: String,
    pub module_version_ref: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub workspace_policy: PubPunkWorkspacePolicy,
    pub publishing_workspace_ref: String,
    pub allowed_source_refs: Vec<String>,
    pub instruction_refs: Vec<String>,
    pub items: Vec<PubPunkInventoryItemInput>,
    pub granted_capabilities: Vec<PubPunkCapabilityGrant>,
    pub privacy_policy: PubPunkPrivacyPolicy,
    pub expected_receipt_fields: Vec<String>,
    pub token_cost_ref: Option<String>,
}

impl PubPunkInventoryInputPacket {
    pub fn new(
        module_version_ref: impl Into<String>,
        contract_ref: impl Into<String>,
        run_ref: impl Into<String>,
        project_ref: impl Into<String>,
        publishing_workspace_ref: impl Into<String>,
    ) -> Self {
        Self {
            module_id: PUBPUNK_MODULE_ID.to_owned(),
            module_version_ref: module_version_ref.into(),
            contract_ref: contract_ref.into(),
            run_ref: run_ref.into(),
            project_ref: project_ref.into(),
            workspace_policy: PubPunkWorkspacePolicy::SplitExplicitRefs,
            publishing_workspace_ref: publishing_workspace_ref.into(),
            allowed_source_refs: Vec::new(),
            instruction_refs: Vec::new(),
            items: Vec::new(),
            granted_capabilities: Vec::new(),
            privacy_policy: PubPunkPrivacyPolicy::safe_metadata_only(),
            expected_receipt_fields: Vec::new(),
            token_cost_ref: None,
        }
    }

    pub fn with_workspace_policy(mut self, workspace_policy: PubPunkWorkspacePolicy) -> Self {
        self.workspace_policy = workspace_policy;
        self
    }

    pub fn with_allowed_source_refs(mut self, allowed_source_refs: Vec<impl Into<String>>) -> Self {
        self.allowed_source_refs = allowed_source_refs.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_instruction_refs(mut self, instruction_refs: Vec<impl Into<String>>) -> Self {
        self.instruction_refs = instruction_refs.into_iter().map(Into::into).collect();
        self
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

    pub fn with_token_cost_ref(mut self, token_cost_ref: impl Into<String>) -> Self {
        self.token_cost_ref = Some(token_cost_ref.into());
        self
    }

    pub fn try_into_inventory_input(
        &self,
    ) -> Result<PubPunkInventoryInput, PubPunkInventoryInputPacketAssessment> {
        let assessment = assess_pubpunk_inventory_input_packet(self);
        if assessment.has_blockers() {
            return Err(assessment);
        }

        Ok(PubPunkInventoryInput {
            module_id: self.module_id.clone(),
            module_version: self.module_version_ref.clone(),
            contract_ref: self.contract_ref.clone(),
            run_ref: self.run_ref.clone(),
            project_ref: self.project_ref.clone(),
            publishing_workspace_ref: self.publishing_workspace_ref.clone(),
            requested_operation: PubPunkInventoryOperation::AssessInventory,
            items: self.items.clone(),
            granted_capabilities: self.granted_capabilities.clone(),
            privacy_policy: self.privacy_policy,
            expected_receipt_fields: self.expected_receipt_fields.clone(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkInventoryInputPacketFindingCode {
    MissingModuleId,
    NonCanonicalModuleId,
    MissingModuleVersionRef,
    MissingContractRef,
    MissingRunRef,
    MissingProjectRef,
    UnsupportedWorkspacePolicy,
    MissingPublishingWorkspaceRef,
    UnsafePublishingWorkspaceRef,
    MissingInstructionRefs,
    MissingRequiredInstructionRef,
    UnsafeInstructionRef,
    UnsafeAllowedSourceRef,
    MissingAssessProvidedInventoryGrant,
    UnsupportedCapabilityGrant,
    MissingExpectedReceiptFields,
    UnsafePrivacyPolicy,
    UnsafeItemSourceRef,
    ItemSourceRefNotAllowed,
    RawBodyProvided,
    UnsafeTokenCostRef,
}

impl PubPunkInventoryInputPacketFindingCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingModuleId => "missing_module_id",
            Self::NonCanonicalModuleId => "non_canonical_module_id",
            Self::MissingModuleVersionRef => "missing_module_version_ref",
            Self::MissingContractRef => "missing_contract_ref",
            Self::MissingRunRef => "missing_run_ref",
            Self::MissingProjectRef => "missing_project_ref",
            Self::UnsupportedWorkspacePolicy => "unsupported_workspace_policy",
            Self::MissingPublishingWorkspaceRef => "missing_publishing_workspace_ref",
            Self::UnsafePublishingWorkspaceRef => "unsafe_publishing_workspace_ref",
            Self::MissingInstructionRefs => "missing_instruction_refs",
            Self::MissingRequiredInstructionRef => "missing_required_instruction_ref",
            Self::UnsafeInstructionRef => "unsafe_instruction_ref",
            Self::UnsafeAllowedSourceRef => "unsafe_allowed_source_ref",
            Self::MissingAssessProvidedInventoryGrant => "missing_assess_provided_inventory_grant",
            Self::UnsupportedCapabilityGrant => "unsupported_capability_grant",
            Self::MissingExpectedReceiptFields => "missing_expected_receipt_fields",
            Self::UnsafePrivacyPolicy => "unsafe_privacy_policy",
            Self::UnsafeItemSourceRef => "unsafe_item_source_ref",
            Self::ItemSourceRefNotAllowed => "item_source_ref_not_allowed",
            Self::RawBodyProvided => "raw_body_provided",
            Self::UnsafeTokenCostRef => "unsafe_token_cost_ref",
        }
    }

    pub fn is_blocking(self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkInventoryInputPacketFinding {
    pub code: PubPunkInventoryInputPacketFindingCode,
    pub ref_value: Option<String>,
    pub capability: Option<PubPunkCapabilityGrant>,
    pub message: &'static str,
}

impl PubPunkInventoryInputPacketFinding {
    fn new(code: PubPunkInventoryInputPacketFindingCode, message: &'static str) -> Self {
        Self {
            code,
            ref_value: None,
            capability: None,
            message,
        }
    }

    fn for_ref(
        code: PubPunkInventoryInputPacketFindingCode,
        ref_value: impl Into<String>,
        message: &'static str,
    ) -> Self {
        Self {
            code,
            ref_value: Some(ref_value.into()),
            capability: None,
            message,
        }
    }

    fn for_capability(capability: PubPunkCapabilityGrant) -> Self {
        Self {
            code: PubPunkInventoryInputPacketFindingCode::UnsupportedCapabilityGrant,
            ref_value: None,
            capability: Some(capability),
            message: "capability is not available in the side-effect-free input packet slice",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkInventoryInputPacketRefs {
    pub module_id: String,
    pub module_version_ref: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub workspace_policy: PubPunkWorkspacePolicy,
    pub publishing_workspace_ref: String,
    pub token_cost_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkInventoryInputPacketAssessment {
    pub schema_version: &'static str,
    pub status: PubPunkAssessmentStatus,
    pub authority: PubPunkAssessmentAuthority,
    pub findings: Vec<PubPunkInventoryInputPacketFinding>,
    pub boundary_flags: PubPunkInventoryBoundaryFlags,
    pub refs: PubPunkInventoryInputPacketRefs,
}

impl PubPunkInventoryInputPacketAssessment {
    pub fn blocking_findings(&self) -> impl Iterator<Item = &PubPunkInventoryInputPacketFinding> {
        self.findings
            .iter()
            .filter(|finding| finding.code.is_blocking())
    }

    pub fn has_blockers(&self) -> bool {
        self.blocking_findings().next().is_some()
    }
}

pub fn assess_pubpunk_inventory_input_packet(
    packet: &PubPunkInventoryInputPacket,
) -> PubPunkInventoryInputPacketAssessment {
    let mut findings = Vec::new();

    push_input_packet_required_ref_finding(
        &mut findings,
        packet.module_id.as_str(),
        PubPunkInventoryInputPacketFindingCode::MissingModuleId,
        "module id is required",
    );
    if !packet.module_id.trim().is_empty() && packet.module_id != PUBPUNK_MODULE_ID {
        findings.push(PubPunkInventoryInputPacketFinding::for_ref(
            PubPunkInventoryInputPacketFindingCode::NonCanonicalModuleId,
            packet.module_id.clone(),
            "PubPunk inventory input packets must use the canonical pubpunk module id",
        ));
    }
    push_input_packet_required_ref_finding(
        &mut findings,
        packet.module_version_ref.as_str(),
        PubPunkInventoryInputPacketFindingCode::MissingModuleVersionRef,
        "module version ref is required",
    );
    push_input_packet_required_ref_finding(
        &mut findings,
        packet.contract_ref.as_str(),
        PubPunkInventoryInputPacketFindingCode::MissingContractRef,
        "contract ref is required",
    );
    push_input_packet_required_ref_finding(
        &mut findings,
        packet.run_ref.as_str(),
        PubPunkInventoryInputPacketFindingCode::MissingRunRef,
        "run ref is required",
    );
    push_input_packet_required_ref_finding(
        &mut findings,
        packet.project_ref.as_str(),
        PubPunkInventoryInputPacketFindingCode::MissingProjectRef,
        "project ref is required",
    );

    if !packet.workspace_policy.selected_for_first_slice() {
        findings.push(PubPunkInventoryInputPacketFinding::for_ref(
            PubPunkInventoryInputPacketFindingCode::UnsupportedWorkspacePolicy,
            packet.workspace_policy.as_str(),
            "the first PubPunk input packet slice only supports split explicit refs",
        ));
    }

    push_input_packet_required_ref_finding(
        &mut findings,
        packet.publishing_workspace_ref.as_str(),
        PubPunkInventoryInputPacketFindingCode::MissingPublishingWorkspaceRef,
        "publishing workspace ref is required",
    );
    if !packet.publishing_workspace_ref.trim().is_empty()
        && !is_safe_workspace_ref(&packet.publishing_workspace_ref)
    {
        findings.push(PubPunkInventoryInputPacketFinding::for_ref(
            PubPunkInventoryInputPacketFindingCode::UnsafePublishingWorkspaceRef,
            packet.publishing_workspace_ref.clone(),
            "publishing workspace ref must be an explicit safe logical or repo-relative ref",
        ));
    }

    if packet.instruction_refs.is_empty() {
        findings.push(PubPunkInventoryInputPacketFinding::new(
            PubPunkInventoryInputPacketFindingCode::MissingInstructionRefs,
            "instruction refs are required for PubPunk input packets",
        ));
    }

    for required_ref in PUBPUNK_REQUIRED_INSTRUCTION_REFS {
        if !packet
            .instruction_refs
            .iter()
            .any(|instruction_ref| instruction_ref == required_ref)
        {
            findings.push(PubPunkInventoryInputPacketFinding::for_ref(
                PubPunkInventoryInputPacketFindingCode::MissingRequiredInstructionRef,
                *required_ref,
                "required PubPunk instruction ref is missing",
            ));
        }
    }

    for instruction_ref in &packet.instruction_refs {
        if !is_safe_source_ref(instruction_ref) {
            findings.push(PubPunkInventoryInputPacketFinding::for_ref(
                PubPunkInventoryInputPacketFindingCode::UnsafeInstructionRef,
                instruction_ref.clone(),
                "instruction refs must be explicit repo-relative refs",
            ));
        }
    }

    for source_ref in &packet.allowed_source_refs {
        if !is_safe_source_ref(source_ref) {
            findings.push(PubPunkInventoryInputPacketFinding::for_ref(
                PubPunkInventoryInputPacketFindingCode::UnsafeAllowedSourceRef,
                source_ref.clone(),
                "allowed source refs must be explicit repo-relative refs",
            ));
        }
    }

    if !packet
        .granted_capabilities
        .contains(&PubPunkCapabilityGrant::AssessProvidedInventory)
    {
        findings.push(PubPunkInventoryInputPacketFinding::new(
            PubPunkInventoryInputPacketFindingCode::MissingAssessProvidedInventoryGrant,
            "assess_provided_inventory must be explicitly granted for this packet",
        ));
    }
    for capability in &packet.granted_capabilities {
        if !capability.supported_by_side_effect_free_assessment() {
            findings.push(PubPunkInventoryInputPacketFinding::for_capability(
                *capability,
            ));
        }
    }

    if packet.expected_receipt_fields.is_empty() {
        findings.push(PubPunkInventoryInputPacketFinding::new(
            PubPunkInventoryInputPacketFindingCode::MissingExpectedReceiptFields,
            "expected receipt fields are required even though this packet does not write receipts",
        ));
    }

    if packet.privacy_policy.allows_private_or_raw_payloads() {
        findings.push(PubPunkInventoryInputPacketFinding::new(
            PubPunkInventoryInputPacketFindingCode::UnsafePrivacyPolicy,
            "privacy policy must disallow raw/private payloads for this packet",
        ));
    }

    for item in &packet.items {
        if !is_safe_source_ref(&item.source_ref) {
            findings.push(PubPunkInventoryInputPacketFinding::for_ref(
                PubPunkInventoryInputPacketFindingCode::UnsafeItemSourceRef,
                item.source_ref.clone(),
                "inventory item refs must be explicit repo-relative metadata refs",
            ));
        }
        if !packet.allowed_source_refs.contains(&item.source_ref) {
            findings.push(PubPunkInventoryInputPacketFinding::for_ref(
                PubPunkInventoryInputPacketFindingCode::ItemSourceRefNotAllowed,
                item.source_ref.clone(),
                "inventory item refs must be included in allowed source refs",
            ));
        }
        if item.raw_body_provided {
            findings.push(PubPunkInventoryInputPacketFinding::for_ref(
                PubPunkInventoryInputPacketFindingCode::RawBodyProvided,
                item.source_ref.clone(),
                "raw post bodies are not accepted by the PubPunk input packet",
            ));
        }
    }

    if let Some(token_cost_ref) = &packet.token_cost_ref {
        if !is_safe_source_ref(token_cost_ref) {
            findings.push(PubPunkInventoryInputPacketFinding::for_ref(
                PubPunkInventoryInputPacketFindingCode::UnsafeTokenCostRef,
                token_cost_ref.clone(),
                "token cost ref must be an explicit repo-relative ref when provided",
            ));
        }
    }

    let status = if findings.iter().any(|finding| finding.code.is_blocking()) {
        PubPunkAssessmentStatus::Blocked
    } else {
        PubPunkAssessmentStatus::Ready
    };

    PubPunkInventoryInputPacketAssessment {
        schema_version: PUBPUNK_INVENTORY_INPUT_PACKET_SCHEMA_VERSION,
        status,
        authority: PubPunkAssessmentAuthority::Advisory,
        findings,
        boundary_flags: PUBPUNK_INVENTORY_ASSESSMENT_BOUNDARY_FLAGS,
        refs: PubPunkInventoryInputPacketRefs {
            module_id: packet.module_id.clone(),
            module_version_ref: packet.module_version_ref.clone(),
            contract_ref: packet.contract_ref.clone(),
            run_ref: packet.run_ref.clone(),
            project_ref: packet.project_ref.clone(),
            workspace_policy: packet.workspace_policy,
            publishing_workspace_ref: packet.publishing_workspace_ref.clone(),
            token_cost_ref: packet.token_cost_ref.clone(),
        },
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkConnectorStrategy {
    Api,
    Browser,
    Manual,
}

impl PubPunkConnectorStrategy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Api => "api",
            Self::Browser => "browser",
            Self::Manual => "manual",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkConnectorStrategySelectionReason {
    ApiPreferredAndAvailable,
    BrowserFallbackAllowed,
    ManualFallbackAllowed,
    BlockedNoAllowedStrategy,
}

impl PubPunkConnectorStrategySelectionReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ApiPreferredAndAvailable => "api_preferred_and_available",
            Self::BrowserFallbackAllowed => "browser_fallback_allowed",
            Self::ManualFallbackAllowed => "manual_fallback_allowed",
            Self::BlockedNoAllowedStrategy => "blocked_no_allowed_strategy",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkChannelConnectorProfileResolutionOperation {
    ResolveChannelConnectorProfile,
}

impl PubPunkChannelConnectorProfileResolutionOperation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ResolveChannelConnectorProfile => "resolve_channel_connector_profile",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkChannelConnectorProfileResolutionPacket {
    pub module_id: String,
    pub module_version_ref: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub workspace_policy: PubPunkWorkspacePolicy,
    pub publishing_workspace_ref: String,
    pub inventory_assessment_ref: String,
    pub candidate_ref: String,
    pub channel_ref: String,
    pub connector_profile_ref: String,
    pub api_availability_ref: String,
    pub browser_automation_policy_ref: String,
    pub manual_handoff_ref: String,
    pub credential_signal_ref: String,
    pub payload_ref: String,
    pub api_available: bool,
    pub browser_allowed: bool,
    pub manual_allowed: bool,
    pub strategy_order: Vec<PubPunkConnectorStrategy>,
    pub allowed_source_refs: Vec<String>,
    pub instruction_refs: Vec<String>,
    pub granted_capabilities: Vec<PubPunkCapabilityGrant>,
    pub privacy_policy: PubPunkPrivacyPolicy,
    pub expected_receipt_fields: Vec<String>,
    pub token_cost_ref: Option<String>,
}

impl PubPunkChannelConnectorProfileResolutionPacket {
    pub fn new(
        module_version_ref: impl Into<String>,
        contract_ref: impl Into<String>,
        run_ref: impl Into<String>,
        project_ref: impl Into<String>,
        publishing_workspace_ref: impl Into<String>,
    ) -> Self {
        Self {
            module_id: PUBPUNK_MODULE_ID.to_owned(),
            module_version_ref: module_version_ref.into(),
            contract_ref: contract_ref.into(),
            run_ref: run_ref.into(),
            project_ref: project_ref.into(),
            workspace_policy: PubPunkWorkspacePolicy::SplitExplicitRefs,
            publishing_workspace_ref: publishing_workspace_ref.into(),
            inventory_assessment_ref: String::new(),
            candidate_ref: String::new(),
            channel_ref: String::new(),
            connector_profile_ref: String::new(),
            api_availability_ref: String::new(),
            browser_automation_policy_ref: String::new(),
            manual_handoff_ref: String::new(),
            credential_signal_ref: String::new(),
            payload_ref: String::new(),
            api_available: false,
            browser_allowed: false,
            manual_allowed: false,
            strategy_order: vec![
                PubPunkConnectorStrategy::Api,
                PubPunkConnectorStrategy::Browser,
                PubPunkConnectorStrategy::Manual,
            ],
            allowed_source_refs: Vec::new(),
            instruction_refs: Vec::new(),
            granted_capabilities: Vec::new(),
            privacy_policy: PubPunkPrivacyPolicy::safe_metadata_only(),
            expected_receipt_fields: Vec::new(),
            token_cost_ref: None,
        }
    }

    pub fn with_workspace_policy(mut self, workspace_policy: PubPunkWorkspacePolicy) -> Self {
        self.workspace_policy = workspace_policy;
        self
    }

    pub fn with_inventory_assessment_ref(
        mut self,
        inventory_assessment_ref: impl Into<String>,
    ) -> Self {
        self.inventory_assessment_ref = inventory_assessment_ref.into();
        self
    }

    pub fn with_candidate_ref(mut self, candidate_ref: impl Into<String>) -> Self {
        self.candidate_ref = candidate_ref.into();
        self
    }

    pub fn with_channel_ref(mut self, channel_ref: impl Into<String>) -> Self {
        self.channel_ref = channel_ref.into();
        self
    }

    pub fn with_connector_profile_ref(mut self, connector_profile_ref: impl Into<String>) -> Self {
        self.connector_profile_ref = connector_profile_ref.into();
        self
    }

    pub fn with_api_availability_ref(mut self, api_availability_ref: impl Into<String>) -> Self {
        self.api_availability_ref = api_availability_ref.into();
        self
    }

    pub fn with_browser_automation_policy_ref(
        mut self,
        browser_automation_policy_ref: impl Into<String>,
    ) -> Self {
        self.browser_automation_policy_ref = browser_automation_policy_ref.into();
        self
    }

    pub fn with_manual_handoff_ref(mut self, manual_handoff_ref: impl Into<String>) -> Self {
        self.manual_handoff_ref = manual_handoff_ref.into();
        self
    }

    pub fn with_credential_signal_ref(mut self, credential_signal_ref: impl Into<String>) -> Self {
        self.credential_signal_ref = credential_signal_ref.into();
        self
    }

    pub fn with_payload_ref(mut self, payload_ref: impl Into<String>) -> Self {
        self.payload_ref = payload_ref.into();
        self
    }

    pub fn with_api_available(mut self, api_available: bool) -> Self {
        self.api_available = api_available;
        self
    }

    pub fn with_browser_allowed(mut self, browser_allowed: bool) -> Self {
        self.browser_allowed = browser_allowed;
        self
    }

    pub fn with_manual_allowed(mut self, manual_allowed: bool) -> Self {
        self.manual_allowed = manual_allowed;
        self
    }

    pub fn with_strategy_order(mut self, strategy_order: Vec<PubPunkConnectorStrategy>) -> Self {
        self.strategy_order = strategy_order;
        self
    }

    pub fn with_allowed_source_refs(mut self, allowed_source_refs: Vec<impl Into<String>>) -> Self {
        self.allowed_source_refs = allowed_source_refs.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_instruction_refs(mut self, instruction_refs: Vec<impl Into<String>>) -> Self {
        self.instruction_refs = instruction_refs.into_iter().map(Into::into).collect();
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

    pub fn with_token_cost_ref(mut self, token_cost_ref: impl Into<String>) -> Self {
        self.token_cost_ref = Some(token_cost_ref.into());
        self
    }

    pub fn try_into_connector_profile_resolution_refs(
        &self,
    ) -> Result<
        PubPunkResolvedChannelConnectorProfileRefs,
        PubPunkChannelConnectorProfileResolutionPacketAssessment,
    > {
        let assessment = assess_pubpunk_channel_connector_profile_resolution_packet(self);
        if assessment.has_blockers() {
            return Err(assessment);
        }

        Ok(PubPunkResolvedChannelConnectorProfileRefs {
            inventory_assessment_ref: self.inventory_assessment_ref.clone(),
            candidate_ref: self.candidate_ref.clone(),
            channel_ref: self.channel_ref.clone(),
            connector_profile_ref: self.connector_profile_ref.clone(),
            selected_strategy: assessment
                .selected_strategy
                .expect("ready connector profile resolution must select a strategy"),
            api_availability_ref: self.api_availability_ref.clone(),
            browser_automation_policy_ref: self.browser_automation_policy_ref.clone(),
            manual_handoff_ref: self.manual_handoff_ref.clone(),
            credential_signal_ref: self.credential_signal_ref.clone(),
            payload_ref: self.payload_ref.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkResolvedChannelConnectorProfileRefs {
    pub inventory_assessment_ref: String,
    pub candidate_ref: String,
    pub channel_ref: String,
    pub connector_profile_ref: String,
    pub selected_strategy: PubPunkConnectorStrategy,
    pub api_availability_ref: String,
    pub browser_automation_policy_ref: String,
    pub manual_handoff_ref: String,
    pub credential_signal_ref: String,
    pub payload_ref: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkChannelConnectorProfileResolutionPacketFindingCode {
    MissingModuleId,
    NonCanonicalModuleId,
    MissingModuleVersionRef,
    MissingContractRef,
    MissingRunRef,
    MissingProjectRef,
    UnsupportedWorkspacePolicy,
    MissingPublishingWorkspaceRef,
    UnsafePublishingWorkspaceRef,
    MissingInventoryAssessmentRef,
    UnsafeInventoryAssessmentRef,
    InventoryAssessmentRefNotAllowed,
    MissingCandidateRef,
    UnsafeCandidateRef,
    CandidateRefNotAllowed,
    MissingChannelRef,
    UnsafeChannelRef,
    ChannelRefNotAllowed,
    MissingConnectorProfileRef,
    UnsafeConnectorProfileRef,
    ConnectorProfileRefNotAllowed,
    MissingApiAvailabilityRef,
    UnsafeApiAvailabilityRef,
    ApiAvailabilityRefNotAllowed,
    MissingBrowserAutomationPolicyRef,
    UnsafeBrowserAutomationPolicyRef,
    BrowserAutomationPolicyRefNotAllowed,
    MissingManualHandoffRef,
    UnsafeManualHandoffRef,
    ManualHandoffRefNotAllowed,
    MissingCredentialSignalRef,
    UnsafeCredentialSignalRef,
    CredentialSignalRefNotAllowed,
    MissingPayloadRef,
    UnsafePayloadRef,
    PayloadRefNotAllowed,
    MissingStrategyOrder,
    UnsupportedStrategyOrder,
    NoAllowedConnectorStrategy,
    MissingInstructionRefs,
    MissingRequiredInstructionRef,
    UnsafeInstructionRef,
    UnsafeAllowedSourceRef,
    MissingResolveConnectorProfileGrant,
    UnsupportedCapabilityGrant,
    MissingExpectedReceiptFields,
    MissingRequiredExpectedReceiptField,
    UnsafePrivacyPolicy,
    UnsafeTokenCostRef,
}

impl PubPunkChannelConnectorProfileResolutionPacketFindingCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingModuleId => "missing_module_id",
            Self::NonCanonicalModuleId => "non_canonical_module_id",
            Self::MissingModuleVersionRef => "missing_module_version_ref",
            Self::MissingContractRef => "missing_contract_ref",
            Self::MissingRunRef => "missing_run_ref",
            Self::MissingProjectRef => "missing_project_ref",
            Self::UnsupportedWorkspacePolicy => "unsupported_workspace_policy",
            Self::MissingPublishingWorkspaceRef => "missing_publishing_workspace_ref",
            Self::UnsafePublishingWorkspaceRef => "unsafe_publishing_workspace_ref",
            Self::MissingInventoryAssessmentRef => "missing_inventory_assessment_ref",
            Self::UnsafeInventoryAssessmentRef => "unsafe_inventory_assessment_ref",
            Self::InventoryAssessmentRefNotAllowed => "inventory_assessment_ref_not_allowed",
            Self::MissingCandidateRef => "missing_candidate_ref",
            Self::UnsafeCandidateRef => "unsafe_candidate_ref",
            Self::CandidateRefNotAllowed => "candidate_ref_not_allowed",
            Self::MissingChannelRef => "missing_channel_ref",
            Self::UnsafeChannelRef => "unsafe_channel_ref",
            Self::ChannelRefNotAllowed => "channel_ref_not_allowed",
            Self::MissingConnectorProfileRef => "missing_connector_profile_ref",
            Self::UnsafeConnectorProfileRef => "unsafe_connector_profile_ref",
            Self::ConnectorProfileRefNotAllowed => "connector_profile_ref_not_allowed",
            Self::MissingApiAvailabilityRef => "missing_api_availability_ref",
            Self::UnsafeApiAvailabilityRef => "unsafe_api_availability_ref",
            Self::ApiAvailabilityRefNotAllowed => "api_availability_ref_not_allowed",
            Self::MissingBrowserAutomationPolicyRef => "missing_browser_automation_policy_ref",
            Self::UnsafeBrowserAutomationPolicyRef => "unsafe_browser_automation_policy_ref",
            Self::BrowserAutomationPolicyRefNotAllowed => {
                "browser_automation_policy_ref_not_allowed"
            }
            Self::MissingManualHandoffRef => "missing_manual_handoff_ref",
            Self::UnsafeManualHandoffRef => "unsafe_manual_handoff_ref",
            Self::ManualHandoffRefNotAllowed => "manual_handoff_ref_not_allowed",
            Self::MissingCredentialSignalRef => "missing_credential_signal_ref",
            Self::UnsafeCredentialSignalRef => "unsafe_credential_signal_ref",
            Self::CredentialSignalRefNotAllowed => "credential_signal_ref_not_allowed",
            Self::MissingPayloadRef => "missing_payload_ref",
            Self::UnsafePayloadRef => "unsafe_payload_ref",
            Self::PayloadRefNotAllowed => "payload_ref_not_allowed",
            Self::MissingStrategyOrder => "missing_strategy_order",
            Self::UnsupportedStrategyOrder => "unsupported_strategy_order",
            Self::NoAllowedConnectorStrategy => "no_allowed_connector_strategy",
            Self::MissingInstructionRefs => "missing_instruction_refs",
            Self::MissingRequiredInstructionRef => "missing_required_instruction_ref",
            Self::UnsafeInstructionRef => "unsafe_instruction_ref",
            Self::UnsafeAllowedSourceRef => "unsafe_allowed_source_ref",
            Self::MissingResolveConnectorProfileGrant => "missing_resolve_connector_profile_grant",
            Self::UnsupportedCapabilityGrant => "unsupported_capability_grant",
            Self::MissingExpectedReceiptFields => "missing_expected_receipt_fields",
            Self::MissingRequiredExpectedReceiptField => "missing_required_expected_receipt_field",
            Self::UnsafePrivacyPolicy => "unsafe_privacy_policy",
            Self::UnsafeTokenCostRef => "unsafe_token_cost_ref",
        }
    }

    pub fn is_blocking(self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkChannelConnectorProfileResolutionPacketFinding {
    pub code: PubPunkChannelConnectorProfileResolutionPacketFindingCode,
    pub ref_value: Option<String>,
    pub capability: Option<PubPunkCapabilityGrant>,
    pub message: &'static str,
}

impl PubPunkChannelConnectorProfileResolutionPacketFinding {
    fn new(
        code: PubPunkChannelConnectorProfileResolutionPacketFindingCode,
        message: &'static str,
    ) -> Self {
        Self {
            code,
            ref_value: None,
            capability: None,
            message,
        }
    }

    fn for_ref(
        code: PubPunkChannelConnectorProfileResolutionPacketFindingCode,
        ref_value: impl Into<String>,
        message: &'static str,
    ) -> Self {
        Self {
            code,
            ref_value: Some(ref_value.into()),
            capability: None,
            message,
        }
    }

    fn for_capability(capability: PubPunkCapabilityGrant) -> Self {
        Self {
            code: PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsupportedCapabilityGrant,
            ref_value: None,
            capability: Some(capability),
            message: "capability is not available in the side-effect-free connector profile resolution packet",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkChannelConnectorProfileResolutionPacketRefs {
    pub module_id: String,
    pub module_version_ref: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub workspace_policy: PubPunkWorkspacePolicy,
    pub publishing_workspace_ref: String,
    pub inventory_assessment_ref: String,
    pub candidate_ref: String,
    pub channel_ref: String,
    pub connector_profile_ref: String,
    pub api_availability_ref: String,
    pub browser_automation_policy_ref: String,
    pub manual_handoff_ref: String,
    pub credential_signal_ref: String,
    pub payload_ref: String,
    pub token_cost_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkChannelConnectorProfileResolutionPacketAssessment {
    pub schema_version: &'static str,
    pub status: PubPunkAssessmentStatus,
    pub authority: PubPunkAssessmentAuthority,
    pub requested_operation: PubPunkChannelConnectorProfileResolutionOperation,
    pub selected_strategy: Option<PubPunkConnectorStrategy>,
    pub selection_reason: PubPunkConnectorStrategySelectionReason,
    pub findings: Vec<PubPunkChannelConnectorProfileResolutionPacketFinding>,
    pub boundary_flags: PubPunkInventoryBoundaryFlags,
    pub refs: PubPunkChannelConnectorProfileResolutionPacketRefs,
}

impl PubPunkChannelConnectorProfileResolutionPacketAssessment {
    pub fn blocking_findings(
        &self,
    ) -> impl Iterator<Item = &PubPunkChannelConnectorProfileResolutionPacketFinding> {
        self.findings
            .iter()
            .filter(|finding| finding.code.is_blocking())
    }

    pub fn has_blockers(&self) -> bool {
        self.blocking_findings().next().is_some()
    }
}

pub fn assess_pubpunk_channel_connector_profile_resolution_packet(
    packet: &PubPunkChannelConnectorProfileResolutionPacket,
) -> PubPunkChannelConnectorProfileResolutionPacketAssessment {
    let mut findings = Vec::new();

    push_connector_profile_resolution_required_ref_finding(
        &mut findings,
        packet.module_id.as_str(),
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingModuleId,
        "module id is required",
    );
    if !packet.module_id.trim().is_empty() && packet.module_id != PUBPUNK_MODULE_ID {
        findings.push(PubPunkChannelConnectorProfileResolutionPacketFinding::for_ref(
            PubPunkChannelConnectorProfileResolutionPacketFindingCode::NonCanonicalModuleId,
            packet.module_id.clone(),
            "PubPunk connector profile resolution packets must use the canonical pubpunk module id",
        ));
    }
    push_connector_profile_resolution_required_ref_finding(
        &mut findings,
        packet.module_version_ref.as_str(),
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingModuleVersionRef,
        "module version ref is required",
    );
    push_connector_profile_resolution_required_ref_finding(
        &mut findings,
        packet.contract_ref.as_str(),
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingContractRef,
        "contract ref is required",
    );
    push_connector_profile_resolution_required_ref_finding(
        &mut findings,
        packet.run_ref.as_str(),
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingRunRef,
        "run ref is required",
    );
    push_connector_profile_resolution_required_ref_finding(
        &mut findings,
        packet.project_ref.as_str(),
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingProjectRef,
        "project ref is required",
    );

    if !packet.workspace_policy.selected_for_first_slice() {
        findings.push(PubPunkChannelConnectorProfileResolutionPacketFinding::for_ref(
            PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsupportedWorkspacePolicy,
            packet.workspace_policy.as_str(),
            "the first PubPunk connector profile resolution supports split explicit refs only",
        ));
    }

    push_connector_profile_resolution_required_ref_finding(
        &mut findings,
        packet.publishing_workspace_ref.as_str(),
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingPublishingWorkspaceRef,
        "publishing workspace ref is required",
    );
    if !packet.publishing_workspace_ref.trim().is_empty()
        && !is_safe_workspace_ref(&packet.publishing_workspace_ref)
    {
        findings.push(PubPunkChannelConnectorProfileResolutionPacketFinding::for_ref(
            PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafePublishingWorkspaceRef,
            packet.publishing_workspace_ref.clone(),
            "publishing workspace ref must be an explicit safe logical or repo-relative ref",
        ));
    }

    validate_connector_profile_resolution_ref(
        &mut findings,
        packet.inventory_assessment_ref.as_str(),
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingInventoryAssessmentRef,
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeInventoryAssessmentRef,
        "inventory assessment ref is required",
        "inventory assessment ref must be an explicit repo-relative ref",
    );
    validate_connector_profile_resolution_ref(
        &mut findings,
        packet.candidate_ref.as_str(),
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingCandidateRef,
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeCandidateRef,
        "candidate ref is required",
        "candidate ref must be an explicit repo-relative ref",
    );
    validate_connector_profile_resolution_ref(
        &mut findings,
        packet.channel_ref.as_str(),
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingChannelRef,
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeChannelRef,
        "channel ref is required",
        "channel ref must be an explicit repo-relative ref",
    );
    validate_connector_profile_resolution_ref(
        &mut findings,
        packet.connector_profile_ref.as_str(),
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingConnectorProfileRef,
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeConnectorProfileRef,
        "connector profile ref is required",
        "connector profile ref must be an explicit repo-relative ref",
    );
    validate_connector_profile_resolution_ref(
        &mut findings,
        packet.api_availability_ref.as_str(),
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingApiAvailabilityRef,
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeApiAvailabilityRef,
        "API availability ref is required",
        "API availability ref must be an explicit repo-relative ref",
    );
    validate_connector_profile_resolution_ref(
        &mut findings,
        packet.browser_automation_policy_ref.as_str(),
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingBrowserAutomationPolicyRef,
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeBrowserAutomationPolicyRef,
        "browser automation policy ref is required",
        "browser automation policy ref must be an explicit repo-relative ref",
    );
    validate_connector_profile_resolution_ref(
        &mut findings,
        packet.manual_handoff_ref.as_str(),
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingManualHandoffRef,
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeManualHandoffRef,
        "manual handoff ref is required",
        "manual handoff ref must be an explicit repo-relative ref",
    );
    validate_connector_profile_resolution_ref(
        &mut findings,
        packet.credential_signal_ref.as_str(),
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingCredentialSignalRef,
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeCredentialSignalRef,
        "credential signal ref is required",
        "credential signal ref must be an explicit repo-relative metadata ref",
    );
    validate_connector_profile_resolution_ref(
        &mut findings,
        packet.payload_ref.as_str(),
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingPayloadRef,
        PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafePayloadRef,
        "payload ref is required",
        "payload ref must be an explicit repo-relative ref",
    );

    if packet.strategy_order.is_empty() {
        findings.push(PubPunkChannelConnectorProfileResolutionPacketFinding::new(
            PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingStrategyOrder,
            "connector strategy order is required",
        ));
    } else {
        let required_order = [
            PubPunkConnectorStrategy::Api,
            PubPunkConnectorStrategy::Browser,
            PubPunkConnectorStrategy::Manual,
        ];
        if packet.strategy_order.as_slice() != required_order.as_slice() {
            findings.push(
                PubPunkChannelConnectorProfileResolutionPacketFinding::new(
                    PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsupportedStrategyOrder,
                    "the first connector profile resolution slice requires API, then browser, then manual strategy order",
                ),
            );
        }
    }

    if packet.instruction_refs.is_empty() {
        findings.push(PubPunkChannelConnectorProfileResolutionPacketFinding::new(
            PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingInstructionRefs,
            "instruction refs are required for PubPunk connector profile resolution packets",
        ));
    }

    for required_ref in PUBPUNK_REQUIRED_INSTRUCTION_REFS {
        if !packet
            .instruction_refs
            .iter()
            .any(|instruction_ref| instruction_ref == required_ref)
        {
            findings.push(PubPunkChannelConnectorProfileResolutionPacketFinding::for_ref(
                PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingRequiredInstructionRef,
                *required_ref,
                "required PubPunk instruction ref is missing",
            ));
        }
    }

    for instruction_ref in &packet.instruction_refs {
        if !is_safe_source_ref(instruction_ref) {
            findings.push(
                PubPunkChannelConnectorProfileResolutionPacketFinding::for_ref(
                    PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeInstructionRef,
                    instruction_ref.clone(),
                    "instruction refs must be explicit repo-relative refs",
                ),
            );
        }
    }

    for source_ref in &packet.allowed_source_refs {
        if !is_safe_source_ref(source_ref) {
            findings.push(PubPunkChannelConnectorProfileResolutionPacketFinding::for_ref(
                PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeAllowedSourceRef,
                source_ref.clone(),
                "allowed source refs must be explicit repo-relative refs",
            ));
        }
    }

    for (source_ref, code, message) in [
        (
            packet.inventory_assessment_ref.as_str(),
            PubPunkChannelConnectorProfileResolutionPacketFindingCode::InventoryAssessmentRefNotAllowed,
            "inventory assessment ref must be included in allowed source refs",
        ),
        (
            packet.candidate_ref.as_str(),
            PubPunkChannelConnectorProfileResolutionPacketFindingCode::CandidateRefNotAllowed,
            "candidate ref must be included in allowed source refs",
        ),
        (
            packet.channel_ref.as_str(),
            PubPunkChannelConnectorProfileResolutionPacketFindingCode::ChannelRefNotAllowed,
            "channel ref must be included in allowed source refs",
        ),
        (
            packet.connector_profile_ref.as_str(),
            PubPunkChannelConnectorProfileResolutionPacketFindingCode::ConnectorProfileRefNotAllowed,
            "connector profile ref must be included in allowed source refs",
        ),
        (
            packet.api_availability_ref.as_str(),
            PubPunkChannelConnectorProfileResolutionPacketFindingCode::ApiAvailabilityRefNotAllowed,
            "API availability ref must be included in allowed source refs",
        ),
        (
            packet.browser_automation_policy_ref.as_str(),
            PubPunkChannelConnectorProfileResolutionPacketFindingCode::BrowserAutomationPolicyRefNotAllowed,
            "browser automation policy ref must be included in allowed source refs",
        ),
        (
            packet.manual_handoff_ref.as_str(),
            PubPunkChannelConnectorProfileResolutionPacketFindingCode::ManualHandoffRefNotAllowed,
            "manual handoff ref must be included in allowed source refs",
        ),
        (
            packet.credential_signal_ref.as_str(),
            PubPunkChannelConnectorProfileResolutionPacketFindingCode::CredentialSignalRefNotAllowed,
            "credential signal ref must be included in allowed source refs",
        ),
        (
            packet.payload_ref.as_str(),
            PubPunkChannelConnectorProfileResolutionPacketFindingCode::PayloadRefNotAllowed,
            "payload ref must be included in allowed source refs",
        ),
    ] {
        if !source_ref.trim().is_empty()
            && !packet
                .allowed_source_refs
                .iter()
                .any(|allowed_ref| allowed_ref == source_ref)
        {
            findings.push(PubPunkChannelConnectorProfileResolutionPacketFinding::for_ref(
                code, source_ref, message,
            ));
        }
    }

    if !packet
        .granted_capabilities
        .contains(&PubPunkCapabilityGrant::ResolveConnectorProfile)
    {
        findings.push(PubPunkChannelConnectorProfileResolutionPacketFinding::new(
            PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingResolveConnectorProfileGrant,
            "resolve_connector_profile must be explicitly granted for this packet",
        ));
    }
    for capability in &packet.granted_capabilities {
        if !capability.supported_by_side_effect_free_connector_profile_resolution() {
            findings.push(
                PubPunkChannelConnectorProfileResolutionPacketFinding::for_capability(*capability),
            );
        }
    }

    if packet.expected_receipt_fields.is_empty() {
        findings.push(PubPunkChannelConnectorProfileResolutionPacketFinding::new(
            PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingExpectedReceiptFields,
            "expected receipt fields are required even though this packet does not write receipts",
        ));
    }
    for required_field in [
        "side_effects",
        "host_validation",
        "connector_profile_resolution",
        "selected_connector_strategy",
        "channel_ref",
        "connector_profile_ref",
        "credential_signal_ref",
        "manual_fallback",
    ] {
        if !packet
            .expected_receipt_fields
            .iter()
            .any(|field| field == required_field)
        {
            findings.push(PubPunkChannelConnectorProfileResolutionPacketFinding::for_ref(
                PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingRequiredExpectedReceiptField,
                required_field,
                "connector profile resolution expectations must include host validation, selected strategy, channel/profile refs, credential signal metadata, and manual fallback coverage",
            ));
        }
    }

    if packet.privacy_policy.allows_private_or_raw_payloads() {
        findings.push(PubPunkChannelConnectorProfileResolutionPacketFinding::new(
            PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafePrivacyPolicy,
            "privacy policy must disallow raw/private payloads for connector profile resolution",
        ));
    }

    if let Some(token_cost_ref) = &packet.token_cost_ref {
        if !is_safe_source_ref(token_cost_ref) {
            findings.push(
                PubPunkChannelConnectorProfileResolutionPacketFinding::for_ref(
                    PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeTokenCostRef,
                    token_cost_ref.clone(),
                    "token cost ref must be an explicit repo-relative ref when provided",
                ),
            );
        }
    }

    let (selected_strategy, selection_reason) =
        select_pubpunk_connector_strategy_from_profile_packet(packet);
    if selected_strategy.is_none() {
        findings.push(PubPunkChannelConnectorProfileResolutionPacketFinding::new(
            PubPunkChannelConnectorProfileResolutionPacketFindingCode::NoAllowedConnectorStrategy,
            "connector profile resolution must select API, browser automation, or manual handoff from explicit signals",
        ));
    }

    let status = if findings.iter().any(|finding| finding.code.is_blocking()) {
        PubPunkAssessmentStatus::Blocked
    } else {
        PubPunkAssessmentStatus::Ready
    };

    PubPunkChannelConnectorProfileResolutionPacketAssessment {
        schema_version: PUBPUNK_CHANNEL_CONNECTOR_PROFILE_RESOLUTION_SCHEMA_VERSION,
        status,
        authority: PubPunkAssessmentAuthority::Advisory,
        requested_operation:
            PubPunkChannelConnectorProfileResolutionOperation::ResolveChannelConnectorProfile,
        selected_strategy,
        selection_reason,
        findings,
        boundary_flags: PUBPUNK_INVENTORY_ASSESSMENT_BOUNDARY_FLAGS,
        refs: PubPunkChannelConnectorProfileResolutionPacketRefs {
            module_id: packet.module_id.clone(),
            module_version_ref: packet.module_version_ref.clone(),
            contract_ref: packet.contract_ref.clone(),
            run_ref: packet.run_ref.clone(),
            project_ref: packet.project_ref.clone(),
            workspace_policy: packet.workspace_policy,
            publishing_workspace_ref: packet.publishing_workspace_ref.clone(),
            inventory_assessment_ref: packet.inventory_assessment_ref.clone(),
            candidate_ref: packet.candidate_ref.clone(),
            channel_ref: packet.channel_ref.clone(),
            connector_profile_ref: packet.connector_profile_ref.clone(),
            api_availability_ref: packet.api_availability_ref.clone(),
            browser_automation_policy_ref: packet.browser_automation_policy_ref.clone(),
            manual_handoff_ref: packet.manual_handoff_ref.clone(),
            credential_signal_ref: packet.credential_signal_ref.clone(),
            payload_ref: packet.payload_ref.clone(),
            token_cost_ref: packet.token_cost_ref.clone(),
        },
    }
}

fn select_pubpunk_connector_strategy_from_profile_packet(
    packet: &PubPunkChannelConnectorProfileResolutionPacket,
) -> (
    Option<PubPunkConnectorStrategy>,
    PubPunkConnectorStrategySelectionReason,
) {
    for strategy in &packet.strategy_order {
        match strategy {
            PubPunkConnectorStrategy::Api if packet.api_available => {
                return (
                    Some(PubPunkConnectorStrategy::Api),
                    PubPunkConnectorStrategySelectionReason::ApiPreferredAndAvailable,
                );
            }
            PubPunkConnectorStrategy::Browser if packet.browser_allowed => {
                return (
                    Some(PubPunkConnectorStrategy::Browser),
                    PubPunkConnectorStrategySelectionReason::BrowserFallbackAllowed,
                );
            }
            PubPunkConnectorStrategy::Manual if packet.manual_allowed => {
                return (
                    Some(PubPunkConnectorStrategy::Manual),
                    PubPunkConnectorStrategySelectionReason::ManualFallbackAllowed,
                );
            }
            _ => {}
        }
    }

    (
        None,
        PubPunkConnectorStrategySelectionReason::BlockedNoAllowedStrategy,
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkPublishRequestOperation {
    RequestExternalPublish,
}

impl PubPunkPublishRequestOperation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RequestExternalPublish => "request_external_publish",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishRequestPacket {
    pub module_id: String,
    pub module_version_ref: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub workspace_policy: PubPunkWorkspacePolicy,
    pub publishing_workspace_ref: String,
    pub inventory_assessment_ref: String,
    pub candidate_ref: String,
    pub channel_ref: String,
    pub connector_profile_resolution_ref: String,
    pub connector_profile_ref: String,
    pub selected_connector_strategy_ref: String,
    pub side_effect_request_ref: String,
    pub intent_ref: String,
    pub policy_ref: String,
    pub adapter_ref: String,
    pub payload_ref: String,
    pub receipt_proposal_ref: String,
    pub allowed_source_refs: Vec<String>,
    pub instruction_refs: Vec<String>,
    pub granted_capabilities: Vec<PubPunkCapabilityGrant>,
    pub privacy_policy: PubPunkPrivacyPolicy,
    pub expected_receipt_fields: Vec<String>,
    pub token_cost_ref: Option<String>,
}

impl PubPunkPublishRequestPacket {
    pub fn new(
        module_version_ref: impl Into<String>,
        contract_ref: impl Into<String>,
        run_ref: impl Into<String>,
        project_ref: impl Into<String>,
        publishing_workspace_ref: impl Into<String>,
    ) -> Self {
        Self {
            module_id: PUBPUNK_MODULE_ID.to_owned(),
            module_version_ref: module_version_ref.into(),
            contract_ref: contract_ref.into(),
            run_ref: run_ref.into(),
            project_ref: project_ref.into(),
            workspace_policy: PubPunkWorkspacePolicy::SplitExplicitRefs,
            publishing_workspace_ref: publishing_workspace_ref.into(),
            inventory_assessment_ref: String::new(),
            candidate_ref: String::new(),
            channel_ref: String::new(),
            connector_profile_resolution_ref: String::new(),
            connector_profile_ref: String::new(),
            selected_connector_strategy_ref: String::new(),
            side_effect_request_ref: String::new(),
            intent_ref: String::new(),
            policy_ref: String::new(),
            adapter_ref: String::new(),
            payload_ref: String::new(),
            receipt_proposal_ref: String::new(),
            allowed_source_refs: Vec::new(),
            instruction_refs: Vec::new(),
            granted_capabilities: Vec::new(),
            privacy_policy: PubPunkPrivacyPolicy::safe_metadata_only(),
            expected_receipt_fields: Vec::new(),
            token_cost_ref: None,
        }
    }

    pub fn with_workspace_policy(mut self, workspace_policy: PubPunkWorkspacePolicy) -> Self {
        self.workspace_policy = workspace_policy;
        self
    }

    pub fn with_inventory_assessment_ref(
        mut self,
        inventory_assessment_ref: impl Into<String>,
    ) -> Self {
        self.inventory_assessment_ref = inventory_assessment_ref.into();
        self
    }

    pub fn with_candidate_ref(mut self, candidate_ref: impl Into<String>) -> Self {
        self.candidate_ref = candidate_ref.into();
        self
    }

    pub fn with_channel_ref(mut self, channel_ref: impl Into<String>) -> Self {
        self.channel_ref = channel_ref.into();
        self
    }

    pub fn with_connector_profile_resolution_ref(
        mut self,
        connector_profile_resolution_ref: impl Into<String>,
    ) -> Self {
        self.connector_profile_resolution_ref = connector_profile_resolution_ref.into();
        self
    }

    pub fn with_connector_profile_ref(mut self, connector_profile_ref: impl Into<String>) -> Self {
        self.connector_profile_ref = connector_profile_ref.into();
        self
    }

    pub fn with_selected_connector_strategy_ref(
        mut self,
        selected_connector_strategy_ref: impl Into<String>,
    ) -> Self {
        self.selected_connector_strategy_ref = selected_connector_strategy_ref.into();
        self
    }

    pub fn with_side_effect_request_ref(
        mut self,
        side_effect_request_ref: impl Into<String>,
    ) -> Self {
        self.side_effect_request_ref = side_effect_request_ref.into();
        self
    }

    pub fn with_intent_ref(mut self, intent_ref: impl Into<String>) -> Self {
        self.intent_ref = intent_ref.into();
        self
    }

    pub fn with_policy_ref(mut self, policy_ref: impl Into<String>) -> Self {
        self.policy_ref = policy_ref.into();
        self
    }

    pub fn with_adapter_ref(mut self, adapter_ref: impl Into<String>) -> Self {
        self.adapter_ref = adapter_ref.into();
        self
    }

    pub fn with_payload_ref(mut self, payload_ref: impl Into<String>) -> Self {
        self.payload_ref = payload_ref.into();
        self
    }

    pub fn with_receipt_proposal_ref(mut self, receipt_proposal_ref: impl Into<String>) -> Self {
        self.receipt_proposal_ref = receipt_proposal_ref.into();
        self
    }

    pub fn with_allowed_source_refs(mut self, allowed_source_refs: Vec<impl Into<String>>) -> Self {
        self.allowed_source_refs = allowed_source_refs.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_instruction_refs(mut self, instruction_refs: Vec<impl Into<String>>) -> Self {
        self.instruction_refs = instruction_refs.into_iter().map(Into::into).collect();
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

    pub fn with_token_cost_ref(mut self, token_cost_ref: impl Into<String>) -> Self {
        self.token_cost_ref = Some(token_cost_ref.into());
        self
    }

    pub fn try_into_side_effect_request_refs(
        &self,
    ) -> Result<PubPunkPublishSideEffectRequestRefs, PubPunkPublishRequestPacketAssessment> {
        let assessment = assess_pubpunk_publish_request_packet(self);
        if assessment.has_blockers() {
            return Err(assessment);
        }

        Ok(PubPunkPublishSideEffectRequestRefs {
            request_ref: self.side_effect_request_ref.clone(),
            target_ref: self.channel_ref.clone(),
            connector_profile_resolution_ref: self.connector_profile_resolution_ref.clone(),
            connector_profile_ref: self.connector_profile_ref.clone(),
            selected_connector_strategy_ref: self.selected_connector_strategy_ref.clone(),
            intent_ref: self.intent_ref.clone(),
            policy_ref: self.policy_ref.clone(),
            adapter_ref: self.adapter_ref.clone(),
            payload_ref: self.payload_ref.clone(),
            receipt_proposal_ref: self.receipt_proposal_ref.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishSideEffectRequestRefs {
    pub request_ref: String,
    pub target_ref: String,
    pub connector_profile_resolution_ref: String,
    pub connector_profile_ref: String,
    pub selected_connector_strategy_ref: String,
    pub intent_ref: String,
    pub policy_ref: String,
    pub adapter_ref: String,
    pub payload_ref: String,
    pub receipt_proposal_ref: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkPublishRequestPacketFindingCode {
    MissingModuleId,
    NonCanonicalModuleId,
    MissingModuleVersionRef,
    MissingContractRef,
    MissingRunRef,
    MissingProjectRef,
    UnsupportedWorkspacePolicy,
    MissingPublishingWorkspaceRef,
    UnsafePublishingWorkspaceRef,
    MissingInventoryAssessmentRef,
    UnsafeInventoryAssessmentRef,
    MissingCandidateRef,
    UnsafeCandidateRef,
    CandidateRefNotAllowed,
    MissingChannelRef,
    UnsafeChannelRef,
    ChannelRefNotAllowed,
    MissingConnectorProfileResolutionRef,
    UnsafeConnectorProfileResolutionRef,
    ConnectorProfileResolutionRefNotAllowed,
    MissingConnectorProfileRef,
    UnsafeConnectorProfileRef,
    ConnectorProfileRefNotAllowed,
    MissingSelectedConnectorStrategyRef,
    UnsafeSelectedConnectorStrategyRef,
    SelectedConnectorStrategyRefNotAllowed,
    MissingSideEffectRequestRef,
    UnsafeSideEffectRequestRef,
    MissingIntentRef,
    UnsafeIntentRef,
    MissingPolicyRef,
    UnsafePolicyRef,
    MissingAdapterRef,
    UnsafeAdapterRef,
    MissingPayloadRef,
    UnsafePayloadRef,
    PayloadRefNotAllowed,
    MissingReceiptProposalRef,
    UnsafeReceiptProposalRef,
    MissingInstructionRefs,
    MissingRequiredInstructionRef,
    UnsafeInstructionRef,
    UnsafeAllowedSourceRef,
    MissingRequestExternalPublishGrant,
    UnsupportedCapabilityGrant,
    MissingExpectedReceiptFields,
    MissingRequiredExpectedReceiptField,
    UnsafePrivacyPolicy,
    UnsafeTokenCostRef,
}

impl PubPunkPublishRequestPacketFindingCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingModuleId => "missing_module_id",
            Self::NonCanonicalModuleId => "non_canonical_module_id",
            Self::MissingModuleVersionRef => "missing_module_version_ref",
            Self::MissingContractRef => "missing_contract_ref",
            Self::MissingRunRef => "missing_run_ref",
            Self::MissingProjectRef => "missing_project_ref",
            Self::UnsupportedWorkspacePolicy => "unsupported_workspace_policy",
            Self::MissingPublishingWorkspaceRef => "missing_publishing_workspace_ref",
            Self::UnsafePublishingWorkspaceRef => "unsafe_publishing_workspace_ref",
            Self::MissingInventoryAssessmentRef => "missing_inventory_assessment_ref",
            Self::UnsafeInventoryAssessmentRef => "unsafe_inventory_assessment_ref",
            Self::MissingCandidateRef => "missing_candidate_ref",
            Self::UnsafeCandidateRef => "unsafe_candidate_ref",
            Self::CandidateRefNotAllowed => "candidate_ref_not_allowed",
            Self::MissingChannelRef => "missing_channel_ref",
            Self::UnsafeChannelRef => "unsafe_channel_ref",
            Self::ChannelRefNotAllowed => "channel_ref_not_allowed",
            Self::MissingConnectorProfileResolutionRef => {
                "missing_connector_profile_resolution_ref"
            }
            Self::UnsafeConnectorProfileResolutionRef => "unsafe_connector_profile_resolution_ref",
            Self::ConnectorProfileResolutionRefNotAllowed => {
                "connector_profile_resolution_ref_not_allowed"
            }
            Self::MissingConnectorProfileRef => "missing_connector_profile_ref",
            Self::UnsafeConnectorProfileRef => "unsafe_connector_profile_ref",
            Self::ConnectorProfileRefNotAllowed => "connector_profile_ref_not_allowed",
            Self::MissingSelectedConnectorStrategyRef => "missing_selected_connector_strategy_ref",
            Self::UnsafeSelectedConnectorStrategyRef => "unsafe_selected_connector_strategy_ref",
            Self::SelectedConnectorStrategyRefNotAllowed => {
                "selected_connector_strategy_ref_not_allowed"
            }
            Self::MissingSideEffectRequestRef => "missing_side_effect_request_ref",
            Self::UnsafeSideEffectRequestRef => "unsafe_side_effect_request_ref",
            Self::MissingIntentRef => "missing_intent_ref",
            Self::UnsafeIntentRef => "unsafe_intent_ref",
            Self::MissingPolicyRef => "missing_policy_ref",
            Self::UnsafePolicyRef => "unsafe_policy_ref",
            Self::MissingAdapterRef => "missing_adapter_ref",
            Self::UnsafeAdapterRef => "unsafe_adapter_ref",
            Self::MissingPayloadRef => "missing_payload_ref",
            Self::UnsafePayloadRef => "unsafe_payload_ref",
            Self::PayloadRefNotAllowed => "payload_ref_not_allowed",
            Self::MissingReceiptProposalRef => "missing_receipt_proposal_ref",
            Self::UnsafeReceiptProposalRef => "unsafe_receipt_proposal_ref",
            Self::MissingInstructionRefs => "missing_instruction_refs",
            Self::MissingRequiredInstructionRef => "missing_required_instruction_ref",
            Self::UnsafeInstructionRef => "unsafe_instruction_ref",
            Self::UnsafeAllowedSourceRef => "unsafe_allowed_source_ref",
            Self::MissingRequestExternalPublishGrant => "missing_request_external_publish_grant",
            Self::UnsupportedCapabilityGrant => "unsupported_capability_grant",
            Self::MissingExpectedReceiptFields => "missing_expected_receipt_fields",
            Self::MissingRequiredExpectedReceiptField => "missing_required_expected_receipt_field",
            Self::UnsafePrivacyPolicy => "unsafe_privacy_policy",
            Self::UnsafeTokenCostRef => "unsafe_token_cost_ref",
        }
    }

    pub fn is_blocking(self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishRequestPacketFinding {
    pub code: PubPunkPublishRequestPacketFindingCode,
    pub ref_value: Option<String>,
    pub capability: Option<PubPunkCapabilityGrant>,
    pub message: &'static str,
}

impl PubPunkPublishRequestPacketFinding {
    fn new(code: PubPunkPublishRequestPacketFindingCode, message: &'static str) -> Self {
        Self {
            code,
            ref_value: None,
            capability: None,
            message,
        }
    }

    fn for_ref(
        code: PubPunkPublishRequestPacketFindingCode,
        ref_value: impl Into<String>,
        message: &'static str,
    ) -> Self {
        Self {
            code,
            ref_value: Some(ref_value.into()),
            capability: None,
            message,
        }
    }

    fn for_capability(capability: PubPunkCapabilityGrant) -> Self {
        Self {
            code: PubPunkPublishRequestPacketFindingCode::UnsupportedCapabilityGrant,
            ref_value: None,
            capability: Some(capability),
            message: "capability is not available in the side-effect-free publish request packet",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishRequestPacketRefs {
    pub module_id: String,
    pub module_version_ref: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub workspace_policy: PubPunkWorkspacePolicy,
    pub publishing_workspace_ref: String,
    pub inventory_assessment_ref: String,
    pub candidate_ref: String,
    pub channel_ref: String,
    pub connector_profile_resolution_ref: String,
    pub connector_profile_ref: String,
    pub selected_connector_strategy_ref: String,
    pub side_effect_request_ref: String,
    pub intent_ref: String,
    pub policy_ref: String,
    pub adapter_ref: String,
    pub payload_ref: String,
    pub receipt_proposal_ref: String,
    pub token_cost_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishRequestPacketAssessment {
    pub schema_version: &'static str,
    pub status: PubPunkAssessmentStatus,
    pub authority: PubPunkAssessmentAuthority,
    pub requested_operation: PubPunkPublishRequestOperation,
    pub findings: Vec<PubPunkPublishRequestPacketFinding>,
    pub boundary_flags: PubPunkInventoryBoundaryFlags,
    pub refs: PubPunkPublishRequestPacketRefs,
}

impl PubPunkPublishRequestPacketAssessment {
    pub fn blocking_findings(&self) -> impl Iterator<Item = &PubPunkPublishRequestPacketFinding> {
        self.findings
            .iter()
            .filter(|finding| finding.code.is_blocking())
    }

    pub fn has_blockers(&self) -> bool {
        self.blocking_findings().next().is_some()
    }
}

pub fn assess_pubpunk_publish_request_packet(
    packet: &PubPunkPublishRequestPacket,
) -> PubPunkPublishRequestPacketAssessment {
    let mut findings = Vec::new();

    push_publish_request_required_ref_finding(
        &mut findings,
        packet.module_id.as_str(),
        PubPunkPublishRequestPacketFindingCode::MissingModuleId,
        "module id is required",
    );
    if !packet.module_id.trim().is_empty() && packet.module_id != PUBPUNK_MODULE_ID {
        findings.push(PubPunkPublishRequestPacketFinding::for_ref(
            PubPunkPublishRequestPacketFindingCode::NonCanonicalModuleId,
            packet.module_id.clone(),
            "PubPunk publish request packets must use the canonical pubpunk module id",
        ));
    }
    push_publish_request_required_ref_finding(
        &mut findings,
        packet.module_version_ref.as_str(),
        PubPunkPublishRequestPacketFindingCode::MissingModuleVersionRef,
        "module version ref is required",
    );
    push_publish_request_required_ref_finding(
        &mut findings,
        packet.contract_ref.as_str(),
        PubPunkPublishRequestPacketFindingCode::MissingContractRef,
        "contract ref is required",
    );
    push_publish_request_required_ref_finding(
        &mut findings,
        packet.run_ref.as_str(),
        PubPunkPublishRequestPacketFindingCode::MissingRunRef,
        "run ref is required",
    );
    push_publish_request_required_ref_finding(
        &mut findings,
        packet.project_ref.as_str(),
        PubPunkPublishRequestPacketFindingCode::MissingProjectRef,
        "project ref is required",
    );

    if !packet.workspace_policy.selected_for_first_slice() {
        findings.push(PubPunkPublishRequestPacketFinding::for_ref(
            PubPunkPublishRequestPacketFindingCode::UnsupportedWorkspacePolicy,
            packet.workspace_policy.as_str(),
            "the first PubPunk publish request packet supports split explicit refs only",
        ));
    }

    push_publish_request_required_ref_finding(
        &mut findings,
        packet.publishing_workspace_ref.as_str(),
        PubPunkPublishRequestPacketFindingCode::MissingPublishingWorkspaceRef,
        "publishing workspace ref is required",
    );
    if !packet.publishing_workspace_ref.trim().is_empty()
        && !is_safe_workspace_ref(&packet.publishing_workspace_ref)
    {
        findings.push(PubPunkPublishRequestPacketFinding::for_ref(
            PubPunkPublishRequestPacketFindingCode::UnsafePublishingWorkspaceRef,
            packet.publishing_workspace_ref.clone(),
            "publishing workspace ref must be an explicit safe logical or repo-relative ref",
        ));
    }

    validate_publish_request_ref(
        &mut findings,
        packet.inventory_assessment_ref.as_str(),
        PubPunkPublishRequestPacketFindingCode::MissingInventoryAssessmentRef,
        PubPunkPublishRequestPacketFindingCode::UnsafeInventoryAssessmentRef,
        "inventory assessment ref is required",
        "inventory assessment ref must be an explicit repo-relative ref",
    );
    validate_publish_request_ref(
        &mut findings,
        packet.candidate_ref.as_str(),
        PubPunkPublishRequestPacketFindingCode::MissingCandidateRef,
        PubPunkPublishRequestPacketFindingCode::UnsafeCandidateRef,
        "candidate ref is required",
        "candidate ref must be an explicit repo-relative ref",
    );
    validate_publish_request_ref(
        &mut findings,
        packet.channel_ref.as_str(),
        PubPunkPublishRequestPacketFindingCode::MissingChannelRef,
        PubPunkPublishRequestPacketFindingCode::UnsafeChannelRef,
        "channel ref is required",
        "channel ref must be an explicit repo-relative ref",
    );
    validate_publish_request_ref(
        &mut findings,
        packet.connector_profile_resolution_ref.as_str(),
        PubPunkPublishRequestPacketFindingCode::MissingConnectorProfileResolutionRef,
        PubPunkPublishRequestPacketFindingCode::UnsafeConnectorProfileResolutionRef,
        "connector profile resolution ref is required",
        "connector profile resolution ref must be an explicit repo-relative ref",
    );
    validate_publish_request_ref(
        &mut findings,
        packet.connector_profile_ref.as_str(),
        PubPunkPublishRequestPacketFindingCode::MissingConnectorProfileRef,
        PubPunkPublishRequestPacketFindingCode::UnsafeConnectorProfileRef,
        "connector profile ref is required",
        "connector profile ref must be an explicit repo-relative ref",
    );
    validate_publish_request_ref(
        &mut findings,
        packet.selected_connector_strategy_ref.as_str(),
        PubPunkPublishRequestPacketFindingCode::MissingSelectedConnectorStrategyRef,
        PubPunkPublishRequestPacketFindingCode::UnsafeSelectedConnectorStrategyRef,
        "selected connector strategy ref is required",
        "selected connector strategy ref must be an explicit repo-relative ref",
    );
    validate_publish_request_ref(
        &mut findings,
        packet.side_effect_request_ref.as_str(),
        PubPunkPublishRequestPacketFindingCode::MissingSideEffectRequestRef,
        PubPunkPublishRequestPacketFindingCode::UnsafeSideEffectRequestRef,
        "side-effect request ref is required",
        "side-effect request ref must be an explicit repo-relative ref",
    );
    validate_publish_request_ref(
        &mut findings,
        packet.intent_ref.as_str(),
        PubPunkPublishRequestPacketFindingCode::MissingIntentRef,
        PubPunkPublishRequestPacketFindingCode::UnsafeIntentRef,
        "intent ref is required",
        "intent ref must be an explicit repo-relative ref",
    );
    validate_publish_request_ref(
        &mut findings,
        packet.policy_ref.as_str(),
        PubPunkPublishRequestPacketFindingCode::MissingPolicyRef,
        PubPunkPublishRequestPacketFindingCode::UnsafePolicyRef,
        "policy ref is required",
        "policy ref must be an explicit repo-relative ref",
    );
    validate_publish_request_ref(
        &mut findings,
        packet.adapter_ref.as_str(),
        PubPunkPublishRequestPacketFindingCode::MissingAdapterRef,
        PubPunkPublishRequestPacketFindingCode::UnsafeAdapterRef,
        "adapter ref is required",
        "adapter ref must be an explicit repo-relative ref",
    );
    validate_publish_request_ref(
        &mut findings,
        packet.payload_ref.as_str(),
        PubPunkPublishRequestPacketFindingCode::MissingPayloadRef,
        PubPunkPublishRequestPacketFindingCode::UnsafePayloadRef,
        "payload ref is required",
        "payload ref must be an explicit repo-relative ref",
    );
    validate_publish_request_ref(
        &mut findings,
        packet.receipt_proposal_ref.as_str(),
        PubPunkPublishRequestPacketFindingCode::MissingReceiptProposalRef,
        PubPunkPublishRequestPacketFindingCode::UnsafeReceiptProposalRef,
        "receipt proposal ref is required",
        "receipt proposal ref must be an explicit repo-relative ref",
    );

    if packet.instruction_refs.is_empty() {
        findings.push(PubPunkPublishRequestPacketFinding::new(
            PubPunkPublishRequestPacketFindingCode::MissingInstructionRefs,
            "instruction refs are required for PubPunk publish request packets",
        ));
    }

    for required_ref in PUBPUNK_REQUIRED_INSTRUCTION_REFS {
        if !packet
            .instruction_refs
            .iter()
            .any(|instruction_ref| instruction_ref == required_ref)
        {
            findings.push(PubPunkPublishRequestPacketFinding::for_ref(
                PubPunkPublishRequestPacketFindingCode::MissingRequiredInstructionRef,
                *required_ref,
                "required PubPunk instruction ref is missing",
            ));
        }
    }

    for instruction_ref in &packet.instruction_refs {
        if !is_safe_source_ref(instruction_ref) {
            findings.push(PubPunkPublishRequestPacketFinding::for_ref(
                PubPunkPublishRequestPacketFindingCode::UnsafeInstructionRef,
                instruction_ref.clone(),
                "instruction refs must be explicit repo-relative refs",
            ));
        }
    }

    for source_ref in &packet.allowed_source_refs {
        if !is_safe_source_ref(source_ref) {
            findings.push(PubPunkPublishRequestPacketFinding::for_ref(
                PubPunkPublishRequestPacketFindingCode::UnsafeAllowedSourceRef,
                source_ref.clone(),
                "allowed source refs must be explicit repo-relative refs",
            ));
        }
    }

    if !packet.candidate_ref.trim().is_empty()
        && !packet.allowed_source_refs.contains(&packet.candidate_ref)
    {
        findings.push(PubPunkPublishRequestPacketFinding::for_ref(
            PubPunkPublishRequestPacketFindingCode::CandidateRefNotAllowed,
            packet.candidate_ref.clone(),
            "candidate ref must be included in allowed source refs",
        ));
    }
    if !packet.channel_ref.trim().is_empty()
        && !packet.allowed_source_refs.contains(&packet.channel_ref)
    {
        findings.push(PubPunkPublishRequestPacketFinding::for_ref(
            PubPunkPublishRequestPacketFindingCode::ChannelRefNotAllowed,
            packet.channel_ref.clone(),
            "channel ref must be included in allowed source refs",
        ));
    }
    if !packet.connector_profile_resolution_ref.trim().is_empty()
        && !packet
            .allowed_source_refs
            .contains(&packet.connector_profile_resolution_ref)
    {
        findings.push(PubPunkPublishRequestPacketFinding::for_ref(
            PubPunkPublishRequestPacketFindingCode::ConnectorProfileResolutionRefNotAllowed,
            packet.connector_profile_resolution_ref.clone(),
            "connector profile resolution ref must be included in allowed source refs",
        ));
    }
    if !packet.connector_profile_ref.trim().is_empty()
        && !packet
            .allowed_source_refs
            .contains(&packet.connector_profile_ref)
    {
        findings.push(PubPunkPublishRequestPacketFinding::for_ref(
            PubPunkPublishRequestPacketFindingCode::ConnectorProfileRefNotAllowed,
            packet.connector_profile_ref.clone(),
            "connector profile ref must be included in allowed source refs",
        ));
    }
    if !packet.selected_connector_strategy_ref.trim().is_empty()
        && !packet
            .allowed_source_refs
            .contains(&packet.selected_connector_strategy_ref)
    {
        findings.push(PubPunkPublishRequestPacketFinding::for_ref(
            PubPunkPublishRequestPacketFindingCode::SelectedConnectorStrategyRefNotAllowed,
            packet.selected_connector_strategy_ref.clone(),
            "selected connector strategy ref must be included in allowed source refs",
        ));
    }
    if !packet.payload_ref.trim().is_empty()
        && !packet.allowed_source_refs.contains(&packet.payload_ref)
    {
        findings.push(PubPunkPublishRequestPacketFinding::for_ref(
            PubPunkPublishRequestPacketFindingCode::PayloadRefNotAllowed,
            packet.payload_ref.clone(),
            "payload ref must be included in allowed source refs",
        ));
    }

    if !packet
        .granted_capabilities
        .contains(&PubPunkCapabilityGrant::RequestExternalPublish)
    {
        findings.push(PubPunkPublishRequestPacketFinding::new(
            PubPunkPublishRequestPacketFindingCode::MissingRequestExternalPublishGrant,
            "request_external_publish must be explicitly granted for this packet",
        ));
    }
    for capability in &packet.granted_capabilities {
        if !capability.supported_by_side_effect_free_publish_request() {
            findings.push(PubPunkPublishRequestPacketFinding::for_capability(
                *capability,
            ));
        }
    }

    if packet.expected_receipt_fields.is_empty() {
        findings.push(PubPunkPublishRequestPacketFinding::new(
            PubPunkPublishRequestPacketFindingCode::MissingExpectedReceiptFields,
            "expected receipt fields are required even though this packet does not write receipts",
        ));
    }
    for required_field in [
        "side_effects",
        "host_validation",
        "connector_profile_resolution",
        "connector_profile_ref",
        "selected_connector_strategy",
    ] {
        if !packet
            .expected_receipt_fields
            .iter()
            .any(|field| field == required_field)
        {
            findings.push(PubPunkPublishRequestPacketFinding::for_ref(
                PubPunkPublishRequestPacketFindingCode::MissingRequiredExpectedReceiptField,
                required_field,
                "publish request receipt expectations must include host side-effect coverage",
            ));
        }
    }

    if packet.privacy_policy.allows_private_or_raw_payloads() {
        findings.push(PubPunkPublishRequestPacketFinding::new(
            PubPunkPublishRequestPacketFindingCode::UnsafePrivacyPolicy,
            "privacy policy must disallow raw/private payloads for this packet",
        ));
    }

    if let Some(token_cost_ref) = &packet.token_cost_ref {
        if !is_safe_source_ref(token_cost_ref) {
            findings.push(PubPunkPublishRequestPacketFinding::for_ref(
                PubPunkPublishRequestPacketFindingCode::UnsafeTokenCostRef,
                token_cost_ref.clone(),
                "token cost ref must be an explicit repo-relative ref when provided",
            ));
        }
    }

    let status = if findings.iter().any(|finding| finding.code.is_blocking()) {
        PubPunkAssessmentStatus::Blocked
    } else {
        PubPunkAssessmentStatus::Ready
    };

    PubPunkPublishRequestPacketAssessment {
        schema_version: PUBPUNK_PUBLISH_REQUEST_PACKET_SCHEMA_VERSION,
        status,
        authority: PubPunkAssessmentAuthority::Advisory,
        requested_operation: PubPunkPublishRequestOperation::RequestExternalPublish,
        findings,
        boundary_flags: PUBPUNK_INVENTORY_ASSESSMENT_BOUNDARY_FLAGS,
        refs: PubPunkPublishRequestPacketRefs {
            module_id: packet.module_id.clone(),
            module_version_ref: packet.module_version_ref.clone(),
            contract_ref: packet.contract_ref.clone(),
            run_ref: packet.run_ref.clone(),
            project_ref: packet.project_ref.clone(),
            workspace_policy: packet.workspace_policy,
            publishing_workspace_ref: packet.publishing_workspace_ref.clone(),
            inventory_assessment_ref: packet.inventory_assessment_ref.clone(),
            candidate_ref: packet.candidate_ref.clone(),
            channel_ref: packet.channel_ref.clone(),
            connector_profile_resolution_ref: packet.connector_profile_resolution_ref.clone(),
            connector_profile_ref: packet.connector_profile_ref.clone(),
            selected_connector_strategy_ref: packet.selected_connector_strategy_ref.clone(),
            side_effect_request_ref: packet.side_effect_request_ref.clone(),
            intent_ref: packet.intent_ref.clone(),
            policy_ref: packet.policy_ref.clone(),
            adapter_ref: packet.adapter_ref.clone(),
            payload_ref: packet.payload_ref.clone(),
            receipt_proposal_ref: packet.receipt_proposal_ref.clone(),
            token_cost_ref: packet.token_cost_ref.clone(),
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkPublishReceiptPreflightOperation {
    PreparePublishReceiptPreflight,
}

impl PubPunkPublishReceiptPreflightOperation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PreparePublishReceiptPreflight => "prepare_publish_receipt_preflight",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishReceiptPreflightPacket {
    pub module_id: String,
    pub module_version_ref: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub workspace_policy: PubPunkWorkspacePolicy,
    pub publishing_workspace_ref: String,
    pub publish_request_ref: String,
    pub receipt_writer_preflight_ref: String,
    pub policy_gate_preflight_ref: String,
    pub receipt_target_ref: String,
    pub receipt_storage_ref: String,
    pub operation_evidence_ref: String,
    pub idempotency_ref: String,
    pub rollback_ref: String,
    pub error_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub channel_ref: String,
    pub connector_profile_resolution_ref: String,
    pub connector_profile_ref: String,
    pub selected_connector_strategy_ref: String,
    pub allowed_source_refs: Vec<String>,
    pub instruction_refs: Vec<String>,
    pub granted_capabilities: Vec<PubPunkCapabilityGrant>,
    pub privacy_policy: PubPunkPrivacyPolicy,
    pub expected_receipt_fields: Vec<String>,
    pub token_cost_ref: Option<String>,
}

impl PubPunkPublishReceiptPreflightPacket {
    pub fn new(
        module_version_ref: impl Into<String>,
        contract_ref: impl Into<String>,
        run_ref: impl Into<String>,
        project_ref: impl Into<String>,
        publishing_workspace_ref: impl Into<String>,
    ) -> Self {
        Self {
            module_id: PUBPUNK_MODULE_ID.to_owned(),
            module_version_ref: module_version_ref.into(),
            contract_ref: contract_ref.into(),
            run_ref: run_ref.into(),
            project_ref: project_ref.into(),
            workspace_policy: PubPunkWorkspacePolicy::SplitExplicitRefs,
            publishing_workspace_ref: publishing_workspace_ref.into(),
            publish_request_ref: String::new(),
            receipt_writer_preflight_ref: String::new(),
            policy_gate_preflight_ref: String::new(),
            receipt_target_ref: String::new(),
            receipt_storage_ref: String::new(),
            operation_evidence_ref: String::new(),
            idempotency_ref: String::new(),
            rollback_ref: String::new(),
            error_ref: String::new(),
            adapter_invocation_receipt_ref: String::new(),
            payload_ref: String::new(),
            channel_ref: String::new(),
            connector_profile_resolution_ref: String::new(),
            connector_profile_ref: String::new(),
            selected_connector_strategy_ref: String::new(),
            allowed_source_refs: Vec::new(),
            instruction_refs: Vec::new(),
            granted_capabilities: Vec::new(),
            privacy_policy: PubPunkPrivacyPolicy::safe_metadata_only(),
            expected_receipt_fields: Vec::new(),
            token_cost_ref: None,
        }
    }

    pub fn with_workspace_policy(mut self, workspace_policy: PubPunkWorkspacePolicy) -> Self {
        self.workspace_policy = workspace_policy;
        self
    }

    pub fn with_publish_request_ref(mut self, publish_request_ref: impl Into<String>) -> Self {
        self.publish_request_ref = publish_request_ref.into();
        self
    }

    pub fn with_receipt_writer_preflight_ref(
        mut self,
        receipt_writer_preflight_ref: impl Into<String>,
    ) -> Self {
        self.receipt_writer_preflight_ref = receipt_writer_preflight_ref.into();
        self
    }

    pub fn with_policy_gate_preflight_ref(
        mut self,
        policy_gate_preflight_ref: impl Into<String>,
    ) -> Self {
        self.policy_gate_preflight_ref = policy_gate_preflight_ref.into();
        self
    }

    pub fn with_receipt_target_ref(mut self, receipt_target_ref: impl Into<String>) -> Self {
        self.receipt_target_ref = receipt_target_ref.into();
        self
    }

    pub fn with_receipt_storage_ref(mut self, receipt_storage_ref: impl Into<String>) -> Self {
        self.receipt_storage_ref = receipt_storage_ref.into();
        self
    }

    pub fn with_operation_evidence_ref(
        mut self,
        operation_evidence_ref: impl Into<String>,
    ) -> Self {
        self.operation_evidence_ref = operation_evidence_ref.into();
        self
    }

    pub fn with_idempotency_ref(mut self, idempotency_ref: impl Into<String>) -> Self {
        self.idempotency_ref = idempotency_ref.into();
        self
    }

    pub fn with_rollback_ref(mut self, rollback_ref: impl Into<String>) -> Self {
        self.rollback_ref = rollback_ref.into();
        self
    }

    pub fn with_error_ref(mut self, error_ref: impl Into<String>) -> Self {
        self.error_ref = error_ref.into();
        self
    }

    pub fn with_adapter_invocation_receipt_ref(
        mut self,
        adapter_invocation_receipt_ref: impl Into<String>,
    ) -> Self {
        self.adapter_invocation_receipt_ref = adapter_invocation_receipt_ref.into();
        self
    }

    pub fn with_payload_ref(mut self, payload_ref: impl Into<String>) -> Self {
        self.payload_ref = payload_ref.into();
        self
    }

    pub fn with_channel_ref(mut self, channel_ref: impl Into<String>) -> Self {
        self.channel_ref = channel_ref.into();
        self
    }

    pub fn with_connector_profile_resolution_ref(
        mut self,
        connector_profile_resolution_ref: impl Into<String>,
    ) -> Self {
        self.connector_profile_resolution_ref = connector_profile_resolution_ref.into();
        self
    }

    pub fn with_connector_profile_ref(mut self, connector_profile_ref: impl Into<String>) -> Self {
        self.connector_profile_ref = connector_profile_ref.into();
        self
    }

    pub fn with_selected_connector_strategy_ref(
        mut self,
        selected_connector_strategy_ref: impl Into<String>,
    ) -> Self {
        self.selected_connector_strategy_ref = selected_connector_strategy_ref.into();
        self
    }

    pub fn with_allowed_source_refs(mut self, allowed_source_refs: Vec<impl Into<String>>) -> Self {
        self.allowed_source_refs = allowed_source_refs.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_instruction_refs(mut self, instruction_refs: Vec<impl Into<String>>) -> Self {
        self.instruction_refs = instruction_refs.into_iter().map(Into::into).collect();
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

    pub fn with_token_cost_ref(mut self, token_cost_ref: impl Into<String>) -> Self {
        self.token_cost_ref = Some(token_cost_ref.into());
        self
    }

    pub fn try_into_receipt_writer_preflight_refs(
        &self,
    ) -> Result<
        PubPunkPublishReceiptWriterPreflightRefs,
        PubPunkPublishReceiptPreflightPacketAssessment,
    > {
        let assessment = assess_pubpunk_publish_receipt_preflight_packet(self);
        if assessment.has_blockers() {
            return Err(assessment);
        }

        Ok(PubPunkPublishReceiptWriterPreflightRefs {
            preflight_ref: self.receipt_writer_preflight_ref.clone(),
            policy_gate_preflight_ref: self.policy_gate_preflight_ref.clone(),
            receipt_target_ref: self.receipt_target_ref.clone(),
            storage_ref: self.receipt_storage_ref.clone(),
            operation_evidence_ref: self.operation_evidence_ref.clone(),
            idempotency_ref: self.idempotency_ref.clone(),
            rollback_ref: self.rollback_ref.clone(),
            error_ref: self.error_ref.clone(),
            adapter_invocation_receipt_ref: self.adapter_invocation_receipt_ref.clone(),
            payload_ref: self.payload_ref.clone(),
            channel_ref: self.channel_ref.clone(),
            connector_profile_resolution_ref: self.connector_profile_resolution_ref.clone(),
            connector_profile_ref: self.connector_profile_ref.clone(),
            selected_connector_strategy_ref: self.selected_connector_strategy_ref.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishReceiptWriterPreflightRefs {
    pub preflight_ref: String,
    pub policy_gate_preflight_ref: String,
    pub receipt_target_ref: String,
    pub storage_ref: String,
    pub operation_evidence_ref: String,
    pub idempotency_ref: String,
    pub rollback_ref: String,
    pub error_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub channel_ref: String,
    pub connector_profile_resolution_ref: String,
    pub connector_profile_ref: String,
    pub selected_connector_strategy_ref: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkPublishReceiptPreflightPacketFindingCode {
    MissingModuleId,
    NonCanonicalModuleId,
    MissingModuleVersionRef,
    MissingContractRef,
    MissingRunRef,
    MissingProjectRef,
    UnsupportedWorkspacePolicy,
    MissingPublishingWorkspaceRef,
    UnsafePublishingWorkspaceRef,
    MissingPublishRequestRef,
    UnsafePublishRequestRef,
    MissingReceiptWriterPreflightRef,
    UnsafeReceiptWriterPreflightRef,
    MissingPolicyGatePreflightRef,
    UnsafePolicyGatePreflightRef,
    MissingReceiptTargetRef,
    UnsafeReceiptTargetRef,
    MissingReceiptStorageRef,
    UnsafeReceiptStorageRef,
    MissingOperationEvidenceRef,
    UnsafeOperationEvidenceRef,
    MissingIdempotencyRef,
    UnsafeIdempotencyRef,
    MissingRollbackRef,
    UnsafeRollbackRef,
    MissingErrorRef,
    UnsafeErrorRef,
    MissingAdapterInvocationReceiptRef,
    UnsafeAdapterInvocationReceiptRef,
    MissingPayloadRef,
    UnsafePayloadRef,
    PayloadRefNotAllowed,
    MissingChannelRef,
    UnsafeChannelRef,
    ChannelRefNotAllowed,
    MissingConnectorProfileResolutionRef,
    UnsafeConnectorProfileResolutionRef,
    ConnectorProfileResolutionRefNotAllowed,
    MissingConnectorProfileRef,
    UnsafeConnectorProfileRef,
    ConnectorProfileRefNotAllowed,
    MissingSelectedConnectorStrategyRef,
    UnsafeSelectedConnectorStrategyRef,
    SelectedConnectorStrategyRefNotAllowed,
    MissingInstructionRefs,
    MissingRequiredInstructionRef,
    UnsafeInstructionRef,
    UnsafeAllowedSourceRef,
    MissingRequestExternalPublishGrant,
    UnsupportedCapabilityGrant,
    MissingExpectedReceiptFields,
    MissingRequiredExpectedReceiptField,
    UnsafePrivacyPolicy,
    UnsafeTokenCostRef,
}

impl PubPunkPublishReceiptPreflightPacketFindingCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingModuleId => "missing_module_id",
            Self::NonCanonicalModuleId => "non_canonical_module_id",
            Self::MissingModuleVersionRef => "missing_module_version_ref",
            Self::MissingContractRef => "missing_contract_ref",
            Self::MissingRunRef => "missing_run_ref",
            Self::MissingProjectRef => "missing_project_ref",
            Self::UnsupportedWorkspacePolicy => "unsupported_workspace_policy",
            Self::MissingPublishingWorkspaceRef => "missing_publishing_workspace_ref",
            Self::UnsafePublishingWorkspaceRef => "unsafe_publishing_workspace_ref",
            Self::MissingPublishRequestRef => "missing_publish_request_ref",
            Self::UnsafePublishRequestRef => "unsafe_publish_request_ref",
            Self::MissingReceiptWriterPreflightRef => "missing_receipt_writer_preflight_ref",
            Self::UnsafeReceiptWriterPreflightRef => "unsafe_receipt_writer_preflight_ref",
            Self::MissingPolicyGatePreflightRef => "missing_policy_gate_preflight_ref",
            Self::UnsafePolicyGatePreflightRef => "unsafe_policy_gate_preflight_ref",
            Self::MissingReceiptTargetRef => "missing_receipt_target_ref",
            Self::UnsafeReceiptTargetRef => "unsafe_receipt_target_ref",
            Self::MissingReceiptStorageRef => "missing_receipt_storage_ref",
            Self::UnsafeReceiptStorageRef => "unsafe_receipt_storage_ref",
            Self::MissingOperationEvidenceRef => "missing_operation_evidence_ref",
            Self::UnsafeOperationEvidenceRef => "unsafe_operation_evidence_ref",
            Self::MissingIdempotencyRef => "missing_idempotency_ref",
            Self::UnsafeIdempotencyRef => "unsafe_idempotency_ref",
            Self::MissingRollbackRef => "missing_rollback_ref",
            Self::UnsafeRollbackRef => "unsafe_rollback_ref",
            Self::MissingErrorRef => "missing_error_ref",
            Self::UnsafeErrorRef => "unsafe_error_ref",
            Self::MissingAdapterInvocationReceiptRef => "missing_adapter_invocation_receipt_ref",
            Self::UnsafeAdapterInvocationReceiptRef => "unsafe_adapter_invocation_receipt_ref",
            Self::MissingPayloadRef => "missing_payload_ref",
            Self::UnsafePayloadRef => "unsafe_payload_ref",
            Self::PayloadRefNotAllowed => "payload_ref_not_allowed",
            Self::MissingChannelRef => "missing_channel_ref",
            Self::UnsafeChannelRef => "unsafe_channel_ref",
            Self::ChannelRefNotAllowed => "channel_ref_not_allowed",
            Self::MissingConnectorProfileResolutionRef => {
                "missing_connector_profile_resolution_ref"
            }
            Self::UnsafeConnectorProfileResolutionRef => "unsafe_connector_profile_resolution_ref",
            Self::ConnectorProfileResolutionRefNotAllowed => {
                "connector_profile_resolution_ref_not_allowed"
            }
            Self::MissingConnectorProfileRef => "missing_connector_profile_ref",
            Self::UnsafeConnectorProfileRef => "unsafe_connector_profile_ref",
            Self::ConnectorProfileRefNotAllowed => "connector_profile_ref_not_allowed",
            Self::MissingSelectedConnectorStrategyRef => "missing_selected_connector_strategy_ref",
            Self::UnsafeSelectedConnectorStrategyRef => "unsafe_selected_connector_strategy_ref",
            Self::SelectedConnectorStrategyRefNotAllowed => {
                "selected_connector_strategy_ref_not_allowed"
            }
            Self::MissingInstructionRefs => "missing_instruction_refs",
            Self::MissingRequiredInstructionRef => "missing_required_instruction_ref",
            Self::UnsafeInstructionRef => "unsafe_instruction_ref",
            Self::UnsafeAllowedSourceRef => "unsafe_allowed_source_ref",
            Self::MissingRequestExternalPublishGrant => "missing_request_external_publish_grant",
            Self::UnsupportedCapabilityGrant => "unsupported_capability_grant",
            Self::MissingExpectedReceiptFields => "missing_expected_receipt_fields",
            Self::MissingRequiredExpectedReceiptField => "missing_required_expected_receipt_field",
            Self::UnsafePrivacyPolicy => "unsafe_privacy_policy",
            Self::UnsafeTokenCostRef => "unsafe_token_cost_ref",
        }
    }

    pub fn is_blocking(self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishReceiptPreflightPacketFinding {
    pub code: PubPunkPublishReceiptPreflightPacketFindingCode,
    pub ref_value: Option<String>,
    pub capability: Option<PubPunkCapabilityGrant>,
    pub message: &'static str,
}

impl PubPunkPublishReceiptPreflightPacketFinding {
    fn new(code: PubPunkPublishReceiptPreflightPacketFindingCode, message: &'static str) -> Self {
        Self {
            code,
            ref_value: None,
            capability: None,
            message,
        }
    }

    fn for_ref(
        code: PubPunkPublishReceiptPreflightPacketFindingCode,
        ref_value: impl Into<String>,
        message: &'static str,
    ) -> Self {
        Self {
            code,
            ref_value: Some(ref_value.into()),
            capability: None,
            message,
        }
    }

    fn for_capability(capability: PubPunkCapabilityGrant) -> Self {
        Self {
            code: PubPunkPublishReceiptPreflightPacketFindingCode::UnsupportedCapabilityGrant,
            ref_value: None,
            capability: Some(capability),
            message: "capability is not available in the side-effect-free publish receipt preflight packet",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishReceiptPreflightPacketRefs {
    pub module_id: String,
    pub module_version_ref: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub workspace_policy: PubPunkWorkspacePolicy,
    pub publishing_workspace_ref: String,
    pub publish_request_ref: String,
    pub receipt_writer_preflight_ref: String,
    pub policy_gate_preflight_ref: String,
    pub receipt_target_ref: String,
    pub receipt_storage_ref: String,
    pub operation_evidence_ref: String,
    pub idempotency_ref: String,
    pub rollback_ref: String,
    pub error_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub channel_ref: String,
    pub connector_profile_resolution_ref: String,
    pub connector_profile_ref: String,
    pub selected_connector_strategy_ref: String,
    pub token_cost_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishReceiptPreflightPacketAssessment {
    pub schema_version: &'static str,
    pub status: PubPunkAssessmentStatus,
    pub authority: PubPunkAssessmentAuthority,
    pub requested_operation: PubPunkPublishReceiptPreflightOperation,
    pub findings: Vec<PubPunkPublishReceiptPreflightPacketFinding>,
    pub boundary_flags: PubPunkInventoryBoundaryFlags,
    pub refs: PubPunkPublishReceiptPreflightPacketRefs,
}

impl PubPunkPublishReceiptPreflightPacketAssessment {
    pub fn blocking_findings(
        &self,
    ) -> impl Iterator<Item = &PubPunkPublishReceiptPreflightPacketFinding> {
        self.findings
            .iter()
            .filter(|finding| finding.code.is_blocking())
    }

    pub fn has_blockers(&self) -> bool {
        self.blocking_findings().next().is_some()
    }
}

pub fn assess_pubpunk_publish_receipt_preflight_packet(
    packet: &PubPunkPublishReceiptPreflightPacket,
) -> PubPunkPublishReceiptPreflightPacketAssessment {
    let mut findings = Vec::new();

    push_publish_receipt_preflight_required_ref_finding(
        &mut findings,
        packet.module_id.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingModuleId,
        "module id is required",
    );
    if !packet.module_id.trim().is_empty() && packet.module_id != PUBPUNK_MODULE_ID {
        findings.push(PubPunkPublishReceiptPreflightPacketFinding::for_ref(
            PubPunkPublishReceiptPreflightPacketFindingCode::NonCanonicalModuleId,
            packet.module_id.clone(),
            "PubPunk publish receipt preflight packets must use the canonical pubpunk module id",
        ));
    }
    push_publish_receipt_preflight_required_ref_finding(
        &mut findings,
        packet.module_version_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingModuleVersionRef,
        "module version ref is required",
    );
    push_publish_receipt_preflight_required_ref_finding(
        &mut findings,
        packet.contract_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingContractRef,
        "contract ref is required",
    );
    push_publish_receipt_preflight_required_ref_finding(
        &mut findings,
        packet.run_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingRunRef,
        "run ref is required",
    );
    push_publish_receipt_preflight_required_ref_finding(
        &mut findings,
        packet.project_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingProjectRef,
        "project ref is required",
    );

    if !packet.workspace_policy.selected_for_first_slice() {
        findings.push(PubPunkPublishReceiptPreflightPacketFinding::for_ref(
            PubPunkPublishReceiptPreflightPacketFindingCode::UnsupportedWorkspacePolicy,
            packet.workspace_policy.as_str(),
            "the first PubPunk publish receipt preflight packet supports split explicit refs only",
        ));
    }

    push_publish_receipt_preflight_required_ref_finding(
        &mut findings,
        packet.publishing_workspace_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingPublishingWorkspaceRef,
        "publishing workspace ref is required",
    );
    if !packet.publishing_workspace_ref.trim().is_empty()
        && !is_safe_workspace_ref(&packet.publishing_workspace_ref)
    {
        findings.push(PubPunkPublishReceiptPreflightPacketFinding::for_ref(
            PubPunkPublishReceiptPreflightPacketFindingCode::UnsafePublishingWorkspaceRef,
            packet.publishing_workspace_ref.clone(),
            "publishing workspace ref must be an explicit safe logical or repo-relative ref",
        ));
    }

    validate_publish_receipt_preflight_ref(
        &mut findings,
        packet.publish_request_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingPublishRequestRef,
        PubPunkPublishReceiptPreflightPacketFindingCode::UnsafePublishRequestRef,
        "publish request ref is required",
        "publish request ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_preflight_ref(
        &mut findings,
        packet.receipt_writer_preflight_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingReceiptWriterPreflightRef,
        PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeReceiptWriterPreflightRef,
        "receipt writer preflight ref is required",
        "receipt writer preflight ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_preflight_ref(
        &mut findings,
        packet.policy_gate_preflight_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingPolicyGatePreflightRef,
        PubPunkPublishReceiptPreflightPacketFindingCode::UnsafePolicyGatePreflightRef,
        "policy gate preflight ref is required",
        "policy gate preflight ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_preflight_ref(
        &mut findings,
        packet.receipt_target_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingReceiptTargetRef,
        PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeReceiptTargetRef,
        "receipt target ref is required",
        "receipt target ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_preflight_ref(
        &mut findings,
        packet.receipt_storage_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingReceiptStorageRef,
        PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeReceiptStorageRef,
        "receipt storage ref is required",
        "receipt storage ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_preflight_ref(
        &mut findings,
        packet.operation_evidence_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingOperationEvidenceRef,
        PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeOperationEvidenceRef,
        "operation evidence ref is required",
        "operation evidence ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_preflight_ref(
        &mut findings,
        packet.idempotency_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingIdempotencyRef,
        PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeIdempotencyRef,
        "idempotency ref is required",
        "idempotency ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_preflight_ref(
        &mut findings,
        packet.rollback_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingRollbackRef,
        PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeRollbackRef,
        "rollback ref is required",
        "rollback ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_preflight_ref(
        &mut findings,
        packet.error_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingErrorRef,
        PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeErrorRef,
        "error ref is required",
        "error ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_preflight_ref(
        &mut findings,
        packet.adapter_invocation_receipt_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingAdapterInvocationReceiptRef,
        PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeAdapterInvocationReceiptRef,
        "adapter invocation receipt ref is required",
        "adapter invocation receipt ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_preflight_ref(
        &mut findings,
        packet.payload_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingPayloadRef,
        PubPunkPublishReceiptPreflightPacketFindingCode::UnsafePayloadRef,
        "payload ref is required",
        "payload ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_preflight_ref(
        &mut findings,
        packet.channel_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingChannelRef,
        PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeChannelRef,
        "channel ref is required",
        "channel ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_preflight_ref(
        &mut findings,
        packet.connector_profile_resolution_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingConnectorProfileResolutionRef,
        PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeConnectorProfileResolutionRef,
        "connector profile resolution ref is required",
        "connector profile resolution ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_preflight_ref(
        &mut findings,
        packet.connector_profile_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingConnectorProfileRef,
        PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeConnectorProfileRef,
        "connector profile ref is required",
        "connector profile ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_preflight_ref(
        &mut findings,
        packet.selected_connector_strategy_ref.as_str(),
        PubPunkPublishReceiptPreflightPacketFindingCode::MissingSelectedConnectorStrategyRef,
        PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeSelectedConnectorStrategyRef,
        "selected connector strategy ref is required",
        "selected connector strategy ref must be an explicit repo-relative ref",
    );

    if packet.instruction_refs.is_empty() {
        findings.push(PubPunkPublishReceiptPreflightPacketFinding::new(
            PubPunkPublishReceiptPreflightPacketFindingCode::MissingInstructionRefs,
            "instruction refs are required for PubPunk publish receipt preflight packets",
        ));
    }

    for required_ref in PUBPUNK_REQUIRED_INSTRUCTION_REFS {
        if !packet
            .instruction_refs
            .iter()
            .any(|instruction_ref| instruction_ref == required_ref)
        {
            findings.push(PubPunkPublishReceiptPreflightPacketFinding::for_ref(
                PubPunkPublishReceiptPreflightPacketFindingCode::MissingRequiredInstructionRef,
                *required_ref,
                "required PubPunk instruction ref is missing",
            ));
        }
    }

    for instruction_ref in &packet.instruction_refs {
        if !is_safe_source_ref(instruction_ref) {
            findings.push(PubPunkPublishReceiptPreflightPacketFinding::for_ref(
                PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeInstructionRef,
                instruction_ref.clone(),
                "instruction refs must be explicit repo-relative refs",
            ));
        }
    }

    for source_ref in &packet.allowed_source_refs {
        if !is_safe_source_ref(source_ref) {
            findings.push(PubPunkPublishReceiptPreflightPacketFinding::for_ref(
                PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeAllowedSourceRef,
                source_ref.clone(),
                "allowed source refs must be explicit repo-relative refs",
            ));
        }
    }

    if !packet.payload_ref.trim().is_empty()
        && !packet.allowed_source_refs.contains(&packet.payload_ref)
    {
        findings.push(PubPunkPublishReceiptPreflightPacketFinding::for_ref(
            PubPunkPublishReceiptPreflightPacketFindingCode::PayloadRefNotAllowed,
            packet.payload_ref.clone(),
            "payload ref must be included in allowed source refs",
        ));
    }
    if !packet.channel_ref.trim().is_empty()
        && !packet.allowed_source_refs.contains(&packet.channel_ref)
    {
        findings.push(PubPunkPublishReceiptPreflightPacketFinding::for_ref(
            PubPunkPublishReceiptPreflightPacketFindingCode::ChannelRefNotAllowed,
            packet.channel_ref.clone(),
            "channel ref must be included in allowed source refs",
        ));
    }
    if !packet.connector_profile_resolution_ref.trim().is_empty()
        && !packet
            .allowed_source_refs
            .contains(&packet.connector_profile_resolution_ref)
    {
        findings.push(PubPunkPublishReceiptPreflightPacketFinding::for_ref(
            PubPunkPublishReceiptPreflightPacketFindingCode::ConnectorProfileResolutionRefNotAllowed,
            packet.connector_profile_resolution_ref.clone(),
            "connector profile resolution ref must be included in allowed source refs",
        ));
    }
    if !packet.connector_profile_ref.trim().is_empty()
        && !packet
            .allowed_source_refs
            .contains(&packet.connector_profile_ref)
    {
        findings.push(PubPunkPublishReceiptPreflightPacketFinding::for_ref(
            PubPunkPublishReceiptPreflightPacketFindingCode::ConnectorProfileRefNotAllowed,
            packet.connector_profile_ref.clone(),
            "connector profile ref must be included in allowed source refs",
        ));
    }
    if !packet.selected_connector_strategy_ref.trim().is_empty()
        && !packet
            .allowed_source_refs
            .contains(&packet.selected_connector_strategy_ref)
    {
        findings.push(PubPunkPublishReceiptPreflightPacketFinding::for_ref(
            PubPunkPublishReceiptPreflightPacketFindingCode::SelectedConnectorStrategyRefNotAllowed,
            packet.selected_connector_strategy_ref.clone(),
            "selected connector strategy ref must be included in allowed source refs",
        ));
    }

    if !packet
        .granted_capabilities
        .contains(&PubPunkCapabilityGrant::RequestExternalPublish)
    {
        findings.push(PubPunkPublishReceiptPreflightPacketFinding::new(
            PubPunkPublishReceiptPreflightPacketFindingCode::MissingRequestExternalPublishGrant,
            "request_external_publish must be explicitly granted for this packet",
        ));
    }
    for capability in &packet.granted_capabilities {
        if !capability.supported_by_side_effect_free_publish_receipt_preflight() {
            findings.push(PubPunkPublishReceiptPreflightPacketFinding::for_capability(
                *capability,
            ));
        }
    }

    if packet.expected_receipt_fields.is_empty() {
        findings.push(PubPunkPublishReceiptPreflightPacketFinding::new(
            PubPunkPublishReceiptPreflightPacketFindingCode::MissingExpectedReceiptFields,
            "expected receipt fields are required even though this packet does not write receipts",
        ));
    }
    for required_field in [
        "side_effects",
        "host_validation",
        "connector_profile_resolution",
        "connector_profile_ref",
        "selected_connector_strategy",
        "adapter_invocation_receipt",
        "operation_evidence",
        "publication_receipt",
    ] {
        if !packet
            .expected_receipt_fields
            .iter()
            .any(|field| field == required_field)
        {
            findings.push(PubPunkPublishReceiptPreflightPacketFinding::for_ref(
                PubPunkPublishReceiptPreflightPacketFindingCode::MissingRequiredExpectedReceiptField,
                required_field,
                "publish receipt preflight expectations must include host and publication receipt evidence coverage",
            ));
        }
    }

    if packet.privacy_policy.allows_private_or_raw_payloads() {
        findings.push(PubPunkPublishReceiptPreflightPacketFinding::new(
            PubPunkPublishReceiptPreflightPacketFindingCode::UnsafePrivacyPolicy,
            "privacy policy must disallow raw/private payloads for this packet",
        ));
    }

    if let Some(token_cost_ref) = &packet.token_cost_ref {
        if !is_safe_source_ref(token_cost_ref) {
            findings.push(PubPunkPublishReceiptPreflightPacketFinding::for_ref(
                PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeTokenCostRef,
                token_cost_ref.clone(),
                "token cost ref must be an explicit repo-relative ref when provided",
            ));
        }
    }

    let status = if findings.iter().any(|finding| finding.code.is_blocking()) {
        PubPunkAssessmentStatus::Blocked
    } else {
        PubPunkAssessmentStatus::Ready
    };

    PubPunkPublishReceiptPreflightPacketAssessment {
        schema_version: PUBPUNK_PUBLISH_RECEIPT_PREFLIGHT_PACKET_SCHEMA_VERSION,
        status,
        authority: PubPunkAssessmentAuthority::Advisory,
        requested_operation:
            PubPunkPublishReceiptPreflightOperation::PreparePublishReceiptPreflight,
        findings,
        boundary_flags: PUBPUNK_INVENTORY_ASSESSMENT_BOUNDARY_FLAGS,
        refs: PubPunkPublishReceiptPreflightPacketRefs {
            module_id: packet.module_id.clone(),
            module_version_ref: packet.module_version_ref.clone(),
            contract_ref: packet.contract_ref.clone(),
            run_ref: packet.run_ref.clone(),
            project_ref: packet.project_ref.clone(),
            workspace_policy: packet.workspace_policy,
            publishing_workspace_ref: packet.publishing_workspace_ref.clone(),
            publish_request_ref: packet.publish_request_ref.clone(),
            receipt_writer_preflight_ref: packet.receipt_writer_preflight_ref.clone(),
            policy_gate_preflight_ref: packet.policy_gate_preflight_ref.clone(),
            receipt_target_ref: packet.receipt_target_ref.clone(),
            receipt_storage_ref: packet.receipt_storage_ref.clone(),
            operation_evidence_ref: packet.operation_evidence_ref.clone(),
            idempotency_ref: packet.idempotency_ref.clone(),
            rollback_ref: packet.rollback_ref.clone(),
            error_ref: packet.error_ref.clone(),
            adapter_invocation_receipt_ref: packet.adapter_invocation_receipt_ref.clone(),
            payload_ref: packet.payload_ref.clone(),
            channel_ref: packet.channel_ref.clone(),
            connector_profile_resolution_ref: packet.connector_profile_resolution_ref.clone(),
            connector_profile_ref: packet.connector_profile_ref.clone(),
            selected_connector_strategy_ref: packet.selected_connector_strategy_ref.clone(),
            token_cost_ref: packet.token_cost_ref.clone(),
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkPublishReceiptWriteHandoffOperation {
    PreparePublishReceiptWriteHandoff,
}

impl PubPunkPublishReceiptWriteHandoffOperation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PreparePublishReceiptWriteHandoff => "prepare_publish_receipt_write_handoff",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishReceiptWriteHandoffPacket {
    pub module_id: String,
    pub module_version_ref: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub workspace_policy: PubPunkWorkspacePolicy,
    pub publishing_workspace_ref: String,
    pub publish_receipt_preflight_ref: String,
    pub receipt_writer_preflight_ref: String,
    pub receipt_writer_active_behavior_ref: String,
    pub receipt_writer_file_io_plan_ref: String,
    pub receipt_writer_target_storage_policy_ref: String,
    pub receipt_writer_host_path_observation_ref: String,
    pub receipt_writer_concrete_path_storage_policy_ref: String,
    pub operation_evidence_persistence_ref: String,
    pub receipt_target_ref: String,
    pub receipt_storage_ref: String,
    pub receipt_target_path_ref: String,
    pub receipt_bytes_ref: String,
    pub operation_evidence_ref: String,
    pub idempotency_ref: String,
    pub rollback_ref: String,
    pub error_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub channel_ref: String,
    pub connector_profile_resolution_ref: String,
    pub connector_profile_ref: String,
    pub selected_connector_strategy_ref: String,
    pub allowed_source_refs: Vec<String>,
    pub instruction_refs: Vec<String>,
    pub granted_capabilities: Vec<PubPunkCapabilityGrant>,
    pub privacy_policy: PubPunkPrivacyPolicy,
    pub expected_receipt_fields: Vec<String>,
    pub token_cost_ref: Option<String>,
}

impl PubPunkPublishReceiptWriteHandoffPacket {
    pub fn new(
        module_version_ref: impl Into<String>,
        contract_ref: impl Into<String>,
        run_ref: impl Into<String>,
        project_ref: impl Into<String>,
        publishing_workspace_ref: impl Into<String>,
    ) -> Self {
        Self {
            module_id: PUBPUNK_MODULE_ID.to_owned(),
            module_version_ref: module_version_ref.into(),
            contract_ref: contract_ref.into(),
            run_ref: run_ref.into(),
            project_ref: project_ref.into(),
            workspace_policy: PubPunkWorkspacePolicy::SplitExplicitRefs,
            publishing_workspace_ref: publishing_workspace_ref.into(),
            publish_receipt_preflight_ref: String::new(),
            receipt_writer_preflight_ref: String::new(),
            receipt_writer_active_behavior_ref: String::new(),
            receipt_writer_file_io_plan_ref: String::new(),
            receipt_writer_target_storage_policy_ref: String::new(),
            receipt_writer_host_path_observation_ref: String::new(),
            receipt_writer_concrete_path_storage_policy_ref: String::new(),
            operation_evidence_persistence_ref: String::new(),
            receipt_target_ref: String::new(),
            receipt_storage_ref: String::new(),
            receipt_target_path_ref: String::new(),
            receipt_bytes_ref: String::new(),
            operation_evidence_ref: String::new(),
            idempotency_ref: String::new(),
            rollback_ref: String::new(),
            error_ref: String::new(),
            adapter_invocation_receipt_ref: String::new(),
            payload_ref: String::new(),
            channel_ref: String::new(),
            connector_profile_resolution_ref: String::new(),
            connector_profile_ref: String::new(),
            selected_connector_strategy_ref: String::new(),
            allowed_source_refs: Vec::new(),
            instruction_refs: Vec::new(),
            granted_capabilities: Vec::new(),
            privacy_policy: PubPunkPrivacyPolicy::safe_metadata_only(),
            expected_receipt_fields: Vec::new(),
            token_cost_ref: None,
        }
    }

    pub fn with_workspace_policy(mut self, workspace_policy: PubPunkWorkspacePolicy) -> Self {
        self.workspace_policy = workspace_policy;
        self
    }

    pub fn with_publish_receipt_preflight_ref(
        mut self,
        publish_receipt_preflight_ref: impl Into<String>,
    ) -> Self {
        self.publish_receipt_preflight_ref = publish_receipt_preflight_ref.into();
        self
    }

    pub fn with_receipt_writer_preflight_ref(
        mut self,
        receipt_writer_preflight_ref: impl Into<String>,
    ) -> Self {
        self.receipt_writer_preflight_ref = receipt_writer_preflight_ref.into();
        self
    }

    pub fn with_receipt_writer_active_behavior_ref(
        mut self,
        receipt_writer_active_behavior_ref: impl Into<String>,
    ) -> Self {
        self.receipt_writer_active_behavior_ref = receipt_writer_active_behavior_ref.into();
        self
    }

    pub fn with_receipt_writer_file_io_plan_ref(
        mut self,
        receipt_writer_file_io_plan_ref: impl Into<String>,
    ) -> Self {
        self.receipt_writer_file_io_plan_ref = receipt_writer_file_io_plan_ref.into();
        self
    }

    pub fn with_receipt_writer_target_storage_policy_ref(
        mut self,
        receipt_writer_target_storage_policy_ref: impl Into<String>,
    ) -> Self {
        self.receipt_writer_target_storage_policy_ref =
            receipt_writer_target_storage_policy_ref.into();
        self
    }

    pub fn with_receipt_writer_host_path_observation_ref(
        mut self,
        receipt_writer_host_path_observation_ref: impl Into<String>,
    ) -> Self {
        self.receipt_writer_host_path_observation_ref =
            receipt_writer_host_path_observation_ref.into();
        self
    }

    pub fn with_receipt_writer_concrete_path_storage_policy_ref(
        mut self,
        receipt_writer_concrete_path_storage_policy_ref: impl Into<String>,
    ) -> Self {
        self.receipt_writer_concrete_path_storage_policy_ref =
            receipt_writer_concrete_path_storage_policy_ref.into();
        self
    }

    pub fn with_operation_evidence_persistence_ref(
        mut self,
        operation_evidence_persistence_ref: impl Into<String>,
    ) -> Self {
        self.operation_evidence_persistence_ref = operation_evidence_persistence_ref.into();
        self
    }

    pub fn with_receipt_target_ref(mut self, receipt_target_ref: impl Into<String>) -> Self {
        self.receipt_target_ref = receipt_target_ref.into();
        self
    }

    pub fn with_receipt_storage_ref(mut self, receipt_storage_ref: impl Into<String>) -> Self {
        self.receipt_storage_ref = receipt_storage_ref.into();
        self
    }

    pub fn with_receipt_target_path_ref(
        mut self,
        receipt_target_path_ref: impl Into<String>,
    ) -> Self {
        self.receipt_target_path_ref = receipt_target_path_ref.into();
        self
    }

    pub fn with_receipt_bytes_ref(mut self, receipt_bytes_ref: impl Into<String>) -> Self {
        self.receipt_bytes_ref = receipt_bytes_ref.into();
        self
    }

    pub fn with_operation_evidence_ref(
        mut self,
        operation_evidence_ref: impl Into<String>,
    ) -> Self {
        self.operation_evidence_ref = operation_evidence_ref.into();
        self
    }

    pub fn with_idempotency_ref(mut self, idempotency_ref: impl Into<String>) -> Self {
        self.idempotency_ref = idempotency_ref.into();
        self
    }

    pub fn with_rollback_ref(mut self, rollback_ref: impl Into<String>) -> Self {
        self.rollback_ref = rollback_ref.into();
        self
    }

    pub fn with_error_ref(mut self, error_ref: impl Into<String>) -> Self {
        self.error_ref = error_ref.into();
        self
    }

    pub fn with_adapter_invocation_receipt_ref(
        mut self,
        adapter_invocation_receipt_ref: impl Into<String>,
    ) -> Self {
        self.adapter_invocation_receipt_ref = adapter_invocation_receipt_ref.into();
        self
    }

    pub fn with_payload_ref(mut self, payload_ref: impl Into<String>) -> Self {
        self.payload_ref = payload_ref.into();
        self
    }

    pub fn with_channel_ref(mut self, channel_ref: impl Into<String>) -> Self {
        self.channel_ref = channel_ref.into();
        self
    }

    pub fn with_connector_profile_resolution_ref(
        mut self,
        connector_profile_resolution_ref: impl Into<String>,
    ) -> Self {
        self.connector_profile_resolution_ref = connector_profile_resolution_ref.into();
        self
    }

    pub fn with_connector_profile_ref(mut self, connector_profile_ref: impl Into<String>) -> Self {
        self.connector_profile_ref = connector_profile_ref.into();
        self
    }

    pub fn with_selected_connector_strategy_ref(
        mut self,
        selected_connector_strategy_ref: impl Into<String>,
    ) -> Self {
        self.selected_connector_strategy_ref = selected_connector_strategy_ref.into();
        self
    }

    pub fn with_allowed_source_refs(mut self, allowed_source_refs: Vec<impl Into<String>>) -> Self {
        self.allowed_source_refs = allowed_source_refs.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_instruction_refs(mut self, instruction_refs: Vec<impl Into<String>>) -> Self {
        self.instruction_refs = instruction_refs.into_iter().map(Into::into).collect();
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

    pub fn with_token_cost_ref(mut self, token_cost_ref: impl Into<String>) -> Self {
        self.token_cost_ref = Some(token_cost_ref.into());
        self
    }

    pub fn try_into_receipt_write_handoff_refs(
        &self,
    ) -> Result<
        PubPunkPublishReceiptWriteHandoffRefs,
        PubPunkPublishReceiptWriteHandoffPacketAssessment,
    > {
        let assessment = assess_pubpunk_publish_receipt_write_handoff_packet(self);
        if assessment.has_blockers() {
            return Err(assessment);
        }

        Ok(PubPunkPublishReceiptWriteHandoffRefs {
            publish_receipt_preflight_ref: self.publish_receipt_preflight_ref.clone(),
            receipt_writer_preflight_ref: self.receipt_writer_preflight_ref.clone(),
            receipt_writer_active_behavior_ref: self.receipt_writer_active_behavior_ref.clone(),
            receipt_writer_file_io_plan_ref: self.receipt_writer_file_io_plan_ref.clone(),
            receipt_writer_target_storage_policy_ref: self
                .receipt_writer_target_storage_policy_ref
                .clone(),
            receipt_writer_host_path_observation_ref: self
                .receipt_writer_host_path_observation_ref
                .clone(),
            receipt_writer_concrete_path_storage_policy_ref: self
                .receipt_writer_concrete_path_storage_policy_ref
                .clone(),
            operation_evidence_persistence_ref: self.operation_evidence_persistence_ref.clone(),
            receipt_target_ref: self.receipt_target_ref.clone(),
            storage_ref: self.receipt_storage_ref.clone(),
            receipt_target_path_ref: self.receipt_target_path_ref.clone(),
            receipt_bytes_ref: self.receipt_bytes_ref.clone(),
            operation_evidence_ref: self.operation_evidence_ref.clone(),
            idempotency_ref: self.idempotency_ref.clone(),
            rollback_ref: self.rollback_ref.clone(),
            error_ref: self.error_ref.clone(),
            adapter_invocation_receipt_ref: self.adapter_invocation_receipt_ref.clone(),
            payload_ref: self.payload_ref.clone(),
            channel_ref: self.channel_ref.clone(),
            connector_profile_resolution_ref: self.connector_profile_resolution_ref.clone(),
            connector_profile_ref: self.connector_profile_ref.clone(),
            selected_connector_strategy_ref: self.selected_connector_strategy_ref.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishReceiptWriteHandoffRefs {
    pub publish_receipt_preflight_ref: String,
    pub receipt_writer_preflight_ref: String,
    pub receipt_writer_active_behavior_ref: String,
    pub receipt_writer_file_io_plan_ref: String,
    pub receipt_writer_target_storage_policy_ref: String,
    pub receipt_writer_host_path_observation_ref: String,
    pub receipt_writer_concrete_path_storage_policy_ref: String,
    pub operation_evidence_persistence_ref: String,
    pub receipt_target_ref: String,
    pub storage_ref: String,
    pub receipt_target_path_ref: String,
    pub receipt_bytes_ref: String,
    pub operation_evidence_ref: String,
    pub idempotency_ref: String,
    pub rollback_ref: String,
    pub error_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub channel_ref: String,
    pub connector_profile_resolution_ref: String,
    pub connector_profile_ref: String,
    pub selected_connector_strategy_ref: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkPublishReceiptWriteHandoffPacketFindingCode {
    MissingModuleId,
    NonCanonicalModuleId,
    MissingModuleVersionRef,
    MissingContractRef,
    MissingRunRef,
    MissingProjectRef,
    UnsupportedWorkspacePolicy,
    MissingPublishingWorkspaceRef,
    UnsafePublishingWorkspaceRef,
    MissingPublishReceiptPreflightRef,
    UnsafePublishReceiptPreflightRef,
    MissingReceiptWriterPreflightRef,
    UnsafeReceiptWriterPreflightRef,
    MissingReceiptWriterActiveBehaviorRef,
    UnsafeReceiptWriterActiveBehaviorRef,
    MissingReceiptWriterFileIoPlanRef,
    UnsafeReceiptWriterFileIoPlanRef,
    MissingReceiptWriterTargetStoragePolicyRef,
    UnsafeReceiptWriterTargetStoragePolicyRef,
    MissingReceiptWriterHostPathObservationRef,
    UnsafeReceiptWriterHostPathObservationRef,
    MissingReceiptWriterConcretePathStoragePolicyRef,
    UnsafeReceiptWriterConcretePathStoragePolicyRef,
    MissingOperationEvidencePersistenceRef,
    UnsafeOperationEvidencePersistenceRef,
    MissingReceiptTargetRef,
    UnsafeReceiptTargetRef,
    MissingReceiptStorageRef,
    UnsafeReceiptStorageRef,
    MissingReceiptTargetPathRef,
    UnsafeReceiptTargetPathRef,
    ReceiptTargetPathOutsidePunkRuns,
    MissingReceiptBytesRef,
    UnsafeReceiptBytesRef,
    ReceiptBytesRefNotAllowed,
    MissingOperationEvidenceRef,
    UnsafeOperationEvidenceRef,
    MissingIdempotencyRef,
    UnsafeIdempotencyRef,
    MissingRollbackRef,
    UnsafeRollbackRef,
    MissingErrorRef,
    UnsafeErrorRef,
    MissingAdapterInvocationReceiptRef,
    UnsafeAdapterInvocationReceiptRef,
    AdapterInvocationReceiptRefNotAllowed,
    MissingPayloadRef,
    UnsafePayloadRef,
    PayloadRefNotAllowed,
    MissingChannelRef,
    UnsafeChannelRef,
    ChannelRefNotAllowed,
    MissingConnectorProfileResolutionRef,
    UnsafeConnectorProfileResolutionRef,
    ConnectorProfileResolutionRefNotAllowed,
    MissingConnectorProfileRef,
    UnsafeConnectorProfileRef,
    ConnectorProfileRefNotAllowed,
    MissingSelectedConnectorStrategyRef,
    UnsafeSelectedConnectorStrategyRef,
    SelectedConnectorStrategyRefNotAllowed,
    MissingInstructionRefs,
    MissingRequiredInstructionRef,
    UnsafeInstructionRef,
    UnsafeAllowedSourceRef,
    MissingRequestPublicationReceiptWriteGrant,
    UnsupportedCapabilityGrant,
    MissingExpectedReceiptFields,
    MissingRequiredExpectedReceiptField,
    UnsafePrivacyPolicy,
    UnsafeTokenCostRef,
}

impl PubPunkPublishReceiptWriteHandoffPacketFindingCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingModuleId => "missing_module_id",
            Self::NonCanonicalModuleId => "non_canonical_module_id",
            Self::MissingModuleVersionRef => "missing_module_version_ref",
            Self::MissingContractRef => "missing_contract_ref",
            Self::MissingRunRef => "missing_run_ref",
            Self::MissingProjectRef => "missing_project_ref",
            Self::UnsupportedWorkspacePolicy => "unsupported_workspace_policy",
            Self::MissingPublishingWorkspaceRef => "missing_publishing_workspace_ref",
            Self::UnsafePublishingWorkspaceRef => "unsafe_publishing_workspace_ref",
            Self::MissingPublishReceiptPreflightRef => "missing_publish_receipt_preflight_ref",
            Self::UnsafePublishReceiptPreflightRef => "unsafe_publish_receipt_preflight_ref",
            Self::MissingReceiptWriterPreflightRef => "missing_receipt_writer_preflight_ref",
            Self::UnsafeReceiptWriterPreflightRef => "unsafe_receipt_writer_preflight_ref",
            Self::MissingReceiptWriterActiveBehaviorRef => {
                "missing_receipt_writer_active_behavior_ref"
            }
            Self::UnsafeReceiptWriterActiveBehaviorRef => {
                "unsafe_receipt_writer_active_behavior_ref"
            }
            Self::MissingReceiptWriterFileIoPlanRef => "missing_receipt_writer_file_io_plan_ref",
            Self::UnsafeReceiptWriterFileIoPlanRef => "unsafe_receipt_writer_file_io_plan_ref",
            Self::MissingReceiptWriterTargetStoragePolicyRef => {
                "missing_receipt_writer_target_storage_policy_ref"
            }
            Self::UnsafeReceiptWriterTargetStoragePolicyRef => {
                "unsafe_receipt_writer_target_storage_policy_ref"
            }
            Self::MissingReceiptWriterHostPathObservationRef => {
                "missing_receipt_writer_host_path_observation_ref"
            }
            Self::UnsafeReceiptWriterHostPathObservationRef => {
                "unsafe_receipt_writer_host_path_observation_ref"
            }
            Self::MissingReceiptWriterConcretePathStoragePolicyRef => {
                "missing_receipt_writer_concrete_path_storage_policy_ref"
            }
            Self::UnsafeReceiptWriterConcretePathStoragePolicyRef => {
                "unsafe_receipt_writer_concrete_path_storage_policy_ref"
            }
            Self::MissingOperationEvidencePersistenceRef => {
                "missing_operation_evidence_persistence_ref"
            }
            Self::UnsafeOperationEvidencePersistenceRef => {
                "unsafe_operation_evidence_persistence_ref"
            }
            Self::MissingReceiptTargetRef => "missing_receipt_target_ref",
            Self::UnsafeReceiptTargetRef => "unsafe_receipt_target_ref",
            Self::MissingReceiptStorageRef => "missing_receipt_storage_ref",
            Self::UnsafeReceiptStorageRef => "unsafe_receipt_storage_ref",
            Self::MissingReceiptTargetPathRef => "missing_receipt_target_path_ref",
            Self::UnsafeReceiptTargetPathRef => "unsafe_receipt_target_path_ref",
            Self::ReceiptTargetPathOutsidePunkRuns => "receipt_target_path_outside_punk_runs",
            Self::MissingReceiptBytesRef => "missing_receipt_bytes_ref",
            Self::UnsafeReceiptBytesRef => "unsafe_receipt_bytes_ref",
            Self::ReceiptBytesRefNotAllowed => "receipt_bytes_ref_not_allowed",
            Self::MissingOperationEvidenceRef => "missing_operation_evidence_ref",
            Self::UnsafeOperationEvidenceRef => "unsafe_operation_evidence_ref",
            Self::MissingIdempotencyRef => "missing_idempotency_ref",
            Self::UnsafeIdempotencyRef => "unsafe_idempotency_ref",
            Self::MissingRollbackRef => "missing_rollback_ref",
            Self::UnsafeRollbackRef => "unsafe_rollback_ref",
            Self::MissingErrorRef => "missing_error_ref",
            Self::UnsafeErrorRef => "unsafe_error_ref",
            Self::MissingAdapterInvocationReceiptRef => "missing_adapter_invocation_receipt_ref",
            Self::UnsafeAdapterInvocationReceiptRef => "unsafe_adapter_invocation_receipt_ref",
            Self::AdapterInvocationReceiptRefNotAllowed => {
                "adapter_invocation_receipt_ref_not_allowed"
            }
            Self::MissingPayloadRef => "missing_payload_ref",
            Self::UnsafePayloadRef => "unsafe_payload_ref",
            Self::PayloadRefNotAllowed => "payload_ref_not_allowed",
            Self::MissingChannelRef => "missing_channel_ref",
            Self::UnsafeChannelRef => "unsafe_channel_ref",
            Self::ChannelRefNotAllowed => "channel_ref_not_allowed",
            Self::MissingConnectorProfileResolutionRef => {
                "missing_connector_profile_resolution_ref"
            }
            Self::UnsafeConnectorProfileResolutionRef => "unsafe_connector_profile_resolution_ref",
            Self::ConnectorProfileResolutionRefNotAllowed => {
                "connector_profile_resolution_ref_not_allowed"
            }
            Self::MissingConnectorProfileRef => "missing_connector_profile_ref",
            Self::UnsafeConnectorProfileRef => "unsafe_connector_profile_ref",
            Self::ConnectorProfileRefNotAllowed => "connector_profile_ref_not_allowed",
            Self::MissingSelectedConnectorStrategyRef => "missing_selected_connector_strategy_ref",
            Self::UnsafeSelectedConnectorStrategyRef => "unsafe_selected_connector_strategy_ref",
            Self::SelectedConnectorStrategyRefNotAllowed => {
                "selected_connector_strategy_ref_not_allowed"
            }
            Self::MissingInstructionRefs => "missing_instruction_refs",
            Self::MissingRequiredInstructionRef => "missing_required_instruction_ref",
            Self::UnsafeInstructionRef => "unsafe_instruction_ref",
            Self::UnsafeAllowedSourceRef => "unsafe_allowed_source_ref",
            Self::MissingRequestPublicationReceiptWriteGrant => {
                "missing_request_publication_receipt_write_grant"
            }
            Self::UnsupportedCapabilityGrant => "unsupported_capability_grant",
            Self::MissingExpectedReceiptFields => "missing_expected_receipt_fields",
            Self::MissingRequiredExpectedReceiptField => "missing_required_expected_receipt_field",
            Self::UnsafePrivacyPolicy => "unsafe_privacy_policy",
            Self::UnsafeTokenCostRef => "unsafe_token_cost_ref",
        }
    }

    pub fn is_blocking(self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishReceiptWriteHandoffPacketFinding {
    pub code: PubPunkPublishReceiptWriteHandoffPacketFindingCode,
    pub ref_value: Option<String>,
    pub capability: Option<PubPunkCapabilityGrant>,
    pub message: &'static str,
}

impl PubPunkPublishReceiptWriteHandoffPacketFinding {
    fn new(
        code: PubPunkPublishReceiptWriteHandoffPacketFindingCode,
        message: &'static str,
    ) -> Self {
        Self {
            code,
            ref_value: None,
            capability: None,
            message,
        }
    }

    fn for_ref(
        code: PubPunkPublishReceiptWriteHandoffPacketFindingCode,
        ref_value: impl Into<String>,
        message: &'static str,
    ) -> Self {
        Self {
            code,
            ref_value: Some(ref_value.into()),
            capability: None,
            message,
        }
    }

    fn for_capability(capability: PubPunkCapabilityGrant) -> Self {
        Self {
            code: PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsupportedCapabilityGrant,
            ref_value: None,
            capability: Some(capability),
            message: "capability is not available in the side-effect-free publish receipt write handoff packet",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishReceiptWriteHandoffPacketRefs {
    pub module_id: String,
    pub module_version_ref: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub workspace_policy: PubPunkWorkspacePolicy,
    pub publishing_workspace_ref: String,
    pub publish_receipt_preflight_ref: String,
    pub receipt_writer_preflight_ref: String,
    pub receipt_writer_active_behavior_ref: String,
    pub receipt_writer_file_io_plan_ref: String,
    pub receipt_writer_target_storage_policy_ref: String,
    pub receipt_writer_host_path_observation_ref: String,
    pub receipt_writer_concrete_path_storage_policy_ref: String,
    pub operation_evidence_persistence_ref: String,
    pub receipt_target_ref: String,
    pub receipt_storage_ref: String,
    pub receipt_target_path_ref: String,
    pub receipt_bytes_ref: String,
    pub operation_evidence_ref: String,
    pub idempotency_ref: String,
    pub rollback_ref: String,
    pub error_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub channel_ref: String,
    pub connector_profile_resolution_ref: String,
    pub connector_profile_ref: String,
    pub selected_connector_strategy_ref: String,
    pub token_cost_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishReceiptWriteHandoffPacketAssessment {
    pub schema_version: &'static str,
    pub status: PubPunkAssessmentStatus,
    pub authority: PubPunkAssessmentAuthority,
    pub requested_operation: PubPunkPublishReceiptWriteHandoffOperation,
    pub findings: Vec<PubPunkPublishReceiptWriteHandoffPacketFinding>,
    pub boundary_flags: PubPunkInventoryBoundaryFlags,
    pub refs: PubPunkPublishReceiptWriteHandoffPacketRefs,
}

impl PubPunkPublishReceiptWriteHandoffPacketAssessment {
    pub fn blocking_findings(
        &self,
    ) -> impl Iterator<Item = &PubPunkPublishReceiptWriteHandoffPacketFinding> {
        self.findings
            .iter()
            .filter(|finding| finding.code.is_blocking())
    }

    pub fn has_blockers(&self) -> bool {
        self.blocking_findings().next().is_some()
    }
}

pub fn assess_pubpunk_publish_receipt_write_handoff_packet(
    packet: &PubPunkPublishReceiptWriteHandoffPacket,
) -> PubPunkPublishReceiptWriteHandoffPacketAssessment {
    let mut findings = Vec::new();

    push_publish_receipt_write_handoff_required_ref_finding(
        &mut findings,
        packet.module_id.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingModuleId,
        "module id is required",
    );
    if !packet.module_id.trim().is_empty() && packet.module_id != PUBPUNK_MODULE_ID {
        findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::for_ref(
            PubPunkPublishReceiptWriteHandoffPacketFindingCode::NonCanonicalModuleId,
            packet.module_id.clone(),
            "PubPunk publish receipt write handoff packets must use the canonical pubpunk module id",
        ));
    }
    push_publish_receipt_write_handoff_required_ref_finding(
        &mut findings,
        packet.module_version_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingModuleVersionRef,
        "module version ref is required",
    );
    push_publish_receipt_write_handoff_required_ref_finding(
        &mut findings,
        packet.contract_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingContractRef,
        "contract ref is required",
    );
    push_publish_receipt_write_handoff_required_ref_finding(
        &mut findings,
        packet.run_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingRunRef,
        "run ref is required",
    );
    push_publish_receipt_write_handoff_required_ref_finding(
        &mut findings,
        packet.project_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingProjectRef,
        "project ref is required",
    );

    if !packet.workspace_policy.selected_for_first_slice() {
        findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::for_ref(
            PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsupportedWorkspacePolicy,
            packet.workspace_policy.as_str(),
            "the first PubPunk publish receipt write handoff supports split explicit refs only",
        ));
    }

    push_publish_receipt_write_handoff_required_ref_finding(
        &mut findings,
        packet.publishing_workspace_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingPublishingWorkspaceRef,
        "publishing workspace ref is required",
    );
    if !packet.publishing_workspace_ref.trim().is_empty()
        && !is_safe_workspace_ref(&packet.publishing_workspace_ref)
    {
        findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::for_ref(
            PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafePublishingWorkspaceRef,
            packet.publishing_workspace_ref.clone(),
            "publishing workspace ref must be an explicit safe logical or repo-relative ref",
        ));
    }

    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.publish_receipt_preflight_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingPublishReceiptPreflightRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafePublishReceiptPreflightRef,
        "publish receipt preflight ref is required",
        "publish receipt preflight ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.receipt_writer_preflight_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingReceiptWriterPreflightRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeReceiptWriterPreflightRef,
        "receipt writer preflight ref is required",
        "receipt writer preflight ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.receipt_writer_active_behavior_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingReceiptWriterActiveBehaviorRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeReceiptWriterActiveBehaviorRef,
        "receipt writer active behavior ref is required",
        "receipt writer active behavior ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.receipt_writer_file_io_plan_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingReceiptWriterFileIoPlanRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeReceiptWriterFileIoPlanRef,
        "receipt writer file IO plan ref is required",
        "receipt writer file IO plan ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.receipt_writer_target_storage_policy_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingReceiptWriterTargetStoragePolicyRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeReceiptWriterTargetStoragePolicyRef,
        "receipt writer target/storage policy ref is required",
        "receipt writer target/storage policy ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.receipt_writer_host_path_observation_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingReceiptWriterHostPathObservationRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeReceiptWriterHostPathObservationRef,
        "receipt writer host path observation ref is required",
        "receipt writer host path observation ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet
            .receipt_writer_concrete_path_storage_policy_ref
            .as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingReceiptWriterConcretePathStoragePolicyRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeReceiptWriterConcretePathStoragePolicyRef,
        "receipt writer concrete path/storage policy ref is required",
        "receipt writer concrete path/storage policy ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.operation_evidence_persistence_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingOperationEvidencePersistenceRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeOperationEvidencePersistenceRef,
        "operation evidence persistence ref is required",
        "operation evidence persistence ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.receipt_target_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingReceiptTargetRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeReceiptTargetRef,
        "receipt target ref is required",
        "receipt target ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.receipt_storage_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingReceiptStorageRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeReceiptStorageRef,
        "receipt storage ref is required",
        "receipt storage ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.receipt_target_path_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingReceiptTargetPathRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeReceiptTargetPathRef,
        "receipt target path ref is required",
        "receipt target path ref must be an explicit repo-relative ref",
    );
    if !packet.receipt_target_path_ref.trim().is_empty()
        && is_safe_source_ref(&packet.receipt_target_path_ref)
        && !is_punk_runs_target_ref(&packet.receipt_target_path_ref)
    {
        findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::for_ref(
            PubPunkPublishReceiptWriteHandoffPacketFindingCode::ReceiptTargetPathOutsidePunkRuns,
            packet.receipt_target_path_ref.clone(),
            "receipt target path ref must stay under .punk/runs for the first active handoff",
        ));
    }
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.receipt_bytes_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingReceiptBytesRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeReceiptBytesRef,
        "receipt bytes ref is required",
        "receipt bytes ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.operation_evidence_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingOperationEvidenceRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeOperationEvidenceRef,
        "operation evidence ref is required",
        "operation evidence ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.idempotency_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingIdempotencyRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeIdempotencyRef,
        "idempotency ref is required",
        "idempotency ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.rollback_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingRollbackRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeRollbackRef,
        "rollback ref is required",
        "rollback ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.error_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingErrorRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeErrorRef,
        "error ref is required",
        "error ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.adapter_invocation_receipt_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingAdapterInvocationReceiptRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeAdapterInvocationReceiptRef,
        "adapter invocation receipt ref is required",
        "adapter invocation receipt ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.payload_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingPayloadRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafePayloadRef,
        "payload ref is required",
        "payload ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.channel_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingChannelRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeChannelRef,
        "channel ref is required",
        "channel ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.connector_profile_resolution_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingConnectorProfileResolutionRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeConnectorProfileResolutionRef,
        "connector profile resolution ref is required",
        "connector profile resolution ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.connector_profile_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingConnectorProfileRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeConnectorProfileRef,
        "connector profile ref is required",
        "connector profile ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_write_handoff_ref(
        &mut findings,
        packet.selected_connector_strategy_ref.as_str(),
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingSelectedConnectorStrategyRef,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeSelectedConnectorStrategyRef,
        "selected connector strategy ref is required",
        "selected connector strategy ref must be an explicit repo-relative ref",
    );

    if packet.instruction_refs.is_empty() {
        findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::new(
            PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingInstructionRefs,
            "instruction refs are required for PubPunk publish receipt write handoff packets",
        ));
    }

    for required_ref in PUBPUNK_REQUIRED_INSTRUCTION_REFS {
        if !packet
            .instruction_refs
            .iter()
            .any(|instruction_ref| instruction_ref == required_ref)
        {
            findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::for_ref(
                PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingRequiredInstructionRef,
                *required_ref,
                "required PubPunk instruction ref is missing",
            ));
        }
    }

    for instruction_ref in &packet.instruction_refs {
        if !is_safe_source_ref(instruction_ref) {
            findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::for_ref(
                PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeInstructionRef,
                instruction_ref.clone(),
                "instruction refs must be explicit repo-relative refs",
            ));
        }
    }

    for source_ref in &packet.allowed_source_refs {
        if !is_safe_source_ref(source_ref) {
            findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::for_ref(
                PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeAllowedSourceRef,
                source_ref.clone(),
                "allowed source refs must be explicit repo-relative refs",
            ));
        }
    }

    if !packet.payload_ref.trim().is_empty()
        && !packet.allowed_source_refs.contains(&packet.payload_ref)
    {
        findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::for_ref(
            PubPunkPublishReceiptWriteHandoffPacketFindingCode::PayloadRefNotAllowed,
            packet.payload_ref.clone(),
            "payload ref must be included in allowed source refs",
        ));
    }
    if !packet.channel_ref.trim().is_empty()
        && !packet.allowed_source_refs.contains(&packet.channel_ref)
    {
        findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::for_ref(
            PubPunkPublishReceiptWriteHandoffPacketFindingCode::ChannelRefNotAllowed,
            packet.channel_ref.clone(),
            "channel ref must be included in allowed source refs",
        ));
    }
    if !packet.connector_profile_resolution_ref.trim().is_empty()
        && !packet
            .allowed_source_refs
            .contains(&packet.connector_profile_resolution_ref)
    {
        findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::for_ref(
            PubPunkPublishReceiptWriteHandoffPacketFindingCode::ConnectorProfileResolutionRefNotAllowed,
            packet.connector_profile_resolution_ref.clone(),
            "connector profile resolution ref must be included in allowed source refs",
        ));
    }
    if !packet.connector_profile_ref.trim().is_empty()
        && !packet
            .allowed_source_refs
            .contains(&packet.connector_profile_ref)
    {
        findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::for_ref(
            PubPunkPublishReceiptWriteHandoffPacketFindingCode::ConnectorProfileRefNotAllowed,
            packet.connector_profile_ref.clone(),
            "connector profile ref must be included in allowed source refs",
        ));
    }
    if !packet.selected_connector_strategy_ref.trim().is_empty()
        && !packet
            .allowed_source_refs
            .contains(&packet.selected_connector_strategy_ref)
    {
        findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::for_ref(
            PubPunkPublishReceiptWriteHandoffPacketFindingCode::SelectedConnectorStrategyRefNotAllowed,
            packet.selected_connector_strategy_ref.clone(),
            "selected connector strategy ref must be included in allowed source refs",
        ));
    }
    if !packet.adapter_invocation_receipt_ref.trim().is_empty()
        && !packet
            .allowed_source_refs
            .contains(&packet.adapter_invocation_receipt_ref)
    {
        findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::for_ref(
            PubPunkPublishReceiptWriteHandoffPacketFindingCode::AdapterInvocationReceiptRefNotAllowed,
            packet.adapter_invocation_receipt_ref.clone(),
            "adapter invocation receipt ref must be included in allowed source refs",
        ));
    }
    if !packet.receipt_bytes_ref.trim().is_empty()
        && !packet
            .allowed_source_refs
            .contains(&packet.receipt_bytes_ref)
    {
        findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::for_ref(
            PubPunkPublishReceiptWriteHandoffPacketFindingCode::ReceiptBytesRefNotAllowed,
            packet.receipt_bytes_ref.clone(),
            "receipt bytes ref must be included in allowed source refs",
        ));
    }

    if !packet
        .granted_capabilities
        .contains(&PubPunkCapabilityGrant::RequestPublicationReceiptWrite)
    {
        findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::new(
            PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingRequestPublicationReceiptWriteGrant,
            "request_publication_receipt_write must be explicitly granted for this packet",
        ));
    }
    for capability in &packet.granted_capabilities {
        if !capability.supported_by_side_effect_free_publish_receipt_write_handoff() {
            findings
                .push(PubPunkPublishReceiptWriteHandoffPacketFinding::for_capability(*capability));
        }
    }

    if packet.expected_receipt_fields.is_empty() {
        findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::new(
            PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingExpectedReceiptFields,
            "expected receipt fields are required even though this packet does not write receipts",
        ));
    }
    for required_field in [
        "side_effects",
        "host_validation",
        "connector_profile_resolution",
        "connector_profile_ref",
        "selected_connector_strategy",
        "adapter_invocation_receipt",
        "operation_evidence",
        "publication_receipt",
        "receipt_bytes",
        "receipt_target_path",
        "receipt_write_result",
    ] {
        if !packet
            .expected_receipt_fields
            .iter()
            .any(|field| field == required_field)
        {
            findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::for_ref(
                PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingRequiredExpectedReceiptField,
                required_field,
                "publish receipt write handoff expectations must include host, publication, bytes, and write-result coverage",
            ));
        }
    }

    if packet.privacy_policy.allows_private_or_raw_payloads() {
        findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::new(
            PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafePrivacyPolicy,
            "privacy policy must disallow raw/private payloads for this packet",
        ));
    }

    if let Some(token_cost_ref) = &packet.token_cost_ref {
        if !is_safe_source_ref(token_cost_ref) {
            findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::for_ref(
                PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeTokenCostRef,
                token_cost_ref.clone(),
                "token cost ref must be an explicit repo-relative ref when provided",
            ));
        }
    }

    let status = if findings.iter().any(|finding| finding.code.is_blocking()) {
        PubPunkAssessmentStatus::Blocked
    } else {
        PubPunkAssessmentStatus::Ready
    };

    PubPunkPublishReceiptWriteHandoffPacketAssessment {
        schema_version: PUBPUNK_PUBLISH_RECEIPT_WRITE_HANDOFF_PACKET_SCHEMA_VERSION,
        status,
        authority: PubPunkAssessmentAuthority::Advisory,
        requested_operation:
            PubPunkPublishReceiptWriteHandoffOperation::PreparePublishReceiptWriteHandoff,
        findings,
        boundary_flags: PUBPUNK_INVENTORY_ASSESSMENT_BOUNDARY_FLAGS,
        refs: PubPunkPublishReceiptWriteHandoffPacketRefs {
            module_id: packet.module_id.clone(),
            module_version_ref: packet.module_version_ref.clone(),
            contract_ref: packet.contract_ref.clone(),
            run_ref: packet.run_ref.clone(),
            project_ref: packet.project_ref.clone(),
            workspace_policy: packet.workspace_policy,
            publishing_workspace_ref: packet.publishing_workspace_ref.clone(),
            publish_receipt_preflight_ref: packet.publish_receipt_preflight_ref.clone(),
            receipt_writer_preflight_ref: packet.receipt_writer_preflight_ref.clone(),
            receipt_writer_active_behavior_ref: packet.receipt_writer_active_behavior_ref.clone(),
            receipt_writer_file_io_plan_ref: packet.receipt_writer_file_io_plan_ref.clone(),
            receipt_writer_target_storage_policy_ref: packet
                .receipt_writer_target_storage_policy_ref
                .clone(),
            receipt_writer_host_path_observation_ref: packet
                .receipt_writer_host_path_observation_ref
                .clone(),
            receipt_writer_concrete_path_storage_policy_ref: packet
                .receipt_writer_concrete_path_storage_policy_ref
                .clone(),
            operation_evidence_persistence_ref: packet.operation_evidence_persistence_ref.clone(),
            receipt_target_ref: packet.receipt_target_ref.clone(),
            receipt_storage_ref: packet.receipt_storage_ref.clone(),
            receipt_target_path_ref: packet.receipt_target_path_ref.clone(),
            receipt_bytes_ref: packet.receipt_bytes_ref.clone(),
            operation_evidence_ref: packet.operation_evidence_ref.clone(),
            idempotency_ref: packet.idempotency_ref.clone(),
            rollback_ref: packet.rollback_ref.clone(),
            error_ref: packet.error_ref.clone(),
            adapter_invocation_receipt_ref: packet.adapter_invocation_receipt_ref.clone(),
            payload_ref: packet.payload_ref.clone(),
            channel_ref: packet.channel_ref.clone(),
            connector_profile_resolution_ref: packet.connector_profile_resolution_ref.clone(),
            connector_profile_ref: packet.connector_profile_ref.clone(),
            selected_connector_strategy_ref: packet.selected_connector_strategy_ref.clone(),
            token_cost_ref: packet.token_cost_ref.clone(),
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkPublishOperationEvidenceHandoffOperation {
    PreparePublishOperationEvidenceHandoff,
}

impl PubPunkPublishOperationEvidenceHandoffOperation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PreparePublishOperationEvidenceHandoff => {
                "prepare_publish_operation_evidence_handoff"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishOperationEvidenceHandoffPacket {
    pub module_id: String,
    pub module_version_ref: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub workspace_policy: PubPunkWorkspacePolicy,
    pub publishing_workspace_ref: String,
    pub publish_receipt_write_handoff_ref: String,
    pub receipt_writer_result_ref: String,
    pub receipt_storage_ref: String,
    pub receipt_target_ref: String,
    pub receipt_target_path_ref: String,
    pub receipt_bytes_ref: String,
    pub operation_evidence_ref: String,
    pub operation_evidence_target_path_ref: String,
    pub operation_evidence_bytes_ref: String,
    pub operation_evidence_write_result_ref: String,
    pub idempotency_ref: String,
    pub rollback_ref: String,
    pub error_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub channel_ref: String,
    pub connector_profile_resolution_ref: String,
    pub connector_profile_ref: String,
    pub selected_connector_strategy_ref: String,
    pub allowed_source_refs: Vec<String>,
    pub instruction_refs: Vec<String>,
    pub granted_capabilities: Vec<PubPunkCapabilityGrant>,
    pub privacy_policy: PubPunkPrivacyPolicy,
    pub expected_receipt_fields: Vec<String>,
    pub token_cost_ref: Option<String>,
}

impl PubPunkPublishOperationEvidenceHandoffPacket {
    pub fn new(
        module_version_ref: impl Into<String>,
        contract_ref: impl Into<String>,
        run_ref: impl Into<String>,
        project_ref: impl Into<String>,
        publishing_workspace_ref: impl Into<String>,
    ) -> Self {
        Self {
            module_id: PUBPUNK_MODULE_ID.to_owned(),
            module_version_ref: module_version_ref.into(),
            contract_ref: contract_ref.into(),
            run_ref: run_ref.into(),
            project_ref: project_ref.into(),
            workspace_policy: PubPunkWorkspacePolicy::SplitExplicitRefs,
            publishing_workspace_ref: publishing_workspace_ref.into(),
            publish_receipt_write_handoff_ref: String::new(),
            receipt_writer_result_ref: String::new(),
            receipt_storage_ref: String::new(),
            receipt_target_ref: String::new(),
            receipt_target_path_ref: String::new(),
            receipt_bytes_ref: String::new(),
            operation_evidence_ref: String::new(),
            operation_evidence_target_path_ref: String::new(),
            operation_evidence_bytes_ref: String::new(),
            operation_evidence_write_result_ref: String::new(),
            idempotency_ref: String::new(),
            rollback_ref: String::new(),
            error_ref: String::new(),
            adapter_invocation_receipt_ref: String::new(),
            payload_ref: String::new(),
            channel_ref: String::new(),
            connector_profile_resolution_ref: String::new(),
            connector_profile_ref: String::new(),
            selected_connector_strategy_ref: String::new(),
            allowed_source_refs: Vec::new(),
            instruction_refs: Vec::new(),
            granted_capabilities: Vec::new(),
            privacy_policy: PubPunkPrivacyPolicy::safe_metadata_only(),
            expected_receipt_fields: Vec::new(),
            token_cost_ref: None,
        }
    }

    pub fn with_workspace_policy(mut self, workspace_policy: PubPunkWorkspacePolicy) -> Self {
        self.workspace_policy = workspace_policy;
        self
    }

    pub fn with_publish_receipt_write_handoff_ref(
        mut self,
        publish_receipt_write_handoff_ref: impl Into<String>,
    ) -> Self {
        self.publish_receipt_write_handoff_ref = publish_receipt_write_handoff_ref.into();
        self
    }

    pub fn with_receipt_writer_result_ref(
        mut self,
        receipt_writer_result_ref: impl Into<String>,
    ) -> Self {
        self.receipt_writer_result_ref = receipt_writer_result_ref.into();
        self
    }

    pub fn with_receipt_storage_ref(mut self, receipt_storage_ref: impl Into<String>) -> Self {
        self.receipt_storage_ref = receipt_storage_ref.into();
        self
    }

    pub fn with_receipt_target_ref(mut self, receipt_target_ref: impl Into<String>) -> Self {
        self.receipt_target_ref = receipt_target_ref.into();
        self
    }

    pub fn with_receipt_target_path_ref(
        mut self,
        receipt_target_path_ref: impl Into<String>,
    ) -> Self {
        self.receipt_target_path_ref = receipt_target_path_ref.into();
        self
    }

    pub fn with_receipt_bytes_ref(mut self, receipt_bytes_ref: impl Into<String>) -> Self {
        self.receipt_bytes_ref = receipt_bytes_ref.into();
        self
    }

    pub fn with_operation_evidence_ref(
        mut self,
        operation_evidence_ref: impl Into<String>,
    ) -> Self {
        self.operation_evidence_ref = operation_evidence_ref.into();
        self
    }

    pub fn with_operation_evidence_target_path_ref(
        mut self,
        operation_evidence_target_path_ref: impl Into<String>,
    ) -> Self {
        self.operation_evidence_target_path_ref = operation_evidence_target_path_ref.into();
        self
    }

    pub fn with_operation_evidence_bytes_ref(
        mut self,
        operation_evidence_bytes_ref: impl Into<String>,
    ) -> Self {
        self.operation_evidence_bytes_ref = operation_evidence_bytes_ref.into();
        self
    }

    pub fn with_operation_evidence_write_result_ref(
        mut self,
        operation_evidence_write_result_ref: impl Into<String>,
    ) -> Self {
        self.operation_evidence_write_result_ref = operation_evidence_write_result_ref.into();
        self
    }

    pub fn with_idempotency_ref(mut self, idempotency_ref: impl Into<String>) -> Self {
        self.idempotency_ref = idempotency_ref.into();
        self
    }

    pub fn with_rollback_ref(mut self, rollback_ref: impl Into<String>) -> Self {
        self.rollback_ref = rollback_ref.into();
        self
    }

    pub fn with_error_ref(mut self, error_ref: impl Into<String>) -> Self {
        self.error_ref = error_ref.into();
        self
    }

    pub fn with_adapter_invocation_receipt_ref(
        mut self,
        adapter_invocation_receipt_ref: impl Into<String>,
    ) -> Self {
        self.adapter_invocation_receipt_ref = adapter_invocation_receipt_ref.into();
        self
    }

    pub fn with_payload_ref(mut self, payload_ref: impl Into<String>) -> Self {
        self.payload_ref = payload_ref.into();
        self
    }

    pub fn with_channel_ref(mut self, channel_ref: impl Into<String>) -> Self {
        self.channel_ref = channel_ref.into();
        self
    }

    pub fn with_connector_profile_resolution_ref(
        mut self,
        connector_profile_resolution_ref: impl Into<String>,
    ) -> Self {
        self.connector_profile_resolution_ref = connector_profile_resolution_ref.into();
        self
    }

    pub fn with_connector_profile_ref(mut self, connector_profile_ref: impl Into<String>) -> Self {
        self.connector_profile_ref = connector_profile_ref.into();
        self
    }

    pub fn with_selected_connector_strategy_ref(
        mut self,
        selected_connector_strategy_ref: impl Into<String>,
    ) -> Self {
        self.selected_connector_strategy_ref = selected_connector_strategy_ref.into();
        self
    }

    pub fn with_allowed_source_refs(mut self, allowed_source_refs: Vec<impl Into<String>>) -> Self {
        self.allowed_source_refs = allowed_source_refs.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_instruction_refs(mut self, instruction_refs: Vec<impl Into<String>>) -> Self {
        self.instruction_refs = instruction_refs.into_iter().map(Into::into).collect();
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

    pub fn with_token_cost_ref(mut self, token_cost_ref: impl Into<String>) -> Self {
        self.token_cost_ref = Some(token_cost_ref.into());
        self
    }

    pub fn try_into_operation_evidence_handoff_refs(
        &self,
    ) -> Result<
        PubPunkPublishOperationEvidenceHandoffRefs,
        PubPunkPublishOperationEvidenceHandoffPacketAssessment,
    > {
        let assessment = assess_pubpunk_publish_operation_evidence_handoff_packet(self);
        if assessment.has_blockers() {
            return Err(assessment);
        }

        Ok(PubPunkPublishOperationEvidenceHandoffRefs {
            publish_receipt_write_handoff_ref: self.publish_receipt_write_handoff_ref.clone(),
            receipt_writer_result_ref: self.receipt_writer_result_ref.clone(),
            storage_ref: self.receipt_storage_ref.clone(),
            receipt_target_ref: self.receipt_target_ref.clone(),
            receipt_target_path_ref: self.receipt_target_path_ref.clone(),
            receipt_bytes_ref: self.receipt_bytes_ref.clone(),
            operation_evidence_ref: self.operation_evidence_ref.clone(),
            operation_evidence_target_path_ref: self.operation_evidence_target_path_ref.clone(),
            operation_evidence_bytes_ref: self.operation_evidence_bytes_ref.clone(),
            operation_evidence_write_result_ref: self.operation_evidence_write_result_ref.clone(),
            idempotency_ref: self.idempotency_ref.clone(),
            rollback_ref: self.rollback_ref.clone(),
            error_ref: self.error_ref.clone(),
            adapter_invocation_receipt_ref: self.adapter_invocation_receipt_ref.clone(),
            payload_ref: self.payload_ref.clone(),
            channel_ref: self.channel_ref.clone(),
            connector_profile_resolution_ref: self.connector_profile_resolution_ref.clone(),
            connector_profile_ref: self.connector_profile_ref.clone(),
            selected_connector_strategy_ref: self.selected_connector_strategy_ref.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishOperationEvidenceHandoffRefs {
    pub publish_receipt_write_handoff_ref: String,
    pub receipt_writer_result_ref: String,
    pub storage_ref: String,
    pub receipt_target_ref: String,
    pub receipt_target_path_ref: String,
    pub receipt_bytes_ref: String,
    pub operation_evidence_ref: String,
    pub operation_evidence_target_path_ref: String,
    pub operation_evidence_bytes_ref: String,
    pub operation_evidence_write_result_ref: String,
    pub idempotency_ref: String,
    pub rollback_ref: String,
    pub error_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub channel_ref: String,
    pub connector_profile_resolution_ref: String,
    pub connector_profile_ref: String,
    pub selected_connector_strategy_ref: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkPublishOperationEvidenceHandoffPacketFindingCode {
    MissingModuleId,
    NonCanonicalModuleId,
    MissingModuleVersionRef,
    MissingContractRef,
    MissingRunRef,
    MissingProjectRef,
    UnsupportedWorkspacePolicy,
    MissingPublishingWorkspaceRef,
    UnsafePublishingWorkspaceRef,
    MissingPublishReceiptWriteHandoffRef,
    UnsafePublishReceiptWriteHandoffRef,
    PublishReceiptWriteHandoffRefNotAllowed,
    MissingReceiptWriterResultRef,
    UnsafeReceiptWriterResultRef,
    ReceiptWriterResultRefNotAllowed,
    MissingReceiptStorageRef,
    UnsafeReceiptStorageRef,
    MissingReceiptTargetRef,
    UnsafeReceiptTargetRef,
    MissingReceiptTargetPathRef,
    UnsafeReceiptTargetPathRef,
    ReceiptTargetPathOutsidePunkRuns,
    MissingReceiptBytesRef,
    UnsafeReceiptBytesRef,
    ReceiptBytesRefNotAllowed,
    MissingOperationEvidenceRef,
    UnsafeOperationEvidenceRef,
    MissingOperationEvidenceTargetPathRef,
    UnsafeOperationEvidenceTargetPathRef,
    OperationEvidenceTargetPathOutsidePunkRuns,
    OperationEvidenceTargetPathMismatch,
    MissingOperationEvidenceBytesRef,
    UnsafeOperationEvidenceBytesRef,
    OperationEvidenceBytesRefNotAllowed,
    MissingOperationEvidenceWriteResultRef,
    UnsafeOperationEvidenceWriteResultRef,
    MissingIdempotencyRef,
    UnsafeIdempotencyRef,
    MissingRollbackRef,
    UnsafeRollbackRef,
    MissingErrorRef,
    UnsafeErrorRef,
    MissingAdapterInvocationReceiptRef,
    UnsafeAdapterInvocationReceiptRef,
    AdapterInvocationReceiptRefNotAllowed,
    MissingPayloadRef,
    UnsafePayloadRef,
    PayloadRefNotAllowed,
    MissingChannelRef,
    UnsafeChannelRef,
    ChannelRefNotAllowed,
    MissingConnectorProfileResolutionRef,
    UnsafeConnectorProfileResolutionRef,
    ConnectorProfileResolutionRefNotAllowed,
    MissingConnectorProfileRef,
    UnsafeConnectorProfileRef,
    ConnectorProfileRefNotAllowed,
    MissingSelectedConnectorStrategyRef,
    UnsafeSelectedConnectorStrategyRef,
    SelectedConnectorStrategyRefNotAllowed,
    MissingInstructionRefs,
    MissingRequiredInstructionRef,
    UnsafeInstructionRef,
    UnsafeAllowedSourceRef,
    MissingRequestOperationEvidenceWriteGrant,
    UnsupportedCapabilityGrant,
    MissingExpectedReceiptFields,
    MissingRequiredExpectedReceiptField,
    UnsafePrivacyPolicy,
    UnsafeTokenCostRef,
}

impl PubPunkPublishOperationEvidenceHandoffPacketFindingCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingModuleId => "missing_module_id",
            Self::NonCanonicalModuleId => "non_canonical_module_id",
            Self::MissingModuleVersionRef => "missing_module_version_ref",
            Self::MissingContractRef => "missing_contract_ref",
            Self::MissingRunRef => "missing_run_ref",
            Self::MissingProjectRef => "missing_project_ref",
            Self::UnsupportedWorkspacePolicy => "unsupported_workspace_policy",
            Self::MissingPublishingWorkspaceRef => "missing_publishing_workspace_ref",
            Self::UnsafePublishingWorkspaceRef => "unsafe_publishing_workspace_ref",
            Self::MissingPublishReceiptWriteHandoffRef => {
                "missing_publish_receipt_write_handoff_ref"
            }
            Self::UnsafePublishReceiptWriteHandoffRef => "unsafe_publish_receipt_write_handoff_ref",
            Self::PublishReceiptWriteHandoffRefNotAllowed => {
                "publish_receipt_write_handoff_ref_not_allowed"
            }
            Self::MissingReceiptWriterResultRef => "missing_receipt_writer_result_ref",
            Self::UnsafeReceiptWriterResultRef => "unsafe_receipt_writer_result_ref",
            Self::ReceiptWriterResultRefNotAllowed => "receipt_writer_result_ref_not_allowed",
            Self::MissingReceiptStorageRef => "missing_receipt_storage_ref",
            Self::UnsafeReceiptStorageRef => "unsafe_receipt_storage_ref",
            Self::MissingReceiptTargetRef => "missing_receipt_target_ref",
            Self::UnsafeReceiptTargetRef => "unsafe_receipt_target_ref",
            Self::MissingReceiptTargetPathRef => "missing_receipt_target_path_ref",
            Self::UnsafeReceiptTargetPathRef => "unsafe_receipt_target_path_ref",
            Self::ReceiptTargetPathOutsidePunkRuns => "receipt_target_path_outside_punk_runs",
            Self::MissingReceiptBytesRef => "missing_receipt_bytes_ref",
            Self::UnsafeReceiptBytesRef => "unsafe_receipt_bytes_ref",
            Self::ReceiptBytesRefNotAllowed => "receipt_bytes_ref_not_allowed",
            Self::MissingOperationEvidenceRef => "missing_operation_evidence_ref",
            Self::UnsafeOperationEvidenceRef => "unsafe_operation_evidence_ref",
            Self::MissingOperationEvidenceTargetPathRef => {
                "missing_operation_evidence_target_path_ref"
            }
            Self::UnsafeOperationEvidenceTargetPathRef => {
                "unsafe_operation_evidence_target_path_ref"
            }
            Self::OperationEvidenceTargetPathOutsidePunkRuns => {
                "operation_evidence_target_path_outside_punk_runs"
            }
            Self::OperationEvidenceTargetPathMismatch => "operation_evidence_target_path_mismatch",
            Self::MissingOperationEvidenceBytesRef => "missing_operation_evidence_bytes_ref",
            Self::UnsafeOperationEvidenceBytesRef => "unsafe_operation_evidence_bytes_ref",
            Self::OperationEvidenceBytesRefNotAllowed => "operation_evidence_bytes_ref_not_allowed",
            Self::MissingOperationEvidenceWriteResultRef => {
                "missing_operation_evidence_write_result_ref"
            }
            Self::UnsafeOperationEvidenceWriteResultRef => {
                "unsafe_operation_evidence_write_result_ref"
            }
            Self::MissingIdempotencyRef => "missing_idempotency_ref",
            Self::UnsafeIdempotencyRef => "unsafe_idempotency_ref",
            Self::MissingRollbackRef => "missing_rollback_ref",
            Self::UnsafeRollbackRef => "unsafe_rollback_ref",
            Self::MissingErrorRef => "missing_error_ref",
            Self::UnsafeErrorRef => "unsafe_error_ref",
            Self::MissingAdapterInvocationReceiptRef => "missing_adapter_invocation_receipt_ref",
            Self::UnsafeAdapterInvocationReceiptRef => "unsafe_adapter_invocation_receipt_ref",
            Self::AdapterInvocationReceiptRefNotAllowed => {
                "adapter_invocation_receipt_ref_not_allowed"
            }
            Self::MissingPayloadRef => "missing_payload_ref",
            Self::UnsafePayloadRef => "unsafe_payload_ref",
            Self::PayloadRefNotAllowed => "payload_ref_not_allowed",
            Self::MissingChannelRef => "missing_channel_ref",
            Self::UnsafeChannelRef => "unsafe_channel_ref",
            Self::ChannelRefNotAllowed => "channel_ref_not_allowed",
            Self::MissingConnectorProfileResolutionRef => {
                "missing_connector_profile_resolution_ref"
            }
            Self::UnsafeConnectorProfileResolutionRef => "unsafe_connector_profile_resolution_ref",
            Self::ConnectorProfileResolutionRefNotAllowed => {
                "connector_profile_resolution_ref_not_allowed"
            }
            Self::MissingConnectorProfileRef => "missing_connector_profile_ref",
            Self::UnsafeConnectorProfileRef => "unsafe_connector_profile_ref",
            Self::ConnectorProfileRefNotAllowed => "connector_profile_ref_not_allowed",
            Self::MissingSelectedConnectorStrategyRef => "missing_selected_connector_strategy_ref",
            Self::UnsafeSelectedConnectorStrategyRef => "unsafe_selected_connector_strategy_ref",
            Self::SelectedConnectorStrategyRefNotAllowed => {
                "selected_connector_strategy_ref_not_allowed"
            }
            Self::MissingInstructionRefs => "missing_instruction_refs",
            Self::MissingRequiredInstructionRef => "missing_required_instruction_ref",
            Self::UnsafeInstructionRef => "unsafe_instruction_ref",
            Self::UnsafeAllowedSourceRef => "unsafe_allowed_source_ref",
            Self::MissingRequestOperationEvidenceWriteGrant => {
                "missing_request_operation_evidence_write_grant"
            }
            Self::UnsupportedCapabilityGrant => "unsupported_capability_grant",
            Self::MissingExpectedReceiptFields => "missing_expected_receipt_fields",
            Self::MissingRequiredExpectedReceiptField => "missing_required_expected_receipt_field",
            Self::UnsafePrivacyPolicy => "unsafe_privacy_policy",
            Self::UnsafeTokenCostRef => "unsafe_token_cost_ref",
        }
    }

    pub fn is_blocking(self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishOperationEvidenceHandoffPacketFinding {
    pub code: PubPunkPublishOperationEvidenceHandoffPacketFindingCode,
    pub ref_value: Option<String>,
    pub capability: Option<PubPunkCapabilityGrant>,
    pub message: &'static str,
}

impl PubPunkPublishOperationEvidenceHandoffPacketFinding {
    fn new(
        code: PubPunkPublishOperationEvidenceHandoffPacketFindingCode,
        message: &'static str,
    ) -> Self {
        Self {
            code,
            ref_value: None,
            capability: None,
            message,
        }
    }

    fn for_ref(
        code: PubPunkPublishOperationEvidenceHandoffPacketFindingCode,
        ref_value: impl Into<String>,
        message: &'static str,
    ) -> Self {
        Self {
            code,
            ref_value: Some(ref_value.into()),
            capability: None,
            message,
        }
    }

    fn for_capability(capability: PubPunkCapabilityGrant) -> Self {
        Self {
            code: PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsupportedCapabilityGrant,
            ref_value: None,
            capability: Some(capability),
            message: "capability is not available in the side-effect-free publish operation-evidence handoff packet",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishOperationEvidenceHandoffPacketRefs {
    pub module_id: String,
    pub module_version_ref: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub workspace_policy: PubPunkWorkspacePolicy,
    pub publishing_workspace_ref: String,
    pub publish_receipt_write_handoff_ref: String,
    pub receipt_writer_result_ref: String,
    pub receipt_storage_ref: String,
    pub receipt_target_ref: String,
    pub receipt_target_path_ref: String,
    pub receipt_bytes_ref: String,
    pub operation_evidence_ref: String,
    pub operation_evidence_target_path_ref: String,
    pub operation_evidence_bytes_ref: String,
    pub operation_evidence_write_result_ref: String,
    pub idempotency_ref: String,
    pub rollback_ref: String,
    pub error_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub channel_ref: String,
    pub connector_profile_resolution_ref: String,
    pub connector_profile_ref: String,
    pub selected_connector_strategy_ref: String,
    pub token_cost_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishOperationEvidenceHandoffPacketAssessment {
    pub schema_version: &'static str,
    pub status: PubPunkAssessmentStatus,
    pub authority: PubPunkAssessmentAuthority,
    pub requested_operation: PubPunkPublishOperationEvidenceHandoffOperation,
    pub findings: Vec<PubPunkPublishOperationEvidenceHandoffPacketFinding>,
    pub boundary_flags: PubPunkInventoryBoundaryFlags,
    pub refs: PubPunkPublishOperationEvidenceHandoffPacketRefs,
}

impl PubPunkPublishOperationEvidenceHandoffPacketAssessment {
    pub fn blocking_findings(
        &self,
    ) -> impl Iterator<Item = &PubPunkPublishOperationEvidenceHandoffPacketFinding> {
        self.findings
            .iter()
            .filter(|finding| finding.code.is_blocking())
    }

    pub fn has_blockers(&self) -> bool {
        self.blocking_findings().next().is_some()
    }
}

pub fn assess_pubpunk_publish_operation_evidence_handoff_packet(
    packet: &PubPunkPublishOperationEvidenceHandoffPacket,
) -> PubPunkPublishOperationEvidenceHandoffPacketAssessment {
    let mut findings = Vec::new();

    push_publish_operation_evidence_handoff_required_ref_finding(
        &mut findings,
        packet.module_id.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingModuleId,
        "module id is required",
    );
    if !packet.module_id.trim().is_empty() && packet.module_id != PUBPUNK_MODULE_ID {
        findings.push(
            PubPunkPublishOperationEvidenceHandoffPacketFinding::for_ref(
                PubPunkPublishOperationEvidenceHandoffPacketFindingCode::NonCanonicalModuleId,
                packet.module_id.clone(),
                "PubPunk publish operation-evidence handoff packets must use the canonical pubpunk module id",
            ),
        );
    }
    push_publish_operation_evidence_handoff_required_ref_finding(
        &mut findings,
        packet.module_version_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingModuleVersionRef,
        "module version ref is required",
    );
    push_publish_operation_evidence_handoff_required_ref_finding(
        &mut findings,
        packet.contract_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingContractRef,
        "contract ref is required",
    );
    push_publish_operation_evidence_handoff_required_ref_finding(
        &mut findings,
        packet.run_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingRunRef,
        "run ref is required",
    );
    push_publish_operation_evidence_handoff_required_ref_finding(
        &mut findings,
        packet.project_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingProjectRef,
        "project ref is required",
    );

    if !packet.workspace_policy.selected_for_first_slice() {
        findings.push(
            PubPunkPublishOperationEvidenceHandoffPacketFinding::for_ref(
                PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsupportedWorkspacePolicy,
                packet.workspace_policy.as_str(),
                "the first PubPunk publish operation-evidence handoff supports split explicit refs only",
            ),
        );
    }

    push_publish_operation_evidence_handoff_required_ref_finding(
        &mut findings,
        packet.publishing_workspace_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingPublishingWorkspaceRef,
        "publishing workspace ref is required",
    );
    if !packet.publishing_workspace_ref.trim().is_empty()
        && !is_safe_workspace_ref(&packet.publishing_workspace_ref)
    {
        findings.push(
            PubPunkPublishOperationEvidenceHandoffPacketFinding::for_ref(
                PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafePublishingWorkspaceRef,
                packet.publishing_workspace_ref.clone(),
                "publishing workspace ref must be an explicit safe logical or repo-relative ref",
            ),
        );
    }

    validate_publish_operation_evidence_handoff_ref(
        &mut findings,
        packet.publish_receipt_write_handoff_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingPublishReceiptWriteHandoffRef,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafePublishReceiptWriteHandoffRef,
        "publish receipt write handoff ref is required",
        "publish receipt write handoff ref must be an explicit repo-relative ref",
    );
    validate_publish_operation_evidence_handoff_ref(
        &mut findings,
        packet.receipt_writer_result_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingReceiptWriterResultRef,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeReceiptWriterResultRef,
        "receipt writer result ref is required",
        "receipt writer result ref must be an explicit repo-relative ref",
    );
    validate_publish_operation_evidence_handoff_ref(
        &mut findings,
        packet.receipt_storage_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingReceiptStorageRef,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeReceiptStorageRef,
        "receipt storage ref is required",
        "receipt storage ref must be an explicit repo-relative ref",
    );
    validate_publish_operation_evidence_handoff_ref(
        &mut findings,
        packet.receipt_target_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingReceiptTargetRef,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeReceiptTargetRef,
        "receipt target ref is required",
        "receipt target ref must be an explicit repo-relative ref",
    );
    validate_publish_operation_evidence_handoff_ref(
        &mut findings,
        packet.receipt_target_path_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingReceiptTargetPathRef,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeReceiptTargetPathRef,
        "receipt target path ref is required",
        "receipt target path ref must be an explicit repo-relative ref",
    );
    if !packet.receipt_target_path_ref.trim().is_empty()
        && is_safe_source_ref(&packet.receipt_target_path_ref)
        && !is_punk_runs_target_ref(&packet.receipt_target_path_ref)
    {
        findings.push(
            PubPunkPublishOperationEvidenceHandoffPacketFinding::for_ref(
                PubPunkPublishOperationEvidenceHandoffPacketFindingCode::ReceiptTargetPathOutsidePunkRuns,
                packet.receipt_target_path_ref.clone(),
                "receipt target path ref must stay under .punk/runs for the operation-evidence handoff",
            ),
        );
    }
    validate_publish_operation_evidence_handoff_ref(
        &mut findings,
        packet.receipt_bytes_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingReceiptBytesRef,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeReceiptBytesRef,
        "receipt bytes ref is required",
        "receipt bytes ref must be an explicit repo-relative ref",
    );
    validate_publish_operation_evidence_handoff_ref(
        &mut findings,
        packet.operation_evidence_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingOperationEvidenceRef,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeOperationEvidenceRef,
        "operation evidence ref is required",
        "operation evidence ref must be an explicit repo-relative ref",
    );
    validate_publish_operation_evidence_handoff_ref(
        &mut findings,
        packet.operation_evidence_target_path_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingOperationEvidenceTargetPathRef,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeOperationEvidenceTargetPathRef,
        "operation evidence target path ref is required",
        "operation evidence target path ref must be an explicit repo-relative ref",
    );
    if !packet.operation_evidence_target_path_ref.trim().is_empty()
        && is_safe_source_ref(&packet.operation_evidence_target_path_ref)
        && !is_punk_runs_target_ref(&packet.operation_evidence_target_path_ref)
    {
        findings.push(
            PubPunkPublishOperationEvidenceHandoffPacketFinding::for_ref(
                PubPunkPublishOperationEvidenceHandoffPacketFindingCode::OperationEvidenceTargetPathOutsidePunkRuns,
                packet.operation_evidence_target_path_ref.clone(),
                "operation evidence target path ref must stay under .punk/runs for the first active handoff",
            ),
        );
    }
    if !packet.operation_evidence_ref.trim().is_empty()
        && !packet.operation_evidence_target_path_ref.trim().is_empty()
        && is_safe_source_ref(&packet.operation_evidence_ref)
        && is_safe_source_ref(&packet.operation_evidence_target_path_ref)
        && packet.operation_evidence_ref != packet.operation_evidence_target_path_ref
    {
        findings.push(
            PubPunkPublishOperationEvidenceHandoffPacketFinding::for_ref(
                PubPunkPublishOperationEvidenceHandoffPacketFindingCode::OperationEvidenceTargetPathMismatch,
                packet.operation_evidence_target_path_ref.clone(),
                "operation evidence target path must match the receipt writer result operation evidence ref",
            ),
        );
    }
    validate_publish_operation_evidence_handoff_ref(
        &mut findings,
        packet.operation_evidence_bytes_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingOperationEvidenceBytesRef,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeOperationEvidenceBytesRef,
        "operation evidence bytes ref is required",
        "operation evidence bytes ref must be an explicit repo-relative ref",
    );
    validate_publish_operation_evidence_handoff_ref(
        &mut findings,
        packet.operation_evidence_write_result_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingOperationEvidenceWriteResultRef,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeOperationEvidenceWriteResultRef,
        "operation evidence write result ref is required",
        "operation evidence write result ref must be an explicit repo-relative ref",
    );
    validate_publish_operation_evidence_handoff_ref(
        &mut findings,
        packet.idempotency_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingIdempotencyRef,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeIdempotencyRef,
        "idempotency ref is required",
        "idempotency ref must be an explicit repo-relative ref",
    );
    validate_publish_operation_evidence_handoff_ref(
        &mut findings,
        packet.rollback_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingRollbackRef,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeRollbackRef,
        "rollback ref is required",
        "rollback ref must be an explicit repo-relative ref",
    );
    validate_publish_operation_evidence_handoff_ref(
        &mut findings,
        packet.error_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingErrorRef,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeErrorRef,
        "error ref is required",
        "error ref must be an explicit repo-relative ref",
    );
    validate_publish_operation_evidence_handoff_ref(
        &mut findings,
        packet.adapter_invocation_receipt_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingAdapterInvocationReceiptRef,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeAdapterInvocationReceiptRef,
        "adapter invocation receipt ref is required",
        "adapter invocation receipt ref must be an explicit repo-relative ref",
    );
    validate_publish_operation_evidence_handoff_ref(
        &mut findings,
        packet.payload_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingPayloadRef,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafePayloadRef,
        "payload ref is required",
        "payload ref must be an explicit repo-relative ref",
    );
    validate_publish_operation_evidence_handoff_ref(
        &mut findings,
        packet.channel_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingChannelRef,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeChannelRef,
        "channel ref is required",
        "channel ref must be an explicit repo-relative ref",
    );
    validate_publish_operation_evidence_handoff_ref(
        &mut findings,
        packet.connector_profile_resolution_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingConnectorProfileResolutionRef,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeConnectorProfileResolutionRef,
        "connector profile resolution ref is required",
        "connector profile resolution ref must be an explicit repo-relative ref",
    );
    validate_publish_operation_evidence_handoff_ref(
        &mut findings,
        packet.connector_profile_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingConnectorProfileRef,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeConnectorProfileRef,
        "connector profile ref is required",
        "connector profile ref must be an explicit repo-relative ref",
    );
    validate_publish_operation_evidence_handoff_ref(
        &mut findings,
        packet.selected_connector_strategy_ref.as_str(),
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingSelectedConnectorStrategyRef,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeSelectedConnectorStrategyRef,
        "selected connector strategy ref is required",
        "selected connector strategy ref must be an explicit repo-relative ref",
    );

    if packet.instruction_refs.is_empty() {
        findings.push(PubPunkPublishOperationEvidenceHandoffPacketFinding::new(
            PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingInstructionRefs,
            "instruction refs are required for PubPunk publish operation-evidence handoff packets",
        ));
    }

    for required_ref in PUBPUNK_REQUIRED_INSTRUCTION_REFS {
        if !packet
            .instruction_refs
            .iter()
            .any(|instruction_ref| instruction_ref == required_ref)
        {
            findings.push(
                PubPunkPublishOperationEvidenceHandoffPacketFinding::for_ref(
                    PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingRequiredInstructionRef,
                    *required_ref,
                    "required PubPunk instruction ref is missing",
                ),
            );
        }
    }

    for instruction_ref in &packet.instruction_refs {
        if !is_safe_source_ref(instruction_ref) {
            findings.push(
                PubPunkPublishOperationEvidenceHandoffPacketFinding::for_ref(
                    PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeInstructionRef,
                    instruction_ref.clone(),
                    "instruction refs must be explicit repo-relative refs",
                ),
            );
        }
    }

    for source_ref in &packet.allowed_source_refs {
        if !is_safe_source_ref(source_ref) {
            findings.push(
                PubPunkPublishOperationEvidenceHandoffPacketFinding::for_ref(
                    PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeAllowedSourceRef,
                    source_ref.clone(),
                    "allowed source refs must be explicit repo-relative refs",
                ),
            );
        }
    }

    for (source_ref, code, message) in [
        (
            packet.publish_receipt_write_handoff_ref.as_str(),
            PubPunkPublishOperationEvidenceHandoffPacketFindingCode::PublishReceiptWriteHandoffRefNotAllowed,
            "publish receipt write handoff ref must be included in allowed source refs",
        ),
        (
            packet.receipt_writer_result_ref.as_str(),
            PubPunkPublishOperationEvidenceHandoffPacketFindingCode::ReceiptWriterResultRefNotAllowed,
            "receipt writer result ref must be included in allowed source refs",
        ),
        (
            packet.payload_ref.as_str(),
            PubPunkPublishOperationEvidenceHandoffPacketFindingCode::PayloadRefNotAllowed,
            "payload ref must be included in allowed source refs",
        ),
        (
            packet.channel_ref.as_str(),
            PubPunkPublishOperationEvidenceHandoffPacketFindingCode::ChannelRefNotAllowed,
            "channel ref must be included in allowed source refs",
        ),
        (
            packet.connector_profile_resolution_ref.as_str(),
            PubPunkPublishOperationEvidenceHandoffPacketFindingCode::ConnectorProfileResolutionRefNotAllowed,
            "connector profile resolution ref must be included in allowed source refs",
        ),
        (
            packet.connector_profile_ref.as_str(),
            PubPunkPublishOperationEvidenceHandoffPacketFindingCode::ConnectorProfileRefNotAllowed,
            "connector profile ref must be included in allowed source refs",
        ),
        (
            packet.selected_connector_strategy_ref.as_str(),
            PubPunkPublishOperationEvidenceHandoffPacketFindingCode::SelectedConnectorStrategyRefNotAllowed,
            "selected connector strategy ref must be included in allowed source refs",
        ),
        (
            packet.adapter_invocation_receipt_ref.as_str(),
            PubPunkPublishOperationEvidenceHandoffPacketFindingCode::AdapterInvocationReceiptRefNotAllowed,
            "adapter invocation receipt ref must be included in allowed source refs",
        ),
        (
            packet.receipt_bytes_ref.as_str(),
            PubPunkPublishOperationEvidenceHandoffPacketFindingCode::ReceiptBytesRefNotAllowed,
            "receipt bytes ref must be included in allowed source refs",
        ),
        (
            packet.operation_evidence_bytes_ref.as_str(),
            PubPunkPublishOperationEvidenceHandoffPacketFindingCode::OperationEvidenceBytesRefNotAllowed,
            "operation evidence bytes ref must be included in allowed source refs",
        ),
    ] {
        if !source_ref.trim().is_empty()
            && !packet
                .allowed_source_refs
                .iter()
                .any(|allowed_ref| allowed_ref == source_ref)
        {
            findings.push(
                PubPunkPublishOperationEvidenceHandoffPacketFinding::for_ref(
                    code,
                    source_ref,
                    message,
                ),
            );
        }
    }

    if !packet
        .granted_capabilities
        .contains(&PubPunkCapabilityGrant::RequestOperationEvidenceWrite)
    {
        findings.push(
            PubPunkPublishOperationEvidenceHandoffPacketFinding::new(
                PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingRequestOperationEvidenceWriteGrant,
                "request_operation_evidence_write must be explicitly granted for this packet",
            ),
        );
    }
    for capability in &packet.granted_capabilities {
        if !capability.supported_by_side_effect_free_publish_operation_evidence_handoff() {
            findings.push(
                PubPunkPublishOperationEvidenceHandoffPacketFinding::for_capability(*capability),
            );
        }
    }

    if packet.expected_receipt_fields.is_empty() {
        findings.push(
            PubPunkPublishOperationEvidenceHandoffPacketFinding::new(
                PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingExpectedReceiptFields,
                "expected receipt fields are required even though this packet does not write operation evidence",
            ),
        );
    }
    for required_field in [
        "side_effects",
        "host_validation",
        "connector_profile_resolution",
        "connector_profile_ref",
        "selected_connector_strategy",
        "adapter_invocation_receipt",
        "operation_evidence",
        "publication_receipt",
        "receipt_write_result",
        "operation_evidence_bytes",
        "operation_evidence_target_path",
        "operation_evidence_write_result",
    ] {
        if !packet
            .expected_receipt_fields
            .iter()
            .any(|field| field == required_field)
        {
            findings.push(
                PubPunkPublishOperationEvidenceHandoffPacketFinding::for_ref(
                    PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingRequiredExpectedReceiptField,
                    required_field,
                    "publish operation-evidence handoff expectations must include host, receipt writer result, evidence bytes, target path, and write-result coverage",
                ),
            );
        }
    }

    if packet.privacy_policy.allows_private_or_raw_payloads() {
        findings.push(PubPunkPublishOperationEvidenceHandoffPacketFinding::new(
            PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafePrivacyPolicy,
            "privacy policy must disallow raw/private payloads for this packet",
        ));
    }

    if let Some(token_cost_ref) = &packet.token_cost_ref {
        if !is_safe_source_ref(token_cost_ref) {
            findings.push(
                PubPunkPublishOperationEvidenceHandoffPacketFinding::for_ref(
                    PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeTokenCostRef,
                    token_cost_ref.clone(),
                    "token cost ref must be an explicit repo-relative ref when provided",
                ),
            );
        }
    }

    let status = if findings.iter().any(|finding| finding.code.is_blocking()) {
        PubPunkAssessmentStatus::Blocked
    } else {
        PubPunkAssessmentStatus::Ready
    };

    PubPunkPublishOperationEvidenceHandoffPacketAssessment {
        schema_version: PUBPUNK_PUBLISH_OPERATION_EVIDENCE_HANDOFF_PACKET_SCHEMA_VERSION,
        status,
        authority: PubPunkAssessmentAuthority::Advisory,
        requested_operation:
            PubPunkPublishOperationEvidenceHandoffOperation::PreparePublishOperationEvidenceHandoff,
        findings,
        boundary_flags: PUBPUNK_INVENTORY_ASSESSMENT_BOUNDARY_FLAGS,
        refs: PubPunkPublishOperationEvidenceHandoffPacketRefs {
            module_id: packet.module_id.clone(),
            module_version_ref: packet.module_version_ref.clone(),
            contract_ref: packet.contract_ref.clone(),
            run_ref: packet.run_ref.clone(),
            project_ref: packet.project_ref.clone(),
            workspace_policy: packet.workspace_policy,
            publishing_workspace_ref: packet.publishing_workspace_ref.clone(),
            publish_receipt_write_handoff_ref: packet.publish_receipt_write_handoff_ref.clone(),
            receipt_writer_result_ref: packet.receipt_writer_result_ref.clone(),
            receipt_storage_ref: packet.receipt_storage_ref.clone(),
            receipt_target_ref: packet.receipt_target_ref.clone(),
            receipt_target_path_ref: packet.receipt_target_path_ref.clone(),
            receipt_bytes_ref: packet.receipt_bytes_ref.clone(),
            operation_evidence_ref: packet.operation_evidence_ref.clone(),
            operation_evidence_target_path_ref: packet.operation_evidence_target_path_ref.clone(),
            operation_evidence_bytes_ref: packet.operation_evidence_bytes_ref.clone(),
            operation_evidence_write_result_ref: packet.operation_evidence_write_result_ref.clone(),
            idempotency_ref: packet.idempotency_ref.clone(),
            rollback_ref: packet.rollback_ref.clone(),
            error_ref: packet.error_ref.clone(),
            adapter_invocation_receipt_ref: packet.adapter_invocation_receipt_ref.clone(),
            payload_ref: packet.payload_ref.clone(),
            channel_ref: packet.channel_ref.clone(),
            connector_profile_resolution_ref: packet.connector_profile_resolution_ref.clone(),
            connector_profile_ref: packet.connector_profile_ref.clone(),
            selected_connector_strategy_ref: packet.selected_connector_strategy_ref.clone(),
            token_cost_ref: packet.token_cost_ref.clone(),
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkPublishReceiptEvidenceEventHandoffOperation {
    PreparePublishReceiptEvidenceEventHandoff,
}

impl PubPunkPublishReceiptEvidenceEventHandoffOperation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PreparePublishReceiptEvidenceEventHandoff => {
                "prepare_publish_receipt_evidence_event_handoff"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishReceiptEvidenceEventHandoffPacket {
    pub module_id: String,
    pub module_version_ref: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub workspace_policy: PubPunkWorkspacePolicy,
    pub publishing_workspace_ref: String,
    pub publish_operation_evidence_handoff_ref: String,
    pub receipt_writer_result_ref: String,
    pub operation_evidence_write_result_ref: String,
    pub receipt_ref: String,
    pub operation_evidence_ref: String,
    pub event_log_ref: String,
    pub event_source_ref: String,
    pub event_correlation_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub channel_ref: String,
    pub connector_profile_ref: String,
    pub allowed_source_refs: Vec<String>,
    pub instruction_refs: Vec<String>,
    pub granted_capabilities: Vec<PubPunkCapabilityGrant>,
    pub privacy_policy: PubPunkPrivacyPolicy,
    pub expected_receipt_fields: Vec<String>,
    pub token_cost_ref: Option<String>,
}

impl PubPunkPublishReceiptEvidenceEventHandoffPacket {
    pub fn new(
        module_version_ref: impl Into<String>,
        contract_ref: impl Into<String>,
        run_ref: impl Into<String>,
        project_ref: impl Into<String>,
        publishing_workspace_ref: impl Into<String>,
    ) -> Self {
        Self {
            module_id: PUBPUNK_MODULE_ID.to_owned(),
            module_version_ref: module_version_ref.into(),
            contract_ref: contract_ref.into(),
            run_ref: run_ref.into(),
            project_ref: project_ref.into(),
            workspace_policy: PubPunkWorkspacePolicy::SplitExplicitRefs,
            publishing_workspace_ref: publishing_workspace_ref.into(),
            publish_operation_evidence_handoff_ref: String::new(),
            receipt_writer_result_ref: String::new(),
            operation_evidence_write_result_ref: String::new(),
            receipt_ref: String::new(),
            operation_evidence_ref: String::new(),
            event_log_ref: String::new(),
            event_source_ref: String::new(),
            event_correlation_ref: String::new(),
            adapter_invocation_receipt_ref: String::new(),
            payload_ref: String::new(),
            channel_ref: String::new(),
            connector_profile_ref: String::new(),
            allowed_source_refs: Vec::new(),
            instruction_refs: Vec::new(),
            granted_capabilities: Vec::new(),
            privacy_policy: PubPunkPrivacyPolicy::safe_metadata_only(),
            expected_receipt_fields: Vec::new(),
            token_cost_ref: None,
        }
    }

    pub fn with_workspace_policy(mut self, workspace_policy: PubPunkWorkspacePolicy) -> Self {
        self.workspace_policy = workspace_policy;
        self
    }

    pub fn with_publish_operation_evidence_handoff_ref(
        mut self,
        publish_operation_evidence_handoff_ref: impl Into<String>,
    ) -> Self {
        self.publish_operation_evidence_handoff_ref = publish_operation_evidence_handoff_ref.into();
        self
    }

    pub fn with_receipt_writer_result_ref(
        mut self,
        receipt_writer_result_ref: impl Into<String>,
    ) -> Self {
        self.receipt_writer_result_ref = receipt_writer_result_ref.into();
        self
    }

    pub fn with_operation_evidence_write_result_ref(
        mut self,
        operation_evidence_write_result_ref: impl Into<String>,
    ) -> Self {
        self.operation_evidence_write_result_ref = operation_evidence_write_result_ref.into();
        self
    }

    pub fn with_receipt_ref(mut self, receipt_ref: impl Into<String>) -> Self {
        self.receipt_ref = receipt_ref.into();
        self
    }

    pub fn with_operation_evidence_ref(
        mut self,
        operation_evidence_ref: impl Into<String>,
    ) -> Self {
        self.operation_evidence_ref = operation_evidence_ref.into();
        self
    }

    pub fn with_event_log_ref(mut self, event_log_ref: impl Into<String>) -> Self {
        self.event_log_ref = event_log_ref.into();
        self
    }

    pub fn with_event_source_ref(mut self, event_source_ref: impl Into<String>) -> Self {
        self.event_source_ref = event_source_ref.into();
        self
    }

    pub fn with_event_correlation_ref(mut self, event_correlation_ref: impl Into<String>) -> Self {
        self.event_correlation_ref = event_correlation_ref.into();
        self
    }

    pub fn with_adapter_invocation_receipt_ref(
        mut self,
        adapter_invocation_receipt_ref: impl Into<String>,
    ) -> Self {
        self.adapter_invocation_receipt_ref = adapter_invocation_receipt_ref.into();
        self
    }

    pub fn with_payload_ref(mut self, payload_ref: impl Into<String>) -> Self {
        self.payload_ref = payload_ref.into();
        self
    }

    pub fn with_channel_ref(mut self, channel_ref: impl Into<String>) -> Self {
        self.channel_ref = channel_ref.into();
        self
    }

    pub fn with_connector_profile_ref(mut self, connector_profile_ref: impl Into<String>) -> Self {
        self.connector_profile_ref = connector_profile_ref.into();
        self
    }

    pub fn with_allowed_source_refs(mut self, allowed_source_refs: Vec<impl Into<String>>) -> Self {
        self.allowed_source_refs = allowed_source_refs.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_instruction_refs(mut self, instruction_refs: Vec<impl Into<String>>) -> Self {
        self.instruction_refs = instruction_refs.into_iter().map(Into::into).collect();
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

    pub fn with_token_cost_ref(mut self, token_cost_ref: impl Into<String>) -> Self {
        self.token_cost_ref = Some(token_cost_ref.into());
        self
    }

    pub fn try_into_receipt_evidence_event_handoff_refs(
        &self,
    ) -> Result<
        PubPunkPublishReceiptEvidenceEventHandoffRefs,
        PubPunkPublishReceiptEvidenceEventHandoffPacketAssessment,
    > {
        let assessment = assess_pubpunk_publish_receipt_evidence_event_handoff_packet(self);
        if assessment.has_blockers() {
            return Err(assessment);
        }

        Ok(PubPunkPublishReceiptEvidenceEventHandoffRefs {
            publish_operation_evidence_handoff_ref: self
                .publish_operation_evidence_handoff_ref
                .clone(),
            receipt_writer_result_ref: self.receipt_writer_result_ref.clone(),
            operation_evidence_write_result_ref: self.operation_evidence_write_result_ref.clone(),
            receipt_ref: self.receipt_ref.clone(),
            operation_evidence_ref: self.operation_evidence_ref.clone(),
            event_log_ref: self.event_log_ref.clone(),
            event_source_ref: self.event_source_ref.clone(),
            event_correlation_ref: self.event_correlation_ref.clone(),
            adapter_invocation_receipt_ref: self.adapter_invocation_receipt_ref.clone(),
            payload_ref: self.payload_ref.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishReceiptEvidenceEventHandoffRefs {
    pub publish_operation_evidence_handoff_ref: String,
    pub receipt_writer_result_ref: String,
    pub operation_evidence_write_result_ref: String,
    pub receipt_ref: String,
    pub operation_evidence_ref: String,
    pub event_log_ref: String,
    pub event_source_ref: String,
    pub event_correlation_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode {
    MissingModuleId,
    NonCanonicalModuleId,
    MissingModuleVersionRef,
    MissingContractRef,
    MissingRunRef,
    MissingProjectRef,
    UnsupportedWorkspacePolicy,
    MissingPublishingWorkspaceRef,
    UnsafePublishingWorkspaceRef,
    MissingPublishOperationEvidenceHandoffRef,
    UnsafePublishOperationEvidenceHandoffRef,
    PublishOperationEvidenceHandoffRefNotAllowed,
    MissingReceiptWriterResultRef,
    UnsafeReceiptWriterResultRef,
    ReceiptWriterResultRefNotAllowed,
    MissingOperationEvidenceWriteResultRef,
    UnsafeOperationEvidenceWriteResultRef,
    OperationEvidenceWriteResultRefNotAllowed,
    MissingReceiptRef,
    UnsafeReceiptRef,
    ReceiptRefOutsidePunkRuns,
    ReceiptRefNotAllowed,
    MissingOperationEvidenceRef,
    UnsafeOperationEvidenceRef,
    OperationEvidenceRefOutsidePunkRuns,
    OperationEvidenceRefNotAllowed,
    ReceiptEvidenceRefsCollapsed,
    MissingEventLogRef,
    UnsafeEventLogRef,
    EventLogRefNotPunkFlowLog,
    MissingEventSourceRef,
    UnsafeEventSourceRef,
    EventSourceRefNotAllowed,
    MissingEventCorrelationRef,
    UnsafeEventCorrelationRef,
    EventCorrelationRefNotAllowed,
    MissingAdapterInvocationReceiptRef,
    UnsafeAdapterInvocationReceiptRef,
    AdapterInvocationReceiptRefNotAllowed,
    MissingPayloadRef,
    UnsafePayloadRef,
    PayloadRefNotAllowed,
    MissingChannelRef,
    UnsafeChannelRef,
    ChannelRefNotAllowed,
    MissingConnectorProfileRef,
    UnsafeConnectorProfileRef,
    ConnectorProfileRefNotAllowed,
    MissingInstructionRefs,
    MissingRequiredInstructionRef,
    UnsafeInstructionRef,
    UnsafeAllowedSourceRef,
    MissingRequestReceiptEvidenceEventHandoffGrant,
    UnsupportedCapabilityGrant,
    MissingExpectedReceiptFields,
    MissingRequiredExpectedReceiptField,
    UnsafePrivacyPolicy,
    UnsafeTokenCostRef,
}

impl PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingModuleId => "missing_module_id",
            Self::NonCanonicalModuleId => "non_canonical_module_id",
            Self::MissingModuleVersionRef => "missing_module_version_ref",
            Self::MissingContractRef => "missing_contract_ref",
            Self::MissingRunRef => "missing_run_ref",
            Self::MissingProjectRef => "missing_project_ref",
            Self::UnsupportedWorkspacePolicy => "unsupported_workspace_policy",
            Self::MissingPublishingWorkspaceRef => "missing_publishing_workspace_ref",
            Self::UnsafePublishingWorkspaceRef => "unsafe_publishing_workspace_ref",
            Self::MissingPublishOperationEvidenceHandoffRef => {
                "missing_publish_operation_evidence_handoff_ref"
            }
            Self::UnsafePublishOperationEvidenceHandoffRef => {
                "unsafe_publish_operation_evidence_handoff_ref"
            }
            Self::PublishOperationEvidenceHandoffRefNotAllowed => {
                "publish_operation_evidence_handoff_ref_not_allowed"
            }
            Self::MissingReceiptWriterResultRef => "missing_receipt_writer_result_ref",
            Self::UnsafeReceiptWriterResultRef => "unsafe_receipt_writer_result_ref",
            Self::ReceiptWriterResultRefNotAllowed => "receipt_writer_result_ref_not_allowed",
            Self::MissingOperationEvidenceWriteResultRef => {
                "missing_operation_evidence_write_result_ref"
            }
            Self::UnsafeOperationEvidenceWriteResultRef => {
                "unsafe_operation_evidence_write_result_ref"
            }
            Self::OperationEvidenceWriteResultRefNotAllowed => {
                "operation_evidence_write_result_ref_not_allowed"
            }
            Self::MissingReceiptRef => "missing_receipt_ref",
            Self::UnsafeReceiptRef => "unsafe_receipt_ref",
            Self::ReceiptRefOutsidePunkRuns => "receipt_ref_outside_punk_runs",
            Self::ReceiptRefNotAllowed => "receipt_ref_not_allowed",
            Self::MissingOperationEvidenceRef => "missing_operation_evidence_ref",
            Self::UnsafeOperationEvidenceRef => "unsafe_operation_evidence_ref",
            Self::OperationEvidenceRefOutsidePunkRuns => "operation_evidence_ref_outside_punk_runs",
            Self::OperationEvidenceRefNotAllowed => "operation_evidence_ref_not_allowed",
            Self::ReceiptEvidenceRefsCollapsed => "receipt_evidence_refs_collapsed",
            Self::MissingEventLogRef => "missing_event_log_ref",
            Self::UnsafeEventLogRef => "unsafe_event_log_ref",
            Self::EventLogRefNotPunkFlowLog => "event_log_ref_not_punk_flow_log",
            Self::MissingEventSourceRef => "missing_event_source_ref",
            Self::UnsafeEventSourceRef => "unsafe_event_source_ref",
            Self::EventSourceRefNotAllowed => "event_source_ref_not_allowed",
            Self::MissingEventCorrelationRef => "missing_event_correlation_ref",
            Self::UnsafeEventCorrelationRef => "unsafe_event_correlation_ref",
            Self::EventCorrelationRefNotAllowed => "event_correlation_ref_not_allowed",
            Self::MissingAdapterInvocationReceiptRef => "missing_adapter_invocation_receipt_ref",
            Self::UnsafeAdapterInvocationReceiptRef => "unsafe_adapter_invocation_receipt_ref",
            Self::AdapterInvocationReceiptRefNotAllowed => {
                "adapter_invocation_receipt_ref_not_allowed"
            }
            Self::MissingPayloadRef => "missing_payload_ref",
            Self::UnsafePayloadRef => "unsafe_payload_ref",
            Self::PayloadRefNotAllowed => "payload_ref_not_allowed",
            Self::MissingChannelRef => "missing_channel_ref",
            Self::UnsafeChannelRef => "unsafe_channel_ref",
            Self::ChannelRefNotAllowed => "channel_ref_not_allowed",
            Self::MissingConnectorProfileRef => "missing_connector_profile_ref",
            Self::UnsafeConnectorProfileRef => "unsafe_connector_profile_ref",
            Self::ConnectorProfileRefNotAllowed => "connector_profile_ref_not_allowed",
            Self::MissingInstructionRefs => "missing_instruction_refs",
            Self::MissingRequiredInstructionRef => "missing_required_instruction_ref",
            Self::UnsafeInstructionRef => "unsafe_instruction_ref",
            Self::UnsafeAllowedSourceRef => "unsafe_allowed_source_ref",
            Self::MissingRequestReceiptEvidenceEventHandoffGrant => {
                "missing_request_receipt_evidence_event_handoff_grant"
            }
            Self::UnsupportedCapabilityGrant => "unsupported_capability_grant",
            Self::MissingExpectedReceiptFields => "missing_expected_receipt_fields",
            Self::MissingRequiredExpectedReceiptField => "missing_required_expected_receipt_field",
            Self::UnsafePrivacyPolicy => "unsafe_privacy_policy",
            Self::UnsafeTokenCostRef => "unsafe_token_cost_ref",
        }
    }

    pub fn is_blocking(self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishReceiptEvidenceEventHandoffPacketFinding {
    pub code: PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode,
    pub ref_value: Option<String>,
    pub capability: Option<PubPunkCapabilityGrant>,
    pub message: &'static str,
}

impl PubPunkPublishReceiptEvidenceEventHandoffPacketFinding {
    fn new(
        code: PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode,
        message: &'static str,
    ) -> Self {
        Self {
            code,
            ref_value: None,
            capability: None,
            message,
        }
    }

    fn for_ref(
        code: PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode,
        ref_value: impl Into<String>,
        message: &'static str,
    ) -> Self {
        Self {
            code,
            ref_value: Some(ref_value.into()),
            capability: None,
            message,
        }
    }

    fn for_capability(capability: PubPunkCapabilityGrant) -> Self {
        Self {
            code: PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsupportedCapabilityGrant,
            ref_value: None,
            capability: Some(capability),
            message: "capability is not available in the side-effect-free publish receipt/evidence event handoff packet",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishReceiptEvidenceEventHandoffPacketRefs {
    pub module_id: String,
    pub module_version_ref: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub workspace_policy: PubPunkWorkspacePolicy,
    pub publishing_workspace_ref: String,
    pub publish_operation_evidence_handoff_ref: String,
    pub receipt_writer_result_ref: String,
    pub operation_evidence_write_result_ref: String,
    pub receipt_ref: String,
    pub operation_evidence_ref: String,
    pub event_log_ref: String,
    pub event_source_ref: String,
    pub event_correlation_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub channel_ref: String,
    pub connector_profile_ref: String,
    pub token_cost_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubPunkPublishReceiptEvidenceEventHandoffPacketAssessment {
    pub schema_version: &'static str,
    pub status: PubPunkAssessmentStatus,
    pub authority: PubPunkAssessmentAuthority,
    pub requested_operation: PubPunkPublishReceiptEvidenceEventHandoffOperation,
    pub findings: Vec<PubPunkPublishReceiptEvidenceEventHandoffPacketFinding>,
    pub boundary_flags: PubPunkInventoryBoundaryFlags,
    pub refs: PubPunkPublishReceiptEvidenceEventHandoffPacketRefs,
}

impl PubPunkPublishReceiptEvidenceEventHandoffPacketAssessment {
    pub fn blocking_findings(
        &self,
    ) -> impl Iterator<Item = &PubPunkPublishReceiptEvidenceEventHandoffPacketFinding> {
        self.findings
            .iter()
            .filter(|finding| finding.code.is_blocking())
    }

    pub fn has_blockers(&self) -> bool {
        self.blocking_findings().next().is_some()
    }
}

pub fn assess_pubpunk_publish_receipt_evidence_event_handoff_packet(
    packet: &PubPunkPublishReceiptEvidenceEventHandoffPacket,
) -> PubPunkPublishReceiptEvidenceEventHandoffPacketAssessment {
    let mut findings = Vec::new();

    push_publish_receipt_evidence_event_handoff_required_ref_finding(
        &mut findings,
        packet.module_id.as_str(),
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingModuleId,
        "module id is required",
    );
    if !packet.module_id.trim().is_empty() && packet.module_id != PUBPUNK_MODULE_ID {
        findings.push(
            PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::for_ref(
                PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::NonCanonicalModuleId,
                packet.module_id.clone(),
                "PubPunk publish receipt/evidence event handoff packets must use the canonical pubpunk module id",
            ),
        );
    }
    push_publish_receipt_evidence_event_handoff_required_ref_finding(
        &mut findings,
        packet.module_version_ref.as_str(),
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingModuleVersionRef,
        "module version ref is required",
    );
    push_publish_receipt_evidence_event_handoff_required_ref_finding(
        &mut findings,
        packet.contract_ref.as_str(),
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingContractRef,
        "contract ref is required",
    );
    push_publish_receipt_evidence_event_handoff_required_ref_finding(
        &mut findings,
        packet.run_ref.as_str(),
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingRunRef,
        "run ref is required",
    );
    push_publish_receipt_evidence_event_handoff_required_ref_finding(
        &mut findings,
        packet.project_ref.as_str(),
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingProjectRef,
        "project ref is required",
    );

    if !packet.workspace_policy.selected_for_first_slice() {
        findings.push(
            PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::for_ref(
                PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsupportedWorkspacePolicy,
                packet.workspace_policy.as_str(),
                "the first PubPunk publish receipt/evidence event handoff supports split explicit refs only",
            ),
        );
    }

    push_publish_receipt_evidence_event_handoff_required_ref_finding(
        &mut findings,
        packet.publishing_workspace_ref.as_str(),
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingPublishingWorkspaceRef,
        "publishing workspace ref is required",
    );
    if !packet.publishing_workspace_ref.trim().is_empty()
        && !is_safe_workspace_ref(&packet.publishing_workspace_ref)
    {
        findings.push(
            PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::for_ref(
                PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafePublishingWorkspaceRef,
                packet.publishing_workspace_ref.clone(),
                "publishing workspace ref must be an explicit safe logical or repo-relative ref",
            ),
        );
    }

    validate_publish_receipt_evidence_event_handoff_ref(
        &mut findings,
        packet.publish_operation_evidence_handoff_ref.as_str(),
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingPublishOperationEvidenceHandoffRef,
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafePublishOperationEvidenceHandoffRef,
        "publish operation evidence handoff ref is required",
        "publish operation evidence handoff ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_evidence_event_handoff_ref(
        &mut findings,
        packet.receipt_writer_result_ref.as_str(),
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingReceiptWriterResultRef,
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeReceiptWriterResultRef,
        "receipt writer result ref is required",
        "receipt writer result ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_evidence_event_handoff_ref(
        &mut findings,
        packet.operation_evidence_write_result_ref.as_str(),
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingOperationEvidenceWriteResultRef,
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeOperationEvidenceWriteResultRef,
        "operation evidence write result ref is required",
        "operation evidence write result ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_evidence_event_handoff_ref(
        &mut findings,
        packet.receipt_ref.as_str(),
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingReceiptRef,
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeReceiptRef,
        "receipt ref is required",
        "receipt ref must be an explicit repo-relative ref",
    );
    if !packet.receipt_ref.trim().is_empty()
        && is_safe_source_ref(&packet.receipt_ref)
        && !is_punk_runs_target_ref(&packet.receipt_ref)
    {
        findings.push(
            PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::for_ref(
                PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::ReceiptRefOutsidePunkRuns,
                packet.receipt_ref.clone(),
                "receipt ref must stay under .punk/runs for the event handoff",
            ),
        );
    }
    validate_publish_receipt_evidence_event_handoff_ref(
        &mut findings,
        packet.operation_evidence_ref.as_str(),
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingOperationEvidenceRef,
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeOperationEvidenceRef,
        "operation evidence ref is required",
        "operation evidence ref must be an explicit repo-relative ref",
    );
    if !packet.operation_evidence_ref.trim().is_empty()
        && is_safe_source_ref(&packet.operation_evidence_ref)
        && !is_punk_runs_target_ref(&packet.operation_evidence_ref)
    {
        findings.push(
            PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::for_ref(
                PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::OperationEvidenceRefOutsidePunkRuns,
                packet.operation_evidence_ref.clone(),
                "operation evidence ref must stay under .punk/runs for the event handoff",
            ),
        );
    }
    if !packet.receipt_ref.trim().is_empty()
        && !packet.operation_evidence_ref.trim().is_empty()
        && is_safe_source_ref(&packet.receipt_ref)
        && is_safe_source_ref(&packet.operation_evidence_ref)
        && packet.receipt_ref == packet.operation_evidence_ref
    {
        findings.push(
            PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::for_ref(
                PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::ReceiptEvidenceRefsCollapsed,
                packet.receipt_ref.clone(),
                "receipt and operation evidence refs must remain separate artifacts",
            ),
        );
    }
    validate_publish_receipt_evidence_event_handoff_ref(
        &mut findings,
        packet.event_log_ref.as_str(),
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingEventLogRef,
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeEventLogRef,
        "event log ref is required",
        "event log ref must be an explicit repo-relative ref",
    );
    if !packet.event_log_ref.trim().is_empty()
        && is_safe_source_ref(&packet.event_log_ref)
        && !is_punk_flow_event_log_ref(&packet.event_log_ref)
    {
        findings.push(
            PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::for_ref(
                PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::EventLogRefNotPunkFlowLog,
                packet.event_log_ref.clone(),
                "event log ref must point to .punk/events/flow.jsonl",
            ),
        );
    }
    validate_publish_receipt_evidence_event_handoff_ref(
        &mut findings,
        packet.event_source_ref.as_str(),
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingEventSourceRef,
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeEventSourceRef,
        "event source ref is required",
        "event source ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_evidence_event_handoff_ref(
        &mut findings,
        packet.event_correlation_ref.as_str(),
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingEventCorrelationRef,
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeEventCorrelationRef,
        "event correlation ref is required",
        "event correlation ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_evidence_event_handoff_ref(
        &mut findings,
        packet.adapter_invocation_receipt_ref.as_str(),
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingAdapterInvocationReceiptRef,
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeAdapterInvocationReceiptRef,
        "adapter invocation receipt ref is required",
        "adapter invocation receipt ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_evidence_event_handoff_ref(
        &mut findings,
        packet.payload_ref.as_str(),
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingPayloadRef,
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafePayloadRef,
        "payload ref is required",
        "payload ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_evidence_event_handoff_ref(
        &mut findings,
        packet.channel_ref.as_str(),
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingChannelRef,
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeChannelRef,
        "channel ref is required",
        "channel ref must be an explicit repo-relative ref",
    );
    validate_publish_receipt_evidence_event_handoff_ref(
        &mut findings,
        packet.connector_profile_ref.as_str(),
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingConnectorProfileRef,
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeConnectorProfileRef,
        "connector profile ref is required",
        "connector profile ref must be an explicit repo-relative ref",
    );

    if packet.instruction_refs.is_empty() {
        findings.push(PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::new(
            PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingInstructionRefs,
            "instruction refs are required for PubPunk publish receipt/evidence event handoff packets",
        ));
    }

    for required_ref in PUBPUNK_REQUIRED_INSTRUCTION_REFS {
        if !packet
            .instruction_refs
            .iter()
            .any(|instruction_ref| instruction_ref == required_ref)
        {
            findings.push(
                PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::for_ref(
                    PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingRequiredInstructionRef,
                    *required_ref,
                    "required PubPunk instruction ref is missing",
                ),
            );
        }
    }

    for instruction_ref in &packet.instruction_refs {
        if !is_safe_source_ref(instruction_ref) {
            findings.push(
                PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::for_ref(
                    PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeInstructionRef,
                    instruction_ref.clone(),
                    "instruction refs must be explicit repo-relative refs",
                ),
            );
        }
    }

    for source_ref in &packet.allowed_source_refs {
        if !is_safe_source_ref(source_ref) {
            findings.push(
                PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::for_ref(
                    PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeAllowedSourceRef,
                    source_ref.clone(),
                    "allowed source refs must be explicit repo-relative refs",
                ),
            );
        }
    }

    for (source_ref, code, message) in [
        (
            packet.publish_operation_evidence_handoff_ref.as_str(),
            PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::PublishOperationEvidenceHandoffRefNotAllowed,
            "publish operation evidence handoff ref must be included in allowed source refs",
        ),
        (
            packet.receipt_writer_result_ref.as_str(),
            PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::ReceiptWriterResultRefNotAllowed,
            "receipt writer result ref must be included in allowed source refs",
        ),
        (
            packet.operation_evidence_write_result_ref.as_str(),
            PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::OperationEvidenceWriteResultRefNotAllowed,
            "operation evidence write result ref must be included in allowed source refs",
        ),
        (
            packet.receipt_ref.as_str(),
            PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::ReceiptRefNotAllowed,
            "receipt ref must be included in allowed source refs",
        ),
        (
            packet.operation_evidence_ref.as_str(),
            PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::OperationEvidenceRefNotAllowed,
            "operation evidence ref must be included in allowed source refs",
        ),
        (
            packet.event_source_ref.as_str(),
            PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::EventSourceRefNotAllowed,
            "event source ref must be included in allowed source refs",
        ),
        (
            packet.event_correlation_ref.as_str(),
            PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::EventCorrelationRefNotAllowed,
            "event correlation ref must be included in allowed source refs",
        ),
        (
            packet.adapter_invocation_receipt_ref.as_str(),
            PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::AdapterInvocationReceiptRefNotAllowed,
            "adapter invocation receipt ref must be included in allowed source refs",
        ),
        (
            packet.payload_ref.as_str(),
            PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::PayloadRefNotAllowed,
            "payload ref must be included in allowed source refs",
        ),
        (
            packet.channel_ref.as_str(),
            PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::ChannelRefNotAllowed,
            "channel ref must be included in allowed source refs",
        ),
        (
            packet.connector_profile_ref.as_str(),
            PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::ConnectorProfileRefNotAllowed,
            "connector profile ref must be included in allowed source refs",
        ),
    ] {
        if !source_ref.trim().is_empty()
            && !packet
                .allowed_source_refs
                .iter()
                .any(|allowed_ref| allowed_ref == source_ref)
        {
            findings.push(
                PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::for_ref(
                    code,
                    source_ref,
                    message,
                ),
            );
        }
    }

    if !packet
        .granted_capabilities
        .contains(&PubPunkCapabilityGrant::RequestReceiptEvidenceEventHandoff)
    {
        findings.push(
            PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::new(
                PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingRequestReceiptEvidenceEventHandoffGrant,
                "request_receipt_evidence_event_handoff must be explicitly granted for this packet",
            ),
        );
    }
    for capability in &packet.granted_capabilities {
        if !capability.supported_by_side_effect_free_publish_receipt_evidence_event_handoff() {
            findings.push(
                PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::for_capability(*capability),
            );
        }
    }

    if packet.expected_receipt_fields.is_empty() {
        findings.push(
            PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::new(
                PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingExpectedReceiptFields,
                "expected receipt fields are required even though this packet does not append events",
            ),
        );
    }
    for required_field in [
        "side_effects",
        "host_validation",
        "adapter_invocation_receipt",
        "operation_evidence",
        "publication_receipt",
        "receipt_write_result",
        "operation_evidence_write_result",
        "receipt_evidence_event_handoff",
    ] {
        if !packet
            .expected_receipt_fields
            .iter()
            .any(|field| field == required_field)
        {
            findings.push(
                PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::for_ref(
                    PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingRequiredExpectedReceiptField,
                    required_field,
                    "publish receipt/evidence event handoff expectations must include host, receipt, operation-evidence, and event handoff coverage",
                ),
            );
        }
    }

    if packet.privacy_policy.allows_private_or_raw_payloads() {
        findings.push(PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::new(
            PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafePrivacyPolicy,
            "privacy policy must disallow raw/private payloads for this packet",
        ));
    }

    if let Some(token_cost_ref) = &packet.token_cost_ref {
        if !is_safe_source_ref(token_cost_ref) {
            findings.push(
                PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::for_ref(
                    PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeTokenCostRef,
                    token_cost_ref.clone(),
                    "token cost ref must be an explicit repo-relative ref when provided",
                ),
            );
        }
    }

    let status = if findings.iter().any(|finding| finding.code.is_blocking()) {
        PubPunkAssessmentStatus::Blocked
    } else {
        PubPunkAssessmentStatus::Ready
    };

    PubPunkPublishReceiptEvidenceEventHandoffPacketAssessment {
        schema_version: PUBPUNK_PUBLISH_RECEIPT_EVIDENCE_EVENT_HANDOFF_PACKET_SCHEMA_VERSION,
        status,
        authority: PubPunkAssessmentAuthority::Advisory,
        requested_operation: PubPunkPublishReceiptEvidenceEventHandoffOperation::PreparePublishReceiptEvidenceEventHandoff,
        findings,
        boundary_flags: PUBPUNK_INVENTORY_ASSESSMENT_BOUNDARY_FLAGS,
        refs: PubPunkPublishReceiptEvidenceEventHandoffPacketRefs {
            module_id: packet.module_id.clone(),
            module_version_ref: packet.module_version_ref.clone(),
            contract_ref: packet.contract_ref.clone(),
            run_ref: packet.run_ref.clone(),
            project_ref: packet.project_ref.clone(),
            workspace_policy: packet.workspace_policy,
            publishing_workspace_ref: packet.publishing_workspace_ref.clone(),
            publish_operation_evidence_handoff_ref: packet
                .publish_operation_evidence_handoff_ref
                .clone(),
            receipt_writer_result_ref: packet.receipt_writer_result_ref.clone(),
            operation_evidence_write_result_ref: packet
                .operation_evidence_write_result_ref
                .clone(),
            receipt_ref: packet.receipt_ref.clone(),
            operation_evidence_ref: packet.operation_evidence_ref.clone(),
            event_log_ref: packet.event_log_ref.clone(),
            event_source_ref: packet.event_source_ref.clone(),
            event_correlation_ref: packet.event_correlation_ref.clone(),
            adapter_invocation_receipt_ref: packet.adapter_invocation_receipt_ref.clone(),
            payload_ref: packet.payload_ref.clone(),
            channel_ref: packet.channel_ref.clone(),
            connector_profile_ref: packet.connector_profile_ref.clone(),
            token_cost_ref: packet.token_cost_ref.clone(),
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

fn push_input_packet_required_ref_finding(
    findings: &mut Vec<PubPunkInventoryInputPacketFinding>,
    value: &str,
    code: PubPunkInventoryInputPacketFindingCode,
    message: &'static str,
) {
    if value.trim().is_empty() {
        findings.push(PubPunkInventoryInputPacketFinding::new(code, message));
    }
}

fn push_reader_required_ref_finding(
    findings: &mut Vec<PubPunkInventoryReaderFinding>,
    value: &str,
    code: PubPunkInventoryReaderFindingCode,
    message: &'static str,
) {
    if value.trim().is_empty() {
        findings.push(PubPunkInventoryReaderFinding::new(code, message));
    }
}

fn push_connector_profile_resolution_required_ref_finding(
    findings: &mut Vec<PubPunkChannelConnectorProfileResolutionPacketFinding>,
    value: &str,
    code: PubPunkChannelConnectorProfileResolutionPacketFindingCode,
    message: &'static str,
) {
    if value.trim().is_empty() {
        findings.push(PubPunkChannelConnectorProfileResolutionPacketFinding::new(
            code, message,
        ));
    }
}

fn validate_connector_profile_resolution_ref(
    findings: &mut Vec<PubPunkChannelConnectorProfileResolutionPacketFinding>,
    value: &str,
    missing_code: PubPunkChannelConnectorProfileResolutionPacketFindingCode,
    unsafe_code: PubPunkChannelConnectorProfileResolutionPacketFindingCode,
    missing_message: &'static str,
    unsafe_message: &'static str,
) {
    if value.trim().is_empty() {
        findings.push(PubPunkChannelConnectorProfileResolutionPacketFinding::new(
            missing_code,
            missing_message,
        ));
    } else if !is_safe_source_ref(value) {
        findings.push(
            PubPunkChannelConnectorProfileResolutionPacketFinding::for_ref(
                unsafe_code,
                value,
                unsafe_message,
            ),
        );
    }
}

fn push_publish_request_required_ref_finding(
    findings: &mut Vec<PubPunkPublishRequestPacketFinding>,
    value: &str,
    code: PubPunkPublishRequestPacketFindingCode,
    message: &'static str,
) {
    if value.trim().is_empty() {
        findings.push(PubPunkPublishRequestPacketFinding::new(code, message));
    }
}

fn validate_publish_request_ref(
    findings: &mut Vec<PubPunkPublishRequestPacketFinding>,
    value: &str,
    missing_code: PubPunkPublishRequestPacketFindingCode,
    unsafe_code: PubPunkPublishRequestPacketFindingCode,
    missing_message: &'static str,
    unsafe_message: &'static str,
) {
    if value.trim().is_empty() {
        findings.push(PubPunkPublishRequestPacketFinding::new(
            missing_code,
            missing_message,
        ));
    } else if !is_safe_source_ref(value) {
        findings.push(PubPunkPublishRequestPacketFinding::for_ref(
            unsafe_code,
            value,
            unsafe_message,
        ));
    }
}

fn push_publish_receipt_preflight_required_ref_finding(
    findings: &mut Vec<PubPunkPublishReceiptPreflightPacketFinding>,
    value: &str,
    code: PubPunkPublishReceiptPreflightPacketFindingCode,
    message: &'static str,
) {
    if value.trim().is_empty() {
        findings.push(PubPunkPublishReceiptPreflightPacketFinding::new(
            code, message,
        ));
    }
}

fn validate_publish_receipt_preflight_ref(
    findings: &mut Vec<PubPunkPublishReceiptPreflightPacketFinding>,
    value: &str,
    missing_code: PubPunkPublishReceiptPreflightPacketFindingCode,
    unsafe_code: PubPunkPublishReceiptPreflightPacketFindingCode,
    missing_message: &'static str,
    unsafe_message: &'static str,
) {
    if value.trim().is_empty() {
        findings.push(PubPunkPublishReceiptPreflightPacketFinding::new(
            missing_code,
            missing_message,
        ));
    } else if !is_safe_source_ref(value) {
        findings.push(PubPunkPublishReceiptPreflightPacketFinding::for_ref(
            unsafe_code,
            value,
            unsafe_message,
        ));
    }
}

fn push_publish_receipt_write_handoff_required_ref_finding(
    findings: &mut Vec<PubPunkPublishReceiptWriteHandoffPacketFinding>,
    value: &str,
    code: PubPunkPublishReceiptWriteHandoffPacketFindingCode,
    message: &'static str,
) {
    if value.trim().is_empty() {
        findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::new(
            code, message,
        ));
    }
}

fn validate_publish_receipt_write_handoff_ref(
    findings: &mut Vec<PubPunkPublishReceiptWriteHandoffPacketFinding>,
    value: &str,
    missing_code: PubPunkPublishReceiptWriteHandoffPacketFindingCode,
    unsafe_code: PubPunkPublishReceiptWriteHandoffPacketFindingCode,
    missing_message: &'static str,
    unsafe_message: &'static str,
) {
    if value.trim().is_empty() {
        findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::new(
            missing_code,
            missing_message,
        ));
    } else if !is_safe_source_ref(value) {
        findings.push(PubPunkPublishReceiptWriteHandoffPacketFinding::for_ref(
            unsafe_code,
            value,
            unsafe_message,
        ));
    }
}

fn push_publish_operation_evidence_handoff_required_ref_finding(
    findings: &mut Vec<PubPunkPublishOperationEvidenceHandoffPacketFinding>,
    value: &str,
    code: PubPunkPublishOperationEvidenceHandoffPacketFindingCode,
    message: &'static str,
) {
    if value.trim().is_empty() {
        findings.push(PubPunkPublishOperationEvidenceHandoffPacketFinding::new(
            code, message,
        ));
    }
}

fn validate_publish_operation_evidence_handoff_ref(
    findings: &mut Vec<PubPunkPublishOperationEvidenceHandoffPacketFinding>,
    value: &str,
    missing_code: PubPunkPublishOperationEvidenceHandoffPacketFindingCode,
    unsafe_code: PubPunkPublishOperationEvidenceHandoffPacketFindingCode,
    missing_message: &'static str,
    unsafe_message: &'static str,
) {
    if value.trim().is_empty() {
        findings.push(PubPunkPublishOperationEvidenceHandoffPacketFinding::new(
            missing_code,
            missing_message,
        ));
    } else if !is_safe_source_ref(value) {
        findings.push(
            PubPunkPublishOperationEvidenceHandoffPacketFinding::for_ref(
                unsafe_code,
                value,
                unsafe_message,
            ),
        );
    }
}

fn push_publish_receipt_evidence_event_handoff_required_ref_finding(
    findings: &mut Vec<PubPunkPublishReceiptEvidenceEventHandoffPacketFinding>,
    value: &str,
    code: PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode,
    message: &'static str,
) {
    if value.trim().is_empty() {
        findings.push(PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::new(
            code, message,
        ));
    }
}

fn validate_publish_receipt_evidence_event_handoff_ref(
    findings: &mut Vec<PubPunkPublishReceiptEvidenceEventHandoffPacketFinding>,
    value: &str,
    missing_code: PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode,
    unsafe_code: PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode,
    missing_message: &'static str,
    unsafe_message: &'static str,
) {
    if value.trim().is_empty() {
        findings.push(PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::new(
            missing_code,
            missing_message,
        ));
    } else if !is_safe_source_ref(value) {
        findings.push(
            PubPunkPublishReceiptEvidenceEventHandoffPacketFinding::for_ref(
                unsafe_code,
                value,
                unsafe_message,
            ),
        );
    }
}

fn is_punk_runs_target_ref(value: &str) -> bool {
    value.trim().starts_with(".punk/runs/")
}

fn is_punk_flow_event_log_ref(value: &str) -> bool {
    value.trim() == ".punk/events/flow.jsonl"
}

fn is_safe_workspace_ref(value: &str) -> bool {
    let value = value.trim();
    if let Some(project_id) = value.strip_prefix("punk-publishing://project/") {
        return is_safe_logical_ref_segment(project_id);
    }

    is_safe_source_ref(value)
}

fn is_safe_logical_ref_segment(value: &str) -> bool {
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
        assess_pubpunk_channel_connector_profile_resolution_packet, assess_pubpunk_inventory,
        assess_pubpunk_inventory_input_packet, assess_pubpunk_inventory_reader_input,
        assess_pubpunk_publish_operation_evidence_handoff_packet,
        assess_pubpunk_publish_receipt_evidence_event_handoff_packet,
        assess_pubpunk_publish_receipt_preflight_packet,
        assess_pubpunk_publish_receipt_write_handoff_packet, assess_pubpunk_publish_request_packet,
        build_pubpunk_inventory_input_packet_from_reader_input, PubPunkAssessmentAuthority,
        PubPunkAssessmentStatus, PubPunkCapabilityGrant,
        PubPunkChannelConnectorProfileResolutionOperation,
        PubPunkChannelConnectorProfileResolutionPacket,
        PubPunkChannelConnectorProfileResolutionPacketFindingCode, PubPunkConnectorStrategy,
        PubPunkConnectorStrategySelectionReason, PubPunkInventoryFindingCode,
        PubPunkInventoryInput, PubPunkInventoryInputPacket, PubPunkInventoryInputPacketFindingCode,
        PubPunkInventoryItemInput, PubPunkInventoryItemKind, PubPunkInventoryItemStatus,
        PubPunkInventoryReaderFindingCode, PubPunkInventoryReaderInput, PubPunkPrivacyPolicy,
        PubPunkPublishOperationEvidenceHandoffOperation,
        PubPunkPublishOperationEvidenceHandoffPacket,
        PubPunkPublishOperationEvidenceHandoffPacketFindingCode,
        PubPunkPublishReceiptEvidenceEventHandoffOperation,
        PubPunkPublishReceiptEvidenceEventHandoffPacket,
        PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode,
        PubPunkPublishReceiptPreflightOperation, PubPunkPublishReceiptPreflightPacket,
        PubPunkPublishReceiptPreflightPacketFindingCode,
        PubPunkPublishReceiptWriteHandoffOperation, PubPunkPublishReceiptWriteHandoffPacket,
        PubPunkPublishReceiptWriteHandoffPacketFindingCode, PubPunkPublishRequestOperation,
        PubPunkPublishRequestPacket, PubPunkPublishRequestPacketFindingCode,
        PubPunkWorkspacePolicy, PUBPUNK_REQUIRED_INSTRUCTION_REFS,
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

    fn valid_packet() -> PubPunkInventoryInputPacket {
        PubPunkInventoryInputPacket::new(
            "v0.1",
            "contracts/publish-cycle-0",
            "runs/local-pubpunk-inventory",
            "project/punk",
            "punk-publishing://project/punk",
        )
        .with_instruction_refs(PUBPUNK_REQUIRED_INSTRUCTION_REFS.to_vec())
        .with_allowed_source_refs(vec!["publishing/posts/example.md"])
        .with_granted_capabilities(vec![PubPunkCapabilityGrant::AssessProvidedInventory])
        .with_expected_receipt_fields(vec![
            "module_id",
            "operation",
            "source_refs",
            "capability_grants",
            "side_effects",
        ])
        .with_token_cost_ref("work/reports/pubpunk-token-cost.md")
    }

    fn valid_reader_input() -> PubPunkInventoryReaderInput {
        PubPunkInventoryReaderInput::new(
            "v0.1",
            "contracts/publish-cycle-0",
            "runs/local-pubpunk-inventory",
            "project/punk",
            "punk-publishing://project/punk",
        )
        .with_instruction_refs(PUBPUNK_REQUIRED_INSTRUCTION_REFS.to_vec())
        .with_allowed_source_refs(vec![
            "publishing/posts/example.md",
            "publishing/publications/example.md",
            "publishing/metrics/example.md",
            "publishing/misc/unclear.md",
        ])
        .with_granted_capabilities(vec![PubPunkCapabilityGrant::ReadWorkspaceMetadata])
        .with_expected_receipt_fields(vec![
            "module_id",
            "operation",
            "source_refs",
            "capability_grants",
            "side_effects",
        ])
        .with_token_cost_ref("work/reports/pubpunk-token-cost.md")
    }

    fn valid_connector_profile_resolution_packet() -> PubPunkChannelConnectorProfileResolutionPacket
    {
        PubPunkChannelConnectorProfileResolutionPacket::new(
            "v0.1",
            "contracts/publish-cycle-0",
            "runs/local-pubpunk-connector-profile-resolution",
            "project/punk",
            "punk-publishing://project/punk",
        )
        .with_inventory_assessment_ref("work/module-assessments/pubpunk-inventory.md")
        .with_candidate_ref("publishing/posts/example.md")
        .with_channel_ref("publishing/channels/github-discussions.md")
        .with_connector_profile_ref("publishing/connectors/github-discussions.md")
        .with_api_availability_ref("publishing/connectors/github-discussions-api.md")
        .with_browser_automation_policy_ref(
            "publishing/connectors/github-discussions-browser-policy.md",
        )
        .with_manual_handoff_ref("publishing/connectors/github-discussions-manual-handoff.md")
        .with_credential_signal_ref("publishing/connectors/github-discussions-credential-signal.md")
        .with_payload_ref("publishing/posts/example.md")
        .with_api_available(true)
        .with_browser_allowed(true)
        .with_manual_allowed(true)
        .with_instruction_refs(PUBPUNK_REQUIRED_INSTRUCTION_REFS.to_vec())
        .with_allowed_source_refs(vec![
            "work/module-assessments/pubpunk-inventory.md",
            "publishing/posts/example.md",
            "publishing/channels/github-discussions.md",
            "publishing/connectors/github-discussions.md",
            "publishing/connectors/github-discussions-api.md",
            "publishing/connectors/github-discussions-browser-policy.md",
            "publishing/connectors/github-discussions-manual-handoff.md",
            "publishing/connectors/github-discussions-credential-signal.md",
        ])
        .with_granted_capabilities(vec![PubPunkCapabilityGrant::ResolveConnectorProfile])
        .with_expected_receipt_fields(vec![
            "side_effects",
            "host_validation",
            "connector_profile_resolution",
            "selected_connector_strategy",
            "channel_ref",
            "connector_profile_ref",
            "credential_signal_ref",
            "manual_fallback",
        ])
        .with_token_cost_ref("work/reports/pubpunk-connector-profile-resolution-token-cost.md")
    }

    fn valid_publish_request_packet() -> PubPunkPublishRequestPacket {
        PubPunkPublishRequestPacket::new(
            "v0.1",
            "contracts/publish-cycle-0",
            "runs/local-pubpunk-publish-request",
            "project/punk",
            "punk-publishing://project/punk",
        )
        .with_inventory_assessment_ref("work/module-assessments/pubpunk-inventory.md")
        .with_candidate_ref("publishing/posts/example.md")
        .with_channel_ref("publishing/channels/github-discussions.md")
        .with_connector_profile_resolution_ref(
            "work/module-assessments/pubpunk-connector-profile-resolution.md",
        )
        .with_connector_profile_ref("publishing/connectors/github-discussions.md")
        .with_selected_connector_strategy_ref(
            "work/module-assessments/pubpunk-selected-connector-strategy.md",
        )
        .with_side_effect_request_ref("work/module-side-effects/pubpunk-publish-example.md")
        .with_intent_ref("work/goals/goal_pubpunk_publish_example.md")
        .with_policy_ref("docs/modules/pubpunk.md")
        .with_adapter_ref("adapters/github-discussions")
        .with_payload_ref("publishing/posts/example.md")
        .with_receipt_proposal_ref("work/module-receipts/pubpunk-publish-example.md")
        .with_instruction_refs(PUBPUNK_REQUIRED_INSTRUCTION_REFS.to_vec())
        .with_allowed_source_refs(vec![
            "publishing/posts/example.md",
            "publishing/channels/github-discussions.md",
            "work/module-assessments/pubpunk-connector-profile-resolution.md",
            "publishing/connectors/github-discussions.md",
            "work/module-assessments/pubpunk-selected-connector-strategy.md",
        ])
        .with_granted_capabilities(vec![PubPunkCapabilityGrant::RequestExternalPublish])
        .with_expected_receipt_fields(vec![
            "module_id",
            "operation",
            "source_refs",
            "capability_grants",
            "side_effects",
            "host_validation",
            "connector_profile_resolution",
            "connector_profile_ref",
            "selected_connector_strategy",
        ])
        .with_token_cost_ref("work/reports/pubpunk-publish-request-token-cost.md")
    }

    fn valid_publish_receipt_preflight_packet() -> PubPunkPublishReceiptPreflightPacket {
        PubPunkPublishReceiptPreflightPacket::new(
            "v0.1",
            "contracts/publish-cycle-0",
            "runs/local-pubpunk-publish-request",
            "project/punk",
            "punk-publishing://project/punk",
        )
        .with_publish_request_ref("work/module-assessments/pubpunk-publish-request.md")
        .with_receipt_writer_preflight_ref(
            "work/module-receipt-preflights/pubpunk-publish-example.md",
        )
        .with_policy_gate_preflight_ref("work/module-policy-gate/pubpunk-publish-example.md")
        .with_receipt_target_ref("work/module-receipts/pubpunk-publish-example.md")
        .with_receipt_storage_ref(".punk/runs/local-pubpunk-publish-request/receipts")
        .with_operation_evidence_ref(
            ".punk/runs/local-pubpunk-publish-request/operation-evidence/pubpunk-publish-example.md",
        )
        .with_idempotency_ref("work/module-idempotency/pubpunk-publish-example.md")
        .with_rollback_ref("work/module-rollbacks/pubpunk-publish-example.md")
        .with_error_ref("work/module-errors/pubpunk-publish-example.md")
        .with_adapter_invocation_receipt_ref("work/module-receipts/github-discussions-invocation.md")
        .with_payload_ref("publishing/posts/example.md")
        .with_channel_ref("publishing/channels/github-discussions.md")
        .with_connector_profile_resolution_ref(
            "work/module-assessments/pubpunk-connector-profile-resolution.md",
        )
        .with_connector_profile_ref("publishing/connectors/github-discussions.md")
        .with_selected_connector_strategy_ref(
            "work/module-assessments/pubpunk-selected-connector-strategy.md",
        )
        .with_instruction_refs(PUBPUNK_REQUIRED_INSTRUCTION_REFS.to_vec())
        .with_allowed_source_refs(vec![
            "publishing/posts/example.md",
            "publishing/channels/github-discussions.md",
            "work/module-assessments/pubpunk-connector-profile-resolution.md",
            "publishing/connectors/github-discussions.md",
            "work/module-assessments/pubpunk-selected-connector-strategy.md",
        ])
        .with_granted_capabilities(vec![PubPunkCapabilityGrant::RequestExternalPublish])
        .with_expected_receipt_fields(vec![
            "module_id",
            "operation",
            "source_refs",
            "capability_grants",
            "side_effects",
            "host_validation",
            "connector_profile_resolution",
            "connector_profile_ref",
            "selected_connector_strategy",
            "adapter_invocation_receipt",
            "operation_evidence",
            "publication_receipt",
        ])
        .with_token_cost_ref("work/reports/pubpunk-publish-receipt-preflight-token-cost.md")
    }

    fn valid_publish_receipt_write_handoff_packet() -> PubPunkPublishReceiptWriteHandoffPacket {
        PubPunkPublishReceiptWriteHandoffPacket::new(
            "v0.1",
            "contracts/publish-cycle-0",
            "runs/local-pubpunk-publish-request",
            "project/punk",
            "punk-publishing://project/punk",
        )
        .with_publish_receipt_preflight_ref(
            "work/module-assessments/pubpunk-publish-receipt-preflight.md",
        )
        .with_receipt_writer_preflight_ref(
            "work/module-receipt-preflights/pubpunk-publish-example.md",
        )
        .with_receipt_writer_active_behavior_ref(
            "work/module-receipt-writer-active-behavior/pubpunk-publish-example.md",
        )
        .with_receipt_writer_file_io_plan_ref(
            "work/module-receipt-writer-file-io/pubpunk-publish-example.md",
        )
        .with_receipt_writer_target_storage_policy_ref(
            "work/module-receipt-writer-target-storage-policy/pubpunk-publish-example.md",
        )
        .with_receipt_writer_host_path_observation_ref(
            "work/module-receipt-writer-host-path-observation/pubpunk-publish-example.md",
        )
        .with_receipt_writer_concrete_path_storage_policy_ref(
            "work/module-receipt-writer-concrete-path-storage-policy/pubpunk-publish-example.md",
        )
        .with_operation_evidence_persistence_ref(
            "work/module-receipt-writer-operation-evidence-persistence/pubpunk-publish-example.md",
        )
        .with_receipt_target_ref("work/module-receipts/pubpunk-publish-example.md")
        .with_receipt_storage_ref(".punk/runs/local-pubpunk-publish-request")
        .with_receipt_target_path_ref(".punk/runs/local-pubpunk-publish-request/receipt.json")
        .with_receipt_bytes_ref("work/module-receipt-bytes/pubpunk-publish-example.json")
        .with_operation_evidence_ref(
            ".punk/runs/local-pubpunk-publish-request/operation-evidence/pubpunk-publish-example.md",
        )
        .with_idempotency_ref("work/module-idempotency/pubpunk-publish-example.md")
        .with_rollback_ref("work/module-rollbacks/pubpunk-publish-example.md")
        .with_error_ref("work/module-errors/pubpunk-publish-example.md")
        .with_adapter_invocation_receipt_ref("work/module-receipts/github-discussions-invocation.md")
        .with_payload_ref("publishing/posts/example.md")
        .with_channel_ref("publishing/channels/github-discussions.md")
        .with_connector_profile_resolution_ref(
            "work/module-assessments/pubpunk-connector-profile-resolution.md",
        )
        .with_connector_profile_ref("publishing/connectors/github-discussions.md")
        .with_selected_connector_strategy_ref(
            "work/module-assessments/pubpunk-selected-connector-strategy.md",
        )
        .with_instruction_refs(PUBPUNK_REQUIRED_INSTRUCTION_REFS.to_vec())
        .with_allowed_source_refs(vec![
            "publishing/posts/example.md",
            "publishing/channels/github-discussions.md",
            "work/module-assessments/pubpunk-connector-profile-resolution.md",
            "publishing/connectors/github-discussions.md",
            "work/module-assessments/pubpunk-selected-connector-strategy.md",
            "work/module-receipts/github-discussions-invocation.md",
            "work/module-receipt-bytes/pubpunk-publish-example.json",
        ])
        .with_granted_capabilities(vec![PubPunkCapabilityGrant::RequestPublicationReceiptWrite])
        .with_expected_receipt_fields(vec![
            "side_effects",
            "host_validation",
            "connector_profile_resolution",
            "connector_profile_ref",
            "selected_connector_strategy",
            "adapter_invocation_receipt",
            "operation_evidence",
            "publication_receipt",
            "receipt_bytes",
            "receipt_target_path",
            "receipt_write_result",
        ])
        .with_token_cost_ref("work/reports/pubpunk-publish-receipt-write-handoff-token-cost.md")
    }

    fn valid_publish_operation_evidence_handoff_packet(
    ) -> PubPunkPublishOperationEvidenceHandoffPacket {
        PubPunkPublishOperationEvidenceHandoffPacket::new(
            "v0.1",
            "contracts/publish-cycle-0",
            "runs/local-pubpunk-publish-request",
            "project/punk",
            "punk-publishing://project/punk",
        )
        .with_publish_receipt_write_handoff_ref(
            "work/module-assessments/pubpunk-publish-receipt-write-handoff.md",
        )
        .with_receipt_writer_result_ref(
            "work/module-receipt-writer-results/pubpunk-publish-example.md",
        )
        .with_receipt_storage_ref(".punk/runs/local-pubpunk-publish-request")
        .with_receipt_target_ref("work/module-receipts/pubpunk-publish-example.md")
        .with_receipt_target_path_ref(".punk/runs/local-pubpunk-publish-request/receipt.json")
        .with_receipt_bytes_ref("work/module-receipt-bytes/pubpunk-publish-example.json")
        .with_operation_evidence_ref(
            ".punk/runs/local-pubpunk-publish-request/operation-evidence.jsonl",
        )
        .with_operation_evidence_target_path_ref(
            ".punk/runs/local-pubpunk-publish-request/operation-evidence.jsonl",
        )
        .with_operation_evidence_bytes_ref(
            "work/module-operation-evidence-bytes/pubpunk-publish-example.jsonl",
        )
        .with_operation_evidence_write_result_ref(
            "work/module-operation-evidence-results/pubpunk-publish-example.md",
        )
        .with_idempotency_ref("work/module-idempotency/pubpunk-publish-example.md")
        .with_rollback_ref("work/module-rollbacks/pubpunk-publish-example.md")
        .with_error_ref("work/module-errors/pubpunk-publish-example.md")
        .with_adapter_invocation_receipt_ref(
            "work/module-receipts/github-discussions-invocation.md",
        )
        .with_payload_ref("publishing/posts/example.md")
        .with_channel_ref("publishing/channels/github-discussions.md")
        .with_connector_profile_resolution_ref(
            "work/module-assessments/pubpunk-connector-profile-resolution.md",
        )
        .with_connector_profile_ref("publishing/connectors/github-discussions.md")
        .with_selected_connector_strategy_ref(
            "work/module-assessments/pubpunk-selected-connector-strategy.md",
        )
        .with_instruction_refs(PUBPUNK_REQUIRED_INSTRUCTION_REFS.to_vec())
        .with_allowed_source_refs(vec![
            "work/module-assessments/pubpunk-publish-receipt-write-handoff.md",
            "work/module-receipt-writer-results/pubpunk-publish-example.md",
            "publishing/posts/example.md",
            "publishing/channels/github-discussions.md",
            "work/module-assessments/pubpunk-connector-profile-resolution.md",
            "publishing/connectors/github-discussions.md",
            "work/module-assessments/pubpunk-selected-connector-strategy.md",
            "work/module-receipts/github-discussions-invocation.md",
            "work/module-receipt-bytes/pubpunk-publish-example.json",
            "work/module-operation-evidence-bytes/pubpunk-publish-example.jsonl",
        ])
        .with_granted_capabilities(vec![PubPunkCapabilityGrant::RequestOperationEvidenceWrite])
        .with_expected_receipt_fields(vec![
            "side_effects",
            "host_validation",
            "connector_profile_resolution",
            "connector_profile_ref",
            "selected_connector_strategy",
            "adapter_invocation_receipt",
            "operation_evidence",
            "publication_receipt",
            "receipt_write_result",
            "operation_evidence_bytes",
            "operation_evidence_target_path",
            "operation_evidence_write_result",
        ])
        .with_token_cost_ref(
            "work/reports/pubpunk-publish-operation-evidence-handoff-token-cost.md",
        )
    }

    fn valid_publish_receipt_evidence_event_handoff_packet(
    ) -> PubPunkPublishReceiptEvidenceEventHandoffPacket {
        PubPunkPublishReceiptEvidenceEventHandoffPacket::new(
            "v0.1",
            "contracts/publish-cycle-0",
            "runs/local-pubpunk-publish-request",
            "project/punk",
            "punk-publishing://project/punk",
        )
        .with_publish_operation_evidence_handoff_ref(
            "work/module-assessments/pubpunk-publish-operation-evidence-handoff.md",
        )
        .with_receipt_writer_result_ref(
            "work/module-receipt-writer-results/pubpunk-publish-example.md",
        )
        .with_operation_evidence_write_result_ref(
            "work/module-operation-evidence-results/pubpunk-publish-example.md",
        )
        .with_receipt_ref(".punk/runs/local-pubpunk-publish-request/receipt.json")
        .with_operation_evidence_ref(
            ".punk/runs/local-pubpunk-publish-request/operation-evidence.jsonl",
        )
        .with_event_log_ref(".punk/events/flow.jsonl")
        .with_event_source_ref("work/module-events/pubpunk-receipt-evidence-source.md")
        .with_event_correlation_ref("work/module-events/pubpunk-receipt-evidence-correlation.md")
        .with_adapter_invocation_receipt_ref(
            "work/module-receipts/github-discussions-invocation.md",
        )
        .with_payload_ref("publishing/posts/example.md")
        .with_channel_ref("publishing/channels/github-discussions.md")
        .with_connector_profile_ref("publishing/connectors/github-discussions.md")
        .with_instruction_refs(PUBPUNK_REQUIRED_INSTRUCTION_REFS.to_vec())
        .with_allowed_source_refs(vec![
            "work/module-assessments/pubpunk-publish-operation-evidence-handoff.md",
            "work/module-receipt-writer-results/pubpunk-publish-example.md",
            "work/module-operation-evidence-results/pubpunk-publish-example.md",
            ".punk/runs/local-pubpunk-publish-request/receipt.json",
            ".punk/runs/local-pubpunk-publish-request/operation-evidence.jsonl",
            "work/module-events/pubpunk-receipt-evidence-source.md",
            "work/module-events/pubpunk-receipt-evidence-correlation.md",
            "publishing/posts/example.md",
            "publishing/channels/github-discussions.md",
            "publishing/connectors/github-discussions.md",
            "work/module-receipts/github-discussions-invocation.md",
        ])
        .with_granted_capabilities(vec![
            PubPunkCapabilityGrant::RequestReceiptEvidenceEventHandoff,
        ])
        .with_expected_receipt_fields(vec![
            "side_effects",
            "host_validation",
            "adapter_invocation_receipt",
            "operation_evidence",
            "publication_receipt",
            "receipt_write_result",
            "operation_evidence_write_result",
            "receipt_evidence_event_handoff",
        ])
        .with_token_cost_ref(
            "work/reports/pubpunk-publish-receipt-evidence-event-handoff-token-cost.md",
        )
    }

    #[test]
    fn inventory_reader_builds_packet_from_explicit_observed_refs() {
        let reader_input = valid_reader_input().with_observed_items(vec![
            PubPunkInventoryItemInput::new(
                "publishing/posts/example.md",
                PubPunkInventoryItemKind::PostDraft,
                PubPunkInventoryItemStatus::ReadyForReview,
            )
            .with_channel("github-discussions"),
            PubPunkInventoryItemInput::new(
                "publishing/publications/example.md",
                PubPunkInventoryItemKind::PublicationReceipt,
                PubPunkInventoryItemStatus::Published,
            )
            .with_publication_receipt(true),
            PubPunkInventoryItemInput::new(
                "publishing/metrics/example.md",
                PubPunkInventoryItemKind::MetricsSnapshot,
                PubPunkInventoryItemStatus::History,
            ),
            PubPunkInventoryItemInput::new(
                "publishing/misc/unclear.md",
                PubPunkInventoryItemKind::Other,
                PubPunkInventoryItemStatus::Unknown,
            ),
        ]);

        let reader_assessment = assess_pubpunk_inventory_reader_input(&reader_input);
        let packet = build_pubpunk_inventory_input_packet_from_reader_input(&reader_input)
            .expect("reader input should build packet");
        let packet_assessment = assess_pubpunk_inventory_input_packet(&packet);
        let inventory_input = packet
            .try_into_inventory_input()
            .expect("packet should convert");
        let inventory_assessment = assess_pubpunk_inventory(&inventory_input);

        assert_eq!(reader_assessment.status, PubPunkAssessmentStatus::Ready);
        assert_eq!(
            reader_assessment.authority,
            PubPunkAssessmentAuthority::Advisory
        );
        assert_eq!(reader_assessment.observed_item_count, 4);
        assert_eq!(reader_assessment.candidate_count, 1);
        assert_eq!(reader_assessment.receipt_count, 1);
        assert_eq!(reader_assessment.metrics_snapshot_count, 1);
        assert_eq!(reader_assessment.unclear_item_count, 1);
        assert!(reader_assessment
            .boundary_flags
            .all_side_effect_flags_false());
        assert_eq!(
            packet.granted_capabilities,
            vec![PubPunkCapabilityGrant::AssessProvidedInventory]
        );
        assert_eq!(packet.items.len(), 4);
        assert_eq!(packet_assessment.status, PubPunkAssessmentStatus::Ready);
        assert_eq!(inventory_assessment.status, PubPunkAssessmentStatus::Ready);
    }

    #[test]
    fn inventory_reader_allows_empty_new_project_inventory() {
        let reader_input = valid_reader_input()
            .with_allowed_source_refs(Vec::<String>::new())
            .with_observed_items(Vec::new());

        let reader_assessment = assess_pubpunk_inventory_reader_input(&reader_input);
        let packet = build_pubpunk_inventory_input_packet_from_reader_input(&reader_input)
            .expect("empty new-project inventory should be allowed");

        assert_eq!(reader_assessment.status, PubPunkAssessmentStatus::Ready);
        assert_eq!(reader_assessment.observed_item_count, 0);
        assert_eq!(reader_assessment.candidate_count, 0);
        assert!(reader_assessment
            .boundary_flags
            .all_side_effect_flags_false());
        assert!(packet.items.is_empty());
    }

    #[test]
    fn inventory_reader_blocks_unallowed_observed_refs_and_raw_bodies() {
        let reader_input = valid_reader_input()
            .with_allowed_source_refs(vec!["publishing/posts/other.md"])
            .with_privacy_policy(PubPunkPrivacyPolicy {
                raw_post_bodies: true,
                ..PubPunkPrivacyPolicy::safe_metadata_only()
            })
            .with_observed_items(vec![PubPunkInventoryItemInput::new(
                "publishing/posts/example.md",
                PubPunkInventoryItemKind::PostDraft,
                PubPunkInventoryItemStatus::Draft,
            )
            .with_raw_body_provided(true)]);

        let reader_assessment = assess_pubpunk_inventory_reader_input(&reader_input);

        assert_eq!(reader_assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(reader_assessment.findings.iter().any(|finding| finding.code
            == PubPunkInventoryReaderFindingCode::ObservedItemSourceRefNotAllowed));
        assert!(reader_assessment
            .findings
            .iter()
            .any(|finding| finding.code == PubPunkInventoryReaderFindingCode::UnsafePrivacyPolicy));
        assert!(reader_assessment
            .findings
            .iter()
            .any(|finding| finding.code == PubPunkInventoryReaderFindingCode::RawBodyProvided));
        assert!(build_pubpunk_inventory_input_packet_from_reader_input(&reader_input).is_err());
    }

    #[test]
    fn inventory_reader_blocks_unsupported_capabilities() {
        let reader_input = valid_reader_input().with_granted_capabilities(vec![
            PubPunkCapabilityGrant::ReadWorkspaceMetadata,
            PubPunkCapabilityGrant::RequestExternalPublish,
        ]);
        let missing_grant_input = valid_reader_input()
            .with_granted_capabilities(vec![PubPunkCapabilityGrant::AssessProvidedInventory]);

        let reader_assessment = assess_pubpunk_inventory_reader_input(&reader_input);
        let missing_grant_assessment = assess_pubpunk_inventory_reader_input(&missing_grant_input);

        assert_eq!(reader_assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(reader_assessment.findings.iter().any(|finding| finding.code
            == PubPunkInventoryReaderFindingCode::UnsupportedCapabilityGrant
            && finding.capability == Some(PubPunkCapabilityGrant::RequestExternalPublish)));
        assert_eq!(
            missing_grant_assessment.status,
            PubPunkAssessmentStatus::Blocked
        );
        assert!(missing_grant_assessment
            .findings
            .iter()
            .any(|finding| finding.code
                == PubPunkInventoryReaderFindingCode::MissingReadWorkspaceMetadataGrant));
    }

    #[test]
    fn inventory_reader_blocks_unsafe_refs() {
        let mut instruction_refs = PUBPUNK_REQUIRED_INSTRUCTION_REFS
            .iter()
            .map(|instruction_ref| (*instruction_ref).to_owned())
            .collect::<Vec<_>>();
        instruction_refs.push("/tmp/pubpunk-instruction.md".to_owned());

        let mut reader_input = valid_reader_input()
            .with_instruction_refs(instruction_refs)
            .with_allowed_source_refs(vec![
                "publishing/posts/example.md",
                "../publishing/posts/escape.md",
            ])
            .with_observed_items(vec![PubPunkInventoryItemInput::new(
                "../publishing/posts/escape.md",
                PubPunkInventoryItemKind::PostDraft,
                PubPunkInventoryItemStatus::Draft,
            )])
            .with_token_cost_ref("../work/reports/token-cost.md");
        reader_input.publishing_workspace_ref = "https://example.com/workspace".to_owned();

        let reader_assessment = assess_pubpunk_inventory_reader_input(&reader_input);

        assert_eq!(reader_assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(reader_assessment.findings.iter().any(|finding| finding.code
            == PubPunkInventoryReaderFindingCode::UnsafePublishingWorkspaceRef));
        assert!(
            reader_assessment
                .findings
                .iter()
                .any(|finding| finding.code
                    == PubPunkInventoryReaderFindingCode::UnsafeInstructionRef)
        );
        assert!(reader_assessment.findings.iter().any(
            |finding| finding.code == PubPunkInventoryReaderFindingCode::UnsafeAllowedSourceRef
        ));
        assert!(reader_assessment.findings.iter().any(|finding| finding.code
            == PubPunkInventoryReaderFindingCode::UnsafeObservedItemSourceRef));
        assert!(reader_assessment
            .findings
            .iter()
            .any(|finding| finding.code == PubPunkInventoryReaderFindingCode::UnsafeTokenCostRef));
    }

    #[test]
    fn connector_profile_resolution_selects_api_first_without_side_effects() {
        let packet = valid_connector_profile_resolution_packet();

        let assessment = assess_pubpunk_channel_connector_profile_resolution_packet(&packet);
        let refs = packet
            .try_into_connector_profile_resolution_refs()
            .expect("connector profile resolution packet should be ready");

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Ready);
        assert_eq!(assessment.authority, PubPunkAssessmentAuthority::Advisory);
        assert_eq!(
            assessment.requested_operation,
            PubPunkChannelConnectorProfileResolutionOperation::ResolveChannelConnectorProfile
        );
        assert_eq!(
            assessment.selected_strategy,
            Some(PubPunkConnectorStrategy::Api)
        );
        assert_eq!(
            assessment.selection_reason,
            PubPunkConnectorStrategySelectionReason::ApiPreferredAndAvailable
        );
        assert!(assessment.boundary_flags.all_side_effect_flags_false());
        assert_eq!(assessment.refs.module_id, "pubpunk");
        assert_eq!(
            assessment.refs.workspace_policy,
            PubPunkWorkspacePolicy::SplitExplicitRefs
        );
        assert_eq!(
            assessment.refs.connector_profile_ref,
            "publishing/connectors/github-discussions.md"
        );
        assert_eq!(
            assessment.refs.credential_signal_ref,
            "publishing/connectors/github-discussions-credential-signal.md"
        );
        assert_eq!(
            assessment.refs.token_cost_ref.as_deref(),
            Some("work/reports/pubpunk-connector-profile-resolution-token-cost.md")
        );
        assert_eq!(refs.selected_strategy, PubPunkConnectorStrategy::Api);
        assert_eq!(
            refs.connector_profile_ref,
            "publishing/connectors/github-discussions.md"
        );
    }

    #[test]
    fn connector_profile_resolution_falls_back_to_browser_then_manual() {
        let browser_packet = valid_connector_profile_resolution_packet().with_api_available(false);
        let manual_packet = valid_connector_profile_resolution_packet()
            .with_api_available(false)
            .with_browser_allowed(false);

        let browser_assessment =
            assess_pubpunk_channel_connector_profile_resolution_packet(&browser_packet);
        let manual_assessment =
            assess_pubpunk_channel_connector_profile_resolution_packet(&manual_packet);

        assert_eq!(browser_assessment.status, PubPunkAssessmentStatus::Ready);
        assert_eq!(
            browser_assessment.selected_strategy,
            Some(PubPunkConnectorStrategy::Browser)
        );
        assert_eq!(
            browser_assessment.selection_reason,
            PubPunkConnectorStrategySelectionReason::BrowserFallbackAllowed
        );
        assert!(browser_assessment
            .boundary_flags
            .all_side_effect_flags_false());
        assert_eq!(manual_assessment.status, PubPunkAssessmentStatus::Ready);
        assert_eq!(
            manual_assessment.selected_strategy,
            Some(PubPunkConnectorStrategy::Manual)
        );
        assert_eq!(
            manual_assessment.selection_reason,
            PubPunkConnectorStrategySelectionReason::ManualFallbackAllowed
        );
        assert!(manual_assessment
            .boundary_flags
            .all_side_effect_flags_false());
    }

    #[test]
    fn connector_profile_resolution_blocks_without_allowed_strategy() {
        let packet = valid_connector_profile_resolution_packet()
            .with_api_available(false)
            .with_browser_allowed(false)
            .with_manual_allowed(false);

        let assessment = assess_pubpunk_channel_connector_profile_resolution_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert_eq!(assessment.selected_strategy, None);
        assert_eq!(
            assessment.selection_reason,
            PubPunkConnectorStrategySelectionReason::BlockedNoAllowedStrategy
        );
        assert!(assessment
            .findings
            .iter()
            .any(|finding| finding.code
                == PubPunkChannelConnectorProfileResolutionPacketFindingCode::NoAllowedConnectorStrategy));
        assert!(packet.try_into_connector_profile_resolution_refs().is_err());
    }

    #[test]
    fn connector_profile_resolution_blocks_non_api_first_strategy_order() {
        let packet = valid_connector_profile_resolution_packet().with_strategy_order(vec![
            PubPunkConnectorStrategy::Manual,
            PubPunkConnectorStrategy::Browser,
            PubPunkConnectorStrategy::Api,
        ]);

        let assessment = assess_pubpunk_channel_connector_profile_resolution_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment
            .findings
            .iter()
            .any(|finding| finding.code
                == PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsupportedStrategyOrder));
    }

    #[test]
    fn connector_profile_resolution_blocks_missing_grant_and_fields() {
        let packet = valid_connector_profile_resolution_packet()
            .with_granted_capabilities(vec![PubPunkCapabilityGrant::RequestExternalPublish])
            .with_expected_receipt_fields(vec!["side_effects"]);

        let assessment = assess_pubpunk_channel_connector_profile_resolution_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingResolveConnectorProfileGrant));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsupportedCapabilityGrant
            && finding.capability == Some(PubPunkCapabilityGrant::RequestExternalPublish)));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::MissingRequiredExpectedReceiptField
            && finding.ref_value.as_deref() == Some("selected_connector_strategy")));
    }

    #[test]
    fn connector_profile_resolution_blocks_unallowed_refs() {
        let packet = valid_connector_profile_resolution_packet().with_allowed_source_refs(vec![
            "publishing/posts/example.md",
            "publishing/channels/github-discussions.md",
        ]);

        let assessment = assess_pubpunk_channel_connector_profile_resolution_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::InventoryAssessmentRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::ConnectorProfileRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::ApiAvailabilityRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::BrowserAutomationPolicyRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::ManualHandoffRefNotAllowed
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::CredentialSignalRefNotAllowed));
    }

    #[test]
    fn connector_profile_resolution_blocks_unsafe_refs_and_privacy() {
        let mut instruction_refs = PUBPUNK_REQUIRED_INSTRUCTION_REFS
            .iter()
            .map(|instruction_ref| (*instruction_ref).to_owned())
            .collect::<Vec<_>>();
        instruction_refs.push("/tmp/pubpunk-instruction.md".to_owned());

        let mut packet = valid_connector_profile_resolution_packet()
            .with_instruction_refs(instruction_refs)
            .with_candidate_ref("../publishing/posts/example.md")
            .with_channel_ref("https://example.com/channel")
            .with_connector_profile_ref("/tmp/connector-profile.md")
            .with_api_availability_ref("../publishing/connectors/api.md")
            .with_browser_automation_policy_ref("https://example.com/browser-policy")
            .with_manual_handoff_ref("../publishing/connectors/manual.md")
            .with_credential_signal_ref("/tmp/credential-signal.md")
            .with_payload_ref("../publishing/posts/example.md")
            .with_allowed_source_refs(vec!["../publishing/posts/example.md"])
            .with_privacy_policy(PubPunkPrivacyPolicy {
                secrets_or_credentials: true,
                ..PubPunkPrivacyPolicy::safe_metadata_only()
            })
            .with_token_cost_ref("../work/reports/token-cost.md");
        packet.publishing_workspace_ref = "https://example.com/workspace".to_owned();

        let assessment = assess_pubpunk_channel_connector_profile_resolution_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafePublishingWorkspaceRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeInstructionRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeAllowedSourceRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeCandidateRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeChannelRef));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeConnectorProfileRef
        }));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeApiAvailabilityRef
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeBrowserAutomationPolicyRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeManualHandoffRef));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeCredentialSignalRef
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafePayloadRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafePrivacyPolicy));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkChannelConnectorProfileResolutionPacketFindingCode::UnsafeTokenCostRef));
    }

    #[test]
    fn publish_request_packet_builds_side_effect_request_refs_without_side_effects() {
        let packet = valid_publish_request_packet();

        let assessment = assess_pubpunk_publish_request_packet(&packet);
        let side_effect_refs = packet
            .try_into_side_effect_request_refs()
            .expect("publish request packet should be ready");

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Ready);
        assert_eq!(assessment.authority, PubPunkAssessmentAuthority::Advisory);
        assert_eq!(
            assessment.requested_operation,
            PubPunkPublishRequestOperation::RequestExternalPublish
        );
        assert_eq!(assessment.refs.module_id, "pubpunk");
        assert_eq!(
            assessment.refs.workspace_policy,
            PubPunkWorkspacePolicy::SplitExplicitRefs
        );
        assert_eq!(
            assessment.refs.token_cost_ref.as_deref(),
            Some("work/reports/pubpunk-publish-request-token-cost.md")
        );
        assert_eq!(
            assessment.refs.connector_profile_resolution_ref,
            "work/module-assessments/pubpunk-connector-profile-resolution.md"
        );
        assert_eq!(
            assessment.refs.connector_profile_ref,
            "publishing/connectors/github-discussions.md"
        );
        assert_eq!(
            assessment.refs.selected_connector_strategy_ref,
            "work/module-assessments/pubpunk-selected-connector-strategy.md"
        );
        assert!(assessment.boundary_flags.all_side_effect_flags_false());
        assert_eq!(
            side_effect_refs.request_ref,
            "work/module-side-effects/pubpunk-publish-example.md"
        );
        assert_eq!(
            side_effect_refs.target_ref,
            "publishing/channels/github-discussions.md"
        );
        assert_eq!(
            side_effect_refs.connector_profile_resolution_ref,
            "work/module-assessments/pubpunk-connector-profile-resolution.md"
        );
        assert_eq!(
            side_effect_refs.connector_profile_ref,
            "publishing/connectors/github-discussions.md"
        );
        assert_eq!(
            side_effect_refs.selected_connector_strategy_ref,
            "work/module-assessments/pubpunk-selected-connector-strategy.md"
        );
        assert_eq!(side_effect_refs.policy_ref, "docs/modules/pubpunk.md");
        assert_eq!(side_effect_refs.adapter_ref, "adapters/github-discussions");
        assert_eq!(side_effect_refs.payload_ref, "publishing/posts/example.md");
        assert_eq!(
            side_effect_refs.receipt_proposal_ref,
            "work/module-receipts/pubpunk-publish-example.md"
        );
    }

    #[test]
    fn publish_request_packet_blocks_missing_publish_grant_and_receipt_fields() {
        let packet = valid_publish_request_packet()
            .with_granted_capabilities(vec![PubPunkCapabilityGrant::AssessProvidedInventory])
            .with_expected_receipt_fields(vec!["side_effects"]);

        let assessment = assess_pubpunk_publish_request_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishRequestPacketFindingCode::MissingRequestExternalPublishGrant));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishRequestPacketFindingCode::UnsupportedCapabilityGrant
            && finding.capability == Some(PubPunkCapabilityGrant::AssessProvidedInventory)));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishRequestPacketFindingCode::MissingRequiredExpectedReceiptField
            && finding.ref_value.as_deref() == Some("host_validation")));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishRequestPacketFindingCode::MissingRequiredExpectedReceiptField
            && finding.ref_value.as_deref() == Some("connector_profile_resolution")));
        assert!(packet.try_into_side_effect_request_refs().is_err());
    }

    #[test]
    fn publish_request_packet_blocks_unallowed_payload_candidate_and_connector_refs() {
        let packet = valid_publish_request_packet()
            .with_allowed_source_refs(vec!["publishing/channels/github-discussions.md"])
            .with_payload_ref("publishing/posts/not-allowed.md");

        let assessment = assess_pubpunk_publish_request_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishRequestPacketFindingCode::CandidateRefNotAllowed));
        assert!(assessment
            .findings
            .iter()
            .any(|finding| finding.code
                == PubPunkPublishRequestPacketFindingCode::PayloadRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishRequestPacketFindingCode::ConnectorProfileResolutionRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishRequestPacketFindingCode::ConnectorProfileRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishRequestPacketFindingCode::SelectedConnectorStrategyRefNotAllowed));
    }

    #[test]
    fn publish_request_packet_blocks_unsafe_refs_and_privacy() {
        let mut instruction_refs = PUBPUNK_REQUIRED_INSTRUCTION_REFS
            .iter()
            .map(|instruction_ref| (*instruction_ref).to_owned())
            .collect::<Vec<_>>();
        instruction_refs.push("/tmp/pubpunk-instruction.md".to_owned());

        let mut packet = valid_publish_request_packet()
            .with_instruction_refs(instruction_refs)
            .with_candidate_ref("../publishing/posts/example.md")
            .with_channel_ref("https://example.com/channel")
            .with_connector_profile_resolution_ref(
                "../work/module-assessments/pubpunk-connector-profile-resolution.md",
            )
            .with_connector_profile_ref("/tmp/github-discussions-connector.md")
            .with_selected_connector_strategy_ref(
                "../work/module-assessments/pubpunk-selected-connector-strategy.md",
            )
            .with_side_effect_request_ref("../work/module-side-effects/request.md")
            .with_policy_ref("/tmp/policy.md")
            .with_allowed_source_refs(vec!["../publishing/posts/example.md"])
            .with_privacy_policy(PubPunkPrivacyPolicy {
                raw_external_payloads: true,
                ..PubPunkPrivacyPolicy::safe_metadata_only()
            })
            .with_token_cost_ref("../work/reports/token-cost.md");
        packet.publishing_workspace_ref = "https://example.com/workspace".to_owned();

        let assessment = assess_pubpunk_publish_request_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishRequestPacketFindingCode::UnsafePublishingWorkspaceRef));
        assert!(assessment
            .findings
            .iter()
            .any(|finding| finding.code
                == PubPunkPublishRequestPacketFindingCode::UnsafeInstructionRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishRequestPacketFindingCode::UnsafeAllowedSourceRef));
        assert!(assessment
            .findings
            .iter()
            .any(|finding| finding.code
                == PubPunkPublishRequestPacketFindingCode::UnsafeCandidateRef));
        assert!(assessment.findings.iter().any(
            |finding| finding.code == PubPunkPublishRequestPacketFindingCode::UnsafeChannelRef
        ));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishRequestPacketFindingCode::UnsafeConnectorProfileResolutionRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishRequestPacketFindingCode::UnsafeConnectorProfileRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishRequestPacketFindingCode::UnsafeSelectedConnectorStrategyRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishRequestPacketFindingCode::UnsafeSideEffectRequestRef));
        assert!(
            assessment
                .findings
                .iter()
                .any(|finding| finding.code
                    == PubPunkPublishRequestPacketFindingCode::UnsafePolicyRef)
        );
        assert!(assessment
            .findings
            .iter()
            .any(|finding| finding.code
                == PubPunkPublishRequestPacketFindingCode::UnsafePrivacyPolicy));
        assert!(assessment
            .findings
            .iter()
            .any(|finding| finding.code
                == PubPunkPublishRequestPacketFindingCode::UnsafeTokenCostRef));
    }

    #[test]
    fn publish_receipt_preflight_packet_builds_writer_refs_without_side_effects() {
        let packet = valid_publish_receipt_preflight_packet();

        let assessment = assess_pubpunk_publish_receipt_preflight_packet(&packet);
        let writer_refs = packet
            .try_into_receipt_writer_preflight_refs()
            .expect("publish receipt preflight packet should be ready");

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Ready);
        assert_eq!(assessment.authority, PubPunkAssessmentAuthority::Advisory);
        assert_eq!(
            assessment.requested_operation,
            PubPunkPublishReceiptPreflightOperation::PreparePublishReceiptPreflight
        );
        assert_eq!(assessment.refs.module_id, "pubpunk");
        assert_eq!(
            assessment.refs.workspace_policy,
            PubPunkWorkspacePolicy::SplitExplicitRefs
        );
        assert_eq!(
            assessment.refs.publish_request_ref,
            "work/module-assessments/pubpunk-publish-request.md"
        );
        assert_eq!(
            assessment.refs.connector_profile_ref,
            "publishing/connectors/github-discussions.md"
        );
        assert_eq!(
            assessment.refs.connector_profile_resolution_ref,
            "work/module-assessments/pubpunk-connector-profile-resolution.md"
        );
        assert_eq!(
            assessment.refs.selected_connector_strategy_ref,
            "work/module-assessments/pubpunk-selected-connector-strategy.md"
        );
        assert_eq!(
            assessment.refs.token_cost_ref.as_deref(),
            Some("work/reports/pubpunk-publish-receipt-preflight-token-cost.md")
        );
        assert!(assessment.boundary_flags.all_side_effect_flags_false());
        assert_eq!(
            writer_refs.preflight_ref,
            "work/module-receipt-preflights/pubpunk-publish-example.md"
        );
        assert_eq!(
            writer_refs.policy_gate_preflight_ref,
            "work/module-policy-gate/pubpunk-publish-example.md"
        );
        assert_eq!(
            writer_refs.receipt_target_ref,
            "work/module-receipts/pubpunk-publish-example.md"
        );
        assert_eq!(
            writer_refs.storage_ref,
            ".punk/runs/local-pubpunk-publish-request/receipts"
        );
        assert_eq!(
            writer_refs.channel_ref,
            "publishing/channels/github-discussions.md"
        );
        assert_eq!(
            writer_refs.connector_profile_resolution_ref,
            "work/module-assessments/pubpunk-connector-profile-resolution.md"
        );
        assert_eq!(
            writer_refs.connector_profile_ref,
            "publishing/connectors/github-discussions.md"
        );
        assert_eq!(
            writer_refs.selected_connector_strategy_ref,
            "work/module-assessments/pubpunk-selected-connector-strategy.md"
        );
        assert_eq!(writer_refs.payload_ref, "publishing/posts/example.md");
    }

    #[test]
    fn publish_receipt_preflight_packet_blocks_missing_publish_grant_and_receipt_fields() {
        let packet = valid_publish_receipt_preflight_packet()
            .with_granted_capabilities(vec![PubPunkCapabilityGrant::AssessProvidedInventory])
            .with_expected_receipt_fields(vec!["side_effects"]);

        let assessment = assess_pubpunk_publish_receipt_preflight_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::MissingRequestExternalPublishGrant
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::UnsupportedCapabilityGrant
            && finding.capability == Some(PubPunkCapabilityGrant::AssessProvidedInventory)));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::MissingRequiredExpectedReceiptField
            && finding.ref_value.as_deref() == Some("host_validation")));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::MissingRequiredExpectedReceiptField
            && finding.ref_value.as_deref() == Some("connector_profile_resolution")));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::MissingRequiredExpectedReceiptField
            && finding.ref_value.as_deref() == Some("adapter_invocation_receipt")));
        assert!(packet.try_into_receipt_writer_preflight_refs().is_err());
    }

    #[test]
    fn publish_receipt_preflight_packet_blocks_unallowed_payload_channel_and_connector_refs() {
        let packet = valid_publish_receipt_preflight_packet()
            .with_allowed_source_refs(vec!["publishing/posts/other.md"])
            .with_channel_ref("publishing/channels/not-allowed.md")
            .with_connector_profile_resolution_ref(
                "work/module-assessments/pubpunk-connector-profile-resolution-not-allowed.md",
            )
            .with_selected_connector_strategy_ref(
                "work/module-assessments/pubpunk-selected-connector-strategy-not-allowed.md",
            )
            .with_connector_profile_ref("publishing/connectors/not-allowed.md");

        let assessment = assess_pubpunk_publish_receipt_preflight_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::PayloadRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::ChannelRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::ConnectorProfileResolutionRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::ConnectorProfileRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::SelectedConnectorStrategyRefNotAllowed));
    }

    #[test]
    fn publish_receipt_preflight_packet_blocks_unsafe_refs_and_privacy() {
        let mut instruction_refs = PUBPUNK_REQUIRED_INSTRUCTION_REFS
            .iter()
            .map(|instruction_ref| (*instruction_ref).to_owned())
            .collect::<Vec<_>>();
        instruction_refs.push("/tmp/pubpunk-instruction.md".to_owned());

        let mut packet = valid_publish_receipt_preflight_packet()
            .with_instruction_refs(instruction_refs)
            .with_publish_request_ref("../work/module-assessments/pubpunk-publish-request.md")
            .with_receipt_writer_preflight_ref("../work/module-receipt-preflights/request.md")
            .with_policy_gate_preflight_ref("https://example.com/policy-gate")
            .with_receipt_storage_ref("https://example.com/receipts")
            .with_operation_evidence_ref("../.punk/runs/evidence.md")
            .with_payload_ref("../publishing/posts/example.md")
            .with_channel_ref("/tmp/channel.md")
            .with_connector_profile_resolution_ref(
                "../work/module-assessments/pubpunk-connector-profile-resolution.md",
            )
            .with_connector_profile_ref("https://example.com/connector")
            .with_selected_connector_strategy_ref(
                "../work/module-assessments/pubpunk-selected-connector-strategy.md",
            )
            .with_allowed_source_refs(vec!["../publishing/posts/example.md"])
            .with_privacy_policy(PubPunkPrivacyPolicy {
                raw_external_payloads: true,
                ..PubPunkPrivacyPolicy::safe_metadata_only()
            })
            .with_token_cost_ref("../work/reports/token-cost.md");
        packet.publishing_workspace_ref = "https://example.com/workspace".to_owned();

        let assessment = assess_pubpunk_publish_receipt_preflight_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::UnsafePublishingWorkspaceRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeInstructionRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeAllowedSourceRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::UnsafePublishRequestRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeReceiptWriterPreflightRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::UnsafePolicyGatePreflightRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeReceiptStorageRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeOperationEvidenceRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::UnsafePayloadRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeChannelRef));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeConnectorProfileResolutionRef
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeConnectorProfileRef));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeSelectedConnectorStrategyRef
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::UnsafePrivacyPolicy));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptPreflightPacketFindingCode::UnsafeTokenCostRef));
    }

    #[test]
    fn publish_receipt_write_handoff_packet_builds_writer_refs_without_side_effects() {
        let packet = valid_publish_receipt_write_handoff_packet();

        let assessment = assess_pubpunk_publish_receipt_write_handoff_packet(&packet);
        let handoff_refs = packet
            .try_into_receipt_write_handoff_refs()
            .expect("publish receipt write handoff packet should be ready");

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Ready);
        assert_eq!(assessment.authority, PubPunkAssessmentAuthority::Advisory);
        assert_eq!(
            assessment.requested_operation,
            PubPunkPublishReceiptWriteHandoffOperation::PreparePublishReceiptWriteHandoff
        );
        assert_eq!(assessment.refs.module_id, "pubpunk");
        assert_eq!(
            assessment.refs.workspace_policy,
            PubPunkWorkspacePolicy::SplitExplicitRefs
        );
        assert_eq!(
            assessment.refs.publish_receipt_preflight_ref,
            "work/module-assessments/pubpunk-publish-receipt-preflight.md"
        );
        assert_eq!(
            assessment.refs.receipt_target_path_ref,
            ".punk/runs/local-pubpunk-publish-request/receipt.json"
        );
        assert_eq!(
            assessment.refs.receipt_bytes_ref,
            "work/module-receipt-bytes/pubpunk-publish-example.json"
        );
        assert_eq!(
            assessment.refs.connector_profile_resolution_ref,
            "work/module-assessments/pubpunk-connector-profile-resolution.md"
        );
        assert_eq!(
            assessment.refs.connector_profile_ref,
            "publishing/connectors/github-discussions.md"
        );
        assert_eq!(
            assessment.refs.selected_connector_strategy_ref,
            "work/module-assessments/pubpunk-selected-connector-strategy.md"
        );
        assert_eq!(
            assessment.refs.token_cost_ref.as_deref(),
            Some("work/reports/pubpunk-publish-receipt-write-handoff-token-cost.md")
        );
        assert!(assessment.boundary_flags.all_side_effect_flags_false());
        assert_eq!(
            handoff_refs.operation_evidence_persistence_ref,
            "work/module-receipt-writer-operation-evidence-persistence/pubpunk-publish-example.md"
        );
        assert_eq!(
            handoff_refs.receipt_target_path_ref,
            ".punk/runs/local-pubpunk-publish-request/receipt.json"
        );
        assert_eq!(
            handoff_refs.receipt_bytes_ref,
            "work/module-receipt-bytes/pubpunk-publish-example.json"
        );
        assert_eq!(
            handoff_refs.channel_ref,
            "publishing/channels/github-discussions.md"
        );
        assert_eq!(
            handoff_refs.connector_profile_resolution_ref,
            "work/module-assessments/pubpunk-connector-profile-resolution.md"
        );
        assert_eq!(
            handoff_refs.connector_profile_ref,
            "publishing/connectors/github-discussions.md"
        );
        assert_eq!(
            handoff_refs.selected_connector_strategy_ref,
            "work/module-assessments/pubpunk-selected-connector-strategy.md"
        );
        assert_eq!(handoff_refs.payload_ref, "publishing/posts/example.md");
    }

    #[test]
    fn publish_receipt_write_handoff_packet_blocks_missing_write_grant_and_receipt_fields() {
        let packet = valid_publish_receipt_write_handoff_packet()
            .with_granted_capabilities(vec![PubPunkCapabilityGrant::RequestExternalPublish])
            .with_expected_receipt_fields(vec!["side_effects"]);

        let assessment = assess_pubpunk_publish_receipt_write_handoff_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
                == PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingRequestPublicationReceiptWriteGrant
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsupportedCapabilityGrant
            && finding.capability == Some(PubPunkCapabilityGrant::RequestExternalPublish)));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingRequiredExpectedReceiptField
            && finding.ref_value.as_deref() == Some("receipt_bytes")));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingRequiredExpectedReceiptField
            && finding.ref_value.as_deref() == Some("connector_profile_resolution")));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::MissingRequiredExpectedReceiptField
            && finding.ref_value.as_deref() == Some("receipt_write_result")));
        assert!(packet.try_into_receipt_write_handoff_refs().is_err());
    }

    #[test]
    fn publish_receipt_write_handoff_packet_blocks_unallowed_source_refs() {
        let packet = valid_publish_receipt_write_handoff_packet()
            .with_allowed_source_refs(vec!["publishing/posts/other.md"])
            .with_channel_ref("publishing/channels/not-allowed.md")
            .with_connector_profile_resolution_ref(
                "work/module-assessments/pubpunk-connector-profile-resolution-not-allowed.md",
            )
            .with_connector_profile_ref("publishing/connectors/not-allowed.md")
            .with_selected_connector_strategy_ref(
                "work/module-assessments/pubpunk-selected-connector-strategy-not-allowed.md",
            )
            .with_adapter_invocation_receipt_ref("work/module-receipts/not-allowed-invocation.md")
            .with_receipt_bytes_ref("work/module-receipt-bytes/not-allowed.json");

        let assessment = assess_pubpunk_publish_receipt_write_handoff_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::PayloadRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::ChannelRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::ConnectorProfileResolutionRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::ConnectorProfileRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::SelectedConnectorStrategyRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::AdapterInvocationReceiptRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::ReceiptBytesRefNotAllowed));
    }

    #[test]
    fn publish_receipt_write_handoff_packet_blocks_unsafe_refs_and_privacy() {
        let mut instruction_refs = PUBPUNK_REQUIRED_INSTRUCTION_REFS
            .iter()
            .map(|instruction_ref| (*instruction_ref).to_owned())
            .collect::<Vec<_>>();
        instruction_refs.push("/tmp/pubpunk-instruction.md".to_owned());

        let mut packet = valid_publish_receipt_write_handoff_packet()
            .with_instruction_refs(instruction_refs)
            .with_publish_receipt_preflight_ref("../work/module-assessments/preflight.md")
            .with_receipt_writer_preflight_ref("../work/module-receipt-preflights/request.md")
            .with_receipt_writer_active_behavior_ref("https://example.com/active-behavior")
            .with_receipt_writer_file_io_plan_ref("../work/module-receipt-writer-file-io.md")
            .with_receipt_writer_target_storage_policy_ref("/tmp/target-storage-policy.md")
            .with_receipt_writer_host_path_observation_ref(
                "../work/module-host-path-observation.md",
            )
            .with_receipt_writer_concrete_path_storage_policy_ref(
                "https://example.com/concrete-path-storage-policy",
            )
            .with_operation_evidence_persistence_ref("../work/operation-evidence-persistence.md")
            .with_receipt_storage_ref("https://example.com/receipts")
            .with_receipt_target_path_ref("receipts/outside.json")
            .with_receipt_bytes_ref("../work/module-receipt-bytes/receipt.json")
            .with_operation_evidence_ref("../.punk/runs/evidence.md")
            .with_payload_ref("../publishing/posts/example.md")
            .with_channel_ref("/tmp/channel.md")
            .with_connector_profile_resolution_ref(
                "../work/module-assessments/pubpunk-connector-profile-resolution.md",
            )
            .with_connector_profile_ref("https://example.com/connector")
            .with_selected_connector_strategy_ref(
                "../work/module-assessments/pubpunk-selected-connector-strategy.md",
            )
            .with_allowed_source_refs(vec!["../publishing/posts/example.md"])
            .with_privacy_policy(PubPunkPrivacyPolicy {
                raw_external_payloads: true,
                ..PubPunkPrivacyPolicy::safe_metadata_only()
            })
            .with_token_cost_ref("../work/reports/token-cost.md");
        packet.publishing_workspace_ref = "https://example.com/workspace".to_owned();

        let assessment = assess_pubpunk_publish_receipt_write_handoff_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafePublishingWorkspaceRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeInstructionRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeAllowedSourceRef));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafePublishReceiptPreflightRef
        }));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeReceiptWriterPreflightRef
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeReceiptWriterActiveBehaviorRef));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeReceiptWriterFileIoPlanRef
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeReceiptWriterTargetStoragePolicyRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeReceiptWriterHostPathObservationRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeReceiptWriterConcretePathStoragePolicyRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeOperationEvidencePersistenceRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeReceiptStorageRef));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::ReceiptTargetPathOutsidePunkRuns
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeReceiptBytesRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeOperationEvidenceRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafePayloadRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeChannelRef));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
                == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeConnectorProfileResolutionRef
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeConnectorProfileRef));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
                == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeSelectedConnectorStrategyRef
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafePrivacyPolicy));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptWriteHandoffPacketFindingCode::UnsafeTokenCostRef));
    }

    #[test]
    fn publish_operation_evidence_handoff_packet_builds_writer_refs_without_side_effects() {
        let packet = valid_publish_operation_evidence_handoff_packet();

        let assessment = assess_pubpunk_publish_operation_evidence_handoff_packet(&packet);
        let handoff_refs = packet
            .try_into_operation_evidence_handoff_refs()
            .expect("publish operation evidence handoff packet should be ready");

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Ready);
        assert_eq!(assessment.authority, PubPunkAssessmentAuthority::Advisory);
        assert_eq!(
            assessment.requested_operation,
            PubPunkPublishOperationEvidenceHandoffOperation::PreparePublishOperationEvidenceHandoff
        );
        assert_eq!(assessment.refs.module_id, "pubpunk");
        assert_eq!(
            assessment.refs.workspace_policy,
            PubPunkWorkspacePolicy::SplitExplicitRefs
        );
        assert_eq!(
            assessment.refs.publish_receipt_write_handoff_ref,
            "work/module-assessments/pubpunk-publish-receipt-write-handoff.md"
        );
        assert_eq!(
            assessment.refs.operation_evidence_target_path_ref,
            ".punk/runs/local-pubpunk-publish-request/operation-evidence.jsonl"
        );
        assert_eq!(
            assessment.refs.operation_evidence_bytes_ref,
            "work/module-operation-evidence-bytes/pubpunk-publish-example.jsonl"
        );
        assert_eq!(
            assessment.refs.connector_profile_resolution_ref,
            "work/module-assessments/pubpunk-connector-profile-resolution.md"
        );
        assert_eq!(
            assessment.refs.connector_profile_ref,
            "publishing/connectors/github-discussions.md"
        );
        assert_eq!(
            assessment.refs.selected_connector_strategy_ref,
            "work/module-assessments/pubpunk-selected-connector-strategy.md"
        );
        assert_eq!(
            assessment.refs.token_cost_ref.as_deref(),
            Some("work/reports/pubpunk-publish-operation-evidence-handoff-token-cost.md")
        );
        assert!(assessment.boundary_flags.all_side_effect_flags_false());
        assert_eq!(
            handoff_refs.receipt_writer_result_ref,
            "work/module-receipt-writer-results/pubpunk-publish-example.md"
        );
        assert_eq!(
            handoff_refs.operation_evidence_ref,
            ".punk/runs/local-pubpunk-publish-request/operation-evidence.jsonl"
        );
        assert_eq!(
            handoff_refs.operation_evidence_target_path_ref,
            ".punk/runs/local-pubpunk-publish-request/operation-evidence.jsonl"
        );
        assert_eq!(
            handoff_refs.channel_ref,
            "publishing/channels/github-discussions.md"
        );
        assert_eq!(
            handoff_refs.connector_profile_resolution_ref,
            "work/module-assessments/pubpunk-connector-profile-resolution.md"
        );
        assert_eq!(
            handoff_refs.connector_profile_ref,
            "publishing/connectors/github-discussions.md"
        );
        assert_eq!(
            handoff_refs.selected_connector_strategy_ref,
            "work/module-assessments/pubpunk-selected-connector-strategy.md"
        );
        assert_eq!(handoff_refs.payload_ref, "publishing/posts/example.md");
    }

    #[test]
    fn publish_operation_evidence_handoff_packet_blocks_missing_write_grant_and_receipt_fields() {
        let packet = valid_publish_operation_evidence_handoff_packet()
            .with_granted_capabilities(vec![PubPunkCapabilityGrant::RequestPublicationReceiptWrite])
            .with_expected_receipt_fields(vec!["side_effects"]);

        let assessment = assess_pubpunk_publish_operation_evidence_handoff_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
                == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingRequestOperationEvidenceWriteGrant
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsupportedCapabilityGrant
            && finding.capability == Some(PubPunkCapabilityGrant::RequestPublicationReceiptWrite)));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingRequiredExpectedReceiptField
            && finding.ref_value.as_deref() == Some("operation_evidence_bytes")));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingRequiredExpectedReceiptField
            && finding.ref_value.as_deref() == Some("connector_profile_resolution")));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::MissingRequiredExpectedReceiptField
            && finding.ref_value.as_deref() == Some("operation_evidence_write_result")));
        assert!(packet.try_into_operation_evidence_handoff_refs().is_err());
    }

    #[test]
    fn publish_operation_evidence_handoff_packet_blocks_unallowed_source_refs() {
        let packet = valid_publish_operation_evidence_handoff_packet()
            .with_allowed_source_refs(vec!["publishing/posts/other.md"])
            .with_publish_receipt_write_handoff_ref("work/module-assessments/not-allowed.md")
            .with_receipt_writer_result_ref("work/module-receipt-writer-results/not-allowed.md")
            .with_channel_ref("publishing/channels/not-allowed.md")
            .with_connector_profile_resolution_ref(
                "work/module-assessments/pubpunk-connector-profile-resolution-not-allowed.md",
            )
            .with_connector_profile_ref("publishing/connectors/not-allowed.md")
            .with_selected_connector_strategy_ref(
                "work/module-assessments/pubpunk-selected-connector-strategy-not-allowed.md",
            )
            .with_adapter_invocation_receipt_ref("work/module-receipts/not-allowed-invocation.md")
            .with_receipt_bytes_ref("work/module-receipt-bytes/not-allowed.json")
            .with_operation_evidence_bytes_ref(
                "work/module-operation-evidence-bytes/not-allowed.jsonl",
            );

        let assessment = assess_pubpunk_publish_operation_evidence_handoff_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::PublishReceiptWriteHandoffRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::ReceiptWriterResultRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::PayloadRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::ChannelRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::ConnectorProfileResolutionRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::ConnectorProfileRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::SelectedConnectorStrategyRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::AdapterInvocationReceiptRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::ReceiptBytesRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::OperationEvidenceBytesRefNotAllowed));
    }

    #[test]
    fn publish_operation_evidence_handoff_packet_blocks_unsafe_refs_and_privacy() {
        let mut instruction_refs = PUBPUNK_REQUIRED_INSTRUCTION_REFS
            .iter()
            .map(|instruction_ref| (*instruction_ref).to_owned())
            .collect::<Vec<_>>();
        instruction_refs.push("/tmp/pubpunk-instruction.md".to_owned());

        let mut packet = valid_publish_operation_evidence_handoff_packet()
            .with_instruction_refs(instruction_refs)
            .with_publish_receipt_write_handoff_ref("../work/module-assessments/write-handoff.md")
            .with_receipt_writer_result_ref("../work/module-receipt-writer-results/result.md")
            .with_receipt_storage_ref("https://example.com/receipts")
            .with_receipt_target_ref("/tmp/receipt.md")
            .with_receipt_target_path_ref("receipts/outside.json")
            .with_receipt_bytes_ref("../work/module-receipt-bytes/receipt.json")
            .with_operation_evidence_ref(
                ".punk/runs/local-pubpunk-publish-request/expected-evidence.jsonl",
            )
            .with_operation_evidence_target_path_ref("operation-evidence/outside.jsonl")
            .with_operation_evidence_bytes_ref(
                "../work/module-operation-evidence-bytes/evidence.jsonl",
            )
            .with_operation_evidence_write_result_ref(
                "../work/module-operation-evidence-results/result.md",
            )
            .with_idempotency_ref("../work/idempotency.md")
            .with_rollback_ref("/tmp/rollback.md")
            .with_error_ref("https://example.com/error")
            .with_adapter_invocation_receipt_ref("../work/module-receipts/invocation.md")
            .with_payload_ref("../publishing/posts/example.md")
            .with_channel_ref("/tmp/channel.md")
            .with_connector_profile_resolution_ref(
                "../work/module-assessments/pubpunk-connector-profile-resolution.md",
            )
            .with_connector_profile_ref("https://example.com/connector")
            .with_selected_connector_strategy_ref(
                "../work/module-assessments/pubpunk-selected-connector-strategy.md",
            )
            .with_allowed_source_refs(vec!["../publishing/posts/example.md"])
            .with_privacy_policy(PubPunkPrivacyPolicy {
                raw_external_payloads: true,
                ..PubPunkPrivacyPolicy::safe_metadata_only()
            })
            .with_token_cost_ref("../work/reports/token-cost.md");
        packet.publishing_workspace_ref = "https://example.com/workspace".to_owned();

        let assessment = assess_pubpunk_publish_operation_evidence_handoff_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafePublishingWorkspaceRef
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeInstructionRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeAllowedSourceRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafePublishReceiptWriteHandoffRef));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeReceiptWriterResultRef
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeReceiptStorageRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeReceiptTargetRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::ReceiptTargetPathOutsidePunkRuns));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeReceiptBytesRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::OperationEvidenceTargetPathOutsidePunkRuns));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::OperationEvidenceTargetPathMismatch));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeOperationEvidenceBytesRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeOperationEvidenceWriteResultRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeIdempotencyRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeRollbackRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeErrorRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeAdapterInvocationReceiptRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafePayloadRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeChannelRef));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeConnectorProfileResolutionRef
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeConnectorProfileRef));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeSelectedConnectorStrategyRef
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafePrivacyPolicy));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishOperationEvidenceHandoffPacketFindingCode::UnsafeTokenCostRef));
    }

    #[test]
    fn publish_receipt_evidence_event_handoff_packet_builds_event_refs_without_side_effects() {
        let packet = valid_publish_receipt_evidence_event_handoff_packet();

        let assessment = assess_pubpunk_publish_receipt_evidence_event_handoff_packet(&packet);
        let handoff_refs = packet
            .try_into_receipt_evidence_event_handoff_refs()
            .expect("publish receipt/evidence event handoff packet should be ready");

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Ready);
        assert_eq!(assessment.authority, PubPunkAssessmentAuthority::Advisory);
        assert_eq!(
            assessment.requested_operation,
            PubPunkPublishReceiptEvidenceEventHandoffOperation::PreparePublishReceiptEvidenceEventHandoff
        );
        assert_eq!(assessment.refs.module_id, "pubpunk");
        assert_eq!(
            assessment.refs.workspace_policy,
            PubPunkWorkspacePolicy::SplitExplicitRefs
        );
        assert_eq!(
            assessment.refs.publish_operation_evidence_handoff_ref,
            "work/module-assessments/pubpunk-publish-operation-evidence-handoff.md"
        );
        assert_eq!(assessment.refs.event_log_ref, ".punk/events/flow.jsonl");
        assert_eq!(
            assessment.refs.token_cost_ref.as_deref(),
            Some("work/reports/pubpunk-publish-receipt-evidence-event-handoff-token-cost.md")
        );
        assert!(assessment.boundary_flags.all_side_effect_flags_false());
        assert_eq!(
            handoff_refs.receipt_ref,
            ".punk/runs/local-pubpunk-publish-request/receipt.json"
        );
        assert_eq!(
            handoff_refs.operation_evidence_ref,
            ".punk/runs/local-pubpunk-publish-request/operation-evidence.jsonl"
        );
        assert_eq!(handoff_refs.event_log_ref, ".punk/events/flow.jsonl");
        assert_eq!(handoff_refs.payload_ref, "publishing/posts/example.md");
    }

    #[test]
    fn publish_receipt_evidence_event_handoff_packet_blocks_missing_grant_and_fields() {
        let packet = valid_publish_receipt_evidence_event_handoff_packet()
            .with_granted_capabilities(vec![PubPunkCapabilityGrant::RequestOperationEvidenceWrite])
            .with_expected_receipt_fields(vec!["side_effects"]);

        let assessment = assess_pubpunk_publish_receipt_evidence_event_handoff_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
                == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingRequestReceiptEvidenceEventHandoffGrant
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsupportedCapabilityGrant
            && finding.capability == Some(PubPunkCapabilityGrant::RequestOperationEvidenceWrite)));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingRequiredExpectedReceiptField
            && finding.ref_value.as_deref() == Some("operation_evidence_write_result")));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::MissingRequiredExpectedReceiptField
            && finding.ref_value.as_deref() == Some("receipt_evidence_event_handoff")));
        assert!(packet
            .try_into_receipt_evidence_event_handoff_refs()
            .is_err());
    }

    #[test]
    fn publish_receipt_evidence_event_handoff_packet_blocks_unallowed_source_refs() {
        let packet = valid_publish_receipt_evidence_event_handoff_packet()
            .with_allowed_source_refs(vec!["publishing/posts/other.md"])
            .with_publish_operation_evidence_handoff_ref("work/module-assessments/not-allowed.md")
            .with_receipt_writer_result_ref("work/module-receipt-writer-results/not-allowed.md")
            .with_operation_evidence_write_result_ref(
                "work/module-operation-evidence-results/not-allowed.md",
            )
            .with_receipt_ref(".punk/runs/not-allowed/receipt.json")
            .with_operation_evidence_ref(".punk/runs/not-allowed/operation-evidence.jsonl")
            .with_event_source_ref("work/module-events/not-allowed-source.md")
            .with_event_correlation_ref("work/module-events/not-allowed-correlation.md")
            .with_channel_ref("publishing/channels/not-allowed.md")
            .with_connector_profile_ref("publishing/connectors/not-allowed.md")
            .with_adapter_invocation_receipt_ref("work/module-receipts/not-allowed.md");

        let assessment = assess_pubpunk_publish_receipt_evidence_event_handoff_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::PublishOperationEvidenceHandoffRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::ReceiptWriterResultRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::OperationEvidenceWriteResultRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::ReceiptRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::OperationEvidenceRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::EventSourceRefNotAllowed
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::EventCorrelationRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::PayloadRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::ChannelRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::ConnectorProfileRefNotAllowed));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::AdapterInvocationReceiptRefNotAllowed));
    }

    #[test]
    fn publish_receipt_evidence_event_handoff_packet_blocks_unsafe_refs_and_privacy() {
        let mut instruction_refs = PUBPUNK_REQUIRED_INSTRUCTION_REFS
            .iter()
            .map(|instruction_ref| (*instruction_ref).to_owned())
            .collect::<Vec<_>>();
        instruction_refs.push("/tmp/pubpunk-instruction.md".to_owned());

        let mut packet = valid_publish_receipt_evidence_event_handoff_packet()
            .with_instruction_refs(instruction_refs)
            .with_publish_operation_evidence_handoff_ref("../work/module-assessments/handoff.md")
            .with_receipt_writer_result_ref("../work/module-receipt-writer-results/result.md")
            .with_operation_evidence_write_result_ref(
                "../work/module-operation-evidence-results/result.md",
            )
            .with_receipt_ref("receipts/outside.json")
            .with_operation_evidence_ref("operation-evidence/outside.jsonl")
            .with_event_log_ref("work/events/flow.jsonl")
            .with_event_source_ref("../work/module-events/source.md")
            .with_event_correlation_ref("/tmp/correlation.md")
            .with_adapter_invocation_receipt_ref("../work/module-receipts/invocation.md")
            .with_payload_ref("../publishing/posts/example.md")
            .with_channel_ref("/tmp/channel.md")
            .with_connector_profile_ref("https://example.com/connector")
            .with_allowed_source_refs(vec!["../publishing/posts/example.md"])
            .with_privacy_policy(PubPunkPrivacyPolicy {
                raw_external_payloads: true,
                ..PubPunkPrivacyPolicy::safe_metadata_only()
            })
            .with_token_cost_ref("../work/reports/token-cost.md");
        packet.publishing_workspace_ref = "https://example.com/workspace".to_owned();

        let assessment = assess_pubpunk_publish_receipt_evidence_event_handoff_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
                == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafePublishingWorkspaceRef
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeInstructionRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeAllowedSourceRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafePublishOperationEvidenceHandoffRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeReceiptWriterResultRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeOperationEvidenceWriteResultRef));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::ReceiptRefOutsidePunkRuns
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::OperationEvidenceRefOutsidePunkRuns));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::EventLogRefNotPunkFlowLog
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeEventSourceRef));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeEventCorrelationRef
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeAdapterInvocationReceiptRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafePayloadRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeChannelRef));
        assert!(assessment.findings.iter().any(|finding| {
            finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeConnectorProfileRef
        }));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafePrivacyPolicy));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::UnsafeTokenCostRef));
    }

    #[test]
    fn publish_receipt_evidence_event_handoff_packet_blocks_collapsed_artifact_refs() {
        let packet = valid_publish_receipt_evidence_event_handoff_packet()
            .with_operation_evidence_ref(".punk/runs/local-pubpunk-publish-request/receipt.json");

        let assessment = assess_pubpunk_publish_receipt_evidence_event_handoff_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkPublishReceiptEvidenceEventHandoffPacketFindingCode::ReceiptEvidenceRefsCollapsed));
        assert!(packet
            .try_into_receipt_evidence_event_handoff_refs()
            .is_err());
    }

    #[test]
    fn input_packet_accepts_explicit_workspace_and_instruction_refs() {
        let packet = valid_packet().with_items(vec![PubPunkInventoryItemInput::new(
            "publishing/posts/example.md",
            PubPunkInventoryItemKind::PostDraft,
            PubPunkInventoryItemStatus::ReadyForReview,
        )]);

        let assessment = assess_pubpunk_inventory_input_packet(&packet);
        let inventory_input = packet.try_into_inventory_input().expect("packet is ready");
        let inventory_assessment = assess_pubpunk_inventory(&inventory_input);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Ready);
        assert_eq!(assessment.authority, PubPunkAssessmentAuthority::Advisory);
        assert_eq!(assessment.refs.module_id, "pubpunk");
        assert_eq!(
            assessment.refs.workspace_policy,
            PubPunkWorkspacePolicy::SplitExplicitRefs
        );
        assert_eq!(
            assessment.refs.token_cost_ref.as_deref(),
            Some("work/reports/pubpunk-token-cost.md")
        );
        assert!(assessment.boundary_flags.all_side_effect_flags_false());
        assert_eq!(inventory_input.module_id, "pubpunk");
        assert_eq!(
            inventory_input.publishing_workspace_ref,
            "punk-publishing://project/punk"
        );
        assert_eq!(inventory_assessment.status, PubPunkAssessmentStatus::Ready);
    }

    #[test]
    fn input_packet_blocks_missing_required_instruction_refs() {
        let packet = valid_packet().with_instruction_refs(vec!["docs/modules/pubpunk.md"]);

        let assessment = assess_pubpunk_inventory_input_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkInventoryInputPacketFindingCode::MissingRequiredInstructionRef
            && finding.ref_value.as_deref()
                == Some("docs/modules/pubpunk-workspace-instructions.md")));
        assert!(packet.try_into_inventory_input().is_err());
    }

    #[test]
    fn input_packet_blocks_unselected_workspace_policy_and_capability() {
        let packet = valid_packet()
            .with_workspace_policy(PubPunkWorkspacePolicy::ProjectPunk)
            .with_granted_capabilities(vec![PubPunkCapabilityGrant::RequestExternalPublish]);

        let assessment = assess_pubpunk_inventory_input_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkInventoryInputPacketFindingCode::UnsupportedWorkspacePolicy));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkInventoryInputPacketFindingCode::MissingAssessProvidedInventoryGrant));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkInventoryInputPacketFindingCode::UnsupportedCapabilityGrant
            && finding.capability == Some(PubPunkCapabilityGrant::RequestExternalPublish)));
    }

    #[test]
    fn input_packet_blocks_unallowed_item_refs_and_raw_bodies() {
        let packet = valid_packet()
            .with_allowed_source_refs(vec!["publishing/posts/other.md"])
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

        let assessment = assess_pubpunk_inventory_input_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment
            .findings
            .iter()
            .any(|finding| finding.code
                == PubPunkInventoryInputPacketFindingCode::UnsafePrivacyPolicy));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkInventoryInputPacketFindingCode::ItemSourceRefNotAllowed));
        assert!(
            assessment
                .findings
                .iter()
                .any(|finding| finding.code
                    == PubPunkInventoryInputPacketFindingCode::RawBodyProvided)
        );
    }

    #[test]
    fn input_packet_blocks_unsafe_refs() {
        let mut instruction_refs = PUBPUNK_REQUIRED_INSTRUCTION_REFS
            .iter()
            .map(|instruction_ref| (*instruction_ref).to_owned())
            .collect::<Vec<_>>();
        instruction_refs.push("/tmp/pubpunk-instruction.md".to_owned());

        let mut packet = valid_packet()
            .with_instruction_refs(instruction_refs)
            .with_allowed_source_refs(vec![
                "publishing/posts/example.md",
                "../publishing/posts/escape.md",
            ])
            .with_items(vec![PubPunkInventoryItemInput::new(
                "../publishing/posts/escape.md",
                PubPunkInventoryItemKind::PostDraft,
                PubPunkInventoryItemStatus::Draft,
            )])
            .with_token_cost_ref("../work/reports/token-cost.md");
        packet.publishing_workspace_ref = "https://example.com/workspace".to_owned();

        let assessment = assess_pubpunk_inventory_input_packet(&packet);

        assert_eq!(assessment.status, PubPunkAssessmentStatus::Blocked);
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkInventoryInputPacketFindingCode::UnsafePublishingWorkspaceRef));
        assert!(assessment
            .findings
            .iter()
            .any(|finding| finding.code
                == PubPunkInventoryInputPacketFindingCode::UnsafeInstructionRef));
        assert!(assessment.findings.iter().any(|finding| finding.code
            == PubPunkInventoryInputPacketFindingCode::UnsafeAllowedSourceRef));
        assert!(assessment
            .findings
            .iter()
            .any(|finding| finding.code
                == PubPunkInventoryInputPacketFindingCode::UnsafeItemSourceRef));
        assert!(assessment
            .findings
            .iter()
            .any(|finding| finding.code
                == PubPunkInventoryInputPacketFindingCode::UnsafeTokenCostRef));
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
