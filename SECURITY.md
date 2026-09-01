# Security Policy

FerrumWeave is currently **pre-alpha / architectural discovery** and has no supported production release.

That does not make security reports unimportant. Repository automation, the project website, dependency configuration, future compiler components, generated artifacts, and supply-chain behavior can all create real risk before a 1.0 release exists.

## Supported versions

There are currently no supported release versions.

Security fixes apply to the active development line as maintainers determine appropriate.

## What to report privately

Please report privately when a finding could plausibly enable:

- arbitrary code execution or command injection;
- malicious code execution through build, test, package, or CI workflows;
- credential, token, or secret exposure;
- dependency or artifact substitution;
- unsafe generated IL or metadata that creates a security boundary bypass beyond documented project limitations;
- path traversal or unintended filesystem access;
- compromised publishing or release processes;
- a vulnerability in the project website or infrastructure that affects users;
- another issue where public disclosure before a fix would create meaningful risk.

Ordinary compiler bugs, incorrect diagnostics, documentation errors, and non-sensitive compatibility problems should use normal GitHub issues once implementation exists.

## How to report

Prefer GitHub's private vulnerability reporting mechanism from the repository's **Security** area when it is available.

If private vulnerability reporting is unavailable, contact a current maintainer privately using a contact method published on that maintainer's GitHub profile.

Do **not** include exploit details, credentials, private data, or weaponizable reproduction steps in a public issue before maintainers have had a reasonable opportunity to investigate.

## A useful report includes

When practical, provide:

- the affected commit, branch, workflow, package, or component;
- the environment needed to reproduce the issue;
- a minimal reproduction;
- expected vs. observed behavior;
- security impact and realistic attack preconditions;
- whether the issue is already public elsewhere;
- any known mitigations.

## Response expectations

FerrumWeave does not yet have a staffed security team or formal response-time SLA.

Maintainers will nevertheless try to acknowledge credible private reports promptly, assess severity, coordinate a fix when appropriate, and agree on responsible disclosure timing with the reporter.

## Dependencies and upstream issues

If the root cause belongs to Rust, .NET, GitHub Actions, a package dependency, or another upstream project, FerrumWeave maintainers may coordinate or redirect the report to the appropriate upstream security process while minimizing unnecessary disclosure.

## Safe research

Good-faith security research intended to improve FerrumWeave is welcome.

Please avoid accessing data that is not yours, degrading shared infrastructure, performing denial-of-service testing against public project services, or publishing a working exploit before maintainers can respond.
