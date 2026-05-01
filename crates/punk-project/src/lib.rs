//! Project identity and Level 0 manual project-memory initialization.

use std::fmt::Write as _;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const PROJECT_INIT_SCHEMA_VERSION: &str = "project-init-level0.v0.1";
pub const PROJECT_INIT_MODE: &str = "manual-project-memory-level0";
pub const PROJECT_INIT_RUNTIME_PERSISTENCE: &str = "inactive";

const WORK_STATUS_TEMPLATE: &str = r#"---
id: work_status
kind: manual-work-ledger
status: active
authority: canonical
owner: TODO
ledger_version: work-ledger.v0.1
dogfooding_level: 0
updated_at: TODO
current_phase: "Dogfooding Level 0 / manual project memory"
current_focus: "Capture initial project truth"
selected_next: "work/goals/goal_capture_initial_project_truth.md"
last_validated_commit: null
---

# Work Status

## Now

- Current stage: project initialized with Punk Level 0 manual memory.
- Current focus: capture initial project truth from existing docs and design.
- Selected next: `work/goals/goal_capture_initial_project_truth.md`

## Blockers

- Fill owner/date TODOs before treating this ledger as current.

## Recent Evidence

- `punk init` created the Level 0 scaffold.

## Open Drift Findings

None.
"#;

const INITIAL_GOAL_TEMPLATE: &str = r#"---
id: goal_capture_initial_project_truth
title: "Capture initial project truth"
status: ready
owner: TODO
module: "project"
priority: P1
authority: canonical
created_at: TODO
updated_at: TODO
selected_at: TODO
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "docs/**"
    - "design/**"
    - ".punk/README.md"
    - ".punk/project.toml"
    - "work/STATUS.md"
  exclude:
    - ".punk/events/**"
    - ".punk/contracts/**"
    - ".punk/runs/**"
    - ".punk/evals/**"
    - ".punk/decisions/**"
    - ".punk/proofs/**"
    - ".punk/indexes/**"
    - ".punk/views/**"
acceptance:
  - "The canonical product/design/source documents are listed with repo-relative refs."
  - "Known constraints, non-goals, and acceptance criteria for the first implementation slice are recorded."
  - "The next bounded implementation goal is created and selected in work/STATUS.md."
knowledge_refs: []
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
research_gate:
  classification: R0
  required: false
  rationale: "Initial truth capture only inventories user-provided/repo-tracked project inputs and does not change architecture."
  research_refs: []
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: initial-project-memory
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Initial project truth capture establishes the manual Level 0 memory baseline."
---

## Context

The project has been initialized with Punk Level 0 manual project memory.

## Intent

Capture the current product, design, technical constraints, non-goals, and first implementation boundary before coding starts.

## Non-scope

Do not implement product behavior in this goal.

Do not write `.punk/` runtime stores.

Do not claim gate acceptance or proofpack coverage.
"#;

const REPORTS_README_TEMPLATE: &str = r#"# Work Reports

Manual Level 0 outcome, evidence, and handoff reports live here.

Use repo-relative artifact refs. Do not treat reports as final gate decisions.
"#;

const ADR_README_TEMPLATE: &str = r#"# Architecture Decisions

Accepted architecture decisions live here.

Use ADRs for durable decisions that should outlive a single work report.
"#;

const RESEARCH_README_TEMPLATE: &str = r#"# Research Notes

Curated research notes live here.

Keep external source notes advisory until promoted through a goal, decision, contract, eval, or report.
"#;

const IDEAS_README_TEMPLATE: &str = r#"# Ideas

Raw or parked ideas live here.

Ideas are not implementation truth until promoted through the project workflow.
"#;

const PUNK_README_TEMPLATE: &str = r#"# .punk

This directory is the Punk project root marker.

Current active behavior is Dogfooding Level 0 manual project memory.

Authoritative live work state remains `work/STATUS.md`.

Runtime stores such as events, contracts, runs, evals, decisions, proofs, indexes, and views are not active yet.
"#;

const PUNK_PROJECT_TOML_TEMPLATE: &str = r#"# Punk project marker.
# This file is setup metadata, not runtime authority.

schema_version = "punk.project.v0.1"
project_id = "TODO"
project_name = "TODO"
dogfooding_level = 0
runtime_persistence = "inactive"
live_work_state = "work/STATUS.md"

[authority]
live_state = "work/STATUS.md"
final_decisions = "not_active"
proofpacks = "not_active"
"#;

const PROJECT_INIT_BOUNDARY_NOTES: &[&str] = &[
    "writes only Level 0 project-memory scaffold files and .punk marker files",
    "creates .punk as a project root marker without runtime stores",
    "does not create contracts, run receipts, gate artifacts, proofpacks, or acceptance claims",
    "uses create-new/no-overwrite behavior and reports conflicts fail-closed",
];

