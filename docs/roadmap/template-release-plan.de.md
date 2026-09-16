<!--
translation-of: docs/roadmap/template-release-plan.md
locale: de
source-revision: 4ed0f1c8df9606bdd7b1135f186ef4bdd710c77a
-->

# FerrumWeave Template-Releaseplan

[English](template-release-plan.md) · **Deutsch** · [Español](template-release-plan.es.md) · [Français](template-release-plan.fr.md) · [Italiano](template-release-plan.it.md) · [Português (Brasil)](template-release-plan.pt-BR.md) · [Русский](template-release-plan.ru.md) · [简体中文](template-release-plan.zh-Hans.md) · [日本語](template-release-plan.ja.md)

FerrumWeaves Developer-Experience-Roadmap behandelt Rust als **.NET-Projektsprache**, nicht als eigenen Projekttyp.

Die primäre Template-UX lautet:

```console
dotnet new <template> -lang Rust
```

Beispiel:

```console
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

Die frühere Form `dotnet new rust` ist nicht das Zielprodukt. Eine Console-Anwendung bleibt ein `console`-Projekt; FerrumWeave ergänzt Rust als Sprachwahl im .NET-Template-Engine.

## Was „unterstützt“ bedeutet

Ein Template gilt nicht als unterstützt, nur weil es Dateien erzeugen kann.

Für eine Unterstützung in einem FerrumWeave-Prerelease gilt:

1. `dotnet new <template> -lang Rust` erzeugt erwartetes `.rsproj` und Rust-Layout aus einem installierten FerrumWeave-Paket.
2. Restore und Build funktionieren über den normalen .NET-SDK-Workflow ohne FerrumWeave-Checkout.
3. Der sinnvolle Standardworkflow läuft erfolgreich: `dotnet run` für Anwendungen, `dotnet test` für Testprojekte.
4. Das Verhalten muss kausal über `rustc -> FerrumWeave CodegenBackend -> CIL/metadata -> CoreCLR` laufen. Legacy-Emitter oder generierte C#/VB/F#-Ersatzprogramme erfüllen den Vertrag nicht.
5. Plattformübergreifende Templates werden auf Windows und Linux zertifiziert. WinForms und WPF werden auf unterstützten Windows-Umgebungen zertifiziert und implizieren keine Linux-Ausführung.
6. Bekannte Einschränkungen werden explizit dokumentiert.

Samples und Conformance-Fixtures können Prerelease-Verträge belegen, erfüllen aber nicht das 1.0-Realprojekt-Gate.

## Versionsfolge

| Release | Phase | Neu unterstützte Templates |
| --- | --- | --- |
| `0.1` | Alpha | `console`, `classlib`, `xunit`, `nunit`, `mstest`, `web`, `webapi` |
| `0.2` | Alpha | `mvc`, `winforms` |
| `0.3` | Beta | `worker` |
| `0.4` | Beta | `wpf` |
| `0.5` | Beta | `grpc` |
| `0.6` | Beta | `blazor` |
| `1.0` | Stable | Alle zugesagten Template-Familien sind zertifiziert und in mindestens einem echten Projekt erfolgreich eingesetzt |

Suffixe wie `0.1.0-alpha.1` oder `0.5.0-beta.1` sind normal; die Tabelle definiert Fähigkeitsgrenzen, nicht die Zahl der nötigen Prereleases.

## 0.1 Alpha — grundlegende .NET-Spracherfahrung

```console
dotnet new console  -lang Rust
dotnet new classlib -lang Rust
dotnet new xunit    -lang Rust
dotnet new nunit    -lang Rust
dotnet new mstest   -lang Rust
dotnet new web      -lang Rust
dotnet new webapi   -lang Rust
```

Der Release muss mindestens beweisen: ausführbarer Entry Point und Console-Ausgabe; eine von anderen .NET-Projekten konsumierbare managed Class Library; Testentdeckung/-ausführung über xUnit, NUnit und MSTest; einen ASP.NET-Core-Host über `dotnet run`; einen minimalen HTTP Request/Response-Pfad für `web` und `webapi`; normales Restore/Build/Run/Test, `ProjectReference` und NuGet; sowie Installation und Nutzung aus einem sauberen externen Verzeichnis.

## 0.2 Alpha — Application Frameworks

```console
dotnet new mvc      -lang Rust
dotnet new winforms -lang Rust
```

`mvc` muss einen echten MVC-Requestpfad beweisen. `winforms` muss einen echten Windows-Forms-Lifecycle und unterstützte Event-/Delegate-Interaktion auf Windows beweisen.

## 0.3 Beta — Hintergrunddienste

```console
dotnet new worker -lang Rust
```

Der Worker muss .NET Generic Host und einen echten Background-Service-Lifecycle ausführen, nicht nur eine umbenannte Console-App.

## 0.4 Beta — WPF

```console
dotnet new wpf -lang Rust
```

WPF ist eine eigene Fähigkeitsgrenze wegen XAML, WindowsDesktop-MSBuild-Targets, generierten Typen/Code-behind, Startup und Eventmodell.

## 0.5 Beta — gRPC

```console
dotnet new grpc -lang Rust
```

Der Vertrag umfasst den generierten protobuf/gRPC-Buildpfad und mindestens einen echten Request/Response-Roundtrip.

## 0.6 Beta — Blazor

Unterstützt wird die Blazor-Template-Oberfläche des jeweils anvisierten .NET SDK. Der genaue Short Name folgt dem aktuellen SDK statt einen veralteten Befehl festzuschreiben. Der Vertrag muss einen real gerenderten Komponentenpfad sowie erforderliche Razor/Blazor-Buildintegration beweisen.

## 1.0 Stable Gate — Evidenz aus echten Projekten

FerrumWeave erreicht 1.0 nicht allein durch grüne Conformance-Suites.

Vor `1.0.0` braucht **jede zugesagte Template-Familie mindestens ein echtes, erfolgreiches FerrumWeave-Projekt**.

Ein echtes Projekt:

- ist kein Compiler-Fixture, Conformance-Test, Template-Smoke-Test oder reines Dokumentationssample;
- hat einen echten Anwendungs- oder Bibliothekszweck;
- nutzt die entsprechende Rust-Template-Familie in normaler Projektstruktur;
- lässt sich reproduzierbar restaurieren/bauen und gegebenenfalls ausführen/testen;
- übt das Template repräsentativ aus;
- dokumentiert FerrumWeave-spezifische Einschränkungen.

Es darf Open Source oder privat belegt sein, benötigt aber genug reproduzierbare Evidenz für den Kompatibilitätsanspruch.

Die Stable-Frage lautet sowohl:

> Kann FerrumWeave jede zugesagte Projektfamilie scaffolden und zertifizieren?

als auch:

> Hat jede zugesagte Familie mindestens eine echte Anwendung überstanden?

Erst wenn beides ja ist, wird das `1.0.0`-Gate überschritten.

## Kompatibilitätsregel

```console
dotnet new console
dotnet new console -lang VB
dotnet new console -lang "F#"
dotnet new console -lang Rust
```

FerrumWeave erweitert das .NET-Sprachökosystem; es darf bestehende Varianten nicht übernehmen oder brechen.
