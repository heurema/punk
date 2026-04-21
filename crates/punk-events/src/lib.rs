//! Minimal append-only event log kernel for Punk Phase 1.
//!
//! This crate intentionally stays narrow in its first implementation step.
//! It defines a small event schema fixture, append-only log behavior, and
//! deterministic JSONL output without activating `.punk/` runtime paths,
//! CLI inspect behavior, or replay/gate/proof integration.

use std::fs::{File, OpenOptions};
use std::io::{self, Cursor, Write};
use std::path::Path;

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const EVENT_SCHEMA_VERSION: &str = "punk.event.v0.1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventPhase {
    Init,
    Plot,
    Cut,
    Gate,
    Eval,
    Inspect,
}

impl EventPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::Plot => "plot",
            Self::Cut => "cut",
            Self::Gate => "gate",
            Self::Eval => "eval",
            Self::Inspect => "inspect",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventKind {
    ProjectInitialized,
    TransitionAttempt,
    TransitionCommitted,
    TransitionDenied,
    GuardEvaluated,
    InspectRead,
    EventReplayStarted,
    EventReplayCompleted,
}

impl EventKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProjectInitialized => "project_initialized",
            Self::TransitionAttempt => "transition_attempt",
            Self::TransitionCommitted => "transition_committed",
            Self::TransitionDenied => "transition_denied",
            Self::GuardEvaluated => "guard_evaluated",
            Self::InspectRead => "inspect_read",
            Self::EventReplayStarted => "event_replay_started",
            Self::EventReplayCompleted => "event_replay_completed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventStatus {
    Started,
    Succeeded,
    Failed,
    Denied,
    Skipped,
}

impl EventStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Started => "started",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Denied => "denied",
            Self::Skipped => "skipped",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventSource {
    pub crate_name: String,
    pub component: String,
}

