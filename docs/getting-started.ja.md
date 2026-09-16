<!--
translation-of: docs/getting-started.md
locale: ja
source-revision: e4bcd0d4b4c4b73232823896144adf098b30758b
-->

# FerrumWeave 0.1 alpha — はじめに

[English](getting-started.md) · [Deutsch](getting-started.de.md) · [Español](getting-started.es.md) · [Français](getting-started.fr.md) · [Italiano](getting-started.it.md) · [Português (Brasil)](getting-started.pt-BR.md) · [Русский](getting-started.ru.md) · [简体中文](getting-started.zh-Hans.md) · **日本語**

> **R10 作業契約:** この文書は FerrumWeave 0.1 alpha がリリース前に認証すべき外部体験を説明します。CI でまだ GREEN ではないコマンドは製品目標であり、リリース済み機能の主張ではありません。

FerrumWeave は通常の .NET project template に **言語選択として Rust** を追加します。中心となる UX は `dotnet new <template> -lang Rust` であり、独自 project type `dotnet new rust` ではありません。

リリース順序と template support は [`template-release-plan.ja.md`](roadmap/template-release-plan.ja.md)、アーキテクチャ判断は [ADR 0004](architecture/adr/0004-r10-rust-as-dotnet-template-language.md) にあります。

## 0.1-alpha の初回実行目標

```console
dotnet new install FerrumWeave.Sdk::0.1.0-alpha.1
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

`FerrumWeave.Sdk` が MSBuild SDK と project templates の現在の package identity です。後続 ADR が変更しない限り、未公開の第二 template package を要求してはいけません。

Rust project は因果的に次の経路を通ります。

```text
Rust source
  -> rustc
  -> FerrumWeave CodegenBackend
  -> CIL / CLR metadata
  -> CoreCLR
```

初回実行契約は FerrumWeave source checkout のないクリーンな外部ディレクトリから動作する必要があります。

## 0.1-alpha template surface

```console
dotnet new console  -lang Rust
dotnet new classlib -lang Rust
dotnet new xunit    -lang Rust
dotnet new nunit    -lang Rust
dotnet new mstest   -lang Rust
dotnet new web      -lang Rust
dotnet new webapi   -lang Rust
```

template はファイル生成だけでは support 済みではありません。restore/build と既定の意味ある workflow（`dotnet run` または `dotnet test`）を実際の FerrumWeave backend 経由で完了する必要があります。

## FerrumWeave MSBuild SDK

```xml
<Project Sdk="FerrumWeave.Sdk/0.1.0-alpha.1"><PropertyGroup><TargetFramework>net10.0</TargetFramework></PropertyGroup></Project>
```

```console
dotnet restore
```

SDK package には `Sdk.props`、`Sdk.targets`、project-template content が含まれ、利用者が手動でコピーする必要はありません。

## ローカル prerelease のテスト

```xml
<?xml version="1.0" encoding="utf-8"?>
<configuration><packageSources><clear /><add key="ferrumweave-local" value="../artifacts/packages" /></packageSources></configuration>
```

公開 package と同じ restore/build/test/run コマンドを使い、maintainer 固有知識を隠れた前提にしてはいけません。

## 互換性 invariant

```console
dotnet new console
dotnet new console -lang VB
dotnet new console -lang "F#"
dotnet new console -lang Rust
```

FerrumWeave は既存 .NET SDK の挙動を保ったまま Rust を追加します。
