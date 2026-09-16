<!--
translation-of: docs/README.md
locale: ru
source-revision: 22aef36588bd35a433ff9564ab947703afd39c4f
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

Следующие страницы пока используют каноническую английскую версию:

- [Repository layout](architecture/repository-layout.md) — границы ownership и структура репозитория.
- [ADR 0004 — Rust as a .NET template language](architecture/adr/0004-r10-rust-as-dotnet-template-language.md) — почему публичный контракт — `dotnet new <template> -lang Rust`.
- [Architecture Decision Records](architecture/adr/) — исторические архитектурные решения.

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

Локализованные документы должны возвращаться на эту главную страницу на том же языке. Следующий инкремент docgraph будет автоматически генерировать и проверять раздел «Что ссылается сюда?». 
