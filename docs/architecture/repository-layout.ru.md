<!--
doc-id: architecture.repository-layout
locale: ru
translation-of: docs/architecture/repository-layout.md
source-revision: 2baacfc0c69ca60057c479a955c57dad2f808817
-->

<!-- ferrumweave-nav:start -->
[← Документация](../README.ru.md) · [README проекта](../../README.ru.md)
<!-- ferrumweave-nav:end -->

# Структура репозитория

[English](repository-layout.md) · [Deutsch](repository-layout.de.md) · [Español](repository-layout.es.md) · [Français](repository-layout.fr.md) · [Italiano](repository-layout.it.md) · [Português (Brasil)](repository-layout.pt-BR.md) · **Русский** · [简体中文](repository-layout.zh-Hans.md) · [日本語](repository-layout.ja.md)

FerrumWeave намеренно организован как **монорепозиторий, ориентированный на возможности**. Репозиторий должен быть понятен участникам как из Rust, так и из .NET, не разделяя продукт навсегда на «сторону Rust» и «сторону .NET».

Этот документ описывает целевую структуру. Каталоги создаются только тогда, когда они действительно нужны конкретному инкременту; проект не сохраняет пустые папки с помощью `.gitkeep`.

## Целевая структура

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

## Правила

### Организовывать по возможностям, а не по языку реализации

Верхнеуровневые каталоги продукта описывают зоны ответственности: `compiler`, `projection`, `sdk` и `tooling`. Не следует создавать корень вида `src/rust` рядом с `src/dotnet`: это закрепило бы архитектурную границу, которую FerrumWeave как раз должен устранять.

Внутри каждой возможности используются естественные соглашения соответствующей экосистемы. Rust-crates используют обычные `Cargo.toml` + `src/`; компоненты .NET используют SDK-style projects и стандартные правила именования .NET.

Служебные утилиты для обслуживания репозитория, которые не являются продуктовым tooling, находятся в `tools/`. Например, `tools/docgraph` обслуживает граф документации и не входит ни в compiler, ни в пользовательскую toolchain FerrumWeave.

### Граница rustc backend должна принадлежать FerrumWeave

`compiler/codegen-backend/` — это продуктовая граница, загружаемая `rustc`. Здесь находится реализация FerrumWeave `CodegenBackend`, направляющая MIR rustc в lowering FerrumWeave, генерацию CIL, metadata и инфраструктуру projection.

Стабильный workspace остаётся независимым от приватных API compiler. Требования nightly/rustc-dev относятся к изолированной backend lane и не должны проникать в обычные crates.

### Сохранять `projection/` независимым

Проекция CLR metadata — общий контракт, а не внутренняя деталь codegen. Compiler, анализ кода, Project References, интеграция NuGet, IntelliSense и со временем debugging могут зависеть от одной модели символов, видимых из .NET.

Направление зависимостей должно оставаться явным. Tooling может использовать контракты projection; projection не должен зависеть от IDE-специфичных деталей.

### Tests — исполняемые контракты интероперабельности

Дерево tests намеренно использует лексику обеих экосистем. `tests/ui` и `tests/codegen` должны быть привычны участникам Rust compiler; `tests/interop`, `tests/sdk`, `tests/e2e` и `tests/fixtures` — участникам .NET.

Каждый interop fixture должен по возможности доказывать поведение в обе стороны. Например, `tests/interop/rust-csharp/` со временем должен содержать контракты, где Rust использует C# API и где C# использует CLR-поверхность, созданную из Rust.

### Samples демонстрируют архитектуру через доказательства

Канонические samples развиваются вертикально:

1. `hello-world` — Rust → CIL → CLR.
2. `consume-dotnet` — Rust использует .NET API или .NET project.
3. `consumed-by-dotnet` — другой язык .NET использует assembly, созданную из Rust.
4. `mixed-solution` — `.rsproj`, `.csproj`, `.fsproj` и `.vbproj` сосуществуют в одной solution.

Sample добавляется только тогда, когда демонстрируемое поведение исполняемо.

### Держать отклонения от продуктовых upstream видимыми и временными

FerrumWeave должен интегрировать прежде, чем изобретать заново, если зависимость действительно входит в product path. Локальные отклонения от `rustc`, `rust-analyzer`, .NET SDK или другого продуктового upstream должны фиксироваться в `docs/upstream/` вместе с upstream revision, локальной потребностью, ссылкой issue/PR и условием выхода.

Предпочтительный цикл для реальных продуктовых upstream:

```text
consume upstream
      ↓
patch only when required
      ↓
submit upstream
      ↓
remove local divergence
```

`rustc_codegen_clr` отличается: это закреплённый **characterization oracle / differential oracle**, а не runtime-, SDK- или backend-зависимость продукта. Поэтому его цикл ориентирован на доказательства:

```text
pin oracle revision
      ↓
characterize observable behavior
      ↓
reproduce the required behavior through FerrumWeave
      ↓
retain only when differential value remains, otherwise remove
```

Происхождение и pins oracle по-прежнему относятся к `docs/upstream/`, но FerrumWeave не патчит и не использует этот backend как реализацию продукта.

### Не создавать будущее заранее

Целевое дерево — это карта, а не требование создавать пустые каталоги. Новые папки появляются вместе с первым реальным контрактом, реализацией, test или документом, которому они нужны.

Так история репозитория остаётся честной: структура следует исполняемым доказательствам, а не спекулятивной архитектуре.

## Две привычные точки входа

По мере роста реализации участник должен естественно работать с репозиторием из любой из двух экосистем:

```bash
cargo test --workspace
```

или:

```bash
dotnet build FerrumWeave.slnx
```

Обе команды должны сходиться к одному продукту и одним и тем же контрактам интероперабельности.

<!-- ferrumweave-backlinks:start -->
## Что ссылается сюда

- [Документация FerrumWeave](../README.ru.md)
<!-- ferrumweave-backlinks:end -->
