<!--
doc-id: architecture.adr.0004-rust-as-dotnet-template-language
locale: ja
translation-of: docs/architecture/adr/0004-r10-rust-as-dotnet-template-language.md
source-revision: 0059e447c7b43940ac034f423b863d598f24b309
-->

# ADR 0004 — Rust を .NET template language として登録する

[English](0004-r10-rust-as-dotnet-template-language.md) · [Deutsch](0004-r10-rust-as-dotnet-template-language.de.md) · [Español](0004-r10-rust-as-dotnet-template-language.es.md) · [Français](0004-r10-rust-as-dotnet-template-language.fr.md) · [Italiano](0004-r10-rust-as-dotnet-template-language.it.md) · [Português (Brasil)](0004-r10-rust-as-dotnet-template-language.pt-BR.md) · [Русский](0004-r10-rust-as-dotnet-template-language.ru.md) · [简体中文](0004-r10-rust-as-dotnet-template-language.zh-Hans.md) · **日本語**

- Status: R10 と prerelease roadmap 向けに Accepted
- Date: 2026-09-15
- Milestone: R10 — Developer experience / 0.1 alpha

## Context

FerrumWeave の目的は、Rust を .NET ecosystem の first-class language にすることです。初期の R10 prototype は専用の template command を公開していました。

```console
dotnet new rust
```

このコマンドは技術的には便利ですが、Rust を project type のように扱ってしまいます。.NET template model では `console`、`classlib`、`web`、test project、desktop project、その他の workload family が project template であり、source language は別の選択軸です。

.NET developer は、同等の project family が無関係な template 名ではなく language によって区別されることを既に期待しています。FerrumWeave はその mental model を維持すべきです。

## Decision

FerrumWeave の primary template UX は次のとおりです。

```console
dotnet new <template> -lang Rust
```

したがって canonical first-use contract は次のようになります。

```console
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

`dotnet new rust` は公開製品 contract の目標ではありません。

FerrumWeave templates は Rust を language として宣言し、template engine がきれいに統合できる範囲で通常の .NET template family に参加しなければなりません。実装は、組み込み template と metadata が似ているから十分だと仮定するのではなく、実行可能な template-resolution contract で検証します。

FerrumWeave のインストールによって既存の C#、F#、Visual Basic template variant の通常動作を壊したり変更したりしてはいけません。

## Release sequence

Template support は次の capability release で導入します。

- `0.1-alpha`: `console`, `classlib`, `xunit`, `nunit`, `mstest`, `web`, `webapi`;
- `0.2-alpha`: `mvc`, `winforms`;
- `0.3-beta`: `worker`;
- `0.4-beta`: `wpf`;
- `0.5-beta`: `grpc`;
- `0.6-beta`: `blazor`.

詳細な support contract と stable-release gate は [`docs/roadmap/template-release-plan.ja.md`](../../roadmap/template-release-plan.ja.md) で管理します。

## Supported とは実行可能であること

ディレクトリが生成されるだけでは、template が supported である証拠にはなりません。

Prerelease compatibility claim のためには、template は少なくとも次を満たす必要があります。

- インストール済み FerrumWeave distribution から scaffold できること。
- FerrumWeave source checkout なしで restore と build ができること。
- 既定の意味ある workflow（適切な `dotnet run` または `dotnet test`）を実行できること。
- FerrumWeave の実際の product path `rustc -> CodegenBackend -> CIL/metadata -> CoreCLR` に到達すること。
- 関連する CI/platform contract を通過すること。
- 制限事項を正直に文書化すること。

Framework-specific template は、generic console application と区別される framework behavior を証明しなければなりません。

## 1.0 gate

Synthetic conformance test が通っただけで FerrumWeave が stable になるわけではありません。

`1.0.0` より前に、確定した prerelease sequence のすべての template family に、その FerrumWeave template を実際に使用して成功している real project が少なくとも 1 つ必要です。

Real project は compiler fixture、template smoke test、documentation sample ではありません。本物の非 test 目的を持ち、その project family に適した再現可能な evidence が必要です。

## Consequences

Positive consequences:

- FerrumWeave は並行 CLI ecosystem ではなく .NET の extension として振る舞う。
- template command は C#、F#、Visual Basic developer にとって自然なままになる。
- product roadmap を project-family contract で測定できる。
- FerrumWeave 固有の command taxonomy を発明せずに将来の template を追加できる。
- 1.0 gate が compatibility claim を real-world evidence に結びつける。

Costs and constraints:

- `dotnet new rust` を前提とする R10 prototype の template metadata と tests は修正が必要。
- template resolution は実際の .NET template engine に対して検証する必要がある。
- framework template を正直に supported とする前に、新しい compiler/runtime capability が必要になる場合がある。
- desktop および framework-specific template は固有の platform/build constraint を維持する。

## Validation

R10 は少なくとも次のコマンドが FerrumWeave インストール後に共存することを実行可能な falsifier で証明しなければなりません。

```console
dotnet new console -lang Rust
dotnet new console -lang VB
dotnet new console -lang "F#"
```

さらに生成された Rust project が legacy emitter や source-language substitution ではなく、実際の FerrumWeave product backend を因果的に使用していることを証明します。
