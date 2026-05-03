//! Project identity and Level 0 manual project-memory initialization.

use std::fmt::Write as _;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const PROJECT_INIT_SCHEMA_VERSION: &str = "project-init-greenfield.v0.1";
pub const PROJECT_INIT_BROWNFIELD_SCHEMA_VERSION: &str = "project-init-brownfield-scaffold.v0.1";
pub const PROJECT_INIT_MODE: &str = "manual-project-memory-level0";
pub const PROJECT_INIT_ENTRY_MODE: &str = "greenfield";
pub const PROJECT_INIT_BROWNFIELD_ENTRY_MODE: &str = "brownfield";
pub const PROJECT_INIT_RUNTIME_PERSISTENCE: &str = "inactive";
pub const PROJECT_ID_FORMAT_NOTE: &str =
    "project id must be a lowercase ASCII slug: a-z, 0-9, and hyphen, starting and ending with a letter or digit";

const MEMORY_ROOT: &str = ".punk/memory";
const STATUS_PATH: &str = ".punk/memory/STATUS.md";
const INITIAL_GOAL_PATH: &str = ".punk/memory/goals/goal_initial_project_setup.md";
const BROWNFIELD_BASELINE_GOAL_PATH: &str =
    ".punk/memory/goals/goal_brownfield_reconstruction_baseline.md";

fn work_status_template(project_id: &ProjectId) -> String {
    format!(
        r#"---
id: work_status
kind: manual-work-ledger
status: active
authority: canonical
owner: TODO
ledger_version: work-ledger.v0.1
dogfooding_level: 0
project_id: "{project_id}"
entry_mode: greenfield
updated_at: TODO
current_phase: "Dogfooding Level 0 / compact project memory"
current_focus: "Initial project setup"
selected_next: "{INITIAL_GOAL_PATH}"
last_validated_commit: null
---

# Work Status

## Now

- Current stage: greenfield project initialized with Punk Level 0 compact manual memory.
- Current focus: complete initial project setup.
- Selected next: `{INITIAL_GOAL_PATH}`

## Blockers

- Fill owner/date TODOs before treating this ledger as current.

## Recent Evidence

- `punk init {project_id}` created the greenfield Level 0 scaffold.

## Open Drift Findings

None.
"#,
        project_id = project_id.as_str()
    )
}

fn initial_goal_template(project_id: &ProjectId) -> String {
    format!(
        r#"---
id: goal_initial_project_setup
title: "Initial project setup"
status: ready
owner: TODO
module: "project"
priority: P1
authority: canonical
project_id: "{project_id}"
entry_mode: greenfield
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
    - ".punk/memory/**"
  exclude:
    - "work/**"
    - "knowledge/**"
    - "docs/adr/**"
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
  - "Project id and greenfield entry mode remain visible in .punk/memory/STATUS.md."
  - "The next bounded implementation goal is created and selected in .punk/memory/STATUS.md."
knowledge_refs: []
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
research_gate:
  classification: R0
  required: false
  rationale: "Initial project setup only inventories user-provided/repo-tracked project inputs and does not change architecture."
  research_refs: []
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: initial-project-memory
  required_updates:
    - ".punk/memory/STATUS.md"
    - ".punk/memory/reports/**"
  rationale: "Initial project truth capture establishes the manual Level 0 memory baseline."
---

## Context

The project has been initialized as a Punk greenfield project with Level 0 manual project memory.

Project id: `{project_id}`

## Intent

Capture the current product, design, technical constraints, non-goals, and first implementation boundary before coding starts.

## Non-scope

Do not implement product behavior in this goal.

Do not write `.punk/` runtime stores.

Do not claim gate acceptance or proofpack coverage.
"#,
        project_id = project_id.as_str()
    )
}

fn brownfield_work_status_template(project_id: &ProjectId) -> String {
    format!(
        r#"---
id: work_status
kind: manual-work-ledger
status: active
authority: canonical
owner: TODO
ledger_version: work-ledger.v0.1
dogfooding_level: 0
project_id: "{project_id}"
entry_mode: brownfield
reconstruction_status: not_started
updated_at: TODO
current_phase: "Dogfooding Level 0 / brownfield reconstruction scaffold"
current_focus: "Prepare brownfield reconstruction baseline"
selected_next: "{BROWNFIELD_BASELINE_GOAL_PATH}"
last_validated_commit: null
---

# Work Status

## Now

- Current stage: brownfield project initialized with Punk Level 0 advisory reconstruction workspace.
- Current focus: prepare a future source-linked brownfield reconstruction baseline.
- Selected next: `{BROWNFIELD_BASELINE_GOAL_PATH}`
- Reconstruction status: not_started.

## Brownfield Boundary

No project knowledge has been reconstructed yet.
Existing code/docs/history are not Punk truth.
Future reconstruction artifacts are advisory candidates until reviewed.

## Blockers

- Fill owner/date TODOs before treating this ledger as current.
- Create and review future source-linked reconstruction artifacts before drafting contracts.

## Recent Evidence

- `punk init {project_id} --mode brownfield` created the brownfield entry scaffold.

## Open Drift Findings

None.
"#,
        project_id = project_id.as_str()
    )
}

