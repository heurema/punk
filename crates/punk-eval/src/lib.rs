//! Minimal deterministic smoke eval harness for the current Punk core.
//!
//! This Phase 1/2 bridge stays library-first. It assesses the existing flow
//! and event kernels without activating `.punk/` runtime state, baseline
//! comparison, waiver storage, or a full eval platform.

use std::fmt::Write as _;

use punk_events::{schema_fixture, MemoryEventLog};
use punk_flow::{transition_attempt_event_draft, FlowCommand, FlowInstance, FlowState};

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const SMOKE_SUITE_ID: &str = "smoke.v0";
pub const SMOKE_REPORT_SCHEMA_VERSION: &str = "smoke-eval-report.v0.1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmokeEvalStatus {
    Pass,
    Fail,
}

impl SmokeEvalStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmokeEvalCaseResult {
    pub case_id: &'static str,
    pub summary: &'static str,
    pub status: SmokeEvalStatus,
    pub assessment: String,
}

impl SmokeEvalCaseResult {
    fn pass(case_id: &'static str, summary: &'static str, assessment: impl Into<String>) -> Self {
        Self {
            case_id,
            summary,
            status: SmokeEvalStatus::Pass,
            assessment: assessment.into(),
        }
    }

    fn fail(case_id: &'static str, summary: &'static str, assessment: impl Into<String>) -> Self {
        Self {
            case_id,
            summary,
            status: SmokeEvalStatus::Fail,
            assessment: assessment.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmokeEvalSummary {
    suite_id: &'static str,
    smoke_result: SmokeEvalStatus,
    assessment: String,
    mode: &'static str,
    runtime_persistence: &'static str,
    report_storage: &'static str,
}

impl SmokeEvalSummary {
    pub fn suite_id(&self) -> &'static str {
        self.suite_id
    }

    pub fn smoke_result(&self) -> SmokeEvalStatus {
        self.smoke_result
    }

    pub fn assessment(&self) -> &str {
        &self.assessment
    }

    pub fn mode(&self) -> &'static str {
        self.mode
    }

    pub fn runtime_persistence(&self) -> &'static str {
        self.runtime_persistence
    }

    pub fn report_storage(&self) -> &'static str {
        self.report_storage
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmokeEvalReport {
    summary: SmokeEvalSummary,
    cases: Vec<SmokeEvalCaseResult>,
    boundary_notes: Vec<&'static str>,
    deferred_notes: Vec<&'static str>,
}

impl SmokeEvalReport {
    pub fn summary(&self) -> &SmokeEvalSummary {
        &self.summary
    }

    pub fn suite_id(&self) -> &'static str {
        self.summary.suite_id()
    }

    pub fn smoke_result(&self) -> SmokeEvalStatus {
        self.summary.smoke_result()
    }

    pub fn assessment(&self) -> &str {
        self.summary.assessment()
    }

