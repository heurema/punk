export const externalLinks = {
  repo: 'https://github.com/heurema/punk',
  roadmap: 'https://github.com/heurema/punk/blob/main/docs/product/ROADMAP.md',
  lawsDoc: 'https://github.com/heurema/punk/blob/main/docs/product/PUNK-LAWS.md',
  publicBuild: 'https://github.com/heurema/punk/tree/main/public',
  publicStories: 'https://github.com/heurema/punk/tree/main/public/stories',
  issues: 'https://github.com/heurema/punk/issues',
} as const;

export const navLinks = [
  { label: 'laws', href: '#laws' },
  { label: 'how', href: '#how' },
  { label: 'build', href: '#build' },
  { label: 'modules', href: '#modules' },
  { label: 'journal', href: '/journal' },
] as const;

export const heroStatus = [
  { label: 'core-first', tone: 'success' },
  { label: 'personal experiment', tone: 'muted' },
  { label: 'may break', tone: 'muted' },
  { label: 'modules later', tone: 'muted' },
  { label: 'building in public', tone: 'accent' },
] as const;

export const kernelFlow = [
  { key: 'goal', tone: 'muted' },
  { key: 'contract', tone: 'accent' },
  { key: 'run', tone: 'warn' },
  { key: 'gate', tone: 'success' },
  { key: 'proof', tone: 'accent' },
] as const;

export const driftCards = [
  { key: 'chat', title: '“we said tuesday?”', meta: 'slack · 3 threads' },
  { key: 'task', title: 'ticket AUTH-42 · stale 11d', meta: 'jira · unowned' },
  { key: 'diff', title: '+382 −190 across 14 files', meta: 'PR #219 · in review' },
  { key: 'memory', title: 'lost context from last session', meta: 'nowhere · evaporated' },
  { key: 'docs', title: 'ADR-007 contradicts ADR-004', meta: 'docs/ · unresolved' },
] as const;

export const laws = [
  { key: '01', title: 'Contract first', body: 'Nothing runs until the spec is hashed, signed, and scope-bound.' },
  { key: '02', title: 'Flow before features', body: 'Harden the work loop before you decorate it. Kernels before cosmetics.' },
  { key: '03', title: 'Eval before expansion', body: 'A capability ships the gate first. No eval, no merge.' },
  { key: '04', title: 'Gate writes the decision', body: 'Deterministic checks decide accept/reject. Humans review the exceptions.' },
  { key: '05', title: 'Proof before acceptance', body: 'Every accepted run emits a replayable receipt: spec + seed + diff + checks.' },
  { key: '06', title: 'Research before decisions', body: 'Major moves ride on written memos. ADRs are mandatory, not optional.' },
  { key: '07', title: 'Memory from day zero', body: 'Context is a first-class, append-only artifact. It outlives people.' },
  { key: '08', title: 'Public build from day zero', body: 'The repo is the source of truth. Work happens where strangers can read it.' },
] as const;

export const lifecycleSteps = [
  { key: 'goal', subtitle: 'plain-markdown promise', tone: 'muted' },
  { key: 'contract', subtitle: 'hashed · signed · scoped', tone: 'accent' },
  { key: 'run', subtitle: 'agents on a leash', tone: 'warn' },
  { key: 'receipt', subtitle: 'tool calls · diff · seed', tone: 'soft' },
  { key: 'gate', subtitle: 'deterministic evals', tone: 'success' },
  { key: 'proof', subtitle: 'replayable artifact', tone: 'accent' },
  { key: 'memory', subtitle: 'append to the kernel', tone: 'info' },
] as const;

export const moduleTimeline = [
  { key: '01', name: 'kernel', state: 'compiling', tone: 'success', meta: 'now · v0.0.1-pre' },
  { key: '02', name: 'devpunk', state: 'staged', tone: 'muted', meta: 'after kernel freeze' },
  { key: '03', name: 'pubpunk', state: 'staged', tone: 'faded', meta: 'after devpunk' },
  { key: '04', name: '—', state: 'open', tone: 'faded', meta: 'rfc window' },
] as const;

export const devpunkFacts = [
  ['grants', 'fs:rw · scoped'],
  ['runtime', 'local · bring-your-own model'],
  ['output', 'signed receipt · replayable'],
  ['intent', 'merge only on gate-pass'],
] as const;

export const pubpunkNotes = [
  'weekly digest of the build',
  'signed public receipts',
  'read-only · no telemetry',
] as const;

export const guideAnswers = [
  'what a law means',
  'why a law exists',
  'what is staged next',
  'which ADR to read',
] as const;

export const guideRefusals = [
  'generic coding help',
  'roadmap dates',
  'opinions on other tools',
  'anything off-contract',
] as const;

export const guideTranscript = [
  { who: 'you', text: 'what is law 04?' },
  {
    who: 'guide',
    cite: 'cites § laws/04',
    tone: 'ok',
    text: 'Gate writes the decision. Runs land on pass/fail from a deterministic eval. Humans only review exceptions.',
  },
  { who: 'you', text: 'write me a React login form.' },
  {
    who: 'guide',
    cite: 'out of scope',
    tone: 'warn',
    text: 'Not my contract. For code, open a goal, sign a contract, and run an agent under it.',
  },
  { who: 'you', text: 'when does pubpunk ship?' },
  {
    who: 'guide',
    cite: 'cites § ADR-007',
    tone: 'ok',
    text: 'After the kernel freezes. No date. Modules are explicitly a later problem.',
  },
] as const;

export const footerColumns = [
  {
    key: 'kernel',
    links: [
      { label: 'laws', href: '#laws' },
      { label: 'how it works', href: '#how' },
      { label: 'build order', href: '#build' },
      { label: 'modules', href: '#modules' },
    ],
  },
  {
    key: 'repo',
    links: [
      { label: 'open the repo', href: externalLinks.repo },
      { label: 'public build artifacts', href: externalLinks.publicBuild },
      { label: 'roadmap', href: externalLinks.roadmap },
      { label: 'punk laws doc', href: externalLinks.lawsDoc },
    ],
  },
  {
    key: 'participate',
    links: [
      { label: 'open issues', href: externalLinks.issues },
      { label: 'propose a module', href: externalLinks.issues },
      { label: 'follow stories', href: externalLinks.publicStories },
    ],
  },
] as const;

export const finalCtaCards = [
  {
    tag: 'primary',
    title: 'Follow the build',
    subtitle: 'repo-tracked public artifacts',
    href: externalLinks.publicBuild,
    cta: 'open public artifacts ↗',
    tone: 'accent',
  },
  {
    tag: 'secondary',
    title: 'Read the roadmap',
    subtitle: 'core first, modules later',
    href: externalLinks.roadmap,
    cta: 'open roadmap ↗',
    tone: 'default',
  },
  {
    tag: 'secondary',
    title: 'Open the repo',
    subtitle: 'main · v0.0.1-pre',
    href: externalLinks.repo,
    cta: 'clone / inspect ↗',
    tone: 'default',
  },
] as const;
