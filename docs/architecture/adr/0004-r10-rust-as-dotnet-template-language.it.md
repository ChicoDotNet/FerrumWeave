<!--
doc-id: architecture.adr.0004-rust-as-dotnet-template-language
locale: it
translation-of: docs/architecture/adr/0004-r10-rust-as-dotnet-template-language.md
source-revision: 0059e447c7b43940ac034f423b863d598f24b309
-->

<!-- ferrumweave-nav:start -->
[← Documentazione](../../README.it.md) · [README del progetto](../../../README.it.md)
<!-- ferrumweave-nav:end -->

# ADR 0004 — Registrare Rust come linguaggio dei template .NET

[English](0004-r10-rust-as-dotnet-template-language.md) · [Deutsch](0004-r10-rust-as-dotnet-template-language.de.md) · [Español](0004-r10-rust-as-dotnet-template-language.es.md) · [Français](0004-r10-rust-as-dotnet-template-language.fr.md) · **Italiano** · [Português (Brasil)](0004-r10-rust-as-dotnet-template-language.pt-BR.md) · [Русский](0004-r10-rust-as-dotnet-template-language.ru.md) · [简体中文](0004-r10-rust-as-dotnet-template-language.zh-Hans.md) · [日本語](0004-r10-rust-as-dotnet-template-language.ja.md)

- Stato: Accettato per R10 e per la roadmap prerelease
- Data: 2026-09-15
- Milestone: R10 — Developer experience / 0.1 alpha

## Contesto

Lo scopo di FerrumWeave è rendere Rust un linguaggio di prima classe nell’ecosistema .NET. Il prototipo R10 originale esponeva un comando template dedicato:

```console
dotnet new rust
```

È una scorciatoia tecnicamente comoda, ma modella Rust come se fosse un tipo di progetto. Nel modello dei template .NET, `console`, `classlib`, `web`, i progetti di test, desktop e le altre famiglie di workload sono template di progetto; il linguaggio sorgente è una dimensione di selezione distinta.

Uno sviluppatore .NET si aspetta già che famiglie di progetto equivalenti si distinguano per linguaggio e non tramite nomi di template scollegati. FerrumWeave deve preservare questo modello mentale.

## Decisione

La UX primaria dei template FerrumWeave è:

```console
dotnet new <template> -lang Rust
```

Il contratto canonico di primo utilizzo è quindi:

```console
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

`dotnet new rust` non è il contratto pubblico obiettivo del prodotto.

I template FerrumWeave devono dichiarare Rust come linguaggio e partecipare alle normali famiglie di template .NET quando la template engine consente un’integrazione pulita. L’implementazione deve essere validata tramite contratti eseguibili di risoluzione dei template, non assumendo che metadata simili ai template integrati siano sufficienti.

L’installazione di FerrumWeave non deve rompere né alterare il normale comportamento delle varianti esistenti C#, F# o Visual Basic.

## Sequenza delle release

Il supporto template viene introdotto in queste release di capacità:

- `0.1-alpha`: `console`, `classlib`, `xunit`, `nunit`, `mstest`, `web`, `webapi`;
- `0.2-alpha`: `mvc`, `winforms`;
- `0.3-beta`: `worker`;
- `0.4-beta`: `wpf`;
- `0.5-beta`: `grpc`;
- `0.6-beta`: `blazor`.

I contratti dettagliati e il gate della release stabile sono mantenuti in [`docs/roadmap/template-release-plan.it.md`](../../roadmap/template-release-plan.it.md).

## Supportato significa eseguibile

La sola generazione di una directory non è prova sufficiente che un template sia supportato.

Per una dichiarazione di compatibilità prerelease, il template deve almeno:

- essere scaffoldato tramite la distribuzione FerrumWeave installata;
- eseguire restore e build senza un checkout dei sorgenti FerrumWeave;
- eseguire il workflow significativo predefinito (`dotnet run` o `dotnet test`);
- raggiungere il vero product path FerrumWeave `rustc -> CodegenBackend -> CIL/metadata -> CoreCLR`;
- superare i contratti CI/piattaforma rilevanti;
- documentare onestamente le limitazioni.

I template specifici di framework devono dimostrare il comportamento del framework che li distingue da una console app generica.

## Gate 1.0

FerrumWeave non diventa stabile solo perché passano test sintetici di conformità.

Prima di `1.0.0`, ogni famiglia di template nella sequenza prerelease impegnata deve avere almeno un progetto reale che utilizzi con successo quel template FerrumWeave.

Un progetto reale non è una fixture del compiler, uno smoke test del template o un sample documentale. Deve avere uno scopo reale diverso dal test e prove riproducibili adeguate alla famiglia di progetto.

## Conseguenze

Conseguenze positive:

- FerrumWeave si comporta come un’estensione di .NET e non come un ecosistema CLI parallelo;
- i comandi template restano naturali per sviluppatori C#, F# e Visual Basic;
- la roadmap diventa misurabile tramite contratti per famiglia di progetto;
- nuovi template possono essere aggiunti senza inventare tassonomie di comandi specifiche di FerrumWeave;
- il gate 1.0 lega le dichiarazioni di compatibilità a prove reali.

Costi e vincoli:

- metadata e test del prototipo R10 che assumono `dotnet new rust` devono essere rivisti;
- la risoluzione dei template deve essere verificata contro la vera template engine .NET;
- i template framework possono richiedere nuove capacità compiler/runtime prima di poter essere dichiarati supportati con onestà;
- i template desktop e framework-specific mantengono i propri vincoli di piattaforma/build.

## Validazione

R10 deve aggiungere falsificatori eseguibili che dimostrino almeno che:

```console
dotnet new console -lang Rust
dotnet new console -lang VB
dotnet new console -lang "F#"
```

coesistono dopo l’installazione di FerrumWeave e che il progetto Rust generato usa causalmente il vero backend prodotto FerrumWeave, non un emitter legacy o una sostituzione del linguaggio sorgente.

<!-- ferrumweave-backlinks:start -->
## Cosa punta qui

- [Documentazione FerrumWeave](../../README.it.md)
<!-- ferrumweave-backlinks:end -->
