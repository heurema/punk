//! Minimal pure flow-state kernel for Punk Phase 1.
//!
//! This crate intentionally stays deterministic and side-effect free in this
//! first implementation step. It models states, commands, and transition
//! guards only. No persistence, event writing, CLI behavior, or `.punk/`
//! runtime state is introduced here.

use punk_contract::ContractStatus;
use punk_domain::{ContractRef, RunId, RunReceipt, RunReceiptId, RunScopeRef};
use punk_events::{
    EventArtifacts, EventCorrelation, EventDraft, EventKind, EventPhase, EventResult, EventSource,
    EventStatus, FlowTransitionRef,
};

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlowState {
    ProjectInitialized,
    GoalCreated,
    ContractDrafted,
    AwaitingApproval,
    Approved,
    Running,
    ReceiptWritten,
    DecisionWritten,
    ProofWritten,
    Reported,
    Closed,
    Blocked,
    Escalated,
    Cancelled,
}

impl FlowState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProjectInitialized => "ProjectInitialized",
            Self::GoalCreated => "GoalCreated",
            Self::ContractDrafted => "ContractDrafted",
            Self::AwaitingApproval => "AwaitingApproval",
            Self::Approved => "Approved",
            Self::Running => "Running",
            Self::ReceiptWritten => "ReceiptWritten",
            Self::DecisionWritten => "DecisionWritten",
            Self::ProofWritten => "ProofWritten",
            Self::Reported => "Reported",
            Self::Closed => "Closed",
            Self::Blocked => "Blocked",
            Self::Escalated => "Escalated",
            Self::Cancelled => "Cancelled",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlowCommand {
    CreateGoal,
    DraftContract,
    RequestApproval,
    Approve,
    StartRun,
    WriteReceipt,
    WriteDecision,
    WriteProof,
    MarkReported,
    Close,
    Block,
    Escalate,
    Cancel,
}

impl FlowCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CreateGoal => "CreateGoal",
            Self::DraftContract => "DraftContract",
            Self::RequestApproval => "RequestApproval",
            Self::Approve => "Approve",
            Self::StartRun => "StartRun",
            Self::WriteReceipt => "WriteReceipt",
            Self::WriteDecision => "WriteDecision",
            Self::WriteProof => "WriteProof",
            Self::MarkReported => "MarkReported",
            Self::Close => "Close",
            Self::Block => "Block",
            Self::Escalate => "Escalate",
            Self::Cancel => "Cancel",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlowInstance {
    state: FlowState,
}

impl FlowInstance {
    pub fn new(state: FlowState) -> Self {
        Self { state }
    }

    pub fn state(&self) -> FlowState {
        self.state
    }

