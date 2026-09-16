<!--
doc-id: architecture.repository-layout
locale: zh-Hans
translation-of: docs/architecture/repository-layout.md
source-revision: 2baacfc0c69ca60057c479a955c57dad2f808817
-->

<!-- ferrumweave-nav:start -->
[← 文档](../README.zh-Hans.md) · [项目 README](../../README.zh-Hans.md)
<!-- ferrumweave-nav:end -->

# 仓库结构

[English](repository-layout.md) · [Deutsch](repository-layout.de.md) · [Español](repository-layout.es.md) · [Français](repository-layout.fr.md) · [Italiano](repository-layout.it.md) · [Português (Brasil)](repository-layout.pt-BR.md) · [Русский](repository-layout.ru.md) · **简体中文** · [日本語](repository-layout.ja.md)

FerrumWeave 有意采用**以能力为中心的 monorepo**。无论贡献者来自 Rust 还是 .NET，仓库结构都应尽量熟悉，同时避免把产品永久拆分成“Rust 一侧”和“.NET 一侧”。

本文描述目标结构。只有当某个增量确实需要时才创建目录；项目不会通过 `.gitkeep` 人为维持空文件夹。

## 目标结构

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

## 规则

### 按能力组织，而不是按实现语言组织

顶层产品目录描述职责：`compiler`、`projection`、`sdk` 和 `tooling`。应避免在根目录中设置 `src/rust` 与 `src/dotnet` 这样的并列结构，因为这会固化 FerrumWeave 本来要消除的架构边界。

在每个能力内部，采用实现它的生态系统的惯用约定。Rust crate 使用标准的 `Cargo.toml` + `src/`；.NET 组件使用 SDK-style project 和常规 .NET 命名。

不属于产品 tooling 的仓库维护工具放在 `tools/` 下。例如，`tools/docgraph` 用于维护文档图；它不是 compiler 的一部分，也不是开发者使用的 FerrumWeave toolchain 的一部分。

### rustc backend 边界由 FerrumWeave 自己负责

`compiler/codegen-backend/` 是由 `rustc` 加载的产品边界。这里包含 FerrumWeave 对 `CodegenBackend` 的实现，并把 rustc MIR 送入 FerrumWeave 自己的 lowering、CIL 生成、metadata 与 projection 基础设施。

稳定 workspace 应保持对 compiler 私有 API 的独立。nightly/rustc-dev 要求属于隔离的 backend lane，不应泄漏到普通 crate。

### 保持 `projection/` 独立

CLR metadata projection 是共享契约，而不是 codegen 的内部实现细节。Compiler、代码分析、Project References、NuGet 集成、IntelliSense，未来甚至 debugging，都可能依赖同一套 .NET 可见符号模型。

依赖方向必须保持清晰。Tooling 可以消费 projection 契约；projection 不应依赖 IDE 特定问题。

### Tests 是可执行的互操作契约

测试树有意借用两个生态系统的词汇。`tests/ui` 和 `tests/codegen` 应让 Rust compiler contributor 感到熟悉；`tests/interop`、`tests/sdk`、`tests/e2e` 和 `tests/fixtures` 应让 .NET contributor 感到自然。

在可行时，每个 interop fixture 都应证明双向行为。例如，`tests/interop/rust-csharp/` 最终既应包含 Rust 消费 C# surface 的契约，也应包含 C# 消费由 Rust 产生的 CLR surface 的契约。

### Samples 用证据展示架构

规范 sample 采用纵向推进：

1. `hello-world` — Rust → CIL → CLR。
2. `consume-dotnet` — Rust 消费 .NET API 或 .NET project。
3. `consumed-by-dotnet` — 另一种 .NET 语言消费 Rust 生成的 assembly。
4. `mixed-solution` — `.rsproj`、`.csproj`、`.fsproj` 和 `.vbproj` 共存在一个 solution 中。

只有当 sample 所展示的行为确实可执行时，才应加入该 sample。

### 产品 upstream 偏差必须可见且临时

当某个依赖确实属于 product path 时，FerrumWeave 应优先集成而不是重造。相对于 `rustc`、`rust-analyzer`、.NET SDK 或其他产品 upstream 的本地偏差，应记录在 `docs/upstream/` 中，并包含 upstream revision、本地需求、issue/PR 链接和退出条件。

真正产品 upstream 的推荐生命周期：

```text
consume upstream
      ↓
patch only when required
      ↓
submit upstream
      ↓
remove local divergence
```

`rustc_codegen_clr` 不同：它是固定的 **characterization oracle / differential oracle**，不是 runtime、SDK 或产品 backend 依赖。因此它的生命周期以证据为中心：

```text
pin oracle revision
      ↓
characterize observable behavior
      ↓
reproduce the required behavior through FerrumWeave
      ↓
retain only when differential value remains, otherwise remove
```

Oracle provenance 和 pin 仍属于 `docs/upstream/`，但 FerrumWeave 不会把该 backend 当作产品实现来 patch 或消费。

### 不要预先创建未来

目标树是一张地图，而不是创建空目录的命令。新目录应随着第一个真正属于它的契约、实现、test 或文档一起出现。

这样可以保持仓库历史的诚实：结构跟随可执行证据，而不是推测性架构。

## 两个熟悉的入口

随着实现增长，贡献者最终应能从任一生态系统自然进入仓库：

```bash
cargo test --workspace
```

或：

```bash
dotnet build FerrumWeave.slnx
```

这两个命令都应最终指向同一个产品和同一组互操作契约。

<!-- ferrumweave-backlinks:start -->
## 链入页面

- [FerrumWeave 文档](../README.zh-Hans.md)
<!-- ferrumweave-backlinks:end -->
