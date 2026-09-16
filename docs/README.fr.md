<!--
translation-of: docs/README.md
locale: fr
source-revision: d50e9029e372521bd00a7b6c8cbc29e234c80009
-->

# Documentation FerrumWeave

[English](README.md) · [Deutsch](README.de.md) · [Español](README.es.md) · **Français** · [Italiano](README.it.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [简体中文](README.zh-Hans.md) · [日本語](README.ja.md)

[← README du projet](../README.fr.md)

La documentation FerrumWeave est organisée selon le parcours réel du lecteur. Les documents anglais sans suffixe de locale restent canoniques ; lorsqu’une version localisée existe, la navigation doit rester dans la même langue autant que possible.

> La documentation est maintenue dans plusieurs langues parce que l’interopérabilité concerne aussi les personnes.

## Commencer ici

- [Premiers pas](getting-started.fr.md) — expérience cible d’installation et de premier projet pour R10 / 0.1-alpha.
- [Plan de publication des templates](roadmap/template-release-plan.fr.md) — familles de templates .NET visées et définition de « supporté ».
- [README du projet](../README.fr.md) — vision, architecture, état actuel et entrée communautaire.
- [Site du projet](https://chicodotnet.github.io/FerrumWeave/fr/) — vue publique et expérience développeur cible.

## Architecture

- [Structure du dépôt](architecture/repository-layout.fr.md) — frontières d’ownership et structure du dépôt.
- [ADR 0004 — Rust comme langage de template .NET](architecture/adr/0004-r10-rust-as-dotnet-template-language.fr.md) — pourquoi le contrat public est `dotnet new <template> -lang Rust`.
- [Architecture Decision Records](architecture/adr/) — décisions historiques et leur justification. *(anglais)*

## Capacité et compatibilité

- [Capability roadmap](roadmap/README.md) *(anglais)*
- [Compatibility](compatibility/README.md) *(anglais)*
- [CTS scalar mappings](compatibility/r04-cts-scalar-mappings.md) *(anglais)*
- [Coverage policy](quality/coverage-policy.md) *(anglais)*

## Communauté et politiques du projet

- [Contributing](../CONTRIBUTING.md) *(anglais)*
- [Governance](../GOVERNANCE.md) *(anglais)*
- [Support](../SUPPORT.md) *(anglais)*
- [Security](../SECURITY.md) *(anglais)*
- [Code of Conduct](../CODE_OF_CONDUCT.md) *(anglais)*
- [DCO](../DCO.md) *(anglais)*

## Internationalisation

- [Documentation internationalization](i18n.md) — locales, provenance, règles sémantiques et politique de fraîcheur. *(anglais pour l’instant)*

Locales publiques actuelles :

`en` · `de` · `es` · `fr` · `it` · `pt-BR` · `ru` · `zh-Hans` · `ja`

Tous les documents spécialisés ou historiques ne sont pas encore traduits. En l’absence de version localisée, la source canonique anglaise sert de fallback.

## Modèle de navigation

```text
README du projet
  → Accueil de la documentation
    → Section / document
```

Les documents portant des métadonnées `doc-id` reçoivent des liens générés vers cet accueil documentaire et vers le README du projet dans la même locale. `ferrumweave-docgraph` génère et certifie également des backlinks localisés « Pages liées ici » à partir des liens Markdown écrits par les humains ; les régions générées sont committed et vérifiées par Documentation graph CI.
