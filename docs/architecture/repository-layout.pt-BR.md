<!--
doc-id: architecture.repository-layout
locale: pt-BR
translation-of: docs/architecture/repository-layout.md
source-revision: 2baacfc0c69ca60057c479a955c57dad2f808817
-->

<!-- ferrumweave-nav:start -->
[← Documentação](../README.pt-BR.md) · [README do projeto](../../README.pt-BR.md)
<!-- ferrumweave-nav:end -->

# Estrutura do repositório

[English](repository-layout.md) · [Deutsch](repository-layout.de.md) · [Español](repository-layout.es.md) · [Français](repository-layout.fr.md) · [Italiano](repository-layout.it.md) · **Português (Brasil)** · [Русский](repository-layout.ru.md) · [简体中文](repository-layout.zh-Hans.md) · [日本語](repository-layout.ja.md)

FerrumWeave é intencionalmente organizado como um **monorepo orientado a capacidades**. O repositório deve parecer familiar tanto para contributors vindos de Rust quanto para contributors vindos de .NET, sem dividir permanentemente o produto em um “lado Rust” e um “lado .NET”.

Este documento descreve a estrutura alvo. Diretórios só são criados quando um incremento realmente precisa deles; o projeto não mantém pastas vazias por meio de arquivos `.gitkeep`.

## Estrutura alvo

```text
FerrumWeave/
├── .cargo/
├── .github/
│   ├── workflows/
│   ├── ISSUE_TEMPLATE/
│   └── PULL_REQUEST_TEMPLATE.md
├── assets/
│   └── brand/
│       ├── hero/
│       ├── logos/
│       ├── mascot/
│       ├── icons/
│       └── merch/
├── compiler/
│   ├── codegen-backend/
│   ├── cil/
│   └── driver/
├── projection/
│   ├── metadata/
│   ├── types/
│   └── support/
├── sdk/
│   ├── FerrumWeave.Sdk/
│   ├── tasks/
│   └── templates/rust/
├── tooling/
│   ├── analyzer/
│   ├── debugger/
│   ├── vscode/
│   └── visualstudio/
├── tools/
│   ├── docgraph/
│   └── quality/
├── tests/
│   ├── ui/
│   ├── codegen/
│   ├── conformance/
│   ├── interop/
│   │   ├── rust-csharp/
│   │   ├── rust-vb/
│   │   └── rust-fsharp/
│   ├── sdk/
│   ├── e2e/
│   └── fixtures/
├── samples/
│   ├── hello-world/
│   ├── consume-dotnet/
│   ├── consumed-by-dotnet/
│   └── mixed-solution/
├── docs/
│   ├── README.md
│   ├── i18n.md
│   ├── architecture/
│   │   └── adr/
│   ├── compatibility/
│   ├── design/
│   ├── roadmap/
│   ├── site/
│   └── upstream/
├── eng/
│   ├── ci/
│   ├── packaging/
│   └── scripts/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── FerrumWeave.slnx
├── global.json
├── Directory.Build.props
├── Directory.Build.targets
├── README.md
├── CONTRIBUTING.md
├── GOVERNANCE.md
├── SECURITY.md
├── CODE_OF_CONDUCT.md
├── NOTICE
├── LICENSE-MIT
└── LICENSE-APACHE
```

## Regras

### Organizar por capacidade, não por linguagem de implementação

As pastas de produto no nível superior descrevem responsabilidades: `compiler`, `projection`, `sdk` e `tooling`. Evite uma raiz como `src/rust` ao lado de `src/dotnet`, pois isso criaria justamente a fronteira arquitetural que FerrumWeave existe para dissolver.

Dentro de uma capacidade, use as convenções idiomáticas do ecossistema que a implementa. Crates Rust usam `Cargo.toml` + `src/`; componentes .NET usam projetos SDK-style e nomenclatura .NET normal.

Utilitários de manutenção do repositório que não são tooling de produto vivem em `tools/`. Por exemplo, `tools/docgraph` mantém o grafo documental; ele não faz parte do compiler nem da toolchain FerrumWeave consumida pelo developer.