const PROJECT_INIT_DEFERRED_NOTES: &[&str] = &[
    "runtime project storage remains inactive",
    "flow persistence and event writing remain inactive",
    "contract writer, receipt writer, gate writer, proof writer, and proofpack writer remain inactive",
    "project-specific source/design refs must be filled manually after init",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectInitArtifactKind {
    Directory,
    File,
}

impl ProjectInitArtifactKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Directory => "directory",
            Self::File => "file",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectInitArtifactStatus {
    Created,
    AlreadyExists,
    Conflict,
    Failed,
}

impl ProjectInitArtifactStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::AlreadyExists => "already_exists",
            Self::Conflict => "conflict",
            Self::Failed => "failed",
        }
    }

    pub fn is_blocking(self) -> bool {
        matches!(self, Self::Conflict | Self::Failed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectInitArtifactReport {
    repo_relative_path: String,
    kind: ProjectInitArtifactKind,
    status: ProjectInitArtifactStatus,
    detail: Option<String>,
}

impl ProjectInitArtifactReport {
    fn new(
        repo_relative_path: impl Into<String>,
        kind: ProjectInitArtifactKind,
        status: ProjectInitArtifactStatus,
        detail: Option<String>,
    ) -> Self {
        Self {
            repo_relative_path: repo_relative_path.into(),
            kind,
            status,
            detail,
        }
    }

    pub fn repo_relative_path(&self) -> &str {
        &self.repo_relative_path
    }

    pub fn kind(&self) -> ProjectInitArtifactKind {
        self.kind
    }

    pub fn status(&self) -> ProjectInitArtifactStatus {
        self.status
    }

    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    pub fn is_blocking(&self) -> bool {
        self.status.is_blocking()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectInitReport {
    project_root: PathBuf,
    artifacts: Vec<ProjectInitArtifactReport>,
}

impl ProjectInitReport {
    pub fn project_root(&self) -> &Path {
        &self.project_root
    }

    pub fn artifacts(&self) -> &[ProjectInitArtifactReport] {
        &self.artifacts
    }

    pub fn boundary_notes(&self) -> &'static [&'static str] {
        PROJECT_INIT_BOUNDARY_NOTES
    }

    pub fn deferred_notes(&self) -> &'static [&'static str] {
        PROJECT_INIT_DEFERRED_NOTES
    }

    pub fn blocked(&self) -> bool {
        self.artifacts
            .iter()
            .any(ProjectInitArtifactReport::is_blocking)
    }

    pub fn exit_code(&self) -> u8 {
        if self.blocked() {
            1
        } else {
            0
        }
    }

    pub fn result_label(&self) -> &'static str {
        if self.blocked() {
            "blocked"
        } else if self
            .artifacts
            .iter()
            .all(|artifact| artifact.status() == ProjectInitArtifactStatus::AlreadyExists)
        {
            "already_initialized"
        } else {
            "initialized"
        }
    }

    pub fn render_human(&self) -> String {
        let mut output = String::new();
        writeln!(&mut output, "punk init").expect("writing to String should succeed");
        writeln!(&mut output, "schema_version: {PROJECT_INIT_SCHEMA_VERSION}")
            .expect("writing to String should succeed");
        writeln!(&mut output, "mode: {PROJECT_INIT_MODE}")
            .expect("writing to String should succeed");
        writeln!(
            &mut output,
            "runtime_persistence: {PROJECT_INIT_RUNTIME_PERSISTENCE}"
        )
        .expect("writing to String should succeed");
        writeln!(
            &mut output,
            "target_root: {}",
            self.project_root().display()
        )
        .expect("writing to String should succeed");
        writeln!(&mut output, "result: {}", self.result_label())
            .expect("writing to String should succeed");
        writeln!(&mut output, "artifacts:").expect("writing to String should succeed");

        for artifact in self.artifacts() {
            writeln!(&mut output, "  - path: {}", artifact.repo_relative_path())
                .expect("writing to String should succeed");
            writeln!(&mut output, "    kind: {}", artifact.kind().as_str())
                .expect("writing to String should succeed");
            writeln!(&mut output, "    status: {}", artifact.status().as_str())
                .expect("writing to String should succeed");
            if let Some(detail) = artifact.detail() {
                writeln!(&mut output, "    detail: {detail}")
                    .expect("writing to String should succeed");
            }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProjectInitEntry {
    Directory(&'static str),
    File(&'static str, &'static str),
}

const PROJECT_INIT_ENTRIES: &[ProjectInitEntry] = &[
    ProjectInitEntry::Directory("work"),
    ProjectInitEntry::Directory("work/goals"),
    ProjectInitEntry::Directory("work/reports"),
    ProjectInitEntry::Directory("docs"),
    ProjectInitEntry::Directory("docs/adr"),
    ProjectInitEntry::Directory("knowledge"),
    ProjectInitEntry::Directory("knowledge/research"),
    ProjectInitEntry::Directory("knowledge/ideas"),
    ProjectInitEntry::Directory(".punk"),
    ProjectInitEntry::File("work/STATUS.md", WORK_STATUS_TEMPLATE),
    ProjectInitEntry::File(
        "work/goals/goal_capture_initial_project_truth.md",
        INITIAL_GOAL_TEMPLATE,
    ),
    ProjectInitEntry::File("work/reports/README.md", REPORTS_README_TEMPLATE),
    ProjectInitEntry::File("docs/adr/README.md", ADR_README_TEMPLATE),
    ProjectInitEntry::File("knowledge/research/README.md", RESEARCH_README_TEMPLATE),
    ProjectInitEntry::File("knowledge/ideas/README.md", IDEAS_README_TEMPLATE),
    ProjectInitEntry::File(".punk/README.md", PUNK_README_TEMPLATE),
    ProjectInitEntry::File(".punk/project.toml", PUNK_PROJECT_TOML_TEMPLATE),
];

pub fn init_level0_project(project_root: impl AsRef<Path>) -> ProjectInitReport {
    let project_root = project_root.as_ref().to_path_buf();
    let mut artifacts = Vec::new();

    match fs::metadata(&project_root) {
        Ok(metadata) if metadata.is_dir() => {}
        Ok(_) => {
            artifacts.push(ProjectInitArtifactReport::new(
                ".",
                ProjectInitArtifactKind::Directory,
                ProjectInitArtifactStatus::Failed,
                Some("project root exists but is not a directory".to_owned()),
            ));
            return ProjectInitReport {
                project_root,
                artifacts,
            };
        }
        Err(error) => {
            artifacts.push(ProjectInitArtifactReport::new(
                ".",
                ProjectInitArtifactKind::Directory,
                ProjectInitArtifactStatus::Failed,
                Some(format!("project root must already exist: {error}")),
            ));
            return ProjectInitReport {
                project_root,
                artifacts,
            };
        }
    }

    for entry in PROJECT_INIT_ENTRIES {
        let report = match *entry {
            ProjectInitEntry::Directory(path) => create_init_directory(&project_root, path),
            ProjectInitEntry::File(path, contents) => {
                create_init_file(&project_root, path, contents.as_bytes())
            }
        };
        artifacts.push(report);
    }

    ProjectInitReport {
        project_root,
        artifacts,
    }
}

fn create_init_directory(
    project_root: &Path,
    repo_relative_path: &'static str,
) -> ProjectInitArtifactReport {
    let path = project_root.join(repo_relative_path);
    match fs::symlink_metadata(&path) {
        Ok(metadata) if metadata.file_type().is_dir() && !metadata.file_type().is_symlink() => {
            ProjectInitArtifactReport::new(
                repo_relative_path,
                ProjectInitArtifactKind::Directory,
                ProjectInitArtifactStatus::AlreadyExists,
                None,
            )
        }
        Ok(_) => ProjectInitArtifactReport::new(
            repo_relative_path,
            ProjectInitArtifactKind::Directory,
            ProjectInitArtifactStatus::Conflict,
            Some("path already exists and is not a plain directory".to_owned()),
        ),
        Err(error) if error.kind() == io::ErrorKind::NotFound => match fs::create_dir(&path) {
            Ok(()) => ProjectInitArtifactReport::new(
                repo_relative_path,
                ProjectInitArtifactKind::Directory,
                ProjectInitArtifactStatus::Created,
                None,
            ),
            Err(create_error) => ProjectInitArtifactReport::new(
                repo_relative_path,
                ProjectInitArtifactKind::Directory,
                ProjectInitArtifactStatus::Failed,
                Some(format!("could not create directory: {create_error}")),
            ),
        },
        Err(error) => ProjectInitArtifactReport::new(
            repo_relative_path,
            ProjectInitArtifactKind::Directory,
            ProjectInitArtifactStatus::Failed,
            Some(format!("could not inspect path: {error}")),
        ),
    }
}

fn create_init_file(
    project_root: &Path,
    repo_relative_path: &'static str,
    contents: &[u8],
) -> ProjectInitArtifactReport {
    let path = project_root.join(repo_relative_path);
    match fs::symlink_metadata(&path) {
        Ok(metadata) if metadata.file_type().is_file() && !metadata.file_type().is_symlink() => {
            match fs::read(&path) {
                Ok(existing) if existing == contents => ProjectInitArtifactReport::new(
                    repo_relative_path,
                    ProjectInitArtifactKind::File,
                    ProjectInitArtifactStatus::AlreadyExists,
                    None,
                ),
                Ok(_) => ProjectInitArtifactReport::new(
                    repo_relative_path,
                    ProjectInitArtifactKind::File,
                    ProjectInitArtifactStatus::Conflict,
                    Some("file already exists with different contents; not overwritten".to_owned()),
                ),
                Err(error) => ProjectInitArtifactReport::new(
                    repo_relative_path,
                    ProjectInitArtifactKind::File,
                    ProjectInitArtifactStatus::Failed,
                    Some(format!("could not read existing file: {error}")),
                ),
            }
        }
        Ok(_) => ProjectInitArtifactReport::new(
            repo_relative_path,
            ProjectInitArtifactKind::File,
            ProjectInitArtifactStatus::Conflict,
            Some("path already exists and is not a plain file".to_owned()),
        ),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            let write_result = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
                .and_then(|mut file| file.write_all(contents));

            match write_result {
                Ok(()) => ProjectInitArtifactReport::new(
                    repo_relative_path,
                    ProjectInitArtifactKind::File,
                    ProjectInitArtifactStatus::Created,
                    None,
                ),
                Err(write_error) => ProjectInitArtifactReport::new(
                    repo_relative_path,
                    ProjectInitArtifactKind::File,
                    ProjectInitArtifactStatus::Failed,
                    Some(format!("could not create file: {write_error}")),
                ),
            }
        }
        Err(error) => ProjectInitArtifactReport::new(
            repo_relative_path,
            ProjectInitArtifactKind::File,
            ProjectInitArtifactStatus::Failed,
            Some(format!("could not inspect path: {error}")),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::{init_level0_project, ProjectInitArtifactKind, ProjectInitArtifactStatus};
    use std::fs;
    use std::process;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn init_creates_level0_manual_memory_scaffold() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");

        let report = init_level0_project(&root);

        assert!(!report.blocked());
        assert_eq!(report.exit_code(), 0);
        assert_eq!(report.result_label(), "initialized");
        assert!(root.join("work/STATUS.md").is_file());
        assert!(root
            .join("work/goals/goal_capture_initial_project_truth.md")
            .is_file());
        assert!(root.join("work/reports/README.md").is_file());
        assert!(root.join("docs/adr/README.md").is_file());
        assert!(root.join("knowledge/research/README.md").is_file());
        assert!(root.join("knowledge/ideas/README.md").is_file());
        assert!(root.join(".punk/README.md").is_file());
        assert!(root.join(".punk/project.toml").is_file());
        assert!(!root.join(".punk/events").exists());
        assert!(!root.join(".punk/contracts").exists());
        assert!(!root.join(".punk/runs").exists());
        assert!(!root.join(".punk/evals").exists());
        assert!(!root.join(".punk/decisions").exists());
        assert!(!root.join(".punk/proofs").exists());

        let status = fs::read_to_string(root.join("work/STATUS.md"))
            .expect("status template should be readable");
        assert!(status.contains("dogfooding_level: 0"));
        assert!(
            status.contains("selected_next: \"work/goals/goal_capture_initial_project_truth.md\"")
        );
        let project_marker = fs::read_to_string(root.join(".punk/project.toml"))
            .expect("project marker should be readable");
        assert!(project_marker.contains("schema_version = \"punk.project.v0.1\""));
        assert!(project_marker.contains("runtime_persistence = \"inactive\""));
        assert!(project_marker.contains("live_work_state = \"work/STATUS.md\""));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn init_is_idempotent_when_existing_files_match() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");

        let first = init_level0_project(&root);
        let second = init_level0_project(&root);

        assert!(!first.blocked());
        assert!(!second.blocked());
        assert_eq!(second.result_label(), "already_initialized");
        assert!(second
            .artifacts()
            .iter()
            .all(|artifact| { artifact.status() == ProjectInitArtifactStatus::AlreadyExists }));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn init_reports_conflict_without_overwriting_existing_status() {
        let root = unique_temp_path();
        fs::create_dir_all(root.join("work")).expect("work dir should be created");
        fs::write(root.join("work/STATUS.md"), "custom status\n")
            .expect("custom status should be written");

        let report = init_level0_project(&root);

        assert!(report.blocked());
        assert_eq!(report.exit_code(), 1);
        assert_eq!(report.result_label(), "blocked");
        assert_eq!(
            fs::read_to_string(root.join("work/STATUS.md")).expect("status should remain readable"),
            "custom status\n"
        );
        assert!(report.artifacts().iter().any(|artifact| {
            artifact.repo_relative_path() == "work/STATUS.md"
                && artifact.kind() == ProjectInitArtifactKind::File
                && artifact.status() == ProjectInitArtifactStatus::Conflict
        }));

        let _ = fs::remove_dir_all(root);
    }

    fn unique_temp_path() -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();
        let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "punk-project-init-test-{}-{}-{}",
            process::id(),
            unique,
            counter
        ))
    }
}
