<!--
translation-of: docs/roadmap/template-release-plan.md
locale: zh-Hans
source-revision: 4ed0f1c8df9606bdd7b1135f186ef4bdd710c77a
-->

# FerrumWeave 模板发布计划

[English](template-release-plan.md) · [Deutsch](template-release-plan.de.md) · [Español](template-release-plan.es.md) · [Français](template-release-plan.fr.md) · [Italiano](template-release-plan.it.md) · [Português (Brasil)](template-release-plan.pt-BR.md) · [Русский](template-release-plan.ru.md) · **简体中文** · [日本語](template-release-plan.ja.md)

FerrumWeave 的开发者体验路线图把 Rust 视为一种 **.NET 项目语言**，而不是独立的项目类型。

主要的 template UX 是：

```console
dotnet new <template> -lang Rust
```

例如：

```console
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

早期的 `dotnet new rust` 形式不是目标产品体验。console 应用仍然是 `console` 项目；FerrumWeave 只是让 Rust 成为 .NET template engine 中另一种语言选择。

## “支持”意味着什么

仅仅能够生成文件，并不代表一个 template 已被支持。

要在 FerrumWeave prerelease 中声明某个 template 为受支持，必须满足：

1. 从已安装的 FerrumWeave package 执行 `dotnet new <template> -lang Rust` 时，必须生成预期的 `.rsproj` 与 Rust source layout。
2. 生成的项目必须通过标准 .NET SDK workflow 完成 restore 和 build，且不能依赖 FerrumWeave 仓库 checkout。
3. 默认的有效 workflow 必须成功：可执行项目使用 `dotnet run`，测试项目使用 `dotnet test`。
4. 行为必须因果地通过 `rustc -> FerrumWeave CodegenBackend -> CIL/metadata -> CoreCLR`。旧式 source-pattern emitter 或生成的 C#/VB/F# 替代品都不能满足此契约。
5. 跨平台 template 必须在 Windows 和 Linux 上认证。WinForms 与 WPF 只在受支持的 Windows 环境中认证，不暗示 Linux 执行能力。
6. 已知限制必须明确记录。

Sample 和 conformance fixture 可以证明 prerelease contract，但不能代替 1.0 的真实项目 gate。

## 版本顺序

| Release | 阶段 | 新增为受支持的 templates |
| --- | --- | --- |
| `0.1` | Alpha | `console`, `classlib`, `xunit`, `nunit`, `mstest`, `web`, `webapi` |
| `0.2` | Alpha | `mvc`, `winforms` |
| `0.3` | Beta | `worker` |
| `0.4` | Beta | `wpf` |
| `0.5` | Beta | `grpc` |
| `0.6` | Beta | `blazor` |
| `1.0` | Stable | 所有承诺的 template family 都有已认证契约，并且至少成功用于一个真实项目 |

版本可以使用 `0.1.0-alpha.1`、`0.5.0-beta.1` 等正常 prerelease 后缀；上表定义的是能力边界，而不是到达该边界所需的精确 prerelease 数量。

## 0.1 alpha — 核心 .NET 语言体验

```console
dotnet new console  -lang Rust
dotnet new classlib -lang Rust
dotnet new xunit    -lang Rust
dotnet new nunit    -lang Rust
dotnet new mstest   -lang Rust
dotnet new web      -lang Rust
dotnet new webapi   -lang Rust
```

此 release 至少必须证明：可执行 entry point 与 console output；可被其他 .NET 项目消费的 managed class library；通过 xUnit、NUnit、MSTest 发现并执行测试；使用 `dotnet run` 启动 ASP.NET Core host；`web` 与 `webapi` 的最小 HTTP request/response 路径；正常的 restore/build/run/test、`ProjectReference` 与 NuGet 行为；以及在不包含 FerrumWeave source checkout 的干净外部目录中安装和使用。

## 0.2 alpha — 应用框架

```console
dotnet new mvc      -lang Rust
dotnet new winforms -lang Rust
```

`mvc` 必须证明真实 MVC request path，而不仅是 ASP.NET Core hosting。`winforms` 必须在 Windows 上证明真实 Windows Forms lifecycle 与受支持的 event/delegate 交互。

## 0.3 beta — 后台服务

```console
dotnet new worker -lang Rust
```

Worker template 必须实际使用 .NET Generic Host 和真实 background-service lifecycle，而不是简单改名的 console app。

## 0.4 beta — WPF

```console
dotnet new wpf -lang Rust
```

WPF 是独立的能力边界，因为 XAML、WindowsDesktop MSBuild targets、生成类型/code-behind、应用 startup 与事件模型带来额外要求。

## 0.5 beta — gRPC

```console
dotnet new grpc -lang Rust
```

契约必须覆盖生成的 protobuf/gRPC build path，并至少完成一次真实 request/response round trip。

## 0.6 beta — Blazor

加入目标 .NET SDK 版本所支持的 Blazor template surface。准确 short name 必须跟随当代 SDK，而不是把已过时的命令固化在 roadmap 中。Beta contract 必须证明真实 rendered component path 以及所需的 Razor/Blazor build integration。

## 1.0 stable gate — 来自真实项目的证据

即使所有 conformance suite 都是绿色，FerrumWeave 也不会因此自动达到 1.0。

在 `1.0.0` 之前，**上面承诺的每个 template family 都必须至少有一个真实项目成功使用 FerrumWeave**。

真实项目必须：

- 不是 compiler fixture、conformance test、template smoke test 或仅用于文档的 sample；
- 具有真正的应用或 library 目的；
- 在正常项目结构中使用对应的 Rust template family；
- 能根据文档可重复地 restore/build，并在适用时 run/test；
- 以代表真实 workload 的方式使用该 template；
- 明确记录 FerrumWeave 特有的限制。

项目可以是 open source，也可以通过私有方式提供证据，但必须有足够可复现的信息来支撑兼容性声明。

因此 stable release 不只问：

> FerrumWeave 能否 scaffold 并认证所有承诺的项目 family？

还要问：

> 每个承诺的 family 是否都在至少一个真实应用中经受了验证？

只有两个答案都为“是”时，FerrumWeave 才通过 `1.0.0` gate。

## 兼容性规则

```console
dotnet new console
dotnet new console -lang VB
dotnet new console -lang "F#"
dotnet new console -lang Rust
```

FerrumWeave 是对 .NET 语言生态的扩展；它不能劫持或破坏已有语言变体。
