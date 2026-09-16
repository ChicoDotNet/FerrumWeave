<!--
translation-of: docs/roadmap/template-release-plan.md
locale: ja
source-revision: 4ed0f1c8df9606bdd7b1135f186ef4bdd710c77a
-->

# FerrumWeave テンプレート リリース計画

[English](template-release-plan.md) · [Deutsch](template-release-plan.de.md) · [Español](template-release-plan.es.md) · [Français](template-release-plan.fr.md) · [Italiano](template-release-plan.it.md) · [Português (Brasil)](template-release-plan.pt-BR.md) · [Русский](template-release-plan.ru.md) · [简体中文](template-release-plan.zh-Hans.md) · **日本語**

FerrumWeave の developer experience roadmap では、Rust を独自のプロジェクト種別ではなく **.NET プロジェクト言語**として扱います。

主要な template UX は次のとおりです。

```console
dotnet new <template> -lang Rust
```

例：

```console
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

以前の `dotnet new rust` 形式は目標とする製品体験ではありません。console application は引き続き `console` project であり、FerrumWeave は .NET template engine に Rust を追加の言語選択肢として加えます。

## 「サポート済み」の意味

ファイルを scaffold できるだけでは、template をサポート済みとはみなしません。

FerrumWeave prerelease でサポート対象として掲示するには：

1. インストール済み FerrumWeave package から `dotnet new <template> -lang Rust` を実行し、期待される `.rsproj` と Rust source layout が生成されること。
2. 生成された project が FerrumWeave repository checkout なしで通常の .NET SDK workflow により restore/build できること。
3. 意味のある既定 workflow が成功すること。実行可能 application は `dotnet run`、test project は `dotnet test`。
4. 結果が `rustc -> FerrumWeave CodegenBackend -> CIL/metadata -> CoreCLR` を因果的に通ること。legacy source-pattern emitter や生成された C#/VB/F# 代替物は契約を満たしません。
5. cross-platform template は Windows と Linux で認証すること。WinForms と WPF は対応 Windows 環境で認証し、Linux 実行を示唆しません。
6. 既知の制限を明示的に文書化すること。

Sample や conformance fixture は prerelease contract の証拠にはなりますが、1.0 の real-project gate を満たしません。

## バージョン順序

| Release | Stage | 新たにサポートされる templates |
| --- | --- | --- |
| `0.1` | Alpha | `console`, `classlib`, `xunit`, `nunit`, `mstest`, `web`, `webapi` |
| `0.2` | Alpha | `mvc`, `winforms` |
| `0.3` | Beta | `worker` |
| `0.4` | Beta | `wpf` |
| `0.5` | Beta | `grpc` |
| `0.6` | Beta | `blazor` |
| `1.0` | Stable | すべての確約済み family に認証済み contract と少なくとも 1 つの成功した実プロジェクトがあること |

`0.1.0-alpha.1` や `0.5.0-beta.1` のような通常の prerelease suffix は使用できます。この表は能力境界を定義するもので、必要な prerelease 数を固定するものではありません。

## 0.1 alpha — コア .NET 言語体験

```console
dotnet new console  -lang Rust
dotnet new classlib -lang Rust
dotnet new xunit    -lang Rust
dotnet new nunit    -lang Rust
dotnet new mstest   -lang Rust
dotnet new web      -lang Rust
dotnet new webapi   -lang Rust
```

Release は最低限、実行可能 entry point と console output、他の .NET project から利用できる managed class library、xUnit/NUnit/MSTest による test discovery/execution、`dotnet run` で起動する ASP.NET Core host、`web` と `webapi` の最小 HTTP request/response、通常の restore/build/run/test・`ProjectReference`・NuGet、そして FerrumWeave source checkout のないクリーンな外部 directory からのインストールと利用を証明する必要があります。

## 0.2 alpha — application frameworks

```console
dotnet new mvc      -lang Rust
dotnet new winforms -lang Rust
```

`mvc` は実際の MVC request path を証明する必要があります。`winforms` は Windows 上で実際の Windows Forms lifecycle と対応 event/delegate interaction を証明します。

## 0.3 beta — background services

```console
dotnet new worker -lang Rust
```

Worker template は .NET Generic Host と実際の background-service lifecycle を実行し、単なる名前変更された console app であってはなりません。

## 0.4 beta — WPF

```console
dotnet new wpf -lang Rust
```

WPF は XAML、WindowsDesktop MSBuild targets、生成型/code-behind、application startup、event model により通常の CLR code generation 以上の要件を持つため、独立した能力境界です。

## 0.5 beta — gRPC

```console
dotnet new grpc -lang Rust
```

契約は生成された protobuf/gRPC build path と、少なくとも 1 回の実際の request/response round trip を含みます。

## 0.6 beta — Blazor

対象 .NET SDK version が提供する Blazor template surface を追加します。正確な short name は現行 SDK に従い、古い command を roadmap に固定しません。Beta contract は実際の rendered component path と必要な Razor/Blazor build integration を証明します。

## 1.0 stable gate — 実プロジェクトからの証拠

すべての conformance suite が GREEN でも、それだけで FerrumWeave は 1.0 にはなりません。

`1.0.0` より前に、**上記で確約したすべての template family に FerrumWeave を正常利用する実プロジェクトが少なくとも 1 つ必要**です。

実プロジェクトは：

- compiler fixture、conformance test、template smoke test、documentation-only sample ではないこと。
- 実際の application/library 目的を持つこと。
- 対応する Rust template family を通常の project structure で使うこと。
- 文書化された手順で再現可能に restore/build し、必要なら run/test できること。
- 想定 workload を代表する形で template を利用すること。
- FerrumWeave 固有の制限を隠さず記録すること。

Project は open source でも private evidence でも構いませんが、compatibility claim を正当化できる十分な再現可能な証拠が必要です。

Stable release の問いは：

> FerrumWeave は約束した全 project family を scaffold し認証できるか？

だけでなく：

> 約束した各 family は少なくとも 1 つの実 application で検証されたか？

でもあります。両方が「はい」のときだけ `1.0.0` gate を通過します。

## 互換性ルール

```console
dotnet new console
dotnet new console -lang VB
dotnet new console -lang "F#"
dotnet new console -lang Rust
```

FerrumWeave は .NET 言語エコシステムを拡張するものであり、既存の language variant を乗っ取ったり壊したりしてはいけません。
