<!--
doc-id: architecture.repository-layout
locale: it
translation-of: docs/architecture/repository-layout.md
source-revision: 2baacfc0c69ca60057c479a955c57dad2f808817
-->

# Struttura del repository

[English](repository-layout.md) · [Deutsch](repository-layout.de.md) · [Español](repository-layout.es.md) · [Français](repository-layout.fr.md) · **Italiano** · [Português (Brasil)](repository-layout.pt-BR.md) · [Русский](repository-layout.ru.md) · [简体中文](repository-layout.zh-Hans.md) · [日本語](repository-layout.ja.md)

FerrumWeave è intenzionalmente organizzato come un **monorepo capability-first**. Il repository deve risultare familiare sia ai contributor provenienti da Rust sia a quelli provenienti da .NET, senza dividere in modo permanente il prodotto in un “lato Rust” e un “lato .NET”.

Questo documento descrive la struttura obiettivo. Le directory vengono create solo quando un incremento ne ha realmente bisogno; il progetto non mantiene cartelle vuote tramite file `.gitkeep`.

## Struttura obiettivo

```text
FerrumWeave/
├── .cargo/
├── .github/
│   ├── workflows/
│   ├── ISSUE_TEMPLATE/
│   └── PULL_REQUEST_TEMPLATE.md
├── assets/
│   └── brand/
│       ├── hero/
│       ├── logos/
│       ├── mascot/
│       ├── icons/
│       └── merch/
├── compiler/
│   ├── codegen-backend/
│   ├── cil/
│   └── driver/
├── projection/
│   ├── metadata/
│   ├── types/
│   └── support/
├── sdk/
│   ├── FerrumWeave.Sdk/
│   ├── tasks/
│   └── templates/rust/
├── tooling/
│   ├── analyzer/
│   ├── debugger/
│   ├── vscode/
│   └── visualstudio/
├── tools/
│   ├── docgraph/
│   └── quality/
├── tests/
│   ├── ui/
│   ├── codegen/
│   ├── conformance/
│   ├── interop/
│   │   ├── rust-csharp/
│   │   ├── rust-vb/
│   │   └── rust-fsharp/
│   ├── sdk/
│   ├── e2e/
│   └── fixtures/
├── samples/
│   ├── hello-world/
│   ├── consume-dotnet/
│   ├── consumed-by-dotnet/
│   └── mixed-solution/
├── docs/
│   ├── README.md
│   ├── i18n.md
│   ├── architecture/
│   │   └── adr/
│   ├── compatibility/
│   ├── design/
│   ├── roadmap/
│   ├── site/
│   └── upstream/
├── eng/
│   ├── ci/
│   ├── packaging/
│   └── scripts/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── FerrumWeave.slnx
├── global.json
├── Directory.Build.props
├── Directory.Build.targets
├── README.md
├── CONTRIBUTING.md
├── GOVERNANCE.md
├── SECURITY.md
├── CODE_OF_CONDUCT.md
├── NOTICE
├── LICENSE-MIT
└── LICENSE-APACHE
```

## Regole

### Organizzare per capacità, non per linguaggio di implementazione

Le directory prodotto di primo livello descrivono responsabilità: `compiler`, `projection`, `sdk` e `tooling`. Va evitata una radice come `src/rust` accanto a `src/dotnet`, perché creerebbe proprio il confine architetturale che FerrumWeave vuole dissolvere.

All’interno di una capacità si usano le convenzioni idiomatiche dell’ecosistema che la implementa. I crate Rust usano `Cargo.toml` + `src/`; i componenti .NET usano progetti SDK-style e la normale nomenclatura .NET.

Le utility di manutenzione del repository che non sono tooling di prodotto vivono sotto `tools/`. Per esempio, `tools/docgraph` mantiene il grafo documentale; non fa parte del compiler né della toolchain FerrumWeave rivolta agli sviluppatori.

### Mantenere sotto responsabilità FerrumWeave il confine del backend rustc

