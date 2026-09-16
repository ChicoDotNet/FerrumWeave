<!--
translation-of: README.md
locale: es
source-revision: 6fb75d076d59b4e1932b685e441487f6caedb36d
-->

[English](README.md) · [Deutsch](README.de.md) · **Español** · [Français](README.fr.md) · [Italiano](README.it.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [简体中文](README.zh-Hans.md) · [日本語](README.ja.md)

> La documentación de FerrumWeave se mantiene en varios idiomas porque la interoperabilidad también trata de personas. La versión inglesa sin sufijo es la fuente canónica cuando una traducción queda temporalmente desactualizada.

<img src="assets/brand/hero/ferrumweave-readme-cover.png" alt="FerrumWeave — llevando Rust al ecosistema .NET" width="100%" />

# FerrumWeave

**Llevando Rust al ecosistema de lenguajes .NET.**

**Recursos del proyecto:** [Estructura del repositorio](docs/architecture/repository-layout.md) · [Plan de releases de templates](docs/roadmap/template-release-plan.md) · [Sitio del proyecto](https://chicodotnet.github.io/FerrumWeave/es/)

FerrumWeave es un esfuerzo experimental y open source para convertir a Rust en un lenguaje de primera clase para la plataforma .NET: compilar código fuente Rust a assemblies .NET, participar en el Common Type System, consumir librerías .NET existentes e interoperar naturalmente con C#, F#, Visual Basic y otros lenguajes construidos alrededor del CLR.

La experiencia de desarrollo a largo plazo debería sentirse natural:

```bash
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

Con código Rust conceptualmente parecido a:

```rust
use dotnet::System::*;
use dotnet::Result;

fn main() -> Result<()> {
    Console::WriteLine("Hello from Rust on .NET")?;
    Ok(())
}
```

Y produciendo un assembly .NET real:

```text
HelloFerrum.dll
```

ejecutado por el runtime de .NET.

> **FerrumWeave está al principio de ese camino.**
>
> Los ejemplos de este README describen la experiencia de desarrollo y la dirección arquitectónica buscadas. No constituyen todavía afirmaciones de funcionalidad liberada.

---

## ¿Por qué FerrumWeave?

Una de las ideas más duraderas de .NET nunca fue C# por sí mismo.

Fue la idea de que **distintos lenguajes podían encontrarse en un runtime común**.

Durante décadas, los desarrolladores han podido escribir software con lenguajes de sintaxis, filosofías e historias muy distintas compartiendo la misma plataforma:

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
- y muchos más

Una librería escrita en un lenguaje con frecuencia podía consumirse desde otro porque la frontera importante no era el lenguaje fuente.

Era la **Common Language Infrastructure**, el **Common Type System**, los metadatos de assemblies y el CLR.

FerrumWeave hace una pregunta sencilla:

> **¿Qué pasaría si Rust pudiera integrarse a esa familia de lenguajes?**

No solamente llamando una librería Rust nativa mediante FFI.

No solamente hospedando el CLR desde un ejecutable Rust.

No traduciendo Rust a C#.

Sino compilando Rust al mismo mundo de CIL, assemblies, metadata, tipos, referencias, paquetes, tooling e interoperabilidad de runtime que hizo posible el .NET multilenguaje desde el inicio.

---

## ¿Por qué Rust?

Rust aporta un conjunto diferente de garantías al desarrollo de software.

Su modelo de ownership, borrow checker, sistema de tipos fuerte, manejo explícito de errores y enfoque en seguridad de memoria y concurrencia permiten detectar categorías completas de defectos antes de llegar a producción.

Rust **no** vuelve imposible que el software falle.

La lógica puede estar equivocada. Los archivos pueden desaparecer. Las redes pueden fallar. Las bases de datos pueden contener datos incorrectos. Los programas pueden hacer panic. Existe código `unsafe`.

Pero Rust puede mover clases importantes de fallos desde:

```text
producción
```

hacia:

```text
tiempo de compilación
```

Esa diferencia importa, especialmente en sistemas de negocio de larga vida.

---

## La oportunidad en software empresarial

Existen enormes sistemas .NET que llevan diez, quince o veinticinco años generando valor.

Pueden contener:

```text
ERP.vbproj
Accounting.csproj
Reporting.fsproj
LegacyIntegration.vbproj
```

Demasiadas conversaciones de modernización comienzan con:

> "Hay que reescribirlo."

FerrumWeave parte de otra idea:

> **Conserva lo que funciona. Fortalece lo que viene después.**

Imagina agregar:

```text
RiskEngine.rsproj
```

a ese mismo sistema.

La aplicación existente en Visual Basic no necesita desaparecer.

El modelo de dominio en C# no necesita reescribirse.

El motor de reportes en F# no necesita un nuevo protocolo de integración.

En cambio:

```text
ERP.vbproj
     │
     ▼
RiskEngine.rsproj
     │
     ▼
Accounting.csproj
```

podrían comunicarse algún día mediante el mismo sistema de tipos de .NET.

Una aplicación Visual Basic con décadas de vida podría llamar código nuevo escrito en Rust.

Rust podría consumir un assembly de dominio escrito en C#.

F# podría consumir un tipo implementado en Rust.

La unidad de migración pasa a ser **el componente**, no la aplicación completa.

Ésa es la visión.

---

# El objetivo

FerrumWeave busca hacer posible lo siguiente:

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

Para Rust, la ruta de compilación buscada es aproximadamente:

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

Rust sigue siendo Rust.

El CLR sigue siendo el CLR.

FerrumWeave debe conectarlos, no reinventarlos innecesariamente.

---

# Lo que FerrumWeave no es

FerrumWeave **no** pretende ser:

### Un nuevo lenguaje parecido a Rust

El objetivo es preservar Rust y beneficiarnos del ecosistema existente de su compilador.

```rust
match
traits
lifetimes
ownership
borrowing
async
Result<T, E>
```

deben seguir siendo conceptos de Rust, no aproximaciones recreadas en otro compilador.

### Un wrapper de Rust alrededor de `dotnet`

Ejecutar Cargo desde un target de MSBuild podría ser útil, pero por sí solo no convertiría a Rust en un lenguaje .NET.

FerrumWeave apunta más profundo.

### Un generador de FFI nativo

La interoperabilidad nativa sigue siendo valiosa, pero el objetivo no es:

```text
C#
 ↓
P/Invoke
 ↓
Rust native DLL
```

La meta es:

```text
C#
  ╲
   CLR
  ╱
Rust
```

### Un reemplazo de .NET

FerrumWeave existe precisamente porque el ecosistema .NET es valioso.

La intención es ampliar sus opciones de lenguaje.

### Un reemplazo de Rust nativo

Siempre habrá excelentes razones para compilar Rust directamente a código nativo.

Un target CLR sería otra opción de deployment e interoperabilidad, no una afirmación de que toda carga Rust deba vivir sobre .NET.

---

# La experiencia de desarrollo buscada

Con el tiempo, un proyecto Rust debería sentirse en casa dentro de una solución .NET.

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

Y los comandos familiares deberían seguir siéndolo:

```bash
dotnet restore
dotnet build
dotnet run
dotnet test
dotnet publish
dotnet pack
```

Un proyecto podría llegar a verse así:

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

Cargo y crates.io deben seguir teniendo su lugar donde las dependencias Rust los requieran.

NuGet y MSBuild deben continuar haciendo lo que ya hacen bien para dependencias .NET.

FerrumWeave debe conectar ambos ecosistemas sin fingir que alguno de ellos no existe.

---

# Librerías .NET desde Rust

Un objetivo central es que las APIs de .NET se vuelvan participantes naturales del código Rust.

Conceptualmente:

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

Lo importante no es la sintaxis exacta del ejemplo.

Lo importante es esto:

> `System.Console`, `System.String`, `System.IO.File` y los tipos .NET definidos por usuarios deben entenderse como tipos y miembros CLR, no como librerías nativas opacas escondidas detrás de una capa FFI mantenida manualmente.

El mismo principio debería aplicarse con el tiempo a paquetes NuGet y referencias de proyecto.

Por ejemplo:

```xml
<ProjectReference Include="../Domain/Domain.csproj" />
```

debería hacer disponible para Rust la superficie CLR pública de `Domain`.

Conceptualmente:

```rust
use Domain::*;

let customer = Customer::new("Ada");
let risk = customer.CalculateRisk()?;
```

---

# Librerías Rust desde otros lenguajes .NET

La interoperabilidad debe funcionar en ambas direcciones.

Rust debe llegar a poder definir tipos públicos orientados al CLR que otros lenguajes .NET puedan consumir.

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

Lenguajes fuente distintos.

Un solo contrato de runtime.

Ése es el estándar de interoperabilidad al que FerrumWeave quiere llegar.

---

# Tipos Rust y tipos CLR

Rust y el CLR tienen modelos de objetos y memoria fundamentalmente distintos.

Esa diferencia no debe ocultarse.

Rust tiene conceptos como:

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

El CLR tiene:

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

FerrumWeave no debe debilitar ninguno de los modelos solamente para hacer que parezcan idénticos.

En su lugar debe definir mapeos con principios claros entre ambos.

Algunos pueden ser naturales:

```text
System.Int32   ↔ i32
System.Int64   ↔ i64
System.Boolean ↔ bool
System.Double  ↔ f64
```

Otros requieren semántica explícita:

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

Definir correctamente esas semánticas es uno de los retos centrales de ingeniería del proyecto.

---

# Seguridad sin abandonar interoperabilidad

FerrumWeave parte de una observación práctica:

El software empresarial rara vez tiene el lujo de empezar desde cero.

Las organizaciones ya tienen aplicaciones funcionales, bases de datos, reglas de negocio, APIs, paquetes, frameworks, desarrolladores y conocimiento operativo.

Adoptar un lenguaje más seguro es mucho más fácil cuando hacerlo no exige abandonar todo lo que lo rodea.

Por ello FerrumWeave explora esta propuesta:

> **Llevar el modelo de seguridad de Rust a componentes .NET nuevos y críticos, preservando la interoperabilidad con las inversiones .NET existentes.**

Eso puede hacer útil a Rust no sólo para programación de sistemas, sino para modernización incremental de software de negocio de larga vida.

---

# ¿Por qué el nombre FerrumWeave?

**Ferrum** significa hierro en latín.

Rust es la oxidación del hierro.

**Weave** significa entrelazar hilos separados para formar una estructura conectada.

El nombre reúne dos ideas:

```text
Ferrum
   │
   └── iron → rust

Weave
   │
   └── interconnection → network → ecosystem
```

Juntas:

> **FerrumWeave representa a Rust entretejido en el ecosistema .NET.**

El nombre es intencionalmente independiente de las marcas Rust y .NET.

Las tecnologías con las que interopera pueden describirse con precisión sin que el proyecto tenga que hacerse pasar por ninguno de los dos ecosistemas.

---

# Construyendo sobre trabajo existente

FerrumWeave no pretende comenzar escribiendo otro compilador de Rust.

El ecosistema Rust ya ofrece infraestructura enormemente valiosa:

- `rustc` para parsing, type checking, borrow checking, MIR y semántica del lenguaje;
- `rust-analyzer` para análisis moderno de código Rust y tooling de desarrollo.

También existe prior art importante en compilación de Rust hacia CLR, especialmente el proyecto experimental `rustc_codegen_clr`.

De la misma forma, .NET ya ofrece infraestructura madura para:

- CLR;
- Common Type System;
- metadata de assemblies;
- MSBuild;
- NuGet;
- CLI `dotnet`;
- proyectos SDK-style;
- debugging y tooling.

La estrategia de FerrumWeave es, por tanto:

> **Integrar antes de reinventar.**

Cuando sea posible, las mejoras deberían contribuirse upstream en lugar de mantenerse para siempre como forks privados.

---

# Arquitectura inicial

El proyecto contempla varias áreas principales:

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

La estructura objetivo del repositorio y sus reglas de materialización están documentadas en [Repository layout](docs/architecture/repository-layout.md).

Esta arquitectura es deliberadamente provisional.

La evidencia ejecutable tendrá más peso que los diagramas.

---

# Primera prueba

El primer milestone significativo es deliberadamente pequeño.

Dado:

```bash
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

y:

```rust
use dotnet::System::*;

fn main() -> Result<()> {
    Console::WriteLine("Hello from FerrumWeave")?;
    Ok(())
}
```

FerrumWeave debe producir un assembly .NET válido y ejecutar `System.Console.WriteLine` mediante el CLR.

Ese resultado validaría varias suposiciones a la vez:

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

El proyecto crecerá verticalmente a partir de contratos funcionales como éste, en lugar de intentar modelar todo el ecosistema .NET antes de que algo ejecute.

---

# Éxito a largo plazo

FerrumWeave no se considerará exitoso sólo porque:

```text
Rust can emit some CIL.
```

El criterio más profundo es:

> **¿Puede un desarrollador .NET tratar a Rust como otra opción seria de lenguaje dentro de un sistema .NET existente?**

Un FerrumWeave maduro debería volver normales escenarios como:

```text
C# → Rust
Rust → C#
VB → Rust
Rust → F#
Rust → NuGet package
.NET project → .rsproj
.rsproj → .NET project
```

con las mismas expectativas que hoy existen alrededor de:

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

Es un objetivo mucho mayor que compilar "Hello World".

Y es la razón de existir de este proyecto.

---

# Principios del proyecto

FerrumWeave pretende seguir varios principios de ingeniería desde el inicio.

### Preservar Rust

Evitar crear un dialecto innecesario de Rust.

### Preservar .NET

Usar CLR, CTS, metadata, MSBuild, NuGet y los demás contratos existentes de la plataforma en lugar de reemplazarlos sin motivo.

### Interoperar incrementalmente

Una aplicación de veinte años no debería requerir reescritura para beneficiarse de un nuevo componente Rust.

### Preferir Rust seguro

`unsafe` sigue siendo una parte legítima de Rust, pero el proyecto debe maximizar el área donde las garantías normales de seguridad de Rust sigan teniendo significado.

### Hacer explícitas las fronteras

El ownership de Rust y el garbage collection del CLR son sistemas diferentes. Las fronteras semánticas difíciles deben modelarse deliberadamente, no ocultarse detrás de sintaxis cómoda.

### Contribuir upstream cuando sea práctico

Un ecosistema sostenible es preferible a forks permanentes.

### Evidencia antes que afirmaciones

La corrección del compilador debe provenir de pruebas ejecutables, validación diferencial, evidencia de conformidad y aplicaciones reales, no de diagramas arquitectónicos ni porcentajes optimistas de compatibilidad.

### La compatibilidad es un contrato

Si FerrumWeave afirma que un comportamiento .NET funciona, ese comportamiento debe protegerse mediante pruebas repetibles.

---

# Open source desde el principio

FerrumWeave está pensado para desarrollarse abiertamente.

FerrumWeave tiene licencia dual, a elección del usuario, bajo:

- [MIT License](LICENSE-MIT); o
- [Apache License, Version 2.0](LICENSE-APACHE).

Este modelo permisivo facilita el uso amplio en entornos open source y comerciales y encaja naturalmente con buena parte del ecosistema Rust.

El software producido usando FerrumWeave debe permanecer bajo la licencia elegida por sus propios autores.

Usar el compilador no debe imponer la licencia de FerrumWeave al programa compilado.

---

# Visión de gobierno

FerrumWeave comienza como un proyecto independiente.

Si llega a ser suficientemente útil para desarrollar un ecosistema real de múltiples compañías y comunidades, su gobierno debe poder independizarse de su creador original.

Un hogar neutral a largo plazo —potencialmente bajo una organización como Linux Foundation u otra fundación open source apropiada— se consideraría un éxito, no una pérdida de propiedad.

Por ello el proyecto debe construirse desde el principio con:

- decisiones técnicas transparentes;
- procedencia limpia de propiedad intelectual;
- trazabilidad de contribuidores;
- activos transferibles del proyecto;
- gobierno abierto conforme crezca la comunidad;
- ninguna dependencia innecesaria de una sola organización comercial.

Actualmente no existe afiliación con ninguna fundación.

---

# Relación con Rust y .NET

FerrumWeave es un proyecto experimental independiente.

Actualmente no está afiliado, patrocinado ni respaldado por Microsoft, .NET Foundation, Rust Foundation o Rust Project.

"Rust" y ".NET" se utilizan para describir con precisión las tecnologías con las que FerrumWeave pretende interoperar.

---

# Estado

**Pre-alpha / descubrimiento arquitectónico.**

El repositorio parte intencionalmente del problema, los principios, los contratos objetivo y las fronteras de ingeniería antes de afirmar una implementación completa del lenguaje.

El primer objetivo no es amplitud de features.

Es un vertical slice confiable:

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

Y a partir de ahí, un contrato a la vez.

---

# La idea en una frase

> **FerrumWeave busca convertir a Rust en un lenguaje .NET de primera clase para que las organizaciones puedan introducir el modelo de seguridad de Rust en componentes nuevos y críticos sin abandonar el software, las librerías, los lenguajes ni el conocimiento operativo que ya poseen.**

---

## Un Hello World muy viejo. Un compilador muy moderno.

Una aplicación Visual Basic escrita hace décadas debería poder algún día llamar código escrito hoy en Rust seguro.

No mediante una frontera de servicios.

No mediante una reescritura.

No porque uno de los lenguajes finja ser el otro.

Porque ambos pueden hablar el lenguaje que el CLR fue diseñado para ofrecer entre lenguajes.

Ése es el tejido.