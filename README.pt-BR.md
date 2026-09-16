<!--
translation-of: README.md
locale: pt-BR
source-revision: 6fb75d076d59b4e1932b685e441487f6caedb36d
-->

[English](README.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Italiano](README.it.md) · **Português (Brasil)** · [Русский](README.ru.md) · [简体中文](README.zh-Hans.md) · [日本語](README.ja.md)

> A documentação do FerrumWeave é mantida em vários idiomas porque interoperabilidade também é sobre pessoas. O documento em inglês sem sufixo permanece como fonte canônica quando uma tradução ficar temporariamente desatualizada.

<img src="assets/brand/hero/ferrumweave-readme-cover.png" alt="FerrumWeave — levando Rust ao ecossistema .NET" width="100%" />

# FerrumWeave

**Levando Rust ao ecossistema de linguagens .NET.**

**Recursos do projeto:** [Estrutura do repositório](docs/architecture/repository-layout.md) · [Plano de releases dos templates](docs/roadmap/template-release-plan.md) · [Site do projeto](https://chicodotnet.github.io/FerrumWeave/pt-BR/)

FerrumWeave é um esforço open source experimental para tornar Rust uma linguagem de primeira classe na plataforma .NET: compilar código-fonte Rust em assemblies .NET, participar do Common Type System, consumir bibliotecas .NET existentes e interoperar naturalmente com C#, F#, Visual Basic e outras linguagens construídas ao redor do CLR.

No longo prazo, a experiência do desenvolvedor deve parecer natural:

```bash
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

Com código Rust conceitualmente parecido com:

```rust
use dotnet::System::*;
use dotnet::Result;

fn main() -> Result<()> {
    Console::WriteLine("Hello from Rust on .NET")?;
    Ok(())
}
```

E produzindo um assembly .NET real:

```text
HelloFerrum.dll
```

executado pelo runtime .NET.

> **FerrumWeave está no começo dessa jornada.**
>
> Os exemplos deste README descrevem a experiência de desenvolvimento e a direção arquitetural pretendidas. Ainda não são afirmações de funcionalidade liberada.

---

## Por que FerrumWeave?

Uma das ideias mais duradouras do .NET nunca foi apenas o C#.

Foi a ideia de que **linguagens diferentes poderiam se encontrar em um runtime comum**.

Há décadas, desenvolvedores conseguem escrever software em linguagens com sintaxes, filosofias e histórias muito diferentes enquanto compartilham a mesma plataforma:

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
- e muitas outras

Uma biblioteca escrita em uma linguagem frequentemente podia ser consumida por outra porque a fronteira importante não era a linguagem-fonte.

Era a **Common Language Infrastructure**, o **Common Type System**, os metadados dos assemblies e o CLR.

FerrumWeave faz uma pergunta simples:

> **E se Rust pudesse entrar nessa família de linguagens?**

Não apenas chamando uma biblioteca Rust nativa via FFI.

Não apenas hospedando o CLR a partir de um executável Rust.

Não traduzindo Rust para C#.

Mas compilando Rust para o mesmo mundo de CIL, assemblies, metadados, tipos, referências, pacotes, tooling e interoperabilidade de runtime que tornou o .NET multilíngue possível.

---

## Por que Rust?

Rust traz um conjunto diferente de garantias para o desenvolvimento de software.

Seu modelo de ownership, borrow checker, sistema de tipos forte, tratamento explícito de erros e foco em segurança de memória e concorrência permitem detectar categorias inteiras de defeitos antes da produção.

Rust **não** torna impossível que o software falhe.

A lógica pode estar errada. Arquivos podem desaparecer. Redes podem falhar. Bancos de dados podem conter dados ruins. Programas podem entrar em panic. Código `unsafe` existe.

Mas Rust pode mover classes importantes de falhas de:

```text
produção
```

para:

```text
tempo de compilação
```

Essa diferença importa, especialmente em sistemas de negócio de longa vida.

---

## A oportunidade no software empresarial

Existem enormes sistemas .NET que entregam valor há dez, quinze ou vinte e cinco anos.

Eles podem conter:

```text
ERP.vbproj
Accounting.csproj
Reporting.fsproj
LegacyIntegration.vbproj
```

Conversas de modernização começam vezes demais com:

> “Precisamos reescrever tudo.”

FerrumWeave parte de outra ideia:

> **Preserve o que funciona. Fortaleça o que vem depois.**

Imagine adicionar:

```text
RiskEngine.rsproj
```

no mesmo sistema.

A aplicação Visual Basic existente não precisa desaparecer. O modelo de domínio em C# não precisa ser reescrito. O motor de relatórios em F# não precisa de um novo protocolo de integração.

Com o tempo:

```text
ERP.vbproj
     │
     ▼
RiskEngine.rsproj
     │
     ▼
Accounting.csproj
```

poderiam conversar usando o mesmo sistema de tipos .NET.

Uma aplicação Visual Basic de décadas atrás poderia chamar novo código Rust. Rust poderia consumir um assembly de domínio escrito em C#. F# poderia consumir um tipo implementado em Rust.

A unidade de migração passa a ser **o componente**, não a aplicação inteira.

Essa é a visão.

---

# O objetivo

FerrumWeave pretende tornar possível:

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

O caminho de compilação pretendido para Rust é aproximadamente:

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

Rust continua sendo Rust.

O CLR continua sendo o CLR.

FerrumWeave deve conectá-los em vez de reinventá-los sem necessidade.

---

# O que FerrumWeave não é

FerrumWeave **não** pretende ser:

### Uma nova linguagem parecida com Rust

O objetivo é preservar Rust e aproveitar o ecossistema existente do compilador.

```rust
match
traits
lifetimes
ownership
borrowing
async
Result<T, E>
```

devem continuar sendo conceitos Rust, não aproximações recriadas em outro compilador.

### Um wrapper Rust em torno de `dotnet`

Executar Cargo a partir de um target MSBuild pode ser útil, mas isso sozinho não transforma Rust em uma linguagem .NET.

FerrumWeave quer ir mais fundo.

### Um gerador de FFI nativo

Interoperabilidade nativa continua valiosa, mas o objetivo não é:

```text
C#
 ↓
P/Invoke
 ↓
Rust native DLL
```

A meta é:

```text
C#
  ╲
   CLR
  ╱
Rust
```

### Um substituto para .NET

FerrumWeave existe porque o ecossistema .NET é valioso. O objetivo é ampliar as opções de linguagem disponíveis.

### Um substituto para Rust nativo

Sempre haverá excelentes razões para compilar Rust diretamente para código nativo. Um target CLR seria outra opção de deployment e interoperabilidade.

---

# A experiência de desenvolvimento desejada

Um projeto Rust deve, no futuro, se sentir em casa dentro de uma solução .NET:

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

Comandos familiares devem continuar familiares:

```bash
dotnet restore
dotnet build
dotnet run
dotnet test
dotnet publish
dotnet pack
```

Um projeto poderia se parecer com:

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

Cargo e crates.io devem manter seu espaço quando dependências Rust precisarem deles. NuGet e MSBuild devem continuar fazendo o que já fazem bem para dependências .NET.

FerrumWeave deve conectar esses ecossistemas sem fingir que um deles não existe.

---

# Bibliotecas .NET a partir de Rust

Um objetivo central é transformar APIs .NET em participantes naturais do código Rust.

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

A propriedade importante não é a sintaxe exata, mas esta:

> `System.Console`, `System.String`, `System.IO.File` e tipos .NET definidos pelo usuário devem ser entendidos como tipos e membros CLR — não como bibliotecas nativas opacas escondidas atrás de uma camada FFI mantida manualmente.

O mesmo princípio deve se aplicar a pacotes NuGet e ProjectReference.

---

# Bibliotecas Rust a partir de outras linguagens .NET

A interoperabilidade precisa funcionar nos dois sentidos.

Rust deve poder definir tipos públicos voltados ao CLR que outras linguagens .NET possam consumir.

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

Linguagens-fonte diferentes. Um único contrato de runtime.

Esse é o padrão de interoperabilidade que FerrumWeave busca atingir.

---

# Tipos Rust e tipos CLR

Rust e CLR têm modelos de objetos e memória fundamentalmente diferentes. Essa diferença não deve ser escondida.

Rust possui conceitos como:

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

O CLR possui:

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

FerrumWeave não deve enfraquecer nenhum dos modelos apenas para fazê-los parecer iguais. Deve definir mapeamentos baseados em princípios claros.

Alguns podem ser naturais:

```text
System.Int32   ↔ i32
System.Int64   ↔ i64
System.Boolean ↔ bool
System.Double  ↔ f64
```

Outros exigem semântica explícita:

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

Definir essas semânticas corretamente é um dos desafios centrais de engenharia do projeto.

---

# Segurança sem abandonar interoperabilidade

Software empresarial raramente tem o luxo de começar do zero.

Organizações já possuem aplicações, bancos de dados, regras de negócio, APIs, pacotes, frameworks, desenvolvedores e conhecimento operacional.

Uma linguagem mais segura é muito mais fácil de adotar quando sua adoção não exige abandonar tudo ao redor.

Por isso FerrumWeave explora a seguinte proposta:

> **Levar o modelo de segurança de Rust para componentes .NET novos e críticos enquanto preserva a interoperabilidade com investimentos .NET existentes.**

---

# Por que o nome FerrumWeave?

**Ferrum** significa ferro em latim. Rust é a oxidação do ferro.

**Weave** significa entrelaçar fios separados em uma estrutura conectada.

```text
Ferrum
   │
   └── iron → rust

Weave
   │
   └── interconnection → network → ecosystem
```

Juntos:

> **FerrumWeave representa Rust entrelaçado no ecossistema .NET.**

O nome é intencionalmente independente das marcas Rust e .NET.

---

# Construindo sobre trabalho existente

FerrumWeave não pretende começar escrevendo outro compilador Rust.

O ecossistema Rust já oferece infraestrutura extremamente valiosa:

- `rustc` para parsing, type checking, borrow checking, MIR e semântica da linguagem;
- `rust-analyzer` para análise moderna de código Rust e tooling de desenvolvimento.

Também existe prior art importante de compilação Rust para CLR, especialmente o projeto experimental `rustc_codegen_clr`.

.NET já oferece infraestrutura madura para CLR, CTS, metadados de assemblies, MSBuild, NuGet, CLI `dotnet`, projetos SDK-style, debugging e tooling.

A estratégia do FerrumWeave é, portanto:

> **Integrar antes de reinventar.**

Quando possível, melhorias devem ser contribuídas upstream em vez de mantidas para sempre em forks privados.

---

# Arquitetura inicial

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

A estrutura alvo do repositório está documentada em [Repository layout](docs/architecture/repository-layout.md).

Essa arquitetura é intencionalmente provisória. Evidência executável tem prioridade sobre diagramas.

---

# Primeira prova

O primeiro milestone relevante é deliberadamente pequeno:

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

FerrumWeave deve produzir um assembly .NET válido e executar `System.Console.WriteLine` através do CLR.

Esse único resultado validaria `.rsproj`, MSBuild, `dotnet` CLI, `rustc`, Rust→CIL, metadados CLR, interoperabilidade CTS e a BCL .NET.

O projeto crescerá verticalmente a partir de contratos funcionais, em vez de tentar modelar todo o ecossistema .NET antes que algo rode.

---

# Sucesso de longo prazo

FerrumWeave não será considerado bem-sucedido apenas porque Rust consegue emitir “algum CIL”.

A pergunta mais profunda é:

> **Um desenvolvedor .NET consegue tratar Rust como outra escolha séria de linguagem dentro de um sistema .NET existente?**

Um FerrumWeave maduro deve tornar normais cenários como:

```text
C# → Rust
Rust → C#
VB → Rust
Rust → F#
Rust → NuGet package
.NET project → .rsproj
.rsproj → .NET project
```

com as expectativas habituais sobre build, referências, pacotes, tipos, exceções, debugging, testes, tooling e publishing.

---

# Princípios do projeto

### Preservar Rust

Evitar criar um dialeto Rust desnecessário.

### Preservar .NET

Usar CLR, CTS, metadados, MSBuild, NuGet e outros contratos existentes da plataforma.

### Interoperar incrementalmente

Uma aplicação de vinte anos não deveria precisar ser reescrita para se beneficiar de um novo componente Rust.

### Preferir Safe Rust

`unsafe` continua sendo parte legítima de Rust, mas o projeto deve maximizar a área em que as garantias normais de segurança continuam significativas.

### Tornar fronteiras explícitas

Ownership Rust e garbage collection CLR são sistemas diferentes. Fronteiras semânticas difíceis devem ser modeladas deliberadamente.

### Contribuir upstream quando fizer sentido

Um ecossistema sustentável é melhor que forks permanentes.

### Evidência antes de afirmações

A correção do compilador deve vir de testes executáveis, validação diferencial, evidência de conformidade e aplicações reais — não de diagramas otimistas.

### Compatibilidade é um contrato

Todo comportamento .NET declarado deve ser protegido por testes repetíveis.

---

# Open source desde o início

FerrumWeave é desenvolvido abertamente e possui licença dual, à escolha do usuário:

- [MIT License](LICENSE-MIT); ou
- [Apache License, Version 2.0](LICENSE-APACHE).

Usar o compilador não deve impor a licença FerrumWeave ao programa compilado.

---

# Visão de governança

FerrumWeave começa como projeto independente.

Se se tornar útil o bastante para criar um ecossistema real de múltiplas empresas e comunidades, sua governança deve poder se tornar independente do criador original.

Uma casa neutra no longo prazo, possivelmente em uma fundação open source apropriada, seria um sucesso, não perda de propriedade.

O projeto deve ser construído com decisões técnicas transparentes, procedência limpa de propriedade intelectual, rastreabilidade de contribuidores, ativos transferíveis e governança aberta.

Atualmente não existe afiliação com nenhuma fundação.

---

# Relação com Rust e .NET

FerrumWeave é um projeto experimental independente.

Atualmente não é afiliado, patrocinado nem endossado pela Microsoft, .NET Foundation, Rust Foundation ou Rust Project.

“Rust” e “.NET” são usados para descrever corretamente as tecnologias com as quais FerrumWeave pretende interoperar.

---

# Status

**Pré-alpha / descoberta arquitetural.**

O repositório começa intencionalmente pelo problema, princípios, contratos-alvo e fronteiras de engenharia antes de afirmar uma implementação completa da linguagem.

O primeiro foco é um vertical slice confiável:

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

Depois disso, um contrato por vez.

---

# A ideia em uma frase

> **FerrumWeave pretende tornar Rust uma linguagem .NET de primeira classe para que organizações possam introduzir o modelo de segurança de Rust em componentes novos e críticos sem abandonar software, bibliotecas, linguagens e conhecimento operacional que já possuem.**

---

## Um Hello World muito antigo. Um compilador muito moderno.

Uma aplicação Visual Basic escrita décadas atrás deveria um dia conseguir chamar código Safe Rust escrito hoje.

Não através de uma fronteira de serviços.

Não por meio de uma reescrita.

Não porque uma linguagem finja ser a outra.

Mas porque ambas podem falar a linguagem que o CLR foi projetado para fornecer entre linguagens.

Esse é o tecido.