    pub fn allowed_commands(&self) -> &'static [FlowCommand] {
        allowed_commands_for(self.state)
    }

    pub fn attempt_transition(self, command: FlowCommand) -> FlowTransitionAttempt {
        match next_state_for(self.state, command) {
            Some(next_state) => FlowTransitionAttempt::applied(self.state, command, next_state),
            None => FlowTransitionAttempt::denied(
                self.state,
                command,
                guard_code_for(self.state, command),
                FLOW_ILLEGAL_TRANSITION_ERROR_CODE,
                allowed_commands_for(self.state),
            ),
        }
    }

    pub fn attempt_transition_with_contract(
        self,
        command: FlowCommand,
        contract_status: ContractStatus,
        scope_valid: bool,
    ) -> FlowTransitionAttempt {
        if self.state == FlowState::Approved && command == FlowCommand::StartRun {
            if !scope_valid {
                return FlowTransitionAttempt::denied(
                    self.state,
                    command,
                    "RUN_REQUIRES_EXPLICIT_CONTRACT_SCOPE",
                    FLOW_ILLEGAL_TRANSITION_ERROR_CODE,
                    allowed_commands_for(self.state),
                );
            }

            if !contract_status.ready_for_bounded_work() {
                return FlowTransitionAttempt::denied(
                    self.state,
                    command,
                    "RUN_REQUIRES_APPROVED_FOR_RUN_CONTRACT",
                    FLOW_ILLEGAL_TRANSITION_ERROR_CODE,
                    allowed_commands_for(self.state),
                );
            }
        }

        self.attempt_transition(command)
    }

    pub fn attempt_transition_with_contract_receipt(
        self,
        command: FlowCommand,
        contract_status: ContractStatus,
        scope_valid: bool,
        receipt_id: RunReceiptId,
        contract_ref: ContractRef,
        run_id: RunId,
        run_scope_ref: RunScopeRef,
    ) -> FlowTransitionReceiptAttempt {
        let attempt = self.attempt_transition_with_contract(command, contract_status, scope_valid);
        let receipt = if self.state == FlowState::Approved
            && command == FlowCommand::StartRun
            && attempt.next_state() == Some(FlowState::Running)
        {
            Some(RunReceipt::new(
                receipt_id,
                contract_ref,
                run_id,
                run_scope_ref,
            ))
        } else {
            None
        };

        FlowTransitionReceiptAttempt::new(attempt, receipt)
    }

    pub fn transition(self, command: FlowCommand) -> Result<Self, TransitionError> {
        self.attempt_transition(command).into_result()
    }

    pub fn transition_with_contract(
        self,
        command: FlowCommand,
        contract_status: ContractStatus,
        scope_valid: bool,
    ) -> Result<Self, TransitionError> {
        self.attempt_transition_with_contract(command, contract_status, scope_valid)
            .into_result()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransitionError {
    pub current_state: FlowState,
    pub attempted_command: FlowCommand,
    pub next_allowed_commands: &'static [FlowCommand],
}

impl TransitionError {
    fn new(current_state: FlowState, attempted_command: FlowCommand) -> Self {
        Self {
            current_state,
            attempted_command,
            next_allowed_commands: allowed_commands_for(current_state),
        }
    }
}

pub const FLOW_ILLEGAL_TRANSITION_ERROR_CODE: &str = "E_FLOW_ILLEGAL_TRANSITION";
pub const FLOW_EVENT_MAPPING_ERROR_CODE: &str = "E_FLOW_EVENT_MAPPING";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlowTransitionAttempt {
    current_state: FlowState,
    attempted_command: FlowCommand,
    outcome: FlowTransitionOutcome,
}

impl FlowTransitionAttempt {
    fn applied(
        current_state: FlowState,
        attempted_command: FlowCommand,
        next_state: FlowState,
    ) -> Self {
        Self {
            current_state,
            attempted_command,
            outcome: FlowTransitionOutcome::Applied { next_state },
        }
    }

    fn denied(
        current_state: FlowState,
        attempted_command: FlowCommand,
        guard_code: &'static str,
        error_code: &'static str,
        next_allowed_commands: &'static [FlowCommand],
    ) -> Self {
        Self {
            current_state,
            attempted_command,
            outcome: FlowTransitionOutcome::Denied {
                guard_code,
                error_code,
                next_allowed_commands,
            },
        }
    }

    pub fn current_state(&self) -> FlowState {
        self.current_state
    }

    pub fn attempted_command(&self) -> FlowCommand {
        self.attempted_command
    }

    pub fn outcome(&self) -> FlowTransitionOutcome {
        self.outcome
    }

    pub fn next_state(&self) -> Option<FlowState> {
        match self.outcome {
            FlowTransitionOutcome::Applied { next_state } => Some(next_state),
            FlowTransitionOutcome::Denied { .. } => None,
        }
    }

    pub fn next_allowed_commands(&self) -> &'static [FlowCommand] {
        match self.outcome {
            FlowTransitionOutcome::Applied { next_state } => allowed_commands_for(next_state),
            FlowTransitionOutcome::Denied {
                next_allowed_commands,
                ..
            } => next_allowed_commands,
        }
    }

    pub fn guard_code(&self) -> Option<&'static str> {
        match self.outcome {
            FlowTransitionOutcome::Applied { .. } => None,
            FlowTransitionOutcome::Denied { guard_code, .. } => Some(guard_code),
        }
    }

    pub fn error_code(&self) -> Option<&'static str> {
        match self.outcome {
            FlowTransitionOutcome::Applied { .. } => None,
            FlowTransitionOutcome::Denied { error_code, .. } => Some(error_code),
        }
    }

    pub fn into_result(self) -> Result<FlowInstance, TransitionError> {
        match self.outcome {
            FlowTransitionOutcome::Applied { next_state } => Ok(FlowInstance::new(next_state)),
            FlowTransitionOutcome::Denied {
                next_allowed_commands,
                ..
            } => {
                let mut error = TransitionError::new(self.current_state, self.attempted_command);
                error.next_allowed_commands = next_allowed_commands;
                Err(error)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowTransitionReceiptAttempt {
    transition: FlowTransitionAttempt,
    receipt: Option<RunReceipt>,
}

impl FlowTransitionReceiptAttempt {
    fn new(transition: FlowTransitionAttempt, receipt: Option<RunReceipt>) -> Self {
        Self {
            transition,
            receipt,
        }
    }

    pub fn transition(&self) -> &FlowTransitionAttempt {
        &self.transition
    }

    pub fn receipt(&self) -> Option<&RunReceipt> {
        self.receipt.as_ref()
    }
}

pub fn transition_attempt_event_draft(
    attempt: &FlowTransitionAttempt,
    flow_id: impl Into<String>,
    goal_ref: Option<&str>,
) -> EventDraft {
    let flow_id = flow_id.into();
    let correlation = match goal_ref {
        Some(goal_ref) => EventCorrelation::new(flow_id).with_goal_ref(goal_ref),
        None => EventCorrelation::new(flow_id),
    };

    let transition = {
        let transition = FlowTransitionRef::new()
            .with_from_state(attempt.current_state().as_str())
            .with_command(attempt.attempted_command().as_str());

        match attempt.next_state() {
            Some(next_state) => transition.with_to_state(next_state.as_str()),
            None => transition,
        }
    };

    let artifacts = match goal_ref {
        Some(goal_ref) => EventArtifacts::new().with_ref(goal_ref),
        None => EventArtifacts::default(),
    };

    let event = match (
        attempt.guard_code(),
        attempt.error_code(),
        attempt.next_state(),
    ) {
        (None, None, Some(_)) => EventDraft::new(
            EventPhase::Cut,
            EventKind::TransitionCommitted,
            EventSource::new("punk-flow", "transition_guard"),
            correlation,
            EventResult::new(EventStatus::Succeeded),
        ),
        (Some(guard_code), Some(error_code), None) => EventDraft::new(
            EventPhase::Cut,
            EventKind::TransitionDenied,
            EventSource::new("punk-flow", "transition_guard"),
            correlation,
            EventResult::new(EventStatus::Denied)
                .with_guard_code(guard_code)
                .with_error_code(error_code),
        ),
        _ => EventDraft::new(
            EventPhase::Cut,
            EventKind::TransitionDenied,
            EventSource::new("punk-flow", "transition_guard"),
            correlation,
            EventResult::new(EventStatus::Denied)
                .with_guard_code("FLOW_EVENT_MAPPING_ERROR")
                .with_error_code(FLOW_EVENT_MAPPING_ERROR_CODE),
        ),
    };

    event.with_transition(transition).with_artifacts(artifacts)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowTransitionOutcome {
    Applied {
        next_state: FlowState,
    },
    Denied {
        guard_code: &'static str,
        error_code: &'static str,
        next_allowed_commands: &'static [FlowCommand],
    },
}

const PROJECT_INITIALIZED_COMMANDS: &[FlowCommand] = &[
    FlowCommand::CreateGoal,
    FlowCommand::Block,
    FlowCommand::Escalate,
    FlowCommand::Cancel,
];
const GOAL_CREATED_COMMANDS: &[FlowCommand] = &[
    FlowCommand::DraftContract,
    FlowCommand::Block,
    FlowCommand::Escalate,
    FlowCommand::Cancel,
];
const CONTRACT_DRAFTED_COMMANDS: &[FlowCommand] = &[
    FlowCommand::RequestApproval,
    FlowCommand::Block,
    FlowCommand::Escalate,
    FlowCommand::Cancel,
];
const AWAITING_APPROVAL_COMMANDS: &[FlowCommand] = &[
    FlowCommand::Approve,
    FlowCommand::Block,
    FlowCommand::Escalate,
    FlowCommand::Cancel,
];
const APPROVED_COMMANDS: &[FlowCommand] = &[
    FlowCommand::StartRun,
    FlowCommand::Block,
    FlowCommand::Escalate,
    FlowCommand::Cancel,
];
const RUNNING_COMMANDS: &[FlowCommand] = &[
    FlowCommand::WriteReceipt,
    FlowCommand::Block,
    FlowCommand::Escalate,
    FlowCommand::Cancel,
];
const RECEIPT_WRITTEN_COMMANDS: &[FlowCommand] = &[
    FlowCommand::WriteDecision,
    FlowCommand::Block,
    FlowCommand::Escalate,
    FlowCommand::Cancel,
];
const DECISION_WRITTEN_COMMANDS: &[FlowCommand] = &[
    FlowCommand::WriteProof,
    FlowCommand::MarkReported,
    FlowCommand::Block,
    FlowCommand::Escalate,
    FlowCommand::Cancel,
];
const PROOF_WRITTEN_COMMANDS: &[FlowCommand] = &[
    FlowCommand::MarkReported,
    FlowCommand::Block,
    FlowCommand::Escalate,
    FlowCommand::Cancel,
];
const REPORTED_COMMANDS: &[FlowCommand] = &[
    FlowCommand::Close,
    FlowCommand::Block,
    FlowCommand::Escalate,
    FlowCommand::Cancel,
];
const CLOSED_COMMANDS: &[FlowCommand] = &[];
const BLOCKED_COMMANDS: &[FlowCommand] = &[FlowCommand::Escalate, FlowCommand::Cancel];
const ESCALATED_COMMANDS: &[FlowCommand] = &[FlowCommand::Cancel];
const CANCELLED_COMMANDS: &[FlowCommand] = &[];

fn guard_code_for(state: FlowState, command: FlowCommand) -> &'static str {
    match (state, command) {
        (FlowState::AwaitingApproval, FlowCommand::StartRun) => "CUT_REQUIRES_APPROVED_CONTRACT",
        (FlowState::ReceiptWritten, FlowCommand::WriteProof) => "PROOF_REQUIRES_DECISION",
        (_, FlowCommand::Close) => "FLOW_REQUIRES_REPORTED_STATE",
        _ => "FLOW_TRANSITION_DENIED",
    }
}

pub fn allowed_commands_for(state: FlowState) -> &'static [FlowCommand] {
    match state {
        FlowState::ProjectInitialized => PROJECT_INITIALIZED_COMMANDS,
        FlowState::GoalCreated => GOAL_CREATED_COMMANDS,
        FlowState::ContractDrafted => CONTRACT_DRAFTED_COMMANDS,
        FlowState::AwaitingApproval => AWAITING_APPROVAL_COMMANDS,
        FlowState::Approved => APPROVED_COMMANDS,
        FlowState::Running => RUNNING_COMMANDS,
        FlowState::ReceiptWritten => RECEIPT_WRITTEN_COMMANDS,
        FlowState::DecisionWritten => DECISION_WRITTEN_COMMANDS,
        FlowState::ProofWritten => PROOF_WRITTEN_COMMANDS,
        FlowState::Reported => REPORTED_COMMANDS,
        FlowState::Closed => CLOSED_COMMANDS,
        FlowState::Blocked => BLOCKED_COMMANDS,
        FlowState::Escalated => ESCALATED_COMMANDS,
        FlowState::Cancelled => CANCELLED_COMMANDS,
    }
}

fn next_state_for(state: FlowState, command: FlowCommand) -> Option<FlowState> {
    match (state, command) {
        (FlowState::ProjectInitialized, FlowCommand::CreateGoal) => Some(FlowState::GoalCreated),
        (FlowState::GoalCreated, FlowCommand::DraftContract) => Some(FlowState::ContractDrafted),
        (FlowState::ContractDrafted, FlowCommand::RequestApproval) => {
            Some(FlowState::AwaitingApproval)
        }
        (FlowState::AwaitingApproval, FlowCommand::Approve) => Some(FlowState::Approved),
        (FlowState::Approved, FlowCommand::StartRun) => Some(FlowState::Running),
        (FlowState::Running, FlowCommand::WriteReceipt) => Some(FlowState::ReceiptWritten),
        (FlowState::ReceiptWritten, FlowCommand::WriteDecision) => Some(FlowState::DecisionWritten),
        (FlowState::DecisionWritten, FlowCommand::WriteProof) => Some(FlowState::ProofWritten),
        (FlowState::DecisionWritten, FlowCommand::MarkReported) => Some(FlowState::Reported),
        (FlowState::ProofWritten, FlowCommand::MarkReported) => Some(FlowState::Reported),
        (FlowState::Reported, FlowCommand::Close) => Some(FlowState::Closed),

        (FlowState::ProjectInitialized, FlowCommand::Block)
        | (FlowState::GoalCreated, FlowCommand::Block)
        | (FlowState::ContractDrafted, FlowCommand::Block)
        | (FlowState::AwaitingApproval, FlowCommand::Block)
        | (FlowState::Approved, FlowCommand::Block)
        | (FlowState::Running, FlowCommand::Block)
        | (FlowState::ReceiptWritten, FlowCommand::Block)
        | (FlowState::DecisionWritten, FlowCommand::Block)
        | (FlowState::ProofWritten, FlowCommand::Block)
        | (FlowState::Reported, FlowCommand::Block) => Some(FlowState::Blocked),

        (FlowState::ProjectInitialized, FlowCommand::Escalate)
        | (FlowState::GoalCreated, FlowCommand::Escalate)
        | (FlowState::ContractDrafted, FlowCommand::Escalate)
        | (FlowState::AwaitingApproval, FlowCommand::Escalate)
        | (FlowState::Approved, FlowCommand::Escalate)
        | (FlowState::Running, FlowCommand::Escalate)
        | (FlowState::ReceiptWritten, FlowCommand::Escalate)
        | (FlowState::DecisionWritten, FlowCommand::Escalate)
        | (FlowState::ProofWritten, FlowCommand::Escalate)
        | (FlowState::Reported, FlowCommand::Escalate)
        | (FlowState::Blocked, FlowCommand::Escalate) => Some(FlowState::Escalated),

        (FlowState::ProjectInitialized, FlowCommand::Cancel)
        | (FlowState::GoalCreated, FlowCommand::Cancel)
        | (FlowState::ContractDrafted, FlowCommand::Cancel)
        | (FlowState::AwaitingApproval, FlowCommand::Cancel)
        | (FlowState::Approved, FlowCommand::Cancel)
        | (FlowState::Running, FlowCommand::Cancel)
        | (FlowState::ReceiptWritten, FlowCommand::Cancel)
        | (FlowState::DecisionWritten, FlowCommand::Cancel)
        | (FlowState::ProofWritten, FlowCommand::Cancel)
        | (FlowState::Reported, FlowCommand::Cancel)
        | (FlowState::Blocked, FlowCommand::Cancel)
        | (FlowState::Escalated, FlowCommand::Cancel) => Some(FlowState::Cancelled),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        transition_attempt_event_draft, FlowCommand, FlowInstance, FlowState, FlowTransitionOutcome,
    };
    use punk_contract::{
        approve_contract, validate_contract, ContractDraft, ContractError, ContractId,
        ContractScope, ContractStatus,
    };
    use punk_domain::{ContractRef, RunId, RunReceiptId, RunScopeRef};
    use punk_events::{EventKind, EventStatus};

    fn valid_contract_scope() -> ContractScope {
        ContractScope::new()
            .with_ref("work/goals/goal_add_contract_lifecycle_minimal.md")
            .with_ref("docs/product/CONTRACT-TRACKER.md")
    }

    fn valid_contract_draft() -> ContractDraft {
        ContractDraft::new(
            ContractId::new("contract_flow_001").expect("contract id should be valid"),
            "Approved contract authorizes run transition",
            valid_contract_scope(),
        )
    }

    #[test]
    fn illegal_transition_is_denied_before_approval() {
        let instance = FlowInstance::new(FlowState::AwaitingApproval);
        let error = instance
            .transition(FlowCommand::StartRun)
            .expect_err("starting a run before approval must fail");

        assert_eq!(error.current_state, FlowState::AwaitingApproval);
        assert_eq!(error.attempted_command, FlowCommand::StartRun);
        assert_eq!(error.next_allowed_commands, instance.allowed_commands());
        assert!(error.next_allowed_commands.contains(&FlowCommand::Approve));
    }

    #[test]
    fn allowed_transition_returns_next_state() {
        let instance = FlowInstance::new(FlowState::AwaitingApproval);
        let next = instance
            .transition(FlowCommand::Approve)
            .expect("approval should be allowed from awaiting_approval");

        assert_eq!(next.state(), FlowState::Approved);
    }

    #[test]
    fn proof_cannot_be_written_before_decision() {
        let instance = FlowInstance::new(FlowState::ReceiptWritten);
        let error = instance
            .transition(FlowCommand::WriteProof)
            .expect_err("proof should require a written decision");

        assert!(error
            .next_allowed_commands
            .contains(&FlowCommand::WriteDecision));
    }

    #[test]
    fn denied_transition_attempt_keeps_state_and_reports_guard_evidence() {
        let instance = FlowInstance::new(FlowState::AwaitingApproval);
        let attempt = instance.attempt_transition(FlowCommand::StartRun);

        assert_eq!(instance.state(), FlowState::AwaitingApproval);
        assert_eq!(attempt.current_state(), FlowState::AwaitingApproval);
        assert_eq!(attempt.attempted_command(), FlowCommand::StartRun);
        assert_eq!(
            attempt.outcome(),
            FlowTransitionOutcome::Denied {
                guard_code: "CUT_REQUIRES_APPROVED_CONTRACT",
                error_code: "E_FLOW_ILLEGAL_TRANSITION",
                next_allowed_commands: instance.allowed_commands(),
            }
        );
        assert_eq!(attempt.guard_code(), Some("CUT_REQUIRES_APPROVED_CONTRACT"));
        assert_eq!(attempt.error_code(), Some("E_FLOW_ILLEGAL_TRANSITION"));
        assert_eq!(attempt.next_state(), None);
    }

    #[test]
    fn applied_transition_attempt_reports_next_state() {
        let instance = FlowInstance::new(FlowState::AwaitingApproval);
        let attempt = instance.attempt_transition(FlowCommand::Approve);

        assert_eq!(instance.state(), FlowState::AwaitingApproval);
        assert_eq!(
            attempt.outcome(),
            FlowTransitionOutcome::Applied {
                next_state: FlowState::Approved
            }
        );
        assert_eq!(attempt.next_state(), Some(FlowState::Approved));
        assert_eq!(
            attempt
                .into_result()
                .expect("transition should succeed")
                .state(),
            FlowState::Approved
        );
    }

    #[test]
    fn allowed_transition_produces_applied_event_evidence() {
        let attempt =
            FlowInstance::new(FlowState::AwaitingApproval).attempt_transition(FlowCommand::Approve);
        let draft = transition_attempt_event_draft(
            &attempt,
            "flow_local_approval",
            Some("work/goals/goal_connect_flow_transitions_to_event_log.md"),
        );

        assert_eq!(draft.kind, EventKind::TransitionCommitted);
        assert_eq!(draft.result.status, EventStatus::Succeeded);
        assert_eq!(draft.result.guard_code, None);
        assert_eq!(draft.result.error_code, None);
        assert_eq!(
            draft.transition.from_state.as_deref(),
            Some("AwaitingApproval")
        );
        assert_eq!(draft.transition.command.as_deref(), Some("Approve"));
        assert_eq!(draft.transition.to_state.as_deref(), Some("Approved"));
        assert_eq!(
            draft.correlation.goal_ref.as_deref(),
            Some("work/goals/goal_connect_flow_transitions_to_event_log.md")
        );
        assert_eq!(
            draft.artifacts.refs,
            vec!["work/goals/goal_connect_flow_transitions_to_event_log.md".to_string()]
        );
    }

    #[test]
    fn denied_transition_produces_guard_denial_event_evidence() {
        let instance = FlowInstance::new(FlowState::AwaitingApproval);
        let attempt = instance.attempt_transition(FlowCommand::StartRun);
        let draft = transition_attempt_event_draft(&attempt, "flow_local_denied", None);

        assert_eq!(instance.state(), FlowState::AwaitingApproval);
        assert_eq!(draft.kind, EventKind::TransitionDenied);
        assert_eq!(draft.result.status, EventStatus::Denied);
        assert_eq!(
            draft.result.guard_code.as_deref(),
            Some("CUT_REQUIRES_APPROVED_CONTRACT")
        );
        assert_eq!(
            draft.result.error_code.as_deref(),
            Some("E_FLOW_ILLEGAL_TRANSITION")
        );
        assert_eq!(
            draft.transition.from_state.as_deref(),
            Some("AwaitingApproval")
        );
        assert_eq!(draft.transition.command.as_deref(), Some("StartRun"));
        assert_eq!(draft.transition.to_state, None);
        assert!(!draft.kind.as_str().contains("decision"));
        assert!(!draft.kind.as_str().contains("gate"));
    }

    #[test]
    fn approved_for_run_contract_allows_start_run_transition() {
        let contract = approve_contract(valid_contract_draft()).expect("contract should approve");
        let instance = FlowInstance::new(FlowState::Approved);
        let next = instance
            .transition_with_contract(
                FlowCommand::StartRun,
                contract.status(),
                contract.scope_valid(),
            )
            .expect("approved contract should authorize start run");

        assert_eq!(next.state(), FlowState::Running);
    }

    #[test]
    fn draft_contract_denies_start_run_transition() {
        let draft = valid_contract_draft();
        let instance = FlowInstance::new(FlowState::Approved);
        let attempt = instance.attempt_transition_with_contract(
            FlowCommand::StartRun,
            ContractStatus::Draft,
            !draft.scope().is_empty(),
        );

        assert_eq!(instance.state(), FlowState::Approved);
        assert_eq!(
            attempt.outcome(),
            FlowTransitionOutcome::Denied {
                guard_code: "RUN_REQUIRES_APPROVED_FOR_RUN_CONTRACT",
                error_code: "E_FLOW_ILLEGAL_TRANSITION",
                next_allowed_commands: instance.allowed_commands(),
            }
        );
        assert_eq!(attempt.next_state(), None);
    }

    #[test]
    fn invalid_contract_cannot_authorize_run_transition() {
        let invalid_draft = ContractDraft::new(
            ContractId::new("contract_flow_002").expect("contract id should be valid"),
            "Scope is required",
            ContractScope::new(),
        );
        let instance = FlowInstance::new(FlowState::Approved);
        let attempt = instance.attempt_transition_with_contract(
            FlowCommand::StartRun,
            ContractStatus::Draft,
            !invalid_draft.scope().is_empty(),
        );

        assert_eq!(
            validate_contract(&invalid_draft),
            Err(ContractError::EmptyScope)
        );
        assert_eq!(instance.state(), FlowState::Approved);
        assert_eq!(
            attempt.outcome(),
            FlowTransitionOutcome::Denied {
                guard_code: "RUN_REQUIRES_EXPLICIT_CONTRACT_SCOPE",
                error_code: "E_FLOW_ILLEGAL_TRANSITION",
                next_allowed_commands: instance.allowed_commands(),
            }
        );
    }

    #[test]
    fn contract_authorized_start_run_produces_guard_evidence_not_decision() {
        let contract = approve_contract(valid_contract_draft()).expect("contract should approve");
        let attempt = FlowInstance::new(FlowState::Approved).attempt_transition_with_contract(
            FlowCommand::StartRun,
            contract.status(),
            contract.scope_valid(),
        );
        let draft = transition_attempt_event_draft(
            &attempt,
            "flow_contract_authorized",
            Some("work/goals/goal_connect_contract_to_flow_state.md"),
        );

        assert_eq!(draft.kind, EventKind::TransitionCommitted);
        assert_eq!(draft.result.status, EventStatus::Succeeded);
        assert!(!draft.kind.as_str().contains("decision"));
        assert!(!draft.kind.as_str().contains("gate"));
    }

    #[test]
    fn draft_contract_denial_stays_guard_evidence_not_final_decision() {
        let draft_contract = valid_contract_draft();
        let attempt = FlowInstance::new(FlowState::Approved).attempt_transition_with_contract(
            FlowCommand::StartRun,
            ContractStatus::Draft,
            !draft_contract.scope().is_empty(),
        );
        let draft = transition_attempt_event_draft(&attempt, "flow_contract_denied", None);

        assert_eq!(draft.kind, EventKind::TransitionDenied);
        assert_eq!(draft.result.status, EventStatus::Denied);
        assert_eq!(
            draft.result.guard_code.as_deref(),
            Some("RUN_REQUIRES_APPROVED_FOR_RUN_CONTRACT")
        );
        assert!(!draft.kind.as_str().contains("decision"));
        assert!(!draft.kind.as_str().contains("gate"));
    }

    #[test]
    fn approved_for_run_contract_can_produce_receipt_evidence_for_start_run() {
        let contract = approve_contract(valid_contract_draft()).expect("contract should approve");
        let receipt_attempt = FlowInstance::new(FlowState::Approved)
            .attempt_transition_with_contract_receipt(
                FlowCommand::StartRun,
                contract.status(),
                contract.scope_valid(),
                RunReceiptId::new("receipt_flow_001").expect("receipt id should be valid"),
                ContractRef::new(contract.id().as_str()).expect("contract ref should be valid"),
                RunId::new("run_flow_001").expect("run id should be valid"),
                RunScopeRef::new("work/goals/goal_connect_run_receipt_to_contract_flow.md")
                    .expect("run scope ref should be valid"),
            );

        assert_eq!(receipt_attempt.transition().next_state(), Some(FlowState::Running));
        let receipt = receipt_attempt
            .receipt()
            .expect("allowed start run should produce receipt evidence");
        assert_eq!(receipt.id().as_str(), "receipt_flow_001");
        assert_eq!(receipt.contract_ref().as_str(), contract.id().as_str());
        assert_eq!(receipt.run_id().as_str(), "run_flow_001");
        assert_eq!(
            receipt.run_scope_ref().as_str(),
            "work/goals/goal_connect_run_receipt_to_contract_flow.md"
        );
        assert!(receipt.run_evidence_only());
        assert!(!receipt.boundary().implies_final_acceptance);
        assert!(!receipt.boundary().requires_runtime_storage);
    }

    #[test]
    fn draft_contract_cannot_produce_receipt() {
        let receipt_attempt = FlowInstance::new(FlowState::Approved)
            .attempt_transition_with_contract_receipt(
                FlowCommand::StartRun,
                ContractStatus::Draft,
                true,
                RunReceiptId::new("receipt_flow_002").expect("receipt id should be valid"),
                ContractRef::new("contract_flow_002").expect("contract ref should be valid"),
                RunId::new("run_flow_002").expect("run id should be valid"),
                RunScopeRef::new("work/goals/goal_connect_run_receipt_to_contract_flow.md")
                    .expect("run scope ref should be valid"),
            );

        assert_eq!(receipt_attempt.transition().next_state(), None);
        assert!(receipt_attempt.receipt().is_none());
    }

    #[test]
    fn invalid_scope_cannot_produce_receipt() {
        let contract = approve_contract(valid_contract_draft()).expect("contract should approve");
        let receipt_attempt = FlowInstance::new(FlowState::Approved)
            .attempt_transition_with_contract_receipt(
                FlowCommand::StartRun,
                contract.status(),
                false,
                RunReceiptId::new("receipt_flow_003").expect("receipt id should be valid"),
                ContractRef::new(contract.id().as_str()).expect("contract ref should be valid"),
                RunId::new("run_flow_003").expect("run id should be valid"),
                RunScopeRef::new("work/goals/goal_connect_run_receipt_to_contract_flow.md")
                    .expect("run scope ref should be valid"),
            );

        assert_eq!(receipt_attempt.transition().next_state(), None);
        assert_eq!(
            receipt_attempt.transition().guard_code(),
            Some("RUN_REQUIRES_EXPLICIT_CONTRACT_SCOPE")
        );
        assert!(receipt_attempt.receipt().is_none());
    }

    #[test]
    fn denied_transition_with_receipt_context_does_not_mutate_flow_state_or_produce_receipt() {
        let instance = FlowInstance::new(FlowState::Approved);
        let receipt_attempt = instance.attempt_transition_with_contract_receipt(
            FlowCommand::StartRun,
            ContractStatus::Draft,
            true,
            RunReceiptId::new("receipt_flow_004").expect("receipt id should be valid"),
            ContractRef::new("contract_flow_004").expect("contract ref should be valid"),
            RunId::new("run_flow_004").expect("run id should be valid"),
            RunScopeRef::new("work/goals/goal_connect_run_receipt_to_contract_flow.md")
                .expect("run scope ref should be valid"),
        );

        assert_eq!(instance.state(), FlowState::Approved);
        assert_eq!(receipt_attempt.transition().next_state(), None);
        assert!(receipt_attempt.receipt().is_none());
    }

    #[test]
    fn denied_transition_with_receipt_context_stays_event_evidence_only() {
        let receipt_attempt = FlowInstance::new(FlowState::Approved)
            .attempt_transition_with_contract_receipt(
                FlowCommand::StartRun,
                ContractStatus::Draft,
                true,
                RunReceiptId::new("receipt_flow_005").expect("receipt id should be valid"),
                ContractRef::new("contract_flow_005").expect("contract ref should be valid"),
                RunId::new("run_flow_005").expect("run id should be valid"),
                RunScopeRef::new("work/goals/goal_connect_run_receipt_to_contract_flow.md")
                    .expect("run scope ref should be valid"),
            );
        let draft = transition_attempt_event_draft(
            receipt_attempt.transition(),
            "flow_receipt_denied",
            Some("work/goals/goal_connect_run_receipt_to_contract_flow.md"),
        );

        assert_eq!(draft.kind, EventKind::TransitionDenied);
        assert_eq!(draft.result.status, EventStatus::Denied);
        assert!(receipt_attempt.receipt().is_none());
    }
}
