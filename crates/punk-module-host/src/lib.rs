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
pub const MODULE_HOST_POLICY_GATE_PREFLIGHT_SCHEMA_VERSION: &str =
    "punk.module_host.policy_gate_preflight.v0.1";
pub const MODULE_HOST_SIDE_EFFECT_RECEIPT_WRITER_PREFLIGHT_SCHEMA_VERSION: &str =
    "punk.module_host.side_effect_receipt_writer_preflight.v0.1";
pub const MODULE_HOST_SIDE_EFFECT_RECEIPT_WRITER_ACTIVE_BEHAVIOR_SCHEMA_VERSION: &str =
    "punk.module_host.side_effect_receipt_writer_active_behavior.v0.1";

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
    SideEffectRequestProposalBlocked,
    MissingPolicyGatePreflightId,
    MissingPolicyGatePolicyRef,
    MissingPolicyGateGateInputRef,
    MissingPolicyGateSideEffectReceiptProposalRef,
    MissingPolicyGateAdapterInvocationReceiptRef,
    MissingPolicyGatePayloadRef,
    MissingPolicyGateProofRequirementRef,
    UnsafePolicyGatePreflightRef,
    PolicyGatePolicyRefMismatch,
    PolicyGateSideEffectReceiptProposalRefMismatch,
    PolicyGatePayloadRefMismatch,
    MissingPolicyGateRequestPrecondition,
    PolicyGatePreflightHasSideEffects,
    PolicyGatePreflightBlocked,
    MissingSideEffectReceiptWriterPreflightId,
    MissingSideEffectReceiptPolicyGatePreflightRef,
    MissingSideEffectReceiptTargetRef,
    MissingSideEffectReceiptStorageRef,
    MissingSideEffectReceiptOperationEvidenceRef,
    MissingSideEffectReceiptIdempotencyRef,
    MissingSideEffectReceiptRollbackRef,
    MissingSideEffectReceiptErrorRef,
    MissingSideEffectReceiptAdapterInvocationReceiptRef,
    MissingSideEffectReceiptPayloadRef,
    UnsafeSideEffectReceiptWriterRef,
    MissingSideEffectReceiptPolicyGateRequirement,
    SideEffectReceiptPolicyGatePreflightRefMismatch,
    SideEffectReceiptTargetRefMismatch,
    SideEffectReceiptAdapterInvocationReceiptRefMismatch,
    SideEffectReceiptPayloadRefMismatch,
    SideEffectReceiptWriterPreflightHasSideEffects,
    SideEffectReceiptWriterPreflightBlocked,
    MissingSideEffectReceiptWriterActiveBehaviorPreflightRequirement,
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
            Self::SideEffectRequestProposalBlocked => "side_effect_request_proposal_blocked",
            Self::MissingPolicyGatePreflightId => "missing_policy_gate_preflight_id",
            Self::MissingPolicyGatePolicyRef => "missing_policy_gate_policy_ref",
            Self::MissingPolicyGateGateInputRef => "missing_policy_gate_gate_input_ref",
            Self::MissingPolicyGateSideEffectReceiptProposalRef => {
                "missing_policy_gate_side_effect_receipt_proposal_ref"
            }
            Self::MissingPolicyGateAdapterInvocationReceiptRef => {
                "missing_policy_gate_adapter_invocation_receipt_ref"
            }
            Self::MissingPolicyGatePayloadRef => "missing_policy_gate_payload_ref",
            Self::MissingPolicyGateProofRequirementRef => {
                "missing_policy_gate_proof_requirement_ref"
            }
            Self::UnsafePolicyGatePreflightRef => "unsafe_policy_gate_preflight_ref",
            Self::PolicyGatePolicyRefMismatch => "policy_gate_policy_ref_mismatch",
            Self::PolicyGateSideEffectReceiptProposalRefMismatch => {
                "policy_gate_side_effect_receipt_proposal_ref_mismatch"
            }
            Self::PolicyGatePayloadRefMismatch => "policy_gate_payload_ref_mismatch",
            Self::MissingPolicyGateRequestPrecondition => {
                "missing_policy_gate_request_precondition"
            }
            Self::PolicyGatePreflightHasSideEffects => "policy_gate_preflight_has_side_effects",
            Self::PolicyGatePreflightBlocked => "policy_gate_preflight_blocked",
            Self::MissingSideEffectReceiptWriterPreflightId => {
                "missing_side_effect_receipt_writer_preflight_id"
            }
            Self::MissingSideEffectReceiptPolicyGatePreflightRef => {
                "missing_side_effect_receipt_policy_gate_preflight_ref"
            }
            Self::MissingSideEffectReceiptTargetRef => "missing_side_effect_receipt_target_ref",
            Self::MissingSideEffectReceiptStorageRef => "missing_side_effect_receipt_storage_ref",
            Self::MissingSideEffectReceiptOperationEvidenceRef => {
                "missing_side_effect_receipt_operation_evidence_ref"
            }
            Self::MissingSideEffectReceiptIdempotencyRef => {
                "missing_side_effect_receipt_idempotency_ref"
            }
            Self::MissingSideEffectReceiptRollbackRef => "missing_side_effect_receipt_rollback_ref",
            Self::MissingSideEffectReceiptErrorRef => "missing_side_effect_receipt_error_ref",
            Self::MissingSideEffectReceiptAdapterInvocationReceiptRef => {
                "missing_side_effect_receipt_adapter_invocation_receipt_ref"
            }
            Self::MissingSideEffectReceiptPayloadRef => "missing_side_effect_receipt_payload_ref",
            Self::UnsafeSideEffectReceiptWriterRef => "unsafe_side_effect_receipt_writer_ref",
            Self::MissingSideEffectReceiptPolicyGateRequirement => {
                "missing_side_effect_receipt_policy_gate_requirement"
            }
            Self::SideEffectReceiptPolicyGatePreflightRefMismatch => {
                "side_effect_receipt_policy_gate_preflight_ref_mismatch"
            }
            Self::SideEffectReceiptTargetRefMismatch => "side_effect_receipt_target_ref_mismatch",
            Self::SideEffectReceiptAdapterInvocationReceiptRefMismatch => {
                "side_effect_receipt_adapter_invocation_receipt_ref_mismatch"
            }
            Self::SideEffectReceiptPayloadRefMismatch => "side_effect_receipt_payload_ref_mismatch",
            Self::SideEffectReceiptWriterPreflightHasSideEffects => {
                "side_effect_receipt_writer_preflight_has_side_effects"
            }
            Self::SideEffectReceiptWriterPreflightBlocked => {
                "side_effect_receipt_writer_preflight_blocked"
            }
            Self::MissingSideEffectReceiptWriterActiveBehaviorPreflightRequirement => {
                "missing_side_effect_receipt_writer_active_behavior_preflight_requirement"
            }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModulePolicyGatePreflightRequirement {
    ReadySideEffectRequestProposal,
    HostPolicyRef,
    GateInputRef,
    SideEffectReceiptProposalRef,
    AdapterInvocationReceiptRef,
    PayloadRef,
    ProofRequirementRef,
}

impl ModulePolicyGatePreflightRequirement {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReadySideEffectRequestProposal => "ready_side_effect_request_proposal",
            Self::HostPolicyRef => "host_policy_ref",
            Self::GateInputRef => "gate_input_ref",
            Self::SideEffectReceiptProposalRef => "side_effect_receipt_proposal_ref",
            Self::AdapterInvocationReceiptRef => "adapter_invocation_receipt_ref",
            Self::PayloadRef => "payload_ref",
            Self::ProofRequirementRef => "proof_requirement_ref",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModulePolicyGatePreflightBoundaryFlags {
    pub invokes_policy_engine: bool,
    pub invokes_gate: bool,
    pub writes_gate_decision: bool,
    pub writes_proofpack: bool,
    pub invokes_adapter: bool,
    pub performs_external_side_effect: bool,
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
    pub creates_acceptance_claim: bool,
}

impl ModulePolicyGatePreflightBoundaryFlags {
    pub const fn pure_preflight() -> Self {
        Self {
            invokes_policy_engine: false,
            invokes_gate: false,
            writes_gate_decision: false,
            writes_proofpack: false,
            invokes_adapter: false,
            performs_external_side_effect: false,
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
            creates_acceptance_claim: false,
        }
    }

    pub fn all_side_effect_flags_false(self) -> bool {
        !self.invokes_policy_engine
            && !self.invokes_gate
            && !self.writes_gate_decision
            && !self.writes_proofpack
            && !self.invokes_adapter
            && !self.performs_external_side_effect
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
            && !self.creates_acceptance_claim
    }
}

pub const MODULE_HOST_PURE_POLICY_GATE_PREFLIGHT_BOUNDARY_FLAGS:
    ModulePolicyGatePreflightBoundaryFlags =
    ModulePolicyGatePreflightBoundaryFlags::pure_preflight();

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModulePolicyGatePreflightDraft {
    pub preflight_id: String,
    pub policy_ref: String,
    pub gate_input_ref: String,
    pub side_effect_receipt_proposal_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub proof_requirement_ref: String,
    pub boundary_flags: ModulePolicyGatePreflightBoundaryFlags,
}

impl ModulePolicyGatePreflightDraft {
    pub fn new(preflight_id: impl Into<String>) -> Self {
        Self {
            preflight_id: preflight_id.into(),
            policy_ref: String::new(),
            gate_input_ref: String::new(),
            side_effect_receipt_proposal_ref: String::new(),
            adapter_invocation_receipt_ref: String::new(),
            payload_ref: String::new(),
            proof_requirement_ref: String::new(),
            boundary_flags: ModulePolicyGatePreflightBoundaryFlags::pure_preflight(),
        }
    }

