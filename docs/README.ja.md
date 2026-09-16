<!--
translation-of: docs/README.md
locale: ja
source-revision: 22aef36588bd35a433ff9564ab947703afd39c4f
-->

# FerrumWeave ドキュメント

[English](README.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [简体中文](README.zh-Hans.md) · **日本語**

[← プロジェクト README](../README.ja.md)

FerrumWeave のドキュメントは、読者が実際にたどる経路を基準に構成します。locale 接尾辞のない英語ドキュメントが正本であり、ローカライズ版が存在する場合は、できるだけ同じ言語のまま移動できるようにします。

> ドキュメントを複数言語で維持するのは、相互運用性が人にも関わるからです。

## ここから始める

- [はじめに](getting-started.ja.md) — 現在の R10 / 0.1-alpha マイルストーンにおけるインストールと最初のプロジェクトの目標体験。
- [テンプレートのリリース計画](roadmap/template-release-plan.ja.md) — 対象となる標準 .NET テンプレート群と「サポート済み」の定義。
- [プロジェクト README](../README.ja.md) — ビジョン、アーキテクチャ方針、現在の状態、コミュニティへの入口。
- [プロジェクトサイト](https://chicodotnet.github.io/FerrumWeave/ja/) — 公開概要と目標とする開発体験。

## アーキテクチャ

次のページは現在、正本の英語版を使用します。

- [Repository layout](architecture/repository-layout.md) — ownership 境界とリポジトリ構造。
- [ADR 0004 — Rust as a .NET template language](architecture/adr/0004-r10-rust-as-dotnet-template-language.md) — 公開契約が `dotnet new <template> -lang Rust` である理由。
- [Architecture Decision Records](architecture/adr/) — 過去のアーキテクチャ判断とその根拠。

## 機能と互換性

- [Capability roadmap](roadmap/README.md) *（英語）*
- [Compatibility](compatibility/README.md) *（英語）*
- [CTS scalar mappings](compatibility/r04-cts-scalar-mappings.md) *（英語）*
- [Coverage policy](quality/coverage-policy.md) *（英語）*

## コミュニティとプロジェクト方針

- [Contributing](../CONTRIBUTING.md) *（英語）*
- [Governance](../GOVERNANCE.md) *（英語）*
- [Support](../SUPPORT.md) *（英語）*
- [Security](../SECURITY.md) *（英語）*
- [Code of Conduct](../CODE_OF_CONDUCT.md) *（英語）*
- [DCO](../DCO.md) *（英語）*

## 国際化

- [Documentation internationalization](i18n.md) — locale 命名、翻訳 provenance、意味上のルール、staleness policy。*（現在は英語）*

現在の公開 locale:

`en` · `de` · `es` · `fr` · `it` · `pt-BR` · `ru` · `zh-Hans` · `ja`

専門的または歴史的なドキュメントのすべてが翻訳済みではありません。ローカライズ版がない場合は、英語の正本を fallback とします。

## ナビゲーションモデル

```text
プロジェクト README
  → ドキュメントホーム
    → セクション / ドキュメント
```

ローカライズされたドキュメントは同じ言語のドキュメントホームへ戻れるようにします。次の docgraph 増分で「ここにリンクしているページ」を自動生成・検証し、backlink を手作業で維持しないようにします。
