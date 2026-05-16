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
pub const MODULE_HOST_SIDE_EFFECT_RECEIPT_WRITER_FILE_IO_PLAN_SCHEMA_VERSION: &str =
    "punk.module_host.side_effect_receipt_writer_file_io_plan.v0.1";
pub const MODULE_HOST_SIDE_EFFECT_RECEIPT_WRITER_TARGET_STORAGE_POLICY_SCHEMA_VERSION: &str =
    "punk.module_host.side_effect_receipt_writer_target_storage_policy.v0.1";
pub const MODULE_HOST_SIDE_EFFECT_RECEIPT_WRITER_HOST_PATH_OBSERVATION_SCHEMA_VERSION: &str =
    "punk.module_host.side_effect_receipt_writer_host_path_observation.v0.1";
pub const MODULE_HOST_SIDE_EFFECT_RECEIPT_WRITER_CONCRETE_PATH_STORAGE_POLICY_SCHEMA_VERSION: &str =
    "punk.module_host.side_effect_receipt_writer_concrete_path_storage_policy.v0.1";

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleSideEffectReceiptWriterFileIoPlanStatus {
    Ready,
    FileIoBlocked,
}

impl ModuleSideEffectReceiptWriterFileIoPlanStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::FileIoBlocked => "file_io_blocked",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleSideEffectReceiptWriterFileIoPlanBlocker {
    ActiveBehaviorBlocked,
    ActiveBehaviorNotPlannedOnly,
    MissingReceiptWriteSelection,
    MissingTargetPathRef,
    UnsafeTargetPathRef,
    MissingErrorRollbackVisibility,
    MissingBoundaryNotes,
}

impl ModuleSideEffectReceiptWriterFileIoPlanBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ActiveBehaviorBlocked => "active_behavior_blocked",
            Self::ActiveBehaviorNotPlannedOnly => "active_behavior_not_planned_only",
            Self::MissingReceiptWriteSelection => "missing_receipt_write_selection",
            Self::MissingTargetPathRef => "missing_target_path_ref",
            Self::UnsafeTargetPathRef => "unsafe_target_path_ref",
            Self::MissingErrorRollbackVisibility => "missing_error_rollback_visibility",
            Self::MissingBoundaryNotes => "missing_boundary_notes",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleSideEffectReceiptWriterWritePolicy {
    AppendOnlyCreateNew,
    IdempotentIfMatching,
    FailIfExists,
}

impl ModuleSideEffectReceiptWriterWritePolicy {
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
pub enum ModuleSideEffectReceiptWriterIdempotencyBasis {
    ReceiptTargetRef,
    ReceiptTargetAndPayloadRefs,
    OperationEvidenceRef,
}

impl ModuleSideEffectReceiptWriterIdempotencyBasis {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReceiptTargetRef => "receipt_target_ref",
            Self::ReceiptTargetAndPayloadRefs => "receipt_target_and_payload_refs",
            Self::OperationEvidenceRef => "operation_evidence_ref",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleSideEffectReceiptWriterTempAtomicPolicy {
    AtomicSiblingTemp,
    ExplicitNonAtomic,
    FailClosedIfAtomicUnavailable,
}

impl ModuleSideEffectReceiptWriterTempAtomicPolicy {
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
pub enum ModuleSideEffectReceiptWriterFileIoFailureVisibility {
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
    PartialReceiptAmbiguous,
    OperationEvidencePersistenceFailed,
}

impl ModuleSideEffectReceiptWriterFileIoFailureVisibility {
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
            Self::PartialReceiptAmbiguous => "partial_receipt_ambiguous",
            Self::OperationEvidencePersistenceFailed => "operation_evidence_persistence_failed",
        }
    }

    pub fn is_conflict_related(self) -> bool {
        self == Self::ExistingTargetDifferent
    }

    pub fn is_rollback_related(self) -> bool {
        matches!(self, Self::CleanupFailed | Self::PartialReceiptAmbiguous)
    }