    pub fn mode(&self) -> &'static str {
        self.summary.mode()
    }

    pub fn runtime_persistence(&self) -> &'static str {
        self.summary.runtime_persistence()
    }

    pub fn report_storage(&self) -> &'static str {
        self.summary.report_storage()
    }

    pub fn cases(&self) -> &[SmokeEvalCaseResult] {
        &self.cases
    }

    pub fn boundary_notes(&self) -> &[&'static str] {
        &self.boundary_notes
    }

    pub fn deferred_notes(&self) -> &[&'static str] {
        &self.deferred_notes
    }

    pub fn passed(&self) -> bool {
        self.smoke_result() == SmokeEvalStatus::Pass
    }

    pub fn exit_code(&self) -> u8 {
        if self.passed() {
            0
        } else {
            1
        }
    }

    pub fn render_json(&self) -> String {
        let mut output = String::new();
        output.push_str("{\n");

        write_json_field(
            &mut output,
            1,
            "schema_version",
            JsonValue::String(SMOKE_REPORT_SCHEMA_VERSION),
            true,
        );
        write_json_field(
            &mut output,
            1,
            "suite_id",
            JsonValue::String(self.suite_id()),
            true,
        );
        write_json_field(&mut output, 1, "run_id", JsonValue::Null, true);
        write_json_field(
            &mut output,
            1,
            "smoke_result",
            JsonValue::String(self.smoke_result().as_str()),
            true,
        );
        write_json_field(&mut output, 1, "mode", JsonValue::String(self.mode()), true);
        write_json_field(
            &mut output,
            1,
            "runtime_persistence",
            JsonValue::String(self.runtime_persistence()),
            true,
        );
        write_json_field(
            &mut output,
            1,
            "report_storage",
            JsonValue::String(self.report_storage()),
            true,
        );

        output.push_str("  \"case_results\": [\n");
        for (index, case) in self.cases().iter().enumerate() {
            output.push_str("    {\n");
            write_json_field(
                &mut output,
                3,
                "case_id",
                JsonValue::String(case.case_id),
                true,
            );
            write_json_field(
                &mut output,
                3,
                "status",
                JsonValue::String(case.status.as_str()),
                true,
            );
            write_json_field(
                &mut output,
                3,
                "summary",
                JsonValue::String(case.summary),
                true,
            );
            write_json_field(
                &mut output,
                3,
                "assessment",
                JsonValue::String(case.assessment.as_str()),
                false,
            );
            output.push_str("    }");
            if index + 1 != self.cases().len() {
                output.push(',');
            }
            output.push('\n');
        }
        output.push_str("  ],\n");

        write_json_field(
            &mut output,
            1,
            "boundary_notes",
            JsonValue::StringArray(self.boundary_notes()),
            true,
        );
        write_json_field(
            &mut output,
            1,
            "deferred",
            JsonValue::StringArray(self.deferred_notes()),
            false,
        );
        output.push_str("}");

        output
    }

    pub fn render_human(&self) -> String {
        let mut output = String::new();
        writeln!(&mut output, "punk eval run smoke").expect("writing to String should succeed");
        writeln!(&mut output, "mode: {}", self.mode()).expect("writing to String should succeed");
        writeln!(
            &mut output,
            "runtime_persistence: {}",
            self.runtime_persistence()
        )
        .expect("writing to String should succeed");
        writeln!(&mut output, "report_storage: {}", self.report_storage())
            .expect("writing to String should succeed");
        writeln!(&mut output, "suite_id: {}", self.suite_id())
            .expect("writing to String should succeed");
        writeln!(
            &mut output,
            "smoke_result: {}",
            self.smoke_result().as_str()
        )
        .expect("writing to String should succeed");
        writeln!(&mut output, "assessment: {}", self.assessment())
            .expect("writing to String should succeed");
        writeln!(&mut output, "case_results:").expect("writing to String should succeed");

        for case in self.cases() {
            writeln!(&mut output, "  - id: {}", case.case_id)
                .expect("writing to String should succeed");
            writeln!(&mut output, "    status: {}", case.status.as_str())
                .expect("writing to String should succeed");
            writeln!(&mut output, "    summary: {}", case.summary)
                .expect("writing to String should succeed");
            writeln!(&mut output, "    assessment: {}", case.assessment)
                .expect("writing to String should succeed");
        }

        writeln!(&mut output, "notes:").expect("writing to String should succeed");
        for note in self.boundary_notes() {
            writeln!(&mut output, "  - {note}").expect("writing to String should succeed");
        }

        writeln!(&mut output, "deferred:").expect("writing to String should succeed");
        for note in self.deferred_notes() {
            writeln!(&mut output, "  - {note}").expect("writing to String should succeed");
        }

        output.trim_end().to_owned()
    }
}

