<!--
doc-id: architecture.adr.0004-rust-as-dotnet-template-language
locale: zh-Hans
translation-of: docs/architecture/adr/0004-r10-rust-as-dotnet-template-language.md
source-revision: 0059e447c7b43940ac034f423b863d598f24b309
-->

<!-- ferrumweave-nav:start -->
[← 文档](../../README.zh-Hans.md) · [项目 README](../../../README.zh-Hans.md)
<!-- ferrumweave-nav:end -->

# ADR 0004 — 将 Rust 注册为 .NET 模板语言

[English](0004-r10-rust-as-dotnet-template-language.md) · [Deutsch](0004-r10-rust-as-dotnet-template-language.de.md) · [Español](0004-r10-rust-as-dotnet-template-language.es.md) · [Français](0004-r10-rust-as-dotnet-template-language.fr.md) · [Italiano](0004-r10-rust-as-dotnet-template-language.it.md) · [Português (Brasil)](0004-r10-rust-as-dotnet-template-language.pt-BR.md) · [Русский](0004-r10-rust-as-dotnet-template-language.ru.md) · **简体中文** · [日本語](0004-r10-rust-as-dotnet-template-language.ja.md)

- 状态：已为 R10 与 prerelease roadmap 接受
- 日期：2026-09-15
- Milestone：R10 — Developer experience / 0.1 alpha

## 背景

FerrumWeave 的目标是让 Rust 成为 .NET 生态中的一等语言。最初的 R10 原型暴露了一个专用 template 命令：

```console
dotnet new rust
```

这个命令在技术上很方便，但它把 Rust 建模成了项目类型。在 .NET template 模型中，`console`、`classlib`、`web`、测试项目、desktop 项目以及其他 workload family 才是 project template；源语言是独立的选择维度。

.NET developer 已经习惯同一项目家族通过语言来区分，而不是使用互不相关的 template 名称。FerrumWeave 应保留这种心智模型。

## 决策

FerrumWeave 的主要 template UX 为：

```console
dotnet new <template> -lang Rust
```

因此，规范的首次使用契约是：

```console
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

`dotnet new rust` 不是目标公共产品契约。

FerrumWeave templates 必须声明 Rust 为语言，并在 template engine 能够干净集成的情况下参与标准 .NET template family。实现必须由可执行的 template-resolution contract 验证，而不能假设只要匹配内置 template metadata 就足够。

安装 FerrumWeave 不得破坏或改变现有 C#、F# 或 Visual Basic template variant 的正常行为。

## Release 顺序

Template 支持按以下 capability release 引入：

- `0.1-alpha`：`console`、`classlib`、`xunit`、`nunit`、`mstest`、`web`、`webapi`；
- `0.2-alpha`：`mvc`、`winforms`；
- `0.3-beta`：`worker`；
- `0.4-beta`：`wpf`；
- `0.5-beta`：`grpc`；
- `0.6-beta`：`blazor`。

详细支持契约和稳定版 gate 维护在 [`docs/roadmap/template-release-plan.zh-Hans.md`](../../roadmap/template-release-plan.zh-Hans.md) 中。

## “支持”意味着可执行

仅仅成功生成目录不足以证明一个 template 已被支持。

对于 prerelease 兼容性声明，template 至少必须：

- 通过已安装的 FerrumWeave distribution 进行 scaffold；
- 在没有 FerrumWeave 源码 checkout 的情况下 restore 与 build；
- 执行默认的有意义 workflow（适用时为 `dotnet run` 或 `dotnet test`）；
- 到达 FerrumWeave 真实产品路径 `rustc -> CodegenBackend -> CIL/metadata -> CoreCLR`；
- 通过相关 CI/platform contract；
- 如实记录限制。

Framework-specific template 必须证明其区别于通用 console application 的 framework 行为。

## 1.0 gate

FerrumWeave 不会仅仅因为合成 conformance tests 通过就变成稳定版本。

在 `1.0.0` 之前，已承诺 prerelease 顺序中的每个 template family 都必须至少有一个真实项目成功使用相应 FerrumWeave template。

真实项目不能是 compiler fixture、template smoke test 或文档 sample。它必须具有真正的非测试用途，并提供适合该项目家族的可复现证据。

## 影响

积极影响：

- FerrumWeave 表现为 .NET 的扩展，而不是平行 CLI 生态；
- template 命令对 C#、F# 和 Visual Basic developer 保持自然；
- 产品 roadmap 可以通过 project-family contract 衡量；
- 未来可添加更多 template，而无需发明 FerrumWeave 特有的命令分类；
- 1.0 gate 将兼容性声明与真实世界证据绑定。

成本与约束：

- 假设 `dotnet new rust` 的 R10 原型 template metadata 和 tests 必须修订；
- template resolution 必须针对真实 .NET template engine 验证；
- framework template 在能够诚实声明支持之前，可能需要新的 compiler/runtime 能力；
- desktop 与 framework-specific template 保留各自的平台/build 约束。

## 验证

R10 至少必须添加可执行 falsifier，证明以下命令在安装 FerrumWeave 后能够共存：

```console
dotnet new console -lang Rust
dotnet new console -lang VB
dotnet new console -lang "F#"
```

同时还必须证明生成的 Rust 项目因果地使用真实 FerrumWeave product backend，而不是 legacy emitter 或源语言替代方案。

<!-- ferrumweave-backlinks:start -->
## 链入页面

- [FerrumWeave 文档](../../README.zh-Hans.md)
<!-- ferrumweave-backlinks:end -->
