use std::env;
use std::process::ExitCode;

use punk_flow::{transition_attempt_event_draft, FlowCommand, FlowInstance, FlowState};

fn main() -> ExitCode {
    match run(env::args()) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}

fn run<I, S>(args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let args: Vec<String> = args.into_iter().map(Into::into).collect();

    match args.as_slice() {
        [_bin] => Ok(render_root_help()),
        [_bin, flow, inspect] if flow == "flow" && inspect == "inspect" => {
            Ok(render_flow_inspect())
        }
        [_bin, flow] if flow == "flow" => Err(flow_usage()),
        [_bin, ..] => Err(root_usage()),
        [] => Err(root_usage()),
    }
}

fn render_root_help() -> String {
    format!(
        concat!(
            "punk: early-stage local-first bounded work kernel
",
            "active inspect surface: `punk flow inspect`
",
            "runtime persistence is not active yet; inspect stays limited and honest

",
            "Usage:
",
            "  punk flow inspect
"
        )
    )
}

fn root_usage() -> String {
    format!(
        concat!(
            "unknown command

",
            "Usage:
",
            "  punk flow inspect

",
            "Notes:
",
            "  - only the first bounded inspect surface is active
",
            "  - .punk runtime persistence is not active yet
"
        )
    )
}

fn flow_usage() -> String {
    format!(
        concat!(
            "unknown flow command

",
            "Usage:
",
            "  punk flow inspect
"
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
            "punk flow inspect
",
            "mode: limited-kernel-preview
",
            "runtime_persistence: inactive
",
            "truth_surface: current flow/event kernel fixtures only
",
            "current_runtime_state: unavailable
",
            "preview_state: {preview_state}
",
            "preview_allowed_commands: {allowed_commands}
",
            "preview_allowed_transition: {allowed_command} -> {allowed_next_state}
",
            "preview_denied_transition: {denied_command}
",
            "preview_guard_code: {guard_code}
",
            "preview_event_kind: {event_kind}
",
            "preview_event_status: {event_status}
",
            "preview_flow_id: {flow_id}
",
            "preview_goal_ref: {goal_ref}
",
            "notes:
",
            "  - no .punk runtime state is read or written
",
            "  - inspect is derived from existing flow and event kernels only
",
            "  - event evidence remains evidence, not decision authority
"
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

fn format_commands(commands: &[FlowCommand]) -> String {
    commands
        .iter()
        .map(|command| command.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::{render_flow_inspect, render_root_help, run};

    #[test]
    fn root_help_points_to_flow_inspect() {
        let output = render_root_help();

        assert!(output.contains("punk flow inspect"));
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
    fn unknown_command_returns_usage_error() {
        let error = run(["punk", "unknown"]).expect_err("unknown command must fail");

        assert!(error.contains("unknown command"));
        assert!(error.contains("punk flow inspect"));
    }
}