pub fn run_smoke_suite() -> SmokeEvalReport {
    let cases = vec![
        eval_flow_allows_approval_transition(),
        eval_flow_denies_run_before_approval(),
        eval_denied_transition_preserves_state(),
        eval_flow_transition_produces_event_evidence(),
        eval_event_log_is_append_only(),
    ];
    let smoke_result = if cases
        .iter()
        .all(|case| case.status == SmokeEvalStatus::Pass)
    {
        SmokeEvalStatus::Pass
    } else {
        SmokeEvalStatus::Fail
    };
    let assessment = if smoke_result == SmokeEvalStatus::Pass {
        "local deterministic smoke harness passed over current flow and event kernels".to_owned()
    } else {
        "local deterministic smoke harness found one or more failing cases over current flow and event kernels"
            .to_owned()
    };

    SmokeEvalReport {
        summary: SmokeEvalSummary {
            suite_id: SMOKE_SUITE_ID,
            smoke_result,
            assessment,
            mode: "local-smoke-check",
            runtime_persistence: "inactive",
            report_storage: "inactive",
        },
        cases,
        boundary_notes: vec![
            "local assessment only; no authority is written here",
            "no .punk/evals runtime state is read or written",
            "JSON output is opt-in only and does not imply a stable public contract",
        ],
        deferred_notes: vec![
            "baseline, waiver, and stored eval reports are not active",
            "schema validation and export adapters are not active",
        ],
    }
}

enum JsonValue<'a> {
    String(&'a str),
    StringArray(&'a [&'a str]),
    Null,
}

fn write_json_field(
    output: &mut String,
    indent_level: usize,
    key: &str,
    value: JsonValue<'_>,
    trailing_comma: bool,
) {
    output.push_str(&"  ".repeat(indent_level));
    push_json_string(output, key);
    output.push_str(": ");

    match value {
        JsonValue::String(value) => push_json_string(output, value),
        JsonValue::StringArray(values) => {
            output.push('[');
            for (index, value) in values.iter().enumerate() {
                if index > 0 {
                    output.push_str(", ");
                }
                push_json_string(output, value);
            }
            output.push(']');
        }
        JsonValue::Null => output.push_str("null"),
    }

    if trailing_comma {
        output.push(',');
    }
    output.push('\n');
}

fn push_json_string(output: &mut String, value: &str) {
    output.push('"');
    for ch in value.chars() {
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{08}' => output.push_str("\\b"),
            '\u{0C}' => output.push_str("\\f"),
            ch if ch.is_control() => {
                write!(output, "\\u{:04x}", ch as u32).expect("writing to String should succeed");
            }
            ch => output.push(ch),
        }
    }
    output.push('"');
}

fn eval_flow_allows_approval_transition() -> SmokeEvalCaseResult {
    let instance = FlowInstance::new(FlowState::AwaitingApproval);
    match instance.transition(FlowCommand::Approve) {
        Ok(next) if next.state() == FlowState::Approved => SmokeEvalCaseResult::pass(
            "eval_flow_allows_approval_transition",
            "allowed transition remains deterministic",
            "approval still advances AwaitingApproval -> Approved",
        ),
        Ok(next) => SmokeEvalCaseResult::fail(
            "eval_flow_allows_approval_transition",
            "allowed transition remains deterministic",
            format!(
                "approval returned unexpected next state {}",
                next.state().as_str()
            ),
        ),
        Err(error) => SmokeEvalCaseResult::fail(
            "eval_flow_allows_approval_transition",
            "allowed transition remains deterministic",
            format!(
                "approval was denied with next allowed commands {}",
                format_commands(error.next_allowed_commands)
            ),
        ),
    }
}

fn eval_flow_denies_run_before_approval() -> SmokeEvalCaseResult {
    let instance = FlowInstance::new(FlowState::AwaitingApproval);
    match instance.transition(FlowCommand::StartRun) {
        Ok(next) => SmokeEvalCaseResult::fail(
            "eval_flow_denies_run_before_approval",
            "illegal run transition stays denied",
            format!(
                "StartRun unexpectedly moved to {}",
                next.state().as_str()
            ),
        ),
        Err(error)
            if error.current_state == FlowState::AwaitingApproval
                && error.attempted_command == FlowCommand::StartRun
                && error.next_allowed_commands.contains(&FlowCommand::Approve) =>
        {
            SmokeEvalCaseResult::pass(
                "eval_flow_denies_run_before_approval",
                "illegal run transition stays denied",
                "StartRun remains denied before approval and still points to Approve as the next allowed command",
            )
        }
        Err(error) => SmokeEvalCaseResult::fail(
            "eval_flow_denies_run_before_approval",
            "illegal run transition stays denied",
            format!(
                "denial shape drifted; next allowed commands are {}",
                format_commands(error.next_allowed_commands)
            ),
        ),
    }
}

