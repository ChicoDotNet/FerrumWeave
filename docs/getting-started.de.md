<!--
translation-of: docs/getting-started.md
locale: de
source-revision: e4bcd0d4b4c4b73232823896144adf098b30758b
-->

# FerrumWeave 0.1 Alpha — Erste Schritte

[English](getting-started.md) · **Deutsch** · [Español](getting-started.es.md) · [Français](getting-started.fr.md) · [Italiano](getting-started.it.md) · [Português (Brasil)](getting-started.pt-BR.md) · [Русский](getting-started.ru.md) · [简体中文](getting-started.zh-Hans.md) · [日本語](getting-started.ja.md)

> **R10-Arbeitsvertrag:** Dieses Dokument beschreibt die externe Erfahrung, die FerrumWeave 0.1 Alpha vor dem Release zertifizieren muss. Befehle, die in CI noch nicht GREEN sind, sind Produktziele und keine Behauptung über bereits veröffentlichte Fähigkeiten.

FerrumWeave erweitert normale .NET-Projekttemplates um **Rust als Sprachwahl**. Die primäre UX ist `dotnet new <template> -lang Rust`, nicht ein eigener Projekttyp `dotnet new rust`.

Release-Reihenfolge und Template-Support stehen in [`template-release-plan.de.md`](roadmap/template-release-plan.de.md). Die Architekturentscheidung ist in [ADR 0004](architecture/adr/0004-r10-rust-as-dotnet-template-language.md) dokumentiert.

## Ziel für den ersten 0.1-Alpha-Lauf

Installiere mit den dokumentierten .NET- und Rust-Voraussetzungen die Alpha-Distribution und erstelle ein normales .NET-Console-Projekt mit Rust:

```console
dotnet new install FerrumWeave.Sdk::0.1.0-alpha.1
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

`FerrumWeave.Sdk` ist die aktuelle Paketidentität für MSBuild SDK und Projekttemplates. Der Release darf kein zweites unveröffentlichtes Template-Paket benötigen, sofern kein späteres ADR dies ausdrücklich ändert.

Akzeptiert wird ein Rust-Projekt, das über den normalen .NET-SDK-Workflow baut und den kausalen Produktpfad erreicht:

```text
Rust source
  -> rustc
  -> FerrumWeave CodegenBackend
  -> CIL / CLR metadata
  -> CoreCLR
```

Der First-Run-Vertrag muss aus einem sauberen externen Verzeichnis funktionieren, in dem das FerrumWeave-Quellrepository nicht vorhanden ist.

## Template-Oberfläche von 0.1 Alpha

```console
dotnet new console  -lang Rust
dotnet new classlib -lang Rust
dotnet new xunit    -lang Rust
dotnet new nunit    -lang Rust
dotnet new mstest   -lang Rust
dotnet new web      -lang Rust
dotnet new webapi   -lang Rust
```

Ein Template gilt nicht schon deshalb als unterstützt, weil es Dateien erzeugt. Es muss Restore/Build und den sinnvollen Standardworkflow (`dotnet run` oder `dotnet test`) über das echte FerrumWeave-Backend erfolgreich abschließen.

## FerrumWeave MSBuild SDK

FerrumWeaves MSBuild SDK wird als NuGet-SDK-Paket verteilt:

```xml
<Project Sdk="FerrumWeave.Sdk/0.1.0-alpha.1">
  <PropertyGroup><TargetFramework>net10.0</TargetFramework></PropertyGroup>
</Project>
```

Restore verwendet die normale .NET-/NuGet-Auflösung:

```console
dotnet restore
```

Das Paket enthält `Sdk.props`, `Sdk.targets` und Template-Inhalt; Nutzer dürfen diese Dateien nicht manuell kopieren müssen.

## Lokal gepackte Prereleases testen

Eine lokal gepackte Alpha kann über eine lokale NuGet-Quelle getestet werden:

```xml
<?xml version="1.0" encoding="utf-8"?>
<configuration><packageSources><clear /><add key="ferrumweave-local" value="../artifacts/packages" /></packageSources></configuration>
```

Danach gelten dieselben Restore-/Build-/Test-/Run-Befehle wie beim veröffentlichten Paket. Maintainer-Wissen darf kein verstecktes Prerequisit sein.

## Kompatibilitätsinvariante

```console
dotnet new console
dotnet new console -lang VB
dotnet new console -lang "F#"
dotnet new console -lang Rust
```

FerrumWeave soll Rust als weitere ernsthafte .NET-Sprachwahl hinzufügen, ohne bestehendes Verhalten zu verändern.
