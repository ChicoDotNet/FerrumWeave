<!--
translation-of: README.md
locale: de
source-revision: 6fb75d076d59b4e1932b685e441487f6caedb36d
-->

[English](README.md) · **Deutsch** · [Español](README.es.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [简体中文](README.zh-Hans.md) · [日本語](README.ja.md)

> Die FerrumWeave-Dokumentation wird in mehreren Sprachen gepflegt, denn Interoperabilität betrifft auch Menschen. Die englische Datei ohne Sprachsuffix ist die kanonische Quelle, falls eine Übersetzung vorübergehend hinterherhinkt.

<img src="assets/brand/hero/ferrumweave-readme-cover.png" alt="FerrumWeave — Rust in das .NET-Ökosystem bringen" width="100%" />

# FerrumWeave

**Rust in das .NET-Sprachökosystem bringen.**

**Projektressourcen:** [Repository-Struktur](docs/architecture/repository-layout.md) · [Template-Releaseplan](docs/roadmap/template-release-plan.md) · [Projektwebsite](https://chicodotnet.github.io/FerrumWeave/de/)

FerrumWeave ist ein experimentelles Open-Source-Projekt mit dem Ziel, Rust zu einer First-Class-Sprache für die .NET-Plattform zu machen: Rust-Quellcode in .NET-Assemblies zu kompilieren, am Common Type System teilzunehmen, bestehende .NET-Bibliotheken zu konsumieren und natürlich mit C#, F#, Visual Basic und anderen CLR-Sprachen zu interoperieren.

Langfristig soll sich die Developer Experience selbstverständlich anfühlen:

```bash
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

Mit Rust-Code, der konzeptionell etwa so aussieht:

```rust
use dotnet::System::*;
use dotnet::Result;

fn main() -> Result<()> {
    Console::WriteLine("Hello from Rust on .NET")?;
    Ok(())
}
```

und eine echte .NET-Assembly erzeugt:

```text
HelloFerrum.dll
```

welche von der .NET-Runtime ausgeführt wird.

> **FerrumWeave steht am Anfang dieses Weges.**
>
> Die Beispiele in diesem README beschreiben die angestrebte Developer Experience und Architektur. Sie sind noch keine Behauptung über bereits veröffentlichte Funktionalität.

---

## Warum FerrumWeave?

Eine der langlebigsten Ideen von .NET war nie C# allein.

Es war die Idee, dass **Sprachen sich auf einer gemeinsamen Runtime treffen können**.

Seit Jahrzehnten können Entwickler Software in Sprachen mit sehr unterschiedlicher Syntax, Philosophie und Geschichte schreiben und dennoch dieselbe Plattform teilen:

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
- und viele weitere

Eine Bibliothek in einer Sprache konnte oft aus einer anderen genutzt werden, weil die entscheidende Grenze nicht die Quellsprache war.

Entscheidend waren die **Common Language Infrastructure**, das **Common Type System**, Assembly-Metadaten und die CLR.

FerrumWeave stellt daher eine einfache Frage:

> **Was wäre, wenn Rust zu dieser Sprachfamilie gehören könnte?**

Nicht nur über FFI-Aufrufe an eine native Rust-Bibliothek.

Nicht nur, indem eine Rust-Anwendung die CLR hostet.

Nicht durch Übersetzung von Rust nach C#.

Sondern indem Rust in dieselbe Welt aus CIL, Assemblies, Metadaten, Typen, Referenzen, Paketen, Tooling und Runtime-Interoperabilität kompiliert wird, die .NET von Anfang an mehrsprachig gemacht hat.

---

## Warum Rust?

Rust bringt andere Garantien in die Softwareentwicklung ein.

Ownership, Borrow Checker, ein starkes Typsystem, explizite Fehlerbehandlung sowie der Fokus auf Speicher- und Nebenläufigkeitssicherheit können ganze Fehlerklassen erkennen, bevor Software die Produktion erreicht.

Rust macht Software **nicht** unfehlbar.

Logik kann falsch sein. Dateien können verschwinden. Netzwerke können ausfallen. Datenbanken können schlechte Daten enthalten. Programme können paniken. `unsafe` existiert.

Rust kann jedoch wichtige Fehlerklassen von:

```text
Produktion
```

in Richtung:

```text
Compile-Zeit
```

verschieben.

Dieser Unterschied ist besonders bei langlebigen Geschäftssystemen wichtig.

---

## Die Chance in Unternehmenssoftware

Es gibt riesige .NET-Systeme, die seit zehn, fünfzehn oder fünfundzwanzig Jahren Wert liefern.

Sie können etwa Folgendes enthalten:

```text
ERP.vbproj
Accounting.csproj
Reporting.fsproj
LegacyIntegration.vbproj
```

Modernisierung beginnt zu oft mit:

> „Wir sollten alles neu schreiben.“

FerrumWeave folgt einer anderen Idee:

> **Bewahre, was funktioniert. Stärke, was als Nächstes entsteht.**

Stellen wir uns vor, wir ergänzen:

```text
RiskEngine.rsproj
```

im selben System.

Die bestehende Visual-Basic-Anwendung muss nicht verschwinden.

Das C#-Domänenmodell muss nicht neu geschrieben werden.

Die F#-Reporting-Engine braucht kein neues Integrationsprotokoll.

Stattdessen könnten:

```text
ERP.vbproj
     │
     ▼
RiskEngine.rsproj
     │
     ▼
Accounting.csproj
```

irgendwann über dasselbe .NET-Typsystem kommunizieren.

Eine jahrzehntealte Visual-Basic-Anwendung könnte neuen Rust-Code aufrufen. Rust könnte eine C#-Domain-Assembly konsumieren. F# könnte einen in Rust implementierten Typ verwenden.

Die Migrationseinheit wird **die Komponente**, nicht die gesamte Anwendung.

Das ist die Vision.

---

# Das Ziel

FerrumWeave möchte Folgendes ermöglichen:

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

Der geplante Rust-Kompilierungspfad ist ungefähr:

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

Rust bleibt Rust.

Die CLR bleibt die CLR.

FerrumWeave soll beide verbinden, statt eines von beiden unnötig neu zu erfinden.

---

# Was FerrumWeave nicht ist

FerrumWeave soll **nicht** sein:

### Eine neue Rust-ähnliche Sprache

Rust selbst und sein bestehendes Compiler-Ökosystem sollen erhalten bleiben.

```rust
match
traits
lifetimes
ownership
borrowing
async
Result<T, E>
```

sollen Rust-Konzepte bleiben und nicht als Näherungen in einem anderen Compiler nachgebaut werden.

### Ein Rust-Wrapper um `dotnet`

Cargo aus einem MSBuild-Target zu starten kann nützlich sein, macht Rust allein aber noch nicht zu einer .NET-Sprache.

FerrumWeave zielt tiefer.

### Ein nativer FFI-Generator

Native Interoperabilität bleibt wertvoll, aber das Ziel ist nicht:

```text
C#
 ↓
P/Invoke
 ↓
Rust native DLL
```

Das Ziel ist:

```text
C#
  ╲
   CLR
  ╱
Rust
```

### Ein Ersatz für .NET

FerrumWeave existiert, weil das .NET-Ökosystem wertvoll ist. Es soll die verfügbaren Sprachoptionen erweitern.

### Ein Ersatz für natives Rust

Es wird immer gute Gründe geben, Rust direkt zu nativem Code zu kompilieren. Ein CLR-Target wäre eine zusätzliche Deployment- und Interoperabilitätsoption.

---

# Die gewünschte Developer Experience

Ein Rust-Projekt soll sich langfristig in einer .NET-Solution zu Hause fühlen:

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

Vertraute Befehle sollen vertraut bleiben:

```bash
dotnet restore
dotnet build
dotnet run
dotnet test
dotnet publish
dotnet pack
```

Ein Projekt könnte später etwa so aussehen:

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

Cargo und crates.io sollen dort ihren Platz behalten, wo Rust-Abhängigkeiten sie benötigen. NuGet und MSBuild sollen weiterhin das tun, was sie für .NET-Abhängigkeiten gut können.

FerrumWeave soll diese Ökosysteme verbinden, ohne so zu tun, als gäbe es eines davon nicht.

---

# .NET-Bibliotheken aus Rust verwenden

Ein zentrales Ziel ist, .NET-APIs zu natürlichen Teilnehmern in Rust-Code zu machen.

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

Nicht die exakte Syntax ist entscheidend, sondern die Eigenschaft:

> `System.Console`, `System.String`, `System.IO.File` und benutzerdefinierte .NET-Typen sollen als CLR-Typen und -Member verstanden werden, nicht als undurchsichtige native Bibliotheken hinter einer manuell gepflegten FFI-Schicht.

Dasselbe Prinzip soll für NuGet-Pakete und ProjectReference gelten.

```xml
<ProjectReference Include="../Domain/Domain.csproj" />
```

soll die öffentliche CLR-Oberfläche von `Domain` für Rust verfügbar machen.

---

# Rust-Bibliotheken aus anderen .NET-Sprachen

Interoperabilität muss in beide Richtungen funktionieren.

Rust soll öffentliche CLR-Typen definieren können, die andere .NET-Sprachen konsumieren.

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

Verschiedene Quellsprachen. Ein Runtime-Vertrag.

Das ist der Interoperabilitätsstandard, den FerrumWeave erreichen möchte.

---

# Rust-Typen und CLR-Typen

Rust und die CLR besitzen grundlegend unterschiedliche Objekt- und Speichermodelle. Dieser Unterschied darf nicht versteckt werden.

Rust kennt unter anderem:

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

Die CLR kennt:

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

FerrumWeave soll keines der Modelle abschwächen, nur damit beide gleich aussehen. Stattdessen braucht es nachvollziehbare Abbildungen.

Natürliche Beispiele:

```text
System.Int32   ↔ i32
System.Int64   ↔ i64
System.Boolean ↔ bool
System.Double  ↔ f64
```

Andere Fälle benötigen explizite Semantik:

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

Diese Semantik korrekt zu definieren gehört zu den zentralen Engineering-Herausforderungen des Projekts.

---

# Sicherheit ohne Aufgabe der Interoperabilität

Unternehmenssoftware bekommt selten die Chance, von null zu beginnen. Organisationen besitzen funktionierende Anwendungen, Datenbanken, Geschäftsregeln, APIs, Pakete, Frameworks, Teams und Betriebswissen.

Eine sicherere Sprache lässt sich leichter einführen, wenn dafür nicht alles Bestehende aufgegeben werden muss.

FerrumWeave untersucht daher folgende Idee:

> **Rusts Sicherheitsmodell in neue und kritische .NET-Komponenten bringen und gleichzeitig die Interoperabilität mit bestehenden .NET-Investitionen erhalten.**

---

# Warum der Name FerrumWeave?

**Ferrum** ist lateinisch für Eisen. Rust ist die Oxidation von Eisen.

**Weave** bedeutet, einzelne Fäden zu einer verbundenen Struktur zu verweben.

```text
Ferrum
   │
   └── iron → rust

Weave
   │
   └── interconnection → network → ecosystem
```

Zusammen:

> **FerrumWeave steht für Rust, das in das .NET-Ökosystem eingewebt wird.**

Der Name ist bewusst unabhängig von den Marken Rust und .NET.

---

# Auf bestehender Arbeit aufbauen

FerrumWeave möchte nicht mit einem weiteren Rust-Compiler beginnen.

Das Rust-Ökosystem bietet bereits wertvolle Infrastruktur:

- `rustc` für Parsing, Type Checking, Borrow Checking, MIR und Sprachsemantik;
- `rust-analyzer` für moderne Rust-Codeanalyse und Entwicklungswerkzeuge.

Auch für Rust→CLR existiert wichtiges Prior Art, insbesondere das experimentelle Projekt `rustc_codegen_clr`.

.NET bietet seinerseits ausgereifte Infrastruktur für CLR, CTS, Assembly-Metadaten, MSBuild, NuGet, `dotnet` CLI, SDK-Projekte, Debugging und Tooling.

Die Strategie lautet daher:

> **Integrieren, bevor wir neu erfinden.**

Wo möglich, sollten Verbesserungen upstream beigetragen und nicht dauerhaft als private Forks gepflegt werden.

---

# Frühe Architektur

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

Die geplante Repository-Struktur ist unter [Repository layout](docs/architecture/repository-layout.md) dokumentiert.

Diese Architektur ist bewusst vorläufig. Ausführbare Evidenz hat Vorrang vor Diagrammen.

---

# Erster Nachweis

Der erste sinnvolle Meilenstein ist absichtlich klein:

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

FerrumWeave soll daraus eine gültige .NET-Assembly erzeugen und `System.Console.WriteLine` über die CLR ausführen.

Damit würden gleichzeitig `.rsproj`, MSBuild, `dotnet` CLI, `rustc`, Rust→CIL, CLR-Metadaten, CTS-Interop und .NET BCL validiert.

Das Projekt soll vertikal aus funktionierenden Verträgen wachsen, statt das gesamte .NET-Ökosystem zu modellieren, bevor überhaupt etwas läuft.

---

# Langfristiger Erfolg

FerrumWeave ist nicht schon dann erfolgreich, wenn Rust „etwas CIL“ ausgeben kann.

Die tiefere Frage lautet:

> **Kann ein .NET-Entwickler Rust als ernsthafte weitere Sprachwahl in einem bestehenden .NET-System behandeln?**

Ein reifes FerrumWeave soll Szenarien wie diese selbstverständlich machen:

```text
C# → Rust
Rust → C#
VB → Rust
Rust → F#
Rust → NuGet package
.NET project → .rsproj
.rsproj → .NET project
```

mit denselben Erwartungen an Build, Referenzen, Pakete, Typen, Exceptions, Debugging, Tests, Tooling und Publishing.

---

# Projektprinzipien

### Rust bewahren

Keinen unnötigen Rust-Dialekt schaffen.

### .NET bewahren

CLR, CTS, Metadaten, MSBuild, NuGet und bestehende Plattformverträge nutzen.

### Schrittweise interoperieren

Eine zwanzig Jahre alte Anwendung sollte nicht neu geschrieben werden müssen, um von einer neuen Rust-Komponente zu profitieren.

### Safe Rust bevorzugen

`unsafe` ist legitim, aber der Bereich, in dem Rusts normale Sicherheitsgarantien gelten, soll möglichst groß bleiben.

### Grenzen explizit machen

Rust Ownership und CLR Garbage Collection sind unterschiedliche Systeme. Schwierige semantische Grenzen sollen bewusst modelliert werden.

### Wo sinnvoll upstream arbeiten

Ein nachhaltiges Ökosystem ist besser als dauerhafte Forks.

### Evidenz vor Behauptungen

Compiler-Korrektheit entsteht aus ausführbaren Tests, differenzieller Validierung, Konformitätsevidenz und realen Anwendungen — nicht aus optimistischen Diagrammen.

### Kompatibilität ist ein Vertrag

Behauptete .NET-Funktionalität muss durch wiederholbare Tests geschützt sein.

---

# Open Source von Anfang an

FerrumWeave wird offen entwickelt und steht wahlweise unter:

- [MIT License](LICENSE-MIT); oder
- [Apache License, Version 2.0](LICENSE-APACHE).

Der Compiler soll dem damit erzeugten Programm keine FerrumWeave-Lizenz aufzwingen.

---

# Governance-Vision

FerrumWeave beginnt als unabhängiges Projekt. Sollte ein echtes Multi-Unternehmens- und Multi-Community-Ökosystem entstehen, muss die Governance sich vom ursprünglichen Schöpfer lösen können.

Ein langfristig neutrales Zuhause — etwa bei einer passenden Open-Source-Stiftung — wäre Erfolg, kein Eigentumsverlust.

Dafür braucht das Projekt transparente technische Entscheidungen, saubere IP-Provenienz, Contributor-Nachvollziehbarkeit, übertragbare Assets und offene Governance.

Derzeit besteht keine Stiftungszugehörigkeit.

---

# Beziehung zu Rust und .NET

FerrumWeave ist ein unabhängiges experimentelles Projekt und derzeit weder mit Microsoft, der .NET Foundation, der Rust Foundation noch dem Rust Project verbunden, von ihnen gesponsert oder unterstützt.

„Rust“ und „.NET“ werden verwendet, um die Technologien korrekt zu benennen, mit denen FerrumWeave interoperieren soll.

---

# Status

**Pre-Alpha / Architekturfindung.**

Das Repository beginnt bewusst mit Problem, Prinzipien, Zielverträgen und Engineering-Grenzen, bevor eine vollständige Sprachimplementierung behauptet wird.

Der erste Fokus ist ein vertrauenswürdiger vertikaler Slice:

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

Danach: ein Vertrag nach dem anderen.

---

# Die Idee in einem Satz

> **FerrumWeave möchte Rust zu einer First-Class-.NET-Sprache machen, damit Organisationen Rusts Sicherheitsmodell in neue und kritische Komponenten einführen können, ohne bestehende Software, Bibliotheken, Sprachen und Betriebswissen aufzugeben.**

---

## Ein sehr altes Hello World. Ein sehr moderner Compiler.

Eine Visual-Basic-Anwendung von vor Jahrzehnten soll irgendwann heutigen Safe-Rust-Code aufrufen können.

Nicht über eine Service-Grenze.

Nicht durch einen Rewrite.

Nicht, weil eine Sprache vorgibt, die andere zu sein.

Sondern weil beide die Sprache sprechen können, die die CLR genau für die Kommunikation zwischen Sprachen bereitstellt.

Das ist das Gewebe.