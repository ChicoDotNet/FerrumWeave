<!--
translation-of: docs/getting-started.md
locale: it
source-revision: e4bcd0d4b4c4b73232823896144adf098b30758b
-->

# FerrumWeave 0.1 alpha — primi passi

[English](getting-started.md) · [Deutsch](getting-started.de.md) · [Español](getting-started.es.md) · [Français](getting-started.fr.md) · **Italiano** · [Português (Brasil)](getting-started.pt-BR.md) · [Русский](getting-started.ru.md) · [简体中文](getting-started.zh-Hans.md) · [日本語](getting-started.ja.md)

> **Contratto di lavoro R10:** questo documento descrive l’esperienza esterna che FerrumWeave 0.1 alpha deve certificare prima del release. I comandi non ancora GREEN in CI sono obiettivi di prodotto, non capacità già rilasciate.

FerrumWeave estende i normali template .NET con **Rust come scelta di linguaggio**. L’esperienza principale è `dotnet new <template> -lang Rust`, non `dotnet new rust`.

Sequenza dei release e supporto template sono in [`template-release-plan.it.md`](roadmap/template-release-plan.it.md). La decisione architetturale è in [ADR 0004](architecture/adr/0004-r10-rust-as-dotnet-template-language.md).

## Primo avvio target di 0.1-alpha

```console
dotnet new install FerrumWeave.Sdk::0.1.0-alpha.1
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

`FerrumWeave.Sdk` è l’identità attuale del package per MSBuild SDK e template. Non deve essere richiesto un secondo package non pubblicato senza un ADR successivo.

Il progetto Rust deve raggiungere causalmente:

```text
Rust source
  -> rustc
  -> FerrumWeave CodegenBackend
  -> CIL / CLR metadata
  -> CoreCLR
```

Il contratto deve funzionare da una directory esterna pulita senza checkout FerrumWeave.

## Superficie 0.1-alpha

```console
dotnet new console  -lang Rust
dotnet new classlib -lang Rust
dotnet new xunit    -lang Rust
dotnet new nunit    -lang Rust
dotnet new mstest   -lang Rust
dotnet new web      -lang Rust
dotnet new webapi   -lang Rust
```

Un template è supportato solo quando restore/build e il workflow significativo (`dotnet run` o `dotnet test`) passano attraverso il vero backend.

## FerrumWeave MSBuild SDK

```xml
<Project Sdk="FerrumWeave.Sdk/0.1.0-alpha.1"><PropertyGroup><TargetFramework>net10.0</TargetFramework></PropertyGroup></Project>
```

```console
dotnet restore
```

Il package contiene `Sdk.props`, `Sdk.targets` e template; non devono essere copiati manualmente.

## Testare prerelease locali

```xml
<?xml version="1.0" encoding="utf-8"?>
<configuration><packageSources><clear /><add key="ferrumweave-local" value="../artifacts/packages" /></packageSources></configuration>
```

Devono valere gli stessi comandi restore/build/test/run del package pubblicato, senza prerequisiti nascosti da maintainer.

## Invariante di compatibilità

```console
dotnet new console
dotnet new console -lang VB
dotnet new console -lang "F#"
dotnet new console -lang Rust
```

FerrumWeave aggiunge Rust preservando il comportamento esistente del .NET SDK.