fn brownfield_baseline_goal_template(project_id: &ProjectId) -> String {
    format!(
        r#"---
id: goal_brownfield_reconstruction_baseline
title: "Brownfield reconstruction baseline"
status: ready
owner: TODO
module: "project"
priority: P1
authority: canonical
project_id: "{project_id}"
entry_mode: brownfield
reconstruction_status: not_started
created_at: TODO
updated_at: TODO
selected_at: TODO
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - ".punk/memory/STATUS.md"
    - ".punk/memory/reconstruction/**"
    - ".punk/memory/reports/**"
  exclude:
    - "work/**"
    - "knowledge/**"
    - "docs/adr/**"
    - ".punk/events/**"
    - ".punk/contracts/**"
    - ".punk/runs/**"
    - ".punk/evals/**"
    - ".punk/decisions/**"
    - ".punk/proofs/**"
    - ".punk/indexes/**"
    - ".punk/views/**"
    - ".punk/runtime/**"
    - ".punk/cache/**"
acceptance:
  - "A future source corpus manifest boundary is defined before any inventory is generated."
  - "Future claim-ledger, unknowns, contradictions, and contract-readiness artifacts remain advisory until reviewed."
  - "No project knowledge is treated as reconstructed or accepted automatically."
  - "No repo scan, AI summary, contract generation, gate decision, proof, or Writer behavior is activated."
knowledge_refs: []
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
research_gate:
  classification: R1
  required: false
  rationale: "Brownfield baseline preparation records advisory reconstruction workspace boundaries without external research or repo analysis."
  research_refs: []
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: project-memory
  required_updates:
    - ".punk/memory/STATUS.md"
    - ".punk/memory/reconstruction/**"
    - ".punk/memory/reports/**"
  rationale: "Brownfield reconstruction preparation changes the manual Level 0 project-memory baseline."
---

## Context

The project has been initialized as a Punk brownfield project with Level 0 advisory reconstruction workspace.

Project id: `{project_id}`

## Intent

Prepare a reviewed source-linked reconstruction baseline before any brownfield claims are promoted.

## Non-scope

Do not scan the repository.

Do not generate summaries, contracts, specs, claims, gate decisions, proofs, or acceptance claims.

Do not write `.punk/` runtime stores.
"#,
        project_id = project_id.as_str()
    )
}

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

const BROWNFIELD_RECONSTRUCTION_README_TEMPLATE: &str = r#"# Brownfield Reconstruction Workspace

This directory is the advisory workspace for future brownfield reconstruction artifacts.

No project knowledge has been reconstructed yet.

Existing code, docs, tests, history, and issues are not Punk truth.

Future reconstruction artifacts are advisory candidates until reviewed.

No repo scan, AI summary, contract, gate decision, proof, or acceptance claim has been created by init.
"#;

const SOURCE_CORPUS_MANIFEST_TEMPLATE: &str = r#"# Source Corpus Manifest

Purpose: Future source-linked inventory of files, docs, tests, CI, schemas, and history sources.

Status: not_started.

No generated inventory exists yet.

Do not infer project intent from this placeholder.
"#;

const CLAIM_LEDGER_TEMPLATE: &str = r#"# Claim Ledger

Purpose: Future ledger of source-linked reconstructed claims.

Status: not_started.

No claims are accepted automatically.

No claims have been reconstructed or reviewed yet.
"#;

const UNKNOWNS_TEMPLATE: &str = r#"# Unknowns

Purpose: Future unknowns/blockers for safe contract drafting.

Status: not_started.

No unknowns have been collected yet.
"#;

const CONTRADICTIONS_TEMPLATE: &str = r#"# Contradictions

Purpose: Future stale/conflicting docs/code/history findings.

Status: not_started.

No contradictions have been collected yet.
"#;

const CONTRACT_READINESS_TEMPLATE: &str = r#"# Contract Readiness

Purpose: Future Contract Readiness Baseline.

Status: not_started.

This is not a contract.
This is not a gate decision.
This is not proof.

No contract-readiness claims have been reconstructed or reviewed yet.
"#;

fn punk_readme_template(entry_mode: ProjectInitEntryMode) -> String {
    match entry_mode {
        ProjectInitEntryMode::Greenfield => r#"# .punk

This directory is the Punk project root marker.

Current active behavior is Dogfooding Level 0 compact manual project memory.

Tracked durable project memory lives under `.punk/memory/`.

Authoritative live work state for this project is `.punk/memory/STATUS.md`.

Runtime and derived stores such as runtime, cache, events, contracts, runs, evals, decisions, proofs, indexes, and views are not active yet.
"#
        .to_owned(),
        ProjectInitEntryMode::Brownfield => r#"# .punk

This directory is the Punk project root marker.

Current active behavior is Dogfooding Level 0 brownfield entry scaffold.

Tracked durable project memory lives under `.punk/memory/`.

Authoritative live work state for this project is `.punk/memory/STATUS.md`.

Brownfield reconstruction has not started.

The reconstruction workspace under `.punk/memory/reconstruction/` is advisory until reviewed.

Runtime and derived stores such as runtime, cache, events, contracts, runs, evals, decisions, proofs, indexes, and views are not active yet.
"#
        .to_owned(),
    }
}

fn punk_project_toml_template(project_id: &ProjectId, entry_mode: ProjectInitEntryMode) -> String {
    let brownfield_section = match entry_mode {
        ProjectInitEntryMode::Greenfield => "",
        ProjectInitEntryMode::Brownfield => {
            r#"
[brownfield]
reconstruction_status = "not_started"
authority = "advisory_candidates_only"
"#
        }
    };

    format!(
        r#"# Punk project marker.
# This file is setup metadata, not runtime authority.

schema_version = "punk.project.v0.1"
project_id = "{project_id}"
entry_mode = "{entry_mode}"
dogfooding_level = 0
runtime_persistence = "inactive"
live_work_state = ".punk/memory/STATUS.md"

[memory]
layout = "compact"
root = ".punk/memory"

[runtime]
active = false
root = ".punk/runtime"

[authority]
live_state = ".punk/memory/STATUS.md"
final_decisions = "not_active"
proofpacks = "not_active"
{brownfield_section}
"#,
        project_id = project_id.as_str(),
        entry_mode = entry_mode.as_str(),
        brownfield_section = brownfield_section
    )
}

