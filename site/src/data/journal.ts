export type JournalEraStatus = 'legacy' | 'ancestor' | 'pivot' | 'current';
export type JournalEntryType = 'experiment' | 'failure' | 'artifact' | 'insight' | 'system';
export type JournalSurvivalStatus = 'current' | 'dropped';

export interface JournalEra {
  id: string;
  label: string;
  date: string;
  status: JournalEraStatus;
  statusLabel: string;
  color: string;
  colorBright: string;
  desc: string;
  verdict: string;
}

export interface JournalEntry {
  id: string;
  era: JournalEra['id'];
  type: JournalEntryType;
  date: string;
  seq: string;
  title: string;
  body: string;
  tags: string[];
  survived: false | 'current' | string;
  artifact?: boolean;
  artifactLabel?: string;
  artifactFragment?: string;
}

export interface JournalSurvivalRow {
  from: string;
  experiment: string;
  principle: string;
  status: JournalSurvivalStatus;
}

export const journalLinks = {
  home: '/',
  laws: '/#laws',
  build: '/#build',
  journal: '/journal',
  repo: 'https://github.com/heurema/punk',
  startHere: 'https://github.com/heurema/punk/blob/main/docs/product/START-HERE.md',
  publicArtifacts: 'https://github.com/heurema/punk/tree/main/public',
} as const;

export const journalTypeConfig = {
  experiment: { label: 'EXPERIMENT', color: '#77a9ff', glyph: '⬡' },
  failure: { label: 'FAILURE', color: '#ff6b62', glyph: '✕' },
  artifact: { label: 'ARTIFACT', color: '#ffc06b', glyph: '◈' },
  insight: { label: 'INSIGHT', color: '#3fe38d', glyph: '◎' },
  system: { label: 'SYSTEM', color: '#ff48a1', glyph: '§' },
} as const satisfies Record<JournalEntryType, { label: string; color: string; glyph: string }>;

export const journalEras: JournalEra[] = [
  {
    id: 'raw-agents',
    label: 'Raw Agents',
    date: '2024 Q1',
    status: 'legacy',
    statusLabel: 'LEGACY',
    color: '#6c6a5e',
    colorBright: '#a6a396',
    desc: 'Chaotic agent soup. No contracts, no stewards, no memory. Pure chaos research.',
    verdict: 'Mostly rubble. One survivor: the budget-cap insight.',
  },
  {
    id: 'vibe-coding',
    label: 'Vibe Coding',
    date: '2024 Q2',
    status: 'legacy',
    statusLabel: 'LEGACY',
    color: '#6c6a5e',
    colorBright: '#a6a396',
    desc: 'LLM-first velocity. Ship fast, debug never. Felt powerful until it did not.',
    verdict: 'Anti-pattern confirmed. The spec is the cure for vibes.',
  },
  {
    id: 'signum',
    label: 'Signum',
    date: '2024 Q3–Q4',
    status: 'ancestor',
    statusLabel: 'ANCESTOR',
    color: '#f7a93b',
    colorBright: '#ffc06b',
    desc: 'Signing and verification experiments. First appearance of the steward concept.',
    verdict: 'Grant taxonomy and audit trail survived intact.',
  },
  {
    id: 'specpunk',
    label: 'SpecPunk',
    date: '2025 Q1–Q2',
    status: 'ancestor',
    statusLabel: 'ANCESTOR',
    color: '#ff48a1',
    colorBright: '#ff48a1',
    desc: 'Specs as contracts. Named the aesthetic. Discovered modules were too early.',
    verdict: 'Core loop, steward model, and the name survived.',
  },
  {
    id: 'reset',
    label: 'Reset',
    date: '2025 Q3',
    status: 'pivot',
    statusLabel: 'PIVOT',
    color: '#e8433a',
    colorBright: '#ff6b62',
    desc: 'Hard scope cut. Modules, cloud sync, team dashboard — all dropped.',
    verdict: 'Discipline enforced. Smaller is the product.',
  },
  {
    id: 'punk',
    label: 'Punk',
    date: '2025 Q4 → now',
    status: 'current',
    statusLabel: 'CURRENT',
    color: '#3fe38d',
    colorBright: '#3fe38d',
    desc: 'Local-first bounded runtime. One machine, one steward, no cloud. This is it.',
    verdict: 'Experimental. Early-stage. Not production-ready. That is the point.',
  },
];

