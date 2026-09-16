<!--
translation-of: docs/README.md
locale: zh-Hans
source-revision: d50e9029e372521bd00a7b6c8cbc29e234c80009
-->

# FerrumWeave 文档

[English](README.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · **简体中文** · [日本語](README.ja.md)

[← 项目 README](../README.zh-Hans.md)

FerrumWeave 文档按照读者实际会走的路径来组织。没有 locale 后缀的英文文档是规范来源；当本地化版本存在时，导航应尽可能让读者继续使用同一种语言。

> 文档维护多种语言，因为互操作性也关乎人。

## 从这里开始

- [开始使用](getting-started.zh-Hans.md) — 当前 R10 / 0.1-alpha 里程碑的安装与首个项目目标体验。
- [模板发布计划](roadmap/template-release-plan.zh-Hans.md) — FerrumWeave 计划支持的标准 .NET 模板家族，以及“支持”的定义。
- [项目 README](../README.zh-Hans.md) — 项目愿景、架构方向、当前状态与社区入口。
- [项目网站](https://chicodotnet.github.io/FerrumWeave/zh-Hans/) — 公开概览与目标开发体验。

## 架构

- [仓库结构](architecture/repository-layout.zh-Hans.md) — ownership 边界与仓库结构。
- [ADR 0004 — 将 Rust 注册为 .NET 模板语言](architecture/adr/0004-r10-rust-as-dotnet-template-language.zh-Hans.md) — 为什么公开契约是 `dotnet new <template> -lang Rust`。
- [Architecture Decision Records](architecture/adr/) — 历史架构决策及其理由。*（英文）*

## 能力与兼容性

- [Capability roadmap](roadmap/README.md) *（英文）*
- [Compatibility](compatibility/README.md) *（英文）*
- [CTS scalar mappings](compatibility/r04-cts-scalar-mappings.md) *（英文）*
- [Coverage policy](quality/coverage-policy.md) *（英文）*

## 社区与项目政策

- [Contributing](../CONTRIBUTING.md) *（英文）*
- [Governance](../GOVERNANCE.md) *（英文）*
- [Support](../SUPPORT.md) *（英文）*
- [Security](../SECURITY.md) *（英文）*
- [Code of Conduct](../CODE_OF_CONDUCT.md) *（英文）*
- [DCO](../DCO.md) *（英文）*

## 国际化

- [Documentation internationalization](i18n.md) — locale 命名、翻译 provenance、语义规则和过期策略。*（目前为英文）*

当前公开 locale：

`en` · `de` · `es` · `fr` · `it` · `pt-BR` · `ru` · `zh-Hans` · `ja`

并非所有专业或历史文档都已经翻译。当本地化版本不存在时，规范英文文档作为 fallback。

## 导航模型

```text
项目 README
  → 文档首页
    → 分区 / 文档
```

带有 `doc-id` metadata 的文档会自动获得返回同一 locale 文档首页与项目 README 的链接。`ferrumweave-docgraph` 还会根据人工编写的 Markdown 链接生成并验证本地化的“链入页面” backlinks；生成区域会提交到仓库，并由 Documentation graph CI 检查是否保持最新。
