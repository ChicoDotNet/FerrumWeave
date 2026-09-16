<!--
doc-id: architecture.adr.0004-rust-as-dotnet-template-language
locale: de
translation-of: docs/architecture/adr/0004-r10-rust-as-dotnet-template-language.md
source-revision: 0059e447c7b43940ac034f423b863d598f24b309
-->

<!-- ferrumweave-nav:start -->
[← Dokumentation](../../README.de.md) · [Projekt-README](../../../README.de.md)
<!-- ferrumweave-nav:end -->

# ADR 0004 — Rust als .NET-Template-Sprache registrieren

[English](0004-r10-rust-as-dotnet-template-language.md) · **Deutsch** · [Español](0004-r10-rust-as-dotnet-template-language.es.md) · [Français](0004-r10-rust-as-dotnet-template-language.fr.md) · [Italiano](0004-r10-rust-as-dotnet-template-language.it.md) · [Português (Brasil)](0004-r10-rust-as-dotnet-template-language.pt-BR.md) · [Русский](0004-r10-rust-as-dotnet-template-language.ru.md) · [简体中文](0004-r10-rust-as-dotnet-template-language.zh-Hans.md) · [日本語](0004-r10-rust-as-dotnet-template-language.ja.md)

- Status: Für R10 und die Prerelease-Roadmap akzeptiert
- Datum: 2026-09-15
- Milestone: R10 — Developer experience / 0.1 alpha

## Kontext

FerrumWeave soll Rust zu einer First-Class-Sprache im .NET-Ökosystem machen. Der ursprüngliche R10-Prototyp bot dafür einen eigenen Template-Befehl an:

```console
dotnet new rust
```

Das ist technisch bequem, modelliert Rust aber wie einen Projekttyp. Im .NET-Template-Modell sind `console`, `classlib`, `web`, Testprojekte, Desktop-Projekte und andere Workload-Familien Projekt-Templates; die Quellsprache ist eine getrennte Auswahlachse.

.NET-Entwickler erwarten bereits, dass äquivalente Projektfamilien sich durch die Sprache unterscheiden statt durch voneinander losgelöste Template-Namen. FerrumWeave soll dieses mentale Modell bewahren.

## Entscheidung

Die primäre Template-UX von FerrumWeave lautet:

```console
dotnet new <template> -lang Rust
```

Der kanonische First-Use-Vertrag ist daher:

```console
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

`dotnet new rust` ist nicht der angestrebte öffentliche Produktvertrag.

FerrumWeave-Templates müssen Rust als Sprache deklarieren und in den normalen .NET-Template-Familien teilnehmen, sofern die Template Engine diese Integration sauber erlaubt. Die Implementierung muss durch ausführbare Template-Resolution-Verträge validiert werden; passende Metadaten allein reichen nicht als Annahme.

Die Installation von FerrumWeave darf das normale Verhalten bestehender C#-, F#- oder Visual-Basic-Varianten weder brechen noch verändern.

## Release-Sequenz

Template-Support wird in folgenden Capability-Releases eingeführt:

- `0.1-alpha`: `console`, `classlib`, `xunit`, `nunit`, `mstest`, `web`, `webapi`;
- `0.2-alpha`: `mvc`, `winforms`;
- `0.3-beta`: `worker`;
- `0.4-beta`: `wpf`;
- `0.5-beta`: `grpc`;
- `0.6-beta`: `blazor`.

Die detaillierten Support-Verträge und das Stable-Release-Gate stehen in [`docs/roadmap/template-release-plan.de.md`](../../roadmap/template-release-plan.de.md).

## Unterstützt bedeutet ausführbar

Ein erfolgreich erzeugtes Verzeichnis reicht nicht als Beweis dafür, dass ein Template unterstützt wird.

Für Prerelease-Kompatibilitätsaussagen muss das Template mindestens:

- über die installierte FerrumWeave-Distribution scaffolden;
- ohne FerrumWeave-Source-Checkout restoren und bauen;
- den standardmäßigen sinnvollen Workflow ausführen (`dotnet run` bzw. `dotnet test`);
- FerrumWeaves echten Produktpfad `rustc -> CodegenBackend -> CIL/metadata -> CoreCLR` erreichen;
- die relevanten CI-/Plattformverträge erfüllen;
- Einschränkungen ehrlich dokumentieren.

Framework-spezifische Templates müssen das Framework-Verhalten belegen, das sie von einer generischen Console-Anwendung unterscheidet.

## 1.0-Gate

FerrumWeave wird nicht allein deshalb stabil, weil synthetische Conformance-Tests bestehen.

Vor `1.0.0` muss jede Template-Familie der festgelegten Prerelease-Sequenz mindestens ein reales Projekt besitzen, das dieses FerrumWeave-Template erfolgreich verwendet.

Ein reales Projekt ist weder Compiler-Fixture noch Template-Smoke-Test noch Dokumentationssample. Es muss einen echten Nicht-Test-Zweck und reproduzierbare, zur Projektfamilie passende Evidenz besitzen.

## Konsequenzen

Positive Konsequenzen:

- FerrumWeave verhält sich wie eine Erweiterung von .NET statt wie ein paralleles CLI-Ökosystem;
- Template-Befehle bleiben für C#-, F#- und Visual-Basic-Entwickler erwartbar;
- die Produkt-Roadmap wird über Verträge pro Projektfamilie messbar;
- zukünftige Templates können hinzugefügt werden, ohne FerrumWeave-spezifische Kommandotaxonomien zu erfinden;
- das 1.0-Gate bindet Kompatibilitätsaussagen an reale Evidenz.

Kosten und Einschränkungen:

- R10-Prototyp-Metadaten und Tests, die `dotnet new rust` voraussetzen, müssen überarbeitet werden;
- Template-Resolution muss gegen die echte .NET Template Engine verifiziert werden;
- Framework-Templates können neue Compiler-/Runtime-Fähigkeiten benötigen, bevor sie ehrlich als unterstützt gelten;
- Desktop- und Framework-spezifische Templates behalten ihre eigenen Plattform-/Build-Grenzen.

## Validierung

R10 muss mindestens ausführbare Falsifizierer für folgendes Zusammenspiel liefern:

```console
dotnet new console -lang Rust
dotnet new console -lang VB
dotnet new console -lang "F#"
```

Nach der FerrumWeave-Installation müssen diese Varianten koexistieren, und das erzeugte Rust-Projekt muss kausal den echten FerrumWeave-Produktbackend verwenden statt eines Legacy-Emitters oder einer Quellsprachsubstitution.

<!-- ferrumweave-backlinks:start -->
## Was hierher verlinkt

- [FerrumWeave-Dokumentation](../../README.de.md)
<!-- ferrumweave-backlinks:end -->