const PROJECT_INIT_BOUNDARY_NOTES: &[&str] = &[
    "writes only greenfield Level 0 compact project-memory scaffold files under .punk/memory",
    "records project_id and entry_mode = greenfield in the scaffold",
    "creates .punk as a project root marker and .punk/memory as tracked durable memory",
    "does not create root-level work, knowledge, docs/adr, or publishing directories",
    "does not implement brownfield reconstruction or grayfield reconciliation",
    "does not create contracts, run receipts, gate artifacts, proofpacks, or acceptance claims",
    "does not perform repo scanning, AI summaries, generated truth, or network behavior",
    "uses create-new/no-overwrite behavior and reports conflicts fail-closed",
];

const PROJECT_INIT_DEFERRED_NOTES: &[&str] = &[
    "brownfield reconstruction and grayfield reconciliation remain deferred",
    ".punk/runtime project storage remains inactive",
    "flow persistence and event writing remain inactive",
    "contract writer, receipt writer, gate writer, proof writer, and proofpack writer remain inactive",
    "project-specific source/design refs must be filled manually after init",
];

const BROWNFIELD_PROJECT_INIT_BOUNDARY_NOTES: &[&str] = &[
    "writes only a brownfield Level 0 advisory reconstruction scaffold under .punk/memory",
    "records project_id and entry_mode = brownfield in the scaffold",
    "records reconstruction_status = not_started and authority = advisory_candidates_only",
    "creates .punk as a project root marker and .punk/memory as tracked durable memory",
    "creates empty reconstruction placeholders for source corpus, claims, unknowns, contradictions, and contract readiness",
    "does not create root-level work, knowledge, docs/adr, or publishing directories",
    "does not scan the repository or infer project knowledge",
    "does not create contracts, specs, gate artifacts, proofpacks, or acceptance claims",
    "uses create-new/no-overwrite behavior and reports conflicts fail-closed",
];

const BROWNFIELD_PROJECT_INIT_DEFERRED_NOTES: &[&str] = &[
    "brownfield reconstruction remains not_started",
    "grayfield reconciliation remains deferred",
    ".punk/runtime project storage remains inactive",
    "flow persistence and event writing remain inactive",
    "contract writer, receipt writer, gate writer, proof writer, and proofpack writer remain inactive",
    "all future reconstructed claims remain advisory candidates until reviewed",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectId(String);

impl ProjectId {
    pub fn parse(value: impl Into<String>) -> Result<Self, ProjectIdError> {
        let value = value.into();
        validate_project_id(&value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectIdError {
    Empty,
    InvalidFormat,
}

impl ProjectIdError {
    pub fn message(self) -> &'static str {
        match self {
            Self::Empty => "project id must not be empty",
            Self::InvalidFormat => PROJECT_ID_FORMAT_NOTE,
        }
    }
}

fn validate_project_id(value: &str) -> Result<(), ProjectIdError> {
    if value.is_empty() {
        return Err(ProjectIdError::Empty);
    }

    let mut previous_was_hyphen = false;
    for (index, character) in value.chars().enumerate() {
        let valid =
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-';
        if !valid {
            return Err(ProjectIdError::InvalidFormat);
        }
        if character == '-' {
            if index == 0 || previous_was_hyphen {
                return Err(ProjectIdError::InvalidFormat);
            }
            previous_was_hyphen = true;
        } else {
            previous_was_hyphen = false;
        }
    }

    if previous_was_hyphen {
        return Err(ProjectIdError::InvalidFormat);
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectInitEntryMode {
    Greenfield,
    Brownfield,
}

impl ProjectInitEntryMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Greenfield => PROJECT_INIT_ENTRY_MODE,
            Self::Brownfield => PROJECT_INIT_BROWNFIELD_ENTRY_MODE,
        }
    }

    fn schema_version(self) -> &'static str {
        match self {
            Self::Greenfield => PROJECT_INIT_SCHEMA_VERSION,
            Self::Brownfield => PROJECT_INIT_BROWNFIELD_SCHEMA_VERSION,
        }
    }

    fn entries(self) -> &'static [ProjectInitEntry] {
        match self {
            Self::Greenfield => GREENFIELD_PROJECT_INIT_ENTRIES,
            Self::Brownfield => BROWNFIELD_PROJECT_INIT_ENTRIES,
        }
    }

    fn boundary_notes(self) -> &'static [&'static str] {
        match self {
            Self::Greenfield => PROJECT_INIT_BOUNDARY_NOTES,
            Self::Brownfield => BROWNFIELD_PROJECT_INIT_BOUNDARY_NOTES,
        }
    }

    fn deferred_notes(self) -> &'static [&'static str] {
        match self {
            Self::Greenfield => PROJECT_INIT_DEFERRED_NOTES,
            Self::Brownfield => BROWNFIELD_PROJECT_INIT_DEFERRED_NOTES,
        }
    }
}

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
    Planned,
    Created,
    AlreadyExists,
    Conflict,
    Failed,
}