    pub fn is_operation_evidence_related(self) -> bool {
        self == Self::OperationEvidencePersistenceFailed
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleSideEffectReceiptWriterFileIoPlanBoundaryFlags {
    pub models_file_io_plan: bool,
    pub models_explicit_storage_ref: bool,
    pub models_receipt_target_ref: bool,
    pub models_target_path_ref: bool,
    pub models_write_policy: bool,
    pub models_idempotency_basis: bool,
    pub models_temp_atomic_policy: bool,
    pub models_error_rollback_visibility: bool,
    pub models_operation_evidence_persistence_visibility: bool,
    pub creates_receipt: bool,
    pub writes_receipt: bool,
    pub reads_files: bool,
    pub writes_files: bool,
    pub writes_event_log: bool,
    pub persists_operation_evidence: bool,
    pub invokes_adapter: bool,
    pub invokes_policy_engine: bool,
    pub invokes_gate: bool,
    pub calls_external_apis: bool,
    pub opens_browser: bool,
    pub reads_credentials: bool,
    pub writes_gate_decision: bool,
    pub writes_proofpack: bool,
    pub publishes: bool,
    pub comments: bool,
    pub creates_pull_request: bool,
    pub creates_acceptance_claim: bool,
    pub target_path_is_authority: bool,
    pub evidence_only: bool,
    pub separates_file_io_plan_from_receipt_availability: bool,
}

impl ModuleSideEffectReceiptWriterFileIoPlanBoundaryFlags {
    pub const fn pure_plan() -> Self {
        Self {
            models_file_io_plan: true,
            models_explicit_storage_ref: true,
            models_receipt_target_ref: true,
            models_target_path_ref: true,
            models_write_policy: true,
            models_idempotency_basis: true,
            models_temp_atomic_policy: true,
            models_error_rollback_visibility: true,
            models_operation_evidence_persistence_visibility: true,
            creates_receipt: false,
            writes_receipt: false,
            reads_files: false,
            writes_files: false,
            writes_event_log: false,
            persists_operation_evidence: false,
            invokes_adapter: false,
            invokes_policy_engine: false,
            invokes_gate: false,
            calls_external_apis: false,
            opens_browser: false,
            reads_credentials: false,
            writes_gate_decision: false,
            writes_proofpack: false,
            publishes: false,
            comments: false,
            creates_pull_request: false,
            creates_acceptance_claim: false,
            target_path_is_authority: false,
            evidence_only: true,
            separates_file_io_plan_from_receipt_availability: true,
        }
    }

    pub fn all_side_effect_flags_false(self) -> bool {
        !self.creates_receipt
            && !self.writes_receipt
            && !self.reads_files
            && !self.writes_files
            && !self.writes_event_log
            && !self.persists_operation_evidence
            && !self.invokes_adapter
            && !self.invokes_policy_engine
            && !self.invokes_gate
            && !self.calls_external_apis
            && !self.opens_browser
            && !self.reads_credentials
            && !self.writes_gate_decision
            && !self.writes_proofpack
            && !self.publishes
            && !self.comments
            && !self.creates_pull_request
            && !self.creates_acceptance_claim
    }
}

pub const MODULE_HOST_PURE_SIDE_EFFECT_RECEIPT_WRITER_FILE_IO_PLAN_BOUNDARY_FLAGS:
    ModuleSideEffectReceiptWriterFileIoPlanBoundaryFlags =
    ModuleSideEffectReceiptWriterFileIoPlanBoundaryFlags::pure_plan();

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleSideEffectReceiptWriterFileIoPlan {
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
    pub target_path_ref: String,
    pub operation_evidence_ref: String,
    pub idempotency_ref: String,
    pub rollback_ref: String,
    pub error_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub write_policy: ModuleSideEffectReceiptWriterWritePolicy,
    pub idempotency_basis: ModuleSideEffectReceiptWriterIdempotencyBasis,
    pub temp_atomic_policy: ModuleSideEffectReceiptWriterTempAtomicPolicy,
    pub planned_steps: Vec<ModuleSideEffectReceiptWriterModeledStep>,
    pub failure_visibility: Vec<ModuleSideEffectReceiptWriterFileIoFailureVisibility>,
    pub blockers: Vec<ModuleSideEffectReceiptWriterFileIoPlanBlocker>,
    pub boundary_notes: Vec<String>,
    pub boundary_flags: ModuleSideEffectReceiptWriterFileIoPlanBoundaryFlags,
}

impl ModuleSideEffectReceiptWriterFileIoPlan {
    pub fn file_io_status(&self) -> ModuleSideEffectReceiptWriterFileIoPlanStatus {
        if self.blockers.is_empty() {
            ModuleSideEffectReceiptWriterFileIoPlanStatus::Ready
        } else {
            ModuleSideEffectReceiptWriterFileIoPlanStatus::FileIoBlocked
        }
    }

    pub fn has_blockers(&self) -> bool {
        !self.blockers.is_empty()
    }

    pub fn is_file_io_ready(&self) -> bool {
        self.status == ModuleHostStatus::Ready
            && self.file_io_status() == ModuleSideEffectReceiptWriterFileIoPlanStatus::Ready
    }

    pub fn plans_step(&self, step: ModuleSideEffectReceiptWriterModeledStep) -> bool {
        self.planned_steps.contains(&step)
    }

    pub fn selects_receipt_write(&self) -> bool {
        self.plans_step(ModuleSideEffectReceiptWriterModeledStep::ReceiptWrite)
    }

    pub fn tracks_failure(
        &self,
        failure: ModuleSideEffectReceiptWriterFileIoFailureVisibility,
    ) -> bool {
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

    pub fn tracks_operation_evidence_persistence_visibility(&self) -> bool {
        self.failure_visibility
            .iter()
            .any(|failure| failure.is_operation_evidence_related())
    }

    pub fn operation_outcome(&self) -> ModuleSideEffectReceiptWriterOutcome {
        if self.is_file_io_ready() {
            ModuleSideEffectReceiptWriterOutcome::PlannedOnly
        } else {
            ModuleSideEffectReceiptWriterOutcome::PreflightFailed
        }
    }

    pub fn is_evidence_only(&self) -> bool {
        self.boundary_flags.evidence_only
    }

    pub fn touches_filesystem(&self) -> bool {
        self.boundary_flags.reads_files || self.boundary_flags.writes_files
    }

    pub fn writes_receipt(&self) -> bool {
        self.boundary_flags.writes_receipt
    }

    pub fn persists_operation_evidence(&self) -> bool {
        self.boundary_flags.persists_operation_evidence
    }

    pub fn target_path_is_authority(&self) -> bool {
        self.boundary_flags.target_path_is_authority
    }
}

pub fn plan_module_side_effect_receipt_writer_file_io(
    active_behavior: &ModuleSideEffectReceiptWriterActiveBehavior,
    target_path_ref: impl Into<String>,
    write_policy: ModuleSideEffectReceiptWriterWritePolicy,
    idempotency_basis: ModuleSideEffectReceiptWriterIdempotencyBasis,
    temp_atomic_policy: ModuleSideEffectReceiptWriterTempAtomicPolicy,
    failure_visibility: Vec<ModuleSideEffectReceiptWriterFileIoFailureVisibility>,
    boundary_notes: Vec<String>,
) -> ModuleSideEffectReceiptWriterFileIoPlan {
    let target_path_ref = target_path_ref.into();
    let blockers = receipt_writer_file_io_plan_blockers(
        active_behavior,
        target_path_ref.as_str(),
        &failure_visibility,
        &boundary_notes,
    );
    let status = if blockers.is_empty() {
        ModuleHostStatus::Ready
    } else {
        ModuleHostStatus::Blocked
    };

    ModuleSideEffectReceiptWriterFileIoPlan {
        schema_version: MODULE_HOST_SIDE_EFFECT_RECEIPT_WRITER_FILE_IO_PLAN_SCHEMA_VERSION,
        status,
        authority: ModuleHostAuthority::Advisory,
        module_id: active_behavior.module_id.clone(),
        module_version: active_behavior.module_version.clone(),
        contract_ref: active_behavior.contract_ref.clone(),
        run_ref: active_behavior.run_ref.clone(),
        project_ref: active_behavior.project_ref.clone(),
        requested_operation: active_behavior.requested_operation.clone(),
        request_id: active_behavior.request_id.clone(),
        kind: active_behavior.kind,
        preflight_id: active_behavior.preflight_id.clone(),
        receipt_target_ref: active_behavior.receipt_target_ref.clone(),
        storage_ref: active_behavior.storage_ref.clone(),
        target_path_ref,
        operation_evidence_ref: active_behavior.operation_evidence_ref.clone(),
        idempotency_ref: active_behavior.idempotency_ref.clone(),
        rollback_ref: active_behavior.rollback_ref.clone(),
        error_ref: active_behavior.error_ref.clone(),
        adapter_invocation_receipt_ref: active_behavior.adapter_invocation_receipt_ref.clone(),
        payload_ref: active_behavior.payload_ref.clone(),
        write_policy,
        idempotency_basis,
        temp_atomic_policy,
        planned_steps: active_behavior.selected_steps.clone(),
        failure_visibility,
        blockers,
        boundary_notes,
        boundary_flags: MODULE_HOST_PURE_SIDE_EFFECT_RECEIPT_WRITER_FILE_IO_PLAN_BOUNDARY_FLAGS,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleSideEffectReceiptWriterTargetStoragePolicyStatus {
    Ready,
    Blocked,
}

impl ModuleSideEffectReceiptWriterTargetStoragePolicyStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker {
    FileIoPlanBlocked,
    StorageRefMissing,
    StorageRefUnsafe,
    ReceiptTargetRefMissing,
    ReceiptTargetRefUnsafe,
    TargetPathRefMissing,
    TargetPathRefUnsafe,
    RefsNotSeparated,
    StorageRootPolicyMissing,
    ReceiptTargetPolicyMissing,
    TargetPathDerivationPolicyMissing,
    PathEncodingPolicyMissing,
    ParentDirectoryPolicyMissing,
    SymlinkPolicyMissing,
    TraversalPolicyMissing,
    StorageRootEscapePolicyMissing,
    RedactionPolicyMissing,
    IdempotencyConflictPolicyMissing,
    TempAtomicPolicyMissing,
    OperationEvidencePersistencePolicyMissing,
    UnsafePolicyRef,
    StorageRootVisibilityMissing,
    TargetPathVisibilityMissing,
    ParentDirectoryVisibilityMissing,
    ConflictVisibilityMissing,
    RollbackVisibilityMissing,
    OperationEvidencePersistenceVisibilityMissing,
    BoundaryNotesMissing,
}

impl ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FileIoPlanBlocked => "file_io_plan_blocked",
            Self::StorageRefMissing => "storage_ref_missing",
            Self::StorageRefUnsafe => "storage_ref_unsafe",
            Self::ReceiptTargetRefMissing => "receipt_target_ref_missing",
            Self::ReceiptTargetRefUnsafe => "receipt_target_ref_unsafe",
            Self::TargetPathRefMissing => "target_path_ref_missing",
            Self::TargetPathRefUnsafe => "target_path_ref_unsafe",
            Self::RefsNotSeparated => "refs_not_separated",
            Self::StorageRootPolicyMissing => "storage_root_policy_missing",
            Self::ReceiptTargetPolicyMissing => "receipt_target_policy_missing",
            Self::TargetPathDerivationPolicyMissing => "target_path_derivation_policy_missing",
            Self::PathEncodingPolicyMissing => "path_encoding_policy_missing",
            Self::ParentDirectoryPolicyMissing => "parent_directory_policy_missing",
            Self::SymlinkPolicyMissing => "symlink_policy_missing",
            Self::TraversalPolicyMissing => "traversal_policy_missing",
            Self::StorageRootEscapePolicyMissing => "storage_root_escape_policy_missing",
            Self::RedactionPolicyMissing => "redaction_policy_missing",
            Self::IdempotencyConflictPolicyMissing => "idempotency_conflict_policy_missing",
            Self::TempAtomicPolicyMissing => "temp_atomic_policy_missing",
            Self::OperationEvidencePersistencePolicyMissing => {
                "operation_evidence_persistence_policy_missing"
            }
            Self::UnsafePolicyRef => "unsafe_policy_ref",
            Self::StorageRootVisibilityMissing => "storage_root_visibility_missing",
            Self::TargetPathVisibilityMissing => "target_path_visibility_missing",
            Self::ParentDirectoryVisibilityMissing => "parent_directory_visibility_missing",
            Self::ConflictVisibilityMissing => "conflict_visibility_missing",
            Self::RollbackVisibilityMissing => "rollback_visibility_missing",
            Self::OperationEvidencePersistenceVisibilityMissing => {
                "operation_evidence_persistence_visibility_missing"
            }
            Self::BoundaryNotesMissing => "boundary_notes_missing",
        }
    }

    pub fn is_fail_closed(self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleSideEffectReceiptWriterTargetStoragePolicyRefs {
    pub storage_root_selection: Option<String>,
    pub receipt_target: Option<String>,
    pub target_path_derivation: Option<String>,
    pub path_encoding: Option<String>,
    pub parent_directory: Option<String>,
    pub symlink: Option<String>,
    pub traversal: Option<String>,
    pub storage_root_escape: Option<String>,
    pub redaction: Option<String>,
    pub idempotency_conflict: Option<String>,
    pub temp_atomic: Option<String>,
    pub operation_evidence_persistence: Option<String>,
}

impl ModuleSideEffectReceiptWriterTargetStoragePolicyRefs {
    pub fn new(
        storage_root_selection: Option<String>,
        receipt_target: Option<String>,
        target_path_derivation: Option<String>,
        path_encoding: Option<String>,
        parent_directory: Option<String>,
        symlink: Option<String>,
        traversal: Option<String>,
        storage_root_escape: Option<String>,
        redaction: Option<String>,
        idempotency_conflict: Option<String>,
        temp_atomic: Option<String>,
        operation_evidence_persistence: Option<String>,
    ) -> Self {
        Self {
            storage_root_selection,
            receipt_target,
            target_path_derivation,
            path_encoding,
            parent_directory,
            symlink,
            traversal,
            storage_root_escape,
            redaction,
            idempotency_conflict,
            temp_atomic,
            operation_evidence_persistence,
        }
    }

    pub fn all_selected(
        storage_root_selection: impl Into<String>,
        receipt_target: impl Into<String>,
        target_path_derivation: impl Into<String>,
        path_encoding: impl Into<String>,
        parent_directory: impl Into<String>,
        symlink: impl Into<String>,
        traversal: impl Into<String>,
        storage_root_escape: impl Into<String>,
        redaction: impl Into<String>,
        idempotency_conflict: impl Into<String>,
        temp_atomic: impl Into<String>,
        operation_evidence_persistence: impl Into<String>,
    ) -> Self {
        Self::new(
            Some(storage_root_selection.into()),
            Some(receipt_target.into()),
            Some(target_path_derivation.into()),
            Some(path_encoding.into()),
            Some(parent_directory.into()),
            Some(symlink.into()),
            Some(traversal.into()),
            Some(storage_root_escape.into()),
            Some(redaction.into()),
            Some(idempotency_conflict.into()),
            Some(temp_atomic.into()),
            Some(operation_evidence_persistence.into()),
        )
    }

    pub fn has_storage_root_selection_policy(&self) -> bool {
        self.storage_root_selection.is_some()
    }

    pub fn has_receipt_target_policy(&self) -> bool {
        self.receipt_target.is_some()
    }

    pub fn has_target_path_derivation_policy(&self) -> bool {
        self.target_path_derivation.is_some()
    }

    pub fn has_path_encoding_policy(&self) -> bool {
        self.path_encoding.is_some()
    }

    pub fn has_parent_directory_policy(&self) -> bool {
        self.parent_directory.is_some()
    }

    pub fn has_symlink_policy(&self) -> bool {
        self.symlink.is_some()
    }

    pub fn has_traversal_policy(&self) -> bool {
        self.traversal.is_some()
    }

    pub fn has_storage_root_escape_policy(&self) -> bool {
        self.storage_root_escape.is_some()
    }

    pub fn has_redaction_policy(&self) -> bool {
        self.redaction.is_some()
    }

    pub fn has_idempotency_conflict_policy(&self) -> bool {
        self.idempotency_conflict.is_some()
    }

    pub fn has_temp_atomic_policy(&self) -> bool {
        self.temp_atomic.is_some()
    }

    pub fn has_operation_evidence_persistence_policy(&self) -> bool {
        self.operation_evidence_persistence.is_some()
    }

    pub fn all_required_selected(&self) -> bool {
        self.has_storage_root_selection_policy()
            && self.has_receipt_target_policy()
            && self.has_target_path_derivation_policy()
            && self.has_path_encoding_policy()
            && self.has_parent_directory_policy()
            && self.has_symlink_policy()
            && self.has_traversal_policy()
            && self.has_storage_root_escape_policy()
            && self.has_redaction_policy()
            && self.has_idempotency_conflict_policy()
            && self.has_temp_atomic_policy()
            && self.has_operation_evidence_persistence_policy()
    }

    pub fn has_unsafe_policy_ref(&self) -> bool {
        [
            self.storage_root_selection.as_ref(),
            self.receipt_target.as_ref(),
            self.target_path_derivation.as_ref(),
            self.path_encoding.as_ref(),
            self.parent_directory.as_ref(),
            self.symlink.as_ref(),
            self.traversal.as_ref(),
            self.storage_root_escape.as_ref(),
            self.redaction.as_ref(),
            self.idempotency_conflict.as_ref(),
            self.temp_atomic.as_ref(),
            self.operation_evidence_persistence.as_ref(),
        ]
        .into_iter()
        .flatten()
        .any(|policy_ref| !is_safe_ref(policy_ref))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleSideEffectReceiptWriterTargetStoragePolicyBoundaryFlags {
    pub models_target_storage_policy: bool,
    pub requires_ready_file_io_plan: bool,
    pub requires_explicit_storage_ref: bool,
    pub requires_explicit_receipt_target_ref: bool,
    pub requires_explicit_target_path_ref: bool,
    pub requires_selected_policy_refs: bool,
    pub keeps_storage_receipt_target_path_and_policy_refs_separate: bool,
    pub blockers_fail_closed: bool,
    pub policy_refs_are_operational_evidence: bool,
    pub evidence_only: bool,
    pub setup_neutral: bool,
    pub reads_files: bool,
    pub writes_files: bool,
    pub creates_receipt: bool,
    pub writes_receipt: bool,
    pub writes_event_log: bool,
    pub persists_operation_evidence: bool,
    pub invokes_adapter: bool,
    pub invokes_policy_engine: bool,
    pub invokes_gate: bool,
    pub calls_external_apis: bool,
    pub opens_browser: bool,
    pub reads_credentials: bool,
    pub writes_gate_decision: bool,
    pub writes_proofpack: bool,
    pub publishes: bool,
    pub comments: bool,
    pub creates_pull_request: bool,
    pub creates_acceptance_claim: bool,
    pub storage_ref_is_authority: bool,
    pub receipt_target_ref_is_authority: bool,
    pub target_path_ref_is_authority: bool,
    pub policy_refs_are_authority: bool,
    pub uses_current_working_directory_as_authority: bool,
    pub uses_global_config_as_authority: bool,
    pub uses_ide_state_as_authority: bool,
    pub uses_executor_memory_as_authority: bool,
}

impl ModuleSideEffectReceiptWriterTargetStoragePolicyBoundaryFlags {
    pub const fn pure_model() -> Self {
        Self {
            models_target_storage_policy: true,
            requires_ready_file_io_plan: true,
            requires_explicit_storage_ref: true,
            requires_explicit_receipt_target_ref: true,
            requires_explicit_target_path_ref: true,
            requires_selected_policy_refs: true,
            keeps_storage_receipt_target_path_and_policy_refs_separate: true,
            blockers_fail_closed: true,
            policy_refs_are_operational_evidence: true,
            evidence_only: true,
            setup_neutral: true,
            reads_files: false,
            writes_files: false,
            creates_receipt: false,
            writes_receipt: false,
            writes_event_log: false,
            persists_operation_evidence: false,
            invokes_adapter: false,
            invokes_policy_engine: false,
            invokes_gate: false,
            calls_external_apis: false,
            opens_browser: false,
            reads_credentials: false,
            writes_gate_decision: false,
            writes_proofpack: false,
            publishes: false,
            comments: false,
            creates_pull_request: false,
            creates_acceptance_claim: false,
            storage_ref_is_authority: false,
            receipt_target_ref_is_authority: false,
            target_path_ref_is_authority: false,
            policy_refs_are_authority: false,
            uses_current_working_directory_as_authority: false,
            uses_global_config_as_authority: false,
            uses_ide_state_as_authority: false,
            uses_executor_memory_as_authority: false,
        }
    }

    pub fn all_side_effect_flags_false(self) -> bool {
        !self.reads_files
            && !self.writes_files
            && !self.creates_receipt
            && !self.writes_receipt
            && !self.writes_event_log
            && !self.persists_operation_evidence
            && !self.invokes_adapter
            && !self.invokes_policy_engine
            && !self.invokes_gate
            && !self.calls_external_apis
            && !self.opens_browser
            && !self.reads_credentials
            && !self.writes_gate_decision
            && !self.writes_proofpack
            && !self.publishes
            && !self.comments
            && !self.creates_pull_request
            && !self.creates_acceptance_claim
    }
}

pub const MODULE_HOST_PURE_SIDE_EFFECT_RECEIPT_WRITER_TARGET_STORAGE_POLICY_BOUNDARY_FLAGS:
    ModuleSideEffectReceiptWriterTargetStoragePolicyBoundaryFlags =
    ModuleSideEffectReceiptWriterTargetStoragePolicyBoundaryFlags::pure_model();

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleSideEffectReceiptWriterTargetStoragePolicyModel {
    pub schema_version: &'static str,
    pub status: ModuleHostStatus,
    pub policy_status: ModuleSideEffectReceiptWriterTargetStoragePolicyStatus,
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
    pub target_path_ref: String,
    pub operation_evidence_ref: String,
    pub idempotency_ref: String,
    pub rollback_ref: String,
    pub error_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub write_policy: ModuleSideEffectReceiptWriterWritePolicy,
    pub idempotency_basis: ModuleSideEffectReceiptWriterIdempotencyBasis,
    pub temp_atomic_policy: ModuleSideEffectReceiptWriterTempAtomicPolicy,
    pub policy_refs: ModuleSideEffectReceiptWriterTargetStoragePolicyRefs,
    pub blockers: Vec<ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker>,
    pub boundary_notes: Vec<String>,
    pub boundary_flags: ModuleSideEffectReceiptWriterTargetStoragePolicyBoundaryFlags,
}

impl ModuleSideEffectReceiptWriterTargetStoragePolicyModel {
    pub fn is_ready(&self) -> bool {
        self.status == ModuleHostStatus::Ready
            && self.policy_status == ModuleSideEffectReceiptWriterTargetStoragePolicyStatus::Ready
    }

    pub fn is_blocked(&self) -> bool {
        self.status == ModuleHostStatus::Blocked
            || self.policy_status == ModuleSideEffectReceiptWriterTargetStoragePolicyStatus::Blocked
    }

    pub fn has_blocker(
        &self,
        blocker: ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker,
    ) -> bool {
        self.blockers.contains(&blocker)
    }

    pub fn blockers_fail_closed(&self) -> bool {
        self.blockers.iter().all(|blocker| blocker.is_fail_closed())
    }

    pub fn refs_are_separated(&self) -> bool {
        self.storage_ref != self.receipt_target_ref
            && self.storage_ref != self.target_path_ref
            && self.receipt_target_ref != self.target_path_ref
    }

    pub fn operation_outcome(&self) -> ModuleSideEffectReceiptWriterOutcome {
        if self.is_ready() {
            ModuleSideEffectReceiptWriterOutcome::PlannedOnly
        } else {
            ModuleSideEffectReceiptWriterOutcome::PreflightFailed
        }
    }

    pub fn is_evidence_only(&self) -> bool {
        self.boundary_flags.evidence_only
    }

    pub fn policy_refs_are_operational_evidence(&self) -> bool {
        self.boundary_flags.policy_refs_are_operational_evidence
    }

    pub fn touches_filesystem(&self) -> bool {
        self.boundary_flags.reads_files || self.boundary_flags.writes_files
    }

    pub fn writes_receipt(&self) -> bool {
        self.boundary_flags.writes_receipt
    }

    pub fn persists_operation_evidence(&self) -> bool {
        self.boundary_flags.persists_operation_evidence
    }

    pub fn invokes_adapter(&self) -> bool {
        self.boundary_flags.invokes_adapter
    }

    pub fn target_path_ref_is_authority(&self) -> bool {
        self.boundary_flags.target_path_ref_is_authority
    }

    pub fn policy_refs_are_authority(&self) -> bool {
        self.boundary_flags.policy_refs_are_authority
    }
}

pub fn model_module_side_effect_receipt_writer_target_storage_policy(
    file_io_plan: &ModuleSideEffectReceiptWriterFileIoPlan,
    policy_refs: ModuleSideEffectReceiptWriterTargetStoragePolicyRefs,
    boundary_notes: Vec<String>,
) -> ModuleSideEffectReceiptWriterTargetStoragePolicyModel {
    let blockers =
        receipt_writer_target_storage_policy_blockers(file_io_plan, &policy_refs, &boundary_notes);
    let policy_status = if blockers.is_empty() {
        ModuleSideEffectReceiptWriterTargetStoragePolicyStatus::Ready
    } else {
        ModuleSideEffectReceiptWriterTargetStoragePolicyStatus::Blocked
    };
    let status = if policy_status == ModuleSideEffectReceiptWriterTargetStoragePolicyStatus::Ready {
        ModuleHostStatus::Ready
    } else {
        ModuleHostStatus::Blocked
    };

    ModuleSideEffectReceiptWriterTargetStoragePolicyModel {
        schema_version: MODULE_HOST_SIDE_EFFECT_RECEIPT_WRITER_TARGET_STORAGE_POLICY_SCHEMA_VERSION,
        status,
        policy_status,
        authority: ModuleHostAuthority::Advisory,
        module_id: file_io_plan.module_id.clone(),
        module_version: file_io_plan.module_version.clone(),
        contract_ref: file_io_plan.contract_ref.clone(),
        run_ref: file_io_plan.run_ref.clone(),
        project_ref: file_io_plan.project_ref.clone(),
        requested_operation: file_io_plan.requested_operation.clone(),
        request_id: file_io_plan.request_id.clone(),
        kind: file_io_plan.kind,
        preflight_id: file_io_plan.preflight_id.clone(),
        receipt_target_ref: file_io_plan.receipt_target_ref.clone(),
        storage_ref: file_io_plan.storage_ref.clone(),
        target_path_ref: file_io_plan.target_path_ref.clone(),
        operation_evidence_ref: file_io_plan.operation_evidence_ref.clone(),
        idempotency_ref: file_io_plan.idempotency_ref.clone(),
        rollback_ref: file_io_plan.rollback_ref.clone(),
        error_ref: file_io_plan.error_ref.clone(),
        adapter_invocation_receipt_ref: file_io_plan.adapter_invocation_receipt_ref.clone(),
        payload_ref: file_io_plan.payload_ref.clone(),
        write_policy: file_io_plan.write_policy,
        idempotency_basis: file_io_plan.idempotency_basis,
        temp_atomic_policy: file_io_plan.temp_atomic_policy,
        policy_refs,
        blockers,
        boundary_notes,
        boundary_flags:
            MODULE_HOST_PURE_SIDE_EFFECT_RECEIPT_WRITER_TARGET_STORAGE_POLICY_BOUNDARY_FLAGS,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleSideEffectReceiptWriterHostPathObservationStatus {
    Observed,
    Blocked,
}

impl ModuleSideEffectReceiptWriterHostPathObservationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Observed => "observed",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleSideEffectReceiptWriterHostPathKind {
    StorageRootRelative,
    RuntimeRelative,
    Absolute,
    Unavailable,
}

impl ModuleSideEffectReceiptWriterHostPathKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::StorageRootRelative => "storage_root_relative",
            Self::RuntimeRelative => "runtime_relative",
            Self::Absolute => "absolute",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn is_available(self) -> bool {
        self != Self::Unavailable
    }

    pub fn is_sensitive(self) -> bool {
        self == Self::Absolute
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleSideEffectReceiptWriterHostPathObservationBlocker {
    TargetStoragePolicyBlocked,
    StorageRefMissing,
    ReceiptTargetRefMissing,
    TargetPathRefMissing,
    PolicyRefsNotSelected,
    MissingHostPathRef,
    UnsafeHostPathRef,
    HostPathAmbiguous,
    HostPathRedactionRequired,
    ParentDirectoryMissing,
    ParentDirectoryNotDirectory,
    SymlinkDisallowed,
    CanonicalizationUnavailable,
    TraversalDetected,
    StorageRootEscapeDetected,
    BoundaryNotesMissing,
}

impl ModuleSideEffectReceiptWriterHostPathObservationBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TargetStoragePolicyBlocked => "target_storage_policy_blocked",
            Self::StorageRefMissing => "storage_ref_missing",
            Self::ReceiptTargetRefMissing => "receipt_target_ref_missing",
            Self::TargetPathRefMissing => "target_path_ref_missing",
            Self::PolicyRefsNotSelected => "policy_refs_not_selected",
            Self::MissingHostPathRef => "missing_host_path_ref",
            Self::UnsafeHostPathRef => "unsafe_host_path_ref",
            Self::HostPathAmbiguous => "host_path_ambiguous",
            Self::HostPathRedactionRequired => "host_path_redaction_required",
            Self::ParentDirectoryMissing => "parent_directory_missing",
            Self::ParentDirectoryNotDirectory => "parent_directory_not_directory",
            Self::SymlinkDisallowed => "symlink_disallowed",
            Self::CanonicalizationUnavailable => "canonicalization_unavailable",
            Self::TraversalDetected => "traversal_detected",
            Self::StorageRootEscapeDetected => "storage_root_escape_detected",
            Self::BoundaryNotesMissing => "boundary_notes_missing",
        }
    }

    pub fn is_fail_closed(self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleSideEffectReceiptWriterHostPathObservationBoundaryFlags {
    pub models_host_path_observation: bool,
    pub requires_ready_target_storage_policy: bool,
    pub requires_explicit_storage_ref: bool,
    pub requires_explicit_receipt_target_ref: bool,
    pub requires_explicit_target_path_ref: bool,
    pub requires_selected_policy_refs: bool,
    pub keeps_storage_receipt_target_path_and_host_observation_separate: bool,
    pub blockers_fail_closed: bool,
    pub host_path_observations_are_operational_evidence: bool,
    pub host_path_observation_implies_receipt_availability: bool,
    pub evidence_only: bool,
    pub setup_neutral: bool,
    pub reads_files: bool,
    pub writes_files: bool,
    pub inspects_filesystem: bool,
    pub resolves_host_paths: bool,
    pub canonicalizes_host_paths: bool,
    pub creates_receipt: bool,
    pub writes_receipt: bool,
    pub writes_event_log: bool,
    pub persists_operation_evidence: bool,
    pub invokes_adapter: bool,
    pub invokes_policy_engine: bool,
    pub invokes_gate: bool,
    pub calls_external_apis: bool,
    pub opens_browser: bool,
    pub reads_credentials: bool,
    pub writes_gate_decision: bool,
    pub writes_proofpack: bool,
    pub publishes: bool,
    pub comments: bool,
    pub creates_pull_request: bool,
    pub creates_acceptance_claim: bool,
    pub storage_ref_is_authority: bool,
    pub receipt_target_ref_is_authority: bool,
    pub target_path_ref_is_authority: bool,
    pub host_path_ref_is_authority: bool,
    pub policy_refs_are_authority: bool,
    pub uses_current_working_directory_as_authority: bool,
    pub uses_global_config_as_authority: bool,
    pub uses_ide_state_as_authority: bool,
    pub uses_executor_memory_as_authority: bool,
}

impl ModuleSideEffectReceiptWriterHostPathObservationBoundaryFlags {
    pub const fn pure_model() -> Self {
        Self {
            models_host_path_observation: true,
            requires_ready_target_storage_policy: true,
            requires_explicit_storage_ref: true,
            requires_explicit_receipt_target_ref: true,
            requires_explicit_target_path_ref: true,
            requires_selected_policy_refs: true,
            keeps_storage_receipt_target_path_and_host_observation_separate: true,
            blockers_fail_closed: true,
            host_path_observations_are_operational_evidence: true,
            host_path_observation_implies_receipt_availability: false,
            evidence_only: true,
            setup_neutral: true,
            reads_files: false,
            writes_files: false,
            inspects_filesystem: false,
            resolves_host_paths: false,
            canonicalizes_host_paths: false,
            creates_receipt: false,
            writes_receipt: false,
            writes_event_log: false,
            persists_operation_evidence: false,
            invokes_adapter: false,
            invokes_policy_engine: false,
            invokes_gate: false,
            calls_external_apis: false,
            opens_browser: false,
            reads_credentials: false,
            writes_gate_decision: false,
            writes_proofpack: false,
            publishes: false,
            comments: false,
            creates_pull_request: false,
            creates_acceptance_claim: false,
            storage_ref_is_authority: false,
            receipt_target_ref_is_authority: false,
            target_path_ref_is_authority: false,
            host_path_ref_is_authority: false,
            policy_refs_are_authority: false,
            uses_current_working_directory_as_authority: false,
            uses_global_config_as_authority: false,
            uses_ide_state_as_authority: false,
            uses_executor_memory_as_authority: false,
        }
    }

    pub fn all_side_effect_flags_false(self) -> bool {
        !self.reads_files
            && !self.writes_files
            && !self.inspects_filesystem
            && !self.resolves_host_paths
            && !self.canonicalizes_host_paths
            && !self.creates_receipt
            && !self.writes_receipt
            && !self.writes_event_log
            && !self.persists_operation_evidence
            && !self.invokes_adapter
            && !self.invokes_policy_engine
            && !self.invokes_gate
            && !self.calls_external_apis
            && !self.opens_browser
            && !self.reads_credentials
            && !self.writes_gate_decision
            && !self.writes_proofpack
            && !self.publishes
            && !self.comments
            && !self.creates_pull_request
            && !self.creates_acceptance_claim
    }
}

pub const MODULE_HOST_PURE_SIDE_EFFECT_RECEIPT_WRITER_HOST_PATH_OBSERVATION_BOUNDARY_FLAGS:
    ModuleSideEffectReceiptWriterHostPathObservationBoundaryFlags =
    ModuleSideEffectReceiptWriterHostPathObservationBoundaryFlags::pure_model();

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleSideEffectReceiptWriterHostPathObservationModel {
    pub schema_version: &'static str,
    pub status: ModuleHostStatus,
    pub observation_status: ModuleSideEffectReceiptWriterHostPathObservationStatus,
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
    pub target_path_ref: String,
    pub operation_evidence_ref: String,
    pub idempotency_ref: String,
    pub rollback_ref: String,
    pub error_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub write_policy: ModuleSideEffectReceiptWriterWritePolicy,
    pub idempotency_basis: ModuleSideEffectReceiptWriterIdempotencyBasis,
    pub temp_atomic_policy: ModuleSideEffectReceiptWriterTempAtomicPolicy,
    pub host_path_kind: ModuleSideEffectReceiptWriterHostPathKind,
    pub host_path_ref: Option<String>,
    pub host_path_redacted: bool,
    pub observation_blockers: Vec<ModuleSideEffectReceiptWriterHostPathObservationBlocker>,
    pub blockers: Vec<ModuleSideEffectReceiptWriterHostPathObservationBlocker>,
    pub boundary_notes: Vec<String>,
    pub boundary_flags: ModuleSideEffectReceiptWriterHostPathObservationBoundaryFlags,
}

impl ModuleSideEffectReceiptWriterHostPathObservationModel {
    pub fn is_observed(&self) -> bool {
        self.status == ModuleHostStatus::Ready
            && self.observation_status
                == ModuleSideEffectReceiptWriterHostPathObservationStatus::Observed
    }

    pub fn is_blocked(&self) -> bool {
        self.status == ModuleHostStatus::Blocked
            || self.observation_status
                == ModuleSideEffectReceiptWriterHostPathObservationStatus::Blocked
    }

    pub fn has_blocker(
        &self,
        blocker: ModuleSideEffectReceiptWriterHostPathObservationBlocker,
    ) -> bool {
        self.blockers.contains(&blocker)
    }

    pub fn blockers_fail_closed(&self) -> bool {
        self.blockers.iter().all(|blocker| blocker.is_fail_closed())
    }

    pub fn refs_are_separated(&self) -> bool {
        self.storage_ref != self.receipt_target_ref
            && self.storage_ref != self.target_path_ref
            && self.receipt_target_ref != self.target_path_ref
    }

    pub fn operation_outcome(&self) -> ModuleSideEffectReceiptWriterOutcome {
        if self.is_observed() {
            ModuleSideEffectReceiptWriterOutcome::PlannedOnly
        } else {
            ModuleSideEffectReceiptWriterOutcome::PreflightFailed
        }
    }

    pub fn is_evidence_only(&self) -> bool {
        self.boundary_flags.evidence_only
    }

    pub fn touches_filesystem(&self) -> bool {
        self.boundary_flags.reads_files
            || self.boundary_flags.writes_files
            || self.boundary_flags.inspects_filesystem
            || self.boundary_flags.resolves_host_paths
    }

    pub fn resolves_host_paths(&self) -> bool {
        self.boundary_flags.resolves_host_paths
    }

    pub fn canonicalizes_host_paths(&self) -> bool {
        self.boundary_flags.canonicalizes_host_paths
    }

    pub fn writes_receipt(&self) -> bool {
        self.boundary_flags.writes_receipt
    }

    pub fn persists_operation_evidence(&self) -> bool {
        self.boundary_flags.persists_operation_evidence
    }

    pub fn invokes_adapter(&self) -> bool {
        self.boundary_flags.invokes_adapter
    }

    pub fn host_path_ref_is_authority(&self) -> bool {
        self.boundary_flags.host_path_ref_is_authority
    }

    pub fn implies_receipt_availability(&self) -> bool {
        self.boundary_flags
            .host_path_observation_implies_receipt_availability
    }
}

pub fn model_module_side_effect_receipt_writer_host_path_observation(
    target_storage_policy: &ModuleSideEffectReceiptWriterTargetStoragePolicyModel,
    host_path_kind: ModuleSideEffectReceiptWriterHostPathKind,
    host_path_ref: Option<String>,
    host_path_redacted: bool,
    observation_blockers: Vec<ModuleSideEffectReceiptWriterHostPathObservationBlocker>,
    boundary_notes: Vec<String>,
) -> ModuleSideEffectReceiptWriterHostPathObservationModel {
    let blockers = receipt_writer_host_path_observation_blockers(
        target_storage_policy,
        host_path_kind,
        host_path_ref.as_deref(),
        host_path_redacted,
        &observation_blockers,
        &boundary_notes,
    );
    let observation_status = if blockers.is_empty() {
        ModuleSideEffectReceiptWriterHostPathObservationStatus::Observed
    } else {
        ModuleSideEffectReceiptWriterHostPathObservationStatus::Blocked
    };
    let status =
        if observation_status == ModuleSideEffectReceiptWriterHostPathObservationStatus::Observed {
            ModuleHostStatus::Ready
        } else {
            ModuleHostStatus::Blocked
        };

    ModuleSideEffectReceiptWriterHostPathObservationModel {
        schema_version: MODULE_HOST_SIDE_EFFECT_RECEIPT_WRITER_HOST_PATH_OBSERVATION_SCHEMA_VERSION,
        status,
        observation_status,
        authority: ModuleHostAuthority::Advisory,
        module_id: target_storage_policy.module_id.clone(),
        module_version: target_storage_policy.module_version.clone(),
        contract_ref: target_storage_policy.contract_ref.clone(),
        run_ref: target_storage_policy.run_ref.clone(),
        project_ref: target_storage_policy.project_ref.clone(),
        requested_operation: target_storage_policy.requested_operation.clone(),
        request_id: target_storage_policy.request_id.clone(),
        kind: target_storage_policy.kind,
        preflight_id: target_storage_policy.preflight_id.clone(),
        receipt_target_ref: target_storage_policy.receipt_target_ref.clone(),
        storage_ref: target_storage_policy.storage_ref.clone(),
        target_path_ref: target_storage_policy.target_path_ref.clone(),
        operation_evidence_ref: target_storage_policy.operation_evidence_ref.clone(),
        idempotency_ref: target_storage_policy.idempotency_ref.clone(),
        rollback_ref: target_storage_policy.rollback_ref.clone(),
        error_ref: target_storage_policy.error_ref.clone(),
        adapter_invocation_receipt_ref: target_storage_policy
            .adapter_invocation_receipt_ref
            .clone(),
        payload_ref: target_storage_policy.payload_ref.clone(),
        write_policy: target_storage_policy.write_policy,
        idempotency_basis: target_storage_policy.idempotency_basis,
        temp_atomic_policy: target_storage_policy.temp_atomic_policy,
        host_path_kind,
        host_path_ref,
        host_path_redacted,
        observation_blockers,
        blockers,
        boundary_notes,
        boundary_flags:
            MODULE_HOST_PURE_SIDE_EFFECT_RECEIPT_WRITER_HOST_PATH_OBSERVATION_BOUNDARY_FLAGS,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleSideEffectReceiptWriterConcretePathStoragePolicyStatus {
    Ready,
    Blocked,
}

impl ModuleSideEffectReceiptWriterConcretePathStoragePolicyStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker {
    TargetStoragePolicyBlocked,
    HostPathObservationBlocked,
    RefMismatch,
    StorageRefMissing,
    ReceiptTargetRefMissing,
    TargetPathRefMissing,
    PolicyRefsNotSelected,
    RefsNotSeparated,
    HostPathUnavailable,
    HostPathRefMissing,
    HostPathRefUnsafe,
    HostPathRedactionRequired,
    HostPathObservationImpliesReceiptAvailability,
    BoundaryNotesMissing,
}

impl ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TargetStoragePolicyBlocked => "target_storage_policy_blocked",
            Self::HostPathObservationBlocked => "host_path_observation_blocked",
            Self::RefMismatch => "ref_mismatch",
            Self::StorageRefMissing => "storage_ref_missing",
            Self::ReceiptTargetRefMissing => "receipt_target_ref_missing",
            Self::TargetPathRefMissing => "target_path_ref_missing",
            Self::PolicyRefsNotSelected => "policy_refs_not_selected",
            Self::RefsNotSeparated => "refs_not_separated",
            Self::HostPathUnavailable => "host_path_unavailable",
            Self::HostPathRefMissing => "host_path_ref_missing",
            Self::HostPathRefUnsafe => "host_path_ref_unsafe",
            Self::HostPathRedactionRequired => "host_path_redaction_required",
            Self::HostPathObservationImpliesReceiptAvailability => {
                "host_path_observation_implies_receipt_availability"
            }
            Self::BoundaryNotesMissing => "boundary_notes_missing",
        }
    }

    pub fn is_fail_closed(self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleSideEffectReceiptWriterConcretePathStoragePolicyBoundaryFlags {
    pub models_concrete_path_storage_policy: bool,
    pub requires_ready_target_storage_policy: bool,
    pub requires_observed_host_path: bool,
    pub requires_selected_policy_refs: bool,
    pub requires_redacted_sensitive_host_path: bool,
    pub keeps_storage_receipt_target_path_host_and_policy_refs_separate: bool,
    pub blockers_fail_closed: bool,
    pub concrete_path_policy_is_operational_evidence: bool,
    pub concrete_path_policy_implies_receipt_availability: bool,
    pub evidence_only: bool,
    pub setup_neutral: bool,
    pub reads_files: bool,
    pub writes_files: bool,
    pub inspects_filesystem: bool,
    pub resolves_host_paths: bool,
    pub canonicalizes_host_paths: bool,
    pub creates_receipt: bool,
    pub writes_receipt: bool,
    pub writes_event_log: bool,
    pub persists_operation_evidence: bool,
    pub invokes_adapter: bool,
    pub invokes_policy_engine: bool,
    pub invokes_gate: bool,
    pub calls_external_apis: bool,
    pub opens_browser: bool,
    pub reads_credentials: bool,
    pub writes_gate_decision: bool,
    pub writes_proofpack: bool,
    pub publishes: bool,
    pub comments: bool,
    pub creates_pull_request: bool,
    pub creates_acceptance_claim: bool,
    pub storage_ref_is_authority: bool,
    pub receipt_target_ref_is_authority: bool,
    pub target_path_ref_is_authority: bool,
    pub host_path_ref_is_authority: bool,
    pub policy_refs_are_authority: bool,
    pub uses_current_working_directory_as_authority: bool,
    pub uses_global_config_as_authority: bool,
    pub uses_ide_state_as_authority: bool,
    pub uses_executor_memory_as_authority: bool,
}

impl ModuleSideEffectReceiptWriterConcretePathStoragePolicyBoundaryFlags {
    pub const fn pure_model() -> Self {
        Self {
            models_concrete_path_storage_policy: true,
            requires_ready_target_storage_policy: true,
            requires_observed_host_path: true,
            requires_selected_policy_refs: true,
            requires_redacted_sensitive_host_path: true,
            keeps_storage_receipt_target_path_host_and_policy_refs_separate: true,
            blockers_fail_closed: true,
            concrete_path_policy_is_operational_evidence: true,
            concrete_path_policy_implies_receipt_availability: false,
            evidence_only: true,
            setup_neutral: true,
            reads_files: false,
            writes_files: false,
            inspects_filesystem: false,
            resolves_host_paths: false,
            canonicalizes_host_paths: false,
            creates_receipt: false,
            writes_receipt: false,
            writes_event_log: false,
            persists_operation_evidence: false,
            invokes_adapter: false,
            invokes_policy_engine: false,
            invokes_gate: false,
            calls_external_apis: false,
            opens_browser: false,
            reads_credentials: false,
            writes_gate_decision: false,
            writes_proofpack: false,
            publishes: false,
            comments: false,
            creates_pull_request: false,
            creates_acceptance_claim: false,
            storage_ref_is_authority: false,
            receipt_target_ref_is_authority: false,
            target_path_ref_is_authority: false,
            host_path_ref_is_authority: false,
            policy_refs_are_authority: false,
            uses_current_working_directory_as_authority: false,
            uses_global_config_as_authority: false,
            uses_ide_state_as_authority: false,
            uses_executor_memory_as_authority: false,
        }
    }

    pub fn all_side_effect_flags_false(self) -> bool {
        !self.reads_files
            && !self.writes_files
            && !self.inspects_filesystem
            && !self.resolves_host_paths
            && !self.canonicalizes_host_paths
            && !self.creates_receipt
            && !self.writes_receipt
            && !self.writes_event_log
            && !self.persists_operation_evidence
            && !self.invokes_adapter
            && !self.invokes_policy_engine
            && !self.invokes_gate
            && !self.calls_external_apis
            && !self.opens_browser
            && !self.reads_credentials
            && !self.writes_gate_decision
            && !self.writes_proofpack
            && !self.publishes
            && !self.comments
            && !self.creates_pull_request
            && !self.creates_acceptance_claim
    }
}

pub const MODULE_HOST_PURE_SIDE_EFFECT_RECEIPT_WRITER_CONCRETE_PATH_STORAGE_POLICY_BOUNDARY_FLAGS:
    ModuleSideEffectReceiptWriterConcretePathStoragePolicyBoundaryFlags =
    ModuleSideEffectReceiptWriterConcretePathStoragePolicyBoundaryFlags::pure_model();

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleSideEffectReceiptWriterConcretePathStoragePolicyModel {
    pub schema_version: &'static str,
    pub status: ModuleHostStatus,
    pub concrete_policy_status: ModuleSideEffectReceiptWriterConcretePathStoragePolicyStatus,
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
    pub target_path_ref: String,
    pub operation_evidence_ref: String,
    pub idempotency_ref: String,
    pub rollback_ref: String,
    pub error_ref: String,
    pub adapter_invocation_receipt_ref: String,
    pub payload_ref: String,
    pub write_policy: ModuleSideEffectReceiptWriterWritePolicy,
    pub idempotency_basis: ModuleSideEffectReceiptWriterIdempotencyBasis,
    pub temp_atomic_policy: ModuleSideEffectReceiptWriterTempAtomicPolicy,
    pub policy_refs: ModuleSideEffectReceiptWriterTargetStoragePolicyRefs,
    pub host_path_kind: ModuleSideEffectReceiptWriterHostPathKind,
    pub host_path_ref: Option<String>,
    pub host_path_redacted: bool,
    pub blockers: Vec<ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker>,
    pub boundary_notes: Vec<String>,
    pub boundary_flags: ModuleSideEffectReceiptWriterConcretePathStoragePolicyBoundaryFlags,
}

impl ModuleSideEffectReceiptWriterConcretePathStoragePolicyModel {
    pub fn is_ready(&self) -> bool {
        self.status == ModuleHostStatus::Ready
            && self.concrete_policy_status
                == ModuleSideEffectReceiptWriterConcretePathStoragePolicyStatus::Ready
    }

    pub fn is_blocked(&self) -> bool {
        self.status == ModuleHostStatus::Blocked
            || self.concrete_policy_status
                == ModuleSideEffectReceiptWriterConcretePathStoragePolicyStatus::Blocked
    }

    pub fn has_blocker(
        &self,
        blocker: ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker,
    ) -> bool {
        self.blockers.contains(&blocker)
    }

    pub fn blockers_fail_closed(&self) -> bool {
        self.blockers.iter().all(|blocker| blocker.is_fail_closed())
    }

    pub fn refs_are_separated(&self) -> bool {
        self.storage_ref != self.receipt_target_ref
            && self.storage_ref != self.target_path_ref
            && self.receipt_target_ref != self.target_path_ref
    }

    pub fn operation_outcome(&self) -> ModuleSideEffectReceiptWriterOutcome {
        if self.is_ready() {
            ModuleSideEffectReceiptWriterOutcome::PlannedOnly
        } else {
            ModuleSideEffectReceiptWriterOutcome::PreflightFailed
        }
    }

    pub fn is_evidence_only(&self) -> bool {
        self.boundary_flags.evidence_only
    }

    pub fn touches_filesystem(&self) -> bool {
        self.boundary_flags.reads_files
            || self.boundary_flags.writes_files
            || self.boundary_flags.inspects_filesystem
            || self.boundary_flags.resolves_host_paths
            || self.boundary_flags.canonicalizes_host_paths
    }

    pub fn resolves_host_paths(&self) -> bool {
        self.boundary_flags.resolves_host_paths
    }

    pub fn canonicalizes_host_paths(&self) -> bool {
        self.boundary_flags.canonicalizes_host_paths
    }

    pub fn writes_receipt(&self) -> bool {
        self.boundary_flags.writes_receipt
    }

    pub fn persists_operation_evidence(&self) -> bool {
        self.boundary_flags.persists_operation_evidence
    }

    pub fn invokes_adapter(&self) -> bool {
        self.boundary_flags.invokes_adapter
    }

    pub fn host_path_ref_is_authority(&self) -> bool {
        self.boundary_flags.host_path_ref_is_authority
    }

    pub fn policy_refs_are_authority(&self) -> bool {
        self.boundary_flags.policy_refs_are_authority
    }

    pub fn implies_receipt_availability(&self) -> bool {
        self.boundary_flags
            .concrete_path_policy_implies_receipt_availability
    }
}

pub fn model_module_side_effect_receipt_writer_concrete_path_storage_policy(
    target_storage_policy: &ModuleSideEffectReceiptWriterTargetStoragePolicyModel,
    host_path_observation: &ModuleSideEffectReceiptWriterHostPathObservationModel,
    boundary_notes: Vec<String>,
) -> ModuleSideEffectReceiptWriterConcretePathStoragePolicyModel {
    let blockers = receipt_writer_concrete_path_storage_policy_blockers(
        target_storage_policy,
        host_path_observation,
        &boundary_notes,
    );
    let concrete_policy_status = if blockers.is_empty() {
        ModuleSideEffectReceiptWriterConcretePathStoragePolicyStatus::Ready
    } else {
        ModuleSideEffectReceiptWriterConcretePathStoragePolicyStatus::Blocked
    };
    let status = if concrete_policy_status
        == ModuleSideEffectReceiptWriterConcretePathStoragePolicyStatus::Ready
    {
        ModuleHostStatus::Ready
    } else {
        ModuleHostStatus::Blocked
    };

    ModuleSideEffectReceiptWriterConcretePathStoragePolicyModel {
        schema_version:
            MODULE_HOST_SIDE_EFFECT_RECEIPT_WRITER_CONCRETE_PATH_STORAGE_POLICY_SCHEMA_VERSION,
        status,
        concrete_policy_status,
        authority: ModuleHostAuthority::Advisory,
        module_id: target_storage_policy.module_id.clone(),
        module_version: target_storage_policy.module_version.clone(),
        contract_ref: target_storage_policy.contract_ref.clone(),
        run_ref: target_storage_policy.run_ref.clone(),
        project_ref: target_storage_policy.project_ref.clone(),
        requested_operation: target_storage_policy.requested_operation.clone(),
        request_id: target_storage_policy.request_id.clone(),
        kind: target_storage_policy.kind,
        preflight_id: target_storage_policy.preflight_id.clone(),
        receipt_target_ref: target_storage_policy.receipt_target_ref.clone(),
        storage_ref: target_storage_policy.storage_ref.clone(),
        target_path_ref: target_storage_policy.target_path_ref.clone(),
        operation_evidence_ref: target_storage_policy.operation_evidence_ref.clone(),
        idempotency_ref: target_storage_policy.idempotency_ref.clone(),
        rollback_ref: target_storage_policy.rollback_ref.clone(),
        error_ref: target_storage_policy.error_ref.clone(),
        adapter_invocation_receipt_ref: target_storage_policy
            .adapter_invocation_receipt_ref
            .clone(),
        payload_ref: target_storage_policy.payload_ref.clone(),
        write_policy: target_storage_policy.write_policy,
        idempotency_basis: target_storage_policy.idempotency_basis,
        temp_atomic_policy: target_storage_policy.temp_atomic_policy,
        policy_refs: target_storage_policy.policy_refs.clone(),
        host_path_kind: host_path_observation.host_path_kind,
        host_path_ref: host_path_observation.host_path_ref.clone(),
        host_path_redacted: host_path_observation.host_path_redacted,
        blockers,
        boundary_notes,
        boundary_flags:
            MODULE_HOST_PURE_SIDE_EFFECT_RECEIPT_WRITER_CONCRETE_PATH_STORAGE_POLICY_BOUNDARY_FLAGS,
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

fn receipt_writer_file_io_plan_blockers(
    active_behavior: &ModuleSideEffectReceiptWriterActiveBehavior,
    target_path_ref: &str,
    failure_visibility: &[ModuleSideEffectReceiptWriterFileIoFailureVisibility],
    boundary_notes: &[String],
) -> Vec<ModuleSideEffectReceiptWriterFileIoPlanBlocker> {
    let mut blockers = Vec::new();

    if active_behavior.status != ModuleHostStatus::Ready || active_behavior.has_blockers() {
        blockers.push(ModuleSideEffectReceiptWriterFileIoPlanBlocker::ActiveBehaviorBlocked);
    }

    if active_behavior.outcome != ModuleSideEffectReceiptWriterOutcome::PlannedOnly {
        blockers.push(ModuleSideEffectReceiptWriterFileIoPlanBlocker::ActiveBehaviorNotPlannedOnly);
    }

    if !active_behavior
        .selected_steps
        .contains(&ModuleSideEffectReceiptWriterModeledStep::ReceiptWrite)
    {
        blockers.push(ModuleSideEffectReceiptWriterFileIoPlanBlocker::MissingReceiptWriteSelection);
    }

    if target_path_ref.trim().is_empty() {
        blockers.push(ModuleSideEffectReceiptWriterFileIoPlanBlocker::MissingTargetPathRef);
    } else if !is_safe_ref(target_path_ref) {
        blockers.push(ModuleSideEffectReceiptWriterFileIoPlanBlocker::UnsafeTargetPathRef);
    }

    if failure_visibility.is_empty() {
        blockers
            .push(ModuleSideEffectReceiptWriterFileIoPlanBlocker::MissingErrorRollbackVisibility);
    }

    if boundary_notes.is_empty() {
        blockers.push(ModuleSideEffectReceiptWriterFileIoPlanBlocker::MissingBoundaryNotes);
    }

    blockers
}

fn receipt_writer_target_storage_policy_blockers(
    file_io_plan: &ModuleSideEffectReceiptWriterFileIoPlan,
    policy_refs: &ModuleSideEffectReceiptWriterTargetStoragePolicyRefs,
    boundary_notes: &[String],
) -> Vec<ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker> {
    let mut blockers = Vec::new();

    if !file_io_plan.is_file_io_ready() || file_io_plan.has_blockers() {
        blockers.push(ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::FileIoPlanBlocked);
    }

    if file_io_plan.storage_ref.trim().is_empty() {
        blockers.push(ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::StorageRefMissing);
    } else if !is_safe_ref(&file_io_plan.storage_ref) {
        blockers.push(ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::StorageRefUnsafe);
    }

    if file_io_plan.receipt_target_ref.trim().is_empty() {
        blockers
            .push(ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::ReceiptTargetRefMissing);
    } else if !is_safe_ref(&file_io_plan.receipt_target_ref) {
        blockers
            .push(ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::ReceiptTargetRefUnsafe);
    }

    if file_io_plan.target_path_ref.trim().is_empty() {
        blockers
            .push(ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::TargetPathRefMissing);
    } else if !is_safe_ref(&file_io_plan.target_path_ref) {
        blockers.push(ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::TargetPathRefUnsafe);
    }

    if file_io_plan.storage_ref == file_io_plan.receipt_target_ref
        || file_io_plan.storage_ref == file_io_plan.target_path_ref
        || file_io_plan.receipt_target_ref == file_io_plan.target_path_ref
    {
        blockers.push(ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::RefsNotSeparated);
    }

    if !policy_refs.has_storage_root_selection_policy() {
        blockers.push(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::StorageRootPolicyMissing,
        );
    }

    if !policy_refs.has_receipt_target_policy() {
        blockers.push(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::ReceiptTargetPolicyMissing,
        );
    }

    if !policy_refs.has_target_path_derivation_policy() {
        blockers.push(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::TargetPathDerivationPolicyMissing,
        );
    }

    if !policy_refs.has_path_encoding_policy() {
        blockers.push(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::PathEncodingPolicyMissing,
        );
    }

    if !policy_refs.has_parent_directory_policy() {
        blockers.push(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::ParentDirectoryPolicyMissing,
        );
    }

    if !policy_refs.has_symlink_policy() {
        blockers
            .push(ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::SymlinkPolicyMissing);
    }

    if !policy_refs.has_traversal_policy() {
        blockers
            .push(ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::TraversalPolicyMissing);
    }

    if !policy_refs.has_storage_root_escape_policy() {
        blockers.push(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::StorageRootEscapePolicyMissing,
        );
    }

    if !policy_refs.has_redaction_policy() {
        blockers
            .push(ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::RedactionPolicyMissing);
    }

    if !policy_refs.has_idempotency_conflict_policy() {
        blockers.push(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::IdempotencyConflictPolicyMissing,
        );
    }

    if !policy_refs.has_temp_atomic_policy() {
        blockers
            .push(ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::TempAtomicPolicyMissing);
    }

    if !policy_refs.has_operation_evidence_persistence_policy() {
        blockers.push(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::OperationEvidencePersistencePolicyMissing,
        );
    }

    if policy_refs.has_unsafe_policy_ref() {
        blockers.push(ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::UnsafePolicyRef);
    }

    if !file_io_plan
        .tracks_failure(ModuleSideEffectReceiptWriterFileIoFailureVisibility::StorageRootMissing)
        && !file_io_plan.tracks_failure(
            ModuleSideEffectReceiptWriterFileIoFailureVisibility::StorageRootDisallowed,
        )
    {
        blockers.push(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::StorageRootVisibilityMissing,
        );
    }

    if !file_io_plan
        .tracks_failure(ModuleSideEffectReceiptWriterFileIoFailureVisibility::TargetPathInvalid)
    {
        blockers.push(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::TargetPathVisibilityMissing,
        );
    }

    if !file_io_plan.tracks_failure(
        ModuleSideEffectReceiptWriterFileIoFailureVisibility::ParentDirectoryMissing,
    ) {
        blockers.push(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::ParentDirectoryVisibilityMissing,
        );
    }

    if !file_io_plan.tracks_conflict_visibility() {
        blockers.push(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::ConflictVisibilityMissing,
        );
    }

    if !file_io_plan.tracks_rollback_visibility() {
        blockers.push(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::RollbackVisibilityMissing,
        );
    }

    if !file_io_plan.tracks_operation_evidence_persistence_visibility() {
        blockers.push(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::OperationEvidencePersistenceVisibilityMissing,
        );
    }

    if boundary_notes.is_empty() {
        blockers
            .push(ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::BoundaryNotesMissing);
    }

    blockers
}

fn receipt_writer_host_path_observation_blockers(
    target_storage_policy: &ModuleSideEffectReceiptWriterTargetStoragePolicyModel,
    host_path_kind: ModuleSideEffectReceiptWriterHostPathKind,
    host_path_ref: Option<&str>,
    host_path_redacted: bool,
    observation_blockers: &[ModuleSideEffectReceiptWriterHostPathObservationBlocker],
    boundary_notes: &[String],
) -> Vec<ModuleSideEffectReceiptWriterHostPathObservationBlocker> {
    let mut blockers = Vec::new();

    if !target_storage_policy.is_ready() || target_storage_policy.is_blocked() {
        push_unique_host_path_observation_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::TargetStoragePolicyBlocked,
        );
    }

    if target_storage_policy.storage_ref.trim().is_empty() {
        push_unique_host_path_observation_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::StorageRefMissing,
        );
    }

    if target_storage_policy.receipt_target_ref.trim().is_empty() {
        push_unique_host_path_observation_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::ReceiptTargetRefMissing,
        );
    }

    if target_storage_policy.target_path_ref.trim().is_empty() {
        push_unique_host_path_observation_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::TargetPathRefMissing,
        );
    }

    if !target_storage_policy.policy_refs.all_required_selected() {
        push_unique_host_path_observation_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::PolicyRefsNotSelected,
        );
    }

    match host_path_ref {
        Some(host_path_ref) if !is_safe_ref(host_path_ref) => {
            push_unique_host_path_observation_blocker(
                &mut blockers,
                ModuleSideEffectReceiptWriterHostPathObservationBlocker::UnsafeHostPathRef,
            );
        }
        Some(_) => {}
        None if host_path_kind.is_available() => {
            push_unique_host_path_observation_blocker(
                &mut blockers,
                ModuleSideEffectReceiptWriterHostPathObservationBlocker::MissingHostPathRef,
            );
        }
        None => {}
    }

    if host_path_kind == ModuleSideEffectReceiptWriterHostPathKind::Unavailable
        && observation_blockers.is_empty()
        && target_storage_policy.is_ready()
    {
        push_unique_host_path_observation_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::HostPathAmbiguous,
        );
    }

    if host_path_kind.is_sensitive() && !host_path_redacted {
        push_unique_host_path_observation_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::HostPathRedactionRequired,
        );
    }

    for blocker in observation_blockers {
        push_unique_host_path_observation_blocker(&mut blockers, *blocker);
    }

    if boundary_notes.is_empty() {
        push_unique_host_path_observation_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::BoundaryNotesMissing,
        );
    }

    blockers
}

fn push_unique_host_path_observation_blocker(
    blockers: &mut Vec<ModuleSideEffectReceiptWriterHostPathObservationBlocker>,
    blocker: ModuleSideEffectReceiptWriterHostPathObservationBlocker,
) {
    if !blockers.contains(&blocker) {
        blockers.push(blocker);
    }
}

fn receipt_writer_concrete_path_storage_policy_blockers(
    target_storage_policy: &ModuleSideEffectReceiptWriterTargetStoragePolicyModel,
    host_path_observation: &ModuleSideEffectReceiptWriterHostPathObservationModel,
    boundary_notes: &[String],
) -> Vec<ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker> {
    let mut blockers = Vec::new();

    if !target_storage_policy.is_ready() || target_storage_policy.is_blocked() {
        push_unique_concrete_path_storage_policy_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::TargetStoragePolicyBlocked,
        );
    }

    if !host_path_observation.is_observed() || host_path_observation.is_blocked() {
        push_unique_concrete_path_storage_policy_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::HostPathObservationBlocked,
        );
    }

    if !receipt_writer_concrete_path_storage_policy_refs_match(
        target_storage_policy,
        host_path_observation,
    ) {
        push_unique_concrete_path_storage_policy_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::RefMismatch,
        );
    }