fn eval_denied_transition_preserves_state() -> SmokeEvalCaseResult {
    let instance = FlowInstance::new(FlowState::AwaitingApproval);
    let attempt = instance.attempt_transition(FlowCommand::StartRun);

    if instance.state() == FlowState::AwaitingApproval && attempt.next_state().is_none() {
        SmokeEvalCaseResult::pass(
            "eval_denied_transition_preserves_state",
            "denied transitions do not mutate state",
            "denied StartRun attempt leaves AwaitingApproval unchanged",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_denied_transition_preserves_state",
            "denied transitions do not mutate state",
            "denied transition changed state evidence unexpectedly",
        )
    }
}

fn eval_flow_transition_produces_event_evidence() -> SmokeEvalCaseResult {
    let attempt =
        FlowInstance::new(FlowState::AwaitingApproval).attempt_transition(FlowCommand::StartRun);
    let draft = transition_attempt_event_draft(
        &attempt,
        "smoke_eval_preview",
        Some("work/goals/goal_add_smoke_eval_harness.md"),
    );

    if draft.kind.as_str() == "transition_denied"
        && draft.result.status.as_str() == "denied"
        && draft.result.guard_code.as_deref() == Some("CUT_REQUIRES_APPROVED_CONTRACT")
    {
        SmokeEvalCaseResult::pass(
            "eval_flow_transition_produces_event_evidence",
            "transition attempts still emit event evidence",
            "denied flow attempt produces guard-denial event evidence without decision authority",
        )
    } else {
        SmokeEvalCaseResult::fail(
            "eval_flow_transition_produces_event_evidence",
            "transition attempts still emit event evidence",
            format!(
                "event evidence drifted to kind={} status={} guard_code={:?}",
                draft.kind.as_str(),
                draft.result.status.as_str(),
                draft.result.guard_code,
            ),
        )
    }
}

