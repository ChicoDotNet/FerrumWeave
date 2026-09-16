<!--
translation-of: docs/roadmap/template-release-plan.md
locale: ru
source-revision: 4ed0f1c8df9606bdd7b1135f186ef4bdd710c77a
-->

# План релизов шаблонов FerrumWeave

[English](template-release-plan.md) · [Deutsch](template-release-plan.de.md) · [Español](template-release-plan.es.md) · [Français](template-release-plan.fr.md) · [Italiano](template-release-plan.it.md) · [Português (Brasil)](template-release-plan.pt-BR.md) · **Русский** · [简体中文](template-release-plan.zh-Hans.md) · [日本語](template-release-plan.ja.md)

Roadmap developer experience FerrumWeave рассматривает Rust как **язык проектов .NET**, а не как отдельный тип проекта.

Основной UX шаблонов:

```console
dotnet new <template> -lang Rust
```

Например:

```console
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

Прежняя форма `dotnet new rust` не является целевым продуктовым интерфейсом. Console-приложение остаётся проектом `console`; FerrumWeave добавляет Rust как ещё один выбор языка в template engine .NET.

## Что означает «поддерживается»

Шаблон не считается поддерживаемым только потому, что способен сгенерировать файлы.

Чтобы шаблон был заявлен как поддерживаемый в prerelease FerrumWeave:

1. `dotnet new <template> -lang Rust` должен создавать ожидаемые `.rsproj` и Rust-layout из установленного пакета FerrumWeave.
2. Созданный проект должен выполнять restore и build через обычный workflow .NET SDK без checkout репозитория FerrumWeave.
3. Значимый workflow по умолчанию должен успешно завершаться: `dotnet run` для запускаемых приложений и `dotnet test` для тестовых проектов.
4. Поведение должно причинно проходить через `rustc -> FerrumWeave CodegenBackend -> CIL/metadata -> CoreCLR`; legacy emitters по шаблонам исходника или сгенерированные заменители на C#/VB/F# контракт не выполняют.
5. Кроссплатформенные шаблоны должны быть сертифицированы на Windows и Linux. WinForms и WPF сертифицируются в поддерживаемых Windows-средах и не подразумевают выполнение на Linux.
6. Известные ограничения должны быть явно документированы.

Samples и conformance fixtures могут доказывать prerelease-контракт, но не заменяют gate реального проекта для 1.0.

## Последовательность версий

| Release | Этап | Шаблоны, впервые заявленные как поддерживаемые |
| --- | --- | --- |
| `0.1` | Alpha | `console`, `classlib`, `xunit`, `nunit`, `mstest`, `web`, `webapi` |
| `0.2` | Alpha | `mvc`, `winforms` |
| `0.3` | Beta | `worker` |
| `0.4` | Beta | `wpf` |
| `0.5` | Beta | `grpc` |
| `0.6` | Beta | `blazor` |
| `1.0` | Stable | Все заявленные семейства имеют сертифицированные контракты и хотя бы один успешный реальный проект |

Суффиксы вроде `0.1.0-alpha.1` и `0.5.0-beta.1` допустимы; таблица задаёт границы возможностей, а не точное число prerelease-итераций.

## 0.1 alpha — базовый опыт языка .NET

```console
dotnet new console  -lang Rust
dotnet new classlib -lang Rust
dotnet new xunit    -lang Rust
dotnet new nunit    -lang Rust
dotnet new mstest   -lang Rust
dotnet new web      -lang Rust
dotnet new webapi   -lang Rust
```

Release должен как минимум доказать: исполняемую точку входа и console output; managed class library, потребляемую другим .NET-проектом; discovery и выполнение через xUnit, NUnit и MSTest; ASP.NET Core host через `dotnet run`; минимальный HTTP request/response для `web` и `webapi`; нормальные restore/build/run/test, `ProjectReference` и NuGet; установку и использование из чистой внешней директории.

## 0.2 alpha — прикладные frameworks

```console
dotnet new mvc      -lang Rust
dotnet new winforms -lang Rust
```

`mvc` должен доказывать настоящий MVC request path. `winforms` — реальный lifecycle Windows Forms и поддерживаемую работу событий/delegates на Windows.

## 0.3 beta — фоновые сервисы

```console
dotnet new worker -lang Rust
```

Worker должен использовать .NET Generic Host и реальный lifecycle background service, а не быть переименованным console-приложением.

## 0.4 beta — WPF

```console
dotnet new wpf -lang Rust
```

WPF выделяется в отдельную границу возможностей из-за XAML, WindowsDesktop MSBuild targets, сгенерированных типов/code-behind, startup и event model.

## 0.5 beta — gRPC

```console
dotnet new grpc -lang Rust
```

Контракт должен покрывать сгенерированный protobuf/gRPC build path и хотя бы один настоящий request/response round trip.

## 0.6 beta — Blazor

Добавляется Blazor template surface, поддерживаемая целевой версией .NET SDK. Точный short name должен следовать актуальному SDK, а не закреплять устаревшую команду. Контракт должен доказать реальный rendered component path и необходимую Razor/Blazor build integration.

## Stable gate 1.0 — доказательства из реальных проектов

FerrumWeave не достигает 1.0 только потому, что все conformance suites зелёные.

До `1.0.0` **каждое заявленное семейство шаблонов должно иметь хотя бы один реальный проект, успешно использующий FerrumWeave**.

Реальный проект:

- не является compiler fixture, conformance test, template smoke test или чисто документационным sample;
- имеет настоящую прикладную или библиотечную цель;
- использует соответствующее Rust-template семейство в нормальной структуре проекта;
- воспроизводимо выполняет restore/build и, где применимо, run/test;
- упражняет шаблон репрезентативным способом;
- документирует специфические ограничения FerrumWeave.

Проект может быть open source или подтверждён приватно, но должен иметь достаточно воспроизводимых доказательств для заявления совместимости.

Вопрос stable release состоит из двух частей:

> Может ли FerrumWeave scaffold и сертифицировать каждое обещанное семейство?

и

> Прошло ли каждое семейство проверку хотя бы одним реальным приложением?

Только когда оба ответа положительны, FerrumWeave проходит gate `1.0.0`.

## Правило совместимости

```console
dotnet new console
dotnet new console -lang VB
dotnet new console -lang "F#"
dotnet new console -lang Rust
```

FerrumWeave расширяет языковую экосистему .NET и не должен перехватывать или ломать существующие варианты.
