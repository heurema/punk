# Security policy

Thanks for helping keep `punk` and its users safe.

## Supported versions

Until `punk` starts publishing a formal release/support matrix, the default
supported line for security fixes is the latest `main` branch state.

Older commits and experimental branches may not receive patches.

## Reporting a vulnerability

Please **do not** post full vulnerability details in a public GitHub issue.

Preferred path:

1. Use any private maintainer contact route currently published on
   [punks.run](https://punks.run).
2. Include the affected path, reproduction steps, impact, and any proposed fix.

If no private contact route is currently available, open a **minimal** GitHub
issue requesting a private handoff, but do **not** include exploit details,
secrets, or full proof-of-concept material there.

## What to expect

We will make a good-faith effort to:

- acknowledge the report in a reasonable time;
- assess the affected surface and severity;
- coordinate remediation and disclosure timing where appropriate.

## Scope reminders

Security-sensitive repo surfaces include:
- transition guards and proof-bearing workflow semantics;
- `scripts/check_*` policy and governance enforcement;
- any future trust-bearing storage, gate, or proof surfaces;
- any future external adapter or provider boundary that changes what data leaves the repo.

## Out of scope

This file is not a bug bounty program and does not create any right to
compensation.
