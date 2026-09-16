<!--
translation-of: README.md
locale: zh-Hans
source-revision: 6fb75d076d59b4e1932b685e441487f6caedb36d
-->

[English](README.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · **简体中文** · [日本語](README.ja.md)

> FerrumWeave 使用多种语言维护文档，因为互操作不仅关乎技术，也关乎人。当某个译本暂时落后时，不带语言后缀的英文文档仍是规范来源。

<img src="assets/brand/hero/ferrumweave-readme-cover.png" alt="FerrumWeave — 将 Rust 带入 .NET 生态" width="100%" />

# FerrumWeave

**将 Rust 带入 .NET 语言生态。**

**项目资源：** [仓库结构](docs/architecture/repository-layout.md) · [模板发布计划](docs/roadmap/template-release-plan.md) · [项目网站](https://chicodotnet.github.io/FerrumWeave/zh-Hans/)

FerrumWeave 是一个实验性的开源项目，目标是让 Rust 成为 .NET 平台的一等语言：把 Rust 源代码编译成 .NET assembly，参与 Common Type System，使用既有 .NET 库，并与 C#、F#、Visual Basic 以及其他基于 CLR 的语言自然互操作。

长期目标中的开发体验应该非常自然：

```bash
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

Rust 代码在概念上可以类似：

```rust
use dotnet::System::*;
use dotnet::Result;

fn main() -> Result<()> {
    Console::WriteLine("Hello from Rust on .NET")?;
    Ok(())
}
```

并生成真正的 .NET assembly：

```text
HelloFerrum.dll
```

由 .NET runtime 执行。

> **FerrumWeave 仍处在这段旅程的起点。**
>
> 本 README 中的示例描述目标开发体验和架构方向，并不表示这些能力已经正式发布。

---

## 为什么是 FerrumWeave？

.NET 最持久的理念之一，从来不只是 C# 本身。

真正重要的是：**不同语言可以在同一个 runtime 上相遇。**

几十年来，开发者可以使用语法、理念和历史完全不同的语言编写软件，同时共享同一个平台：

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
- 以及更多语言

一种语言编写的库往往可以被另一种语言使用，因为真正重要的边界不是源语言。

真正的边界是 **Common Language Infrastructure**、**Common Type System**、assembly metadata 和 CLR。

FerrumWeave 提出一个简单的问题：

> **如果 Rust 也能加入这个语言家族，会怎样？**

不仅仅是通过 FFI 调用原生 Rust 库。

不仅仅是在 Rust 可执行程序中托管 CLR。

也不是把 Rust 翻译成 C#。

而是把 Rust 编译到同一个 CIL、assembly、metadata、类型、引用、包、tooling 与 runtime 互操作世界中——正是这些能力让多语言 .NET 成为可能。

---

## 为什么是 Rust？

Rust 为软件开发带来了一组不同的保证。

它的 ownership 模型、borrow checker、强类型系统、显式错误处理，以及对内存安全和并发安全的重视，可以让整类缺陷在进入生产环境之前就被发现。

Rust **并不会**让软件永远不出错。

逻辑仍可能错误。文件可能消失。网络可能故障。数据库可能包含坏数据。程序可能 panic。`unsafe` 也确实存在。

但 Rust 能够把一些重要的故障类型从：

```text
production
```

向：

```text
compile time
```

前移。

对于长期运行的业务系统，这种差异非常重要。

---

## 企业软件中的机会

大量 .NET 系统已经稳定创造价值十年、十五年，甚至二十五年。

它们可能包含：

```text
ERP.vbproj
Accounting.csproj
Reporting.fsproj
LegacyIntegration.vbproj
```

现代化讨论经常过早地从一句话开始：

> “我们应该重写它。”

FerrumWeave 建立在另一种思路上：

> **保留已经有效的部分，强化接下来新增的部分。**

设想在同一个系统中加入：

```text
RiskEngine.rsproj
```

现有 Visual Basic 应用无需消失。C# 领域模型无需重写。F# 报表引擎也无需引入新的集成协议。

未来：

```text
ERP.vbproj
     │
     ▼
RiskEngine.rsproj
     │
     ▼
Accounting.csproj
```

可以通过同一个 .NET 类型系统互相通信。

几十年前编写的 Visual Basic 应用可以调用今天新写的 Rust 代码。Rust 可以使用 C# 编写的领域 assembly。F# 可以使用 Rust 实现的类型。

迁移单位因此变成 **组件**，而不是整个应用。

这就是愿景。

---

# 目标

FerrumWeave 希望实现：

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

Rust 的目标编译路径大致为：

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

Rust 仍然是 Rust。

CLR 仍然是 CLR。

FerrumWeave 应该连接两者，而不是无谓地重新发明任何一方。

---

# FerrumWeave 不是什么

FerrumWeave **不是**：

### 一个新的类 Rust 语言

目标是保留 Rust 本身，并利用现有 Rust 编译器生态。

```rust
match
traits
lifetimes
ownership
borrowing
async
Result<T, E>
```

这些都应继续是 Rust 的概念，而不是在另一个编译器中重新实现的近似版本。

### 包在 `dotnet` 外面的 Rust wrapper

从 MSBuild target 运行 Cargo 可能有价值，但仅靠这一点并不能让 Rust 成为 .NET 语言。

FerrumWeave 的目标更深。

### 原生 FFI 生成器

原生互操作仍然重要，但目标不是：

```text
C#
 ↓
P/Invoke
 ↓
Rust native DLL
```

目标是：

```text
C#
  ╲
   CLR
  ╱
Rust
```

### .NET 的替代品

FerrumWeave 正是因为 .NET 生态有价值才存在。目标是扩展语言选择，而不是替换平台。

### 原生 Rust 的替代品

直接把 Rust 编译成本机代码永远有很好的理由。CLR target 只是额外的 deployment 和互操作选项。

---

# 目标开发体验

未来，Rust 项目应该能自然地存在于 .NET solution 中：

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

熟悉的命令应该继续保持熟悉：

```bash
dotnet restore
dotnet build
dotnet run
dotnet test
dotnet publish
dotnet pack
```

项目最终可能类似：

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

当 Rust 依赖需要 Cargo 和 crates.io 时，它们应该继续发挥作用。NuGet 和 MSBuild 也应继续负责它们已经擅长的 .NET 依赖工作。

FerrumWeave 的责任是连接这些生态，而不是假装其中任何一个不存在。

---

# 从 Rust 使用 .NET 库

一个核心目标是让 .NET API 自然参与 Rust 代码。

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

关键并不是示例中的精确语法，而是：

> `System.Console`、`System.String`、`System.IO.File` 和用户定义的 .NET 类型，应被理解为 CLR 类型与成员，而不是隐藏在手工维护 FFI 层背后的不透明原生库。

同样的原则最终也应适用于 NuGet package 和 ProjectReference。

---

# 从其他 .NET 语言使用 Rust 库

互操作必须双向成立。

Rust 应该能够定义面向 CLR 的公共类型，让其他 .NET 语言直接使用。

Rust：

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

C#：

```csharp
var engine = new RiskEngine();
var score = engine.Calculate(customer);
```

Visual Basic：

```vb
Dim engine = New RiskEngine()
Dim score = engine.Calculate(customer)
```

F#：

```fsharp
let engine = RiskEngine()
let score = engine.Calculate(customer)
```

不同的源语言，一个 runtime contract。

这就是 FerrumWeave 最终希望达到的互操作标准。

---

# Rust 类型与 CLR 类型

Rust 与 CLR 的对象和内存模型从根本上不同。这个差异不应该被掩盖。

Rust 有：

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

CLR 有：

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

FerrumWeave 不应为了让二者看起来一样而削弱任何一方，而应定义有原则、可解释的映射。

有些映射很自然：

```text
System.Int32   ↔ i32
System.Int64   ↔ i64
System.Boolean ↔ bool
System.Double  ↔ f64
```

另一些则需要明确语义：

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

正确设计这些语义，是本项目最核心的工程挑战之一。

---

# 不放弃互操作的安全性

企业软件很少有从零开始的奢侈。

组织已经拥有正在工作的应用、数据库、业务规则、API、package、framework、开发团队与运维知识。

如果采用更安全的语言不要求放弃这些资产，那么采用成本会低得多。

因此 FerrumWeave 探索如下命题：

> **把 Rust 的安全模型带到新的、关键的 .NET 组件中，同时保留与现有 .NET 投资的互操作性。**

---

# 为什么叫 FerrumWeave？

**Ferrum** 在拉丁语中意为铁。Rust 是铁的氧化。

**Weave** 表示把独立的线交织成连接结构。

```text
Ferrum
   │
   └── iron → rust

Weave
   │
   └── interconnection → network → ecosystem
```

合在一起：

> **FerrumWeave 表示 Rust 被编织进 .NET 生态。**

这个名称刻意独立于 Rust 与 .NET 商标。

---

# 建立在现有工作之上

FerrumWeave 不打算从头再写一个 Rust 编译器。

Rust 生态已经提供了极具价值的基础设施：

- `rustc`：解析、类型检查、borrow checking、MIR 与语言语义；
- `rust-analyzer`：现代 Rust 代码分析和开发 tooling。

Rust→CLR 方向也已有重要 prior art，尤其是实验性的 `rustc_codegen_clr`。

.NET 同样已经提供成熟的 CLR、CTS、assembly metadata、MSBuild、NuGet、`dotnet` CLI、SDK-style project、debugging 与 tooling。

因此 FerrumWeave 的策略是：

> **先集成，再重新发明。**

可以 upstream 的改进，应尽量 upstream，而不是永久维护私有 fork。

---

# 初期架构

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

目标仓库结构记录在 [Repository layout](docs/architecture/repository-layout.md) 中。

此架构刻意保持暂定。可执行证据优先于图表。

---

# 第一个证明

第一个有意义的 milestone 会刻意保持很小：

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

FerrumWeave 应生成有效的 .NET assembly，并通过 CLR 执行 `System.Console.WriteLine`。

这一个结果就能同时验证 `.rsproj`、MSBuild、`dotnet` CLI、`rustc`、Rust→CIL、CLR metadata、CTS interop 与 .NET BCL。

项目会围绕能够运行的契约纵向增长，而不是在任何东西运行之前就试图建模整个 .NET 生态。

---

# 长期成功标准

FerrumWeave 不会仅仅因为 Rust 能输出“一些 CIL”就算成功。

更深层的标准是：

> **.NET 开发者能否在已有 .NET 系统中，把 Rust 当作另一个严肃的一等语言选择？**

成熟的 FerrumWeave 应让这些场景变得自然：

```text
C# → Rust
Rust → C#
VB → Rust
Rust → F#
Rust → NuGet package
.NET project → .rsproj
.rsproj → .NET project
```

并拥有开发者对 build、references、packages、types、exceptions、debugging、testing、tooling 和 publishing 已经熟悉的预期。

---

# 项目原则

### 保留 Rust

避免制造不必要的 Rust 方言。

### 保留 .NET

使用 CLR、CTS、metadata、MSBuild、NuGet 与已有平台契约。

### 渐进式互操作

一个运行了二十年的应用，不应该为了获得一个新 Rust 组件的收益而被迫重写。

### 优先 Safe Rust

`unsafe` 是 Rust 的合法组成部分，但项目应尽可能扩大普通 Rust 安全保证仍然有效的范围。

### 明确边界

Rust ownership 与 CLR garbage collection 是不同系统。困难的语义边界应该被明确建模，而不是用方便的语法掩盖。

### 可行时 upstream

可持续生态优于永久 fork。

### 证据先于声明

编译器正确性应来自可执行测试、差分验证、conformance evidence 和真实应用，而不是架构图或乐观的兼容率。

### 兼容性是一份契约

FerrumWeave 声称支持的 .NET 行为，都应该有可重复测试保护。

---

# 从第一天开始开源

FerrumWeave 公开开发，并可由用户选择使用：

- [MIT License](LICENSE-MIT)；或
- [Apache License, Version 2.0](LICENSE-APACHE)。

使用编译器不应把 FerrumWeave 的许可证强加给被编译的软件。

---

# 治理愿景

FerrumWeave 从独立项目开始。

如果未来形成真正的多公司、多社区生态，治理结构应该能够脱离最初创建者而独立存在。

长期进入合适的中立开源基金会会被视为成功，而不是失去所有权。

因此项目需要透明的技术决策、清晰的知识产权来源、贡献者可追溯性、可转移项目资产以及随社区成长而开放的治理。

目前没有任何基金会隶属关系。

---

# 与 Rust 和 .NET 的关系

FerrumWeave 是独立实验项目。

目前与 Microsoft、.NET Foundation、Rust Foundation 或 Rust Project 均无隶属、赞助或官方背书关系。

“Rust”和“.NET”仅用于准确描述 FerrumWeave 希望互操作的技术。

---

# 状态

**Pre-alpha / 架构探索阶段。**

仓库刻意先明确问题、原则、目标契约和工程边界，再去声明完整的语言实现。

第一个重点不是功能数量，而是可信的 vertical slice：

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

之后，一个契约一个契约地推进。

---

# 一句话说明这个想法

> **FerrumWeave 希望让 Rust 成为 .NET 的一等语言，使组织能够在新的关键组件中引入 Rust 的安全模型，同时不放弃已经拥有的软件、库、语言与运维知识。**

---

## 一个非常古老的 Hello World，一个非常现代的编译器。

几十年前写下的 Visual Basic 应用，未来应该可以调用今天编写的 Safe Rust 代码。

不是通过 service boundary。

不是通过 rewrite。

也不是因为某种语言假装成另一种语言。

而是因为二者都能使用 CLR 从一开始就为语言之间提供的共同契约。

这就是 weave。