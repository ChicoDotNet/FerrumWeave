<!--
translation-of: docs/roadmap/template-release-plan.md
locale: it
source-revision: 4ed0f1c8df9606bdd7b1135f186ef4bdd710c77a
-->

# Piano di release dei template FerrumWeave

[English](template-release-plan.md) · [Deutsch](template-release-plan.de.md) · [Español](template-release-plan.es.md) · [Français](template-release-plan.fr.md) · **Italiano** · [Português (Brasil)](template-release-plan.pt-BR.md) · [Русский](template-release-plan.ru.md) · [简体中文](template-release-plan.zh-Hans.md) · [日本語](template-release-plan.ja.md)

La roadmap della developer experience di FerrumWeave considera Rust un **linguaggio di progetto .NET**, non un tipo di progetto separato.

La UX primaria dei template è:

```console
dotnet new <template> -lang Rust
```

Per esempio:

```console
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

La precedente forma `dotnet new rust` non è l’esperienza target. Un’app console resta un progetto `console`; FerrumWeave aggiunge Rust come ulteriore scelta di linguaggio nel template engine .NET.

## Cosa significa "supportato"

Un template non è considerato supportato solo perché genera file.

Per essere indicato come supportato in un prerelease FerrumWeave:

1. `dotnet new <template> -lang Rust` deve creare il `.rsproj` e il layout Rust attesi da un package FerrumWeave installato.
2. Il progetto generato deve fare restore e build attraverso il normale workflow del .NET SDK senza richiedere il checkout del repository FerrumWeave.
3. Il workflow significativo predefinito deve riuscire: `dotnet run` per le applicazioni eseguibili e `dotnet test` per i progetti di test.
4. Il comportamento deve passare causalmente da `rustc -> FerrumWeave CodegenBackend -> CIL/metadata -> CoreCLR`; emitter legacy basati su pattern del sorgente o sostituti generati in C#/VB/F# non soddisfano il contratto.
5. I template multipiattaforma devono essere certificati su Windows e Linux. WinForms e WPF sono certificati su ambienti Windows supportati e non implicano esecuzione Linux.
6. Le limitazioni note devono essere documentate esplicitamente.

Sample e fixture di conformità possono dimostrare un contratto prerelease, ma non soddisfano il gate di progetto reale di 1.0.

## Sequenza delle versioni

| Release | Fase | Template introdotti come supportati |
| --- | --- | --- |
| `0.1` | Alpha | `console`, `classlib`, `xunit`, `nunit`, `mstest`, `web`, `webapi` |
| `0.2` | Alpha | `mvc`, `winforms` |
| `0.3` | Beta | `worker` |
| `0.4` | Beta | `wpf` |
| `0.5` | Beta | `grpc` |
| `0.6` | Beta | `blazor` |
| `1.0` | Stable | Tutte le famiglie impegnate hanno contratti certificati e almeno un progetto reale riuscito |

Sono normali suffissi come `0.1.0-alpha.1` o `0.5.0-beta.1`; la tabella definisce i confini di capacità, non il numero esatto di prerelease.

## 0.1 alpha — esperienza base come linguaggio .NET

```console
dotnet new console  -lang Rust
dotnet new classlib -lang Rust
dotnet new xunit    -lang Rust
dotnet new nunit    -lang Rust
dotnet new mstest   -lang Rust
dotnet new web      -lang Rust
dotnet new webapi   -lang Rust
```

La release deve dimostrare almeno: entry point eseguibile e output console; class library managed consumabile da un altro progetto .NET; discovery ed esecuzione tramite xUnit, NUnit e MSTest; host ASP.NET Core avviato con `dotnet run`; un percorso HTTP request/response minimo per `web` e `webapi`; restore/build/run/test, `ProjectReference` e NuGet normali; installazione e uso da una directory esterna pulita.

## 0.2 alpha — framework applicativi

```console
dotnet new mvc      -lang Rust
dotnet new winforms -lang Rust
```

`mvc` deve provare un vero percorso di richiesta MVC. `winforms` deve provare un vero lifecycle Windows Forms e l’interazione supportata di eventi/delegate su Windows.

## 0.3 beta — servizi in background

```console
dotnet new worker -lang Rust
```

Il template Worker deve esercitare .NET Generic Host e un vero lifecycle di background service, non una console app rinominata.

## 0.4 beta — WPF

```console
dotnet new wpf -lang Rust
```

WPF è un confine di capacità proprio perché XAML, WindowsDesktop MSBuild targets, tipi/code-behind generati, startup e modello eventi introducono requisiti specifici.

## 0.5 beta — gRPC

```console
dotnet new grpc -lang Rust
```

Il contratto deve coprire il percorso di build protobuf/gRPC generato e almeno un vero round trip request/response.

## 0.6 beta — Blazor

Aggiunge la superficie dei template Blazor supportata dalla versione target del .NET SDK. Lo short name esatto deve seguire l’SDK contemporaneo invece di congelare un comando obsoleto. Il contratto deve provare un componente realmente renderizzato e l’integrazione Razor/Blazor richiesta.

## Gate stabile 1.0 — evidenza da progetti reali

FerrumWeave non raggiunge 1.0 soltanto perché tutte le suite di conformità sono verdi.

Prima di `1.0.0`, **ogni famiglia di template impegnata deve avere almeno un progetto reale che usa FerrumWeave con successo**.

Un progetto reale:

- non è una fixture del compiler, un test di conformità, uno smoke test del template o un sample solo documentale;
- ha uno scopo reale di applicazione o libreria;
- usa la famiglia Rust corrispondente nella normale struttura di progetto;
- può fare restore/build e, quando applicabile, run/test in modo riproducibile;
- esercita il template in modo rappresentativo;
- documenta le limitazioni specifiche di FerrumWeave.

Può essere open source o dimostrato privatamente, ma deve avere evidenza riproducibile sufficiente a giustificare la compatibilità dichiarata.

La domanda per la stable release è quindi sia:

> FerrumWeave può scaffoldare e certificare ogni famiglia promessa?

sia:

> Ogni famiglia promessa è sopravvissuta al contatto con almeno una vera applicazione?

Solo quando entrambe le risposte sono sì FerrumWeave supera il gate `1.0.0`.

## Regola di compatibilità

```console
dotnet new console
dotnet new console -lang VB
dotnet new console -lang "F#"
dotnet new console -lang Rust
```

FerrumWeave estende l’ecosistema dei linguaggi .NET; non deve dirottare o rompere le varianti esistenti.
