<!--
doc-id: architecture.repository-layout
locale: ja
translation-of: docs/architecture/repository-layout.md
source-revision: 2baacfc0c69ca60057c479a955c57dad2f808817
-->

# リポジトリ構成

[English](repository-layout.md) · [Deutsch](repository-layout.de.md) · [Español](repository-layout.es.md) · [Français](repository-layout.fr.md) · [Italiano](repository-layout.it.md) · [Português (Brasil)](repository-layout.pt-BR.md) · [Русский](repository-layout.ru.md) · [简体中文](repository-layout.zh-Hans.md) · **日本語**

FerrumWeave は意図的に **capability-first monorepo** として構成されています。Rust から来た contributor にも .NET から来た contributor にも自然に感じられる一方で、製品を恒久的な「Rust 側」と「.NET 側」に分割しないことを目指します。

この文書は目標構造を示します。ディレクトリは実際の increment が必要とした時点で作成し、`.gitkeep` で空ディレクトリだけを維持することはしません。

## 目標構造

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

## ルール

### 実装言語ではなく capability で整理する

トップレベルの製品ディレクトリは責務を表します。`compiler`、`projection`、`sdk`、`tooling` です。`src/rust` と `src/dotnet` を並べるような構成は、FerrumWeave が解消しようとしている境界を逆に固定してしまうため避けます。

各 capability の内部では、それを実装する ecosystem の慣例を使います。Rust crate は通常の `Cargo.toml` + `src/`、.NET component は SDK-style project と一般的な .NET naming を使います。

製品 tooling ではないリポジトリ保守用ユーティリティは `tools/` に置きます。たとえば `tools/docgraph` はドキュメントグラフを維持するためのもので、compiler や開発者向け FerrumWeave toolchain の一部ではありません。

### rustc backend の境界は FerrumWeave が所有する

`compiler/codegen-backend/` は `rustc` がロードする製品境界です。ここに FerrumWeave の `CodegenBackend` 実装があり、rustc MIR を FerrumWeave の lowering、CIL emission、metadata、projection infrastructure へ渡します。

stable workspace は compiler-private API から独立させます。nightly/rustc-dev の要件は隔離された backend lane に属し、通常の crate へ漏らしません。

### `projection/` を独立させる

CLR metadata projection は共有 contract であり、codegen の内部実装詳細ではありません。Compiler、code analysis、Project References、NuGet integration、IntelliSense、将来的には debugging まで、同じ .NET-visible symbol model に依存する可能性があります。

依存方向は明示的に保ちます。Tooling は projection contract を利用できますが、projection は IDE 固有の事情に依存してはいけません。

### Tests は実行可能な interoperability contract

test tree は意図的に両 ecosystem の語彙を使います。`tests/ui` と `tests/codegen` は Rust compiler contributor に自然であり、`tests/interop`、`tests/sdk`、`tests/e2e`、`tests/fixtures` は .NET contributor に自然であるべきです。

可能であれば各 interop fixture は両方向の挙動を証明します。たとえば `tests/interop/rust-csharp/` には、Rust が C# surface を消費する contract と、C# が Rust 由来の CLR surface を消費する contract の両方を将来的に含めます。

### Samples は証拠でアーキテクチャを示す

canonical sample は縦方向に進みます。

1. `hello-world` — Rust → CIL → CLR。
2. `consume-dotnet` — Rust が .NET API または .NET project を利用する。
3. `consumed-by-dotnet` — 別の .NET language が Rust 生成 assembly を利用する。
4. `mixed-solution` — `.rsproj`、`.csproj`、`.fsproj`、`.vbproj` が同一 solution に共存する。

Sample は、示している挙動が実際に実行可能になってから追加します。

### 製品 upstream との差分は可視かつ一時的にする

依存関係が本当に product path の一部なら、FerrumWeave は再発明する前に統合を優先します。`rustc`、`rust-analyzer`、.NET SDK、その他の製品 upstream との差分は、upstream revision、ローカル要件、issue/PR link、exit condition とともに `docs/upstream/` に記録します。

実際の product upstream に対する推奨ライフサイクル：

```text
consume upstream
      ↓
patch only when required
      ↓
submit upstream
      ↓
remove local divergence
```

`rustc_codegen_clr` は異なります。これは固定された **characterization oracle / differential oracle** であり、runtime、SDK、製品 backend の依存関係ではありません。そのためライフサイクルは evidence 中心です。

```text
pin oracle revision
      ↓
characterize observable behavior
      ↓
reproduce the required behavior through FerrumWeave
      ↓
retain only when differential value remains, otherwise remove
```

Oracle provenance と pin は引き続き `docs/upstream/` に置きますが、FerrumWeave はこの backend を製品実装として patch したり利用したりしません。

### 未来を先に作らない

目標 tree は地図であって、空ディレクトリを作れという命令ではありません。新しいフォルダは、そこに属する最初の本物の contract、implementation、test、document と同時に現れます。

これにより repository history は正直なままです。構造は推測的アーキテクチャではなく、実行可能な evidence に従います。

## 2 つの自然な入口

実装が成長するにつれて、contributor はどちらの ecosystem からでも自然にリポジトリへ入れるべきです。

```bash
cargo test --workspace
```

または：

```bash
dotnet build FerrumWeave.slnx
```

どちらも同じ製品と同じ interoperability contract に収束する必要があります。
