# FerrumWeave

**Bringing Rust into the .NET language ecosystem.**

FerrumWeave is an experimental open-source effort to make Rust a first-class language for the .NET platform: compiling Rust source code into .NET assemblies, participating in the Common Type System, consuming existing .NET libraries, and interoperating naturally with languages such as C#, F#, Visual Basic, and others built around the CLR.

The long-term developer experience should feel unsurprising:

```bash
dotnet new rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

With Rust code conceptually resembling:

```rust
use dotnet::System::*;
use dotnet::Result;

fn main() -> Result<()> {
    Console::WriteLine("Hello from Rust on .NET")?;
    Ok(())
}
```

And producing a real .NET assembly:

```text
HelloFerrum.dll
```

executed by the .NET runtime.

> **FerrumWeave is at the beginning of that journey.**
>
> The examples in this README describe the intended developer experience and architectural direction. They are not yet claims of implemented functionality.

---

## Why FerrumWeave?

One of .NET's most enduring ideas was never C# itself.

It was the idea that **languages could meet at a common runtime**.

For decades, developers have been able to write software using languages with very different syntax, philosophies, and histories while sharing the same underlying platform:

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
- and many others

A library written in one language could often be consumed from another because the important boundary was not the source language.

It was the **Common Language Infrastructure**, the **Common Type System**, assembly metadata, and the CLR.

FerrumWeave asks a simple question:

> **What if Rust could join that language family?**

Not merely by calling a native Rust library through FFI.

Not merely by hosting the CLR from a Rust executable.

Not by translating Rust into C#.

But by compiling Rust into the same world of CIL, assemblies, metadata, types, references, packages, tooling, and runtime interoperability that made multi-language .NET possible in the first place.

---

## Why Rust?

Rust brings a different set of guarantees to software development.

Its ownership model, borrow checker, strong type system, explicit error handling, and focus on memory and concurrency safety allow entire categories of defects to be caught before software reaches production.

Rust does **not** make software incapable of failing.

Logic can still be wrong. Files can disappear. Networks can fail. Databases can contain bad data. Programs can panic. Unsafe code exists.

But Rust can move important classes of failures from:

```text
production
```

toward:

```text
compile time
```

That distinction matters.

Especially in long-lived business systems.

---

## The business software opportunity

There are enormous .NET systems that have been delivering value for ten, fifteen, or twenty-five years.

They may contain:

```text
ERP.vbproj
Accounting.csproj
Reporting.fsproj
LegacyIntegration.vbproj
```

The usual modernization conversation too often begins with:

> "We should rewrite it."

FerrumWeave is built around a different idea:

> **Preserve what works. Strengthen what comes next.**

Imagine adding:

```text
RiskEngine.rsproj
```

to that same system.

The existing Visual Basic application does not need to disappear.

The C# domain model does not need to be rewritten.

The F# reporting engine does not need a new integration protocol.

Instead:

```text
ERP.vbproj
     │
     ▼
RiskEngine.rsproj
     │
     ▼
Accounting.csproj
```

could eventually communicate through the same .NET type system.

A decades-old Visual Basic application could call newly written Rust code.

Rust could consume a domain assembly written in C#.

F# could consume a type implemented in Rust.

The migration unit becomes **the component**, not the application.

That is the vision.

---

# The goal

FerrumWeave aims to make this possible:

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

For Rust, the intended compilation path is approximately:

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

Rust remains Rust.

The CLR remains the CLR.

FerrumWeave should connect them rather than unnecessarily reinvent either.

---

# What FerrumWeave is not

FerrumWeave is **not** intended to be:

### A new Rust-like language

The objective is to preserve the Rust language and benefit from the existing Rust compiler ecosystem.

```rust
match
traits
lifetimes
ownership
borrowing
async
Result<T, E>
```

should remain Rust concepts rather than approximations recreated in another compiler.

### A Rust wrapper around `dotnet`

Running Cargo from an MSBuild target could be useful, but that alone would not make Rust a .NET language.

FerrumWeave ultimately aims deeper.

### A native FFI generator

Native interoperability remains valuable, but the goal is not:

```text
C#
 ↓
P/Invoke
 ↓
Rust native DLL
```

The target is:

```text
C#
  ╲
   CLR
  ╱
Rust
```

### A replacement for .NET

FerrumWeave exists because the .NET ecosystem is valuable.

The objective is to extend the language choices available to it.

### A replacement for native Rust

There will always be excellent reasons to compile Rust directly to native code.

A CLR target would be another deployment and interoperability option, not a declaration that every Rust workload belongs on .NET.

---

# The desired developer experience

Eventually, a Rust project should look at home inside a .NET solution.

```text
EnterpriseSystem.slnx
│
├── Domain/
│   └── Domain.csproj
│
├── Reporting/
│   └── Reporting.fsproj
│
├── Legacy/
│   └── Legacy.vbproj
│
└── RiskEngine/
    └── RiskEngine.rsproj
