<!--
translation-of: README.md
locale: ja
source-revision: 6fb75d076d59b4e1932b685e441487f6caedb36d
-->

[English](README.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [简体中文](README.zh-Hans.md) · **日本語**

> FerrumWeave のドキュメントは複数の言語で整備します。相互運用性は技術だけでなく、人にも関わるからです。翻訳が一時的に遅れた場合は、言語サフィックスのない英語文書が正規の情報源です。

<img src="assets/brand/hero/ferrumweave-readme-cover.png" alt="FerrumWeave — Rust を .NET エコシステムへ" width="100%" />

# FerrumWeave

**Rust を .NET の言語エコシステムへ。**

**プロジェクト資料:** [リポジトリ構成](docs/architecture/repository-layout.md) · [テンプレートのリリース計画](docs/roadmap/template-release-plan.md) · [プロジェクトサイト](https://chicodotnet.github.io/FerrumWeave/ja/)

FerrumWeave は、Rust を .NET プラットフォームの第一級言語にすることを目指す実験的なオープンソースプロジェクトです。Rust のソースコードを .NET assembly にコンパイルし、Common Type System に参加し、既存の .NET ライブラリを利用し、C#、F#、Visual Basic、その他 CLR 上の言語と自然に相互運用できる世界を目指します。

長期的には、開発体験が特別なものではなく自然に感じられるべきです。

```bash
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

概念的には、Rust コードは次のようになります。

```rust
use dotnet::System::*;
use dotnet::Result;

fn main() -> Result<()> {
    Console::WriteLine("Hello from Rust on .NET")?;
    Ok(())
}
```

そして実際の .NET assembly を生成します。

```text
HelloFerrum.dll
```

これは .NET runtime によって実行されます。

> **FerrumWeave はまだこの道の出発点にあります。**
>
> この README の例は、目標とする開発体験とアーキテクチャの方向性を示すものです。すでにリリース済みの機能を主張するものではありません。

---

## なぜ FerrumWeave なのか？

.NET の最も長く生き続けてきたアイデアのひとつは、C# そのものではありません。

それは、**異なる言語が共通の runtime 上で出会える**という考え方です。

何十年もの間、開発者は構文、思想、歴史が大きく異なる言語を使いながら、同じ基盤を共有してソフトウェアを作ってきました。

- C#
- Visual Basic .NET
- F#
- C++/CLI
- JScript .NET
- J#
- IronPython
- IronRuby
- Nemerle
- Boo
- Oxygene
- その他多くの言語

ある言語で書かれたライブラリを別の言語から使えたのは、重要な境界がソース言語そのものではなかったからです。

その境界は **Common Language Infrastructure**、**Common Type System**、assembly metadata、そして CLR でした。

FerrumWeave はシンプルな問いを投げかけます。

> **もし Rust もその言語ファミリーに加われるなら？**

FFI 経由でネイティブ Rust ライブラリを呼ぶだけではありません。

Rust 実行ファイルから CLR を host するだけでもありません。

Rust を C# に変換することでもありません。

Rust を、CIL、assembly、metadata、型、参照、package、tooling、runtime 相互運用性という、もともと多言語 .NET を可能にした同じ世界へコンパイルすることを目指します。

---

## なぜ Rust なのか？

Rust はソフトウェア開発に異なる種類の保証をもたらします。

ownership モデル、borrow checker、強力な型システム、明示的なエラー処理、メモリ安全性と並行処理安全性への重視によって、ある種の欠陥を production に到達する前に検出できます。

Rust があればソフトウェアが **絶対に失敗しない** わけではありません。

ロジックは間違えます。ファイルは消えます。ネットワークは壊れます。データベースには不正なデータが入ります。プログラムは panic できます。`unsafe` も存在します。

しかし Rust は重要な失敗の一部を、

```text
production
```

から、

```text
compile time
```

へ移動できます。

長期間運用される業務システムでは、この違いが大きな意味を持ちます。

---

## 業務ソフトウェアにおける機会

10年、15年、あるいは25年にわたって価値を提供している巨大な .NET システムは数多く存在します。

たとえば次のようなプロジェクトが含まれているかもしれません。

```text
ERP.vbproj
Accounting.csproj
Reporting.fsproj
LegacyIntegration.vbproj
```

モダナイゼーションの議論は、あまりにも頻繁にこう始まります。

> 「全部書き直そう。」

FerrumWeave は別の考え方から始めます。

> **動いているものは残す。次に作るものを強くする。**

同じシステムに、

```text
RiskEngine.rsproj
```

を追加できると想像してください。

既存の Visual Basic アプリケーションを消す必要はありません。C# のドメインモデルを書き直す必要もありません。F# のレポートエンジンに新しい統合プロトコルを追加する必要もありません。

将来的には、

```text
ERP.vbproj
     │
     ▼
RiskEngine.rsproj
     │
     ▼
Accounting.csproj
```

が同じ .NET 型システムを通じて通信できるようになります。

何十年も前の Visual Basic アプリケーションが、今日書かれた新しい Rust コードを呼べる。Rust が C# で書かれたドメイン assembly を利用できる。F# が Rust で実装された型を使える。

移行単位はアプリケーション全体ではなく **コンポーネント** になります。

それがこのプロジェクトのビジョンです。

---

# 目標

FerrumWeave は次の世界を目指します。

```text
                    .NET
                     │
             Common Type System
                     │
       ┌─────────────┼─────────────┐
       │             │             │
      C#            F#           Rust
       │             │             │
    Roslyn          fsc          rustc
       │             │             │
       └─────────────┼─────────────┘
                     │
                     ▼
                CIL + Metadata
                     │
                     ▼
                    CLR
```

Rust の想定コンパイル経路はおおよそ次の通りです。

```text
Rust source
    │
    ▼
rustc frontend
    │
    ▼
HIR / MIR
    │
    ▼
CLR code generation
    │
    ▼
CIL + .NET metadata
    │
    ▼
.NET assembly
    │
    ▼
CLR
```

Rust は Rust のままです。

CLR は CLR のままです。

FerrumWeave はどちらかを不要に作り直すのではなく、両者を接続するべきです。

---

# FerrumWeave が目指さないもの

FerrumWeave は次のものを目指していません。

### 新しい Rust 風言語

Rust 言語そのものを保持し、既存の Rust コンパイラエコシステムを活用することが目的です。

```rust
match
traits
lifetimes
ownership
borrowing
async
Result<T, E>
```

これらは別のコンパイラで近似実装されるのではなく、Rust の概念であり続けるべきです。

### `dotnet` の周りに置く Rust wrapper

MSBuild target から Cargo を起動することは有用かもしれませんが、それだけでは Rust は .NET 言語にはなりません。

FerrumWeave はさらに深い統合を目指します。

### ネイティブ FFI ジェネレータ

ネイティブ相互運用性は重要ですが、目標は次ではありません。

```text
C#
 ↓
P/Invoke
 ↓
Rust native DLL
```

目標は次です。

```text
C#
  ╲
   CLR
  ╱
Rust
```

### .NET の置き換え

FerrumWeave が存在するのは .NET エコシステムに価値があるからです。目的は言語の選択肢を増やすことです。

### ネイティブ Rust の置き換え

Rust を直接ネイティブコードへコンパイルするべき理由は今後も数多くあります。CLR target は deployment と相互運用の追加選択肢です。

---

# 目標とする開発体験

将来的に Rust プロジェクトは .NET solution の中で自然に見えるべきです。

```text
EnterpriseSystem.slnx
│
├── Domain/
│   └── Domain.csproj
├── Reporting/
│   └── Reporting.fsproj
├── Legacy/
│   └── Legacy.vbproj
└── RiskEngine/
    └── RiskEngine.rsproj
```

馴染みのあるコマンドは馴染みのあるままであるべきです。

```bash
dotnet restore
dotnet build
dotnet run
dotnet test
dotnet publish
dotnet pack
```

プロジェクトは最終的に次のような形になるかもしれません。

```xml
<Project Sdk="FerrumWeave.Sdk">
  <PropertyGroup>
    <TargetFramework>net10.0</TargetFramework>
    <RustEdition>2024</RustEdition>
  </PropertyGroup>
  <ItemGroup>
    <ProjectReference Include="../Domain/Domain.csproj" />
    <PackageReference Include="Some.DotNet.Package" Version="..." />
  </ItemGroup>
</Project>
```

Rust 依存関係に必要な場面では Cargo と crates.io が引き続き役割を持つべきです。.NET 依存関係については NuGet と MSBuild が得意な仕事を続けるべきです。

FerrumWeave はどちらかのエコシステムを無視するのではなく、両者を接続します。

---

# Rust から .NET ライブラリを利用する

中心的な目標のひとつは、.NET API が Rust コードの自然な参加者になることです。

```rust
use dotnet::System::*;
use dotnet::System::IO::*;

fn main() -> Result<()> {
    Console::Write("Name: ")?;
    let name = Console::ReadLine()?;
    File::WriteAllText("name.txt", &name)?;
    Ok(())
}
```

重要なのは正確な構文ではなく、次の性質です。

> `System.Console`、`System.String`、`System.IO.File`、ユーザー定義の .NET 型を、手作業で保守する FFI 層の向こうにある不透明なネイティブライブラリではなく、CLR の型と member として扱うこと。

同じ原則は NuGet package と ProjectReference にも適用されるべきです。

---

# 他の .NET 言語から Rust ライブラリを利用する

相互運用性は双方向でなければなりません。

Rust は、他の .NET 言語から利用できる public CLR-facing type を定義できるようになるべきです。

Rust:

```rust
pub struct RiskEngine {
    // ...
}

impl RiskEngine {
    pub fn calculate(&self, customer: Customer) -> RiskScore {
        // ...
    }
}
```

C#:

```csharp
var engine = new RiskEngine();
var score = engine.Calculate(customer);
```

Visual Basic:

```vb
Dim engine = New RiskEngine()
Dim score = engine.Calculate(customer)
```

F#:

```fsharp
let engine = RiskEngine()
let score = engine.Calculate(customer)
```

異なるソース言語。ひとつの runtime contract。

FerrumWeave が最終的に目指す相互運用性はこのレベルです。

---

# Rust 型と CLR 型

Rust と CLR は、オブジェクトとメモリについて根本的に異なるモデルを持っています。この違いを隠してはいけません。

Rust には次のような概念があります。

```text
ownership
borrowing
lifetimes
RAII
Box<T>
Vec<T>
String
Option<T>
Result<T, E>
```

CLR には次があります。

```text
managed references
garbage collection
System.Object
System.String
arrays
interfaces
delegates
exceptions
Task<T>
```

FerrumWeave は両者を同じように見せるために、どちらかのモデルを弱めるべきではありません。代わりに、原則に基づいた対応関係を定義する必要があります。

自然な対応もあります。

```text
System.Int32   ↔ i32
System.Int64   ↔ i64
System.Boolean ↔ bool
System.Double  ↔ f64
```

一方、明示的な意味論が必要なものもあります。

```text
System.String
managed classes
interfaces
delegates
exceptions
Task<T>
Span<T>
Nullable<T>
```

これらの意味論を正しく決めることは、このプロジェクトの中核的なエンジニアリング課題です。

---

# 相互運用性を捨てない安全性

企業ソフトウェアがゼロからやり直せることはほとんどありません。

組織には、既に動いているアプリケーション、データベース、業務ルール、API、package、framework、開発者、運用知識があります。

より安全な言語を導入するために、それらすべてを捨てる必要がないなら、導入ははるかに現実的になります。

FerrumWeave は次の考え方を検証します。

> **既存の .NET 資産との相互運用性を保ちながら、新しく重要な .NET コンポーネントに Rust の安全モデルを持ち込む。**

---

# FerrumWeave という名前について

**Ferrum** はラテン語で鉄を意味します。Rust は鉄の酸化です。

**Weave** は別々の糸を織り合わせ、ひとつのつながった構造にすることを意味します。

```text
Ferrum
   │
   └── iron → rust

Weave
   │
   └── interconnection → network → ecosystem
```

つまり：

> **FerrumWeave は、.NET エコシステムに織り込まれる Rust を表します。**

この名称は Rust と .NET の商標から意図的に独立しています。

---

# 既存の成果を土台にする

FerrumWeave は、新しい Rust コンパイラを一から書くことから始めません。

Rust エコシステムにはすでに価値ある基盤があります。

- `rustc`: parsing、type checking、borrow checking、MIR、言語意味論
- `rust-analyzer`: 現代的な Rust コード解析と開発 tooling

Rust→CLR についても重要な prior art があり、特に実験的な `rustc_codegen_clr` があります。

.NET には CLR、CTS、assembly metadata、MSBuild、NuGet、`dotnet` CLI、SDK-style project、debugging、tooling という成熟した基盤があります。

FerrumWeave の戦略は：

> **再発明する前に統合する。**

可能な改善は永続的な private fork にせず、upstream へ還元することを優先します。

---

# 初期アーキテクチャ

```text
FerrumWeave
│
├── CLR code generation
│   └── Rust MIR → CIL / metadata
├── CLR projection
│   └── .NET metadata → Rust-visible types and members
├── SDK
│   └── .rsproj / MSBuild / dotnet CLI integration
├── interoperability
│   └── Rust ↔ CTS semantics
├── code analysis
│   └── rust-analyzer awareness of CLR symbols
├── debugging
│   └── source mapping / PDB / stepping / locals
└── tooling
    └── templates, testing, publishing and packaging
```

目標リポジトリ構成は [Repository layout](docs/architecture/repository-layout.md) に記載されています。

このアーキテクチャは意図的に暫定です。実行可能な証拠が図より優先されます。

---

# 最初の証明

最初の意味ある milestone は意図的に小さくします。

```bash
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

```rust
use dotnet::System::*;

fn main() -> Result<()> {
    Console::WriteLine("Hello from FerrumWeave")?;
    Ok(())
}
```

FerrumWeave は有効な .NET assembly を生成し、CLR を通じて `System.Console.WriteLine` を実行できなければなりません。

このひとつの結果で `.rsproj`、MSBuild、`dotnet` CLI、`rustc`、Rust→CIL、CLR metadata、CTS interop、.NET BCL を同時に検証できます。

プロジェクトは、何も動かないうちに .NET 全体をモデリングするのではなく、実際に動く契約から縦方向に成長します。

---

# 長期的な成功

Rust が「何らかの CIL」を出力できるだけでは FerrumWeave の成功とは言えません。

より深い基準は：

> **.NET 開発者が、既存の .NET システムの中で Rust を本格的な別言語の選択肢として扱えるか？**

成熟した FerrumWeave では、次のようなシナリオが自然であるべきです。

```text
C# → Rust
Rust → C#
VB → Rust
Rust → F#
Rust → NuGet package
.NET project → .rsproj
.rsproj → .NET project
```

そして build、references、packages、types、exceptions、debugging、testing、tooling、publishing について、既存 .NET と同じ期待を持てるべきです。

---

# プロジェクト原則

### Rust を保つ

不要な Rust 方言を作らない。

### .NET を保つ

CLR、CTS、metadata、MSBuild、NuGet、その他既存のプラットフォーム契約を活用する。

### 段階的に相互運用する

20年前のアプリケーションが、新しい Rust コンポーネントを導入するためだけに全面 rewrite を要求されるべきではない。

### Safe Rust を優先する

`unsafe` は Rust の正当な一部ですが、通常の Rust 安全保証が意味を持つ領域を最大化する。

### 境界を明示する

Rust ownership と CLR garbage collection は別の仕組みです。難しい意味論の境界は意図的にモデル化する。

### 実用的なら upstream する

恒久的な fork より持続可能なエコシステムを優先する。

### 主張より証拠

コンパイラの正しさは、実行可能テスト、差分検証、conformance evidence、実アプリケーションによって示す。楽観的な図や互換率では示さない。

### 互換性は契約

FerrumWeave が対応を主張する .NET の挙動は、再現可能なテストで守られるべきです。

---

# 最初からオープンソース

FerrumWeave は公開で開発され、利用者の選択により次のいずれかで提供されます。

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

コンパイラを使うことで、生成されたプログラムに FerrumWeave のライセンスが強制されるべきではありません。

---

# Governance のビジョン

FerrumWeave は独立プロジェクトとして始まります。

本当に複数企業・複数コミュニティのエコシステムへ成長した場合、governance は元の作成者から独立できるべきです。

将来的に適切な中立 open-source foundation を居場所とすることは、所有権の喪失ではなく成功です。

そのため、透明な技術判断、明確な IP provenance、contributor の追跡可能性、移転可能なプロジェクト資産、開かれた governance を重視します。

現時点で foundation との提携はありません。

---

# Rust と .NET との関係

FerrumWeave は独立した実験プロジェクトです。

Microsoft、.NET Foundation、Rust Foundation、Rust Project のいずれとも、現時点で提携、スポンサー、公式承認の関係はありません。

「Rust」と「.NET」は、FerrumWeave が相互運用を目指す技術を正確に表すために使用しています。

---

# ステータス

**Pre-alpha / アーキテクチャ探索。**

このリポジトリは、完成した言語実装を主張する前に、問題、原則、目標契約、エンジニアリング境界を明確にすることから始めています。

最初の焦点は、信頼できる vertical slice です。

```text
Rust source
    ↓
.rsproj
    ↓
dotnet build / dotnet run
    ↓
CIL
    ↓
CLR
    ↓
System.Console.WriteLine
```

その後は、一つずつ契約を積み重ねます。

---

# 一文で言うと

> **FerrumWeave は Rust を .NET の第一級言語にし、組織が既存のソフトウェア、ライブラリ、言語、運用知識を捨てることなく、新しく重要なコンポーネントへ Rust の安全モデルを導入できるようにすることを目指します。**

---

## とても古い Hello World。とても現代的なコンパイラ。

何十年も前に書かれた Visual Basic アプリケーションが、今日書かれた Safe Rust コードを呼べる未来を目指します。

service boundary を挟むからではありません。

rewrite するからでもありません。

どちらかの言語がもう一方を装うからでもありません。

両方が、CLR がもともと言語間のために設計した共通の契約を話せるからです。

それが weave です。