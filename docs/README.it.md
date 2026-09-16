<!--
translation-of: docs/README.md
locale: it
source-revision: d50e9029e372521bd00a7b6c8cbc29e234c80009
-->

# Documentazione FerrumWeave

[English](README.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · **Italiano** · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [简体中文](README.zh-Hans.md) · [日本語](README.ja.md)

[← README del progetto](../README.it.md)

La documentazione di FerrumWeave è organizzata secondo il percorso reale del lettore. I documenti inglesi senza suffisso locale restano canonici; quando esiste una versione localizzata, la navigazione dovrebbe rimanere nella stessa lingua quando possibile.

> La documentazione è mantenuta in più lingue perché l’interoperabilità riguarda anche le persone.

## Inizia qui

- [Primi passi](getting-started.it.md) — esperienza target di installazione e primo progetto per R10 / 0.1-alpha.
- [Piano di release dei template](roadmap/template-release-plan.it.md) — famiglie di template .NET previste e significato di “supportato”.
- [README del progetto](../README.it.md) — visione, architettura, stato attuale e accesso alla community.
- [Sito del progetto](https://chicodotnet.github.io/FerrumWeave/it/) — panoramica pubblica e developer experience target.

## Architettura

- [Struttura del repository](architecture/repository-layout.it.md) — confini di ownership e struttura del repository.
- [ADR 0004 — Rust come linguaggio dei template .NET](architecture/adr/0004-r10-rust-as-dotnet-template-language.it.md) — perché il contratto pubblico è `dotnet new <template> -lang Rust`.
- [Architecture Decision Records](architecture/adr/) — decisioni storiche e relative motivazioni. *(inglese)*

## Capability e compatibilità

- [Capability roadmap](roadmap/README.md) *(inglese)*
- [Compatibility](compatibility/README.md) *(inglese)*
- [CTS scalar mappings](compatibility/r04-cts-scalar-mappings.md) *(inglese)*
- [Coverage policy](quality/coverage-policy.md) *(inglese)*

## Community e policy del progetto

- [Contributing](../CONTRIBUTING.md) *(inglese)*
- [Governance](../GOVERNANCE.md) *(inglese)*
- [Support](../SUPPORT.md) *(inglese)*
- [Security](../SECURITY.md) *(inglese)*
- [Code of Conduct](../CODE_OF_CONDUCT.md) *(inglese)*
- [DCO](../DCO.md) *(inglese)*

## Internazionalizzazione

- [Documentation internationalization](i18n.md) — locale, provenance, regole semantiche e politica di aggiornamento. *(inglese per ora)*

Locale pubbliche attuali:

`en` · `de` · `es` · `fr` · `it` · `pt-BR` · `ru` · `zh-Hans` · `ja`

Non tutti i documenti specialistici o storici sono già tradotti. Se manca una versione localizzata, la fonte canonica inglese è il fallback.

## Modello di navigazione

```text
README del progetto
  → Home della documentazione
    → Sezione / documento
```

I documenti con metadata `doc-id` ricevono link generati verso questa home della documentazione e verso il README del progetto nella stessa locale. `ferrumweave-docgraph` genera e certifica anche backlink localizzati “Cosa punta qui” dai link Markdown scritti manualmente; le regioni generate sono committed e Documentation graph CI ne verifica l’aggiornamento.