```

And familiar commands should remain familiar:

```bash
dotnet restore
dotnet build
dotnet run
dotnet test
dotnet publish
dotnet pack
```

A project could eventually resemble:

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

Cargo and crates.io should continue to have a place where Rust dependencies require them.

NuGet and MSBuild should continue to do what they already do well for .NET dependencies.

FerrumWeave should connect these ecosystems without pretending that either one does not exist.

---

# .NET libraries from Rust

A central objective is for .NET APIs to become natural participants in Rust code.

Conceptually:

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

The important property is not the exact syntax shown above.

The important property is this:

> `System.Console`, `System.String`, `System.IO.File`, and user-defined .NET types should be understood as CLR types and members — not opaque native libraries hidden behind a manually maintained FFI layer.

The same principle should eventually apply to NuGet packages and project references.

For example:

```xml
<ProjectReference Include="../Domain/Domain.csproj" />
```

should make the public CLR surface of `Domain` available to Rust.

Conceptually:

```rust
use Domain::*;

let customer = Customer::new("Ada");
let risk = customer.CalculateRisk()?;
```

---

# Rust libraries from other .NET languages

Interoperability must work in both directions.

Rust should eventually be capable of defining public CLR-facing types that other .NET languages can consume.

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

Different source languages.

One runtime contract.

That is the interoperability standard FerrumWeave ultimately wants to reach.

---

# Rust types and CLR types

Rust and the CLR have fundamentally different object and memory models.

That difference should not be hidden.

Rust has concepts such as:

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

The CLR has:

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

FerrumWeave should not weaken either model merely to make them look identical.

Instead, it should define principled mappings between them.

Some mappings may be natural:

```text
System.Int32   ↔ i32
System.Int64   ↔ i64
System.Boolean ↔ bool
System.Double  ↔ f64
```

Others require explicit semantics:

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

Determining those semantics correctly is one of the core engineering challenges of this project.

---

# Safety without abandoning interoperability

FerrumWeave begins from a practical observation:

Enterprise software rarely gets the luxury of starting over.

Organizations have working applications, existing databases, business rules, APIs, packages, frameworks, developers, and operational knowledge.

A safer language is much easier to adopt if adopting it does not require abandoning everything around it.

FerrumWeave therefore explores this proposition:

> **Bring Rust's safety model to new and critical .NET components while preserving interoperability with existing .NET investments.**

That makes Rust potentially useful not only for systems programming, but also for incremental modernization of long-lived line-of-business software.

---

# Why the name FerrumWeave?

**Ferrum** is Latin for iron.

Rust is the oxidation of iron.

**Weave** means to interlace separate threads into a connected structure.

The name therefore carries two ideas:

```text
Ferrum
   │
   └── iron → rust

Weave
   │
   └── interconnection → network → ecosystem
```

Together:

> **FerrumWeave represents Rust woven into the .NET ecosystem.**

The name is intentionally independent from both the Rust and .NET trademarks.

The technologies it interoperates with can be described accurately without requiring the project itself to impersonate either ecosystem.

---

# Building on existing work

FerrumWeave does not intend to begin by writing another Rust compiler.

The Rust ecosystem already provides enormously valuable infrastructure:

- `rustc` for parsing, type checking, borrow checking, MIR, and language semantics;
- `rust-analyzer` for modern Rust code analysis and development tooling.

There is also important prior art in compiling Rust toward the CLR, particularly the experimental `rustc_codegen_clr` project.

Likewise, .NET already provides mature infrastructure for:

- the CLR;
- the Common Type System;
- assembly metadata;
- MSBuild;
- NuGet;
- the `dotnet` CLI;
- SDK-style projects;
- debugging and tooling.

FerrumWeave's strategy is therefore:

> **Integrate before reinventing.**

Where possible, improvements should be contributed upstream rather than maintained forever as private forks.

---

# Early architecture

The project currently expects several major areas of work:

```text
FerrumWeave
│
├── CLR code generation
│   └── Rust MIR → CIL / metadata
│
├── CLR projection
│   └── .NET metadata → Rust-visible types and members
│
├── SDK
│   └── .rsproj / MSBuild / dotnet CLI integration
│
├── interoperability
│   └── Rust ↔ CTS semantics
│
├── code analysis
│   └── rust-analyzer awareness of CLR symbols
│
├── debugging
│   └── source mapping / PDB / stepping / locals
│
└── tooling
    └── templates, testing, publishing and packaging
