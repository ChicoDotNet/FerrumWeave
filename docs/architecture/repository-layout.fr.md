<!--
doc-id: architecture.repository-layout
locale: fr
translation-of: docs/architecture/repository-layout.md
source-revision: 2baacfc0c69ca60057c479a955c57dad2f808817
-->

# Structure du dépôt

[English](repository-layout.md) · [Deutsch](repository-layout.de.md) · [Español](repository-layout.es.md) · **Français** · [Italiano](repository-layout.it.md) · [Português (Brasil)](repository-layout.pt-BR.md) · [Русский](repository-layout.ru.md) · [简体中文](repository-layout.zh-Hans.md) · [日本語](repository-layout.ja.md)

FerrumWeave est volontairement organisé comme un **monorepo centré sur les capacités**. Le dépôt doit être familier aussi bien aux contributeurs venant de Rust qu’à ceux venant de .NET, sans diviser durablement le produit en un « côté Rust » et un « côté .NET ».

Ce document décrit la structure cible. Les répertoires ne sont créés que lorsqu’un incrément en a réellement besoin ; le projet ne maintient pas de dossiers vides au moyen de fichiers `.gitkeep`.

## Structure cible

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

## Règles

### Organiser par capacité, pas par langage d’implémentation

Les dossiers produit de premier niveau décrivent des responsabilités : `compiler`, `projection`, `sdk` et `tooling`. Il faut éviter une racine telle que `src/rust` à côté de `src/dotnet`, car elle créerait précisément la frontière architecturale que FerrumWeave cherche à dissoudre.

À l’intérieur d’une capacité, on utilise les conventions idiomatiques de l’écosystème qui l’implémente. Les crates Rust utilisent `Cargo.toml` + `src/` ; les composants .NET utilisent des projets SDK-style et les conventions .NET habituelles.

Les utilitaires de maintenance du dépôt qui ne font pas partie du tooling produit vivent sous `tools/`. Par exemple, `tools/docgraph` maintient le graphe documentaire ; il ne fait partie ni du compiler ni de la toolchain FerrumWeave destinée aux développeurs.

### Garder la frontière du backend rustc sous responsabilité de FerrumWeave

`compiler/codegen-backend/` est la frontière produit chargée par `rustc`. Elle contient l’implémentation FerrumWeave de `CodegenBackend` et dirige le MIR de rustc vers le lowering FerrumWeave, l’émission CIL, les metadata et l’infrastructure de projection.

Le workspace stable reste indépendant des API privées du compiler. Les exigences nightly/rustc-dev appartiennent à la lane backend isolée et ne doivent pas contaminer les crates ordinaires.

### Garder `projection/` indépendant

La projection des metadata CLR est un contrat partagé, pas un détail interne de la génération de code. Le compiler, l’analyse de code, les Project References, l’intégration NuGet, IntelliSense puis éventuellement le debugging peuvent tous dépendre du même modèle de symboles visibles côté .NET.

La direction des dépendances doit rester explicite. Le tooling peut consommer les contrats de projection ; projection ne doit pas dépendre de préoccupations spécifiques à un IDE.

### Les tests sont des contrats exécutables d’interopérabilité

L’arbre de tests emprunte volontairement le vocabulaire des deux écosystèmes. `tests/ui` et `tests/codegen` doivent être familiers aux contributeurs du compilateur Rust ; `tests/interop`, `tests/sdk`, `tests/e2e` et `tests/fixtures` doivent être naturels pour les contributeurs .NET.

Chaque fixture d’interop devrait démontrer le comportement dans les deux sens lorsque c’est pratique. Par exemple, `tests/interop/rust-csharp/` doit à terme contenir des contrats où Rust consomme une surface C# et où C# consomme une surface CLR produite depuis Rust.

### Les samples vendent l’architecture par la preuve

Les samples canoniques progressent verticalement :

1. `hello-world` — Rust → CIL → CLR.
2. `consume-dotnet` — Rust consomme des API .NET ou un projet .NET.
3. `consumed-by-dotnet` — un autre langage .NET consomme une assembly produite depuis Rust.
4. `mixed-solution` — `.rsproj`, `.csproj`, `.fsproj` et `.vbproj` coexistent dans une même solution.

Un sample n’est ajouté que lorsque le comportement qu’il illustre est exécutable.

### Garder les divergences upstream du produit visibles et temporaires

FerrumWeave doit intégrer avant de réinventer lorsqu’une dépendance appartient réellement au product path. Toute divergence locale par rapport à `rustc`, `rust-analyzer`, au .NET SDK ou à un autre upstream produit doit être documentée sous `docs/upstream/` avec la révision upstream, le besoin local, le lien issue/PR et la condition de sortie.

Cycle préféré pour les véritables upstreams produit :

```text
consume upstream
      ↓
patch only when required
      ↓
submit upstream
      ↓
remove local divergence
```

`rustc_codegen_clr` est différent : c’est un **oracle de caractérisation / oracle différentiel** épinglé, et non une dépendance runtime, SDK ou backend produit. Son cycle est donc orienté preuve :

```text
pin oracle revision
      ↓
characterize observable behavior
      ↓
reproduce the required behavior through FerrumWeave
      ↓
retain only when differential value remains, otherwise remove
```

La provenance et les pins de l’oracle restent dans `docs/upstream/`, mais FerrumWeave ne patche ni ne consomme ce backend comme implémentation produit.

### Ne pas précréer le futur

L’arbre cible est une carte, pas une obligation de créer des répertoires vides. Les nouveaux dossiers apparaissent avec le premier vrai contrat, la première implémentation, le premier test ou document qui leur appartient.

Ainsi, l’historique du dépôt reste honnête : la structure suit les preuves exécutables plutôt qu’une architecture spéculative.

## Deux points d’entrée familiers

À mesure que l’implémentation grandit, un contributeur doit pouvoir aborder le dépôt naturellement depuis l’un ou l’autre écosystème :

```bash
cargo test --workspace
```

ou :

```bash
dotnet build FerrumWeave.slnx
```

Ces deux commandes doivent converger vers le même produit et les mêmes contrats d’interopérabilité.
