//! Minimal deterministic smoke eval harness for the current Punk core.
//!
//! This Phase 1/2 bridge stays library-first. It assesses the existing flow
//! and event kernels without activating `.punk/` runtime state, baseline
//! comparison, waiver storage, or a full eval platform.

use punk_events::{schema_fixture, MemoryEventLog};
use punk_flow::{transition_attempt_event_draft, FlowCommand, FlowInstance, FlowState};

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const SMOKE_SUITE_ID: &str = "smoke.v0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmokeStatus {
    Pass,
    Fail,
}

impl SmokeStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmokeCaseResult {
    pub case_id: &'static str,
    pub summary: &'static str,
    pub status: SmokeStatus,
    pub assessment: String,
}

impl SmokeCaseResult {
    fn pass(case_id: &'static str, summary: &'static str, assessment: impl Into<String>) -> Self {
        Self {
            case_id,
            summary,
            status: SmokeStatus::Pass,
            assessment: assessment.into(),
        }
    }

    fn fail(case_id: &'static str, summary: &'static str, assessment: impl Into<String>) -> Self {
        Self {
            case_id,
            summary,
            status: SmokeStatus::Fail,
            assessment: assessment.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmokeSuiteReport {
    suite_id: &'static str,
    cases: Vec<SmokeCaseResult>,
}

impl SmokeSuiteReport {
    pub fn suite_id(&self) -> &'static str {
        self.suite_id
    }

    pub fn cases(&self) -> &[SmokeCaseResult] {
        &self.cases
    }

    pub fn passed(&self) -> bool {
        self.cases
            .iter()
            .all(|case| case.status == SmokeStatus::Pass)
    }
}

pub fn run_smoke_suite() -> SmokeSuiteReport {
    SmokeSuiteReport {
        suite_id: SMOKE_SUITE_ID,
        cases: vec![
            eval_flow_allows_approval_transition(),
            eval_flow_denies_run_before_approval(),
            eval_denied_transition_preserves_state(),
            eval_flow_transition_produces_event_evidence(),
            eval_event_log_is_append_only(),
        ],
    }
}

fn eval_flow_allows_approval_transition() -> SmokeCaseResult {
    let instance = FlowInstance::new(FlowState::AwaitingApproval);
    match instance.transition(FlowCommand::Approve) {
        Ok(next) if next.state() == FlowState::Approved => SmokeCaseResult::pass(
            "eval_flow_allows_approval_transition",
            "allowed transition remains deterministic",
            "smoke_result: pass; assessment: approval still advances AwaitingApproval -> Approved",
        ),
        Ok(next) => SmokeCaseResult::fail(
            "eval_flow_allows_approval_transition",
            "allowed transition remains deterministic",
            format!(
                "smoke_result: fail; assessment: approval returned unexpected next state {}",
                next.state().as_str()
            ),
        ),
        Err(error) => SmokeCaseResult::fail(
            "eval_flow_allows_approval_transition",
            "allowed transition remains deterministic",
            format!(
                "smoke_result: fail; assessment: approval was denied with next allowed commands {}",
                format_commands(error.next_allowed_commands)
            ),
        ),
    }
}

fn eval_flow_denies_run_before_approval() -> SmokeCaseResult {
    let instance = FlowInstance::new(FlowState::AwaitingApproval);
    match instance.transition(FlowCommand::StartRun) {
        Ok(next) => SmokeCaseResult::fail(
            "eval_flow_denies_run_before_approval",
            "illegal run transition stays denied",
            format!(
                "smoke_result: fail; assessment: StartRun unexpectedly moved to {}",
                next.state().as_str()
            ),
        ),
        Err(error)
            if error.current_state == FlowState::AwaitingApproval
                && error.attempted_command == FlowCommand::StartRun
                && error.next_allowed_commands.contains(&FlowCommand::Approve) =>
        {
            SmokeCaseResult::pass(
                "eval_flow_denies_run_before_approval",
                "illegal run transition stays denied",
                "smoke_result: pass; assessment: StartRun remains denied before approval and still points to Approve as the next allowed command",
            )
        }
        Err(error) => SmokeCaseResult::fail(
            "eval_flow_denies_run_before_approval",
            "illegal run transition stays denied",
            format!(
                "smoke_result: fail; assessment: denial shape drifted; next allowed commands are {}",
                format_commands(error.next_allowed_commands)
            ),
        ),
    }
}

fn eval_denied_transition_preserves_state() -> SmokeCaseResult {
    let instance = FlowInstance::new(FlowState::AwaitingApproval);
    let attempt = instance.attempt_transition(FlowCommand::StartRun);

    if instance.state() == FlowState::AwaitingApproval && attempt.next_state().is_none() {
        SmokeCaseResult::pass(
            "eval_denied_transition_preserves_state",
            "denied transitions do not mutate state",
            "smoke_result: pass; assessment: denied StartRun attempt leaves AwaitingApproval unchanged",
        )
    } else {
        SmokeCaseResult::fail(
            "eval_denied_transition_preserves_state",
            "denied transitions do not mutate state",
            "smoke_result: fail; assessment: denied transition changed state evidence unexpectedly",
        )
    }
}

fn eval_flow_transition_produces_event_evidence() -> SmokeCaseResult {
    let attempt = FlowInstance::new(FlowState::AwaitingApproval)
        .attempt_transition(FlowCommand::StartRun);
    let draft = transition_attempt_event_draft(
        &attempt,
        "smoke_eval_preview",
        Some("work/goals/goal_add_smoke_eval_harness.md"),
    );

    if draft.kind.as_str() == "transition_denied"
        && draft.result.status.as_str() == "denied"
        && draft.result.guard_code.as_deref() == Some("CUT_REQUIRES_APPROVED_CONTRACT")
    {
        SmokeCaseResult::pass(
            "eval_flow_transition_produces_event_evidence",
            "transition attempts still emit event evidence",
            "smoke_result: pass; assessment: denied flow attempt produces guard-denial event evidence without decision authority",
        )
    } else {
        SmokeCaseResult::fail(
            "eval_flow_transition_produces_event_evidence",
            "transition attempts still emit event evidence",
            format!(
                "smoke_result: fail; assessment: event evidence drifted to kind={} status={} guard_code={:?}",
                draft.kind.as_str(),
                draft.result.status.as_str(),
                draft.result.guard_code,
            ),
        )
    }
}

fn eval_event_log_is_append_only() -> SmokeCaseResult {
    let mut log = MemoryEventLog::default();
    let first = log.append(schema_fixture());

    let second_attempt = FlowInstance::new(FlowState::AwaitingApproval)
        .attempt_transition(FlowCommand::Approve);
    let second_draft = transition_attempt_event_draft(&second_attempt, "smoke_eval_append", None);
    let second = log.append(second_draft);

    match (first, second) {
        (Ok(first), Ok(second))
            if first.sequence == 1
                && second.sequence == 2
                && log.events().len() == 2
                && first.event_id == "evt_0000000000000001"
                && second.event_id == "evt_0000000000000002" =>
        {
            SmokeCaseResult::pass(
                "eval_event_log_is_append_only",
                "append-only event log stays monotonic",
                "smoke_result: pass; assessment: append-only log preserved prior records and emitted monotonic ids/sequences",
            )
        }
        (Ok(first), Ok(second)) => SmokeCaseResult::fail(
            "eval_event_log_is_append_only",
            "append-only event log stays monotonic",
            format!(
                "smoke_result: fail; assessment: append-only evidence drifted; first={} second={} len={}",
                first.event_id,
                second.event_id,
                log.events().len(),
            ),
        ),
        (Err(error), _) | (_, Err(error)) => SmokeCaseResult::fail(
            "eval_event_log_is_append_only",
            "append-only event log stays monotonic",
            format!("smoke_result: fail; assessment: append failed with {error:?}"),
        ),
    }
}

fn format_commands(commands: &[FlowCommand]) -> String {
    commands
        .iter()
        .map(|command| command.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::{run_smoke_suite, SmokeStatus, SMOKE_SUITE_ID};

    #[test]
    fn smoke_suite_reports_expected_suite_id() {
        let report = run_smoke_suite();

        assert_eq!(report.suite_id(), SMOKE_SUITE_ID);
        assert_eq!(report.cases().len(), 5);
    }

    #[test]
    fn smoke_suite_passes_against_current_flow_and_event_kernels() {
        let report = run_smoke_suite();

        assert!(report.passed(), "all current smoke cases should pass");
        assert!(report
            .cases()
            .iter()
            .all(|case| case.status == SmokeStatus::Pass));
    }

    #[test]
    fn smoke_suite_uses_assessment_not_decision_wording() {
        let report = run_smoke_suite();

        for case in report.cases() {
            assert!(case.assessment.contains("smoke_result:"));
            assert!(case.assessment.contains("assessment:"));
            assert!(!case.assessment.contains("accepted"));
            assert!(!case.assessment.contains("approved"));
            assert!(!case.assessment.contains("gate decision"));
            assert!(!case.assessment.contains("proof complete"));
        }
    }
}