impl ProjectInitArtifactStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
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
    project_id: ProjectId,
    entry_mode: ProjectInitEntryMode,
    artifacts: Vec<ProjectInitArtifactReport>,
}

impl ProjectInitReport {
    pub fn project_root(&self) -> &Path {
        &self.project_root
    }

    pub fn project_id(&self) -> &ProjectId {
        &self.project_id
    }

    pub fn entry_mode(&self) -> &'static str {
        self.entry_mode.as_str()
    }

    pub fn schema_version(&self) -> &'static str {
        self.entry_mode.schema_version()
    }

    pub fn artifacts(&self) -> &[ProjectInitArtifactReport] {
        &self.artifacts
    }

    pub fn boundary_notes(&self) -> &'static [&'static str] {
        self.entry_mode.boundary_notes()
    }

    pub fn deferred_notes(&self) -> &'static [&'static str] {
        self.entry_mode.deferred_notes()
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
        writeln!(&mut output, "schema_version: {}", self.schema_version())
            .expect("writing to String should succeed");
        writeln!(&mut output, "mode: {PROJECT_INIT_MODE}")
            .expect("writing to String should succeed");
        writeln!(&mut output, "entry_mode: {}", self.entry_mode())
            .expect("writing to String should succeed");
        writeln!(&mut output, "project_id: {}", self.project_id().as_str())
            .expect("writing to String should succeed");
        writeln!(
            &mut output,
            "runtime_persistence: {PROJECT_INIT_RUNTIME_PERSISTENCE}"
        )
        .expect("writing to String should succeed");
        writeln!(&mut output, "target_root: .").expect("writing to String should succeed");
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
    GeneratedFile(&'static str, ProjectInitTemplate),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProjectInitTemplate {
    WorkStatus,
    InitialGoal,
    PunkReadme,
    PunkProjectToml,
}

const GREENFIELD_PROJECT_INIT_ENTRIES: &[ProjectInitEntry] = &[
    ProjectInitEntry::Directory(".punk"),
    ProjectInitEntry::Directory(MEMORY_ROOT),
    ProjectInitEntry::Directory(".punk/memory/goals"),
    ProjectInitEntry::Directory(".punk/memory/reports"),
    ProjectInitEntry::Directory(".punk/memory/knowledge"),
    ProjectInitEntry::Directory(".punk/memory/knowledge/research"),
    ProjectInitEntry::Directory(".punk/memory/knowledge/ideas"),
    ProjectInitEntry::Directory(".punk/memory/adr"),
    ProjectInitEntry::GeneratedFile(STATUS_PATH, ProjectInitTemplate::WorkStatus),
    ProjectInitEntry::GeneratedFile(INITIAL_GOAL_PATH, ProjectInitTemplate::InitialGoal),
    ProjectInitEntry::File(".punk/memory/reports/README.md", REPORTS_README_TEMPLATE),
    ProjectInitEntry::File(".punk/memory/adr/README.md", ADR_README_TEMPLATE),
    ProjectInitEntry::File(
        ".punk/memory/knowledge/research/README.md",
        RESEARCH_README_TEMPLATE,
    ),
    ProjectInitEntry::File(
        ".punk/memory/knowledge/ideas/README.md",
        IDEAS_README_TEMPLATE,
    ),
    ProjectInitEntry::GeneratedFile(".punk/README.md", ProjectInitTemplate::PunkReadme),
    ProjectInitEntry::GeneratedFile(".punk/project.toml", ProjectInitTemplate::PunkProjectToml),
];

const BROWNFIELD_PROJECT_INIT_ENTRIES: &[ProjectInitEntry] = &[
    ProjectInitEntry::Directory(".punk"),
    ProjectInitEntry::Directory(MEMORY_ROOT),
    ProjectInitEntry::Directory(".punk/memory/goals"),
    ProjectInitEntry::Directory(".punk/memory/reports"),
    ProjectInitEntry::Directory(".punk/memory/reconstruction"),
    ProjectInitEntry::GeneratedFile(STATUS_PATH, ProjectInitTemplate::WorkStatus),
    ProjectInitEntry::GeneratedFile(
        BROWNFIELD_BASELINE_GOAL_PATH,
        ProjectInitTemplate::InitialGoal,
    ),
    ProjectInitEntry::File(
        ".punk/memory/reconstruction/README.md",
        BROWNFIELD_RECONSTRUCTION_README_TEMPLATE,
    ),
    ProjectInitEntry::File(
        ".punk/memory/reconstruction/source-corpus-manifest.md",
        SOURCE_CORPUS_MANIFEST_TEMPLATE,
    ),
    ProjectInitEntry::File(
        ".punk/memory/reconstruction/claim-ledger.md",
        CLAIM_LEDGER_TEMPLATE,
    ),
    ProjectInitEntry::File(".punk/memory/reconstruction/unknowns.md", UNKNOWNS_TEMPLATE),
    ProjectInitEntry::File(
        ".punk/memory/reconstruction/contradictions.md",
        CONTRADICTIONS_TEMPLATE,
    ),
    ProjectInitEntry::File(
        ".punk/memory/reconstruction/contract-readiness.md",
        CONTRACT_READINESS_TEMPLATE,
    ),
    ProjectInitEntry::GeneratedFile(".punk/README.md", ProjectInitTemplate::PunkReadme),
    ProjectInitEntry::GeneratedFile(".punk/project.toml", ProjectInitTemplate::PunkProjectToml),
];

pub fn init_level0_project(
    project_root: impl AsRef<Path>,
    project_id: ProjectId,
) -> ProjectInitReport {
    init_project(project_root, project_id, ProjectInitEntryMode::Greenfield)
}

pub fn init_project(
    project_root: impl AsRef<Path>,
    project_id: ProjectId,
    entry_mode: ProjectInitEntryMode,
) -> ProjectInitReport {
    let project_root = project_root.as_ref().to_path_buf();

    match fs::metadata(&project_root) {
        Ok(metadata) if metadata.is_dir() => {}
        Ok(_) => {
            let artifacts = vec![ProjectInitArtifactReport::new(
                ".",
                ProjectInitArtifactKind::Directory,
                ProjectInitArtifactStatus::Failed,
                Some("project root exists but is not a directory".to_owned()),
            )];
            return ProjectInitReport {
                project_root,
                project_id,
                entry_mode,
                artifacts,
            };
        }
        Err(error) => {
            let artifacts = vec![ProjectInitArtifactReport::new(
                ".",
                ProjectInitArtifactKind::Directory,
                ProjectInitArtifactStatus::Failed,
                Some(format!("project root must already exist: {error}")),
            )];
            return ProjectInitReport {
                project_root,
                project_id,
                entry_mode,
                artifacts,
            };
        }
    }

    let preflight = preflight_init_entries(&project_root, &project_id, entry_mode);
    if preflight.iter().any(ProjectInitArtifactReport::is_blocking) {
        return ProjectInitReport {
            project_root,
            project_id,
            entry_mode,
            artifacts: preflight,
        };
    }

    let artifacts = apply_init_entries(&project_root, &project_id, entry_mode, &preflight);

    ProjectInitReport {
        project_root,
        project_id,
        entry_mode,
        artifacts,
    }
}

fn render_init_template(
    template: ProjectInitTemplate,
    project_id: &ProjectId,
    entry_mode: ProjectInitEntryMode,
) -> String {
    match (template, entry_mode) {
        (ProjectInitTemplate::WorkStatus, ProjectInitEntryMode::Greenfield) => {
            work_status_template(project_id)
        }
        (ProjectInitTemplate::WorkStatus, ProjectInitEntryMode::Brownfield) => {
            brownfield_work_status_template(project_id)
        }
        (ProjectInitTemplate::InitialGoal, ProjectInitEntryMode::Greenfield) => {
            initial_goal_template(project_id)
        }
        (ProjectInitTemplate::InitialGoal, ProjectInitEntryMode::Brownfield) => {
            brownfield_baseline_goal_template(project_id)
        }
        (ProjectInitTemplate::PunkReadme, _) => punk_readme_template(entry_mode),
        (ProjectInitTemplate::PunkProjectToml, _) => {
            punk_project_toml_template(project_id, entry_mode)
        }
    }
}

fn preflight_init_entries(
    project_root: &Path,
    project_id: &ProjectId,
    entry_mode: ProjectInitEntryMode,
) -> Vec<ProjectInitArtifactReport> {
    entry_mode
        .entries()
        .iter()
        .map(|entry| match *entry {
            ProjectInitEntry::Directory(path) => preflight_init_directory(project_root, path),
            ProjectInitEntry::File(path, contents) => {
                preflight_init_file(project_root, path, contents.as_bytes())
            }
            ProjectInitEntry::GeneratedFile(path, template) => {
                let contents = render_init_template(template, project_id, entry_mode);
                preflight_init_file(project_root, path, contents.as_bytes())
            }
        })
        .collect()
}

fn apply_init_entries(
    project_root: &Path,
    project_id: &ProjectId,
    entry_mode: ProjectInitEntryMode,
    preflight: &[ProjectInitArtifactReport],
) -> Vec<ProjectInitArtifactReport> {
    entry_mode
        .entries()
        .iter()
        .zip(preflight)
        .map(|(entry, preflight_report)| {
            if preflight_report.status() == ProjectInitArtifactStatus::AlreadyExists {
                return preflight_report.clone();
            }

            match *entry {
                ProjectInitEntry::Directory(path) => create_init_directory(project_root, path),
                ProjectInitEntry::File(path, contents) => {
                    create_init_file(project_root, path, contents.as_bytes())
                }
                ProjectInitEntry::GeneratedFile(path, template) => {
                    let contents = render_init_template(template, project_id, entry_mode);
                    create_init_file(project_root, path, contents.as_bytes())
                }
            }
        })
        .collect()
}

fn preflight_init_directory(
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
        Err(error) if error.kind() == io::ErrorKind::NotFound => ProjectInitArtifactReport::new(
            repo_relative_path,
            ProjectInitArtifactKind::Directory,
            ProjectInitArtifactStatus::Planned,
            None,
        ),
        Err(error) => ProjectInitArtifactReport::new(
            repo_relative_path,
            ProjectInitArtifactKind::Directory,
            ProjectInitArtifactStatus::Failed,
            Some(format!("could not inspect path: {error}")),
        ),
    }
}

