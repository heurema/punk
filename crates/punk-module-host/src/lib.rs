//! Incubating side-effect-free Module Host envelope models.
//!
//! This crate defines a pure invocation and assessment envelope for module
//! models. It does not load plugins, call modules, expose CLI behavior, read or
//! write files, invoke adapters, access credentials, publish externally, write
//! receipts, mutate event logs, write gate decisions, write proofpacks, or
//! claim acceptance.

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const MODULE_HOST_INVOCATION_SCHEMA_VERSION: &str = "punk.module_host.invocation.v0.1";
pub const MODULE_HOST_ASSESSMENT_ENVELOPE_SCHEMA_VERSION: &str =
    "punk.module_host.assessment_envelope.v0.1";
pub const MODULE_HOST_RECEIPT_PROPOSAL_SCHEMA_VERSION: &str =
    "punk.module_host.receipt_proposal.v0.1";
pub const MODULE_HOST_SIDE_EFFECT_REQUEST_PROPOSAL_SCHEMA_VERSION: &str =
    "punk.module_host.side_effect_request_proposal.v0.1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleHostAuthority {
    Advisory,
}

impl ModuleHostAuthority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Advisory => "advisory",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleHostStatus {
    Ready,
    Blocked,
}

impl ModuleHostStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleOutputAuthority {
    Advisory,
    Decision,
    Proof,
    ProjectTruth,
}

impl ModuleOutputAuthority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Advisory => "advisory",
            Self::Decision => "decision",
            Self::Proof => "proof",
            Self::ProjectTruth => "project_truth",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleOutputStatus {
    Ready,
    Blocked,
    Unknown,
}

impl ModuleOutputStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Blocked => "blocked",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleCapabilityGrant {
    AssessProvidedInput,
    ReadFiles,
    WriteFiles,
    WriteReceipts,
    DirectEventLogWrite,
    CallExternalApi,
    OpenBrowser,
    ReadCredentials,
    InvokeAdapter,
    ExternalSideEffect,
    WriteGateDecision,
    WriteProofpack,
    CreateAcceptanceClaim,
}