```

This architecture is intentionally provisional.

Executable evidence will outrank diagrams.

---

# First proof

The first meaningful milestone is deliberately small.

Given:

```bash
dotnet new rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

and:

```rust
use dotnet::System::*;

fn main() -> Result<()> {
    Console::WriteLine("Hello from FerrumWeave")?;
    Ok(())
}
```

FerrumWeave should produce a valid .NET assembly and execute `System.Console.WriteLine` through the CLR.

That one result would validate several assumptions at once:

```text
.rsproj
   +
MSBuild
   +
dotnet CLI
   +
rustc
   +
Rust → CIL
   +
CLR metadata
   +
CTS interop
   +
.NET BCL
```

The project will grow vertically from working contracts like this rather than attempting to model the entire .NET ecosystem before anything runs.

---

# Long-term success

FerrumWeave will not be considered successful merely because:

```text
Rust can emit some CIL.
```

The deeper standard is:

> **Can a .NET developer treat Rust as another serious language choice inside an existing .NET system?**

A mature FerrumWeave should make scenarios like these unsurprising:

```text
C# → Rust
Rust → C#
VB → Rust
Rust → F#
Rust → NuGet package
.NET project → .rsproj
.rsproj → .NET project
```

with the same expectations developers already have around:

```text
build
references
packages
types
exceptions
debugging
testing
tooling
publishing
```

That is a much larger objective than compiling "Hello World".

It is also the reason this project exists.

---

# Project principles

FerrumWeave intends to follow several engineering principles from its beginning.

### Preserve Rust

Avoid creating an unnecessary Rust dialect.

### Preserve .NET

Use CLR, CTS, metadata, MSBuild, NuGet, and other existing platform contracts rather than replacing them without reason.

### Interoperate incrementally

A twenty-year-old application should not require a rewrite to benefit from one new Rust component.

### Prefer safe Rust

`unsafe` remains a legitimate part of Rust, but the project should maximize the area where Rust's normal safety guarantees remain meaningful.

### Make boundaries explicit

Rust ownership and CLR garbage collection are different systems. Difficult semantic boundaries should be modeled deliberately rather than hidden behind convenient syntax.

### Upstream where practical

A sustainable ecosystem is preferable to permanent forks.

### Evidence before claims

Compiler correctness must come from executable tests, differential validation, conformance evidence, and real applications — not from architectural diagrams or optimistic compatibility percentages.

### Compatibility is a contract

If FerrumWeave claims that a .NET behavior works, that behavior should be protected by repeatable tests.

---

# Open source from the beginning

FerrumWeave is intended to be developed openly.

The current licensing direction is a permissive dual-license model:

```text
MIT OR Apache-2.0
```

allowing broad use in open-source and commercial environments while aligning naturally with much of the Rust ecosystem.

Software produced using FerrumWeave should remain under the license chosen by its own authors.

Using the compiler should not impose FerrumWeave's license on the program being compiled.

---

# Governance vision

FerrumWeave begins as an independent project.

If it becomes useful enough to develop a genuine multi-company and multi-community ecosystem, its governance should be capable of becoming independent of its original creator.

A long-term neutral home — potentially under an organization such as the Linux Foundation or another appropriate open-source foundation — would be considered a success rather than a loss of ownership.

The project should therefore be built from the beginning with:

- transparent technical decisions;
- clean intellectual-property provenance;
- contributor traceability;
- transferable project assets;
- open governance as the community grows;
- no unnecessary dependency on a single commercial organization.

No foundation affiliation currently exists.

---

# Relationship to Rust and .NET

FerrumWeave is an independent experimental project.

It is not currently affiliated with, sponsored by, or endorsed by Microsoft, the .NET Foundation, the Rust Foundation, or the Rust Project.

"Rust" and ".NET" are used in this project to accurately describe the technologies with which FerrumWeave intends to interoperate.

---

# Status

**Pre-alpha / architectural discovery.**

The repository is intentionally starting with the problem, principles, target contracts, and engineering boundaries before claiming a working language implementation.

The first objective is not feature breadth.

It is one trustworthy vertical slice:

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

From there, one contract at a time.

---

# The idea in one sentence

> **FerrumWeave aims to make Rust a first-class .NET language so organizations can introduce Rust's safety model into new and critical components without abandoning the software, libraries, languages, and operational knowledge they already have.**

---

## A very old Hello World. A very modern compiler.

A Visual Basic application written decades ago should eventually be able to call code written today in safe Rust.

Not through a service boundary.

Not through a rewrite.

Not because either language pretends to be the other.

Because both can speak the language the CLR was designed to provide between languages.

That is the weave.
