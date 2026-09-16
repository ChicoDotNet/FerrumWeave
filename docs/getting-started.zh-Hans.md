<!--
translation-of: docs/getting-started.md
locale: zh-Hans
source-revision: e4bcd0d4b4c4b73232823896144adf098b30758b
-->

# FerrumWeave 0.1 alpha — 入门

[English](getting-started.md) · [Deutsch](getting-started.de.md) · [Español](getting-started.es.md) · [Français](getting-started.fr.md) · [Italiano](getting-started.it.md) · [Português (Brasil)](getting-started.pt-BR.md) · [Русский](getting-started.ru.md) · **简体中文** · [日本語](getting-started.ja.md)

> **R10 工作契约：** 本文描述 FerrumWeave 0.1 alpha 在发布前必须认证的外部体验。尚未在 CI 中变为 GREEN 的命令是产品目标，不代表已发布能力。

FerrumWeave 将普通 .NET 项目模板扩展为支持 **Rust 作为语言选择**。因此主要体验是 `dotnet new <template> -lang Rust`，而不是单独的 `dotnet new rust` 项目类型。

发布顺序和模板支持定义见 [`template-release-plan.zh-Hans.md`](roadmap/template-release-plan.zh-Hans.md)，架构决策见 [ADR 0004](architecture/adr/0004-r10-rust-as-dotnet-template-language.md)。

## 0.1-alpha 首次运行目标

```console
dotnet new install FerrumWeave.Sdk::0.1.0-alpha.1
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

`FerrumWeave.Sdk` 是当前 MSBuild SDK 与项目模板的 package identity；除非后续 ADR 明确修改 packaging 决策，否则不得要求第二个未发布模板 package。

Rust 项目必须因果地经过：

```text
Rust source
  -> rustc
  -> FerrumWeave CodegenBackend
  -> CIL / CLR metadata
  -> CoreCLR
```

首次运行契约必须能在没有 FerrumWeave 源码 checkout 的干净外部目录中工作。

## 0.1-alpha 模板范围

```console
dotnet new console  -lang Rust
dotnet new classlib -lang Rust
dotnet new xunit    -lang Rust
dotnet new nunit    -lang Rust
dotnet new mstest   -lang Rust
dotnet new web      -lang Rust
dotnet new webapi   -lang Rust
```

模板只有在 restore/build 以及默认有意义 workflow（`dotnet run` 或 `dotnet test`）通过真实 FerrumWeave backend 时才算支持。

## FerrumWeave MSBuild SDK

```xml
<Project Sdk="FerrumWeave.Sdk/0.1.0-alpha.1"><PropertyGroup><TargetFramework>net10.0</TargetFramework></PropertyGroup></Project>
```

```console
dotnet restore
```

SDK package 包含 `Sdk.props`、`Sdk.targets` 和项目模板内容，使用者不应手工复制这些文件。

## 测试本地 prerelease

```xml
<?xml version="1.0" encoding="utf-8"?>
<configuration><packageSources><clear /><add key="ferrumweave-local" value="../artifacts/packages" /></packageSources></configuration>
```

应使用与正式 package 相同的 restore/build/test/run 命令，不得依赖 maintainer 专有知识。

## 兼容性不变量

```console
dotnet new console
dotnet new console -lang VB
dotnet new console -lang "F#"
dotnet new console -lang Rust
```

FerrumWeave 的目标是增加 Rust 语言选择，同时保留 .NET SDK 的既有行为。