export const journalEntries: JournalEntry[] = [
  {
    id: 'e001',
    era: 'raw-agents',
    type: 'experiment',
    date: '2024-01-14',
    seq: '001',
    title: 'First multi-agent run',
    body: 'Spawned 4 agents against a single codebase with no handoff protocol. Three wrote the same function. One deleted it. No diff was mergeable.',
    tags: ['agents', 'chaos'],
    survived: false,
  },
  {
    id: 'e002',
    era: 'raw-agents',
    type: 'failure',
    date: '2024-02-03',
    seq: '002',
    title: 'The runaway loop incident',
    body: '$43 in API calls. 900 files touched across 6 hours. Zero merged. The agent had no budget. It had no stop condition. It had opinions.',
    tags: ['budget', 'loops'],
    survived: 'Per-agent budget cap',
    artifact: true,
    artifactLabel: 'INCIDENT REPORT',
    artifactFragment: '// incident-2024-02-03\ncost: $43.17\nfiles_modified: 900\nmerged: 0\nroot_cause: no_budget_ceiling\nresolution: never_again',
  },
  {
    id: 'e003',
    era: 'raw-agents',
    type: 'insight',
    date: '2024-02-20',
    seq: '003',
    title: 'Agents need contracts, not prompts',
    body: 'Every failure traced back to ambiguity. The agent did exactly what was asked. The ask was wrong. The prompt was a vibe. Conclusion: the spec is the unit of work, not the message.',
    tags: ['specs', 'contracts'],
    survived: 'Spec-as-contract',
  },
  {
    id: 'e004',
    era: 'vibe-coding',
    type: 'experiment',
    date: '2024-04-08',
    seq: '004',
    title: 'LLM-native IDE prototype',
    body: 'Replaced the file tree with a chat surface. No editor, no diff, no history. Felt fast for exactly 2 days. Code review became impossible. The diff was a lie.',
    tags: ['editor', 'ux'],
    survived: false,
  },
  {
    id: 'e005',
    era: 'vibe-coding',
    type: 'artifact',
    date: '2024-05-11',
    seq: '005',
    title: 'README: why vibes fail at scale',
    body: '3-page internal note. Core argument: “vibe” is a failure mode dressed as speed. Coherence decays without a fixed source of truth. The LLM is not the source of truth. The spec is.',
    tags: ['docs', 'internal'],
    survived: 'Spec-as-truth principle',
    artifact: true,
    artifactLabel: 'INTERNAL NOTE · 3pp',
    artifactFragment: '# why-vibes-fail.md\n\n> Coherence decays without a\n> fixed source of truth.\n>\n> The LLM is not the source.\n> The spec is.',
  },
  {
    id: 'e006',
    era: 'vibe-coding',
    type: 'failure',
    date: '2024-06-02',
    seq: '006',
    title: 'Agent autonomy without stewardship',
    body: 'Granted an agent write access to main. It passed all tests. It rewrote the auth layer. It was technically correct and completely unreviewed. Stewardship entered the vocabulary that day.',
    tags: ['stewardship', 'grants'],
    survived: 'Steward role requirement',
  },
  {
    id: 'e007',
    era: 'signum',
    type: 'system',
    date: '2024-07-18',
    seq: '007',
    title: 'Signum v0.1: signed event log',
    body: 'Every agent action produces a signed, content-addressed event. Any run is reproducible from its spec + seed. This became the audit trail you see in Punk today.',
    tags: ['audit', 'crypto'],
    survived: 'Audit log in runtime',
    artifact: true,
    artifactLabel: 'SYSTEM DESIGN',
    artifactFragment: '// signum-event v0.1\nspec_hash: sha256:3f9a...\nagent_id:   pid/4221\naction:     fs:rw\npath:       src/auth/**\nsigned_by:  @mira\ntimestamp:  2024-07-18T14:03:11Z',
  },
  {
    id: 'e008',
    era: 'signum',
    type: 'experiment',
    date: '2024-09-04',
    seq: '008',
    title: 'Steward as a named role',
    body: 'First time “steward” appeared in a spec file. A human who signs grants — not a reviewer, not an approver. The word carried a different weight. It stuck.',
    tags: ['stewardship', 'naming'],
    survived: 'Steward concept in Punk',
  },
  {
    id: 'e009',
    era: 'signum',
    type: 'failure',
    date: '2024-10-29',
    seq: '009',
    title: 'Multi-party signing was the wrong abstraction',
    body: 'Built distributed stewardship — multiple humans co-signing grants. Correct in theory. Coordination overhead destroyed the UX. Reverted to single-steward, local-first. Simplicity won.',
    tags: ['distributed', 'scope'],
    survived: false,
  },
  {
    id: 'e010',
    era: 'signum',
    type: 'artifact',
    date: '2024-11-15',
    seq: '010',
    title: 'Grant taxonomy v0.2',
    body: 'fs:rw, fs:ro, net:out, exec — these exact labels appear in Punk today. First stable grant vocabulary. Survived two full rewrites without modification.',
    tags: ['grants', 'taxonomy'],
    survived: 'fs:rw / fs:ro / net:out / exec labels',
    artifact: true,
    artifactLabel: 'GRANT VOCAB · v0.2',
    artifactFragment: '## grants\nfs:rw   on src/auth/**\nfs:ro   on docs/**\nnet:out to api.stripe.com\nexec    — denied\n\n// still the same in punk v0.4',
  },
  {
    id: 'e011',
    era: 'specpunk',
    type: 'system',
    date: '2025-01-07',
    seq: '011',
    title: 'spec.md as the single source of truth',
    body: 'Append-only markdown. Versioned. Re-read every tick by every agent. Same spec + same seed = same outcome. The core loop. Still the core loop.',
    tags: ['specs', 'determinism'],
    survived: 'The core runtime loop',
  },
  {
    id: 'e012',
    era: 'specpunk',
    type: 'experiment',
    date: '2025-02-14',
    seq: '012',
    title: 'Module marketplace concept',
    body: 'Prototype for community-authored shareable spec modules. Looked compelling. Added enormous surface area. Tabled for v1+. Modules are later. Core is now.',
    tags: ['modules', 'community'],
    survived: false,
  },
  {
    id: 'e013',
    era: 'specpunk',
    type: 'artifact',
    date: '2025-03-01',
    seq: '013',
    title: 'The porcupine mascot',
    body: 'Spines = grants. Every quill is a permission boundary. The animal does not attack. It makes the cost of bad contact obvious. A local-first creature. Does not need the cloud.',
    tags: ['brand', 'mascot'],
    survived: 'Brand and philosophy symbol',
    artifact: true,
    artifactLabel: 'BRAND NOTE',
    artifactFragment: '// mascot-rationale.md\n\nSpines = grants.\nThe porcupine does not\nattack. It makes the\ncost of bad contact\nobvious.\n\n// deny-by-default',
  },
  {
    id: 'e014',
    era: 'specpunk',
    type: 'insight',
    date: '2025-03-28',
    seq: '014',
    title: '“Punk” as a technical stance',
    body: 'Not aesthetic. Not retro. DIY ownership: you hold the keys, you read the diff before merging, you deny by default. Suspicious of platforms. The name stopped being a joke.',
    tags: ['philosophy', 'naming'],
    survived: 'Core product philosophy',
  },
  {
    id: 'e015',
    era: 'reset',
    type: 'system',
    date: '2025-07-03',
    seq: '015',
    title: 'Scope decision: core first',
    body: 'Dropped: module marketplace. Dropped: cloud sync. Dropped: team dashboard. Dropped: multi-steward. Kept: spec runtime, steward grants, local-only. One machine. One steward.',
    tags: ['scope', 'discipline'],
    survived: 'Bounded core scope',
  },
  {
    id: 'e016',
    era: 'reset',
    type: 'failure',
    date: '2025-08-11',
    seq: '016',
    title: 'Over-engineered grant DSL',
    body: 'Built a full grammar for grant expressions. Nested, composable, extensible. It was expressive and unreadable in a spec file. Replaced with the six flat labels from Signum v0.2.',
    tags: ['grants', 'simplicity'],
    survived: false,
  },
  {
    id: 'e017',
    era: 'reset',
    type: 'artifact',
    date: '2025-09-20',
    seq: '017',
    title: 'v0.3 internal README',
    body: '“Punk is not production-ready. It is a bounded runtime for experimenters who want to understand their tools. You should read the source before you run it.” — still the product framing.',
    tags: ['docs', 'scope'],
    survived: 'Product framing',
    artifact: true,
    artifactLabel: 'README · v0.3 INTERNAL',
    artifactFragment: '# punk v0.3 (internal)\n\nNot production-ready.\nBounded runtime for\nexperimenters.\n\nRead the source first.\nThen run it.\n\n// modules: later\n// core: now',
  },
  {
    id: 'e018',
    era: 'punk',
    type: 'system',
    date: '2025-10-15',
    seq: '018',
    title: 'v0.4 runtime loop',
    body: 'spec init → spec run → steward review → merge. Four commands. Local. No network at rest. The entire product in one shell session on one machine.',
    tags: ['runtime', 'ux'],
    survived: 'current',
  },
  {
    id: 'e019',
    era: 'punk',
    type: 'experiment',
    date: '2025-11-30',
    seq: '019',
    title: 'Budget enforcer at spawn',
    body: 'Per-agent token and cost ceiling enforced at spawn time — not post-hoc in billing. The $43 incident from February 2024 cannot happen. The ceiling is structural.',
    tags: ['budget', 'safety'],
    survived: 'current',
  },
  {
    id: 'e020',
    era: 'punk',
    type: 'insight',
    date: '2026-01-08',
    seq: '020',
    title: 'Runtime executes. Human writes.',
    body: 'Punk does not write specs. It runs them. The human writes the spec. The agent executes it. The steward signs grants. Roles are fixed and non-negotiable. The runtime stays in its lane.',
    tags: ['philosophy', 'roles'],
    survived: 'current',
  },
];