fn preflight_init_file(
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
        Err(error) if error.kind() == io::ErrorKind::NotFound => ProjectInitArtifactReport::new(
            repo_relative_path,
            ProjectInitArtifactKind::File,
            ProjectInitArtifactStatus::Planned,
            None,
        ),
        Err(error) => ProjectInitArtifactReport::new(
            repo_relative_path,
            ProjectInitArtifactKind::File,
            ProjectInitArtifactStatus::Failed,
            Some(format!("could not inspect path: {error}")),
        ),
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
    use super::{
        init_level0_project, init_project, ProjectId, ProjectIdError, ProjectInitArtifactKind,
        ProjectInitArtifactStatus, ProjectInitEntryMode, BROWNFIELD_BASELINE_GOAL_PATH,
        INITIAL_GOAL_PATH, PROJECT_INIT_BROWNFIELD_ENTRY_MODE, PROJECT_INIT_ENTRY_MODE,
        STATUS_PATH,
    };
    use std::fs;
    use std::process;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn init_creates_level0_manual_memory_scaffold() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_level0_project(&root, project_id);

        assert!(!report.blocked());
        assert_eq!(report.exit_code(), 0);
        assert_eq!(report.result_label(), "initialized");
        assert_eq!(report.project_id().as_str(), "weekend-project");
        assert_eq!(report.entry_mode(), PROJECT_INIT_ENTRY_MODE);
        assert!(root.join(STATUS_PATH).is_file());
        assert!(root.join(INITIAL_GOAL_PATH).is_file());
        assert!(root.join(".punk/memory/reports/README.md").is_file());
        assert!(root.join(".punk/memory/adr/README.md").is_file());
        assert!(root
            .join(".punk/memory/knowledge/research/README.md")
            .is_file());
        assert!(root
            .join(".punk/memory/knowledge/ideas/README.md")
            .is_file());
        assert!(root.join(".punk/README.md").is_file());
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
        assert!(!root.join(".punk/evals").exists());
        assert!(!root.join(".punk/decisions").exists());
        assert!(!root.join(".punk/proofs").exists());
        assert!(!root.join(".punk/indexes").exists());
        assert!(!root.join(".punk/views").exists());

        let status =
            fs::read_to_string(root.join(STATUS_PATH)).expect("status template should be readable");
        assert!(status.contains("dogfooding_level: 0"));
        assert!(status.contains("project_id: \"weekend-project\""));
        assert!(status.contains("entry_mode: greenfield"));
        assert!(status.contains(&format!("selected_next: \"{INITIAL_GOAL_PATH}\"")));
        let goal = fs::read_to_string(root.join(INITIAL_GOAL_PATH))
            .expect("initial goal template should be readable");
        assert!(goal.contains("id: goal_initial_project_setup"));
        assert!(goal.contains("project_id: \"weekend-project\""));
        assert!(goal.contains("entry_mode: greenfield"));
        let project_marker = fs::read_to_string(root.join(".punk/project.toml"))
            .expect("project marker should be readable");
        assert!(project_marker.contains("schema_version = \"punk.project.v0.1\""));
        assert!(project_marker.contains("project_id = \"weekend-project\""));
        assert!(project_marker.contains("entry_mode = \"greenfield\""));
        assert!(project_marker.contains("runtime_persistence = \"inactive\""));
        assert!(project_marker.contains("live_work_state = \".punk/memory/STATUS.md\""));
        assert!(project_marker.contains("[memory]"));
        assert!(project_marker.contains("layout = \"compact\""));
        assert!(project_marker.contains("root = \".punk/memory\""));
        assert!(project_marker.contains("[runtime]"));
        assert!(project_marker.contains("active = false"));
        assert!(project_marker.contains("root = \".punk/runtime\""));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn greenfield_default_behavior_unchanged() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_level0_project(&root, project_id);

        assert!(!report.blocked());
        assert_eq!(report.entry_mode(), PROJECT_INIT_ENTRY_MODE);
        assert!(root.join(INITIAL_GOAL_PATH).is_file());
        assert!(!root.join(BROWNFIELD_BASELINE_GOAL_PATH).exists());
        assert!(!root.join(".punk/memory/reconstruction").exists());

        let project_marker = fs::read_to_string(root.join(".punk/project.toml"))
            .expect("project marker should be readable");
        assert!(project_marker.contains("entry_mode = \"greenfield\""));
        assert!(!project_marker.contains("[brownfield]"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn brownfield_init_creates_compact_memory_scaffold() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_project(&root, project_id, ProjectInitEntryMode::Brownfield);

        assert!(!report.blocked());
        assert_eq!(report.exit_code(), 0);
        assert_eq!(report.result_label(), "initialized");
        assert_eq!(report.entry_mode(), PROJECT_INIT_BROWNFIELD_ENTRY_MODE);
        assert!(root.join(STATUS_PATH).is_file());
        assert!(root.join(BROWNFIELD_BASELINE_GOAL_PATH).is_file());
        assert!(root.join(".punk/memory/reports").is_dir());
        assert!(root.join(".punk/memory/reconstruction").is_dir());
        assert!(root.join(".punk/README.md").is_file());
        assert!(root.join(".punk/project.toml").is_file());
        assert!(!root.join("work").exists());
        assert!(!root.join("knowledge").exists());
        assert!(!root.join("docs").exists());
        assert!(!root.join("docs/adr").exists());
        assert!(!root.join("publishing").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn brownfield_init_records_entry_mode_brownfield() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_project(&root, project_id, ProjectInitEntryMode::Brownfield);

        assert_eq!(report.entry_mode(), PROJECT_INIT_BROWNFIELD_ENTRY_MODE);
        let status =
            fs::read_to_string(root.join(STATUS_PATH)).expect("status template should be readable");
        let project_marker = fs::read_to_string(root.join(".punk/project.toml"))
            .expect("project marker should be readable");
        assert!(status.contains("entry_mode: brownfield"));
        assert!(project_marker.contains("entry_mode = \"brownfield\""));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn brownfield_init_records_reconstruction_not_started() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_project(&root, project_id, ProjectInitEntryMode::Brownfield);

        assert!(!report.blocked());
        let status =
            fs::read_to_string(root.join(STATUS_PATH)).expect("status template should be readable");
        let project_marker = fs::read_to_string(root.join(".punk/project.toml"))
            .expect("project marker should be readable");
        assert!(status.contains("reconstruction_status: not_started"));
        assert!(status.contains(&format!(
            "selected_next: \"{BROWNFIELD_BASELINE_GOAL_PATH}\""
        )));
        assert!(status.contains("No project knowledge has been reconstructed yet."));
        assert!(status.contains("Existing code/docs/history are not Punk truth."));
        assert!(status
            .contains("Future reconstruction artifacts are advisory candidates until reviewed."));
        assert!(project_marker.contains("[brownfield]"));
        assert!(project_marker.contains("reconstruction_status = \"not_started\""));
        assert!(project_marker.contains("authority = \"advisory_candidates_only\""));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn brownfield_init_creates_reconstruction_placeholders() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_project(&root, project_id, ProjectInitEntryMode::Brownfield);

        assert!(!report.blocked());
        for path in [
            ".punk/memory/reconstruction/README.md",
            ".punk/memory/reconstruction/source-corpus-manifest.md",
            ".punk/memory/reconstruction/claim-ledger.md",
            ".punk/memory/reconstruction/unknowns.md",
            ".punk/memory/reconstruction/contradictions.md",
            ".punk/memory/reconstruction/contract-readiness.md",
        ] {
            assert!(root.join(path).is_file(), "{path} should be created");
        }
        let source_manifest =
            fs::read_to_string(root.join(".punk/memory/reconstruction/source-corpus-manifest.md"))
                .expect("source corpus manifest should be readable");
        let claim_ledger =
            fs::read_to_string(root.join(".punk/memory/reconstruction/claim-ledger.md"))
                .expect("claim ledger should be readable");
        let contract_readiness =
            fs::read_to_string(root.join(".punk/memory/reconstruction/contract-readiness.md"))
                .expect("contract readiness should be readable");
        assert!(source_manifest.contains(
            "Future source-linked inventory of files, docs, tests, CI, schemas, and history sources."
        ));
        assert!(claim_ledger.contains("No claims are accepted automatically."));
        assert!(contract_readiness.contains("This is not a contract."));
        assert!(contract_readiness.contains("This is not a gate decision."));
        assert!(contract_readiness.contains("This is not proof."));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn brownfield_init_does_not_scan_repo() {
        let root = unique_temp_path();
        fs::create_dir_all(root.join("src")).expect("source dir should be created");
        fs::write(root.join("src/main.rs"), "fn main() {}\n").expect("source should be written");
        fs::write(root.join("README.md"), "# Existing project\n")
            .expect("readme should be written");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_project(&root, project_id, ProjectInitEntryMode::Brownfield);

        assert!(!report.blocked());
        let source_manifest =
            fs::read_to_string(root.join(".punk/memory/reconstruction/source-corpus-manifest.md"))
                .expect("source corpus manifest should be readable");
        let claim_ledger =
            fs::read_to_string(root.join(".punk/memory/reconstruction/claim-ledger.md"))
                .expect("claim ledger should be readable");
        assert!(!source_manifest.contains("src/main.rs"));
        assert!(!source_manifest.contains("README.md"));
        assert!(!claim_ledger.contains("fn main"));
        assert!(!claim_ledger.contains("Existing project"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn brownfield_init_does_not_create_runtime_dirs() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_project(&root, project_id, ProjectInitEntryMode::Brownfield);

        assert!(!report.blocked());
        for path in [
            ".punk/events",
            ".punk/contracts",
            ".punk/runs",
            ".punk/evals",
            ".punk/decisions",
            ".punk/proofs",
            ".punk/views",
            ".punk/indexes",
            ".punk/runtime",
            ".punk/cache",
        ] {
            assert!(!root.join(path).exists(), "{path} should stay inactive");
        }

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn brownfield_init_does_not_create_contracts_or_claims() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_project(&root, project_id, ProjectInitEntryMode::Brownfield);

        assert!(!report.blocked());
        assert!(!root.join(".punk/contracts").exists());
        assert!(!root.join(".punk/proofs").exists());
        let claim_ledger =
            fs::read_to_string(root.join(".punk/memory/reconstruction/claim-ledger.md"))
                .expect("claim ledger should be readable");
        let contract_readiness =
            fs::read_to_string(root.join(".punk/memory/reconstruction/contract-readiness.md"))
                .expect("contract readiness should be readable");
        assert!(claim_ledger.contains("No claims are accepted automatically."));
        assert!(claim_ledger.contains("No claims have been reconstructed or reviewed yet."));
        assert!(contract_readiness
            .contains("No contract-readiness claims have been reconstructed or reviewed yet."));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn brownfield_init_conflict_is_atomic_noop() {
        let root = unique_temp_path();
        fs::create_dir_all(root.join(".punk/memory")).expect("memory dir should be created");
        fs::write(root.join(STATUS_PATH), "custom brownfield status\n")
            .expect("custom status should be written");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_project(&root, project_id, ProjectInitEntryMode::Brownfield);

        assert!(report.blocked());
        assert_eq!(report.exit_code(), 1);
        assert_eq!(report.result_label(), "blocked");
        assert!(report.artifacts().iter().any(|artifact| {
            artifact.repo_relative_path() == STATUS_PATH
                && artifact.kind() == ProjectInitArtifactKind::File
                && artifact.status() == ProjectInitArtifactStatus::Conflict
        }));
        assert!(report.artifacts().iter().any(|artifact| {
            artifact.repo_relative_path() == BROWNFIELD_BASELINE_GOAL_PATH
                && artifact.kind() == ProjectInitArtifactKind::File
                && artifact.status() == ProjectInitArtifactStatus::Planned
        }));
        assert_eq!(
            fs::read_to_string(root.join(STATUS_PATH)).expect("status should remain readable"),
            "custom brownfield status\n"
        );
        assert!(!root.join(BROWNFIELD_BASELINE_GOAL_PATH).exists());
        assert!(!root.join(".punk/memory/reports").exists());
        assert!(!root.join(".punk/memory/reconstruction").exists());
        assert!(!root.join(".punk/README.md").exists());
        assert!(!root.join(".punk/project.toml").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn init_is_idempotent_when_existing_files_match() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let first = init_level0_project(&root, project_id.clone());
        let second = init_level0_project(&root, project_id);

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
        fs::create_dir_all(root.join(".punk/memory")).expect("memory dir should be created");
        fs::write(root.join(STATUS_PATH), "custom status\n")
            .expect("custom status should be written");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_level0_project(&root, project_id);

        assert!(report.blocked());
        assert_eq!(report.exit_code(), 1);
        assert_eq!(report.result_label(), "blocked");
        assert_eq!(
            fs::read_to_string(root.join(STATUS_PATH)).expect("status should remain readable"),
            "custom status\n"
        );
        assert!(report.artifacts().iter().any(|artifact| {
            artifact.repo_relative_path() == STATUS_PATH
                && artifact.kind() == ProjectInitArtifactKind::File
                && artifact.status() == ProjectInitArtifactStatus::Conflict
        }));
        assert!(!root.join(INITIAL_GOAL_PATH).exists());
        assert!(!root.join(".punk/memory/reports/README.md").exists());
        assert!(!root.join(".punk/memory/adr/README.md").exists());
        assert!(!root
            .join(".punk/memory/knowledge/research/README.md")
            .exists());
        assert!(!root.join(".punk/memory/knowledge/ideas/README.md").exists());
        assert!(!root.join(".punk/README.md").exists());
        assert!(!root.join(".punk/project.toml").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn init_conflict_is_noop_for_all_other_artifacts() {
        let root = unique_temp_path();
        fs::create_dir_all(root.join(".punk/memory")).expect("memory dir should be created");
        fs::write(root.join(STATUS_PATH), "custom status\n")
            .expect("custom status should be written");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_level0_project(&root, project_id);

        assert!(report.blocked());
        assert!(report.artifacts().iter().any(|artifact| {
            artifact.repo_relative_path() == INITIAL_GOAL_PATH
                && artifact.status() == ProjectInitArtifactStatus::Planned
        }));
        assert_eq!(
            fs::read_to_string(root.join(STATUS_PATH)).expect("status should remain readable"),
            "custom status\n"
        );
        assert!(!root.join(INITIAL_GOAL_PATH).exists());
        assert!(!root.join(".punk/memory/reports/README.md").exists());
        assert!(!root.join(".punk/memory/adr/README.md").exists());
        assert!(!root.join(".punk/memory/knowledge").exists());
        assert!(!root.join(".punk/README.md").exists());
        assert!(!root.join(".punk/project.toml").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn init_failed_preflight_does_not_leave_partial_scaffold() {
        let root = unique_temp_path();
        fs::create_dir_all(root.join(".punk/memory/reports"))
            .expect("reports dir should be created");
        fs::write(
            root.join(".punk/memory/reports/README.md"),
            "custom report notes\n",
        )
        .expect("custom report readme should be written");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_level0_project(&root, project_id);

        assert!(report.blocked());
        assert_eq!(report.result_label(), "blocked");
        assert!(report.artifacts().iter().any(|artifact| {
            artifact.repo_relative_path() == ".punk/memory/reports/README.md"
                && artifact.kind() == ProjectInitArtifactKind::File
                && artifact.status() == ProjectInitArtifactStatus::Conflict
        }));
        assert_eq!(
            fs::read_to_string(root.join(".punk/memory/reports/README.md"))
                .expect("report readme should remain readable"),
            "custom report notes\n"
        );
        assert!(!root.join(STATUS_PATH).exists());
        assert!(!root.join(INITIAL_GOAL_PATH).exists());
        assert!(!root.join(".punk/memory/knowledge").exists());
        assert!(!root.join(".punk/memory/adr").exists());
        assert!(!root.join(".punk/README.md").exists());
        assert!(!root.join(".punk/project.toml").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn render_human_redacts_absolute_target_root() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_level0_project(&root, project_id);
        let rendered = report.render_human();

        assert!(rendered.contains("target_root: ."));
        assert!(!rendered.contains(&root.display().to_string()));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn project_id_requires_lowercase_slug() {
        assert_eq!(
            ProjectId::parse("").expect_err("empty project id should fail"),
            ProjectIdError::Empty
        );
        for invalid in [
            "Weekend",
            "weekend project",
            "-weekend",
            "weekend-",
            "weekend--project",
            "weekend_project",
            "weekend.project",
        ] {
            assert_eq!(
                ProjectId::parse(invalid).expect_err("invalid project id should fail"),
                ProjectIdError::InvalidFormat
            );
        }

        assert_eq!(
            ProjectId::parse("weekend-project-01")
                .expect("valid project id should parse")
                .as_str(),
            "weekend-project-01"
        );
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
