//! Incubating side-effect-free PubPunk module models.
//!
//! This crate models a publication inventory input packet and assessment from
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
pub const PUBPUNK_PUBLISH_REQUEST_PACKET_SCHEMA_VERSION: &str =
    "punk.pubpunk.publish_request_packet.v0.1";
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

    pub fn supported_by_side_effect_free_reader(self) -> bool {
        matches!(self, Self::ReadWorkspaceMetadata)
    }

    pub fn supported_by_side_effect_free_publish_request(self) -> bool {
        matches!(self, Self::RequestExternalPublish)
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
    for required_field in ["side_effects", "host_validation"] {
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
        assess_pubpunk_inventory, assess_pubpunk_inventory_input_packet,
        assess_pubpunk_inventory_reader_input, assess_pubpunk_publish_request_packet,
        build_pubpunk_inventory_input_packet_from_reader_input, PubPunkAssessmentAuthority,
        PubPunkAssessmentStatus, PubPunkCapabilityGrant, PubPunkInventoryFindingCode,
        PubPunkInventoryInput, PubPunkInventoryInputPacket, PubPunkInventoryInputPacketFindingCode,
        PubPunkInventoryItemInput, PubPunkInventoryItemKind, PubPunkInventoryItemStatus,
        PubPunkInventoryReaderFindingCode, PubPunkInventoryReaderInput, PubPunkPrivacyPolicy,
        PubPunkPublishRequestOperation, PubPunkPublishRequestPacket,
        PubPunkPublishRequestPacketFindingCode, PubPunkWorkspacePolicy,
        PUBPUNK_REQUIRED_INSTRUCTION_REFS,
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
        ])
        .with_granted_capabilities(vec![PubPunkCapabilityGrant::RequestExternalPublish])
        .with_expected_receipt_fields(vec![
            "module_id",
            "operation",
            "source_refs",
            "capability_grants",
            "side_effects",
            "host_validation",
        ])
        .with_token_cost_ref("work/reports/pubpunk-publish-request-token-cost.md")
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
        assert!(assessment.boundary_flags.all_side_effect_flags_false());
        assert_eq!(
            side_effect_refs.request_ref,
            "work/module-side-effects/pubpunk-publish-example.md"
        );
        assert_eq!(
            side_effect_refs.target_ref,
            "publishing/channels/github-discussions.md"
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
        assert!(packet.try_into_side_effect_request_refs().is_err());
    }

    #[test]
    fn publish_request_packet_blocks_unallowed_payload_and_candidate_refs() {
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
