use std::env;
use std::fmt::Write as _;
use std::process::ExitCode;

use punk_eval::run_smoke_suite;
use punk_flow::{transition_attempt_event_draft, FlowCommand, FlowInstance, FlowState};

#[derive(Debug, Clone, PartialEq, Eq)]
struct CommandOutput {
    text: String,
    exit_code: u8,
}

impl CommandOutput {
    fn success(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            exit_code: 0,
        }
    }

    fn with_exit(text: impl Into<String>, exit_code: u8) -> Self {
        Self {
            text: text.into(),
            exit_code,
        }
    }
}

fn main() -> ExitCode {
    match run(env::args()) {
        Ok(output) => {
            println!("{}", output.text);
            ExitCode::from(output.exit_code)
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}

fn run<I, S>(args: I) -> Result<CommandOutput, String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let args: Vec<String> = args.into_iter().map(Into::into).collect();

    match args.as_slice() {
        [_bin] => Ok(CommandOutput::success(render_root_help())),
        [_bin, flow, inspect] if flow == "flow" && inspect == "inspect" => {
            Ok(CommandOutput::success(render_flow_inspect()))
        }
        [_bin, eval, run, smoke] if eval == "eval" && run == "run" && smoke == "smoke" => {
            Ok(render_smoke_eval())
        }
        [_bin, flow, ..] if flow == "flow" => Err(flow_usage()),
        [_bin, eval, ..] if eval == "eval" => Err(eval_usage()),
        [_bin, ..] => Err(root_usage()),
        [] => Err(root_usage()),
    }
}

fn render_root_help() -> String {
    format!(
        concat!(
            "punk: early-stage local-first bounded work kernel\n",
            "active inspect surface: `punk flow inspect`\n",
            "active eval surface: `punk eval run smoke`\n",
            "runtime persistence is not active yet; inspect and eval stay limited and honest\n\n",
            "Usage:\n",
            "  punk flow inspect\n",
            "  punk eval run smoke\n"
        )
    )
}

fn root_usage() -> String {
    format!(
        concat!(
            "unknown command\n\n",
            "Usage:\n",
            "  punk flow inspect\n",
            "  punk eval run smoke\n\n",
            "Notes:\n",
            "  - only bounded inspect and smoke-eval surfaces are active\n",
            "  - .punk runtime persistence is not active yet\n"
        )
    )
}

fn flow_usage() -> String {
    format!(
        concat!(
            "unknown flow command\n\n",
            "Usage:\n",
            "  punk flow inspect\n"
        )
    )
}

fn eval_usage() -> String {
    format!(
        concat!(
            "unknown eval command\n\n",
            "Usage:\n",
            "  punk eval run smoke\n"
        )
    )
}

fn render_flow_inspect() -> String {
    let preview_instance = FlowInstance::new(FlowState::AwaitingApproval);
    let denied_attempt = preview_instance.attempt_transition(FlowCommand::StartRun);
    let allowed_attempt = preview_instance.attempt_transition(FlowCommand::Approve);
    let denied_event = transition_attempt_event_draft(
        &denied_attempt,
        "flow_inspect_preview",
        Some("work/goals/goal_add_flow_inspect_command.md"),
    );

    let allowed_next_state = allowed_attempt
        .next_state()
        .map(FlowState::as_str)
        .unwrap_or("<none>");

    format!(
        concat!(
            "punk flow inspect\n",
            "mode: limited-kernel-preview\n",
            "runtime_persistence: inactive\n",
            "truth_surface: current flow/event kernel fixtures only\n",
            "current_runtime_state: unavailable\n",
            "preview_state: {preview_state}\n",
            "preview_allowed_commands: {allowed_commands}\n",
            "preview_allowed_transition: {allowed_command} -> {allowed_next_state}\n",
            "preview_denied_transition: {denied_command}\n",
            "preview_guard_code: {guard_code}\n",
            "preview_event_kind: {event_kind}\n",
            "preview_event_status: {event_status}\n",
            "preview_flow_id: {flow_id}\n",
            "preview_goal_ref: {goal_ref}\n",
            "notes:\n",
            "  - no .punk runtime state is read or written\n",
            "  - inspect is derived from existing flow and event kernels only\n",
            "  - event evidence remains evidence, not decision authority\n"
        ),
        preview_state = preview_instance.state().as_str(),
        allowed_commands = format_commands(preview_instance.allowed_commands()),
        allowed_command = allowed_attempt.attempted_command().as_str(),
        allowed_next_state = allowed_next_state,
        denied_command = denied_attempt.attempted_command().as_str(),
        guard_code = denied_attempt.guard_code().unwrap_or("<none>"),
        event_kind = denied_event.kind.as_str(),
        event_status = denied_event.result.status.as_str(),
        flow_id = denied_event.correlation.flow_id,
        goal_ref = denied_event.correlation.goal_ref.as_deref().unwrap_or("<none>"),
    )
}

fn render_smoke_eval() -> CommandOutput {
    let report = run_smoke_suite();
    let smoke_result = if report.passed() { "pass" } else { "fail" };
    let assessment = if report.passed() {
        "local deterministic smoke harness passed over current flow and event kernels"
    } else {
        "local deterministic smoke harness found one or more failing cases over current flow and event kernels"
    };

    let mut output = String::new();
    writeln!(&mut output, "punk eval run smoke").expect("writing to String should succeed");
    writeln!(&mut output, "mode: local-smoke-check").expect("writing to String should succeed");
    writeln!(&mut output, "runtime_persistence: inactive")
        .expect("writing to String should succeed");
    writeln!(&mut output, "report_storage: inactive").expect("writing to String should succeed");
    writeln!(&mut output, "suite_id: {}", report.suite_id())
        .expect("writing to String should succeed");
    writeln!(&mut output, "smoke_result: {smoke_result}")
        .expect("writing to String should succeed");
    writeln!(&mut output, "assessment: {assessment}")
        .expect("writing to String should succeed");
    writeln!(&mut output, "case_results:").expect("writing to String should succeed");

    for case in report.cases() {
        writeln!(&mut output, "  - {}: {}", case.case_id, case.status.as_str())
            .expect("writing to String should succeed");
    }

    output.push_str(
        concat!(
            "notes:\n",
            "  - local assessment only; not a gate decision\n",
            "  - no .punk/evals runtime state is read or written\n",
            "  - no baseline, waiver, or stored eval reports are active"
        ),
    );

    CommandOutput::with_exit(output, if report.passed() { 0 } else { 1 })
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
    use super::{eval_usage, render_flow_inspect, render_root_help, render_smoke_eval, run};

    #[test]
    fn root_help_points_to_active_surfaces() {
        let output = render_root_help();

        assert!(output.contains("punk flow inspect"));
        assert!(output.contains("punk eval run smoke"));
        assert!(output.contains("runtime persistence is not active yet"));
    }

    #[test]
    fn flow_inspect_is_honest_about_limited_runtime() {
        let output = render_flow_inspect();

        assert!(output.contains("mode: limited-kernel-preview"));
        assert!(output.contains("runtime_persistence: inactive"));
        assert!(output.contains("current_runtime_state: unavailable"));
        assert!(output.contains("no .punk runtime state is read or written"));
        assert!(output.contains("event evidence remains evidence, not decision authority"));
    }

    #[test]
    fn flow_inspect_reports_allowed_and_denied_preview_evidence() {
        let output = render_flow_inspect();

        assert!(output.contains("preview_state: AwaitingApproval"));
        assert!(output.contains("preview_allowed_commands: Approve, Block, Escalate, Cancel"));
        assert!(output.contains("preview_allowed_transition: Approve -> Approved"));
        assert!(output.contains("preview_denied_transition: StartRun"));
        assert!(output.contains("preview_guard_code: CUT_REQUIRES_APPROVED_CONTRACT"));
        assert!(output.contains("preview_event_kind: transition_denied"));
        assert!(output.contains("preview_event_status: denied"));
    }

    #[test]
    fn smoke_eval_command_reports_local_assessment_and_success_exit_code() {
        let output = render_smoke_eval();

        assert_eq!(output.exit_code, 0);
        assert!(output.text.contains("punk eval run smoke"));
        assert!(output.text.contains("mode: local-smoke-check"));
        assert!(output.text.contains("runtime_persistence: inactive"));
        assert!(output.text.contains("report_storage: inactive"));
        assert!(output.text.contains("suite_id: smoke.v0"));
        assert!(output.text.contains("smoke_result: pass"));
        assert!(output
            .text
            .contains("assessment: local deterministic smoke harness passed"));
        assert!(output.text.contains("case_results:"));
        assert!(output.text.contains("eval_flow_allows_approval_transition: pass"));
        assert!(output.text.contains("eval_event_log_is_append_only: pass"));
        assert!(output
            .text
            .contains("no .punk/evals runtime state is read or written"));
        assert!(output
            .text
            .contains("local assessment only; not a gate decision"));
        assert!(!output.text.contains("accepted"));
        assert!(!output.text.contains("approved"));
        assert!(!output.text.contains("proof complete"));
    }

    #[test]
    fn run_returns_success_exit_code_for_smoke_command() {
        let output = run(["punk", "eval", "run", "smoke"]).expect("smoke command must run");

        assert_eq!(output.exit_code, 0);
        assert!(output.text.contains("smoke_result: pass"));
    }

    #[test]
    fn unknown_eval_command_returns_usage_error() {
        let error = run(["punk", "eval", "unknown"]).expect_err("unknown eval command must fail");

        assert_eq!(error, eval_usage());
        assert!(error.contains("punk eval run smoke"));
    }

    #[test]
    fn unknown_command_returns_usage_error() {
        let error = run(["punk", "unknown"]).expect_err("unknown command must fail");

        assert!(error.contains("unknown command"));
        assert!(error.contains("punk flow inspect"));
        assert!(error.contains("punk eval run smoke"));
    }
}
