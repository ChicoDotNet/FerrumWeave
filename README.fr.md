<!--
translation-of: README.md
locale: fr
source-revision: 6fb75d076d59b4e1932b685e441487f6caedb36d
-->

[English](README.md) · [Deutsch](README.de.md) · [Español](README.es.md) · **Français** · [Italiano](README.it.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [简体中文](README.zh-Hans.md) · [日本語](README.ja.md)

> La documentation de FerrumWeave est maintenue en plusieurs langues, car l’interopérabilité concerne aussi les personnes. Le document anglais sans suffixe reste la source canonique lorsqu’une traduction prend temporairement du retard.

<img src="assets/brand/hero/ferrumweave-readme-cover.png" alt="FerrumWeave — amener Rust dans l’écosystème .NET" width="100%" />

# FerrumWeave

**Amener Rust dans l’écosystème des langages .NET.**

**Ressources du projet :** [Structure du dépôt](docs/architecture/repository-layout.md) · [Plan de publication des templates](docs/roadmap/template-release-plan.md) · [Site du projet](https://chicodotnet.github.io/FerrumWeave/fr/)

FerrumWeave est un effort open source expérimental visant à faire de Rust un langage de première classe sur la plateforme .NET : compiler du code source Rust en assemblies .NET, participer au Common Type System, consommer des bibliothèques .NET existantes et interopérer naturellement avec C#, F#, Visual Basic et les autres langages construits autour du CLR.

À terme, l’expérience développeur devrait sembler naturelle :

```bash
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

Avec du code Rust ressemblant conceptuellement à :

```rust
use dotnet::System::*;
use dotnet::Result;

fn main() -> Result<()> {
    Console::WriteLine("Hello from Rust on .NET")?;
    Ok(())
}
```

Et produisant une vraie assembly .NET :

```text
HelloFerrum.dll
```

exécutée par le runtime .NET.

> **FerrumWeave n’en est qu’au début de ce parcours.**
>
> Les exemples de ce README décrivent l’expérience développeur et la direction architecturale visées. Ils ne constituent pas encore des déclarations de fonctionnalités publiées.

---

## Pourquoi FerrumWeave ?

L’une des idées les plus durables de .NET n’a jamais été C# en lui-même.

C’était l’idée que **des langages différents pouvaient se rencontrer sur un runtime commun**.

Depuis des décennies, les développeurs peuvent écrire des logiciels dans des langages aux syntaxes, philosophies et histoires très différentes tout en partageant la même plateforme :

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
- et bien d’autres

Une bibliothèque écrite dans un langage pouvait souvent être consommée depuis un autre, car la frontière importante n’était pas le langage source.

Cette frontière était la **Common Language Infrastructure**, le **Common Type System**, les métadonnées des assemblies et le CLR.

FerrumWeave pose une question simple :

> **Et si Rust pouvait rejoindre cette famille de langages ?**

Pas seulement en appelant une bibliothèque Rust native via FFI.

Pas seulement en hébergeant le CLR depuis un exécutable Rust.

Pas en traduisant Rust en C#.

Mais en compilant Rust vers le même monde de CIL, assemblies, métadonnées, types, références, packages, tooling et interopérabilité runtime qui a rendu possible le .NET multilangage.

---

## Pourquoi Rust ?

Rust apporte un ensemble différent de garanties au développement logiciel.

Son modèle d’ownership, le borrow checker, son système de types fort, la gestion explicite des erreurs et son attention à la sûreté mémoire et à la concurrence permettent de détecter des catégories entières de défauts avant la production.

Rust ne rend **pas** les logiciels incapables d’échouer.

La logique peut être fausse. Des fichiers peuvent disparaître. Les réseaux peuvent tomber. Les bases de données peuvent contenir de mauvaises données. Les programmes peuvent paniquer. Le code `unsafe` existe.

Mais Rust peut déplacer des classes importantes de défaillances de :

```text
production
```

vers :

```text
compilation
```

Cette différence compte, surtout dans les systèmes métier à longue durée de vie.

---

## L’opportunité dans le logiciel d’entreprise

Il existe d’immenses systèmes .NET qui délivrent de la valeur depuis dix, quinze ou vingt-cinq ans.

Ils peuvent contenir :

```text
ERP.vbproj
Accounting.csproj
Reporting.fsproj
LegacyIntegration.vbproj
```

Les conversations de modernisation commencent trop souvent par :

> « Il faut tout réécrire. »

FerrumWeave part d’une autre idée :

> **Préserver ce qui fonctionne. Renforcer ce qui vient ensuite.**

Imaginez ajouter :

```text
RiskEngine.rsproj
```

au même système.

L’application Visual Basic existante n’a pas besoin de disparaître. Le modèle de domaine C# n’a pas besoin d’être réécrit. Le moteur de reporting F# n’a pas besoin d’un nouveau protocole d’intégration.

À terme :

```text
ERP.vbproj
     │
     ▼
RiskEngine.rsproj
     │
     ▼
Accounting.csproj
```

pourraient communiquer via le même système de types .NET.

Une application Visual Basic vieille de plusieurs décennies pourrait appeler du nouveau code Rust. Rust pourrait consommer une assembly de domaine écrite en C#. F# pourrait consommer un type implémenté en Rust.

L’unité de migration devient **le composant**, pas l’application entière.

Voilà la vision.

---

# L’objectif

FerrumWeave vise à rendre possible :

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

Le chemin de compilation Rust visé est approximativement :

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

Rust reste Rust.

Le CLR reste le CLR.

FerrumWeave doit les relier plutôt que réinventer inutilement l’un ou l’autre.

---

# Ce que FerrumWeave n’est pas

FerrumWeave n’a **pas** vocation à être :

### Un nouveau langage ressemblant à Rust

L’objectif est de préserver Rust et de bénéficier de l’écosystème de son compilateur.

```rust
match
traits
lifetimes
ownership
borrowing
async
Result<T, E>
```

doivent rester des concepts Rust, pas des approximations recréées ailleurs.

### Un wrapper Rust autour de `dotnet`

Exécuter Cargo depuis une cible MSBuild peut être utile, mais cela ne suffit pas à faire de Rust un langage .NET.

FerrumWeave vise plus profond.

### Un générateur de FFI native

L’interopérabilité native reste utile, mais la cible n’est pas :

```text
C#
 ↓
P/Invoke
 ↓
Rust native DLL
```

La cible est :

```text
C#
  ╲
   CLR
  ╱
Rust
```

### Un remplacement de .NET

FerrumWeave existe précisément parce que l’écosystème .NET a de la valeur. L’objectif est d’élargir ses choix de langages.

### Un remplacement de Rust natif

Il existera toujours d’excellentes raisons de compiler Rust directement en code natif. Une cible CLR serait une option supplémentaire de déploiement et d’interopérabilité.

---

# L’expérience développeur souhaitée

Un projet Rust devrait à terme se sentir chez lui dans une solution .NET :

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

Les commandes familières doivent rester familières :

```bash
dotnet restore
dotnet build
dotnet run
dotnet test
dotnet publish
dotnet pack
```

Un projet pourrait ressembler à :

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

Cargo et crates.io doivent garder leur place lorsque les dépendances Rust en ont besoin. NuGet et MSBuild doivent continuer à faire ce qu’ils savent déjà bien faire pour les dépendances .NET.

FerrumWeave doit relier ces écosystèmes sans prétendre que l’un d’eux n’existe pas.

---

# Bibliothèques .NET depuis Rust

Un objectif central est que les APIs .NET deviennent des participants naturels du code Rust.

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

La propriété importante n’est pas la syntaxe exacte, mais celle-ci :

> `System.Console`, `System.String`, `System.IO.File` et les types .NET définis par l’utilisateur doivent être compris comme des types et membres CLR, et non comme des bibliothèques natives opaques cachées derrière une couche FFI maintenue manuellement.

Le même principe doit s’appliquer aux packages NuGet et aux références de projet.

---

# Bibliothèques Rust depuis d’autres langages .NET

L’interopérabilité doit fonctionner dans les deux sens.

Rust doit pouvoir définir des types publics orientés CLR que les autres langages .NET consomment.

Rust :

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

C# :

```csharp
var engine = new RiskEngine();
var score = engine.Calculate(customer);
```

Visual Basic :

```vb
Dim engine = New RiskEngine()
Dim score = engine.Calculate(customer)
```

F# :

```fsharp
let engine = RiskEngine()
let score = engine.Calculate(customer)
```

Des langages sources différents. Un seul contrat runtime.

C’est le niveau d’interopérabilité que FerrumWeave veut atteindre.

---

# Types Rust et types CLR

Rust et le CLR ont des modèles objets et mémoire fondamentalement différents. Cette différence ne doit pas être cachée.

Rust possède des concepts tels que :

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

Le CLR possède :

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

FerrumWeave ne doit affaiblir aucun modèle simplement pour leur donner l’air identique. Il doit définir des mappings fondés sur des principes explicites.

Certains mappings sont naturels :

```text
System.Int32   ↔ i32
System.Int64   ↔ i64
System.Boolean ↔ bool
System.Double  ↔ f64
```

D’autres exigent une sémantique explicite :

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

Définir correctement ces sémantiques constitue l’un des défis d’ingénierie centraux du projet.

---

# Sécurité sans abandonner l’interopérabilité

Les logiciels d’entreprise ont rarement le luxe de repartir de zéro.

Les organisations disposent déjà d’applications, de bases de données, de règles métier, d’APIs, de packages, de frameworks, de développeurs et de connaissances opérationnelles.

Un langage plus sûr est beaucoup plus facile à adopter si son adoption n’oblige pas à abandonner tout le reste.

FerrumWeave explore donc cette proposition :

> **Apporter le modèle de sûreté de Rust aux composants .NET nouveaux et critiques tout en préservant l’interopérabilité avec les investissements .NET existants.**

---

# Pourquoi le nom FerrumWeave ?

**Ferrum** signifie fer en latin. Rust est l’oxydation du fer.

**Weave** signifie entrelacer des fils séparés dans une structure connectée.

```text
Ferrum
   │
   └── iron → rust

Weave
   │
   └── interconnection → network → ecosystem
```

Ensemble :

> **FerrumWeave représente Rust tissé dans l’écosystème .NET.**

Le nom est volontairement indépendant des marques Rust et .NET.

---

# Construire sur l’existant

FerrumWeave n’a pas l’intention de commencer par écrire un nouveau compilateur Rust.

L’écosystème Rust fournit déjà une infrastructure précieuse :

- `rustc` pour le parsing, le type checking, le borrow checking, MIR et la sémantique du langage ;
- `rust-analyzer` pour l’analyse moderne du code Rust et le tooling de développement.

Il existe aussi un prior art important vers le CLR, notamment le projet expérimental `rustc_codegen_clr`.

.NET fournit déjà une infrastructure mature pour le CLR, le CTS, les métadonnées, MSBuild, NuGet, le CLI `dotnet`, les projets SDK-style, le debugging et le tooling.

La stratégie de FerrumWeave est donc :

> **Intégrer avant de réinventer.**

Lorsque c’est possible, les améliorations doivent être contribuées upstream plutôt que maintenues indéfiniment dans des forks privés.

---

# Architecture initiale

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

La structure cible du dépôt est documentée dans [Repository layout](docs/architecture/repository-layout.md).

Cette architecture est volontairement provisoire. Les preuves exécutables priment sur les diagrammes.

---

# Première preuve

Le premier jalon significatif est volontairement petit :

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

FerrumWeave doit produire une assembly .NET valide et exécuter `System.Console.WriteLine` via le CLR.

Ce résultat validerait à la fois `.rsproj`, MSBuild, le CLI `dotnet`, `rustc`, Rust→CIL, les métadonnées CLR, l’interop CTS et la BCL .NET.

Le projet grandira verticalement à partir de contrats exécutables plutôt que d’essayer de modéliser tout l’écosystème .NET avant que quoi que ce soit ne fonctionne.

---

# Succès à long terme

FerrumWeave ne sera pas considéré comme réussi simplement parce que Rust peut émettre « un peu de CIL ».

La question plus profonde est :

> **Un développeur .NET peut-il traiter Rust comme un choix de langage sérieux au sein d’un système .NET existant ?**

Un FerrumWeave mature doit rendre ordinaires des scénarios comme :

```text
C# → Rust
Rust → C#
VB → Rust
Rust → F#
Rust → NuGet package
.NET project → .rsproj
.rsproj → .NET project
```

avec les attentes habituelles autour du build, des références, des packages, des types, des exceptions, du debugging, des tests, du tooling et du publishing.

---

# Principes du projet

### Préserver Rust

Éviter de créer un dialecte Rust inutile.

### Préserver .NET

Utiliser le CLR, le CTS, les métadonnées, MSBuild, NuGet et les contrats existants de la plateforme.

### Interopérer progressivement

Une application vieille de vingt ans ne devrait pas devoir être réécrite pour bénéficier d’un nouveau composant Rust.

### Préférer Safe Rust

`unsafe` est une partie légitime de Rust, mais le projet doit maximiser la zone où les garanties normales de sûreté restent significatives.

### Rendre les frontières explicites

L’ownership de Rust et le garbage collection du CLR sont des systèmes différents. Les frontières difficiles doivent être modélisées délibérément.

### Contribuer upstream lorsque c’est pratique

Un écosystème durable vaut mieux que des forks permanents.

### Les preuves avant les affirmations

La correction du compilateur doit venir de tests exécutables, de validation différentielle, de preuves de conformité et d’applications réelles — pas de diagrammes optimistes.

### La compatibilité est un contrat

Tout comportement .NET revendiqué doit être protégé par des tests répétables.

---

# Open source dès le premier jour

FerrumWeave est développé ouvertement et distribué, au choix, sous :

- [MIT License](LICENSE-MIT) ; ou
- [Apache License, Version 2.0](LICENSE-APACHE).

L’utilisation du compilateur ne doit pas imposer la licence FerrumWeave au logiciel compilé.

---

# Vision de gouvernance

FerrumWeave commence comme projet indépendant.

S’il devient suffisamment utile pour créer un véritable écosystème multi-entreprises et multi-communautés, sa gouvernance doit pouvoir devenir indépendante de son créateur initial.

Un foyer neutre à long terme, éventuellement au sein d’une fondation open source appropriée, serait un succès et non une perte de propriété.

Le projet doit donc privilégier décisions techniques transparentes, provenance IP propre, traçabilité des contributeurs, actifs transférables et gouvernance ouverte.

Aucune affiliation à une fondation n’existe actuellement.

---

# Relation avec Rust et .NET

FerrumWeave est un projet expérimental indépendant.

Il n’est actuellement ni affilié, ni sponsorisé, ni approuvé par Microsoft, la .NET Foundation, la Rust Foundation ou le Rust Project.

« Rust » et « .NET » sont utilisés pour décrire précisément les technologies avec lesquelles FerrumWeave cherche à interopérer.

---

# État

**Pré-alpha / exploration architecturale.**

Le dépôt commence volontairement par le problème, les principes, les contrats cibles et les frontières d’ingénierie avant de revendiquer une implémentation complète du langage.

Le premier objectif est un vertical slice digne de confiance :

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

Puis, un contrat après l’autre.

---

# L’idée en une phrase

> **FerrumWeave vise à faire de Rust un langage .NET de première classe afin que les organisations puissent introduire le modèle de sûreté de Rust dans de nouveaux composants critiques sans abandonner leurs logiciels, bibliothèques, langages et connaissances opérationnelles existants.**

---

## Un très ancien Hello World. Un compilateur très moderne.

Une application Visual Basic écrite il y a plusieurs décennies devrait un jour pouvoir appeler du code Safe Rust écrit aujourd’hui.

Pas via une frontière de services.

Pas via une réécriture.

Pas parce qu’un langage prétend être l’autre.

Parce que les deux peuvent parler le langage que le CLR a été conçu pour fournir entre les langages.

Voilà le tissage.