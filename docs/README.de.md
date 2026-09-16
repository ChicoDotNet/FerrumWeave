<!--
translation-of: docs/README.md
locale: de
source-revision: d50e9029e372521bd00a7b6c8cbc29e234c80009
-->

# FerrumWeave-Dokumentation

[English](README.md) · **Deutsch** · [Español](README.es.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [简体中文](README.zh-Hans.md) · [日本語](README.ja.md)

[← Projekt-README](../README.de.md)

Die FerrumWeave-Dokumentation folgt dem Weg, den Leserinnen und Leser tatsächlich nehmen. Englische Dokumente ohne Locale-Suffix sind kanonisch; existiert eine lokalisierte Fassung, soll die Navigation möglichst in derselben Sprache bleiben.

> Dokumentation wird in mehreren Sprachen gepflegt, weil Interoperabilität auch Menschen betrifft.

## Hier anfangen

- [Erste Schritte](getting-started.de.md) — Zielerlebnis für Installation und erstes Projekt im aktiven Meilenstein R10 / 0.1-alpha.
- [Template-Releaseplan](roadmap/template-release-plan.de.md) — unterstützte .NET-Templatefamilien und die Bedeutung von „supported“.
- [Projekt-README](../README.de.md) — Vision, Architektur, Status und Community-Einstieg.
- [Projektwebsite](https://chicodotnet.github.io/FerrumWeave/de/) — öffentliche Übersicht und Ziel-DX.

## Architektur

- [Repository-Struktur](architecture/repository-layout.de.md) — Ownership-Grenzen und Repository-Struktur.
- [ADR 0004 — Rust als .NET-Template-Sprache](architecture/adr/0004-r10-rust-as-dotnet-template-language.de.md) — warum der Vertrag `dotnet new <template> -lang Rust` lautet.
- [Architecture Decision Records](architecture/adr/) — historische Architekturentscheidungen. *(Englisch)*

## Capability und Kompatibilität

- [Capability roadmap](roadmap/README.md) *(Englisch)*
- [Compatibility](compatibility/README.md) *(Englisch)*
- [CTS scalar mappings](compatibility/r04-cts-scalar-mappings.md) *(Englisch)*
- [Coverage policy](quality/coverage-policy.md) *(Englisch)*

## Community und Projektregeln

- [Contributing](../CONTRIBUTING.md) *(Englisch)*
- [Governance](../GOVERNANCE.md) *(Englisch)*
- [Support](../SUPPORT.md) *(Englisch)*
- [Security](../SECURITY.md) *(Englisch)*
- [Code of Conduct](../CODE_OF_CONDUCT.md) *(Englisch)*
- [DCO](../DCO.md) *(Englisch)*

## Internationalisierung

- [Documentation internationalization](i18n.md) — Locale-Namen, Provenance, semantische Regeln und Staleness-Policy. *(Derzeit Englisch)*

Aktuelle öffentliche Locales:

`en` · `de` · `es` · `fr` · `it` · `pt-BR` · `ru` · `zh-Hans` · `ja`

Nicht jedes spezialisierte oder historische Dokument ist bereits übersetzt. Fehlt eine lokalisierte Fassung, ist Englisch der Fallback.

## Navigationsmodell

```text
Projekt-README
  → Dokumentations-Startseite
    → Abschnitt / Dokument
```

Dokumente mit `doc-id`-Metadaten erhalten automatisch erzeugte Links zurück zu dieser Dokumentations-Startseite und zum Projekt-README im selben Locale. `ferrumweave-docgraph` erzeugt und prüft außerdem lokalisierte „Was hierher verlinkt“-Backlinks aus von Menschen geschriebenen Markdown-Links; die generierten Bereiche werden committed und durch Documentation graph CI verifiziert.
