<!--
translation-of: docs/getting-started.md
locale: fr
source-revision: e4bcd0d4b4c4b73232823896144adf098b30758b
-->

# FerrumWeave 0.1 alpha — démarrage

[English](getting-started.md) · [Deutsch](getting-started.de.md) · [Español](getting-started.es.md) · **Français** · [Italiano](getting-started.it.md) · [Português (Brasil)](getting-started.pt-BR.md) · [Русский](getting-started.ru.md) · [简体中文](getting-started.zh-Hans.md) · [日本語](getting-started.ja.md)

> **Contrat de travail R10 :** ce document décrit l’expérience externe que FerrumWeave 0.1 alpha doit certifier avant sa publication. Les commandes qui ne sont pas encore GREEN dans CI sont des objectifs produit, pas des capacités déjà publiées.

FerrumWeave étend les templates .NET normaux avec **Rust comme choix de langage**. L’expérience principale est donc `dotnet new <template> -lang Rust`, et non `dotnet new rust`.

La séquence des releases et la définition du support sont documentées dans [`template-release-plan.fr.md`](roadmap/template-release-plan.fr.md). La décision architecturale est dans [ADR 0004](architecture/adr/0004-r10-rust-as-dotnet-template-language.md).

## Premier lancement cible de 0.1-alpha

```console
dotnet new install FerrumWeave.Sdk::0.1.0-alpha.1
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

`FerrumWeave.Sdk` est l’identité actuelle du package pour le MSBuild SDK et les templates. Aucun second package de templates non publié ne doit être nécessaire sans ADR ultérieur.

Le projet Rust doit passer par le workflow .NET normal et atteindre causalement :

```text
Rust source
  -> rustc
  -> FerrumWeave CodegenBackend
  -> CIL / CLR metadata
  -> CoreCLR
```

Le contrat doit fonctionner depuis un répertoire externe propre, sans checkout du dépôt FerrumWeave.

## Surface 0.1-alpha

```console
dotnet new console  -lang Rust
dotnet new classlib -lang Rust
dotnet new xunit    -lang Rust
dotnet new nunit    -lang Rust
dotnet new mstest   -lang Rust
dotnet new web      -lang Rust
dotnet new webapi   -lang Rust
```

Un template n’est pas supporté simplement parce qu’il génère des fichiers : il doit restaurer, construire et exécuter son workflow significatif via le vrai backend FerrumWeave.

## FerrumWeave MSBuild SDK

```xml
<Project Sdk="FerrumWeave.Sdk/0.1.0-alpha.1">
  <PropertyGroup><TargetFramework>net10.0</TargetFramework></PropertyGroup>
</Project>
```

```console
dotnet restore
```

Le package contient `Sdk.props`, `Sdk.targets` et le contenu des templates ; les consommateurs ne doivent pas les copier manuellement.

## Tester un prerelease local

```xml
<?xml version="1.0" encoding="utf-8"?>
<configuration><packageSources><clear /><add key="ferrumweave-local" value="../artifacts/packages" /></packageSources></configuration>
```

Les mêmes commandes restore/build/test/run que pour le package publié doivent fonctionner. Les connaissances internes des maintainers ne doivent pas devenir un prérequis caché.

## Invariant de compatibilité

```console
dotnet new console
dotnet new console -lang VB
dotnet new console -lang "F#"
dotnet new console -lang Rust
```

FerrumWeave doit ajouter Rust sans détourner les variantes de langage existantes.
