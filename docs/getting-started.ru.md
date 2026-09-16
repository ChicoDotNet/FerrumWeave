<!--
translation-of: docs/getting-started.md
locale: ru
source-revision: e4bcd0d4b4c4b73232823896144adf098b30758b
-->

# FerrumWeave 0.1 alpha — начало работы

[English](getting-started.md) · [Deutsch](getting-started.de.md) · [Español](getting-started.es.md) · [Français](getting-started.fr.md) · [Italiano](getting-started.it.md) · [Português (Brasil)](getting-started.pt-BR.md) · **Русский** · [简体中文](getting-started.zh-Hans.md) · [日本語](getting-started.ja.md)

> **Рабочий контракт R10:** этот документ описывает внешний опыт, который FerrumWeave 0.1 alpha должен сертифицировать до релиза. Команды, которые ещё не GREEN в CI, являются целями продукта, а не заявлением о выпущенной функциональности.

FerrumWeave расширяет обычные шаблоны .NET, добавляя **Rust как выбор языка**. Основная UX — `dotnet new <template> -lang Rust`, а не отдельный тип проекта `dotnet new rust`.

План релизов и поддержка шаблонов описаны в [`template-release-plan.ru.md`](roadmap/template-release-plan.ru.md). Архитектурное решение — в [ADR 0004](architecture/adr/0004-r10-rust-as-dotnet-template-language.md).

## Целевой первый запуск 0.1-alpha

```console
dotnet new install FerrumWeave.Sdk::0.1.0-alpha.1
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

`FerrumWeave.Sdk` — текущий package для MSBuild SDK и шаблонов. Второй неопубликованный package не должен требоваться без отдельного ADR.

Rust-проект должен причинно пройти путь:

```text
Rust source
  -> rustc
  -> FerrumWeave CodegenBackend
  -> CIL / CLR metadata
  -> CoreCLR
```

Контракт должен работать из чистой внешней директории без checkout репозитория FerrumWeave.

## Шаблоны 0.1-alpha

```console
dotnet new console  -lang Rust
dotnet new classlib -lang Rust
dotnet new xunit    -lang Rust
dotnet new nunit    -lang Rust
dotnet new mstest   -lang Rust
dotnet new web      -lang Rust
dotnet new webapi   -lang Rust
```

Шаблон поддерживается только если restore/build и его значимый workflow (`dotnet run` или `dotnet test`) работают через настоящий FerrumWeave backend.

## FerrumWeave MSBuild SDK

```xml
<Project Sdk="FerrumWeave.Sdk/0.1.0-alpha.1"><PropertyGroup><TargetFramework>net10.0</TargetFramework></PropertyGroup></Project>
```

```console
dotnet restore
```

Package содержит `Sdk.props`, `Sdk.targets` и шаблоны; пользователю не нужно копировать их вручную.

## Локальный prerelease

```xml
<?xml version="1.0" encoding="utf-8"?>
<configuration><packageSources><clear /><add key="ferrumweave-local" value="../artifacts/packages" /></packageSources></configuration>
```

Должны работать те же restore/build/test/run команды, что и для опубликованного package, без скрытых знаний maintainer.

## Инвариант совместимости

```console
dotnet new console
dotnet new console -lang VB
dotnet new console -lang "F#"
dotnet new console -lang Rust
```

FerrumWeave добавляет Rust, не изменяя ожидаемое поведение .NET SDK.
