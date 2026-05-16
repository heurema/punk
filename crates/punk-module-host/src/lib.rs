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
        preflight_module_invocation, wrap_module_assessment, ModuleCapabilityGrant,
        ModuleHostFindingCode, ModuleHostStatus, ModuleInvocationEnvelope, ModuleOutputAuthority,
        ModuleOutputBoundaryFlags, ModuleOutputStatus, ModuleOutputSummary, ModulePrivacyPolicy,
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
            "operation",
            "source_refs",
            "capability_grants",
            "side_effects",
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
}
