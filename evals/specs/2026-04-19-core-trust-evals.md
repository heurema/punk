# Core trust eval specs

Date: 2026-04-19
Status: proposed eval backlog

## Smoke evals

### telemetry_event_schema_fixture

Purpose: ensure emitted events match `punk.event.v0.1`.

Fixture:

- temporary Punk project
- run `punk init`
- run one valid `plot` transition

Expected result:

- `.punk/events/*.jsonl` exists
- each event has `schema_version`, `event_id`, `emitted_at`, `kind`, `phase`, `correlation`, `result`, `privacy`
- no raw user content is present

Catches:

- schema drift
- missing ids
- unsafe event fields

### flow_denied_cut_before_approved_contract

Purpose: illegal transition must be denied and audited.

Fixture:

- initialized project
- attempt `cut` without approved contract

Expected result:

- command fails or returns denied status
- `transition_denied` event written
- guard code is `CUT_REQUIRES_APPROVED_CONTRACT`
- no run receipt is written
- no gate decision is written

Catches:

- bypassed lifecycle
- missing guard evidence

### eval_machine_and_human_reports

Purpose: eval runner writes both machine-readable and human-readable outputs.

Fixture:

- smoke eval suite with one passing and one failing fixture

Expected result:

- machine JSON report exists
- Markdown report exists
- statuses, durations, case ids, and failure codes match
- event log references report ids

Catches:

- opaque evals
- non-deterministic reporting

## Regression evals

### replay_reconstructs_flow_state

Purpose: replay event log into expected flow state.

Fixture:

- sequence: init -> plot -> contract accepted -> cut denied or allowed -> gate

Expected result:

- replay state equals inspect state
- derived views can be regenerated from events and artifacts

Catches:

- non-replayable state
- derived view as second source of truth

### proofpack_hashes_match

Purpose: proofpack references valid hashes.

Fixture:

- accepted contract
- run receipt
- eval report
- gate decision
- proofpack manifest

Expected result:

- all referenced files exist
- hashes match
- decision hash matches proofpack reference
- proofpack has no raw source or prompt content

Catches:

- unverifiable proof
- stale proof refs

## Security and privacy evals

### telemetry_no_network_default

Purpose: default Punk commands must not export telemetry.

Fixture:

- run `punk init`, `punk flow inspect`, and a minimal eval under a network monitor or mock deny layer

Expected result:

- no outbound network attempt
- no export sink config created
- local event written

Catches:

- hidden analytics
- accidental remote side effects

### telemetry_redaction_paths_and_secrets

Purpose: telemetry rejects sensitive fields.

Fixture:

- project path containing a username-like segment
- env var with token-looking value
- git remote URL
- source snippet in a failing command

Expected result:

- event log contains no token, remote URL, absolute home path, raw source snippet, or env value
- events use workspace-relative refs, hashes, and error codes

Catches:

- privacy leaks
- raw-content telemetry

## Contract lifecycle evals

### contract_required_before_cut

Purpose: enforce contract-first flow.

Fixture:

- attempt `cut` before contract approval

Expected result:

- denied event
- clear reason code
- no final decision

Catches:

- lifecycle bypass

### contract_acceptance_links_to_goal

Purpose: contract must reference goal and context.

Fixture:

- create and accept contract

Expected result:

- contract id, goal ref, and hash are present
- event references contract id

Catches:

- orphan contracts

## Gate and proof evals

### only_gate_writes_decision

Purpose: no module or eval can write final decision.

Fixture:

- simulate module assessment and eval pass

Expected result:

- assessment exists
- no decision until `gate` runs
- decision source is `punk-gate`

Catches:

- module deciding
- eval acceptance without gate

### proofpack_requires_decision

Purpose: proofpack cannot imply acceptance without a gate decision.

Fixture:

- run contract and eval outputs without gate decision
- attempt proofpack generation

Expected result:

- proofpack generation is denied
- denial event is written
- no accepted proof artifact exists

Catches:

- proof before decision
- narrative-only acceptance
