# FerrumWeave documentation

**English** · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [简体中文](README.zh-Hans.md) · [日本語](README.ja.md)

[← Project README](../README.md)

FerrumWeave documentation is organized around the path a reader is trying to follow. English documents without a locale suffix are canonical; when a localized equivalent exists, the documentation should keep readers in the same language whenever practical.

> Documentation is maintained in multiple languages because interoperability is about people, too.

## Start here

- [Getting Started](getting-started.md) — target installation and first-project experience for the active R10 / 0.1-alpha milestone.
- [Template Release Plan](roadmap/template-release-plan.md) — which standard .NET template families FerrumWeave intends to support and what “supported” means.
- [Project README](../README.md) — project vision, architecture direction, current status, and community entry point.
- [Project website](https://chicodotnet.github.io/FerrumWeave/) — public overview and current target developer experience.

## Architecture

- [Repository layout](architecture/repository-layout.md) — ownership boundaries, repository structure, and where compiler/SDK/site work belongs.
- [ADR 0004 — Rust as a .NET template language](architecture/adr/0004-r10-rust-as-dotnet-template-language.md) — why the public template contract is `dotnet new <template> -lang Rust` rather than `dotnet new rust`.
- [Architecture Decision Records](architecture/adr/) — historical decisions and their rationale.

## Capability and compatibility

- [Capability roadmap](roadmap/README.md) — milestone evidence from R00 onward and the active R10 frontier.
- [Compatibility](compatibility/README.md) — compatibility claims and their evidence boundaries.
- [CTS scalar mappings](compatibility/r04-cts-scalar-mappings.md) — documented Rust ↔ CLR scalar mapping surface.
- [Coverage policy](quality/coverage-policy.md) — code and functional coverage expectations.

## Community and project policy

- [Contributing](../CONTRIBUTING.md) — contribution workflow, evidence expectations, and Rust/.NET design principles.
- [Governance](../GOVERNANCE.md) — how decisions are made and how stewardship can evolve.
- [Support](../SUPPORT.md) — where to ask questions and report ordinary problems.
- [Security](../SECURITY.md) — private vulnerability-reporting guidance.
- [Code of Conduct](../CODE_OF_CONDUCT.md) — collaboration standards.
- [DCO](../DCO.md) — contribution certification and sign-off policy.

## Internationalization

- [Documentation internationalization](i18n.md) — locale naming, translation provenance, semantic rules, and staleness policy.

Current public documentation locales are:

`en` · `de` · `es` · `fr` · `it` · `pt-BR` · `ru` · `zh-Hans` · `ja`

Not every specialized or historical document is translated yet. When no localized equivalent exists, the canonical English document is the fallback.

## Navigation model

Public documentation exposes a stable semantic path rather than relying on browser history:

```text
Project README
  → Documentation home
    → Section / document
```

Documents carrying `doc-id` metadata receive generated links back to this documentation home and the project README in the same locale. `ferrumweave-docgraph` also generates and certifies localized “What links here” backlinks from human-authored Markdown links; generated regions are committed and verified by Documentation graph CI.
