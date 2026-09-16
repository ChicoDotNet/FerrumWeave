<!--
doc-id: architecture.repository-layout
locale: es
translation-of: docs/architecture/repository-layout.md
source-revision: 2baacfc0c69ca60057c479a955c57dad2f808817
-->

# Estructura del repositorio

[English](repository-layout.md) · [Deutsch](repository-layout.de.md) · **Español** · [Français](repository-layout.fr.md) · [Italiano](repository-layout.it.md) · [Português (Brasil)](repository-layout.pt-BR.md) · [Русский](repository-layout.ru.md) · [简体中文](repository-layout.zh-Hans.md) · [日本語](repository-layout.ja.md)

FerrumWeave está diseñado intencionalmente como un **monorepo organizado por capacidades**. El repositorio debe resultar familiar tanto a contributors de Rust como de .NET, sin dividir el producto de forma permanente en un “lado Rust” y un “lado .NET”.

Este documento describe la estructura objetivo. Los directorios sólo se crean cuando un incremento realmente los necesita; el proyecto no mantiene carpetas vacías con archivos `.gitkeep`.

## Estructura objetivo

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

## Reglas

### Organizar por capacidad, no por lenguaje de implementación

Las carpetas de producto de primer nivel describen responsabilidades: `compiler`, `projection`, `sdk` y `tooling`. Debe evitarse una raíz como `src/rust` junto a `src/dotnet`, porque crearía una frontera arquitectónica que FerrumWeave precisamente busca disolver.

Dentro de cada capacidad se usan las convenciones nativas del ecosistema que la implementa. Los crates Rust usan `Cargo.toml` + `src/`; los componentes .NET usan proyectos SDK-style y nomenclatura normal de .NET.

Las utilidades de mantenimiento del repositorio que no son tooling de producto viven bajo `tools/`. Por ejemplo, `tools/docgraph` mantiene el grafo documental; no forma parte del compiler ni de la toolchain de FerrumWeave que consume el developer.

### Mantener bajo propiedad de FerrumWeave la frontera con el backend de rustc

`compiler/codegen-backend/` es la frontera de producto que carga `rustc`. Allí vive la implementación de FerrumWeave de `CodegenBackend`, que dirige el MIR de rustc hacia lowering, emisión de CIL, metadata e infraestructura de projection propias de FerrumWeave.

El workspace estable permanece independiente de APIs privadas del compiler. Los requisitos nightly/rustc-dev pertenecen a la lane aislada del backend y no deben filtrarse a crates normales.

### Mantener `projection/` independiente

La proyección de metadata CLR es un contrato compartido, no un detalle interno del codegen. Compiler, análisis de código, Project References, integración NuGet, IntelliSense y eventualmente debugging pueden depender del mismo modelo de símbolos visibles desde .NET.

La dirección de dependencias debe seguir siendo explícita. El tooling puede consumir contratos de projection; projection no debe depender de detalles específicos del IDE.

### Los tests son contratos ejecutables de interoperabilidad

El árbol de tests toma deliberadamente vocabulario de ambos ecosistemas. `tests/ui` y `tests/codegen` deben resultar familiares a contributors del compilador Rust; `tests/interop`, `tests/sdk`, `tests/e2e` y `tests/fixtures` deben resultar naturales a contributors .NET.

Cada fixture de interop debería demostrar comportamiento en ambas direcciones cuando sea práctico. Por ejemplo, `tests/interop/rust-csharp/` deberá contener contratos donde Rust consume una superficie C# y donde C# consume una superficie CLR producida desde Rust.

### Los samples venden la arquitectura con evidencia

Los samples canónicos progresan verticalmente:

1. `hello-world` — Rust → CIL → CLR.
2. `consume-dotnet` — Rust consume APIs .NET o un proyecto .NET.
3. `consumed-by-dotnet` — otro lenguaje .NET consume un assembly producido desde Rust.
4. `mixed-solution` — `.rsproj`, `.csproj`, `.fsproj` y `.vbproj` coexisten en una misma solución.

Un sample sólo se agrega cuando el comportamiento que demuestra es ejecutable.

### Mantener visible y temporal la divergencia con upstreams de producto

FerrumWeave debe integrar antes de reinventar cuando una dependencia sea realmente parte del product path. La divergencia local respecto de `rustc`, `rust-analyzer`, .NET SDK u otro upstream de producto debe registrarse bajo `docs/upstream/` con revisión upstream, necesidad local, enlace a issue/PR y condición de salida.

El ciclo preferido para upstreams reales de producto es:

```text
consume upstream
      ↓
patch only when required
      ↓
submit upstream
      ↓
remove local divergence
```

`rustc_codegen_clr` es diferente: es un **oráculo de caracterización / oráculo diferencial** fijado, no una dependencia de runtime, SDK ni backend de producto. Su ciclo es por tanto orientado a evidencia:

```text
pin oracle revision
      ↓
characterize observable behavior
      ↓
reproduce the required behavior through FerrumWeave
      ↓
retain only when differential value remains, otherwise remove
```

La procedencia y los pins del oráculo siguen perteneciendo a `docs/upstream/`, pero FerrumWeave no parchea ni consume ese backend como implementación del producto.

### No precrear el futuro

El árbol objetivo es un mapa, no un mandato para crear directorios vacíos. Las nuevas carpetas aparecen junto con el primer contrato, implementación, test o documento real que pertenece ahí.

Así la historia del repositorio se mantiene honesta: la estructura sigue evidencia ejecutable, no arquitectura especulativa.

## Dos puntos de entrada familiares

Conforme crezca la implementación, un contributor debería poder aproximarse al repositorio con naturalidad desde cualquiera de los dos ecosistemas:

```bash
cargo test --workspace
```

o:

```bash
dotnet build FerrumWeave.slnx
```

Ambos comandos deben converger sobre el mismo producto y los mismos contratos de interoperabilidad.