`compiler/codegen-backend/` è il confine di prodotto caricato da `rustc`. Contiene l’implementazione FerrumWeave di `CodegenBackend` e instrada il MIR di rustc verso lowering, emissione CIL, metadata e infrastruttura di projection di FerrumWeave.

Il workspace stabile resta indipendente dalle API private del compiler. I requisiti nightly/rustc-dev appartengono alla lane isolata del backend e non devono filtrare nei crate ordinari.

### Mantenere `projection/` indipendente

La projection dei metadata CLR è un contratto condiviso, non un dettaglio interno del codegen. Compiler, analisi del codice, Project References, integrazione NuGet, IntelliSense ed eventualmente debugging possono dipendere dallo stesso modello di simboli visibili da .NET.

La direzione delle dipendenze deve restare esplicita. Il tooling può consumare i contratti di projection; projection non deve dipendere da dettagli specifici dell’IDE.

### I test sono contratti eseguibili di interoperabilità

L’albero dei test prende deliberatamente il vocabolario da entrambi gli ecosistemi. `tests/ui` e `tests/codegen` devono risultare familiari ai contributor del compilatore Rust; `tests/interop`, `tests/sdk`, `tests/e2e` e `tests/fixtures` devono essere naturali per i contributor .NET.

Ogni fixture di interop dovrebbe provare il comportamento in entrambe le direzioni quando è pratico. Per esempio, `tests/interop/rust-csharp/` dovrebbe in futuro contenere contratti in cui Rust consuma una superficie C# e C# consuma una superficie CLR prodotta da Rust.

### I sample vendono l’architettura con evidenza

I sample canonici avanzano verticalmente:

1. `hello-world` — Rust → CIL → CLR.
2. `consume-dotnet` — Rust consuma API .NET o un progetto .NET.
3. `consumed-by-dotnet` — un altro linguaggio .NET consuma una assembly prodotta da Rust.
4. `mixed-solution` — `.rsproj`, `.csproj`, `.fsproj` e `.vbproj` coesistono in una soluzione.

Un sample viene aggiunto solo quando il comportamento che dimostra è eseguibile.

### Rendere visibile e temporanea la divergenza dagli upstream di prodotto

FerrumWeave deve integrare prima di reinventare quando una dipendenza appartiene davvero al product path. La divergenza locale rispetto a `rustc`, `rust-analyzer`, .NET SDK o un altro upstream di prodotto va documentata sotto `docs/upstream/` con revisione upstream, esigenza locale, link issue/PR e condizione di uscita.

Ciclo preferito per i veri upstream di prodotto:

```text
consume upstream
      ↓
patch only when required
      ↓
submit upstream
      ↓
remove local divergence
```

`rustc_codegen_clr` è diverso: è un **characterization oracle / differential oracle** fissato, non una dipendenza runtime, SDK o backend di prodotto. Il suo ciclo è quindi orientato all’evidenza:

```text
pin oracle revision
      ↓
characterize observable behavior
      ↓
reproduce the required behavior through FerrumWeave
      ↓
retain only when differential value remains, otherwise remove
```

Provenance e pin dell’oracle restano sotto `docs/upstream/`, ma FerrumWeave non patcha né consuma quel backend come implementazione del prodotto.

### Non precreare il futuro

L’albero obiettivo è una mappa, non un mandato a creare directory vuote. Le nuove cartelle arrivano con il primo vero contratto, implementazione, test o documento che vi appartiene.

In questo modo la storia del repository resta onesta: la struttura segue evidenza eseguibile, non architettura speculativa.

## Due punti di ingresso familiari

Con la crescita dell’implementazione, un contributor dovrebbe poter affrontare naturalmente il repository da entrambi gli ecosistemi:

```bash
cargo test --workspace
```

oppure:

```bash
dotnet build FerrumWeave.slnx
```

Entrambi i comandi devono convergere sullo stesso prodotto e sugli stessi contratti di interoperabilità.
