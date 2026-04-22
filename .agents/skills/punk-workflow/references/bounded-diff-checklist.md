# Bounded diff checklist

Before editing:

- confirm `selected_next` or explicit user override;
- classify the task as `R0`, `R1`, `R2`, or `R3`;
- if the task is `R1+`, identify the repo-tracked or user-provided research that justifies proceeding;
- if the task is `R1+` and research is missing, stop and ask the user for Deep Research;
- read the selected goal scope and exclusions;
- name the single logical diff you are about to make;
- list the required checks for that diff.
- if several meaningful diffs have landed or repeated process corrections happened, consider whether a Work Ledger Review is due.

Before staging:

- verify no unrelated cleanup leaked into the diff;
- verify repo-tracked refs are repo-relative;
- verify `.punk/` and other out-of-scope surfaces were not touched;
- verify the selected goal, `work/STATUS.md`, and a report were updated for meaningful work;
- verify the selected goal has a `research_gate` block with the right classification.

Before finishing:

- run `python3 scripts/check_research_gate.py`;
- run `python3 scripts/check_work_ledger.py`;
- run `cargo check --workspace` if Rust changed;
- run docs governance checks if docs, reports, or work-ledger artifacts changed;
- state what remains out of scope;
- make the next selected goal explicit.
