<!--
doc-id: architecture.adr.0004-rust-as-dotnet-template-language
locale: pt-BR
translation-of: docs/architecture/adr/0004-r10-rust-as-dotnet-template-language.md
source-revision: 0059e447c7b43940ac034f423b863d598f24b309
-->

# ADR 0004 — Registrar Rust como linguagem de templates .NET

[English](0004-r10-rust-as-dotnet-template-language.md) · [Deutsch](0004-r10-rust-as-dotnet-template-language.de.md) · [Español](0004-r10-rust-as-dotnet-template-language.es.md) · [Français](0004-r10-rust-as-dotnet-template-language.fr.md) · [Italiano](0004-r10-rust-as-dotnet-template-language.it.md) · **Português (Brasil)** · [Русский](0004-r10-rust-as-dotnet-template-language.ru.md) · [简体中文](0004-r10-rust-as-dotnet-template-language.zh-Hans.md) · [日本語](0004-r10-rust-as-dotnet-template-language.ja.md)

- Status: Aceito para R10 e para o roadmap de prereleases
- Data: 2026-09-15
- Milestone: R10 — Developer experience / 0.1 alpha

## Contexto

O objetivo do FerrumWeave é tornar Rust uma linguagem de primeira classe no ecossistema .NET. O protótipo original de R10 expunha um comando de template dedicado:

```console
dotnet new rust
```

Esse comando é tecnicamente conveniente, mas modela Rust como se fosse um tipo de projeto. No modelo de templates .NET, `console`, `classlib`, `web`, projetos de teste, desktop e outras famílias de workload são templates de projeto; a linguagem fonte é uma dimensão de seleção separada.

Um developer .NET já espera que famílias equivalentes de projeto se diferenciem por linguagem, e não por nomes de template desconectados. FerrumWeave deve preservar esse modelo mental.

## Decisão

A UX principal dos templates FerrumWeave é:

```console
dotnet new <template> -lang Rust
```

O contrato canônico de primeiro uso é, portanto:

```console
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

`dotnet new rust` não é o contrato público alvo do produto.

Os templates FerrumWeave devem declarar Rust como linguagem e participar das famílias normais de templates .NET sempre que a template engine permitir uma integração limpa. A implementação deve ser validada por contratos executáveis de resolução de templates, e não pela suposição de que metadata semelhante à dos templates nativos é suficiente.

Instalar FerrumWeave não deve quebrar nem alterar o comportamento normal das variantes existentes de C#, F# ou Visual Basic.

## Sequência de releases

O suporte a templates é introduzido nestas releases de capacidade:

- `0.1-alpha`: `console`, `classlib`, `xunit`, `nunit`, `mstest`, `web`, `webapi`;
- `0.2-alpha`: `mvc`, `winforms`;
- `0.3-beta`: `worker`;
- `0.4-beta`: `wpf`;
- `0.5-beta`: `grpc`;
- `0.6-beta`: `blazor`.

Os contratos detalhados e o gate de release estável são mantidos em [`docs/roadmap/template-release-plan.pt-BR.md`](../../roadmap/template-release-plan.pt-BR.md).

## Suportado significa executável

Gerar um diretório não é evidência suficiente de que um template é suportado.

Para uma afirmação de compatibilidade prerelease, o template deve no mínimo:

- ser scaffoldado pela distribuição FerrumWeave instalada;
- restaurar e compilar sem um checkout dos fontes de FerrumWeave;
- executar o workflow significativo padrão (`dotnet run` ou `dotnet test`);
- alcançar o product path real `rustc -> CodegenBackend -> CIL/metadata -> CoreCLR` do FerrumWeave;
- passar os contratos relevantes de CI/plataforma;
- documentar honestamente suas limitações.

Templates específicos de framework devem comprovar o comportamento do framework que os diferencia de uma aplicação console genérica.

## Gate 1.0

FerrumWeave não se torna estável apenas porque testes sintéticos de conformidade passam.

Antes de `1.0.0`, cada família de template da sequência prerelease comprometida deve ter pelo menos um projeto real usando com sucesso esse template FerrumWeave.

Um projeto real não é fixture do compiler, smoke test de template nem sample de documentação. Deve ter um propósito genuíno além de teste e evidência reproduzível apropriada para a família de projeto.

## Consequências

Consequências positivas:

- FerrumWeave se comporta como extensão do .NET em vez de ecossistema CLI paralelo;
- comandos de template continuam naturais para developers C#, F# e Visual Basic;
- o roadmap se torna mensurável por contratos de família de projeto;
- futuros templates podem ser adicionados sem inventar taxonomias de comando específicas de FerrumWeave;
- o gate 1.0 liga afirmações de compatibilidade a evidência do mundo real.

Custos e restrições:

- metadata e testes do protótipo R10 que assumem `dotnet new rust` devem ser revisados;
- a resolução de templates deve ser verificada contra a template engine real do .NET;
- templates específicos de framework podem exigir novas capacidades de compiler/runtime antes de serem honestamente declarados suportados;
- templates desktop e framework-specific mantêm seus próprios limites de plataforma/build.

## Validação

R10 deve adicionar falsificadores executáveis comprovando ao menos que:

```console
dotnet new console -lang Rust
dotnet new console -lang VB
dotnet new console -lang "F#"
```

coexistem após a instalação de FerrumWeave e que o projeto Rust gerado usa causalmente o backend real do produto FerrumWeave, e não um emitter legacy nem uma substituição da linguagem fonte.
