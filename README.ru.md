<!--
translation-of: README.md
locale: ru
source-revision: 6fb75d076d59b4e1932b685e441487f6caedb36d
-->

[English](README.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Português (Brasil)](README.pt-BR.md) · **Русский** · [简体中文](README.zh-Hans.md) · [日本語](README.ja.md)

> Документация FerrumWeave поддерживается на нескольких языках, потому что интероперабельность касается не только технологий, но и людей. Английский документ без языкового суффикса остаётся каноническим источником, если перевод временно отстаёт.

<img src="assets/brand/hero/ferrumweave-readme-cover.png" alt="FerrumWeave — Rust в экосистеме .NET" width="100%" />

# FerrumWeave

**Rust как часть языковой экосистемы .NET.**

**Ресурсы проекта:** [Структура репозитория](docs/architecture/repository-layout.md) · [План выпуска шаблонов](docs/roadmap/template-release-plan.md) · [Сайт проекта](https://chicodotnet.github.io/FerrumWeave/ru/)

FerrumWeave — экспериментальный open-source проект, цель которого сделать Rust полноценным языком платформы .NET: компилировать исходный код Rust в .NET assemblies, участвовать в Common Type System, использовать существующие .NET-библиотеки и естественно взаимодействовать с C#, F#, Visual Basic и другими языками вокруг CLR.

В долгосрочной перспективе опыт разработчика должен быть привычным:

```bash
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

С Rust-кодом, концептуально похожим на:

```rust
use dotnet::System::*;
use dotnet::Result;

fn main() -> Result<()> {
    Console::WriteLine("Hello from Rust on .NET")?;
    Ok(())
}
```

И с созданием настоящей .NET assembly:

```text
HelloFerrum.dll
```

которая выполняется .NET runtime.

> **FerrumWeave находится в самом начале этого пути.**
>
> Примеры в этом README описывают целевой developer experience и архитектурное направление. Они ещё не являются заявлениями о выпущенной функциональности.

---

## Зачем FerrumWeave?

Одна из самых долговечных идей .NET заключалась не в самом C#.

Она заключалась в том, что **разные языки могут встречаться на общей runtime-платформе**.

На протяжении десятилетий разработчики могли использовать языки с совершенно разными синтаксисом, философией и историей, оставаясь на одной платформе:

- C#
- Visual Basic .NET
- F#
- C++/CLI
- JScript .NET
- J#
- IronPython
- IronRuby
- Nemerle
- Boo
- Oxygene
- и многие другие

Библиотеку, написанную на одном языке, часто можно было использовать из другого, потому что главной границей был не исходный язык.

Этой границей были **Common Language Infrastructure**, **Common Type System**, metadata assemblies и CLR.

FerrumWeave задаёт простой вопрос:

> **Что, если Rust сможет войти в эту семью языков?**

Не только вызывая нативную Rust-библиотеку через FFI.

Не только размещая CLR внутри Rust-приложения.

Не переводя Rust в C#.

А компилируя Rust в тот же мир CIL, assemblies, metadata, типов, ссылок, пакетов, tooling и runtime-интероперабельности, который изначально сделал .NET многоязычной платформой.

---

## Почему Rust?

Rust приносит в разработку другой набор гарантий.

Ownership, borrow checker, строгая система типов, явная обработка ошибок и акцент на безопасности памяти и конкурентности позволяют выявлять целые классы дефектов до production.

Rust **не** делает программы неспособными ошибаться.

Логика может быть неверной. Файлы могут исчезать. Сеть может падать. В базах могут быть плохие данные. Программа может паниковать. Существует `unsafe`.

Но Rust способен перенести важные классы ошибок из:

```text
production
```

в:

```text
compile time
```

Для долгоживущих бизнес-систем это принципиально важно.

---

## Возможность для корпоративного ПО

Существуют огромные .NET-системы, которые приносят пользу десять, пятнадцать или двадцать пять лет.

В них могут быть:

```text
ERP.vbproj
Accounting.csproj
Reporting.fsproj
LegacyIntegration.vbproj
```

Разговор о модернизации слишком часто начинается словами:

> «Надо всё переписать».

FerrumWeave исходит из другой идеи:

> **Сохранять то, что работает. Усиливать то, что создаётся дальше.**

Представьте, что в ту же систему добавляется:

```text
RiskEngine.rsproj
```

Существующее приложение Visual Basic не обязано исчезать. Доменную модель на C# не нужно переписывать. F#-движку отчётности не нужен новый протокол интеграции.

В перспективе:

```text
ERP.vbproj
     │
     ▼
RiskEngine.rsproj
     │
     ▼
Accounting.csproj
```

могут взаимодействовать через одну систему типов .NET.

Старое приложение Visual Basic сможет вызывать новый Rust-код. Rust сможет использовать доменную assembly на C#. F# сможет использовать тип, реализованный в Rust.

Единицей миграции становится **компонент**, а не всё приложение.

Это и есть замысел.

---

# Цель

FerrumWeave стремится сделать возможным следующее:

```text
                    .NET
                     │
             Common Type System
                     │
       ┌─────────────┼─────────────┐
       │             │             │
      C#            F#           Rust
       │             │             │
    Roslyn          fsc          rustc
       │             │             │
       └─────────────┼─────────────┘
                     │
                     ▼
                CIL + Metadata
                     │
                     ▼
                    CLR
```

Предполагаемый путь компиляции Rust:

```text
Rust source
    │
    ▼
rustc frontend
    │
    ▼
HIR / MIR
    │
    ▼
CLR code generation
    │
    ▼
CIL + .NET metadata
    │
    ▼
.NET assembly
    │
    ▼
CLR
```

Rust остаётся Rust.

CLR остаётся CLR.

FerrumWeave должен соединять их, а не без необходимости изобретать заново.

---

# Чем FerrumWeave не является

FerrumWeave **не** задуман как:

### Новый Rust-подобный язык

Цель — сохранить Rust и использовать существующую экосистему компилятора.

```rust
match
traits
lifetimes
ownership
borrowing
async
Result<T, E>
```

должны оставаться понятиями Rust, а не приближениями, заново реализованными в другом компиляторе.

### Rust-обёртка вокруг `dotnet`

Запуск Cargo из MSBuild target может быть полезен, но сам по себе не делает Rust языком .NET.

FerrumWeave идёт глубже.

### Генератор нативного FFI

Нативная интероперабельность полезна, но цель не такая:

```text
C#
 ↓
P/Invoke
 ↓
Rust native DLL
```

Цель такая:

```text
C#
  ╲
   CLR
  ╱
Rust
```

### Замена .NET

FerrumWeave существует именно потому, что экосистема .NET ценна. Проект расширяет выбор языков, а не заменяет платформу.

### Замена нативного Rust

Всегда будут отличные причины компилировать Rust напрямую в нативный код. CLR target — дополнительный вариант deployment и интероперабельности.

---

# Желаемый developer experience

В будущем Rust-проект должен естественно выглядеть внутри .NET solution:

```text
EnterpriseSystem.slnx
│
├── Domain/
│   └── Domain.csproj
├── Reporting/
│   └── Reporting.fsproj
├── Legacy/
│   └── Legacy.vbproj
└── RiskEngine/
    └── RiskEngine.rsproj
```

Привычные команды должны оставаться привычными:

```bash
dotnet restore
dotnet build
dotnet run
dotnet test
dotnet publish
dotnet pack
```

Проект может выглядеть так:

```xml
<Project Sdk="FerrumWeave.Sdk">
  <PropertyGroup>
    <TargetFramework>net10.0</TargetFramework>
    <RustEdition>2024</RustEdition>
  </PropertyGroup>
  <ItemGroup>
    <ProjectReference Include="../Domain/Domain.csproj" />
    <PackageReference Include="Some.DotNet.Package" Version="..." />
  </ItemGroup>
</Project>
```

Cargo и crates.io должны сохранять своё место там, где они нужны Rust-зависимостям. NuGet и MSBuild должны продолжать выполнять свою роль для .NET-зависимостей.

FerrumWeave должен соединять эти экосистемы, а не делать вид, что одной из них не существует.

---

# .NET-библиотеки из Rust

Одна из центральных целей — сделать .NET APIs естественной частью Rust-кода.

```rust
use dotnet::System::*;
use dotnet::System::IO::*;

fn main() -> Result<()> {
    Console::Write("Name: ")?;
    let name = Console::ReadLine()?;
    File::WriteAllText("name.txt", &name)?;
    Ok(())
}
```

Важна не точная форма синтаксиса, а принцип:

> `System.Console`, `System.String`, `System.IO.File` и пользовательские .NET-типы должны восприниматься как CLR-типы и члены, а не как непрозрачные нативные библиотеки за вручную поддерживаемым FFI-слоем.

То же должно относиться к NuGet packages и ProjectReference.

---

# Rust-библиотеки из других .NET-языков

Интероперабельность должна работать в обоих направлениях.

Rust должен уметь определять публичные CLR-facing типы, которые смогут использовать другие .NET-языки.

Rust:

```rust
pub struct RiskEngine {
    // ...
}

impl RiskEngine {
    pub fn calculate(&self, customer: Customer) -> RiskScore {
        // ...
    }
}
```

C#:

```csharp
var engine = new RiskEngine();
var score = engine.Calculate(customer);
```

Visual Basic:

```vb
Dim engine = New RiskEngine()
Dim score = engine.Calculate(customer)
```

F#:

```fsharp
let engine = RiskEngine()
let score = engine.Calculate(customer)
```

Разные исходные языки. Один runtime-контракт.

К такому стандарту интероперабельности стремится FerrumWeave.

---

# Типы Rust и CLR

Rust и CLR имеют принципиально разные модели объектов и памяти. Эту разницу нельзя скрывать.

Rust использует понятия:

```text
ownership
borrowing
lifetimes
RAII
Box<T>
Vec<T>
String
Option<T>
Result<T, E>
```

CLR использует:

```text
managed references
garbage collection
System.Object
System.String
arrays
interfaces
delegates
exceptions
Task<T>
```

FerrumWeave не должен ослаблять ни одну модель только ради внешнего сходства. Вместо этого нужны принципиально определённые отображения.

Некоторые соответствия естественны:

```text
System.Int32   ↔ i32
System.Int64   ↔ i64
System.Boolean ↔ bool
System.Double  ↔ f64
```

Другие требуют явной семантики:

```text
System.String
managed classes
interfaces
delegates
exceptions
Task<T>
Span<T>
Nullable<T>
```

Корректно определить эти правила — одна из главных инженерных задач проекта.

---

# Безопасность без отказа от интероперабельности

Корпоративное ПО редко может позволить себе начать заново.

У организаций уже есть работающие приложения, базы данных, бизнес-правила, APIs, пакеты, frameworks, разработчики и операционные знания.

Более безопасный язык гораздо легче внедрять, если для этого не требуется отказаться от всего существующего.

Поэтому FerrumWeave исследует идею:

> **Привнести модель безопасности Rust в новые и критически важные .NET-компоненты, сохраняя интероперабельность с существующими инвестициями в .NET.**

---

# Почему название FerrumWeave?

**Ferrum** по-латыни означает железо. Rust — окисление железа.

**Weave** означает переплетать отдельные нити в связанную структуру.

```text
Ferrum
   │
   └── iron → rust

Weave
   │
   └── interconnection → network → ecosystem
```

Вместе:

> **FerrumWeave — это Rust, вплетённый в экосистему .NET.**

Название намеренно независимо от торговых марок Rust и .NET.

---

# Использовать существующую работу

FerrumWeave не собирается начинать с написания ещё одного Rust-компилятора.

Экосистема Rust уже предоставляет огромную ценность:

- `rustc` для parsing, type checking, borrow checking, MIR и семантики языка;
- `rust-analyzer` для современного анализа Rust-кода и tooling.

Существует и важный prior art в направлении Rust→CLR, особенно экспериментальный `rustc_codegen_clr`.

.NET уже предоставляет зрелые CLR, CTS, assembly metadata, MSBuild, NuGet, `dotnet` CLI, SDK-style проекты, debugging и tooling.

Стратегия FerrumWeave:

> **Сначала интегрировать, потом изобретать заново.**

По возможности улучшения следует вносить upstream вместо вечного содержания приватных forks.

---

# Начальная архитектура

```text
FerrumWeave
│
├── CLR code generation
│   └── Rust MIR → CIL / metadata
├── CLR projection
│   └── .NET metadata → Rust-visible types and members
├── SDK
│   └── .rsproj / MSBuild / dotnet CLI integration
├── interoperability
│   └── Rust ↔ CTS semantics
├── code analysis
│   └── rust-analyzer awareness of CLR symbols
├── debugging
│   └── source mapping / PDB / stepping / locals
└── tooling
    └── templates, testing, publishing and packaging
```

Целевая структура репозитория описана в [Repository layout](docs/architecture/repository-layout.md).

Архитектура намеренно предварительная. Исполняемая доказательная база важнее диаграмм.

---

# Первый доказательный шаг

Первый значимый milestone намеренно мал:

```bash
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

```rust
use dotnet::System::*;

fn main() -> Result<()> {
    Console::WriteLine("Hello from FerrumWeave")?;
    Ok(())
}
```

FerrumWeave должен создать корректную .NET assembly и выполнить `System.Console.WriteLine` через CLR.

Один такой результат проверяет `.rsproj`, MSBuild, `dotnet` CLI, `rustc`, Rust→CIL, CLR metadata, CTS interop и .NET BCL.

Проект будет расти вертикально от работающих контрактов, а не пытаться заранее смоделировать всю .NET-экосистему.

---

# Долгосрочный успех

FerrumWeave нельзя считать успешным только потому, что Rust способен выдавать «какой-то CIL».

Более важный вопрос:

> **Сможет ли .NET-разработчик воспринимать Rust как ещё один серьёзный язык внутри существующей .NET-системы?**

Зрелый FerrumWeave должен сделать обычными сценарии:

```text
C# → Rust
Rust → C#
VB → Rust
Rust → F#
Rust → NuGet package
.NET project → .rsproj
.rsproj → .NET project
```

с привычными ожиданиями от build, references, packages, types, exceptions, debugging, testing, tooling и publishing.

---

# Принципы проекта

### Сохранять Rust

Не создавать ненужный диалект Rust.

### Сохранять .NET

Использовать CLR, CTS, metadata, MSBuild, NuGet и существующие контракты платформы.

### Внедрять постепенно

Двадцатилетнее приложение не должно требовать полного rewrite ради одного нового Rust-компонента.

### Предпочитать Safe Rust

`unsafe` — законная часть Rust, но проект должен максимально сохранять область действия обычных гарантий безопасности.

### Делать границы явными

Rust ownership и CLR garbage collection — разные системы. Сложные семантические границы следует моделировать явно.

### Работать upstream, где это разумно

Устойчивая экосистема лучше постоянных forks.

### Доказательства раньше заявлений

Корректность компилятора должна подтверждаться исполняемыми тестами, differential validation, conformance evidence и реальными приложениями, а не оптимистичными диаграммами.

### Совместимость — контракт

Если FerrumWeave заявляет поддержку .NET-поведения, оно должно быть защищено повторяемыми тестами.

---

# Open source с самого начала

FerrumWeave разрабатывается открыто и доступен по выбору под:

- [MIT License](LICENSE-MIT); или
- [Apache License, Version 2.0](LICENSE-APACHE).

Использование компилятора не должно навязывать лицензию FerrumWeave скомпилированной программе.

---

# Видение governance

FerrumWeave начинается как независимый проект.

Если он станет достаточно полезен для реальной много-командной и много-корпоративной экосистемы, governance должен иметь возможность стать независимым от первоначального автора.

Нейтральный долгосрочный дом в подходящей open-source foundation будет успехом, а не потерей собственности.

Проект должен поддерживать прозрачные технические решения, чистую IP-provenance, отслеживаемость вкладов, переносимые активы и открытую governance.

Сейчас проект ни с какой foundation не связан.

---

# Отношение к Rust и .NET

FerrumWeave — независимый экспериментальный проект.

Он не аффилирован, не спонсируется и не одобрен Microsoft, .NET Foundation, Rust Foundation или Rust Project.

Названия «Rust» и «.NET» используются для точного описания технологий, с которыми FerrumWeave собирается взаимодействовать.

---

# Статус

**Pre-alpha / архитектурное исследование.**

Репозиторий намеренно начинает с проблемы, принципов, целевых контрактов и инженерных границ, прежде чем заявлять полную реализацию языка.

Первый фокус — надёжный вертикальный slice:

```text
Rust source
    ↓
.rsproj
    ↓
dotnet build / dotnet run
    ↓
CIL
    ↓
CLR
    ↓
System.Console.WriteLine
```

Дальше — один контракт за другим.

---

# Идея одним предложением

> **FerrumWeave стремится сделать Rust полноценным языком .NET, чтобы организации могли внедрять модель безопасности Rust в новые критически важные компоненты, не отказываясь от существующего ПО, библиотек, языков и операционного знания.**

---

## Очень старый Hello World. Очень современный компилятор.

Приложение Visual Basic, написанное десятилетия назад, в будущем должно уметь вызывать современный Safe Rust-код.

Не через service boundary.

Не через rewrite.

Не потому, что один язык притворяется другим.

А потому, что оба способны говорить на языке, который CLR изначально создавался предоставлять между языками.

В этом и есть переплетение.