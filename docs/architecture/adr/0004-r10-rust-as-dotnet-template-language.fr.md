<!--
doc-id: architecture.adr.0004-rust-as-dotnet-template-language
locale: fr
translation-of: docs/architecture/adr/0004-r10-rust-as-dotnet-template-language.md
source-revision: 0059e447c7b43940ac034f423b863d598f24b309
-->

<!-- ferrumweave-nav:start -->
[← Documentation](../../README.fr.md) · [README du projet](../../../README.fr.md)
<!-- ferrumweave-nav:end -->

# ADR 0004 — Enregistrer Rust comme langage de template .NET

[English](0004-r10-rust-as-dotnet-template-language.md) · [Deutsch](0004-r10-rust-as-dotnet-template-language.de.md) · [Español](0004-r10-rust-as-dotnet-template-language.es.md) · **Français** · [Italiano](0004-r10-rust-as-dotnet-template-language.it.md) · [Português (Brasil)](0004-r10-rust-as-dotnet-template-language.pt-BR.md) · [Русский](0004-r10-rust-as-dotnet-template-language.ru.md) · [简体中文](0004-r10-rust-as-dotnet-template-language.zh-Hans.md) · [日本語](0004-r10-rust-as-dotnet-template-language.ja.md)

- Statut : Accepté pour R10 et la roadmap prerelease
- Date : 2026-09-15
- Milestone : R10 — Developer experience / 0.1 alpha

## Contexte

FerrumWeave vise à faire de Rust un langage de première classe dans l’écosystème .NET. Le prototype R10 original exposait une commande de template dédiée :

```console
dotnet new rust
```

Cette commande est pratique techniquement, mais elle modélise Rust comme s’il s’agissait d’un type de projet. Dans le modèle de templates .NET, `console`, `classlib`, `web`, les projets de test, desktop et autres familles de workload sont des templates de projet ; le langage source est une dimension de sélection distincte.

Un développeur .NET s’attend déjà à ce que des familles de projets équivalentes se distinguent par langage plutôt que par des noms de template sans relation. FerrumWeave doit conserver ce modèle mental.

## Décision

L’UX principale des templates FerrumWeave est :

```console
dotnet new <template> -lang Rust
```

Le contrat canonique de premier usage est donc :

```console
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

`dotnet new rust` n’est pas le contrat produit public cible.

Les templates FerrumWeave doivent déclarer Rust comme langage et participer aux familles de templates .NET normales lorsque la template engine permet une intégration propre. L’implémentation doit être validée par des contrats exécutables de résolution de template, et non par l’hypothèse que des metadata similaires aux templates intégrés suffisent.

Installer FerrumWeave ne doit ni casser ni modifier le comportement normal des variantes C#, F# ou Visual Basic existantes.

## Séquence des releases

Le support des templates est introduit dans les releases de capacité suivantes :

- `0.1-alpha` : `console`, `classlib`, `xunit`, `nunit`, `mstest`, `web`, `webapi` ;
- `0.2-alpha` : `mvc`, `winforms` ;
- `0.3-beta` : `worker` ;
- `0.4-beta` : `wpf` ;
- `0.5-beta` : `grpc` ;
- `0.6-beta` : `blazor`.

Les contrats détaillés et le gate de release stable sont maintenus dans [`docs/roadmap/template-release-plan.fr.md`](../../roadmap/template-release-plan.fr.md).

## Supporté signifie exécutable

La génération d’un répertoire ne suffit pas à prouver qu’un template est supporté.

Pour une affirmation de compatibilité prerelease, le template doit au minimum :

- être scaffoldé via la distribution FerrumWeave installée ;
- restaurer et compiler sans checkout des sources FerrumWeave ;
- exécuter le workflow significatif par défaut (`dotnet run` ou `dotnet test`) ;
- atteindre le vrai product path FerrumWeave `rustc -> CodegenBackend -> CIL/metadata -> CoreCLR` ;
- passer les contrats CI/plateforme pertinents ;
- documenter honnêtement ses limitations.

Les templates propres à un framework doivent prouver le comportement framework qui les distingue d’une application console générique.

## Gate 1.0

FerrumWeave ne devient pas stable simplement parce que des tests synthétiques de conformité passent.

Avant `1.0.0`, chaque famille de templates de la séquence prerelease engagée doit avoir au moins un projet réel utilisant avec succès ce template FerrumWeave.

Un projet réel n’est ni une fixture compiler, ni un smoke test de template, ni un sample documentaire. Il doit avoir un véritable objectif hors test et une preuve reproductible adaptée à la famille de projet.

## Conséquences

Conséquences positives :

- FerrumWeave se comporte comme une extension de .NET plutôt que comme un écosystème CLI parallèle ;
- les commandes de template restent naturelles pour les développeurs C#, F# et Visual Basic ;
- la roadmap devient mesurable par contrats de famille de projets ;
- de futurs templates peuvent être ajoutés sans inventer une taxonomie de commandes propre à FerrumWeave ;
- le gate 1.0 relie les affirmations de compatibilité à des preuves réelles.

Coûts et contraintes :

- les metadata et tests du prototype R10 supposant `dotnet new rust` doivent être révisés ;
- la résolution de template doit être vérifiée contre la vraie template engine .NET ;
- certains templates framework peuvent nécessiter de nouvelles capacités compiler/runtime avant d’être honnêtement déclarés supportés ;
- les templates desktop et framework conservent leurs propres contraintes de plateforme/build.

## Validation

R10 doit ajouter des falsificateurs exécutables démontrant au minimum que :

```console
dotnet new console -lang Rust
dotnet new console -lang VB
dotnet new console -lang "F#"
```

coexistent après installation de FerrumWeave, et que le projet Rust généré utilise causalement le vrai backend produit FerrumWeave, et non un emitter legacy ou une substitution de langage source.

<!-- ferrumweave-backlinks:start -->
## Pages liées ici

- [Documentation FerrumWeave](../../README.fr.md)
<!-- ferrumweave-backlinks:end -->