    pub fn with_policy_ref(mut self, policy_ref: impl Into<String>) -> Self {
        self.policy_ref = policy_ref.into();
        self
    }

    pub fn with_gate_input_ref(mut self, gate_input_ref: impl Into<String>) -> Self {
        self.gate_input_ref = gate_input_ref.into();
        self
    }

    pub fn with_side_effect_receipt_proposal_ref(
        mut self,
        side_effect_receipt_proposal_ref: impl Into<String>,
    ) -> Self {
        self.side_effect_receipt_proposal_ref = side_effect_receipt_proposal_ref.into();
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

    pub fn with_proof_requirement_ref(mut self, proof_requirement_ref: impl Into<String>) -> Self {
        self.proof_requirement_ref = proof_requirement_ref.into();
        self
    }

    pub fn with_boundary_flags(
        mut self,
        boundary_flags: ModulePolicyGatePreflightBoundaryFlags,
    ) -> Self {
        self.boundary_flags = boundary_flags;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModulePolicyGatePreflight {
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
    pub preflight_id: String,
    pub policy_ref: String,
    pub gate_input_ref: String,
    pub side_effect_receipt_proposal_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub proof_requirement_ref: String,
    pub required_requirements: Vec<ModulePolicyGatePreflightRequirement>,
    pub covered_requirements: Vec<ModulePolicyGatePreflightRequirement>,
    pub findings: Vec<ModuleHostFinding>,
    pub boundary_flags: ModulePolicyGatePreflightBoundaryFlags,
}

impl ModulePolicyGatePreflight {
    pub fn has_blockers(&self) -> bool {
        !self.findings.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleSideEffectReceiptWriterPreflightRequirement {
    ReadyPolicyGatePreflight,
    PolicyGatePreflightRef,
    ReceiptTargetRef,
    StorageRef,
    OperationEvidenceRef,
    IdempotencyRef,
    RollbackRef,
    ErrorRef,
    AdapterInvocationReceiptRef,
    PayloadRef,
}

impl ModuleSideEffectReceiptWriterPreflightRequirement {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReadyPolicyGatePreflight => "ready_policy_gate_preflight",
            Self::PolicyGatePreflightRef => "policy_gate_preflight_ref",
            Self::ReceiptTargetRef => "receipt_target_ref",
            Self::StorageRef => "storage_ref",
            Self::OperationEvidenceRef => "operation_evidence_ref",
            Self::IdempotencyRef => "idempotency_ref",
            Self::RollbackRef => "rollback_ref",
            Self::ErrorRef => "error_ref",
            Self::AdapterInvocationReceiptRef => "adapter_invocation_receipt_ref",
            Self::PayloadRef => "payload_ref",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleSideEffectReceiptWriterPreflightBoundaryFlags {
    pub creates_receipt: bool,
    pub writes_receipt: bool,
    pub writes_event_log: bool,
    pub reads_files: bool,
    pub writes_files: bool,
    pub calls_external_apis: bool,
    pub opens_browser: bool,
    pub reads_credentials: bool,
    pub invokes_adapter: bool,
    pub invokes_policy_engine: bool,
    pub invokes_gate: bool,
    pub writes_gate_decision: bool,
    pub writes_proofpack: bool,
    pub performs_external_side_effect: bool,
    pub publishes: bool,
    pub comments: bool,
    pub creates_pull_request: bool,
    pub creates_acceptance_claim: bool,
}

impl ModuleSideEffectReceiptWriterPreflightBoundaryFlags {
    pub const fn pure_preflight() -> Self {
        Self {
            creates_receipt: false,
            writes_receipt: false,
            writes_event_log: false,
            reads_files: false,
            writes_files: false,
            calls_external_apis: false,
            opens_browser: false,
            reads_credentials: false,
            invokes_adapter: false,
            invokes_policy_engine: false,
            invokes_gate: false,
            writes_gate_decision: false,
            writes_proofpack: false,
            performs_external_side_effect: false,
            publishes: false,
            comments: false,
            creates_pull_request: false,
            creates_acceptance_claim: false,
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
            && !self.invokes_adapter
            && !self.invokes_policy_engine
            && !self.invokes_gate
            && !self.writes_gate_decision
            && !self.writes_proofpack
            && !self.performs_external_side_effect
            && !self.publishes
            && !self.comments
            && !self.creates_pull_request
            && !self.creates_acceptance_claim
    }
}

pub const MODULE_HOST_PURE_SIDE_EFFECT_RECEIPT_WRITER_PREFLIGHT_BOUNDARY_FLAGS:
    ModuleSideEffectReceiptWriterPreflightBoundaryFlags =
    ModuleSideEffectReceiptWriterPreflightBoundaryFlags::pure_preflight();

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleSideEffectReceiptWriterPreflightDraft {
    pub preflight_id: String,
    pub policy_gate_preflight_ref: String,
    pub receipt_target_ref: String,
    pub storage_ref: String,
    pub operation_evidence_ref: String,
    pub idempotency_ref: String,
    pub rollback_ref: String,
    pub error_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub boundary_flags: ModuleSideEffectReceiptWriterPreflightBoundaryFlags,
}

impl ModuleSideEffectReceiptWriterPreflightDraft {
    pub fn new(preflight_id: impl Into<String>) -> Self {
        Self {
            preflight_id: preflight_id.into(),
            policy_gate_preflight_ref: String::new(),
            receipt_target_ref: String::new(),
            storage_ref: String::new(),
            operation_evidence_ref: String::new(),
            idempotency_ref: String::new(),
            rollback_ref: String::new(),
            error_ref: String::new(),
            adapter_invocation_receipt_ref: String::new(),
            payload_ref: String::new(),
            boundary_flags: ModuleSideEffectReceiptWriterPreflightBoundaryFlags::pure_preflight(),
        }
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

    pub fn with_storage_ref(mut self, storage_ref: impl Into<String>) -> Self {
        self.storage_ref = storage_ref.into();
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

    pub fn with_boundary_flags(
        mut self,
        boundary_flags: ModuleSideEffectReceiptWriterPreflightBoundaryFlags,
    ) -> Self {
        self.boundary_flags = boundary_flags;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleSideEffectReceiptWriterPreflight {
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
    pub preflight_id: String,
    pub policy_gate_preflight_ref: String,
    pub receipt_target_ref: String,
    pub storage_ref: String,
    pub operation_evidence_ref: String,
    pub idempotency_ref: String,
    pub rollback_ref: String,
    pub error_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub required_requirements: Vec<ModuleSideEffectReceiptWriterPreflightRequirement>,
    pub covered_requirements: Vec<ModuleSideEffectReceiptWriterPreflightRequirement>,
    pub findings: Vec<ModuleHostFinding>,
    pub boundary_flags: ModuleSideEffectReceiptWriterPreflightBoundaryFlags,
}

impl ModuleSideEffectReceiptWriterPreflight {
    pub fn has_blockers(&self) -> bool {
        !self.findings.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleSideEffectReceiptWriterModeledStep {
    ReceiptTargetCheck,
    ReceiptWrite,
    OperationEvidenceRecord,
    RollbackVisibility,
    ErrorVisibility,
}

impl ModuleSideEffectReceiptWriterModeledStep {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReceiptTargetCheck => "receipt_target_check",
            Self::ReceiptWrite => "receipt_write",
            Self::OperationEvidenceRecord => "operation_evidence_record",
            Self::RollbackVisibility => "rollback_visibility",
            Self::ErrorVisibility => "error_visibility",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleSideEffectReceiptWriterStepStatus {
    NotSelected,
    NotAttempted,
    Completed,
    Failed,
    Skipped,
}

impl ModuleSideEffectReceiptWriterStepStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotSelected => "not_selected",
            Self::NotAttempted => "not_attempted",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Skipped => "skipped",
        }
    }

    pub fn is_attempted(self) -> bool {
        matches!(self, Self::Completed | Self::Failed)
    }

    pub fn is_completed(self) -> bool {
        self == Self::Completed
    }

    pub fn is_failed(self) -> bool {
        self == Self::Failed
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleSideEffectReceiptWriterOutcome {
    PlannedOnly,
    PreflightFailed,
    Written,
    Idempotent,
    Conflict,
    WriteFailed,
    PartialOrAmbiguous,
}

impl ModuleSideEffectReceiptWriterOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PlannedOnly => "planned_only",
            Self::PreflightFailed => "preflight_failed",
            Self::Written => "written",
            Self::Idempotent => "idempotent",
            Self::Conflict => "conflict",
            Self::WriteFailed => "write_failed",
            Self::PartialOrAmbiguous => "partial_or_ambiguous",
        }
    }

    pub fn is_terminal_success(self) -> bool {
        matches!(self, Self::Written | Self::Idempotent)
    }

    pub fn is_failed(self) -> bool {
        matches!(
            self,
            Self::PreflightFailed | Self::Conflict | Self::WriteFailed | Self::PartialOrAmbiguous
        )
    }

    pub fn needs_error_visibility(self) -> bool {
        matches!(
            self,
            Self::Conflict | Self::WriteFailed | Self::PartialOrAmbiguous
        )
    }

    pub fn needs_rollback_visibility(self) -> bool {
        matches!(self, Self::WriteFailed | Self::PartialOrAmbiguous)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleSideEffectReceiptWriterObservedTargetState {
    NotChecked,
    Missing,
    ExistsMatching,
    ExistsDifferent,
    AmbiguousPartial,
}

impl ModuleSideEffectReceiptWriterObservedTargetState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotChecked => "not_checked",
            Self::Missing => "missing",
            Self::ExistsMatching => "exists_matching",
            Self::ExistsDifferent => "exists_different",
            Self::AmbiguousPartial => "ambiguous_partial",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleSideEffectReceiptWriterObservedWriteResult {
    NotAttempted,
    Written,
    WriteFailed,
    PartialOrAmbiguous,
}

impl ModuleSideEffectReceiptWriterObservedWriteResult {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotAttempted => "not_attempted",
            Self::Written => "written",
            Self::WriteFailed => "write_failed",
            Self::PartialOrAmbiguous => "partial_or_ambiguous",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleSideEffectReceiptWriterObservation {
    pub target_state: ModuleSideEffectReceiptWriterObservedTargetState,
    pub write_result: ModuleSideEffectReceiptWriterObservedWriteResult,
    pub operation_evidence_status: ModuleSideEffectReceiptWriterStepStatus,
    pub rollback_status: ModuleSideEffectReceiptWriterStepStatus,
    pub error_status: ModuleSideEffectReceiptWriterStepStatus,
    pub boundary_notes: Vec<String>,
}

impl ModuleSideEffectReceiptWriterObservation {
    pub fn new(
        target_state: ModuleSideEffectReceiptWriterObservedTargetState,
        write_result: ModuleSideEffectReceiptWriterObservedWriteResult,
        operation_evidence_status: ModuleSideEffectReceiptWriterStepStatus,
        rollback_status: ModuleSideEffectReceiptWriterStepStatus,
        error_status: ModuleSideEffectReceiptWriterStepStatus,
        boundary_notes: Vec<String>,
    ) -> Self {
        Self {
            target_state,
            write_result,
            operation_evidence_status,
            rollback_status,
            error_status,
            boundary_notes,
        }
    }

    pub fn target_missing_written() -> Self {
        Self::new(
            ModuleSideEffectReceiptWriterObservedTargetState::Missing,
            ModuleSideEffectReceiptWriterObservedWriteResult::Written,
            ModuleSideEffectReceiptWriterStepStatus::Completed,
            ModuleSideEffectReceiptWriterStepStatus::Skipped,
            ModuleSideEffectReceiptWriterStepStatus::Skipped,
            vec!["Receipt writer active behavior observation is modeled only.".to_owned()],
        )
    }

    pub fn target_exists_matching() -> Self {
        Self::new(
            ModuleSideEffectReceiptWriterObservedTargetState::ExistsMatching,
            ModuleSideEffectReceiptWriterObservedWriteResult::NotAttempted,
            ModuleSideEffectReceiptWriterStepStatus::Completed,
            ModuleSideEffectReceiptWriterStepStatus::Skipped,
            ModuleSideEffectReceiptWriterStepStatus::Skipped,
            vec!["Idempotent receipt writer observation is modeled only.".to_owned()],
        )
    }

    pub fn target_exists_different() -> Self {
        Self::new(
            ModuleSideEffectReceiptWriterObservedTargetState::ExistsDifferent,
            ModuleSideEffectReceiptWriterObservedWriteResult::NotAttempted,
            ModuleSideEffectReceiptWriterStepStatus::Completed,
            ModuleSideEffectReceiptWriterStepStatus::Skipped,
            ModuleSideEffectReceiptWriterStepStatus::Completed,
            vec!["Conflicting receipt writer observation is modeled only.".to_owned()],
        )
    }

    pub fn write_failed() -> Self {
        Self::new(
            ModuleSideEffectReceiptWriterObservedTargetState::Missing,
            ModuleSideEffectReceiptWriterObservedWriteResult::WriteFailed,
            ModuleSideEffectReceiptWriterStepStatus::Completed,
            ModuleSideEffectReceiptWriterStepStatus::Completed,
            ModuleSideEffectReceiptWriterStepStatus::Completed,
            vec!["Failed receipt writer observation is modeled only.".to_owned()],
        )
    }

    pub fn partial_or_ambiguous() -> Self {
        Self::new(
            ModuleSideEffectReceiptWriterObservedTargetState::AmbiguousPartial,
            ModuleSideEffectReceiptWriterObservedWriteResult::PartialOrAmbiguous,
            ModuleSideEffectReceiptWriterStepStatus::Completed,
            ModuleSideEffectReceiptWriterStepStatus::Completed,
            ModuleSideEffectReceiptWriterStepStatus::Completed,
            vec!["Partial receipt writer observation is modeled only.".to_owned()],
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleSideEffectReceiptWriterActiveBehaviorBoundaryFlags {
    pub models_active_behavior: bool,
    pub models_selected_steps: bool,
    pub models_observed_outcome: bool,
    pub models_error_rollback_visibility: bool,
    pub creates_receipt: bool,
    pub writes_receipt: bool,
    pub writes_event_log: bool,
    pub reads_files: bool,
    pub writes_files: bool,
    pub persists_operation_evidence: bool,
    pub calls_external_apis: bool,
    pub opens_browser: bool,
    pub reads_credentials: bool,
    pub invokes_adapter: bool,
    pub invokes_policy_engine: bool,
    pub invokes_gate: bool,
    pub writes_gate_decision: bool,
    pub writes_proofpack: bool,
    pub performs_external_side_effect: bool,
    pub publishes: bool,
    pub comments: bool,
    pub creates_pull_request: bool,
    pub creates_acceptance_claim: bool,
    pub evidence_only: bool,
}

impl ModuleSideEffectReceiptWriterActiveBehaviorBoundaryFlags {
    pub const fn pure_model() -> Self {
        Self {
            models_active_behavior: true,
            models_selected_steps: true,
            models_observed_outcome: true,
            models_error_rollback_visibility: true,
            creates_receipt: false,
            writes_receipt: false,
            writes_event_log: false,
            reads_files: false,
            writes_files: false,
            persists_operation_evidence: false,
            calls_external_apis: false,
            opens_browser: false,
            reads_credentials: false,
            invokes_adapter: false,
            invokes_policy_engine: false,
            invokes_gate: false,
            writes_gate_decision: false,
            writes_proofpack: false,
            performs_external_side_effect: false,
            publishes: false,
            comments: false,
            creates_pull_request: false,
            creates_acceptance_claim: false,
            evidence_only: true,
        }
    }

    pub fn all_side_effect_flags_false(self) -> bool {
        !self.creates_receipt
            && !self.writes_receipt
            && !self.writes_event_log
            && !self.reads_files
            && !self.writes_files
            && !self.persists_operation_evidence
            && !self.calls_external_apis
            && !self.opens_browser
            && !self.reads_credentials
            && !self.invokes_adapter
            && !self.invokes_policy_engine
            && !self.invokes_gate
            && !self.writes_gate_decision
            && !self.writes_proofpack
            && !self.performs_external_side_effect
            && !self.publishes
            && !self.comments
            && !self.creates_pull_request
            && !self.creates_acceptance_claim
    }
}

pub const MODULE_HOST_PURE_SIDE_EFFECT_RECEIPT_WRITER_ACTIVE_BEHAVIOR_BOUNDARY_FLAGS:
    ModuleSideEffectReceiptWriterActiveBehaviorBoundaryFlags =
    ModuleSideEffectReceiptWriterActiveBehaviorBoundaryFlags::pure_model();

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleSideEffectReceiptWriterActiveBehavior {
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
    pub preflight_id: String,
    pub receipt_target_ref: String,
    pub storage_ref: String,
    pub operation_evidence_ref: String,
    pub idempotency_ref: String,
    pub rollback_ref: String,
    pub error_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub outcome: ModuleSideEffectReceiptWriterOutcome,
    pub selected_steps: Vec<ModuleSideEffectReceiptWriterModeledStep>,
    pub attempted_steps: Vec<ModuleSideEffectReceiptWriterModeledStep>,
    pub completed_steps: Vec<ModuleSideEffectReceiptWriterModeledStep>,
    pub failed_steps: Vec<ModuleSideEffectReceiptWriterModeledStep>,
    pub operation_evidence_status: ModuleSideEffectReceiptWriterStepStatus,
    pub rollback_status: ModuleSideEffectReceiptWriterStepStatus,
    pub error_status: ModuleSideEffectReceiptWriterStepStatus,
    pub observation: Option<ModuleSideEffectReceiptWriterObservation>,
    pub findings: Vec<ModuleHostFinding>,
    pub boundary_flags: ModuleSideEffectReceiptWriterActiveBehaviorBoundaryFlags,
}

impl ModuleSideEffectReceiptWriterActiveBehavior {
    pub fn has_blockers(&self) -> bool {
        !self.findings.is_empty()
    }

    pub fn is_evidence_only(&self) -> bool {
        self.boundary_flags.evidence_only
    }

    pub fn has_conflict(&self) -> bool {
        self.outcome == ModuleSideEffectReceiptWriterOutcome::Conflict
    }

    pub fn has_write_failure(&self) -> bool {
        self.outcome == ModuleSideEffectReceiptWriterOutcome::WriteFailed
    }

    pub fn has_partial_or_ambiguous_state(&self) -> bool {
        self.outcome == ModuleSideEffectReceiptWriterOutcome::PartialOrAmbiguous
    }

    pub fn rollback_visible(&self) -> bool {
        self.rollback_status == ModuleSideEffectReceiptWriterStepStatus::Completed
    }

    pub fn error_visible(&self) -> bool {
        self.error_status == ModuleSideEffectReceiptWriterStepStatus::Completed
    }

    pub fn selected_step_was_attempted(
        &self,
        step: ModuleSideEffectReceiptWriterModeledStep,
    ) -> bool {
        self.attempted_steps.contains(&step)
    }

    pub fn selected_step_completed(&self, step: ModuleSideEffectReceiptWriterModeledStep) -> bool {
        self.completed_steps.contains(&step)
    }

    pub fn selected_step_failed(&self, step: ModuleSideEffectReceiptWriterModeledStep) -> bool {
        self.failed_steps.contains(&step)
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

pub fn preflight_module_policy_gate(
    side_effect_proposal: &ModuleSideEffectRequestProposal,
    draft: &ModulePolicyGatePreflightDraft,
) -> ModulePolicyGatePreflight {
    let mut findings = Vec::new();
    let required_requirements = default_policy_gate_preflight_requirements();

    if side_effect_proposal.status != ModuleHostStatus::Ready || side_effect_proposal.has_blockers()
    {
        findings.push(ModuleHostFinding::new(
            ModuleHostFindingCode::SideEffectRequestProposalBlocked,
            "side-effect request proposal must be ready before policy gate preflight can be modeled",
        ));
    }

    for precondition in &side_effect_proposal.required_preconditions {
        if !side_effect_proposal
            .covered_preconditions
            .contains(precondition)
        {
            findings.push(ModuleHostFinding::new(
                ModuleHostFindingCode::MissingPolicyGateRequestPrecondition,
                "side-effect request proposal must cover all required preconditions",
            ));
            break;
        }
    }

    push_required_policy_gate_ref_finding(
        &mut findings,
        draft.preflight_id.as_str(),
        ModuleHostFindingCode::MissingPolicyGatePreflightId,
        "policy gate preflight id is required",
    );
    push_required_policy_gate_ref_finding(
        &mut findings,
        draft.policy_ref.as_str(),
        ModuleHostFindingCode::MissingPolicyGatePolicyRef,
        "policy ref is required",
    );
    push_required_policy_gate_ref_finding(
        &mut findings,
        draft.gate_input_ref.as_str(),
        ModuleHostFindingCode::MissingPolicyGateGateInputRef,
        "gate input ref is required",
    );
    push_required_policy_gate_ref_finding(
        &mut findings,
        draft.side_effect_receipt_proposal_ref.as_str(),
        ModuleHostFindingCode::MissingPolicyGateSideEffectReceiptProposalRef,
        "side-effect receipt proposal ref is required",
    );
    push_required_policy_gate_ref_finding(
        &mut findings,
        draft.adapter_invocation_receipt_ref.as_str(),
        ModuleHostFindingCode::MissingPolicyGateAdapterInvocationReceiptRef,
        "adapter invocation receipt ref is required",
    );
    push_required_policy_gate_ref_finding(
        &mut findings,
        draft.payload_ref.as_str(),
        ModuleHostFindingCode::MissingPolicyGatePayloadRef,
        "payload ref is required",
    );
    push_required_policy_gate_ref_finding(
        &mut findings,
        draft.proof_requirement_ref.as_str(),
        ModuleHostFindingCode::MissingPolicyGateProofRequirementRef,
        "proof requirement ref is required",
    );

    if !draft.policy_ref.trim().is_empty() && draft.policy_ref != side_effect_proposal.policy_ref {
        findings.push(ModuleHostFinding::for_input_ref(
            ModuleHostFindingCode::PolicyGatePolicyRefMismatch,
            draft.policy_ref.clone(),
            "policy gate preflight policy ref must match the side-effect request policy ref",
        ));
    }

    if !draft.side_effect_receipt_proposal_ref.trim().is_empty()
        && draft.side_effect_receipt_proposal_ref != side_effect_proposal.receipt_proposal_ref
    {
        findings.push(ModuleHostFinding::for_input_ref(
            ModuleHostFindingCode::PolicyGateSideEffectReceiptProposalRefMismatch,
            draft.side_effect_receipt_proposal_ref.clone(),
            "policy gate preflight receipt proposal ref must match the side-effect request receipt proposal ref",
        ));
    }

    if !draft.payload_ref.trim().is_empty() && draft.payload_ref != side_effect_proposal.payload_ref
    {
        findings.push(ModuleHostFinding::for_input_ref(
            ModuleHostFindingCode::PolicyGatePayloadRefMismatch,
            draft.payload_ref.clone(),
            "policy gate preflight payload ref must match the side-effect request payload ref",
        ));
    }

    if !draft.boundary_flags.all_side_effect_flags_false() {
        findings.push(ModuleHostFinding::new(
            ModuleHostFindingCode::PolicyGatePreflightHasSideEffects,
            "policy gate preflight must not invoke policy, gate, adapters, or side effects",
        ));
    }

    let status = if findings.is_empty() {
        ModuleHostStatus::Ready
    } else {
        ModuleHostStatus::Blocked
    };
    let covered_requirements = if status == ModuleHostStatus::Ready {
        required_requirements.clone()
    } else {
        Vec::new()
    };

    ModulePolicyGatePreflight {
        schema_version: MODULE_HOST_POLICY_GATE_PREFLIGHT_SCHEMA_VERSION,
        status,
        authority: ModuleHostAuthority::Advisory,
        module_id: side_effect_proposal.module_id.clone(),
        module_version: side_effect_proposal.module_version.clone(),
        contract_ref: side_effect_proposal.contract_ref.clone(),
        run_ref: side_effect_proposal.run_ref.clone(),
        project_ref: side_effect_proposal.project_ref.clone(),
        requested_operation: side_effect_proposal.requested_operation.clone(),
        request_id: side_effect_proposal.request_id.clone(),
        kind: side_effect_proposal.kind,
        preflight_id: draft.preflight_id.clone(),
        policy_ref: draft.policy_ref.clone(),
        gate_input_ref: draft.gate_input_ref.clone(),
        side_effect_receipt_proposal_ref: draft.side_effect_receipt_proposal_ref.clone(),
        adapter_invocation_receipt_ref: draft.adapter_invocation_receipt_ref.clone(),
        payload_ref: draft.payload_ref.clone(),
        proof_requirement_ref: draft.proof_requirement_ref.clone(),
        required_requirements,
        covered_requirements,
        findings,
        boundary_flags: MODULE_HOST_PURE_POLICY_GATE_PREFLIGHT_BOUNDARY_FLAGS,
    }
}

pub fn preflight_module_side_effect_receipt_writer(
    policy_gate_preflight: &ModulePolicyGatePreflight,
    draft: &ModuleSideEffectReceiptWriterPreflightDraft,
) -> ModuleSideEffectReceiptWriterPreflight {
    let mut findings = Vec::new();
    let required_requirements = default_side_effect_receipt_writer_preflight_requirements();

    if policy_gate_preflight.status != ModuleHostStatus::Ready
        || policy_gate_preflight.has_blockers()
    {
        findings.push(ModuleHostFinding::new(
            ModuleHostFindingCode::PolicyGatePreflightBlocked,
            "policy gate preflight must be ready before receipt writer preflight can be modeled",
        ));
    }

    for requirement in &policy_gate_preflight.required_requirements {
        if !policy_gate_preflight
            .covered_requirements
            .contains(requirement)
        {
            findings.push(ModuleHostFinding::new(
                ModuleHostFindingCode::MissingSideEffectReceiptPolicyGateRequirement,
                "policy gate preflight must cover all required requirements",
            ));
            break;
        }
    }

    push_required_receipt_writer_ref_finding(
        &mut findings,
        draft.preflight_id.as_str(),
        ModuleHostFindingCode::MissingSideEffectReceiptWriterPreflightId,
        "side-effect receipt writer preflight id is required",
    );
    push_required_receipt_writer_ref_finding(
        &mut findings,
        draft.policy_gate_preflight_ref.as_str(),
        ModuleHostFindingCode::MissingSideEffectReceiptPolicyGatePreflightRef,
        "policy gate preflight ref is required",
    );
    push_required_receipt_writer_ref_finding(
        &mut findings,
        draft.receipt_target_ref.as_str(),
        ModuleHostFindingCode::MissingSideEffectReceiptTargetRef,
        "side-effect receipt target ref is required",
    );
    push_required_receipt_writer_ref_finding(
        &mut findings,
        draft.storage_ref.as_str(),
        ModuleHostFindingCode::MissingSideEffectReceiptStorageRef,
        "side-effect receipt storage ref is required",
    );
    push_required_receipt_writer_ref_finding(
        &mut findings,
        draft.operation_evidence_ref.as_str(),
        ModuleHostFindingCode::MissingSideEffectReceiptOperationEvidenceRef,
        "side-effect receipt operation evidence ref is required",
    );
    push_required_receipt_writer_ref_finding(
        &mut findings,
        draft.idempotency_ref.as_str(),
        ModuleHostFindingCode::MissingSideEffectReceiptIdempotencyRef,
        "side-effect receipt idempotency ref is required",
    );
    push_required_receipt_writer_ref_finding(
        &mut findings,
        draft.rollback_ref.as_str(),
        ModuleHostFindingCode::MissingSideEffectReceiptRollbackRef,
        "side-effect receipt rollback ref is required",
    );
    push_required_receipt_writer_ref_finding(
        &mut findings,
        draft.error_ref.as_str(),
        ModuleHostFindingCode::MissingSideEffectReceiptErrorRef,
        "side-effect receipt error ref is required",
    );
    push_required_receipt_writer_ref_finding(
        &mut findings,
        draft.adapter_invocation_receipt_ref.as_str(),
        ModuleHostFindingCode::MissingSideEffectReceiptAdapterInvocationReceiptRef,
        "side-effect receipt adapter invocation receipt ref is required",
    );
    push_required_receipt_writer_ref_finding(
        &mut findings,
        draft.payload_ref.as_str(),
        ModuleHostFindingCode::MissingSideEffectReceiptPayloadRef,
        "side-effect receipt payload ref is required",
    );

    if !draft.policy_gate_preflight_ref.trim().is_empty()
        && draft.policy_gate_preflight_ref != policy_gate_preflight.preflight_id
    {
        findings.push(ModuleHostFinding::for_input_ref(
            ModuleHostFindingCode::SideEffectReceiptPolicyGatePreflightRefMismatch,
            draft.policy_gate_preflight_ref.clone(),
            "receipt writer policy gate preflight ref must match the policy gate preflight id",
        ));
    }

    if !draft.receipt_target_ref.trim().is_empty()
        && draft.receipt_target_ref != policy_gate_preflight.side_effect_receipt_proposal_ref
    {
        findings.push(ModuleHostFinding::for_input_ref(
            ModuleHostFindingCode::SideEffectReceiptTargetRefMismatch,
            draft.receipt_target_ref.clone(),
            "receipt writer target ref must match the policy gate side-effect receipt proposal ref",
        ));
    }

    if !draft.adapter_invocation_receipt_ref.trim().is_empty()
        && draft.adapter_invocation_receipt_ref
            != policy_gate_preflight.adapter_invocation_receipt_ref
    {
        findings.push(ModuleHostFinding::for_input_ref(
            ModuleHostFindingCode::SideEffectReceiptAdapterInvocationReceiptRefMismatch,
            draft.adapter_invocation_receipt_ref.clone(),
            "receipt writer adapter invocation receipt ref must match the policy gate ref",
        ));
    }

    if !draft.payload_ref.trim().is_empty()
        && draft.payload_ref != policy_gate_preflight.payload_ref
    {
        findings.push(ModuleHostFinding::for_input_ref(
            ModuleHostFindingCode::SideEffectReceiptPayloadRefMismatch,
            draft.payload_ref.clone(),
            "receipt writer payload ref must match the policy gate payload ref",
        ));
    }

    if !draft.boundary_flags.all_side_effect_flags_false() {
        findings.push(ModuleHostFinding::new(
            ModuleHostFindingCode::SideEffectReceiptWriterPreflightHasSideEffects,
            "side-effect receipt writer preflight must not write receipts or perform side effects",
        ));
    }

    let status = if findings.is_empty() {
        ModuleHostStatus::Ready
    } else {
        ModuleHostStatus::Blocked
    };
    let covered_requirements = if status == ModuleHostStatus::Ready {
        required_requirements.clone()
    } else {
        Vec::new()
    };

    ModuleSideEffectReceiptWriterPreflight {
        schema_version: MODULE_HOST_SIDE_EFFECT_RECEIPT_WRITER_PREFLIGHT_SCHEMA_VERSION,
        status,
        authority: ModuleHostAuthority::Advisory,
        module_id: policy_gate_preflight.module_id.clone(),
        module_version: policy_gate_preflight.module_version.clone(),
        contract_ref: policy_gate_preflight.contract_ref.clone(),
        run_ref: policy_gate_preflight.run_ref.clone(),
        project_ref: policy_gate_preflight.project_ref.clone(),
        requested_operation: policy_gate_preflight.requested_operation.clone(),
        request_id: policy_gate_preflight.request_id.clone(),
        kind: policy_gate_preflight.kind,
        preflight_id: draft.preflight_id.clone(),
        policy_gate_preflight_ref: draft.policy_gate_preflight_ref.clone(),
        receipt_target_ref: draft.receipt_target_ref.clone(),
        storage_ref: draft.storage_ref.clone(),
        operation_evidence_ref: draft.operation_evidence_ref.clone(),
        idempotency_ref: draft.idempotency_ref.clone(),
        rollback_ref: draft.rollback_ref.clone(),
        error_ref: draft.error_ref.clone(),
        adapter_invocation_receipt_ref: draft.adapter_invocation_receipt_ref.clone(),
        payload_ref: draft.payload_ref.clone(),
        required_requirements,
        covered_requirements,
        findings,
        boundary_flags: MODULE_HOST_PURE_SIDE_EFFECT_RECEIPT_WRITER_PREFLIGHT_BOUNDARY_FLAGS,
    }
}

pub fn model_module_side_effect_receipt_writer_active_behavior(
    preflight: &ModuleSideEffectReceiptWriterPreflight,
    observation: Option<&ModuleSideEffectReceiptWriterObservation>,
) -> ModuleSideEffectReceiptWriterActiveBehavior {
    let mut findings = Vec::new();

    if preflight.status != ModuleHostStatus::Ready || preflight.has_blockers() {
        findings.push(ModuleHostFinding::new(
            ModuleHostFindingCode::SideEffectReceiptWriterPreflightBlocked,
            "side-effect receipt writer preflight must be ready before active behavior can be modeled",
        ));
    }

    for requirement in &preflight.required_requirements {
        if !preflight.covered_requirements.contains(requirement) {
            findings.push(ModuleHostFinding::new(
                ModuleHostFindingCode::MissingSideEffectReceiptWriterActiveBehaviorPreflightRequirement,
                "side-effect receipt writer preflight must cover all required requirements",
            ));
            break;
        }
    }

    let status = if findings.is_empty() {
        ModuleHostStatus::Ready
    } else {
        ModuleHostStatus::Blocked
    };
    let outcome = if status == ModuleHostStatus::Ready {
        receipt_writer_active_behavior_outcome(observation)
    } else {
        ModuleSideEffectReceiptWriterOutcome::PreflightFailed
    };
    let selected_steps = if status == ModuleHostStatus::Ready {
        default_side_effect_receipt_writer_modeled_steps()
    } else {
        Vec::new()
    };
    let attempted_steps = receipt_writer_steps_with_status(
        &selected_steps,
        outcome,
        observation,
        ModuleSideEffectReceiptWriterStepStatus::is_attempted,
    );
    let completed_steps = receipt_writer_steps_with_status(
        &selected_steps,
        outcome,
        observation,
        ModuleSideEffectReceiptWriterStepStatus::is_completed,
    );
    let failed_steps = receipt_writer_steps_with_status(
        &selected_steps,
        outcome,
        observation,
        ModuleSideEffectReceiptWriterStepStatus::is_failed,
    );
    let operation_evidence_status = receipt_writer_step_status(
        outcome,
        observation,
        ModuleSideEffectReceiptWriterModeledStep::OperationEvidenceRecord,
    );
    let rollback_status = receipt_writer_step_status(
        outcome,
        observation,
        ModuleSideEffectReceiptWriterModeledStep::RollbackVisibility,
    );
    let error_status = receipt_writer_step_status(
        outcome,
        observation,
        ModuleSideEffectReceiptWriterModeledStep::ErrorVisibility,
    );

    ModuleSideEffectReceiptWriterActiveBehavior {
        schema_version: MODULE_HOST_SIDE_EFFECT_RECEIPT_WRITER_ACTIVE_BEHAVIOR_SCHEMA_VERSION,
        status,
        authority: ModuleHostAuthority::Advisory,
        module_id: preflight.module_id.clone(),
        module_version: preflight.module_version.clone(),
        contract_ref: preflight.contract_ref.clone(),
        run_ref: preflight.run_ref.clone(),
        project_ref: preflight.project_ref.clone(),
        requested_operation: preflight.requested_operation.clone(),
        request_id: preflight.request_id.clone(),
        kind: preflight.kind,
        preflight_id: preflight.preflight_id.clone(),
        receipt_target_ref: preflight.receipt_target_ref.clone(),
        storage_ref: preflight.storage_ref.clone(),
        operation_evidence_ref: preflight.operation_evidence_ref.clone(),
        idempotency_ref: preflight.idempotency_ref.clone(),
        rollback_ref: preflight.rollback_ref.clone(),
        error_ref: preflight.error_ref.clone(),
        adapter_invocation_receipt_ref: preflight.adapter_invocation_receipt_ref.clone(),
        payload_ref: preflight.payload_ref.clone(),
        outcome,
        selected_steps,
        attempted_steps,
        completed_steps,
        failed_steps,
        operation_evidence_status,
        rollback_status,
        error_status,
        observation: observation.cloned(),
        findings,
        boundary_flags: MODULE_HOST_PURE_SIDE_EFFECT_RECEIPT_WRITER_ACTIVE_BEHAVIOR_BOUNDARY_FLAGS,
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

fn default_policy_gate_preflight_requirements() -> Vec<ModulePolicyGatePreflightRequirement> {
    vec![
        ModulePolicyGatePreflightRequirement::ReadySideEffectRequestProposal,
        ModulePolicyGatePreflightRequirement::HostPolicyRef,
        ModulePolicyGatePreflightRequirement::GateInputRef,
        ModulePolicyGatePreflightRequirement::SideEffectReceiptProposalRef,
        ModulePolicyGatePreflightRequirement::AdapterInvocationReceiptRef,
        ModulePolicyGatePreflightRequirement::PayloadRef,
        ModulePolicyGatePreflightRequirement::ProofRequirementRef,
    ]
}

fn default_side_effect_receipt_writer_preflight_requirements(
) -> Vec<ModuleSideEffectReceiptWriterPreflightRequirement> {
    vec![
        ModuleSideEffectReceiptWriterPreflightRequirement::ReadyPolicyGatePreflight,
        ModuleSideEffectReceiptWriterPreflightRequirement::PolicyGatePreflightRef,
        ModuleSideEffectReceiptWriterPreflightRequirement::ReceiptTargetRef,
        ModuleSideEffectReceiptWriterPreflightRequirement::StorageRef,
        ModuleSideEffectReceiptWriterPreflightRequirement::OperationEvidenceRef,
        ModuleSideEffectReceiptWriterPreflightRequirement::IdempotencyRef,
        ModuleSideEffectReceiptWriterPreflightRequirement::RollbackRef,
        ModuleSideEffectReceiptWriterPreflightRequirement::ErrorRef,
        ModuleSideEffectReceiptWriterPreflightRequirement::AdapterInvocationReceiptRef,
        ModuleSideEffectReceiptWriterPreflightRequirement::PayloadRef,
    ]
}

fn default_side_effect_receipt_writer_modeled_steps(
) -> Vec<ModuleSideEffectReceiptWriterModeledStep> {
    vec![
        ModuleSideEffectReceiptWriterModeledStep::ReceiptTargetCheck,
        ModuleSideEffectReceiptWriterModeledStep::ReceiptWrite,
        ModuleSideEffectReceiptWriterModeledStep::OperationEvidenceRecord,
        ModuleSideEffectReceiptWriterModeledStep::RollbackVisibility,
        ModuleSideEffectReceiptWriterModeledStep::ErrorVisibility,
    ]
}

fn receipt_writer_active_behavior_outcome(
    observation: Option<&ModuleSideEffectReceiptWriterObservation>,
) -> ModuleSideEffectReceiptWriterOutcome {
    let Some(observation) = observation else {
        return ModuleSideEffectReceiptWriterOutcome::PlannedOnly;
    };

    if observation.target_state
        == ModuleSideEffectReceiptWriterObservedTargetState::AmbiguousPartial
        || observation.write_result
            == ModuleSideEffectReceiptWriterObservedWriteResult::PartialOrAmbiguous
    {
        return ModuleSideEffectReceiptWriterOutcome::PartialOrAmbiguous;
    }

    if observation.target_state == ModuleSideEffectReceiptWriterObservedTargetState::ExistsDifferent
    {
        return ModuleSideEffectReceiptWriterOutcome::Conflict;
    }

    if observation.target_state == ModuleSideEffectReceiptWriterObservedTargetState::ExistsMatching
    {
        return ModuleSideEffectReceiptWriterOutcome::Idempotent;
    }

    if observation.write_result == ModuleSideEffectReceiptWriterObservedWriteResult::WriteFailed {
        return ModuleSideEffectReceiptWriterOutcome::WriteFailed;
    }

    if observation.write_result == ModuleSideEffectReceiptWriterObservedWriteResult::Written {
        return ModuleSideEffectReceiptWriterOutcome::Written;
    }

    ModuleSideEffectReceiptWriterOutcome::PlannedOnly
}

fn receipt_writer_steps_with_status(
    selected_steps: &[ModuleSideEffectReceiptWriterModeledStep],
    outcome: ModuleSideEffectReceiptWriterOutcome,
    observation: Option<&ModuleSideEffectReceiptWriterObservation>,
    predicate: fn(ModuleSideEffectReceiptWriterStepStatus) -> bool,
) -> Vec<ModuleSideEffectReceiptWriterModeledStep> {
    selected_steps
        .iter()
        .copied()
        .filter(|step| predicate(receipt_writer_step_status(outcome, observation, *step)))
        .collect()
}

fn receipt_writer_step_status(
    outcome: ModuleSideEffectReceiptWriterOutcome,
    observation: Option<&ModuleSideEffectReceiptWriterObservation>,
    step: ModuleSideEffectReceiptWriterModeledStep,
) -> ModuleSideEffectReceiptWriterStepStatus {
    if outcome == ModuleSideEffectReceiptWriterOutcome::PreflightFailed {
        return ModuleSideEffectReceiptWriterStepStatus::NotSelected;
    }

    let Some(observation) = observation else {
        return ModuleSideEffectReceiptWriterStepStatus::NotAttempted;
    };

    match step {
        ModuleSideEffectReceiptWriterModeledStep::ReceiptTargetCheck => {
            match observation.target_state {
                ModuleSideEffectReceiptWriterObservedTargetState::NotChecked => {
                    ModuleSideEffectReceiptWriterStepStatus::NotAttempted
                }
                ModuleSideEffectReceiptWriterObservedTargetState::AmbiguousPartial => {
                    ModuleSideEffectReceiptWriterStepStatus::Failed
                }
                ModuleSideEffectReceiptWriterObservedTargetState::Missing
                | ModuleSideEffectReceiptWriterObservedTargetState::ExistsMatching
                | ModuleSideEffectReceiptWriterObservedTargetState::ExistsDifferent => {
                    ModuleSideEffectReceiptWriterStepStatus::Completed
                }
            }
        }
        ModuleSideEffectReceiptWriterModeledStep::ReceiptWrite => match outcome {
            ModuleSideEffectReceiptWriterOutcome::Written => {
                ModuleSideEffectReceiptWriterStepStatus::Completed
            }
            ModuleSideEffectReceiptWriterOutcome::WriteFailed
            | ModuleSideEffectReceiptWriterOutcome::PartialOrAmbiguous => {
                ModuleSideEffectReceiptWriterStepStatus::Failed
            }
            ModuleSideEffectReceiptWriterOutcome::Idempotent
            | ModuleSideEffectReceiptWriterOutcome::Conflict => {
                ModuleSideEffectReceiptWriterStepStatus::Skipped
            }
            ModuleSideEffectReceiptWriterOutcome::PlannedOnly
            | ModuleSideEffectReceiptWriterOutcome::PreflightFailed => {
                ModuleSideEffectReceiptWriterStepStatus::NotAttempted
            }
        },
        ModuleSideEffectReceiptWriterModeledStep::OperationEvidenceRecord => {
            observation.operation_evidence_status
        }
        ModuleSideEffectReceiptWriterModeledStep::RollbackVisibility => observation.rollback_status,
        ModuleSideEffectReceiptWriterModeledStep::ErrorVisibility => observation.error_status,
    }
}

fn push_required_receipt_writer_ref_finding(
    findings: &mut Vec<ModuleHostFinding>,
    value: &str,
    missing_code: ModuleHostFindingCode,
    missing_message: &'static str,
) {
    if value.trim().is_empty() {
        findings.push(ModuleHostFinding::new(missing_code, missing_message));
    } else if !is_safe_ref(value) {
        findings.push(ModuleHostFinding::for_input_ref(
            ModuleHostFindingCode::UnsafeSideEffectReceiptWriterRef,
            value.to_owned(),
            "side-effect receipt writer refs must be explicit repo-relative refs",
        ));
    }
}

fn push_required_policy_gate_ref_finding(
    findings: &mut Vec<ModuleHostFinding>,
    value: &str,
    missing_code: ModuleHostFindingCode,
    missing_message: &'static str,
) {
    if value.trim().is_empty() {
        findings.push(ModuleHostFinding::new(missing_code, missing_message));
    } else if !is_safe_ref(value) {
        findings.push(ModuleHostFinding::for_input_ref(
            ModuleHostFindingCode::UnsafePolicyGatePreflightRef,
            value.to_owned(),
            "policy gate preflight refs must be explicit repo-relative refs",
        ));
    }
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
        model_module_side_effect_receipt_writer_active_behavior, preflight_module_invocation,
        preflight_module_policy_gate, preflight_module_side_effect_receipt_writer,
        propose_module_assessment_receipt, propose_module_side_effect_request,
        wrap_module_assessment, ModuleCapabilityGrant, ModuleHostFindingCode, ModuleHostStatus,
        ModuleInvocationEnvelope, ModuleOutputAuthority, ModuleOutputBoundaryFlags,
        ModuleOutputStatus, ModuleOutputSummary, ModulePolicyGatePreflightBoundaryFlags,
        ModulePolicyGatePreflightDraft, ModulePolicyGatePreflightRequirement, ModulePrivacyPolicy,
        ModuleReceiptProposalField, ModuleSideEffectKind, ModuleSideEffectPrecondition,
        ModuleSideEffectReceiptWriterModeledStep, ModuleSideEffectReceiptWriterObservation,
        ModuleSideEffectReceiptWriterOutcome, ModuleSideEffectReceiptWriterPreflightBoundaryFlags,
        ModuleSideEffectReceiptWriterPreflightDraft,
        ModuleSideEffectReceiptWriterPreflightRequirement, ModuleSideEffectRequestBoundaryFlags,
        ModuleSideEffectRequestDraft,
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

    fn ready_side_effect_proposal() -> (
        ModuleInvocationEnvelope,
        super::ModuleAssessmentReceiptProposal,
        super::ModuleSideEffectRequestProposal,
    ) {
        let (invocation, receipt_proposal) = ready_receipt_proposal();
        let proposal = propose_module_side_effect_request(
            &invocation,
            &receipt_proposal,
            &side_effect_request(),
        );
        (invocation, receipt_proposal, proposal)
    }

    fn policy_gate_preflight_draft() -> ModulePolicyGatePreflightDraft {
        ModulePolicyGatePreflightDraft::new(
            "work/module-policy-gate/pubpunk-publish-community-lab.md",
        )
        .with_policy_ref("docs/modules/pubpunk.md")
        .with_gate_input_ref("work/module-gate-inputs/pubpunk-publish-community-lab.md")
        .with_side_effect_receipt_proposal_ref(
            "work/module-receipts/pubpunk-publish-community-lab.md",
        )
        .with_adapter_invocation_receipt_ref(
            "work/module-receipts/github-discussions-invocation.md",
        )
        .with_payload_ref("publishing/posts/community-lab.md")
        .with_proof_requirement_ref(
            "work/module-proof-requirements/pubpunk-publish-community-lab.md",
        )
    }

    fn ready_policy_gate_preflight() -> super::ModulePolicyGatePreflight {
        let (_, _, side_effect_proposal) = ready_side_effect_proposal();
        preflight_module_policy_gate(&side_effect_proposal, &policy_gate_preflight_draft())
    }

    fn side_effect_receipt_writer_preflight_draft() -> ModuleSideEffectReceiptWriterPreflightDraft {
        ModuleSideEffectReceiptWriterPreflightDraft::new(
            "work/module-receipt-writer/pubpunk-publish-community-lab.md",
        )
        .with_policy_gate_preflight_ref("work/module-policy-gate/pubpunk-publish-community-lab.md")
        .with_receipt_target_ref("work/module-receipts/pubpunk-publish-community-lab.md")
        .with_storage_ref(".punk/runs/pubpunk-publish-community-lab")
        .with_operation_evidence_ref(
            "work/module-operation-evidence/pubpunk-publish-community-lab.md",
        )
        .with_idempotency_ref("work/module-idempotency/pubpunk-publish-community-lab.md")
        .with_rollback_ref("work/module-rollback/pubpunk-publish-community-lab.md")
        .with_error_ref("work/module-errors/pubpunk-publish-community-lab.md")
        .with_adapter_invocation_receipt_ref(
            "work/module-receipts/github-discussions-invocation.md",
        )
        .with_payload_ref("publishing/posts/community-lab.md")
    }

    fn ready_side_effect_receipt_writer_preflight() -> super::ModuleSideEffectReceiptWriterPreflight
    {
        preflight_module_side_effect_receipt_writer(
            &ready_policy_gate_preflight(),
            &side_effect_receipt_writer_preflight_draft(),
        )
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

    #[test]
    fn policy_gate_preflight_accepts_ready_request_and_safe_refs() {
        let (_, _, side_effect_proposal) = ready_side_effect_proposal();
        let preflight =
            preflight_module_policy_gate(&side_effect_proposal, &policy_gate_preflight_draft());

        assert_eq!(preflight.status, ModuleHostStatus::Ready);
        assert!(!preflight.has_blockers());
        assert_eq!(preflight.kind, ModuleSideEffectKind::Publish);
        assert!(preflight
            .covered_requirements
            .contains(&ModulePolicyGatePreflightRequirement::GateInputRef));
        assert!(preflight
            .covered_requirements
            .contains(&ModulePolicyGatePreflightRequirement::ProofRequirementRef));
        assert!(preflight.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn policy_gate_preflight_blocks_blocked_side_effect_request() {
        let (invocation, receipt_proposal) = ready_receipt_proposal();
        let unsafe_request =
            side_effect_request().with_target_ref("https://github.com/heurema/punk");
        let side_effect_proposal =
            propose_module_side_effect_request(&invocation, &receipt_proposal, &unsafe_request);
        let preflight =
            preflight_module_policy_gate(&side_effect_proposal, &policy_gate_preflight_draft());

        assert_eq!(preflight.status, ModuleHostStatus::Blocked);
        assert!(preflight.findings.iter().any(|finding| {
            finding.code == ModuleHostFindingCode::SideEffectRequestProposalBlocked
        }));
        assert!(preflight.covered_requirements.is_empty());
        assert!(preflight.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn policy_gate_preflight_blocks_missing_request_precondition() {
        let (_, _, mut side_effect_proposal) = ready_side_effect_proposal();
        side_effect_proposal
            .covered_preconditions
            .retain(|precondition| *precondition != ModuleSideEffectPrecondition::SafePayloadRef);
        let preflight =
            preflight_module_policy_gate(&side_effect_proposal, &policy_gate_preflight_draft());

        assert_eq!(preflight.status, ModuleHostStatus::Blocked);
        assert!(preflight.findings.iter().any(|finding| {
            finding.code == ModuleHostFindingCode::MissingPolicyGateRequestPrecondition
        }));
        assert!(preflight.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn policy_gate_preflight_blocks_unsafe_refs() {
        let (_, _, side_effect_proposal) = ready_side_effect_proposal();
        let draft = policy_gate_preflight_draft()
            .with_gate_input_ref("../work/module-gate-inputs/pubpunk.md");
        let preflight = preflight_module_policy_gate(&side_effect_proposal, &draft);

        assert_eq!(preflight.status, ModuleHostStatus::Blocked);
        assert!(preflight
            .findings
            .iter()
            .any(|finding| finding.code == ModuleHostFindingCode::UnsafePolicyGatePreflightRef));
        assert!(preflight.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn policy_gate_preflight_blocks_mismatched_policy_receipt_or_payload_refs() {
        let (_, _, side_effect_proposal) = ready_side_effect_proposal();
        let draft = policy_gate_preflight_draft()
            .with_policy_ref("docs/modules/devpunk.md")
            .with_side_effect_receipt_proposal_ref("work/module-receipts/other.md")
            .with_payload_ref("publishing/posts/other.md");
        let preflight = preflight_module_policy_gate(&side_effect_proposal, &draft);

        assert_eq!(preflight.status, ModuleHostStatus::Blocked);
        assert!(preflight
            .findings
            .iter()
            .any(|finding| finding.code == ModuleHostFindingCode::PolicyGatePolicyRefMismatch));
        assert!(preflight.findings.iter().any(|finding| {
            finding.code == ModuleHostFindingCode::PolicyGateSideEffectReceiptProposalRefMismatch
        }));
        assert!(preflight
            .findings
            .iter()
            .any(|finding| finding.code == ModuleHostFindingCode::PolicyGatePayloadRefMismatch));
        assert!(preflight.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn policy_gate_preflight_blocks_side_effect_flags() {
        let (_, _, side_effect_proposal) = ready_side_effect_proposal();
        let draft = policy_gate_preflight_draft().with_boundary_flags(
            ModulePolicyGatePreflightBoundaryFlags {
                invokes_gate: true,
                writes_gate_decision: true,
                ..ModulePolicyGatePreflightBoundaryFlags::pure_preflight()
            },
        );
        let preflight = preflight_module_policy_gate(&side_effect_proposal, &draft);

        assert_eq!(preflight.status, ModuleHostStatus::Blocked);
        assert!(preflight.findings.iter().any(|finding| {
            finding.code == ModuleHostFindingCode::PolicyGatePreflightHasSideEffects
        }));
        assert!(preflight.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn side_effect_receipt_writer_preflight_accepts_ready_policy_gate_and_safe_refs() {
        let policy_gate_preflight = ready_policy_gate_preflight();
        let preflight = preflight_module_side_effect_receipt_writer(
            &policy_gate_preflight,
            &side_effect_receipt_writer_preflight_draft(),
        );

        assert_eq!(preflight.status, ModuleHostStatus::Ready);
        assert!(!preflight.has_blockers());
        assert_eq!(preflight.kind, ModuleSideEffectKind::Publish);
        assert!(preflight
            .covered_requirements
            .contains(&ModuleSideEffectReceiptWriterPreflightRequirement::StorageRef));
        assert!(preflight
            .covered_requirements
            .contains(&ModuleSideEffectReceiptWriterPreflightRequirement::IdempotencyRef));
        assert!(preflight
            .covered_requirements
            .contains(&ModuleSideEffectReceiptWriterPreflightRequirement::RollbackRef));
        assert!(preflight.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn side_effect_receipt_writer_preflight_blocks_blocked_policy_gate() {
        let (_, _, side_effect_proposal) = ready_side_effect_proposal();
        let blocked_policy_gate_draft = policy_gate_preflight_draft()
            .with_gate_input_ref("../work/module-gate-inputs/pubpunk.md");
        let policy_gate_preflight =
            preflight_module_policy_gate(&side_effect_proposal, &blocked_policy_gate_draft);
        let preflight = preflight_module_side_effect_receipt_writer(
            &policy_gate_preflight,
            &side_effect_receipt_writer_preflight_draft(),
        );

        assert_eq!(preflight.status, ModuleHostStatus::Blocked);
        assert!(preflight
            .findings
            .iter()
            .any(|finding| finding.code == ModuleHostFindingCode::PolicyGatePreflightBlocked));
        assert!(preflight.covered_requirements.is_empty());
        assert!(preflight.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn side_effect_receipt_writer_preflight_blocks_missing_policy_gate_requirement() {
        let mut policy_gate_preflight = ready_policy_gate_preflight();
        policy_gate_preflight
            .covered_requirements
            .retain(|requirement| *requirement != ModulePolicyGatePreflightRequirement::PayloadRef);
        let preflight = preflight_module_side_effect_receipt_writer(
            &policy_gate_preflight,
            &side_effect_receipt_writer_preflight_draft(),
        );

        assert_eq!(preflight.status, ModuleHostStatus::Blocked);
        assert!(preflight.findings.iter().any(|finding| {
            finding.code == ModuleHostFindingCode::MissingSideEffectReceiptPolicyGateRequirement
        }));
        assert!(preflight.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn side_effect_receipt_writer_preflight_blocks_unsafe_refs() {
        let policy_gate_preflight = ready_policy_gate_preflight();
        let draft = side_effect_receipt_writer_preflight_draft()
            .with_operation_evidence_ref("../work/module-operation-evidence/pubpunk.md");
        let preflight = preflight_module_side_effect_receipt_writer(&policy_gate_preflight, &draft);

        assert_eq!(preflight.status, ModuleHostStatus::Blocked);
        assert!(
            preflight
                .findings
                .iter()
                .any(|finding| finding.code
                    == ModuleHostFindingCode::UnsafeSideEffectReceiptWriterRef)
        );
        assert!(preflight.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn side_effect_receipt_writer_preflight_blocks_mismatched_link_refs() {
        let policy_gate_preflight = ready_policy_gate_preflight();
        let draft = side_effect_receipt_writer_preflight_draft()
            .with_policy_gate_preflight_ref("work/module-policy-gate/other.md")
            .with_receipt_target_ref("work/module-receipts/other.md")
            .with_adapter_invocation_receipt_ref("work/module-receipts/other-adapter.md")
            .with_payload_ref("publishing/posts/other.md");
        let preflight = preflight_module_side_effect_receipt_writer(&policy_gate_preflight, &draft);

        assert_eq!(preflight.status, ModuleHostStatus::Blocked);
        assert!(preflight.findings.iter().any(|finding| {
            finding.code == ModuleHostFindingCode::SideEffectReceiptPolicyGatePreflightRefMismatch
        }));
        assert!(preflight.findings.iter().any(|finding| {
            finding.code == ModuleHostFindingCode::SideEffectReceiptTargetRefMismatch
        }));
        assert!(preflight.findings.iter().any(|finding| {
            finding.code
                == ModuleHostFindingCode::SideEffectReceiptAdapterInvocationReceiptRefMismatch
        }));
        assert!(preflight.findings.iter().any(|finding| {
            finding.code == ModuleHostFindingCode::SideEffectReceiptPayloadRefMismatch
        }));
        assert!(preflight.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn side_effect_receipt_writer_preflight_blocks_side_effect_flags() {
        let policy_gate_preflight = ready_policy_gate_preflight();
        let draft = side_effect_receipt_writer_preflight_draft().with_boundary_flags(
            ModuleSideEffectReceiptWriterPreflightBoundaryFlags {
                writes_receipt: true,
                writes_event_log: true,
                ..ModuleSideEffectReceiptWriterPreflightBoundaryFlags::pure_preflight()
            },
        );
        let preflight = preflight_module_side_effect_receipt_writer(&policy_gate_preflight, &draft);

        assert_eq!(preflight.status, ModuleHostStatus::Blocked);
        assert!(preflight.findings.iter().any(|finding| {
            finding.code == ModuleHostFindingCode::SideEffectReceiptWriterPreflightHasSideEffects
        }));
        assert!(preflight.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn side_effect_receipt_writer_active_behavior_plans_without_io() {
        let preflight = ready_side_effect_receipt_writer_preflight();
        let model = model_module_side_effect_receipt_writer_active_behavior(&preflight, None);

        assert_eq!(model.status, ModuleHostStatus::Ready);
        assert!(!model.has_blockers());
        assert_eq!(
            model.outcome,
            ModuleSideEffectReceiptWriterOutcome::PlannedOnly
        );
        assert!(model
            .selected_steps
            .contains(&ModuleSideEffectReceiptWriterModeledStep::ReceiptWrite));
        assert!(model.attempted_steps.is_empty());
        assert!(model.completed_steps.is_empty());
        assert!(model.failed_steps.is_empty());
        assert!(model.boundary_flags.models_active_behavior);
        assert!(model.boundary_flags.evidence_only);
        assert!(model.boundary_flags.all_side_effect_flags_false());
        assert!(model.is_evidence_only());
    }

    #[test]
    fn side_effect_receipt_writer_active_behavior_maps_writer_outcomes() {
        let preflight = ready_side_effect_receipt_writer_preflight();

        let written = model_module_side_effect_receipt_writer_active_behavior(
            &preflight,
            Some(&ModuleSideEffectReceiptWriterObservation::target_missing_written()),
        );
        let idempotent = model_module_side_effect_receipt_writer_active_behavior(
            &preflight,
            Some(&ModuleSideEffectReceiptWriterObservation::target_exists_matching()),
        );
        let conflict = model_module_side_effect_receipt_writer_active_behavior(
            &preflight,
            Some(&ModuleSideEffectReceiptWriterObservation::target_exists_different()),
        );
        let write_failed = model_module_side_effect_receipt_writer_active_behavior(
            &preflight,
            Some(&ModuleSideEffectReceiptWriterObservation::write_failed()),
        );
        let partial = model_module_side_effect_receipt_writer_active_behavior(
            &preflight,
            Some(&ModuleSideEffectReceiptWriterObservation::partial_or_ambiguous()),
        );

        assert_eq!(
            written.outcome,
            ModuleSideEffectReceiptWriterOutcome::Written
        );
        assert!(
            written.selected_step_completed(ModuleSideEffectReceiptWriterModeledStep::ReceiptWrite)
        );
        assert_eq!(
            idempotent.outcome,
            ModuleSideEffectReceiptWriterOutcome::Idempotent
        );
        assert!(!idempotent
            .selected_step_was_attempted(ModuleSideEffectReceiptWriterModeledStep::ReceiptWrite));
        assert_eq!(
            conflict.outcome,
            ModuleSideEffectReceiptWriterOutcome::Conflict
        );
        assert!(conflict.has_conflict());
        assert!(conflict.error_visible());
        assert_eq!(
            write_failed.outcome,
            ModuleSideEffectReceiptWriterOutcome::WriteFailed
        );
        assert!(write_failed.has_write_failure());
        assert!(write_failed.rollback_visible());
        assert!(write_failed.error_visible());
        assert_eq!(
            partial.outcome,
            ModuleSideEffectReceiptWriterOutcome::PartialOrAmbiguous
        );
        assert!(partial.has_partial_or_ambiguous_state());
        assert!(partial.rollback_visible());
        assert!(partial.error_visible());
        assert!(
            partial.selected_step_failed(ModuleSideEffectReceiptWriterModeledStep::ReceiptWrite)
        );
        assert!(partial.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn side_effect_receipt_writer_active_behavior_blocks_blocked_preflight() {
        let mut preflight = ready_side_effect_receipt_writer_preflight();
        preflight.status = ModuleHostStatus::Blocked;
        preflight.findings.push(super::ModuleHostFinding::new(
            ModuleHostFindingCode::PolicyGatePreflightBlocked,
            "test blocked preflight",
        ));

        let model = model_module_side_effect_receipt_writer_active_behavior(
            &preflight,
            Some(&ModuleSideEffectReceiptWriterObservation::target_missing_written()),
        );

        assert_eq!(model.status, ModuleHostStatus::Blocked);
        assert_eq!(
            model.outcome,
            ModuleSideEffectReceiptWriterOutcome::PreflightFailed
        );
        assert!(model.findings.iter().any(|finding| {
            finding.code == ModuleHostFindingCode::SideEffectReceiptWriterPreflightBlocked
        }));
        assert!(model.selected_steps.is_empty());
        assert!(model.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn side_effect_receipt_writer_active_behavior_blocks_missing_preflight_requirement() {
        let mut preflight = ready_side_effect_receipt_writer_preflight();
        preflight.covered_requirements.retain(|requirement| {
            *requirement != ModuleSideEffectReceiptWriterPreflightRequirement::StorageRef
        });

        let model = model_module_side_effect_receipt_writer_active_behavior(&preflight, None);

        assert_eq!(model.status, ModuleHostStatus::Blocked);
        assert_eq!(
            model.outcome,
            ModuleSideEffectReceiptWriterOutcome::PreflightFailed
        );
        assert!(model.findings.iter().any(|finding| {
            finding.code
                == ModuleHostFindingCode::MissingSideEffectReceiptWriterActiveBehaviorPreflightRequirement
        }));
        assert!(model.boundary_flags.all_side_effect_flags_false());
    }
}
