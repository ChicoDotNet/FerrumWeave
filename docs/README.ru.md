<!--
translation-of: docs/README.md
locale: ru
source-revision: d50e9029e372521bd00a7b6c8cbc29e234c80009
-->

# Документация FerrumWeave

[English](README.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Português (Brasil)](README.pt-BR.md) · **Русский** · [简体中文](README.zh-Hans.md) · [日本語](README.ja.md)

[← README проекта](../README.ru.md)

Документация FerrumWeave организована вокруг реального пути читателя. Английские документы без суффикса локали остаются каноническими; если существует локализованная версия, навигация должна по возможности сохранять тот же язык.

> Документация поддерживается на нескольких языках, потому что интероперабельность — это также про людей.

## С чего начать

- [Начало работы](getting-started.ru.md) — целевой сценарий установки и первого проекта для активного этапа R10 / 0.1-alpha.
- [План выпуска шаблонов](roadmap/template-release-plan.ru.md) — стандартные семейства шаблонов .NET и значение слова «поддерживается».
- [README проекта](../README.ru.md) — видение, архитектура, текущее состояние и вход в сообщество.
- [Сайт проекта](https://chicodotnet.github.io/FerrumWeave/ru/) — публичный обзор и целевой developer experience.

## Архитектура

- [Структура репозитория](architecture/repository-layout.ru.md) — границы ownership и структура репозитория.
- [ADR 0004 — Rust как язык шаблонов .NET](architecture/adr/0004-r10-rust-as-dotnet-template-language.ru.md) — почему публичный контракт — `dotnet new <template> -lang Rust`.
- [Architecture Decision Records](architecture/adr/) — исторические архитектурные решения. *(английский)*

## Возможности и совместимость

- [Capability roadmap](roadmap/README.md) *(английский)*
- [Compatibility](compatibility/README.md) *(английский)*
- [CTS scalar mappings](compatibility/r04-cts-scalar-mappings.md) *(английский)*
- [Coverage policy](quality/coverage-policy.md) *(английский)*

## Сообщество и политика проекта

- [Contributing](../CONTRIBUTING.md) *(английский)*
- [Governance](../GOVERNANCE.md) *(английский)*
- [Support](../SUPPORT.md) *(английский)*
- [Security](../SECURITY.md) *(английский)*
- [Code of Conduct](../CODE_OF_CONDUCT.md) *(английский)*
- [DCO](../DCO.md) *(английский)*

## Интернационализация

- [Documentation internationalization](i18n.md) — локали, provenance, семантические правила и политика актуальности. *(пока английский)*

Текущие публичные локали:

`en` · `de` · `es` · `fr` · `it` · `pt-BR` · `ru` · `zh-Hans` · `ja`

Не каждый специализированный или исторический документ уже переведён. Если локализованной версии нет, используется канонический английский источник.

## Модель навигации

```text
README проекта
  → Главная документации
    → Раздел / документ
```

Документы с metadata `doc-id` получают сгенерированные ссылки назад на главную документации и README проекта в той же локали. `ferrumweave-docgraph` также генерирует и сертифицирует локализованный раздел «Что ссылается сюда» из Markdown-ссылок, написанных людьми; сгенерированные области сохраняются в репозитории и проверяются Documentation graph CI.
