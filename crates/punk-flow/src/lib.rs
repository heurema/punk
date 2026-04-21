//! Minimal pure flow-state kernel for Punk Phase 1.
//!
//! This crate intentionally stays deterministic and side-effect free in this
//! first implementation step. It models states, commands, and transition
//! guards only. No persistence, event writing, CLI behavior, or `.punk/`
//! runtime state is introduced here.

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

    pub fn transition(self, command: FlowCommand) -> Result<Self, TransitionError> {
        next_state_for(self.state, command)
            .map(Self::new)
            .ok_or_else(|| TransitionError::new(self.state, command))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
        (FlowState::ContractDrafted, FlowCommand::RequestApproval) => Some(FlowState::AwaitingApproval),
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
    use super::{FlowCommand, FlowInstance, FlowState};

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
}