impl ModuleCapabilityGrant {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AssessProvidedInput => "assess_provided_input",
            Self::ReadFiles => "read_files",
            Self::WriteFiles => "write_files",
            Self::WriteReceipts => "write_receipts",
            Self::DirectEventLogWrite => "direct_event_log_write",
            Self::CallExternalApi => "call_external_api",
            Self::OpenBrowser => "open_browser",
            Self::ReadCredentials => "read_credentials",
            Self::InvokeAdapter => "invoke_adapter",
            Self::ExternalSideEffect => "external_side_effect",
            Self::WriteGateDecision => "write_gate_decision",
            Self::WriteProofpack => "write_proofpack",
            Self::CreateAcceptanceClaim => "create_acceptance_claim",
        }
    }

    pub fn supported_by_pure_envelope(self) -> bool {
        matches!(self, Self::AssessProvidedInput)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModulePrivacyPolicy {
    pub raw_payloads: bool,
    pub raw_prompts: bool,
    pub raw_transcripts: bool,
    pub secrets_or_credentials: bool,
    pub customer_data: bool,
    pub sensitive_code: bool,
}

impl ModulePrivacyPolicy {
    pub const fn safe_metadata_only() -> Self {
        Self {
            raw_payloads: false,
            raw_prompts: false,
            raw_transcripts: false,
            secrets_or_credentials: false,
            customer_data: false,
            sensitive_code: false,
        }
    }

    pub fn allows_private_or_raw_payloads(self) -> bool {
        self.raw_payloads
            || self.raw_prompts
            || self.raw_transcripts
            || self.secrets_or_credentials
            || self.customer_data
            || self.sensitive_code
    }
}

impl Default for ModulePrivacyPolicy {
    fn default() -> Self {
        Self::safe_metadata_only()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleHostBoundaryFlags {
    pub public_cli: bool,
    pub loads_plugins: bool,
    pub invokes_modules: bool,
    pub reads_files: bool,
    pub writes_files: bool,
    pub writes_receipts: bool,
    pub writes_event_log: bool,
    pub calls_external_apis: bool,
    pub opens_browser: bool,
    pub reads_credentials: bool,
    pub invokes_adapters: bool,
    pub writes_gate_decision: bool,
    pub writes_proofpack: bool,
    pub creates_acceptance_claim: bool,
    pub external_side_effects: bool,
}

impl ModuleHostBoundaryFlags {
    pub const fn pure_envelope() -> Self {
        Self {
            public_cli: false,
            loads_plugins: false,
            invokes_modules: false,
            reads_files: false,
            writes_files: false,
            writes_receipts: false,
            writes_event_log: false,
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
            && !self.loads_plugins
            && !self.invokes_modules
            && !self.reads_files
            && !self.writes_files
            && !self.writes_receipts
            && !self.writes_event_log
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

pub const MODULE_HOST_PURE_ENVELOPE_BOUNDARY_FLAGS: ModuleHostBoundaryFlags =
    ModuleHostBoundaryFlags::pure_envelope();

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleInvocationEnvelope {
    pub schema_version: &'static str,
    pub module_id: String,
    pub module_version: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub requested_operation: String,
    pub input_refs: Vec<String>,
    pub granted_capabilities: Vec<ModuleCapabilityGrant>,
    pub privacy_policy: ModulePrivacyPolicy,
    pub expected_receipt_fields: Vec<String>,
}

impl ModuleInvocationEnvelope {
    pub fn new(
        module_id: impl Into<String>,
        module_version: impl Into<String>,
        contract_ref: impl Into<String>,
        run_ref: impl Into<String>,
        project_ref: impl Into<String>,
        requested_operation: impl Into<String>,
    ) -> Self {
        Self {
            schema_version: MODULE_HOST_INVOCATION_SCHEMA_VERSION,
            module_id: module_id.into(),
            module_version: module_version.into(),
            contract_ref: contract_ref.into(),
            run_ref: run_ref.into(),
            project_ref: project_ref.into(),
            requested_operation: requested_operation.into(),
            input_refs: Vec::new(),
            granted_capabilities: Vec::new(),
            privacy_policy: ModulePrivacyPolicy::safe_metadata_only(),
            expected_receipt_fields: Vec::new(),
        }
    }

    pub fn with_input_refs(mut self, input_refs: Vec<impl Into<String>>) -> Self {
        self.input_refs = input_refs.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_granted_capabilities(
        mut self,
        granted_capabilities: Vec<ModuleCapabilityGrant>,
    ) -> Self {
        self.granted_capabilities = granted_capabilities;
        self
    }

    pub fn with_privacy_policy(mut self, privacy_policy: ModulePrivacyPolicy) -> Self {
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
pub struct ModuleOutputBoundaryFlags {
    pub public_cli: bool,
    pub reads_files: bool,
    pub writes_files: bool,
    pub writes_receipts: bool,
    pub writes_event_log: bool,
    pub calls_external_apis: bool,
    pub opens_browser: bool,
    pub reads_credentials: bool,
    pub invokes_adapters: bool,
    pub writes_gate_decision: bool,
    pub writes_proofpack: bool,
    pub creates_acceptance_claim: bool,
    pub external_side_effects: bool,
}

impl ModuleOutputBoundaryFlags {
    pub const fn side_effect_free() -> Self {
        Self {
            public_cli: false,
            reads_files: false,
            writes_files: false,
            writes_receipts: false,
            writes_event_log: false,
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
            && !self.writes_receipts
            && !self.writes_event_log
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleOutputSummary {
    pub assessment_ref: String,
    pub status: ModuleOutputStatus,
    pub authority: ModuleOutputAuthority,
    pub finding_count: usize,
    pub boundary_flags: ModuleOutputBoundaryFlags,
}

impl ModuleOutputSummary {
    pub fn new(
        assessment_ref: impl Into<String>,
        status: ModuleOutputStatus,
        authority: ModuleOutputAuthority,
        finding_count: usize,
        boundary_flags: ModuleOutputBoundaryFlags,
    ) -> Self {
        Self {
            assessment_ref: assessment_ref.into(),
            status,
            authority,
            finding_count,
            boundary_flags,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleHostFindingCode {
    MissingModuleId,
    MissingModuleVersion,
    MissingContractRef,
    MissingRunRef,
    MissingProjectRef,
    MissingRequestedOperation,
    MissingInputRefs,
    UnsafeInputRef,
    MissingExpectedReceiptFields,
    UnsupportedCapabilityGrant,
    UnsafePrivacyPolicy,
    MissingModuleAssessmentRef,
    NonAdvisoryModuleOutput,
    ModuleOutputHasSideEffects,
    AssessmentEnvelopeBlocked,
    AssessmentEnvelopeRefMismatch,
    UnknownExpectedReceiptField,
    ReceiptProposalBlocked,
    ReceiptProposalRefMismatch,
    MissingSideEffectRequestId,
    MissingSideEffectTargetRef,
    MissingSideEffectIntentRef,
    MissingSideEffectPolicyRef,
    MissingSideEffectReceiptProposalRef,
    MissingSideEffectAdapterRef,
    MissingSideEffectPayloadRef,
    UnsafeSideEffectRequestRef,
    MissingSideEffectReceiptCoverage,
    SideEffectRequestHasSideEffects,
}

impl ModuleHostFindingCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingModuleId => "missing_module_id",
            Self::MissingModuleVersion => "missing_module_version",
            Self::MissingContractRef => "missing_contract_ref",
            Self::MissingRunRef => "missing_run_ref",
            Self::MissingProjectRef => "missing_project_ref",
            Self::MissingRequestedOperation => "missing_requested_operation",
            Self::MissingInputRefs => "missing_input_refs",
            Self::UnsafeInputRef => "unsafe_input_ref",
            Self::MissingExpectedReceiptFields => "missing_expected_receipt_fields",
            Self::UnsupportedCapabilityGrant => "unsupported_capability_grant",
            Self::UnsafePrivacyPolicy => "unsafe_privacy_policy",
            Self::MissingModuleAssessmentRef => "missing_module_assessment_ref",
            Self::NonAdvisoryModuleOutput => "non_advisory_module_output",
            Self::ModuleOutputHasSideEffects => "module_output_has_side_effects",
            Self::AssessmentEnvelopeBlocked => "assessment_envelope_blocked",
            Self::AssessmentEnvelopeRefMismatch => "assessment_envelope_ref_mismatch",
            Self::UnknownExpectedReceiptField => "unknown_expected_receipt_field",
            Self::ReceiptProposalBlocked => "receipt_proposal_blocked",
            Self::ReceiptProposalRefMismatch => "receipt_proposal_ref_mismatch",
            Self::MissingSideEffectRequestId => "missing_side_effect_request_id",
            Self::MissingSideEffectTargetRef => "missing_side_effect_target_ref",
            Self::MissingSideEffectIntentRef => "missing_side_effect_intent_ref",
            Self::MissingSideEffectPolicyRef => "missing_side_effect_policy_ref",
            Self::MissingSideEffectReceiptProposalRef => "missing_side_effect_receipt_proposal_ref",
            Self::MissingSideEffectAdapterRef => "missing_side_effect_adapter_ref",
            Self::MissingSideEffectPayloadRef => "missing_side_effect_payload_ref",
            Self::UnsafeSideEffectRequestRef => "unsafe_side_effect_request_ref",
            Self::MissingSideEffectReceiptCoverage => "missing_side_effect_receipt_coverage",
            Self::SideEffectRequestHasSideEffects => "side_effect_request_has_side_effects",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleHostFinding {
    pub code: ModuleHostFindingCode,
    pub input_ref: Option<String>,
    pub capability: Option<ModuleCapabilityGrant>,
    pub message: &'static str,
}

impl ModuleHostFinding {
    fn new(code: ModuleHostFindingCode, message: &'static str) -> Self {
        Self {
            code,
            input_ref: None,
            capability: None,
            message,
        }
    }

    fn for_input_ref(
        code: ModuleHostFindingCode,
        input_ref: impl Into<String>,
        message: &'static str,
    ) -> Self {
        Self {
            code,
            input_ref: Some(input_ref.into()),
            capability: None,
            message,
        }
    }

    fn for_capability(capability: ModuleCapabilityGrant) -> Self {
        Self {
            code: ModuleHostFindingCode::UnsupportedCapabilityGrant,
            input_ref: None,
            capability: Some(capability),
            message: "capability is not available in the pure module-host envelope slice",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleHostPreflight {
    pub schema_version: &'static str,
    pub status: ModuleHostStatus,
    pub authority: ModuleHostAuthority,
    pub findings: Vec<ModuleHostFinding>,
    pub boundary_flags: ModuleHostBoundaryFlags,
}

impl ModuleHostPreflight {
    pub fn has_blockers(&self) -> bool {
        !self.findings.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleAssessmentEnvelope {
    pub schema_version: &'static str,
    pub status: ModuleHostStatus,
    pub authority: ModuleHostAuthority,
    pub module_id: String,
    pub module_version: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub requested_operation: String,
    pub module_output_status: ModuleOutputStatus,
    pub module_output_ref: String,
    pub module_finding_count: usize,
    pub findings: Vec<ModuleHostFinding>,
    pub boundary_flags: ModuleHostBoundaryFlags,
}

impl ModuleAssessmentEnvelope {
    pub fn has_blockers(&self) -> bool {
        !self.findings.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleReceiptProposalField {
    ModuleId,
    ModuleVersion,
    Operation,
    ContractRef,
    RunRef,
    ProjectRef,
    InputRefs,
    CapabilityGrants,
    PrivacyPolicy,
    ModuleOutputRef,
    ModuleOutputStatus,
    ModuleFindings,
    BoundaryFlags,
    SideEffects,
    HostValidation,
}

impl ModuleReceiptProposalField {
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim() {
            "module_id" => Some(Self::ModuleId),
            "module_version" => Some(Self::ModuleVersion),
            "operation" => Some(Self::Operation),
            "contract_ref" => Some(Self::ContractRef),
            "run_ref" => Some(Self::RunRef),
            "project_ref" => Some(Self::ProjectRef),
            "input_refs" => Some(Self::InputRefs),
            "capability_grants" => Some(Self::CapabilityGrants),
            "privacy_policy" => Some(Self::PrivacyPolicy),
            "module_output_ref" => Some(Self::ModuleOutputRef),
            "module_output_status" => Some(Self::ModuleOutputStatus),
            "module_findings" => Some(Self::ModuleFindings),
            "boundary_flags" => Some(Self::BoundaryFlags),
            "side_effects" => Some(Self::SideEffects),
            "host_validation" => Some(Self::HostValidation),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ModuleId => "module_id",
            Self::ModuleVersion => "module_version",
            Self::Operation => "operation",
            Self::ContractRef => "contract_ref",
            Self::RunRef => "run_ref",
            Self::ProjectRef => "project_ref",
            Self::InputRefs => "input_refs",
            Self::CapabilityGrants => "capability_grants",
            Self::PrivacyPolicy => "privacy_policy",
            Self::ModuleOutputRef => "module_output_ref",
            Self::ModuleOutputStatus => "module_output_status",
            Self::ModuleFindings => "module_findings",
            Self::BoundaryFlags => "boundary_flags",
            Self::SideEffects => "side_effects",
            Self::HostValidation => "host_validation",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleReceiptProposalBoundaryFlags {
    pub creates_receipt: bool,
    pub writes_receipt: bool,
    pub writes_event_log: bool,
    pub reads_files: bool,
    pub writes_files: bool,
    pub calls_external_apis: bool,
    pub opens_browser: bool,
    pub reads_credentials: bool,
    pub invokes_adapters: bool,
    pub invokes_modules: bool,
    pub writes_gate_decision: bool,
    pub writes_proofpack: bool,
    pub creates_acceptance_claim: bool,
    pub external_side_effects: bool,
}

impl ModuleReceiptProposalBoundaryFlags {
    pub const fn pure_proposal() -> Self {
        Self {
            creates_receipt: false,
            writes_receipt: false,
            writes_event_log: false,
            reads_files: false,
            writes_files: false,
            calls_external_apis: false,
            opens_browser: false,
            reads_credentials: false,
            invokes_adapters: false,
            invokes_modules: false,
            writes_gate_decision: false,
            writes_proofpack: false,
            creates_acceptance_claim: false,
            external_side_effects: false,
        }
    }

    pub fn all_side_effect_flags_false(self) -> bool {
        !self.creates_receipt
            && !self.writes_receipt
            && !self.writes_event_log
            && !self.reads_files
            && !self.writes_files
            && !self.calls_external_apis
            && !self.opens_browser
            && !self.reads_credentials
            && !self.invokes_adapters
            && !self.invokes_modules
            && !self.writes_gate_decision
            && !self.writes_proofpack
            && !self.creates_acceptance_claim
            && !self.external_side_effects
    }
}

pub const MODULE_HOST_PURE_RECEIPT_PROPOSAL_BOUNDARY_FLAGS: ModuleReceiptProposalBoundaryFlags =
    ModuleReceiptProposalBoundaryFlags::pure_proposal();

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleAssessmentReceiptProposal {
    pub schema_version: &'static str,
    pub status: ModuleHostStatus,
    pub authority: ModuleHostAuthority,
    pub module_id: String,
    pub module_version: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub requested_operation: String,
    pub module_output_ref: String,
    pub required_fields: Vec<ModuleReceiptProposalField>,
    pub covered_fields: Vec<ModuleReceiptProposalField>,
    pub findings: Vec<ModuleHostFinding>,
    pub boundary_flags: ModuleReceiptProposalBoundaryFlags,
}

impl ModuleAssessmentReceiptProposal {
    pub fn has_blockers(&self) -> bool {
        !self.findings.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleSideEffectKind {
    Publish,
    Comment,
    CreatePullRequest,
}

impl ModuleSideEffectKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Publish => "publish",
            Self::Comment => "comment",
            Self::CreatePullRequest => "create_pull_request",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleSideEffectPrecondition {
    HostPolicyRef,
    ReadyReceiptProposal,
    SideEffectReceiptCoverage,
    AdapterInvocationReceipt,
    SafePayloadRef,
    GateOrPolicyApproval,
}

impl ModuleSideEffectPrecondition {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HostPolicyRef => "host_policy_ref",
            Self::ReadyReceiptProposal => "ready_receipt_proposal",
            Self::SideEffectReceiptCoverage => "side_effect_receipt_coverage",
            Self::AdapterInvocationReceipt => "adapter_invocation_receipt",
            Self::SafePayloadRef => "safe_payload_ref",
            Self::GateOrPolicyApproval => "gate_or_policy_approval",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleSideEffectRequestBoundaryFlags {
    pub performs_external_side_effect: bool,
    pub invokes_adapter: bool,
    pub publishes: bool,
    pub comments: bool,
    pub creates_pull_request: bool,
    pub writes_receipt: bool,
    pub writes_event_log: bool,
    pub reads_files: bool,
    pub writes_files: bool,
    pub calls_external_apis: bool,
    pub opens_browser: bool,
    pub reads_credentials: bool,
    pub writes_gate_decision: bool,
    pub writes_proofpack: bool,
    pub creates_acceptance_claim: bool,
}

impl ModuleSideEffectRequestBoundaryFlags {
    pub const fn pure_proposal() -> Self {
        Self {
            performs_external_side_effect: false,
            invokes_adapter: false,
            publishes: false,
            comments: false,
            creates_pull_request: false,
            writes_receipt: false,
            writes_event_log: false,
            reads_files: false,
            writes_files: false,
            calls_external_apis: false,
            opens_browser: false,
            reads_credentials: false,
            writes_gate_decision: false,
            writes_proofpack: false,
            creates_acceptance_claim: false,
        }
    }

    pub fn all_side_effect_flags_false(self) -> bool {
        !self.performs_external_side_effect
            && !self.invokes_adapter
            && !self.publishes
            && !self.comments
            && !self.creates_pull_request
            && !self.writes_receipt
            && !self.writes_event_log
            && !self.reads_files
            && !self.writes_files
            && !self.calls_external_apis
            && !self.opens_browser
            && !self.reads_credentials
            && !self.writes_gate_decision
            && !self.writes_proofpack
            && !self.creates_acceptance_claim
    }
}

pub const MODULE_HOST_PURE_SIDE_EFFECT_REQUEST_BOUNDARY_FLAGS:
    ModuleSideEffectRequestBoundaryFlags = ModuleSideEffectRequestBoundaryFlags::pure_proposal();

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleSideEffectRequestDraft {
    pub request_id: String,
    pub kind: ModuleSideEffectKind,
    pub target_ref: String,
    pub intent_ref: String,
    pub policy_ref: String,
    pub receipt_proposal_ref: String,
    pub adapter_ref: String,
    pub payload_ref: String,
    pub boundary_flags: ModuleSideEffectRequestBoundaryFlags,
}

impl ModuleSideEffectRequestDraft {
    pub fn new(request_id: impl Into<String>, kind: ModuleSideEffectKind) -> Self {
        Self {
            request_id: request_id.into(),
            kind,
            target_ref: String::new(),
            intent_ref: String::new(),
            policy_ref: String::new(),
            receipt_proposal_ref: String::new(),
            adapter_ref: String::new(),
            payload_ref: String::new(),
            boundary_flags: ModuleSideEffectRequestBoundaryFlags::pure_proposal(),
        }
    }

    pub fn with_target_ref(mut self, target_ref: impl Into<String>) -> Self {
        self.target_ref = target_ref.into();
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

    pub fn with_receipt_proposal_ref(mut self, receipt_proposal_ref: impl Into<String>) -> Self {
        self.receipt_proposal_ref = receipt_proposal_ref.into();
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

    pub fn with_boundary_flags(
        mut self,
        boundary_flags: ModuleSideEffectRequestBoundaryFlags,
    ) -> Self {
        self.boundary_flags = boundary_flags;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleSideEffectRequestProposal {
    pub schema_version: &'static str,
    pub status: ModuleHostStatus,
    pub authority: ModuleHostAuthority,
    pub module_id: String,
    pub module_version: String,
    pub contract_ref: String,
    pub run_ref: String,
    pub project_ref: String,
    pub requested_operation: String,
    pub request_id: String,
    pub kind: ModuleSideEffectKind,
    pub target_ref: String,
    pub intent_ref: String,
    pub policy_ref: String,
    pub receipt_proposal_ref: String,
    pub adapter_ref: String,
    pub payload_ref: String,
    pub required_preconditions: Vec<ModuleSideEffectPrecondition>,
    pub covered_preconditions: Vec<ModuleSideEffectPrecondition>,
    pub findings: Vec<ModuleHostFinding>,
    pub boundary_flags: ModuleSideEffectRequestBoundaryFlags,
}

impl ModuleSideEffectRequestProposal {
    pub fn has_blockers(&self) -> bool {
        !self.findings.is_empty()
    }
}

pub fn preflight_module_invocation(input: &ModuleInvocationEnvelope) -> ModuleHostPreflight {
    let findings = invocation_findings(input);
    let status = if findings.is_empty() {
        ModuleHostStatus::Ready
    } else {
        ModuleHostStatus::Blocked
    };

    ModuleHostPreflight {
        schema_version: MODULE_HOST_INVOCATION_SCHEMA_VERSION,
        status,
        authority: ModuleHostAuthority::Advisory,
        findings,
        boundary_flags: MODULE_HOST_PURE_ENVELOPE_BOUNDARY_FLAGS,
    }
}

pub fn propose_module_assessment_receipt(
    invocation: &ModuleInvocationEnvelope,
    envelope: &ModuleAssessmentEnvelope,
) -> ModuleAssessmentReceiptProposal {
    let mut findings = invocation_findings(invocation);
    let mut required_fields = Vec::new();

    if envelope.status != ModuleHostStatus::Ready || envelope.has_blockers() {
        findings.push(ModuleHostFinding::new(
            ModuleHostFindingCode::AssessmentEnvelopeBlocked,
            "assessment envelope must be ready before a receipt proposal can be modeled",
        ));
    }

    if !invocation_matches_envelope(invocation, envelope) {
        findings.push(ModuleHostFinding::new(
            ModuleHostFindingCode::AssessmentEnvelopeRefMismatch,
            "assessment envelope refs must match the invocation refs",
        ));
    }

    if envelope.module_output_ref.trim().is_empty() || !is_safe_ref(&envelope.module_output_ref) {
        findings.push(ModuleHostFinding::for_input_ref(
            ModuleHostFindingCode::UnsafeInputRef,
            envelope.module_output_ref.clone(),
            "module output ref must be explicit and repo-relative",
        ));
    }

    if !envelope.boundary_flags.all_side_effect_flags_false() {
        findings.push(ModuleHostFinding::new(
            ModuleHostFindingCode::ModuleOutputHasSideEffects,
            "assessment envelope reports side-effect or authority-bearing behavior",
        ));
    }

    for expected_field in &invocation.expected_receipt_fields {
        match ModuleReceiptProposalField::parse(expected_field) {
            Some(field) => {
                if !required_fields.contains(&field) {
                    required_fields.push(field);
                }
            }
            None => findings.push(ModuleHostFinding::for_input_ref(
                ModuleHostFindingCode::UnknownExpectedReceiptField,
                expected_field.clone(),
                "expected receipt field is not known to the module-host proposal model",
            )),
        }
    }

    let status = if findings.is_empty() {
        ModuleHostStatus::Ready
    } else {
        ModuleHostStatus::Blocked
    };
    let covered_fields = if status == ModuleHostStatus::Ready {
        required_fields.clone()
    } else {
        Vec::new()
    };

    ModuleAssessmentReceiptProposal {
        schema_version: MODULE_HOST_RECEIPT_PROPOSAL_SCHEMA_VERSION,
        status,
        authority: ModuleHostAuthority::Advisory,
        module_id: invocation.module_id.clone(),
        module_version: invocation.module_version.clone(),
        contract_ref: invocation.contract_ref.clone(),
        run_ref: invocation.run_ref.clone(),
        project_ref: invocation.project_ref.clone(),
        requested_operation: invocation.requested_operation.clone(),
        module_output_ref: envelope.module_output_ref.clone(),
        required_fields,
        covered_fields,
        findings,
        boundary_flags: MODULE_HOST_PURE_RECEIPT_PROPOSAL_BOUNDARY_FLAGS,
    }
}

pub fn propose_module_side_effect_request(
    invocation: &ModuleInvocationEnvelope,
    receipt_proposal: &ModuleAssessmentReceiptProposal,
    request: &ModuleSideEffectRequestDraft,
) -> ModuleSideEffectRequestProposal {
    let mut findings = invocation_findings(invocation);
    let required_preconditions = default_side_effect_preconditions();

    if receipt_proposal.status != ModuleHostStatus::Ready || receipt_proposal.has_blockers() {
        findings.push(ModuleHostFinding::new(
            ModuleHostFindingCode::ReceiptProposalBlocked,
            "receipt proposal must be ready before a side-effect request can be modeled",
        ));
    }

    if !invocation_matches_receipt_proposal(invocation, receipt_proposal) {
        findings.push(ModuleHostFinding::new(
            ModuleHostFindingCode::ReceiptProposalRefMismatch,
            "receipt proposal refs must match the invocation refs",
        ));
    }

    if !receipt_proposal
        .covered_fields
        .contains(&ModuleReceiptProposalField::SideEffects)
        || !receipt_proposal
            .covered_fields
            .contains(&ModuleReceiptProposalField::HostValidation)
    {
        findings.push(ModuleHostFinding::new(
            ModuleHostFindingCode::MissingSideEffectReceiptCoverage,
            "receipt proposal must cover side_effects and host_validation fields",
        ));
    }

    push_required_safe_ref_finding(
        &mut findings,
        request.request_id.as_str(),
        ModuleHostFindingCode::MissingSideEffectRequestId,
        "side-effect request id is required",
    );
    push_required_safe_ref_finding(
        &mut findings,
        request.target_ref.as_str(),
        ModuleHostFindingCode::MissingSideEffectTargetRef,
        "side-effect target ref is required",
    );
    push_required_safe_ref_finding(
        &mut findings,
        request.intent_ref.as_str(),
        ModuleHostFindingCode::MissingSideEffectIntentRef,
        "side-effect intent ref is required",
    );
    push_required_safe_ref_finding(
        &mut findings,
        request.policy_ref.as_str(),
        ModuleHostFindingCode::MissingSideEffectPolicyRef,
        "side-effect policy ref is required",
    );
    push_required_safe_ref_finding(
        &mut findings,
        request.receipt_proposal_ref.as_str(),
        ModuleHostFindingCode::MissingSideEffectReceiptProposalRef,
        "side-effect receipt proposal ref is required",
    );
    push_required_safe_ref_finding(
        &mut findings,
        request.adapter_ref.as_str(),
        ModuleHostFindingCode::MissingSideEffectAdapterRef,
        "side-effect adapter ref is required",
    );
    push_required_safe_ref_finding(
        &mut findings,
        request.payload_ref.as_str(),
        ModuleHostFindingCode::MissingSideEffectPayloadRef,
        "side-effect payload ref is required",
    );

    if !request.boundary_flags.all_side_effect_flags_false() {
        findings.push(ModuleHostFinding::new(
            ModuleHostFindingCode::SideEffectRequestHasSideEffects,
            "side-effect request proposal must not perform the side effect",
        ));
    }

    let status = if findings.is_empty() {
        ModuleHostStatus::Ready
    } else {
        ModuleHostStatus::Blocked
    };
    let covered_preconditions = if status == ModuleHostStatus::Ready {
        required_preconditions.clone()
    } else {
        Vec::new()
    };

    ModuleSideEffectRequestProposal {
        schema_version: MODULE_HOST_SIDE_EFFECT_REQUEST_PROPOSAL_SCHEMA_VERSION,
        status,
        authority: ModuleHostAuthority::Advisory,
        module_id: invocation.module_id.clone(),
        module_version: invocation.module_version.clone(),
        contract_ref: invocation.contract_ref.clone(),
        run_ref: invocation.run_ref.clone(),
        project_ref: invocation.project_ref.clone(),
        requested_operation: invocation.requested_operation.clone(),
        request_id: request.request_id.clone(),
        kind: request.kind,
        target_ref: request.target_ref.clone(),
        intent_ref: request.intent_ref.clone(),
        policy_ref: request.policy_ref.clone(),
        receipt_proposal_ref: request.receipt_proposal_ref.clone(),
        adapter_ref: request.adapter_ref.clone(),
        payload_ref: request.payload_ref.clone(),
        required_preconditions,
        covered_preconditions,
        findings,
        boundary_flags: MODULE_HOST_PURE_SIDE_EFFECT_REQUEST_BOUNDARY_FLAGS,
    }
}

pub fn wrap_module_assessment(
    invocation: &ModuleInvocationEnvelope,
    output: &ModuleOutputSummary,
) -> ModuleAssessmentEnvelope {
    let mut findings = invocation_findings(invocation);

    if output.assessment_ref.trim().is_empty() {
        findings.push(ModuleHostFinding::new(
            ModuleHostFindingCode::MissingModuleAssessmentRef,
            "module assessment ref is required",
        ));
    } else if !is_safe_ref(output.assessment_ref.as_str()) {
        findings.push(ModuleHostFinding::for_input_ref(
            ModuleHostFindingCode::UnsafeInputRef,
            output.assessment_ref.clone(),
            "module assessment ref must be explicit and repo-relative",
        ));
    }

    if output.authority != ModuleOutputAuthority::Advisory {
        findings.push(ModuleHostFinding::new(
            ModuleHostFindingCode::NonAdvisoryModuleOutput,
            "module output must be advisory evidence only",
        ));
    }

    if !output.boundary_flags.all_side_effect_flags_false() {
        findings.push(ModuleHostFinding::new(
            ModuleHostFindingCode::ModuleOutputHasSideEffects,
            "module output reports side-effect or authority-bearing behavior",
        ));
    }

    let status = if findings.is_empty() {
        ModuleHostStatus::Ready
    } else {
        ModuleHostStatus::Blocked
    };

    ModuleAssessmentEnvelope {
        schema_version: MODULE_HOST_ASSESSMENT_ENVELOPE_SCHEMA_VERSION,
        status,
        authority: ModuleHostAuthority::Advisory,
        module_id: invocation.module_id.clone(),
        module_version: invocation.module_version.clone(),
        contract_ref: invocation.contract_ref.clone(),
        run_ref: invocation.run_ref.clone(),
        project_ref: invocation.project_ref.clone(),
        requested_operation: invocation.requested_operation.clone(),
        module_output_status: output.status,
        module_output_ref: output.assessment_ref.clone(),
        module_finding_count: output.finding_count,
        findings,
        boundary_flags: MODULE_HOST_PURE_ENVELOPE_BOUNDARY_FLAGS,
    }
}

fn invocation_findings(input: &ModuleInvocationEnvelope) -> Vec<ModuleHostFinding> {
    let mut findings = Vec::new();

    push_required_finding(
        &mut findings,
        input.module_id.as_str(),
        ModuleHostFindingCode::MissingModuleId,
        "module id is required",
    );
    push_required_finding(
        &mut findings,
        input.module_version.as_str(),
        ModuleHostFindingCode::MissingModuleVersion,
        "module version is required",
    );
    push_required_finding(
        &mut findings,
        input.contract_ref.as_str(),
        ModuleHostFindingCode::MissingContractRef,
        "contract ref is required",
    );
    push_required_finding(
        &mut findings,
        input.run_ref.as_str(),
        ModuleHostFindingCode::MissingRunRef,
        "run ref is required",
    );
    push_required_finding(
        &mut findings,
        input.project_ref.as_str(),
        ModuleHostFindingCode::MissingProjectRef,
        "project ref is required",
    );
    push_required_finding(
        &mut findings,
        input.requested_operation.as_str(),
        ModuleHostFindingCode::MissingRequestedOperation,
        "requested operation is required",
    );

    if input.input_refs.is_empty() {
        findings.push(ModuleHostFinding::new(
            ModuleHostFindingCode::MissingInputRefs,
            "at least one explicit input ref is required",
        ));
    }

    for input_ref in &input.input_refs {
        if !is_safe_ref(input_ref) {
            findings.push(ModuleHostFinding::for_input_ref(
                ModuleHostFindingCode::UnsafeInputRef,
                input_ref.clone(),
                "input refs must be explicit repo-relative refs",
            ));
        }
    }

    if input.expected_receipt_fields.is_empty() {
        findings.push(ModuleHostFinding::new(
            ModuleHostFindingCode::MissingExpectedReceiptFields,
            "expected receipt fields must be declared even when no receipt is written",
        ));
    }

    for capability in &input.granted_capabilities {
        if !capability.supported_by_pure_envelope() {
            findings.push(ModuleHostFinding::for_capability(*capability));
        }
    }

    if input.privacy_policy.allows_private_or_raw_payloads() {
        findings.push(ModuleHostFinding::new(
            ModuleHostFindingCode::UnsafePrivacyPolicy,
            "privacy policy must disallow raw/private payloads for this pure envelope slice",
        ));
    }

    findings
}

fn invocation_matches_envelope(
    invocation: &ModuleInvocationEnvelope,
    envelope: &ModuleAssessmentEnvelope,
) -> bool {
    invocation.module_id == envelope.module_id
        && invocation.module_version == envelope.module_version
        && invocation.contract_ref == envelope.contract_ref
        && invocation.run_ref == envelope.run_ref
        && invocation.project_ref == envelope.project_ref
        && invocation.requested_operation == envelope.requested_operation
}

fn invocation_matches_receipt_proposal(
    invocation: &ModuleInvocationEnvelope,
    proposal: &ModuleAssessmentReceiptProposal,
) -> bool {
    invocation.module_id == proposal.module_id
        && invocation.module_version == proposal.module_version
        && invocation.contract_ref == proposal.contract_ref
        && invocation.run_ref == proposal.run_ref
        && invocation.project_ref == proposal.project_ref
        && invocation.requested_operation == proposal.requested_operation
}

fn default_side_effect_preconditions() -> Vec<ModuleSideEffectPrecondition> {
    vec![
        ModuleSideEffectPrecondition::HostPolicyRef,
        ModuleSideEffectPrecondition::ReadyReceiptProposal,
        ModuleSideEffectPrecondition::SideEffectReceiptCoverage,
        ModuleSideEffectPrecondition::AdapterInvocationReceipt,
        ModuleSideEffectPrecondition::SafePayloadRef,
        ModuleSideEffectPrecondition::GateOrPolicyApproval,
    ]
}

fn push_required_safe_ref_finding(
    findings: &mut Vec<ModuleHostFinding>,
    value: &str,
    missing_code: ModuleHostFindingCode,
    missing_message: &'static str,
) {
    if value.trim().is_empty() {
        findings.push(ModuleHostFinding::new(missing_code, missing_message));
    } else if !is_safe_ref(value) {
        findings.push(ModuleHostFinding::for_input_ref(
            ModuleHostFindingCode::UnsafeSideEffectRequestRef,
            value.to_owned(),
            "side-effect request refs must be explicit repo-relative refs",
        ));
    }
}

fn push_required_finding(
    findings: &mut Vec<ModuleHostFinding>,
    value: &str,
    code: ModuleHostFindingCode,
    message: &'static str,
) {
    if value.trim().is_empty() {
        findings.push(ModuleHostFinding::new(code, message));
    }
}

fn is_safe_ref(value: &str) -> bool {
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
        preflight_module_invocation, propose_module_assessment_receipt,
        propose_module_side_effect_request, wrap_module_assessment, ModuleCapabilityGrant,
        ModuleHostFindingCode, ModuleHostStatus, ModuleInvocationEnvelope, ModuleOutputAuthority,
        ModuleOutputBoundaryFlags, ModuleOutputStatus, ModuleOutputSummary, ModulePrivacyPolicy,
        ModuleReceiptProposalField, ModuleSideEffectKind, ModuleSideEffectPrecondition,
        ModuleSideEffectRequestBoundaryFlags, ModuleSideEffectRequestDraft,
    };

    fn valid_invocation() -> ModuleInvocationEnvelope {
        ModuleInvocationEnvelope::new(
            "pubpunk",
            "v0.1",
            "contracts/pubpunk-inventory",
            "runs/pubpunk-inventory",
            "project/punk",
            "assess_inventory",
        )
        .with_input_refs(vec!["publishing/posts/example.md"])
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
        ])
    }

    fn advisory_output() -> ModuleOutputSummary {
        ModuleOutputSummary::new(
            "work/module-assessments/pubpunk-inventory.md",
            ModuleOutputStatus::Ready,
            ModuleOutputAuthority::Advisory,
            2,
            ModuleOutputBoundaryFlags::side_effect_free(),
        )
    }

    fn ready_receipt_proposal() -> (
        ModuleInvocationEnvelope,
        super::ModuleAssessmentReceiptProposal,
    ) {
        let invocation = valid_invocation();
        let envelope = wrap_module_assessment(&invocation, &advisory_output());
        let proposal = propose_module_assessment_receipt(&invocation, &envelope);
        (invocation, proposal)
    }

    fn side_effect_request() -> ModuleSideEffectRequestDraft {
        ModuleSideEffectRequestDraft::new(
            "work/module-side-effects/pubpunk-publish-community-lab.md",
            ModuleSideEffectKind::Publish,
        )
        .with_target_ref("publishing/channels/github-discussions-community-lab.md")
        .with_intent_ref("work/goals/goal_pubpunk_publish_cycle_0.md")
        .with_policy_ref("docs/modules/pubpunk.md")
        .with_receipt_proposal_ref("work/module-receipts/pubpunk-publish-community-lab.md")
        .with_adapter_ref("adapters/github-discussions")
        .with_payload_ref("publishing/posts/community-lab.md")
    }

    #[test]
    fn preflight_accepts_pure_assessment_invocation() {
        let preflight = preflight_module_invocation(&valid_invocation());

        assert_eq!(preflight.status, ModuleHostStatus::Ready);
        assert!(!preflight.has_blockers());
        assert!(preflight.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn preflight_blocks_unsupported_capabilities() {
        let invocation = valid_invocation()
            .with_granted_capabilities(vec![ModuleCapabilityGrant::CallExternalApi]);
        let preflight = preflight_module_invocation(&invocation);

        assert_eq!(preflight.status, ModuleHostStatus::Blocked);
        assert!(preflight
            .findings
            .iter()
            .any(|finding| finding.code == ModuleHostFindingCode::UnsupportedCapabilityGrant));
        assert!(preflight.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn preflight_blocks_raw_private_policy() {
        let invocation = valid_invocation().with_privacy_policy(ModulePrivacyPolicy {
            raw_payloads: true,
            ..ModulePrivacyPolicy::safe_metadata_only()
        });
        let preflight = preflight_module_invocation(&invocation);

        assert_eq!(preflight.status, ModuleHostStatus::Blocked);
        assert!(preflight
            .findings
            .iter()
            .any(|finding| finding.code == ModuleHostFindingCode::UnsafePrivacyPolicy));
        assert!(preflight.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn preflight_blocks_unsafe_input_refs() {
        let invocation = valid_invocation().with_input_refs(vec!["../publishing/posts/example.md"]);
        let preflight = preflight_module_invocation(&invocation);

        assert_eq!(preflight.status, ModuleHostStatus::Blocked);
        assert!(preflight
            .findings
            .iter()
            .any(|finding| finding.code == ModuleHostFindingCode::UnsafeInputRef));
        assert!(preflight.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn assessment_envelope_accepts_advisory_side_effect_free_output() {
        let envelope = wrap_module_assessment(&valid_invocation(), &advisory_output());

        assert_eq!(envelope.status, ModuleHostStatus::Ready);
        assert!(!envelope.has_blockers());
        assert_eq!(envelope.module_id, "pubpunk");
        assert_eq!(envelope.module_output_status, ModuleOutputStatus::Ready);
        assert_eq!(
            envelope.module_output_ref,
            "work/module-assessments/pubpunk-inventory.md"
        );
        assert_eq!(envelope.module_finding_count, 2);
        assert!(envelope.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn assessment_envelope_blocks_non_advisory_output() {
        let output = ModuleOutputSummary::new(
            "work/module-assessments/pubpunk-inventory.md",
            ModuleOutputStatus::Ready,
            ModuleOutputAuthority::Decision,
            0,
            ModuleOutputBoundaryFlags::side_effect_free(),
        );
        let envelope = wrap_module_assessment(&valid_invocation(), &output);

        assert_eq!(envelope.status, ModuleHostStatus::Blocked);
        assert!(envelope
            .findings
            .iter()
            .any(|finding| finding.code == ModuleHostFindingCode::NonAdvisoryModuleOutput));
        assert!(envelope.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn assessment_envelope_blocks_side_effect_output() {
        let output = ModuleOutputSummary::new(
            "work/module-assessments/pubpunk-inventory.md",
            ModuleOutputStatus::Ready,
            ModuleOutputAuthority::Advisory,
            0,
            ModuleOutputBoundaryFlags {
                calls_external_apis: true,
                ..ModuleOutputBoundaryFlags::side_effect_free()
            },
        );
        let envelope = wrap_module_assessment(&valid_invocation(), &output);

        assert_eq!(envelope.status, ModuleHostStatus::Blocked);
        assert!(envelope
            .findings
            .iter()
            .any(|finding| finding.code == ModuleHostFindingCode::ModuleOutputHasSideEffects));
        assert!(envelope.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn receipt_proposal_accepts_matching_advisory_envelope() {
        let invocation = valid_invocation();
        let envelope = wrap_module_assessment(&invocation, &advisory_output());
        let proposal = propose_module_assessment_receipt(&invocation, &envelope);

        assert_eq!(proposal.status, ModuleHostStatus::Ready);
        assert!(!proposal.has_blockers());
        assert_eq!(proposal.module_id, "pubpunk");
        assert_eq!(
            proposal.module_output_ref,
            "work/module-assessments/pubpunk-inventory.md"
        );
        assert!(proposal
            .required_fields
            .contains(&ModuleReceiptProposalField::ModuleOutputRef));
        assert!(proposal
            .covered_fields
            .contains(&ModuleReceiptProposalField::HostValidation));
        assert!(proposal.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn receipt_proposal_blocks_blocked_assessment_envelope() {
        let invocation = valid_invocation();
        let blocked_output = ModuleOutputSummary::new(
            "work/module-assessments/pubpunk-inventory.md",
            ModuleOutputStatus::Ready,
            ModuleOutputAuthority::Decision,
            0,
            ModuleOutputBoundaryFlags::side_effect_free(),
        );
        let envelope = wrap_module_assessment(&invocation, &blocked_output);
        let proposal = propose_module_assessment_receipt(&invocation, &envelope);

        assert_eq!(proposal.status, ModuleHostStatus::Blocked);
        assert!(proposal
            .findings
            .iter()
            .any(|finding| finding.code == ModuleHostFindingCode::AssessmentEnvelopeBlocked));
        assert!(proposal.covered_fields.is_empty());
        assert!(proposal.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn receipt_proposal_blocks_envelope_ref_mismatch() {
        let invocation = valid_invocation();
        let mut envelope = wrap_module_assessment(&invocation, &advisory_output());
        envelope.run_ref = "runs/other".to_owned();

        let proposal = propose_module_assessment_receipt(&invocation, &envelope);

        assert_eq!(proposal.status, ModuleHostStatus::Blocked);
        assert!(proposal
            .findings
            .iter()
            .any(|finding| finding.code == ModuleHostFindingCode::AssessmentEnvelopeRefMismatch));
        assert!(proposal.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn receipt_proposal_blocks_unknown_expected_receipt_field() {
        let invocation =
            valid_invocation().with_expected_receipt_fields(vec!["module_id", "raw_body"]);
        let envelope = wrap_module_assessment(&invocation, &advisory_output());
        let proposal = propose_module_assessment_receipt(&invocation, &envelope);

        assert_eq!(proposal.status, ModuleHostStatus::Blocked);
        assert!(proposal
            .required_fields
            .contains(&ModuleReceiptProposalField::ModuleId));
        assert!(proposal
            .findings
            .iter()
            .any(|finding| finding.code == ModuleHostFindingCode::UnknownExpectedReceiptField));
        assert!(proposal.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn side_effect_request_proposal_accepts_ready_receipt_and_safe_refs() {
        let (invocation, receipt_proposal) = ready_receipt_proposal();
        let proposal = propose_module_side_effect_request(
            &invocation,
            &receipt_proposal,
            &side_effect_request(),
        );

        assert_eq!(proposal.status, ModuleHostStatus::Ready);
        assert!(!proposal.has_blockers());
        assert_eq!(proposal.kind, ModuleSideEffectKind::Publish);
        assert!(proposal
            .covered_preconditions
            .contains(&ModuleSideEffectPrecondition::AdapterInvocationReceipt));
        assert!(proposal
            .covered_preconditions
            .contains(&ModuleSideEffectPrecondition::GateOrPolicyApproval));
        assert!(proposal.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn side_effect_request_proposal_blocks_blocked_receipt_proposal() {
        let invocation =
            valid_invocation().with_expected_receipt_fields(vec!["module_id", "raw_body"]);
        let envelope = wrap_module_assessment(&invocation, &advisory_output());
        let receipt_proposal = propose_module_assessment_receipt(&invocation, &envelope);
        let proposal = propose_module_side_effect_request(
            &invocation,
            &receipt_proposal,
            &side_effect_request(),
        );

        assert_eq!(proposal.status, ModuleHostStatus::Blocked);
        assert!(proposal
            .findings
            .iter()
            .any(|finding| finding.code == ModuleHostFindingCode::ReceiptProposalBlocked));
        assert!(proposal.covered_preconditions.is_empty());
        assert!(proposal.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn side_effect_request_proposal_blocks_receipt_ref_mismatch() {
        let (invocation, mut receipt_proposal) = ready_receipt_proposal();
        receipt_proposal.project_ref = "project/other".to_owned();
        let proposal = propose_module_side_effect_request(
            &invocation,
            &receipt_proposal,
            &side_effect_request(),
        );

        assert_eq!(proposal.status, ModuleHostStatus::Blocked);
        assert!(proposal
            .findings
            .iter()
            .any(|finding| finding.code == ModuleHostFindingCode::ReceiptProposalRefMismatch));
        assert!(proposal.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn side_effect_request_proposal_blocks_missing_side_effect_receipt_coverage() {
        let invocation = valid_invocation().with_expected_receipt_fields(vec![
            "module_id",
            "module_version",
            "host_validation",
        ]);
        let envelope = wrap_module_assessment(&invocation, &advisory_output());
        let receipt_proposal = propose_module_assessment_receipt(&invocation, &envelope);
        let proposal = propose_module_side_effect_request(
            &invocation,
            &receipt_proposal,
            &side_effect_request(),
        );

        assert_eq!(receipt_proposal.status, ModuleHostStatus::Ready);
        assert_eq!(proposal.status, ModuleHostStatus::Blocked);
        assert!(proposal.findings.iter().any(|finding| {
            finding.code == ModuleHostFindingCode::MissingSideEffectReceiptCoverage
        }));
        assert!(proposal.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn side_effect_request_proposal_blocks_unsafe_refs() {
        let (invocation, receipt_proposal) = ready_receipt_proposal();
        let request = side_effect_request().with_target_ref("https://github.com/heurema/punk");
        let proposal = propose_module_side_effect_request(&invocation, &receipt_proposal, &request);

        assert_eq!(proposal.status, ModuleHostStatus::Blocked);
        assert!(proposal
            .findings
            .iter()
            .any(|finding| finding.code == ModuleHostFindingCode::UnsafeSideEffectRequestRef));
        assert!(proposal.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn side_effect_request_proposal_blocks_performed_side_effect_flags() {
        let (invocation, receipt_proposal) = ready_receipt_proposal();
        let request =
            side_effect_request().with_boundary_flags(ModuleSideEffectRequestBoundaryFlags {
                publishes: true,
                calls_external_apis: true,
                ..ModuleSideEffectRequestBoundaryFlags::pure_proposal()
            });
        let proposal = propose_module_side_effect_request(&invocation, &receipt_proposal, &request);

        assert_eq!(proposal.status, ModuleHostStatus::Blocked);
        assert!(proposal.findings.iter().any(|finding| {
            finding.code == ModuleHostFindingCode::SideEffectRequestHasSideEffects
        }));
        assert!(proposal.boundary_flags.all_side_effect_flags_false());
    }
}
