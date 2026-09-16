<!--
doc-id: architecture.adr.0004-rust-as-dotnet-template-language
locale: ru
translation-of: docs/architecture/adr/0004-r10-rust-as-dotnet-template-language.md
source-revision: 0059e447c7b43940ac034f423b863d598f24b309
-->

<!-- ferrumweave-nav:start -->
[← Документация](../../README.ru.md) · [README проекта](../../../README.ru.md)
<!-- ferrumweave-nav:end -->

# ADR 0004 — Зарегистрировать Rust как язык шаблонов .NET

[English](0004-r10-rust-as-dotnet-template-language.md) · [Deutsch](0004-r10-rust-as-dotnet-template-language.de.md) · [Español](0004-r10-rust-as-dotnet-template-language.es.md) · [Français](0004-r10-rust-as-dotnet-template-language.fr.md) · [Italiano](0004-r10-rust-as-dotnet-template-language.it.md) · [Português (Brasil)](0004-r10-rust-as-dotnet-template-language.pt-BR.md) · **Русский** · [简体中文](0004-r10-rust-as-dotnet-template-language.zh-Hans.md) · [日本語](0004-r10-rust-as-dotnet-template-language.ja.md)

- Статус: Принято для R10 и prerelease roadmap
- Дата: 2026-09-15
- Milestone: R10 — Developer experience / 0.1 alpha

## Контекст

Цель FerrumWeave — сделать Rust языком первого класса в экосистеме .NET. Исходный прототип R10 предоставлял отдельную команду template:

```console
dotnet new rust
```

Технически это удобно, но такой подход моделирует Rust как тип проекта. В модели .NET templates `console`, `classlib`, `web`, тестовые проекты, desktop-проекты и другие workload families являются project templates; исходный язык — отдельное измерение выбора.

.NET-разработчик уже ожидает, что эквивалентные семейства проектов отличаются языком, а не несвязанными именами templates. FerrumWeave должен сохранить эту модель.

## Решение

Основной UX templates FerrumWeave:

```console
dotnet new <template> -lang Rust
```

Поэтому канонический контракт первого использования:

```console
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

`dotnet new rust` не является целевым публичным контрактом продукта.

Templates FerrumWeave должны объявлять Rust как язык и участвовать в обычных семействах .NET templates там, где template engine позволяет чистую интеграцию. Реализация должна подтверждаться исполняемыми контрактами разрешения templates, а не предположением, что достаточно совпадающих metadata встроенных templates.

Установка FerrumWeave не должна ломать или менять обычное поведение существующих вариантов C#, F# и Visual Basic.

## Последовательность releases

Поддержка templates вводится по следующим capability releases:

- `0.1-alpha`: `console`, `classlib`, `xunit`, `nunit`, `mstest`, `web`, `webapi`;
- `0.2-alpha`: `mvc`, `winforms`;
- `0.3-beta`: `worker`;
- `0.4-beta`: `wpf`;
- `0.5-beta`: `grpc`;
- `0.6-beta`: `blazor`.

Подробные контракты поддержки и gate стабильного release находятся в [`docs/roadmap/template-release-plan.ru.md`](../../roadmap/template-release-plan.ru.md).

## Поддерживается означает исполняется

Сам факт успешного создания каталога не доказывает, что template поддерживается.

Для prerelease-заявления о совместимости template как минимум должен:

- scaffold-иться через установленный дистрибутив FerrumWeave;
- выполнять restore и build без checkout исходников FerrumWeave;
- выполнять значимый workflow по умолчанию (`dotnet run` или `dotnet test`);
- проходить реальный product path FerrumWeave `rustc -> CodegenBackend -> CIL/metadata -> CoreCLR`;
- проходить соответствующие CI/platform contracts;
- честно документировать ограничения.

Framework-specific templates должны доказывать поведение framework, отличающее их от обычного console application.

## Gate 1.0

FerrumWeave не становится стабильным только потому, что проходят синтетические conformance tests.

До `1.0.0` у каждого семейства templates из зафиксированной prerelease-последовательности должен существовать хотя бы один реальный проект, успешно использующий этот FerrumWeave template.

Реальный проект — это не compiler fixture, не template smoke test и не sample документации. Он должен иметь настоящую цель помимо тестирования и воспроизводимые доказательства, соответствующие семейству проекта.

## Последствия

Положительные последствия:

- FerrumWeave ведёт себя как расширение .NET, а не как параллельная CLI-экосистема;
- команды templates остаются ожидаемыми для разработчиков C#, F# и Visual Basic;
- roadmap продукта становится измеримой через контракты семейств проектов;
- новые templates можно добавлять без изобретения FerrumWeave-специфичной таксономии команд;
- gate 1.0 связывает заявления о совместимости с реальными доказательствами.

Стоимость и ограничения:

- metadata и tests прототипа R10, предполагающие `dotnet new rust`, должны быть пересмотрены;
- разрешение templates должно проверяться против настоящего .NET template engine;
- framework templates могут потребовать новых возможностей compiler/runtime до того, как их можно будет честно считать поддерживаемыми;
- desktop и framework-specific templates сохраняют собственные platform/build ограничения.

## Валидация

R10 должен добавить исполняемые falsifiers, подтверждающие как минимум совместное существование:

```console
dotnet new console -lang Rust
dotnet new console -lang VB
dotnet new console -lang "F#"
```

после установки FerrumWeave, а также то, что сгенерированный Rust-проект каузально использует настоящий product backend FerrumWeave, а не legacy emitter или подмену исходного языка.

<!-- ferrumweave-backlinks:start -->
## Что ссылается сюда

- [Документация FerrumWeave](../../README.ru.md)
<!-- ferrumweave-backlinks:end -->
