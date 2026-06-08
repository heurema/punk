use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use punk_eval::run_smoke_suite;
use punk_flow::{transition_attempt_event_draft, FlowCommand, FlowInstance, FlowState};
use punk_project::{
    init_project, locate_publishing_workspace, ProjectId, ProjectInitEntryMode,
    PROJECT_ID_FORMAT_NOTE,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct CommandOutput {
    text: String,
    exit_code: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SmokeEvalFormat {
    Human,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PublishingLocateFormat {
    Human,
    Json,
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
    let project_root =
        env::current_dir().map_err(|error| format!("could not read cwd: {error}"))?;
    run_at(args, &project_root)
}

fn run_at<I, S>(args: I, project_root: &Path) -> Result<CommandOutput, String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let args: Vec<String> = args.into_iter().map(Into::into).collect();

    match args.as_slice() {
        [] => Ok(CommandOutput::success(render_root_help())),
        [_bin] => Ok(CommandOutput::success(render_root_help())),
        [_bin, help] if help == "--help" || help == "-h" => {
            Ok(CommandOutput::success(render_root_help()))
        }
        [_bin, init, project_id] if init == "init" => {
            let project_id = parse_project_id(project_id)?;
            Ok(render_project_init(
                project_root,
                project_id,
                ProjectInitEntryMode::Greenfield,
            ))
        }
        [_bin, init, project_id, mode_flag, mode] if init == "init" && mode_flag == "--mode" => {
            let project_id = parse_project_id(project_id)?;
            let entry_mode = parse_init_entry_mode(mode)?;
            Ok(render_project_init(project_root, project_id, entry_mode))
        }
        [_bin, flow, inspect] if flow == "flow" && inspect == "inspect" => {
            Ok(CommandOutput::success(render_flow_inspect()))
        }
        [_bin, publishing, locate, options @ ..]
            if publishing == "publishing" && locate == "locate" =>
        {
            let options = parse_publishing_locate_options(project_root, options)?;
            Ok(render_publishing_locate(
                &options.project_root,
                options.format,
            ))
        }
        [_bin, eval, run, smoke] if eval == "eval" && run == "run" && smoke == "smoke" => {
            Ok(render_smoke_eval(SmokeEvalFormat::Human))
        }
        [_bin, eval, run, smoke, format_flag, format]
            if eval == "eval" && run == "run" && smoke == "smoke" && format_flag == "--format" =>
        {
            Ok(render_smoke_eval(parse_smoke_eval_format(format)?))
        }
        [_bin, init, ..] if init == "init" => Err(init_usage()),
        [_bin, flow, ..] if flow == "flow" => Err(flow_usage()),
        [_bin, publishing, ..] if publishing == "publishing" => Err(publishing_usage()),
        [_bin, eval, ..] if eval == "eval" => Err(eval_usage()),
        [_bin, ..] => Err(root_usage()),
    }
}

fn render_root_help() -> String {
    format!(concat!(
        "punk: early-stage local-first bounded work kernel\n",
        "active setup surface: `punk init`\n",
        "active inspect surface: `punk flow inspect`\n",
        "active publishing resolver surface: `punk publishing locate`\n",
        "active eval surface: `punk eval run smoke`\n",
        "runtime persistence is limited to a local event-log writer slice; init and inspect do not read or write runtime state\n\n",
        "Usage:\n",
        "  punk init <project-id>\n",
        "  punk init <project-id> --mode <greenfield|brownfield>\n",
        "  punk flow inspect\n",
        "  punk publishing locate [--project-root <path>] [--json]\n",
        "  punk eval run smoke\n",
        "  punk eval run smoke --format json\n"
    ))
}

fn root_usage() -> String {
    format!(concat!(
        "unknown command\n\n",
        "Usage:\n",
        "  punk init <project-id>\n",
        "  punk init <project-id> --mode <greenfield|brownfield>\n",
        "  punk flow inspect\n",
        "  punk publishing locate [--project-root <path>] [--json]\n",
        "  punk eval run smoke\n",
        "  punk eval run smoke --format json\n\n",
        "Notes:\n",
        "  - init writes only a compact Level 0 repo-tracked .punk/memory scaffold\n",
        "  - init operates in the current directory and does not create a <project-id> subdirectory\n",
        "  - only bounded init, inspect, and smoke-eval surfaces are active\n",
        "  - broad .punk runtime persistence is not active; local event-log writer behavior is library-only\n"
    ))
}

fn init_usage() -> String {
    format!(
        concat!(
        "unknown init command\n\n",
        "Usage:\n",
        "  punk init <project-id>\n",
        "  punk init <project-id> --mode <greenfield|brownfield>\n\n",
        "Notes:\n",
        "  - run from the target project root; init writes into the current directory in place\n",
        "  - init does not create a new subdirectory named <project-id>\n",
        "  - default mode is greenfield: creates compact .punk/memory tracked memory plus .punk marker/setup files\n",
        "  - brownfield mode creates only an advisory reconstruction workspace; it does not scan or reconstruct the project\n",
        "  - {}\n",
        "  - existing files are never overwritten\n",
        "  - .punk runtime persistence is not active during init\n"
    ),
        PROJECT_ID_FORMAT_NOTE
    )
}

fn flow_usage() -> String {
    format!(concat!(
        "unknown flow command\n\n",
        "Usage:\n",
        "  punk flow inspect\n"
    ))
}

fn eval_usage() -> String {
    format!(concat!(
        "unknown eval command\n\n",
        "Usage:\n",
        "  punk eval run smoke\n",
        "  punk eval run smoke --format json\n"
    ))
}

fn publishing_usage() -> String {
    format!(concat!(
        "unknown publishing command\n\n",
        "Usage:\n",
        "  punk publishing locate [--project-root <path>] [--json]\n\n",
        "Notes:\n",
        "  - locate reads .punk/publishing.toml and local-only .punk/publishing.local.toml only\n",
        "  - locate does not create files, publish, call APIs, open browsers, or read credentials\n",
        "  - .punk/publishing.local.toml is local-only pointer state, not project truth\n"
    ))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PublishingLocateOptions {
    project_root: PathBuf,
    format: PublishingLocateFormat,
}

fn parse_publishing_locate_options(
    cwd: &Path,
    options: &[String],
) -> Result<PublishingLocateOptions, String> {
    let mut project_root = cwd.to_path_buf();
    let mut format = PublishingLocateFormat::Human;
    let mut index = 0;

    while index < options.len() {
        match options[index].as_str() {
            "--json" => {
                format = PublishingLocateFormat::Json;
                index += 1;
            }
            "--project-root" => {
                let Some(value) = options.get(index + 1) else {
                    return Err(publishing_usage());
                };
                project_root = resolve_cli_path(cwd, value);
                index += 2;
            }
            _ => return Err(publishing_usage()),
        }
    }

    Ok(PublishingLocateOptions {
        project_root,
        format,
    })
}

fn resolve_cli_path(cwd: &Path, value: &str) -> PathBuf {
    let path = PathBuf::from(value);
    if path.is_absolute() {
        path
    } else {
        cwd.join(path)
    }
}

fn render_flow_inspect() -> String {
    let preview_instance = FlowInstance::new(FlowState::AwaitingApproval);
    let denied_attempt = preview_instance.attempt_transition(FlowCommand::StartRun);
    let allowed_attempt = preview_instance.attempt_transition(FlowCommand::Approve);
    let denied_event =
        transition_attempt_event_draft(&denied_attempt, "flow_inspect_preview", None);
    let denied_transition_label = if denied_attempt.next_state().is_some() {
        "preview_applied_transition"
    } else {
        "preview_denied_transition"
    };

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
            "{denied_transition_label}: {denied_command}\n",
            "preview_guard_code: {guard_code}\n",
            "preview_event_kind: {event_kind}\n",
            "preview_event_status: {event_status}\n",
            "preview_flow_id: {flow_id}\n",
            "notes:\n",
            "  - no .punk runtime state is read or written\n",
            "  - inspect is derived from existing flow and event kernels only\n",
            "  - event evidence remains evidence, not decision authority\n"
        ),
        preview_state = preview_instance.state().as_str(),
        allowed_commands = format_commands(preview_instance.allowed_commands()),
        allowed_command = allowed_attempt.attempted_command().as_str(),
        allowed_next_state = allowed_next_state,
        denied_transition_label = denied_transition_label,
        denied_command = denied_attempt.attempted_command().as_str(),
        guard_code = denied_attempt.guard_code().unwrap_or("<none>"),
        event_kind = denied_event.kind.as_str(),
        event_status = denied_event.result.status.as_str(),
        flow_id = denied_event.correlation.flow_id,
    )
}

fn render_smoke_eval(format: SmokeEvalFormat) -> CommandOutput {
    let report = run_smoke_suite();
    let text = match format {
        SmokeEvalFormat::Human => report.render_human(),
        SmokeEvalFormat::Json => report.render_json(),
    };
    CommandOutput::with_exit(text, report.exit_code())
}

fn render_publishing_locate(project_root: &Path, format: PublishingLocateFormat) -> CommandOutput {
    let report = locate_publishing_workspace(project_root);
    let text = match format {
        PublishingLocateFormat::Human => report.render_human(),
        PublishingLocateFormat::Json => report.render_json(),
    };
    CommandOutput::with_exit(text, report.exit_code())
}

fn render_project_init(
    project_root: &Path,
    project_id: ProjectId,
    entry_mode: ProjectInitEntryMode,
) -> CommandOutput {
    let report = init_project(project_root, project_id, entry_mode);
    CommandOutput::with_exit(report.render_human(), report.exit_code())
}

fn parse_project_id(value: &str) -> Result<ProjectId, String> {
    ProjectId::parse(value.to_owned()).map_err(|error| {
        format!(
            "invalid project id: {}\n\n{}",
            error.message(),
            init_usage()
        )
    })
}

fn parse_init_entry_mode(value: &str) -> Result<ProjectInitEntryMode, String> {
    match value {
        "greenfield" => Ok(ProjectInitEntryMode::Greenfield),
        "brownfield" => Ok(ProjectInitEntryMode::Brownfield),
        _ => Err(format!("invalid init mode: {value}\n\n{}", init_usage())),
    }
}

fn parse_smoke_eval_format(value: &str) -> Result<SmokeEvalFormat, String> {
    match value {
        "json" => Ok(SmokeEvalFormat::Json),
        _ => Err(format!(
            "unsupported eval format: {value}\n\n{}",
            eval_usage()
        )),
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
        eval_usage, init_usage, publishing_usage, render_flow_inspect, render_publishing_locate,
        render_root_help, render_smoke_eval, run, run_at, PublishingLocateFormat, SmokeEvalFormat,
    };
    use std::fs;
    use std::process;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn root_help_points_to_active_surfaces() {
        let output = render_root_help();

        assert!(output.contains("punk init"));
        assert!(output.contains("punk flow inspect"));
        assert!(output.contains("punk publishing locate"));
        assert!(output.contains("punk eval run smoke"));
        assert!(output.contains("runtime persistence is limited to a local event-log writer slice"));
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
        assert!(!output.contains("preview_goal_ref:"));
        assert!(!output.contains("work/goals/"));
    }

    #[test]
    fn publishing_locate_resolves_local_workspace_pointer() {
        let root = unique_temp_path();
        let workspace = root.join("external-publishing-workspace");
        fs::create_dir_all(root.join(".punk")).expect("punk dir should be created");
        fs::create_dir_all(&workspace).expect("workspace dir should be created");
        write_publishing_binding(&root, "goalrail", "punk-publishing://project/goalrail");
        write_publishing_local_pointer(
            &root,
            "punk-publishing://project/goalrail",
            &workspace.display().to_string(),
        );

        let output = render_publishing_locate(&root, PublishingLocateFormat::Human);

        assert_eq!(output.exit_code, 0);
        assert!(output.text.contains("punk publishing locate"));
        assert!(output
            .text
            .contains("mode: local-publishing-workspace-resolver"));
        assert!(output.text.contains("status: located"));
        assert!(output.text.contains("project_id: goalrail"));
        assert!(output
            .text
            .contains("workspace_ref: punk-publishing://project/goalrail"));
        assert!(output.text.contains("workspace_exists: true"));
        assert!(output
            .text
            .contains("no files are created, modified, or published"));
        assert!(output
            .text
            .contains("no browser, network API, credential, token, adapter, bot"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn publishing_locate_json_supports_project_root_option() {
        let root = unique_temp_path();
        let cwd = unique_temp_path();
        let workspace = root.join("external-publishing-workspace");
        fs::create_dir_all(root.join(".punk")).expect("punk dir should be created");
        fs::create_dir_all(&cwd).expect("cwd should be created");
        fs::create_dir_all(&workspace).expect("workspace dir should be created");
        write_publishing_binding(&root, "goalrail", "punk-publishing://project/goalrail");
        write_publishing_local_pointer(
            &root,
            "punk-publishing://project/goalrail",
            &workspace.display().to_string(),
        );

        let output = run_at(
            [
                "punk",
                "publishing",
                "locate",
                "--project-root",
                root.to_str().expect("test path should be utf-8"),
                "--json",
            ],
            &cwd,
        )
        .expect("publishing locate should run");

        assert_eq!(output.exit_code, 0);
        assert!(output.text.starts_with("{"));
        assert!(output.text.contains("\"status\": \"located\""));
        assert!(output.text.contains("\"project_id\": \"goalrail\""));
        assert!(output
            .text
            .contains("\"workspace_ref\": \"punk-publishing://project/goalrail\""));
        assert!(output.text.contains("\"workspace_exists\": true"));
        assert!(output.text.contains("\"writes_files\": false"));
        assert!(output.text.contains("\"external_side_effects\": false"));

        let _ = fs::remove_dir_all(root);
        let _ = fs::remove_dir_all(cwd);
    }

    #[test]
    fn publishing_locate_blocks_missing_local_pointer() {
        let root = unique_temp_path();
        fs::create_dir_all(root.join(".punk")).expect("punk dir should be created");
        write_publishing_binding(&root, "goalrail", "punk-publishing://project/goalrail");

        let output = run_at(["punk", "publishing", "locate", "--json"], &root)
            .expect("publishing locate should report blocked state");

        assert_eq!(output.exit_code, 1);
        assert!(output.text.contains("\"status\": \"blocked\""));
        assert!(output
            .text
            .contains("\"blocker_code\": \"local_pointer_missing\""));
        assert!(output.text.contains("\"workspace_root\": null"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn smoke_eval_command_reports_local_assessment_and_success_exit_code() {
        let output = render_smoke_eval(SmokeEvalFormat::Human);

        assert_eq!(output.exit_code, 0);
        assert!(output.text.contains("punk eval run smoke"));
        assert!(output.text.contains("mode: local-smoke-check"));
        assert!(output
            .text
            .contains("runtime_persistence: local-event-log-writer"));
        assert!(output.text.contains("report_storage: inactive"));
        assert!(output.text.contains("suite_id: smoke.v0"));
        assert!(output.text.contains("smoke_result: pass"));
        assert!(output
            .text
            .contains("assessment: local deterministic smoke harness passed"));
        assert!(output.text.contains("case_results:"));
        assert!(output
            .text
            .contains("  - id: eval_flow_allows_approval_transition"));
        assert!(output.text.contains("    status: pass"));
        assert!(output
            .text
            .contains("    summary: append-only event log stays monotonic"));
        assert!(output
            .text
            .contains("no .punk/evals runtime state is read or written"));
        assert!(output
            .text
            .contains("local assessment only; no authority is written here"));
        assert!(output.text.contains("deferred:"));
        assert!(output
            .text
            .contains("baseline, waiver, and stored eval reports are not active"));
        assert!(!output.text.contains("acceptance: accepted"));
        assert!(!output.text.contains("acceptance: approved"));
        assert!(!output.text.contains("proof complete"));
    }

    #[test]
    fn smoke_eval_command_supports_opt_in_json_output() {
        let output = render_smoke_eval(SmokeEvalFormat::Json);

        assert_eq!(output.exit_code, 0);
        assert!(output.text.starts_with("{"));
        assert!(output
            .text
            .contains("\"schema_version\": \"smoke-eval-report.v0.1\""));
        assert!(output.text.contains("\"smoke_result\": \"pass\""));
        assert!(output.text.contains("\"report_storage\": \"inactive\""));
        assert!(!output.text.contains("punk eval run smoke\n"));
        assert!(!output.text.contains("case_results:"));
    }

    #[test]
    fn run_returns_success_exit_code_for_smoke_command() {
        let output = run(["punk", "eval", "run", "smoke"]).expect("smoke command must run");

        assert_eq!(output.exit_code, 0);
        assert!(output.text.contains("smoke_result: pass"));
    }

    #[test]
    fn bare_binary_dispatch_returns_success_help() {
        let output = run(["punk"]).expect("bare binary should render help");

        assert_eq!(output.exit_code, 0);
        assert!(output.text.contains("Usage:"));
        assert!(output.text.contains("punk init <project-id>"));
    }

    #[test]
    fn help_flags_dispatch_to_success_help() {
        for flag in ["--help", "-h"] {
            let output = run(["punk", flag]).expect("help flag should render help");

            assert_eq!(output.exit_code, 0);
            assert!(output.text.contains("Usage:"));
            assert!(output.text.contains("punk eval run smoke"));
        }
    }

    #[test]
    fn empty_test_args_dispatch_to_success_help() {
        let output = run_at(std::iter::empty::<&str>(), std::path::Path::new("."))
            .expect("empty test args should render help defensively");

        assert_eq!(output.exit_code, 0);
        assert!(output.text.contains("Usage:"));
    }

    #[test]
    fn run_returns_json_for_smoke_command_when_requested() {
        let output = run(["punk", "eval", "run", "smoke", "--format", "json"])
            .expect("json smoke command must run");

        assert_eq!(output.exit_code, 0);
        assert!(output.text.starts_with("{"));
        assert!(output.text.contains("\"case_results\": ["));
    }

    #[test]
    fn init_command_creates_level0_project_memory_scaffold() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");

        let output =
            run_at(["punk", "init", "weekend-project"], &root).expect("init command should run");

        assert_eq!(output.exit_code, 0);
        assert!(output.text.contains("punk init"));
        assert!(output.text.contains("mode: manual-project-memory-level0"));
        assert!(output.text.contains("entry_mode: greenfield"));
        assert!(output.text.contains("project_id: weekend-project"));
        assert!(output.text.contains("runtime_persistence: inactive"));
        assert!(output.text.contains("target_root: ."));
        assert!(!output.text.contains(&root.display().to_string()));
        assert!(output.text.contains("result: initialized"));
        assert!(output.text.contains("path: .punk/memory/STATUS.md"));
        assert!(output
            .text
            .contains("path: .punk/memory/goals/goal_initial_project_setup.md"));
        assert!(output.text.contains("path: .punk/instructions/INDEX.md"));
        assert!(output
            .text
            .contains("does not implement brownfield reconstruction or grayfield reconciliation"));
        assert!(output
            .text
            .contains("creates thin local instruction entrypoints under .punk/instructions"));
        assert!(output.text.contains(
            "does not create root-level work, knowledge, docs/adr, or publishing directories"
        ));
        assert!(output.text.contains(
            "creates .punk as a project root marker and .punk/memory as tracked durable memory"
        ));
        assert!(root.join(".punk/memory/STATUS.md").is_file());
        assert!(root
            .join(".punk/memory/goals/goal_initial_project_setup.md")
            .is_file());
        assert!(root.join(".punk/instructions/INDEX.md").is_file());
        assert!(root.join(".punk/instructions/pages/init.md").is_file());
        assert!(root.join(".punk/instructions/modules/README.md").is_file());
        assert!(root.join(".punk/project.toml").is_file());
        assert!(!root.join("work").exists());
        assert!(!root.join("knowledge").exists());
        assert!(!root.join("docs").exists());
        assert!(!root.join("docs/adr").exists());
        assert!(!root.join("publishing").exists());
        assert!(!root.join(".punk/runtime").exists());
        assert!(!root.join(".punk/cache").exists());
        assert!(!root.join(".punk/events").exists());
        assert!(!root.join(".punk/contracts").exists());
        assert!(!root.join(".punk/runs").exists());
        assert!(!root.join(".punk/decisions").exists());
        assert!(!root.join(".punk/proofs").exists());
        assert!(!root.join(".punk/indexes").exists());
        assert!(!root.join(".punk/views").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn init_command_accepts_explicit_greenfield_mode() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");

        let output = run_at(
            ["punk", "init", "weekend-project", "--mode", "greenfield"],
            &root,
        )
        .expect("greenfield init command should run");

        assert_eq!(output.exit_code, 0);
        assert!(output.text.contains("entry_mode: greenfield"));
        assert!(root.join(".punk/memory/STATUS.md").is_file());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn init_command_creates_brownfield_reconstruction_scaffold_when_requested() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");

        let output = run_at(
            ["punk", "init", "weekend-project", "--mode", "brownfield"],
            &root,
        )
        .expect("brownfield init command should run");

        assert_eq!(output.exit_code, 0);
        assert!(output.text.contains("punk init"));
        assert!(output
            .text
            .contains("schema_version: project-init-brownfield-scaffold.v0.1"));
        assert!(output.text.contains("entry_mode: brownfield"));
        assert!(output.text.contains("project_id: weekend-project"));
        assert!(output.text.contains("runtime_persistence: inactive"));
        assert!(output.text.contains("target_root: ."));
        assert!(!output.text.contains(&root.display().to_string()));
        assert!(output.text.contains("result: initialized"));
        assert!(output.text.contains("path: .punk/memory/STATUS.md"));
        assert!(output
            .text
            .contains("path: .punk/memory/goals/goal_brownfield_reconstruction_baseline.md"));
        assert!(output
            .text
            .contains("path: .punk/memory/reconstruction/claim-ledger.md"));
        assert!(output.text.contains("path: .punk/instructions/INDEX.md"));
        assert!(output
            .text
            .contains("does not scan the repository or infer project knowledge"));
        assert!(output
            .text
            .contains("creates thin local instruction entrypoints under .punk/instructions"));
        assert!(output
            .text
            .contains("brownfield reconstruction remains not_started"));
        assert!(root.join(".punk/memory/STATUS.md").is_file());
        assert!(root
            .join(".punk/memory/goals/goal_brownfield_reconstruction_baseline.md")
            .is_file());
        assert!(root
            .join(".punk/memory/reconstruction/contract-readiness.md")
            .is_file());
        assert!(root.join(".punk/instructions/INDEX.md").is_file());
        assert!(root.join(".punk/instructions/pages/modules.md").is_file());
        assert!(root.join(".punk/instructions/modules/README.md").is_file());
        assert!(root.join(".punk/project.toml").is_file());
        assert!(!root.join("work").exists());
        assert!(!root.join("knowledge").exists());
        assert!(!root.join("docs").exists());
        assert!(!root.join("docs/adr").exists());
        assert!(!root.join("publishing").exists());
        assert!(!root.join(".punk/runtime").exists());
        assert!(!root.join(".punk/cache").exists());
        assert!(!root.join(".punk/events").exists());
        assert!(!root.join(".punk/contracts").exists());
        assert!(!root.join(".punk/runs").exists());
        assert!(!root.join(".punk/decisions").exists());
        assert!(!root.join(".punk/proofs").exists());
        assert!(!root.join(".punk/indexes").exists());
        assert!(!root.join(".punk/views").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn init_command_rejects_unknown_mode() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");

        let error = run_at(
            ["punk", "init", "weekend-project", "--mode", "grayfield"],
            &root,
        )
        .expect_err("unsupported init mode must fail");

        assert!(error.contains("invalid init mode: grayfield"));
        assert!(error.contains("punk init <project-id> --mode <greenfield|brownfield>"));
        assert!(!root.join(".punk/memory/STATUS.md").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn init_command_reports_conflict_without_overwrite() {
        let root = unique_temp_path();
        fs::create_dir_all(root.join(".punk/memory")).expect("memory dir should be created");
        fs::write(root.join(".punk/memory/STATUS.md"), "custom status\n")
            .expect("custom status should be written");

        let output = run_at(["punk", "init", "weekend-project"], &root)
            .expect("init command should report conflict");

        assert_eq!(output.exit_code, 1);
        assert!(output.text.contains("result: blocked"));
        assert!(output.text.contains("status: conflict"));
        assert!(output.text.contains("status: planned"));
        assert!(output.text.contains("not overwritten"));
        assert_eq!(
            fs::read_to_string(root.join(".punk/memory/STATUS.md"))
                .expect("status should be readable"),
            "custom status\n"
        );
        assert!(!root
            .join(".punk/memory/goals/goal_initial_project_setup.md")
            .exists());
        assert!(!root.join(".punk/memory/reports/README.md").exists());
        assert!(!root.join(".punk/README.md").exists());
        assert!(!root.join(".punk/project.toml").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn init_command_requires_project_id() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");

        let error = run_at(["punk", "init"], &root).expect_err("missing project id must fail");

        assert_eq!(error, init_usage());
        assert!(error.contains("punk init <project-id>"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn init_command_rejects_invalid_project_id() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");

        let error = run_at(["punk", "init", "Weekend Project"], &root)
            .expect_err("invalid project id must fail");

        assert!(error.contains("invalid project id"));
        assert!(error.contains("lowercase ASCII slug"));
        assert!(!root.join(".punk/memory/STATUS.md").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn unknown_eval_command_returns_usage_error() {
        let error = run(["punk", "eval", "unknown"]).expect_err("unknown eval command must fail");

        assert_eq!(error, eval_usage());
        assert!(error.contains("punk eval run smoke"));
    }

    #[test]
    fn unsupported_eval_format_returns_usage_error() {
        let error = run(["punk", "eval", "run", "smoke", "--format", "yaml"])
            .expect_err("unsupported eval format must fail");

        assert!(error.contains("unsupported eval format: yaml"));
        assert!(error.contains("--format json"));
    }

    #[test]
    fn malformed_flow_command_returns_flow_usage_error() {
        let error = run(["punk", "flow", "inspect", "extra"])
            .expect_err("malformed flow command must fail");

        assert_eq!(error, super::flow_usage());
        assert!(error.contains("punk flow inspect"));
    }

    #[test]
    fn unknown_publishing_command_returns_usage_error() {
        let error =
            run(["punk", "publishing", "unknown"]).expect_err("unknown publishing command fails");

        assert_eq!(error, publishing_usage());
        assert!(error.contains("punk publishing locate"));
    }

    #[test]
    fn unknown_command_returns_usage_error() {
        let error = run(["punk", "unknown"]).expect_err("unknown command must fail");

        assert!(error.contains("unknown command"));
        assert!(error.contains("punk init"));
        assert!(error.contains("punk flow inspect"));
        assert!(error.contains("punk publishing locate"));
        assert!(error.contains("punk eval run smoke"));
    }

    fn write_publishing_binding(root: &std::path::Path, project_id: &str, workspace_ref: &str) {
        fs::write(
            root.join(".punk/publishing.toml"),
            format!(
                r#"schema_version = "punk.publishing.binding.v1"
project_id = "{project_id}"
workspace_ref = "{workspace_ref}"

[workspace]
workspace_ref_kind = "logical"
"#
            ),
        )
        .expect("publishing binding should be written");
    }

    fn write_publishing_local_pointer(
        root: &std::path::Path,
        workspace_ref: &str,
        workspace_root: &str,
    ) {
        fs::write(
            root.join(".punk/publishing.local.toml"),
            format!(
                r#"schema_version = "punk.publishing.local.v1"
workspace_ref = "{workspace_ref}"
workspace_root = "{workspace_root}"
"#
            ),
        )
        .expect("local publishing pointer should be written");
    }

    fn unique_temp_path() -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();
        let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "punk-cli-init-test-{}-{}-{}",
            process::id(),
            unique,
            counter
        ))
    }
}
