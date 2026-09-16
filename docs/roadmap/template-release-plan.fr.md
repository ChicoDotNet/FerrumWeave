<!--
translation-of: docs/roadmap/template-release-plan.md
locale: fr
source-revision: 4ed0f1c8df9606bdd7b1135f186ef4bdd710c77a
-->

# Plan de publication des templates FerrumWeave

[English](template-release-plan.md) · [Deutsch](template-release-plan.de.md) · [Español](template-release-plan.es.md) · **Français** · [Italiano](template-release-plan.it.md) · [Português (Brasil)](template-release-plan.pt-BR.md) · [Русский](template-release-plan.ru.md) · [简体中文](template-release-plan.zh-Hans.md) · [日本語](template-release-plan.ja.md)

La roadmap d’expérience développeur de FerrumWeave considère Rust comme un **langage de projet .NET**, et non comme un type de projet distinct.

L’UX principale des templates est :

```console
dotnet new <template> -lang Rust
```

Par exemple :

```console
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

L’ancienne forme `dotnet new rust` n’est pas l’expérience cible. Une application console reste un projet `console`; FerrumWeave ajoute Rust comme choix de langage au moteur de templates .NET.

## Ce que signifie « supporté »

Un template n’est pas considéré comme supporté simplement parce qu’il peut générer des fichiers.

Pour être annoncé comme supporté dans un prerelease FerrumWeave :

1. `dotnet new <template> -lang Rust` doit créer le `.rsproj` et le layout Rust attendus depuis un package FerrumWeave installé.
2. Le projet généré doit restaurer et construire via le workflow SDK .NET normal, sans checkout du dépôt FerrumWeave.
3. Son workflow significatif par défaut doit réussir : `dotnet run` pour les applications exécutables, `dotnet test` pour les projets de tests.
4. Le comportement doit passer causalement par `rustc -> FerrumWeave CodegenBackend -> CIL/metadata -> CoreCLR`; les anciens emitters basés sur des patterns de source ou les substituts générés en C#/VB/F# ne satisfont pas le contrat.
5. Les templates cross-platform doivent être certifiés sur Windows et Linux. WinForms et WPF sont certifiés sur Windows supporté et n’impliquent pas une exécution Linux.
6. Les limitations connues doivent être documentées explicitement.

Les samples et fixtures de conformité peuvent prouver un contrat de prerelease, mais ne satisfont pas le gate « projet réel » de 1.0.

## Séquence des versions

| Release | Étape | Templates introduits comme supportés |
| --- | --- | --- |
| `0.1` | Alpha | `console`, `classlib`, `xunit`, `nunit`, `mstest`, `web`, `webapi` |
| `0.2` | Alpha | `mvc`, `winforms` |
| `0.3` | Beta | `worker` |
| `0.4` | Beta | `wpf` |
| `0.5` | Beta | `grpc` |
| `0.6` | Beta | `blazor` |
| `1.0` | Stable | Toutes les familles engagées ont des contrats certifiés et au moins un projet réel réussi |

Des suffixes comme `0.1.0-alpha.1` ou `0.5.0-beta.1` restent normaux ; cette table définit les frontières de capacité, pas le nombre exact de prereleases.

## 0.1 alpha — expérience de base comme langage .NET

```console
dotnet new console  -lang Rust
dotnet new classlib -lang Rust
dotnet new xunit    -lang Rust
dotnet new nunit    -lang Rust
dotnet new mstest   -lang Rust
dotnet new web      -lang Rust
dotnet new webapi   -lang Rust
```

Le release doit au minimum prouver : un point d’entrée exécutable et une sortie console ; une class library managée consommable depuis un autre projet .NET ; découverte et exécution via xUnit, NUnit et MSTest ; un host ASP.NET Core lancé par `dotnet run` ; un chemin HTTP request/response minimal pour `web` et `webapi` ; restore/build/run/test, `ProjectReference` et NuGet normaux ; installation et utilisation depuis un répertoire externe propre.

## 0.2 alpha — frameworks applicatifs

```console
dotnet new mvc      -lang Rust
dotnet new winforms -lang Rust
```

`mvc` doit prouver un véritable chemin de requête MVC. `winforms` doit prouver un vrai cycle de vie Windows Forms et l’interaction d’événements/delegates supportée sous Windows.

## 0.3 beta — services d’arrière-plan

```console
dotnet new worker -lang Rust
```

Le template Worker doit exercer le .NET Generic Host et un vrai cycle de vie de background service, pas une console app renommée.

## 0.4 beta — WPF

```console
dotnet new wpf -lang Rust
```

WPF constitue une frontière propre à cause de XAML, des targets WindowsDesktop MSBuild, des types/code-behind générés, du démarrage et du modèle d’événements.

## 0.5 beta — gRPC

```console
dotnet new grpc -lang Rust
```

Le contrat couvre le chemin de build protobuf/gRPC généré et au moins un vrai aller-retour request/response.

## 0.6 beta — Blazor

Ajoute la surface de templates Blazor supportée par la version cible du SDK .NET. Le short name exact suit le SDK contemporain au lieu de figer une commande obsolète. Le contrat doit prouver un composant réellement rendu et l’intégration Razor/Blazor requise.

## Gate stable 1.0 — preuves issues de projets réels

FerrumWeave n’atteint pas 1.0 uniquement parce que toutes les suites de conformité sont vertes.

Avant `1.0.0`, **chaque famille de templates engagée doit avoir au moins un vrai projet utilisant FerrumWeave avec succès**.

Un vrai projet :

- n’est ni fixture du compilateur, ni test de conformité, ni smoke test de template, ni sample purement documentaire ;
- a un objectif réel d’application ou de bibliothèque ;
- utilise la famille de templates Rust correspondante dans sa structure normale ;
- peut restaurer/construire et, si nécessaire, s’exécuter ou se tester de façon reproductible ;
- exerce le template de façon représentative ;
- documente les limitations propres à FerrumWeave au lieu de les masquer.

Le projet peut être open source ou prouvé en privé, mais il doit fournir assez de preuves reproductibles pour justifier la revendication de compatibilité.

La question stable est donc à la fois :

> FerrumWeave peut-il scaffold et certifier chaque famille promise ?

et :

> Chaque famille promise a-t-elle survécu au contact d’au moins une vraie application ?

Ce n’est que lorsque les deux réponses sont oui que FerrumWeave franchit le gate `1.0.0`.

## Règle de compatibilité

```console
dotnet new console
dotnet new console -lang VB
dotnet new console -lang "F#"
dotnet new console -lang Rust
```

FerrumWeave étend l’écosystème des langages .NET ; il ne doit pas détourner ni casser les variantes existantes.