impl EventSource {
    pub fn new(crate_name: impl Into<String>, component: impl Into<String>) -> Self {
        Self {
            crate_name: crate_name.into(),
            component: component.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventCorrelation {
    pub flow_id: String,
    pub goal_ref: Option<String>,
}

impl EventCorrelation {
    pub fn new(flow_id: impl Into<String>) -> Self {
        Self {
            flow_id: flow_id.into(),
            goal_ref: None,
        }
    }

    pub fn with_goal_ref(mut self, goal_ref: impl Into<String>) -> Self {
        self.goal_ref = Some(goal_ref.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowTransitionRef {
    pub from_state: Option<String>,
    pub command: Option<String>,
    pub to_state: Option<String>,
}

impl FlowTransitionRef {
    pub fn new() -> Self {
        Self {
            from_state: None,
            command: None,
            to_state: None,
        }
    }

    pub fn with_from_state(mut self, state: impl Into<String>) -> Self {
        self.from_state = Some(state.into());
        self
    }

    pub fn with_command(mut self, command: impl Into<String>) -> Self {
        self.command = Some(command.into());
        self
    }

    pub fn with_to_state(mut self, state: impl Into<String>) -> Self {
        self.to_state = Some(state.into());
        self
    }
}

impl Default for FlowTransitionRef {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventResult {
    pub status: EventStatus,
    pub guard_code: Option<String>,
    pub error_code: Option<String>,
}

impl EventResult {
    pub fn new(status: EventStatus) -> Self {
        Self {
            status,
            guard_code: None,
            error_code: None,
        }
    }

    pub fn with_guard_code(mut self, guard_code: impl Into<String>) -> Self {
        self.guard_code = Some(guard_code.into());
        self
    }

    pub fn with_error_code(mut self, error_code: impl Into<String>) -> Self {
        self.error_code = Some(error_code.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventArtifacts {
    pub refs: Vec<String>,
}

impl EventArtifacts {
    pub fn new() -> Self {
        Self { refs: Vec::new() }
    }

    pub fn with_ref(mut self, artifact_ref: impl Into<String>) -> Self {
        self.refs.push(artifact_ref.into());
        self
    }
}

impl Default for EventArtifacts {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventDraft {
    pub phase: EventPhase,
    pub kind: EventKind,
    pub source: EventSource,
    pub correlation: EventCorrelation,
    pub transition: FlowTransitionRef,
    pub result: EventResult,
    pub artifacts: EventArtifacts,
}

impl EventDraft {
    pub fn new(
        phase: EventPhase,
        kind: EventKind,
        source: EventSource,
        correlation: EventCorrelation,
        result: EventResult,
    ) -> Self {
        Self {
            phase,
            kind,
            source,
            correlation,
            transition: FlowTransitionRef::default(),
            result,
            artifacts: EventArtifacts::default(),
        }
    }

    pub fn with_transition(mut self, transition: FlowTransitionRef) -> Self {
        self.transition = transition;
        self
    }

    pub fn with_artifacts(mut self, artifacts: EventArtifacts) -> Self {
        self.artifacts = artifacts;
        self
    }
}

pub fn schema_fixture() -> EventDraft {
    EventDraft::new(
        EventPhase::Cut,
        EventKind::TransitionDenied,
        EventSource::new("punk-flow", "transition_guard"),
        EventCorrelation::new("flow_local_001")
            .with_goal_ref("work/goals/goal_add_append_only_event_log.md"),
        EventResult::new(EventStatus::Denied)
            .with_guard_code("CUT_REQUIRES_APPROVED_CONTRACT")
            .with_error_code("E_FLOW_ILLEGAL_TRANSITION"),
    )
    .with_transition(
        FlowTransitionRef::new()
            .with_from_state("AwaitingApproval")
            .with_command("StartRun"),
    )
    .with_artifacts(EventArtifacts::new().with_ref("work/goals/goal_add_append_only_event_log.md"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventRecord {
    pub schema_version: &'static str,
    pub event_id: String,
    pub sequence: u64,
    pub phase: EventPhase,
    pub kind: EventKind,
    pub source: EventSource,
    pub correlation: EventCorrelation,
    pub transition: FlowTransitionRef,
    pub result: EventResult,
    pub artifacts: EventArtifacts,
}

impl EventRecord {
    fn from_draft(sequence: u64, draft: EventDraft) -> Self {
        Self {
            schema_version: EVENT_SCHEMA_VERSION,
            event_id: format!("evt_{sequence:016}"),
            sequence,
            phase: draft.phase,
            kind: draft.kind,
            source: draft.source,
            correlation: draft.correlation,
            transition: draft.transition,
            result: draft.result,
            artifacts: draft.artifacts,
        }
    }
}

#[derive(Debug)]
pub enum EventLogError {
    Io(io::Error),
    SequenceOverflow,
}

impl From<io::Error> for EventLogError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

#[derive(Debug)]
pub struct AppendOnlyEventLog<W> {
    sink: W,
    events: Vec<EventRecord>,
    next_sequence: u64,
}

impl<W: Write> AppendOnlyEventLog<W> {
    pub fn new(sink: W) -> Self {
        Self {
            sink,
            events: Vec::new(),
            next_sequence: 1,
        }
    }

    pub fn append(&mut self, draft: EventDraft) -> Result<EventRecord, EventLogError> {
        let sequence = self.next_sequence;
        let record = EventRecord::from_draft(sequence, draft);
        let line = serialize_event_record(&record);

        self.sink.write_all(line.as_bytes())?;
        self.sink.write_all(b"\n")?;
        self.events.push(record.clone());
        self.next_sequence = self
            .next_sequence
            .checked_add(1)
            .ok_or(EventLogError::SequenceOverflow)?;
        Ok(record)
    }

    pub fn events(&self) -> &[EventRecord] {
        &self.events
    }

    pub fn into_inner(self) -> W {
        self.sink
    }
}

pub type MemoryEventLog = AppendOnlyEventLog<Cursor<Vec<u8>>>;

impl Default for MemoryEventLog {
    fn default() -> Self {
        Self::new(Cursor::new(Vec::new()))
    }
}

impl MemoryEventLog {
    pub fn into_string(self) -> String {
        String::from_utf8(self.into_inner().into_inner())
            .expect("event log output should stay valid UTF-8")
    }
}

pub fn create_new_file_event_log(path: impl AsRef<Path>) -> io::Result<AppendOnlyEventLog<File>> {
    let file = OpenOptions::new()
        .create_new(true)
        .append(true)
        .write(true)
        .open(path)?;
    Ok(AppendOnlyEventLog::new(file))
}

pub fn serialize_event_record(record: &EventRecord) -> String {
    let mut output = String::new();
    output.push('{');
    push_json_field(&mut output, "schema_version", record.schema_version);
    output.push(',');
    push_json_field(&mut output, "event_id", &record.event_id);
    output.push(',');
    output.push_str("\"sequence\":");
    output.push_str(&record.sequence.to_string());
    output.push(',');
    push_json_field(&mut output, "phase", record.phase.as_str());
    output.push(',');
    push_json_field(&mut output, "kind", record.kind.as_str());
    output.push(',');
    output.push_str("\"source\":{");
    push_json_field(&mut output, "crate", &record.source.crate_name);
    output.push(',');
    push_json_field(&mut output, "component", &record.source.component);
    output.push('}');
    output.push(',');
    output.push_str("\"correlation\":{");
    push_json_field(&mut output, "flow_id", &record.correlation.flow_id);
    output.push(',');
    push_optional_json_field(
        &mut output,
        "goal_ref",
        record.correlation.goal_ref.as_deref(),
    );
    output.push('}');
    output.push(',');
    output.push_str("\"transition\":{");
    push_optional_json_field(
        &mut output,
        "from_state",
        record.transition.from_state.as_deref(),
    );
    output.push(',');
    push_optional_json_field(&mut output, "command", record.transition.command.as_deref());
    output.push(',');
    push_optional_json_field(
        &mut output,
        "to_state",
        record.transition.to_state.as_deref(),
    );
    output.push('}');
    output.push(',');
    output.push_str("\"result\":{");
    push_json_field(&mut output, "status", record.result.status.as_str());
    output.push(',');
    push_optional_json_field(
        &mut output,
        "guard_code",
        record.result.guard_code.as_deref(),
    );
    output.push(',');
    push_optional_json_field(
        &mut output,
        "error_code",
        record.result.error_code.as_deref(),
    );
    output.push('}');
    output.push(',');
    output.push_str("\"artifacts\":{");
    output.push_str("\"refs\":[");
    for (index, artifact_ref) in record.artifacts.refs.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_json_string(&mut output, artifact_ref);
    }
    output.push_str("]}");
    output.push('}');
    output
}

fn push_json_field(output: &mut String, key: &str, value: &str) {
    push_json_string(output, key);
    output.push(':');
    push_json_string(output, value);
}

fn push_optional_json_field(output: &mut String, key: &str, value: Option<&str>) {
    push_json_string(output, key);
    output.push(':');
    match value {
        Some(value) => push_json_string(output, value),
        None => output.push_str("null"),
    }
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
            ch if ch.is_control() => output.push_str(&format!("\\u{:04x}", ch as u32)),
            ch => output.push(ch),
        }
    }
    output.push('"');
}

#[cfg(test)]
mod tests {
    use super::{
        create_new_file_event_log, schema_fixture, AppendOnlyEventLog, EventArtifacts,
        EventCorrelation, EventDraft, EventKind, EventPhase, EventResult, EventSource, EventStatus,
        FlowTransitionRef, MemoryEventLog,
    };
    use std::fs;
    use std::io::Cursor;
    use std::path::PathBuf;
    use std::process;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn appending_assigns_monotonic_sequence_and_stable_ids() {
        let mut log = MemoryEventLog::default();

        let first = log
            .append(schema_fixture())
            .expect("first append should succeed");
        let second = log
            .append(
                EventDraft::new(
                    EventPhase::Cut,
                    EventKind::TransitionCommitted,
                    EventSource::new("punk-flow", "transition_guard"),
                    EventCorrelation::new("flow_local_001")
                        .with_goal_ref("work/goals/goal_add_append_only_event_log.md"),
                    EventResult::new(EventStatus::Succeeded),
                )
                .with_transition(
                    FlowTransitionRef::new()
                        .with_from_state("AwaitingApproval")
                        .with_command("Approve")
                        .with_to_state("Approved"),
                )
                .with_artifacts(EventArtifacts::new().with_ref(".tmp/events/test-log.jsonl")),
            )
            .expect("second append should succeed");

        assert_eq!(first.sequence, 1);
        assert_eq!(first.event_id, "evt_0000000000000001");
        assert_eq!(second.sequence, 2);
        assert_eq!(second.event_id, "evt_0000000000000002");
        assert_eq!(log.events()[0], first);
        assert_eq!(log.events()[1], second);
    }

    #[test]
    fn append_only_log_preserves_previous_events_and_serializes_deterministically() {
        let mut log = MemoryEventLog::default();
        let first = log
            .append(schema_fixture())
            .expect("first append should succeed");
        let second = log
            .append(
                EventDraft::new(
                    EventPhase::Cut,
                    EventKind::GuardEvaluated,
                    EventSource::new("punk-flow", "transition_guard"),
                    EventCorrelation::new("flow_local_001"),
                    EventResult::new(EventStatus::Succeeded)
                        .with_guard_code("CUT_ALLOWED_APPROVED_CONTRACT"),
                )
                .with_transition(FlowTransitionRef::new().with_command("Approve")),
            )
            .expect("second append should succeed");

        assert_eq!(log.events().len(), 2);
        assert_eq!(log.events()[0], first);
        assert_eq!(log.events()[1], second);

        let rendered = log.into_string();
        let expected = concat!(
            "{\"schema_version\":\"punk.event.v0.1\",\"event_id\":\"evt_0000000000000001\",\"sequence\":1,\"phase\":\"cut\",\"kind\":\"transition_denied\",\"source\":{\"crate\":\"punk-flow\",\"component\":\"transition_guard\"},\"correlation\":{\"flow_id\":\"flow_local_001\",\"goal_ref\":\"work/goals/goal_add_append_only_event_log.md\"},\"transition\":{\"from_state\":\"AwaitingApproval\",\"command\":\"StartRun\",\"to_state\":null},\"result\":{\"status\":\"denied\",\"guard_code\":\"CUT_REQUIRES_APPROVED_CONTRACT\",\"error_code\":\"E_FLOW_ILLEGAL_TRANSITION\"},\"artifacts\":{\"refs\":[\"work/goals/goal_add_append_only_event_log.md\"]}}\n",
            "{\"schema_version\":\"punk.event.v0.1\",\"event_id\":\"evt_0000000000000002\",\"sequence\":2,\"phase\":\"cut\",\"kind\":\"guard_evaluated\",\"source\":{\"crate\":\"punk-flow\",\"component\":\"transition_guard\"},\"correlation\":{\"flow_id\":\"flow_local_001\",\"goal_ref\":null},\"transition\":{\"from_state\":null,\"command\":\"Approve\",\"to_state\":null},\"result\":{\"status\":\"succeeded\",\"guard_code\":\"CUT_ALLOWED_APPROVED_CONTRACT\",\"error_code\":null},\"artifacts\":{\"refs\":[]}}\n"
        );

        assert_eq!(rendered, expected);
    }

    #[test]
    fn deterministic_jsonl_serialization_escapes_strings() {
        let mut log = MemoryEventLog::default();
        let record = log
            .append(
                EventDraft::new(
                    EventPhase::Cut,
                    EventKind::GuardEvaluated,
                    EventSource::new("punk-events", "json_writer"),
                    EventCorrelation::new("flow_\"quoted\"")
                        .with_goal_ref("work/goals/path\\with\\slashes.md"),
                    EventResult::new(EventStatus::Failed)
                        .with_guard_code("guard \"quoted\"")
                        .with_error_code("line\nbreak"),
                )
                .with_transition(
                    FlowTransitionRef::new()
                        .with_from_state("state \"one\"")
                        .with_command("path\\to\\command")
                        .with_to_state("line\nstate"),
                )
                .with_artifacts(
                    EventArtifacts::new()
                        .with_ref("line\nbreak")
                        .with_ref("quote\"and\\slash"),
                ),
            )
            .expect("append should succeed");

        let rendered = log.into_string();

        assert_eq!(record.sequence, 1);
        assert!(rendered.contains("\"flow_id\":\"flow_\\\"quoted\\\"\""));
        assert!(rendered.contains("\"goal_ref\":\"work/goals/path\\\\with\\\\slashes.md\""));
        assert!(rendered.contains("\"guard_code\":\"guard \\\"quoted\\\"\""));
        assert!(rendered.contains("\"error_code\":\"line\\nbreak\""));
        assert!(rendered.contains("\"command\":\"path\\\\to\\\\command\""));
        assert!(rendered.contains("\"to_state\":\"line\\nstate\""));
        assert!(rendered.contains("\"refs\":[\"line\\nbreak\",\"quote\\\"and\\\\slash\"]"));
    }

    #[test]
    fn file_backed_writer_leaves_deterministic_test_evidence() {
        let temp_path = unique_temp_path();
        fs::create_dir_all(&temp_path).expect("temp dir should be created");
        let file_path = temp_path.join("events.jsonl");

        {
            let mut log =
                create_new_file_event_log(&file_path).expect("file log should be created");
            let record = log.append(schema_fixture()).expect("append should succeed");
            assert_eq!(record.event_id, "evt_0000000000000001");
            assert_eq!(record.kind, EventKind::TransitionDenied);
        }

        let rendered = fs::read_to_string(&file_path).expect("event file should be readable");
        assert_eq!(rendered.lines().count(), 1);
        assert!(rendered.contains("\"event_id\":\"evt_0000000000000001\""));
        assert!(rendered.contains("\"kind\":\"transition_denied\""));
        assert!(!rendered.contains("decision_id"));

        fs::remove_dir_all(&temp_path).expect("temp dir should be removed");
    }

    #[test]
    fn file_log_creation_rejects_existing_paths_instead_of_truncating() {
        let temp_path = unique_temp_path();
        fs::create_dir_all(&temp_path).expect("temp dir should be created");
        let file_path = temp_path.join("events.jsonl");
        fs::write(&file_path, b"existing\n").expect("seed file should be written");

        let error =
            create_new_file_event_log(&file_path).expect_err("existing file should be rejected");
        assert_eq!(error.kind(), std::io::ErrorKind::AlreadyExists);

        fs::remove_dir_all(&temp_path).expect("temp dir should be removed");
    }

    #[test]
    fn explicit_memory_sink_can_be_used_without_runtime_paths() {
        let mut log = AppendOnlyEventLog::new(Cursor::new(Vec::new()));
        let event = log.append(schema_fixture()).expect("append should succeed");

        assert_eq!(event.correlation.flow_id, "flow_local_001");
        assert_eq!(event.result.status, EventStatus::Denied);
        assert_eq!(
            event.result.error_code.as_deref(),
            Some("E_FLOW_ILLEGAL_TRANSITION")
        );
        assert_eq!(log.events().len(), 1);
    }

    fn unique_temp_path() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("punk-events-tests-{}-{}", process::id(), unique))
    }
}