    if target_storage_policy.storage_ref.trim().is_empty() {
        push_unique_concrete_path_storage_policy_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::StorageRefMissing,
        );
    }

    if target_storage_policy.receipt_target_ref.trim().is_empty() {
        push_unique_concrete_path_storage_policy_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::ReceiptTargetRefMissing,
        );
    }

    if target_storage_policy.target_path_ref.trim().is_empty() {
        push_unique_concrete_path_storage_policy_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::TargetPathRefMissing,
        );
    }

    if !target_storage_policy.policy_refs.all_required_selected() {
        push_unique_concrete_path_storage_policy_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::PolicyRefsNotSelected,
        );
    }

    if !target_storage_policy.refs_are_separated() || !host_path_observation.refs_are_separated() {
        push_unique_concrete_path_storage_policy_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::RefsNotSeparated,
        );
    }

    if !host_path_observation.host_path_kind.is_available() {
        push_unique_concrete_path_storage_policy_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::HostPathUnavailable,
        );
    }

    match host_path_observation.host_path_ref.as_deref() {
        Some(host_path_ref) if !is_safe_ref(host_path_ref) => {
            push_unique_concrete_path_storage_policy_blocker(
                &mut blockers,
                ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::HostPathRefUnsafe,
            );
        }
        Some(_) => {}
        None => push_unique_concrete_path_storage_policy_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::HostPathRefMissing,
        ),
    }

    if host_path_observation.host_path_kind.is_sensitive()
        && !host_path_observation.host_path_redacted
    {
        push_unique_concrete_path_storage_policy_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::HostPathRedactionRequired,
        );
    }

    if host_path_observation.implies_receipt_availability() {
        push_unique_concrete_path_storage_policy_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::HostPathObservationImpliesReceiptAvailability,
        );
    }

    if boundary_notes.is_empty() {
        push_unique_concrete_path_storage_policy_blocker(
            &mut blockers,
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::BoundaryNotesMissing,
        );
    }

    blockers
}