### Manter sob responsabilidade do FerrumWeave a fronteira do backend rustc

`compiler/codegen-backend/` é a fronteira de produto carregada por `rustc`. Ela contém a implementação FerrumWeave de `CodegenBackend` e direciona o MIR de rustc para lowering, emissão CIL, metadata e infraestrutura de projection do FerrumWeave.

O workspace estável permanece independente das APIs privadas do compiler. Requisitos nightly/rustc-dev pertencem à lane isolada do backend e não devem vazar para crates comuns.

### Manter `projection/` independente

A projection de metadata CLR é um contrato compartilhado, não um detalhe interno do codegen. Compiler, análise de código, Project References, integração NuGet, IntelliSense e eventualmente debugging podem depender do mesmo modelo de símbolos visíveis pelo .NET.

A direção das dependências deve continuar explícita. Tooling pode consumir contratos de projection; projection não deve depender de preocupações específicas de IDE.

### Tests são contratos executáveis de interoperabilidade

A árvore de tests usa deliberadamente vocabulário dos dois ecossistemas. `tests/ui` e `tests/codegen` devem ser familiares para contributors do compilador Rust; `tests/interop`, `tests/sdk`, `tests/e2e` e `tests/fixtures` devem ser naturais para contributors .NET.

Cada fixture de interop deve provar comportamento nos dois sentidos sempre que for prático. Por exemplo, `tests/interop/rust-csharp/` deverá conter contratos onde Rust consome uma superfície C# e onde C# consome uma superfície CLR produzida a partir de Rust.

### Samples vendem a arquitetura com evidência

Os samples canônicos avançam verticalmente:

1. `hello-world` — Rust → CIL → CLR.
2. `consume-dotnet` — Rust consome APIs .NET ou um projeto .NET.
3. `consumed-by-dotnet` — outra linguagem .NET consome uma assembly produzida a partir de Rust.
4. `mixed-solution` — `.rsproj`, `.csproj`, `.fsproj` e `.vbproj` coexistem em uma solução.

Um sample só é adicionado quando o comportamento demonstrado é executável.

### Manter divergência de upstream de produto visível e temporária

FerrumWeave deve integrar antes de reinventar quando uma dependência realmente pertence ao product path. Divergência local de `rustc`, `rust-analyzer`, .NET SDK ou outro upstream de produto deve ser registrada em `docs/upstream/` com revisão upstream, necessidade local, link de issue/PR e condição de saída.

Ciclo preferido para upstreams reais de produto:

```text
consume upstream
      ↓
patch only when required
      ↓
submit upstream
      ↓
remove local divergence
```

`rustc_codegen_clr` é diferente: é um **characterization oracle / differential oracle** fixado, não uma dependência de runtime, SDK ou backend de produto. Seu ciclo é orientado à evidência:

```text
pin oracle revision
      ↓
characterize observable behavior
      ↓
reproduce the required behavior through FerrumWeave
      ↓
retain only when differential value remains, otherwise remove
```

Proveniência e pins do oracle continuam em `docs/upstream/`, mas FerrumWeave não aplica patches nem consome esse backend como implementação de produto.

### Não pré-criar o futuro

A árvore alvo é um mapa, não uma ordem para criar diretórios vazios. Novas pastas chegam com o primeiro contrato, implementação, test ou documento real que pertence a elas.

Isso mantém o histórico do repositório honesto: a estrutura segue evidência executável, e não arquitetura especulativa.

## Dois pontos de entrada familiares

Conforme a implementação cresce, um contributor deve conseguir abordar o repositório naturalmente a partir de qualquer um dos dois ecossistemas:

```bash
cargo test --workspace
```

ou:

```bash
dotnet build FerrumWeave.slnx
```

Os dois comandos devem convergir para o mesmo produto e os mesmos contratos de interoperabilidade.

<!-- ferrumweave-backlinks:start -->
## O que aponta para cá

- [Documentação do FerrumWeave](../README.pt-BR.md)
<!-- ferrumweave-backlinks:end -->