fn eval_event_log_is_append_only() -> SmokeEvalCaseResult {
    let mut log = MemoryEventLog::default();
    let first = log.append(schema_fixture());

    let second_attempt =
        FlowInstance::new(FlowState::AwaitingApproval).attempt_transition(FlowCommand::Approve);
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
            SmokeEvalCaseResult::pass(
                "eval_event_log_is_append_only",
                "append-only event log stays monotonic",
                "append-only log preserved prior records and emitted monotonic ids and sequences",
            )
        }
        (Ok(first), Ok(second)) => SmokeEvalCaseResult::fail(
            "eval_event_log_is_append_only",
            "append-only event log stays monotonic",
            format!(
                "append-only evidence drifted; first={} second={} len={}",
                first.event_id,
                second.event_id,
                log.events().len(),
            ),
        ),
        (Err(error), _) | (_, Err(error)) => SmokeEvalCaseResult::fail(
            "eval_event_log_is_append_only",
            "append-only event log stays monotonic",
            format!("append failed with {error:?}"),
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
    use super::{
        run_smoke_suite, SmokeEvalCaseResult, SmokeEvalReport, SmokeEvalStatus, SmokeEvalSummary,
        SMOKE_REPORT_SCHEMA_VERSION, SMOKE_SUITE_ID,
    };

    #[test]
    fn smoke_report_contains_expected_summary_and_cases() {
        let report = run_smoke_suite();

        assert_eq!(report.suite_id(), SMOKE_SUITE_ID);
        assert_eq!(report.smoke_result(), SmokeEvalStatus::Pass);
        assert_eq!(report.mode(), "local-smoke-check");
        assert_eq!(report.runtime_persistence(), "inactive");
        assert_eq!(report.report_storage(), "inactive");
        assert_eq!(report.cases().len(), 5);
    }

    #[test]
    fn smoke_report_passes_against_current_flow_and_event_kernels() {
        let report = run_smoke_suite();

        assert!(report.passed(), "all current smoke cases should pass");
        assert!(report
            .cases()
            .iter()
            .all(|case| case.status == SmokeEvalStatus::Pass));
    }

    #[test]
    fn smoke_report_renders_human_output_with_boundaries_and_deferred_notes() {
        let report = run_smoke_suite();
        let rendered = report.render_human();

        assert!(rendered.contains("punk eval run smoke"));
        assert!(rendered.contains("mode: local-smoke-check"));
        assert!(rendered.contains("runtime_persistence: inactive"));
        assert!(rendered.contains("report_storage: inactive"));
        assert!(rendered.contains("smoke_result: pass"));
        assert!(rendered.contains("assessment: local deterministic smoke harness passed"));
        assert!(rendered.contains("case_results:"));
        assert!(rendered.contains("  - id: eval_flow_allows_approval_transition"));
        assert!(rendered.contains("    status: pass"));
        assert!(rendered.contains("notes:"));
        assert!(rendered.contains("local assessment only; no authority is written here"));
        assert!(rendered
            .contains("JSON output is opt-in only and does not imply a stable public contract"));
        assert!(rendered.contains("deferred:"));
        assert!(rendered.contains("baseline, waiver, and stored eval reports are not active"));
    }

    #[test]
    fn smoke_report_uses_assessment_not_decision_wording() {
        let report = run_smoke_suite();
        let rendered = report.render_human();

        assert!(report
            .assessment()
            .contains("local deterministic smoke harness"));
        assert!(!rendered.contains("accepted"));
        assert!(!rendered.contains("approved"));
        assert!(!rendered.contains("gate decision"));
        assert!(!rendered.contains("proof complete"));
        assert!(!rendered.contains("final decision"));
    }

    #[test]
    fn smoke_report_renders_json_output_with_v0_1_shape() {
        let report = run_smoke_suite();
        let rendered = report.render_json();

        assert!(rendered.starts_with("{\n"));
        assert!(rendered.contains(&format!(
            "\"schema_version\": \"{SMOKE_REPORT_SCHEMA_VERSION}\""
        )));
        assert!(rendered.contains("\"suite_id\": \"smoke.v0\""));
        assert!(rendered.contains("\"run_id\": null"));
        assert!(rendered.contains("\"smoke_result\": \"pass\""));
        assert!(rendered.contains("\"mode\": \"local-smoke-check\""));
        assert!(rendered.contains("\"runtime_persistence\": \"inactive\""));
        assert!(rendered.contains("\"report_storage\": \"inactive\""));
        assert!(rendered.contains("\"case_results\": ["));
        assert!(rendered.contains("\"case_id\": \"eval_flow_allows_approval_transition\""));
        assert!(rendered.contains("\"status\": \"pass\""));
        assert!(rendered.contains("\"boundary_notes\": ["));
        assert!(rendered.contains("\"deferred\": ["));
    }

    #[test]
    fn smoke_report_json_escapes_quotes_backslashes_and_newlines() {
        let report = SmokeEvalReport {
            summary: SmokeEvalSummary {
                suite_id: SMOKE_SUITE_ID,
                smoke_result: SmokeEvalStatus::Pass,
                assessment: "hello \"punk\"\npath\\to\\file".to_owned(),
                mode: "local-smoke-check",
                runtime_persistence: "inactive",
                report_storage: "inactive",
            },
            cases: vec![SmokeEvalCaseResult::pass(
                "case_quotes",
                "line\nbreak",
                "hello \"punk\"\npath\\to\\file",
            )],
            boundary_notes: vec!["line\nbreak"],
            deferred_notes: vec!["path\\to\\file"],
        };

        let rendered = report.render_json();

        assert!(rendered.contains("hello \\\"punk\\\"\\npath\\\\to\\\\file"));
        assert!(rendered.contains("\"summary\": \"line\\nbreak\""));
        assert!(rendered.contains("\"deferred\": [\"path\\\\to\\\\file\"]"));
    }
}