fn receipt_writer_concrete_path_storage_policy_refs_match(
    target_storage_policy: &ModuleSideEffectReceiptWriterTargetStoragePolicyModel,
    host_path_observation: &ModuleSideEffectReceiptWriterHostPathObservationModel,
) -> bool {
    target_storage_policy.module_id == host_path_observation.module_id
        && target_storage_policy.module_version == host_path_observation.module_version
        && target_storage_policy.contract_ref == host_path_observation.contract_ref
        && target_storage_policy.run_ref == host_path_observation.run_ref
        && target_storage_policy.project_ref == host_path_observation.project_ref
        && target_storage_policy.requested_operation == host_path_observation.requested_operation
        && target_storage_policy.request_id == host_path_observation.request_id
        && target_storage_policy.kind == host_path_observation.kind
        && target_storage_policy.preflight_id == host_path_observation.preflight_id
        && target_storage_policy.receipt_target_ref == host_path_observation.receipt_target_ref
        && target_storage_policy.storage_ref == host_path_observation.storage_ref
        && target_storage_policy.target_path_ref == host_path_observation.target_path_ref
        && target_storage_policy.operation_evidence_ref
            == host_path_observation.operation_evidence_ref
        && target_storage_policy.idempotency_ref == host_path_observation.idempotency_ref
        && target_storage_policy.rollback_ref == host_path_observation.rollback_ref
        && target_storage_policy.error_ref == host_path_observation.error_ref
        && target_storage_policy.adapter_invocation_receipt_ref
            == host_path_observation.adapter_invocation_receipt_ref
        && target_storage_policy.payload_ref == host_path_observation.payload_ref
        && target_storage_policy.write_policy == host_path_observation.write_policy
        && target_storage_policy.idempotency_basis == host_path_observation.idempotency_basis
        && target_storage_policy.temp_atomic_policy == host_path_observation.temp_atomic_policy
}