export const journalSurvival: JournalSurvivalRow[] = [
  { from: 'Raw Agents', experiment: 'Runaway loop · 2024-02', principle: 'Per-agent budget cap at spawn', status: 'current' },
  { from: 'Raw Agents', experiment: 'Three agents, one function', principle: 'Spec-as-contract (single source of truth)', status: 'current' },
  { from: 'Vibe Coding', experiment: 'LLM-native IDE', principle: '— dropped', status: 'dropped' },
  { from: 'Vibe Coding', experiment: 'Agent write to main, unreviewed', principle: 'Steward grant requirement', status: 'current' },
  { from: 'Signum', experiment: 'Signed event log v0.1', principle: 'Audit trail in runtime', status: 'current' },
  { from: 'Signum', experiment: 'Multi-party signing', principle: '— dropped: single-steward wins', status: 'dropped' },
  { from: 'Signum', experiment: 'Grant taxonomy v0.2', principle: 'fs:rw / fs:ro / net:out / exec labels', status: 'current' },
  { from: 'SpecPunk', experiment: 'spec.md append-only loop', principle: 'Core runtime loop', status: 'current' },
  { from: 'SpecPunk', experiment: 'Module marketplace prototype', principle: '— tabled: modules are later', status: 'dropped' },
  { from: 'SpecPunk', experiment: '“Punk” as technical stance', principle: 'DIY ownership philosophy', status: 'current' },
  { from: 'Reset', experiment: 'Hard scope cut (2025-07)', principle: 'One machine, one steward, no cloud', status: 'current' },
  { from: 'Reset', experiment: 'Grant DSL with full grammar', principle: '— dropped: flat labels only', status: 'dropped' },
];
