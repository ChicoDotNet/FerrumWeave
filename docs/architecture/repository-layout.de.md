<!--
doc-id: architecture.repository-layout
locale: de
translation-of: docs/architecture/repository-layout.md
source-revision: 2baacfc0c69ca60057c479a955c57dad2f808817
-->

<!-- ferrumweave-nav:start -->
[← Dokumentation](../README.de.md) · [Projekt-README](../../README.de.md)
<!-- ferrumweave-nav:end -->

# Repository-Struktur

[English](repository-layout.md) · **Deutsch** · [Español](repository-layout.es.md) · [Français](repository-layout.fr.md) · [Italiano](repository-layout.it.md) · [Português (Brasil)](repository-layout.pt-BR.md) · [Русский](repository-layout.ru.md) · [简体中文](repository-layout.zh-Hans.md) · [日本語](repository-layout.ja.md)

FerrumWeave ist bewusst als **capability-first Monorepo** organisiert. Das Repository soll sich für Mitwirkende aus Rust ebenso vertraut anfühlen wie für Mitwirkende aus .NET, ohne das Produkt dauerhaft in eine „Rust-Seite“ und eine „.NET-Seite“ aufzuteilen.

Dieses Dokument beschreibt die Zielstruktur. Verzeichnisse werden erst angelegt, wenn ein Inkrement sie tatsächlich benötigt; das Projekt hält keine leeren Ordner mit `.gitkeep` künstlich am Leben.

## Zielstruktur

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

## Regeln

### Nach Fähigkeit organisieren, nicht nach Implementierungssprache

Die obersten Produktverzeichnisse beschreiben Verantwortlichkeiten: `compiler`, `projection`, `sdk` und `tooling`. Ein Root wie `src/rust` neben `src/dotnet` soll vermieden werden; das würde genau die Architekturgrenze verfestigen, die FerrumWeave auflösen soll.

Innerhalb einer Fähigkeit gelten die idiomatischen Konventionen des jeweiligen Ökosystems. Rust-Crates verwenden normales `Cargo.toml` + `src/`; .NET-Komponenten verwenden SDK-Style-Projekte und übliche .NET-Namensgebung.

Repository-Werkzeuge, die kein Produkt-Tooling sind, liegen unter `tools/`. `tools/docgraph` pflegt beispielsweise den Dokumentationsgraphen; es gehört weder zum Compiler noch zur entwicklerseitigen FerrumWeave-Toolchain.

### Die rustc-Backend-Grenze gehört FerrumWeave

`compiler/codegen-backend/` ist die Produktgrenze, die von `rustc` geladen wird. Dort lebt FerrumWeaves Implementierung von `CodegenBackend`; sie leitet rustc-MIR in FerrumWeave-Lowering, CIL-Emission, Metadaten und Projection-Infrastruktur weiter.

Der stabile Workspace bleibt unabhängig von compiler-internen APIs. Nightly-/rustc-dev-Anforderungen gehören in die isolierte Backend-Lane und dürfen nicht in normale Crates durchsickern.

### `projection/` unabhängig halten

CLR-Metadatenprojektion ist ein geteilter Vertrag und kein internes Detail der Codegenerierung. Compiler, Codeanalyse, Project References, NuGet-Integration, IntelliSense und später Debugging können alle vom selben Modell .NET-sichtbarer Symbole abhängen.

Abhängigkeitsrichtungen sollen explizit bleiben. Tooling darf Projection-Verträge konsumieren; Projection soll nicht von IDE-spezifischen Belangen abhängen.

### Tests sind ausführbare Interoperabilitätsverträge

Der Testbaum verwendet bewusst Vokabular aus beiden Ökosystemen. `tests/ui` und `tests/codegen` sollen Rust-Compiler-Contributors vertraut vorkommen; `tests/interop`, `tests/sdk`, `tests/e2e` und `tests/fixtures` sollen für .NET-Contributors erwartbar sein.

Jede Interop-Fixture sollte Verhalten nach Möglichkeit in beide Richtungen belegen. `tests/interop/rust-csharp/` soll beispielsweise langfristig sowohl Rust→C# als auch C#→Rust-Verträge enthalten.

### Samples verkaufen die Architektur mit Evidenz

Die kanonischen Samples wachsen vertikal:

1. `hello-world` — Rust → CIL → CLR.
2. `consume-dotnet` — Rust konsumiert .NET-APIs oder ein .NET-Projekt.
3. `consumed-by-dotnet` — eine andere .NET-Sprache konsumiert eine von Rust erzeugte Assembly.
4. `mixed-solution` — `.rsproj`, `.csproj`, `.fsproj` und `.vbproj` koexistieren in einer Solution.

Ein Sample wird erst hinzugefügt, wenn das gezeigte Verhalten ausführbar ist.

### Produkt-Upstream-Abweichungen sichtbar und temporär halten

FerrumWeave soll integrieren, bevor es neu erfindet, wenn eine Abhängigkeit tatsächlich Teil des Produktpfads ist. Lokale Abweichungen von `rustc`, `rust-analyzer`, dem .NET SDK oder einem anderen Produkt-Upstream gehören unter `docs/upstream/`, inklusive Revision, lokaler Anforderung, Issue/PR-Link und Exit-Kriterium.

Bevorzugter Lebenszyklus echter Produkt-Upstreams:

```text
consume upstream
      ↓
patch only when required
      ↓
submit upstream
      ↓
remove local divergence
```

`rustc_codegen_clr` ist anders: Es ist ein gepinnter **characterization oracle / differential oracle**, keine Runtime-, SDK- oder Backend-Produktabhängigkeit. Sein Lebenszyklus ist daher evidenzorientiert:

```text
pin oracle revision
      ↓
characterize observable behavior
      ↓
reproduce the required behavior through FerrumWeave
      ↓
retain only when differential value remains, otherwise remove
```

Oracle-Provenance und Pins gehören weiterhin nach `docs/upstream/`, aber FerrumWeave patcht oder konsumiert dieses Backend nicht als Produktimplementierung.

### Die Zukunft nicht vorab anlegen

Der Zielbaum ist eine Karte, kein Auftrag, leere Verzeichnisse anzulegen. Neue Ordner entstehen mit dem ersten echten Vertrag, der ersten Implementierung, dem ersten Test oder Dokument, das dort hingehört.

So bleibt die Repository-Historie ehrlich: Struktur folgt ausführbarer Evidenz statt spekulativer Architektur.

## Zwei vertraute Einstiegspunkte

Mit wachsender Implementierung soll sich das Repository aus beiden Ökosystemen natürlich betreten lassen:

```bash
cargo test --workspace
```

oder:

```bash
dotnet build FerrumWeave.slnx
```

Beide Befehle sollen auf dasselbe Produkt und dieselben Interoperabilitätsverträge zulaufen.

<!-- ferrumweave-backlinks:start -->
## Was hierher verlinkt

- [FerrumWeave-Dokumentation](../README.de.md)
<!-- ferrumweave-backlinks:end -->