fn push_unique_concrete_path_storage_policy_blocker(
    blockers: &mut Vec<ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker>,
    blocker: ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker,
) {
    if !blockers.contains(&blocker) {
        blockers.push(blocker);
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
        model_module_side_effect_receipt_writer_active_behavior,
        model_module_side_effect_receipt_writer_concrete_path_storage_policy,
        model_module_side_effect_receipt_writer_host_path_observation,
        model_module_side_effect_receipt_writer_target_storage_policy,
        plan_module_side_effect_receipt_writer_file_io, preflight_module_invocation,
        preflight_module_policy_gate, preflight_module_side_effect_receipt_writer,
        propose_module_assessment_receipt, propose_module_side_effect_request,
        wrap_module_assessment, ModuleCapabilityGrant, ModuleHostFindingCode, ModuleHostStatus,
        ModuleInvocationEnvelope, ModuleOutputAuthority, ModuleOutputBoundaryFlags,
        ModuleOutputStatus, ModuleOutputSummary, ModulePolicyGatePreflightBoundaryFlags,
        ModulePolicyGatePreflightDraft, ModulePolicyGatePreflightRequirement, ModulePrivacyPolicy,
        ModuleReceiptProposalField, ModuleSideEffectKind, ModuleSideEffectPrecondition,
        ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker,
        ModuleSideEffectReceiptWriterConcretePathStoragePolicyStatus,
        ModuleSideEffectReceiptWriterFileIoFailureVisibility,
        ModuleSideEffectReceiptWriterFileIoPlanBlocker, ModuleSideEffectReceiptWriterHostPathKind,
        ModuleSideEffectReceiptWriterHostPathObservationBlocker,
        ModuleSideEffectReceiptWriterHostPathObservationStatus,
        ModuleSideEffectReceiptWriterIdempotencyBasis, ModuleSideEffectReceiptWriterModeledStep,
        ModuleSideEffectReceiptWriterObservation, ModuleSideEffectReceiptWriterOutcome,
        ModuleSideEffectReceiptWriterPreflightBoundaryFlags,
        ModuleSideEffectReceiptWriterPreflightDraft,
        ModuleSideEffectReceiptWriterPreflightRequirement,
        ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker,
        ModuleSideEffectReceiptWriterTargetStoragePolicyRefs,
        ModuleSideEffectReceiptWriterTargetStoragePolicyStatus,
        ModuleSideEffectReceiptWriterTempAtomicPolicy, ModuleSideEffectReceiptWriterWritePolicy,
        ModuleSideEffectRequestBoundaryFlags, ModuleSideEffectRequestDraft,
        MODULE_HOST_SIDE_EFFECT_RECEIPT_WRITER_CONCRETE_PATH_STORAGE_POLICY_SCHEMA_VERSION,
        MODULE_HOST_SIDE_EFFECT_RECEIPT_WRITER_HOST_PATH_OBSERVATION_SCHEMA_VERSION,
        MODULE_HOST_SIDE_EFFECT_RECEIPT_WRITER_TARGET_STORAGE_POLICY_SCHEMA_VERSION,
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

    fn ready_side_effect_receipt_writer_active_behavior(
    ) -> super::ModuleSideEffectReceiptWriterActiveBehavior {
        model_module_side_effect_receipt_writer_active_behavior(
            &ready_side_effect_receipt_writer_preflight(),
            None,
        )
    }

    fn ready_side_effect_receipt_writer_file_io_plan(
    ) -> super::ModuleSideEffectReceiptWriterFileIoPlan {
        plan_module_side_effect_receipt_writer_file_io(
            &ready_side_effect_receipt_writer_active_behavior(),
            ".punk/runs/pubpunk-publish-community-lab/receipt.json",
            ModuleSideEffectReceiptWriterWritePolicy::IdempotentIfMatching,
            ModuleSideEffectReceiptWriterIdempotencyBasis::ReceiptTargetAndPayloadRefs,
            ModuleSideEffectReceiptWriterTempAtomicPolicy::AtomicSiblingTemp,
            vec![
                ModuleSideEffectReceiptWriterFileIoFailureVisibility::StorageRootMissing,
                ModuleSideEffectReceiptWriterFileIoFailureVisibility::TargetPathInvalid,
                ModuleSideEffectReceiptWriterFileIoFailureVisibility::ParentDirectoryMissing,
                ModuleSideEffectReceiptWriterFileIoFailureVisibility::ExistingTargetDifferent,
                ModuleSideEffectReceiptWriterFileIoFailureVisibility::CleanupFailed,
                ModuleSideEffectReceiptWriterFileIoFailureVisibility::OperationEvidencePersistenceFailed,
            ],
            vec!["File IO plan is explicit and side-effect-free.".to_owned()],
        )
    }

    fn ready_target_storage_policy_refs() -> ModuleSideEffectReceiptWriterTargetStoragePolicyRefs {
        ModuleSideEffectReceiptWriterTargetStoragePolicyRefs::all_selected(
            "policies/module-host/storage-root-selection.v0.1",
            "policies/module-host/receipt-target.v0.1",
            "policies/module-host/target-path-derivation.v0.1",
            "policies/module-host/path-encoding.v0.1",
            "policies/module-host/parent-directory.v0.1",
            "policies/module-host/symlink.v0.1",
            "policies/module-host/traversal.v0.1",
            "policies/module-host/storage-root-escape.v0.1",
            "policies/module-host/redaction.v0.1",
            "policies/module-host/idempotency-conflict.v0.1",
            "policies/module-host/temp-atomic.v0.1",
            "policies/module-host/operation-evidence-persistence.v0.1",
        )
    }

    fn ready_target_storage_policy_model(
    ) -> super::ModuleSideEffectReceiptWriterTargetStoragePolicyModel {
        model_module_side_effect_receipt_writer_target_storage_policy(
            &ready_side_effect_receipt_writer_file_io_plan(),
            ready_target_storage_policy_refs(),
            vec!["Target/storage policy remains operational evidence only.".to_owned()],
        )
    }

    fn ready_host_path_observation_model(
    ) -> super::ModuleSideEffectReceiptWriterHostPathObservationModel {
        model_module_side_effect_receipt_writer_host_path_observation(
            &ready_target_storage_policy_model(),
            ModuleSideEffectReceiptWriterHostPathKind::StorageRootRelative,
            Some("storage-root-relative:pubpunk-publish-community-lab/receipt.json".to_owned()),
            true,
            vec![],
            vec!["Host path observation remains redacted evidence only.".to_owned()],
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

    #[test]
    fn side_effect_receipt_writer_file_io_plan_records_explicit_targets_without_writing() {
        let active_behavior = ready_side_effect_receipt_writer_active_behavior();
        let plan = plan_module_side_effect_receipt_writer_file_io(
            &active_behavior,
            ".punk/runs/pubpunk-publish-community-lab/receipt.json",
            ModuleSideEffectReceiptWriterWritePolicy::IdempotentIfMatching,
            ModuleSideEffectReceiptWriterIdempotencyBasis::ReceiptTargetAndPayloadRefs,
            ModuleSideEffectReceiptWriterTempAtomicPolicy::AtomicSiblingTemp,
            vec![
                ModuleSideEffectReceiptWriterFileIoFailureVisibility::StorageRootMissing,
                ModuleSideEffectReceiptWriterFileIoFailureVisibility::TargetPathInvalid,
                ModuleSideEffectReceiptWriterFileIoFailureVisibility::ExistingTargetDifferent,
                ModuleSideEffectReceiptWriterFileIoFailureVisibility::AtomicMoveFailed,
                ModuleSideEffectReceiptWriterFileIoFailureVisibility::CleanupFailed,
                ModuleSideEffectReceiptWriterFileIoFailureVisibility::OperationEvidencePersistenceFailed,
            ],
            vec![
                "File IO plan models explicit targets and policies without touching the filesystem."
                    .to_owned(),
            ],
        );

        assert_eq!(
            plan.schema_version,
            super::MODULE_HOST_SIDE_EFFECT_RECEIPT_WRITER_FILE_IO_PLAN_SCHEMA_VERSION
        );
        assert_eq!(plan.status, ModuleHostStatus::Ready);
        assert_eq!(plan.file_io_status().as_str(), "ready");
        assert!(plan.is_file_io_ready());
        assert_eq!(
            plan.receipt_target_ref,
            "work/module-receipts/pubpunk-publish-community-lab.md"
        );
        assert_eq!(plan.storage_ref, ".punk/runs/pubpunk-publish-community-lab");
        assert_eq!(
            plan.target_path_ref,
            ".punk/runs/pubpunk-publish-community-lab/receipt.json"
        );
        assert!(plan.selects_receipt_write());
        assert!(plan.tracks_conflict_visibility());
        assert!(plan.tracks_rollback_visibility());
        assert!(plan.tracks_operation_evidence_persistence_visibility());
        assert_eq!(
            plan.operation_outcome(),
            ModuleSideEffectReceiptWriterOutcome::PlannedOnly
        );
        assert!(plan.boundary_flags.all_side_effect_flags_false());
        assert!(plan.is_evidence_only());
        assert!(!plan.touches_filesystem());
        assert!(!plan.writes_receipt());
        assert!(!plan.persists_operation_evidence());
        assert!(!plan.target_path_is_authority());
    }

    #[test]
    fn side_effect_receipt_writer_file_io_plan_exposes_write_and_atomic_policy() {
        let active_behavior = ready_side_effect_receipt_writer_active_behavior();
        let plan = plan_module_side_effect_receipt_writer_file_io(
            &active_behavior,
            ".punk/runs/pubpunk-publish-community-lab/receipt.json",
            ModuleSideEffectReceiptWriterWritePolicy::IdempotentIfMatching,
            ModuleSideEffectReceiptWriterIdempotencyBasis::ReceiptTargetAndPayloadRefs,
            ModuleSideEffectReceiptWriterTempAtomicPolicy::FailClosedIfAtomicUnavailable,
            vec![
                ModuleSideEffectReceiptWriterFileIoFailureVisibility::ExistingTargetMatching,
                ModuleSideEffectReceiptWriterFileIoFailureVisibility::ExistingTargetDifferent,
                ModuleSideEffectReceiptWriterFileIoFailureVisibility::PartialReceiptAmbiguous,
            ],
            vec!["Idempotency and conflict handling are modeled only.".to_owned()],
        );

        assert_eq!(plan.write_policy.as_str(), "idempotent_if_matching");
        assert!(plan.write_policy.supports_idempotency());
        assert!(!plan.write_policy.allows_silent_overwrite());
        assert_eq!(
            plan.idempotency_basis.as_str(),
            "receipt_target_and_payload_refs"
        );
        assert_eq!(
            plan.temp_atomic_policy.as_str(),
            "fail_closed_if_atomic_unavailable"
        );
        assert!(plan.temp_atomic_policy.prefers_atomic_move());
        assert!(plan
            .temp_atomic_policy
            .fails_closed_when_atomic_unavailable());
        assert!(plan.tracks_failure(
            ModuleSideEffectReceiptWriterFileIoFailureVisibility::ExistingTargetDifferent,
        ));
        assert_eq!(
            ModuleSideEffectReceiptWriterFileIoFailureVisibility::ExistingTargetDifferent.as_str(),
            "existing_target_different"
        );
    }

    #[test]
    fn side_effect_receipt_writer_file_io_plan_blocks_incomplete_plan() {
        let active_behavior = ready_side_effect_receipt_writer_active_behavior();
        let plan = plan_module_side_effect_receipt_writer_file_io(
            &active_behavior,
            "",
            ModuleSideEffectReceiptWriterWritePolicy::FailIfExists,
            ModuleSideEffectReceiptWriterIdempotencyBasis::ReceiptTargetRef,
            ModuleSideEffectReceiptWriterTempAtomicPolicy::ExplicitNonAtomic,
            vec![],
            vec![],
        );

        assert_eq!(plan.status, ModuleHostStatus::Blocked);
        assert_eq!(plan.file_io_status().as_str(), "file_io_blocked");
        assert!(plan.has_blockers());
        assert!(plan
            .blockers
            .contains(&ModuleSideEffectReceiptWriterFileIoPlanBlocker::MissingTargetPathRef));
        assert!(plan.blockers.contains(
            &ModuleSideEffectReceiptWriterFileIoPlanBlocker::MissingErrorRollbackVisibility,
        ));
        assert!(plan
            .blockers
            .contains(&ModuleSideEffectReceiptWriterFileIoPlanBlocker::MissingBoundaryNotes));
        assert_eq!(
            plan.operation_outcome(),
            ModuleSideEffectReceiptWriterOutcome::PreflightFailed
        );
        assert!(plan.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn side_effect_receipt_writer_file_io_plan_blocks_non_planned_active_behavior() {
        let preflight = ready_side_effect_receipt_writer_preflight();
        let written_behavior = model_module_side_effect_receipt_writer_active_behavior(
            &preflight,
            Some(&ModuleSideEffectReceiptWriterObservation::target_missing_written()),
        );
        let plan = plan_module_side_effect_receipt_writer_file_io(
            &written_behavior,
            "../receipt.json",
            ModuleSideEffectReceiptWriterWritePolicy::AppendOnlyCreateNew,
            ModuleSideEffectReceiptWriterIdempotencyBasis::OperationEvidenceRef,
            ModuleSideEffectReceiptWriterTempAtomicPolicy::AtomicSiblingTemp,
            vec![ModuleSideEffectReceiptWriterFileIoFailureVisibility::TempWriteFailed],
            vec!["Unsafe target path is blocked before any IO.".to_owned()],
        );

        assert_eq!(plan.status, ModuleHostStatus::Blocked);
        assert!(plan.blockers.contains(
            &ModuleSideEffectReceiptWriterFileIoPlanBlocker::ActiveBehaviorNotPlannedOnly,
        ));
        assert!(plan
            .blockers
            .contains(&ModuleSideEffectReceiptWriterFileIoPlanBlocker::UnsafeTargetPathRef));
        assert_eq!(
            ModuleSideEffectReceiptWriterFileIoPlanBlocker::ActiveBehaviorNotPlannedOnly.as_str(),
            "active_behavior_not_planned_only"
        );
        assert!(plan.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn side_effect_receipt_writer_target_storage_policy_model_ready_with_explicit_policy_refs() {
        let file_io_plan = ready_side_effect_receipt_writer_file_io_plan();
        let model = model_module_side_effect_receipt_writer_target_storage_policy(
            &file_io_plan,
            ready_target_storage_policy_refs(),
            vec!["Target/storage policy remains operational evidence only.".to_owned()],
        );

        assert_eq!(
            model.schema_version,
            MODULE_HOST_SIDE_EFFECT_RECEIPT_WRITER_TARGET_STORAGE_POLICY_SCHEMA_VERSION
        );
        assert_eq!(model.status, ModuleHostStatus::Ready);
        assert_eq!(
            model.policy_status,
            ModuleSideEffectReceiptWriterTargetStoragePolicyStatus::Ready
        );
        assert!(model.is_ready());
        assert!(!model.is_blocked());
        assert_eq!(
            model.receipt_target_ref,
            "work/module-receipts/pubpunk-publish-community-lab.md"
        );
        assert_eq!(
            model.storage_ref,
            ".punk/runs/pubpunk-publish-community-lab"
        );
        assert_eq!(
            model.target_path_ref,
            ".punk/runs/pubpunk-publish-community-lab/receipt.json"
        );
        assert!(model.refs_are_separated());
        assert!(model.policy_refs.all_required_selected());
        assert!(model.blockers.is_empty());
        assert!(model.blockers_fail_closed());
        assert_eq!(
            model.operation_outcome(),
            ModuleSideEffectReceiptWriterOutcome::PlannedOnly
        );
        assert!(model.boundary_flags.models_target_storage_policy);
        assert!(model.boundary_flags.requires_ready_file_io_plan);
        assert!(model.boundary_flags.requires_selected_policy_refs);
        assert!(
            model
                .boundary_flags
                .keeps_storage_receipt_target_path_and_policy_refs_separate
        );
        assert!(model.boundary_flags.all_side_effect_flags_false());
        assert!(model.is_evidence_only());
        assert!(model.policy_refs_are_operational_evidence());
        assert!(!model.touches_filesystem());
        assert!(!model.writes_receipt());
        assert!(!model.persists_operation_evidence());
        assert!(!model.invokes_adapter());
        assert!(!model.target_path_ref_is_authority());
        assert!(!model.policy_refs_are_authority());
    }

    #[test]
    fn side_effect_receipt_writer_target_storage_policy_model_blocks_missing_policy_refs() {
        let file_io_plan = ready_side_effect_receipt_writer_file_io_plan();
        let model = model_module_side_effect_receipt_writer_target_storage_policy(
            &file_io_plan,
            ModuleSideEffectReceiptWriterTargetStoragePolicyRefs::new(
                None, None, None, None, None, None, None, None, None, None, None, None,
            ),
            vec![],
        );

        assert_eq!(model.status, ModuleHostStatus::Blocked);
        assert_eq!(
            model.policy_status,
            ModuleSideEffectReceiptWriterTargetStoragePolicyStatus::Blocked
        );
        assert!(model.is_blocked());
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::StorageRootPolicyMissing
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::ReceiptTargetPolicyMissing
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::TargetPathDerivationPolicyMissing
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::PathEncodingPolicyMissing
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::IdempotencyConflictPolicyMissing
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::TempAtomicPolicyMissing
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::OperationEvidencePersistencePolicyMissing
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::BoundaryNotesMissing
        ));
        assert!(model.blockers_fail_closed());
        assert_eq!(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::StorageRootPolicyMissing
                .as_str(),
            "storage_root_policy_missing"
        );
        assert_eq!(
            model.operation_outcome(),
            ModuleSideEffectReceiptWriterOutcome::PreflightFailed
        );
        assert!(model.boundary_flags.all_side_effect_flags_false());
    }

    #[test]
    fn side_effect_receipt_writer_target_storage_policy_model_fails_closed_on_unsafe_plan() {
        let active_behavior = ready_side_effect_receipt_writer_active_behavior();
        let unsafe_file_io_plan = plan_module_side_effect_receipt_writer_file_io(
            &active_behavior,
            ".punk/runs/pubpunk-publish-community-lab/../receipt.json",
            ModuleSideEffectReceiptWriterWritePolicy::IdempotentIfMatching,
            ModuleSideEffectReceiptWriterIdempotencyBasis::ReceiptTargetAndPayloadRefs,
            ModuleSideEffectReceiptWriterTempAtomicPolicy::AtomicSiblingTemp,
            vec![
                ModuleSideEffectReceiptWriterFileIoFailureVisibility::TargetPathInvalid,
                ModuleSideEffectReceiptWriterFileIoFailureVisibility::ExistingTargetDifferent,
            ],
            vec!["Unsafe target path remains model data only.".to_owned()],
        );
        let policy_refs = ModuleSideEffectReceiptWriterTargetStoragePolicyRefs::new(
            Some("/unsafe/policy".to_owned()),
            Some("policies/module-host/receipt-target.v0.1".to_owned()),
            None,
            Some("policies/module-host/path-encoding.v0.1".to_owned()),
            None,
            None,
            None,
            None,
            None,
            None,
            Some("policies/module-host/temp-atomic.v0.1".to_owned()),
            None,
        );
        let model = model_module_side_effect_receipt_writer_target_storage_policy(
            &unsafe_file_io_plan,
            policy_refs,
            vec!["Unsafe policy refs fail closed before IO.".to_owned()],
        );

        assert_eq!(model.status, ModuleHostStatus::Blocked);
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::FileIoPlanBlocked
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::TargetPathRefUnsafe
        ));
        assert!(model
            .has_blocker(ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::UnsafePolicyRef));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::StorageRootVisibilityMissing
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::ParentDirectoryVisibilityMissing
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::RollbackVisibilityMissing
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterTargetStoragePolicyBlocker::OperationEvidencePersistenceVisibilityMissing
        ));
        assert!(model.blockers_fail_closed());
        assert!(!model.touches_filesystem());
        assert!(!model.writes_receipt());
        assert!(!model.persists_operation_evidence());
    }

    #[test]
    fn side_effect_receipt_writer_host_path_observation_model_observes_explicit_redacted_ref() {
        let target_storage_policy = ready_target_storage_policy_model();
        let model = model_module_side_effect_receipt_writer_host_path_observation(
            &target_storage_policy,
            ModuleSideEffectReceiptWriterHostPathKind::StorageRootRelative,
            Some("storage-root-relative:pubpunk-publish-community-lab/receipt.json".to_owned()),
            true,
            vec![],
            vec!["Host path observation remains redacted evidence only.".to_owned()],
        );

        assert_eq!(
            model.schema_version,
            MODULE_HOST_SIDE_EFFECT_RECEIPT_WRITER_HOST_PATH_OBSERVATION_SCHEMA_VERSION
        );
        assert_eq!(model.status, ModuleHostStatus::Ready);
        assert_eq!(
            model.observation_status,
            ModuleSideEffectReceiptWriterHostPathObservationStatus::Observed
        );
        assert!(model.is_observed());
        assert!(!model.is_blocked());
        assert_eq!(
            model.receipt_target_ref,
            "work/module-receipts/pubpunk-publish-community-lab.md"
        );
        assert_eq!(
            model.storage_ref,
            ".punk/runs/pubpunk-publish-community-lab"
        );
        assert_eq!(
            model.target_path_ref,
            ".punk/runs/pubpunk-publish-community-lab/receipt.json"
        );
        assert!(model.refs_are_separated());
        assert_eq!(
            model.host_path_kind,
            ModuleSideEffectReceiptWriterHostPathKind::StorageRootRelative
        );
        assert_eq!(model.host_path_kind.as_str(), "storage_root_relative");
        assert_eq!(
            model.host_path_ref.as_deref(),
            Some("storage-root-relative:pubpunk-publish-community-lab/receipt.json")
        );
        assert!(model.host_path_redacted);
        assert!(model.observation_blockers.is_empty());
        assert!(model.blockers.is_empty());
        assert!(model.blockers_fail_closed());
        assert_eq!(
            model.operation_outcome(),
            ModuleSideEffectReceiptWriterOutcome::PlannedOnly
        );
        assert!(model.boundary_flags.models_host_path_observation);
        assert!(model.boundary_flags.requires_ready_target_storage_policy);
        assert!(
            model
                .boundary_flags
                .host_path_observations_are_operational_evidence
        );
        assert!(model.boundary_flags.all_side_effect_flags_false());
        assert!(model.is_evidence_only());
        assert!(!model.touches_filesystem());
        assert!(!model.resolves_host_paths());
        assert!(!model.canonicalizes_host_paths());
        assert!(!model.writes_receipt());
        assert!(!model.persists_operation_evidence());
        assert!(!model.invokes_adapter());
        assert!(!model.host_path_ref_is_authority());
        assert!(!model.implies_receipt_availability());
    }

    #[test]
    fn side_effect_receipt_writer_host_path_observation_model_fails_closed_on_sensitive_or_unsafe_observation(
    ) {
        let target_storage_policy = ready_target_storage_policy_model();
        let model = model_module_side_effect_receipt_writer_host_path_observation(
            &target_storage_policy,
            ModuleSideEffectReceiptWriterHostPathKind::Absolute,
            Some("/private/tmp/receipt.json".to_owned()),
            false,
            vec![
                ModuleSideEffectReceiptWriterHostPathObservationBlocker::ParentDirectoryMissing,
                ModuleSideEffectReceiptWriterHostPathObservationBlocker::SymlinkDisallowed,
                ModuleSideEffectReceiptWriterHostPathObservationBlocker::CanonicalizationUnavailable,
                ModuleSideEffectReceiptWriterHostPathObservationBlocker::TraversalDetected,
                ModuleSideEffectReceiptWriterHostPathObservationBlocker::StorageRootEscapeDetected,
            ],
            vec![],
        );

        assert_eq!(model.status, ModuleHostStatus::Blocked);
        assert_eq!(
            model.observation_status,
            ModuleSideEffectReceiptWriterHostPathObservationStatus::Blocked
        );
        assert!(model.is_blocked());
        assert_eq!(
            model.operation_outcome(),
            ModuleSideEffectReceiptWriterOutcome::PreflightFailed
        );
        assert_eq!(model.host_path_kind.as_str(), "absolute");
        assert!(model.host_path_kind.is_sensitive());
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::UnsafeHostPathRef
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::HostPathRedactionRequired
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::ParentDirectoryMissing
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::SymlinkDisallowed
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::CanonicalizationUnavailable
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::TraversalDetected
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::StorageRootEscapeDetected
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::BoundaryNotesMissing
        ));
        assert!(model.blockers_fail_closed());
        assert!(!model.touches_filesystem());
        assert!(!model.writes_receipt());
        assert!(!model.persists_operation_evidence());
    }

    #[test]
    fn side_effect_receipt_writer_host_path_observation_model_blocks_unready_policy() {
        let file_io_plan = ready_side_effect_receipt_writer_file_io_plan();
        let target_storage_policy = model_module_side_effect_receipt_writer_target_storage_policy(
            &file_io_plan,
            ModuleSideEffectReceiptWriterTargetStoragePolicyRefs::new(
                None, None, None, None, None, None, None, None, None, None, None, None,
            ),
            vec!["Incomplete policy refs keep host path observation blocked.".to_owned()],
        );
        let model = model_module_side_effect_receipt_writer_host_path_observation(
            &target_storage_policy,
            ModuleSideEffectReceiptWriterHostPathKind::Unavailable,
            None,
            true,
            vec![],
            vec!["Host path observation inherits target/storage readiness.".to_owned()],
        );

        assert_eq!(model.status, ModuleHostStatus::Blocked);
        assert_eq!(
            model.observation_status,
            ModuleSideEffectReceiptWriterHostPathObservationStatus::Blocked
        );
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::TargetStoragePolicyBlocked
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::PolicyRefsNotSelected
        ));
        assert!(!model.has_blocker(
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::HostPathAmbiguous
        ));
        assert!(model.blockers_fail_closed());
        assert_eq!(
            ModuleSideEffectReceiptWriterHostPathObservationBlocker::TargetStoragePolicyBlocked
                .as_str(),
            "target_storage_policy_blocked"
        );
        assert!(!model.touches_filesystem());
        assert!(!model.writes_receipt());
    }

    #[test]
    fn side_effect_receipt_writer_concrete_path_storage_policy_model_ready_with_explicit_inputs() {
        let target_storage_policy = ready_target_storage_policy_model();
        let host_path_observation = ready_host_path_observation_model();
        let model = model_module_side_effect_receipt_writer_concrete_path_storage_policy(
            &target_storage_policy,
            &host_path_observation,
            vec!["Concrete path/storage policy remains operational evidence only.".to_owned()],
        );

        assert_eq!(
            model.schema_version,
            MODULE_HOST_SIDE_EFFECT_RECEIPT_WRITER_CONCRETE_PATH_STORAGE_POLICY_SCHEMA_VERSION
        );
        assert_eq!(model.status, ModuleHostStatus::Ready);
        assert_eq!(
            model.concrete_policy_status,
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyStatus::Ready
        );
        assert!(model.is_ready());
        assert!(!model.is_blocked());
        assert_eq!(
            model.receipt_target_ref,
            "work/module-receipts/pubpunk-publish-community-lab.md"
        );
        assert_eq!(
            model.storage_ref,
            ".punk/runs/pubpunk-publish-community-lab"
        );
        assert_eq!(
            model.target_path_ref,
            ".punk/runs/pubpunk-publish-community-lab/receipt.json"
        );
        assert!(model.refs_are_separated());
        assert!(model.policy_refs.all_required_selected());
        assert_eq!(
            model.host_path_kind,
            ModuleSideEffectReceiptWriterHostPathKind::StorageRootRelative
        );
        assert_eq!(model.host_path_kind.as_str(), "storage_root_relative");
        assert_eq!(
            model.host_path_ref.as_deref(),
            Some("storage-root-relative:pubpunk-publish-community-lab/receipt.json")
        );
        assert!(model.host_path_redacted);
        assert!(model.blockers.is_empty());
        assert!(model.blockers_fail_closed());
        assert_eq!(
            model.operation_outcome(),
            ModuleSideEffectReceiptWriterOutcome::PlannedOnly
        );
        assert!(model.boundary_flags.models_concrete_path_storage_policy);
        assert!(model.boundary_flags.requires_ready_target_storage_policy);
        assert!(model.boundary_flags.requires_observed_host_path);
        assert!(model.boundary_flags.requires_selected_policy_refs);
        assert!(model.boundary_flags.requires_redacted_sensitive_host_path);
        assert!(
            model
                .boundary_flags
                .concrete_path_policy_is_operational_evidence
        );
        assert!(model.boundary_flags.all_side_effect_flags_false());
        assert!(model.is_evidence_only());
        assert!(!model.touches_filesystem());
        assert!(!model.resolves_host_paths());
        assert!(!model.canonicalizes_host_paths());
        assert!(!model.writes_receipt());
        assert!(!model.persists_operation_evidence());
        assert!(!model.invokes_adapter());
        assert!(!model.host_path_ref_is_authority());
        assert!(!model.policy_refs_are_authority());
        assert!(!model.implies_receipt_availability());
    }

    #[test]
    fn side_effect_receipt_writer_concrete_path_storage_policy_model_blocks_unsafe_or_unredacted_observation(
    ) {
        let target_storage_policy = ready_target_storage_policy_model();
        let host_path_observation = model_module_side_effect_receipt_writer_host_path_observation(
            &target_storage_policy,
            ModuleSideEffectReceiptWriterHostPathKind::Absolute,
            Some("/private/tmp/receipt.json".to_owned()),
            false,
            vec![
                ModuleSideEffectReceiptWriterHostPathObservationBlocker::ParentDirectoryMissing,
                ModuleSideEffectReceiptWriterHostPathObservationBlocker::SymlinkDisallowed,
                ModuleSideEffectReceiptWriterHostPathObservationBlocker::CanonicalizationUnavailable,
                ModuleSideEffectReceiptWriterHostPathObservationBlocker::TraversalDetected,
                ModuleSideEffectReceiptWriterHostPathObservationBlocker::StorageRootEscapeDetected,
            ],
            vec![],
        );
        let model = model_module_side_effect_receipt_writer_concrete_path_storage_policy(
            &target_storage_policy,
            &host_path_observation,
            vec!["Unsafe host path observations block concrete path policy readiness.".to_owned()],
        );

        assert_eq!(model.status, ModuleHostStatus::Blocked);
        assert_eq!(
            model.concrete_policy_status,
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyStatus::Blocked
        );
        assert!(model.is_blocked());
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::HostPathObservationBlocked
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::HostPathRefUnsafe
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::HostPathRedactionRequired
        ));
        assert!(model.blockers_fail_closed());
        assert_eq!(
            model.operation_outcome(),
            ModuleSideEffectReceiptWriterOutcome::PreflightFailed
        );
        assert!(!model.touches_filesystem());
        assert!(!model.writes_receipt());
        assert!(!model.persists_operation_evidence());
        assert!(!model.invokes_adapter());
    }

    #[test]
    fn side_effect_receipt_writer_concrete_path_storage_policy_model_blocks_unready_policy_and_ref_mismatch(
    ) {
        let file_io_plan = ready_side_effect_receipt_writer_file_io_plan();
        let target_storage_policy = model_module_side_effect_receipt_writer_target_storage_policy(
            &file_io_plan,
            ModuleSideEffectReceiptWriterTargetStoragePolicyRefs::new(
                None, None, None, None, None, None, None, None, None, None, None, None,
            ),
            vec!["Incomplete policy refs keep concrete path policy blocked.".to_owned()],
        );
        let mut host_path_observation = ready_host_path_observation_model();
        host_path_observation.target_path_ref = "other/receipt.json".to_owned();
        let model = model_module_side_effect_receipt_writer_concrete_path_storage_policy(
            &target_storage_policy,
            &host_path_observation,
            vec![],
        );

        assert_eq!(model.status, ModuleHostStatus::Blocked);
        assert_eq!(
            model.concrete_policy_status,
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyStatus::Blocked
        );
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::TargetStoragePolicyBlocked
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::PolicyRefsNotSelected
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::RefMismatch
        ));
        assert!(model.has_blocker(
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::BoundaryNotesMissing
        ));
        assert!(model.blockers_fail_closed());
        assert_eq!(
            ModuleSideEffectReceiptWriterConcretePathStoragePolicyBlocker::TargetStoragePolicyBlocked
                .as_str(),
            "target_storage_policy_blocked"
        );
        assert!(!model.touches_filesystem());
        assert!(!model.writes_receipt());
        assert!(!model.persists_operation_evidence());
    }
}
