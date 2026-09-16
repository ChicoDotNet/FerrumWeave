<!--
translation-of: docs/getting-started.md
locale: pt-BR
source-revision: e4bcd0d4b4c4b73232823896144adf098b30758b
-->

# FerrumWeave 0.1 alpha — primeiros passos

[English](getting-started.md) · [Deutsch](getting-started.de.md) · [Español](getting-started.es.md) · [Français](getting-started.fr.md) · [Italiano](getting-started.it.md) · **Português (Brasil)** · [Русский](getting-started.ru.md) · [简体中文](getting-started.zh-Hans.md) · [日本語](getting-started.ja.md)

> **Contrato de trabalho R10:** este documento descreve a experiência externa que FerrumWeave 0.1 alpha deve certificar antes do release. Comandos ainda não GREEN em CI são objetivos de produto, não capacidade já liberada.

FerrumWeave estende templates .NET normais com **Rust como opção de linguagem**. A experiência principal é `dotnet new <template> -lang Rust`, não `dotnet new rust`.

Sequência de releases e suporte estão em [`template-release-plan.pt-BR.md`](roadmap/template-release-plan.pt-BR.md). A decisão arquitetural está no [ADR 0004](architecture/adr/0004-r10-rust-as-dotnet-template-language.md).

## Primeiro uso alvo da 0.1-alpha

```console
dotnet new install FerrumWeave.Sdk::0.1.0-alpha.1
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

`FerrumWeave.Sdk` é a identidade atual do package para MSBuild SDK e templates. Não deve haver segundo package não publicado sem ADR posterior.

O projeto Rust deve atingir causalmente:

```text
Rust source
  -> rustc
  -> FerrumWeave CodegenBackend
  -> CIL / CLR metadata
  -> CoreCLR
```

O contrato deve funcionar em diretório externo limpo sem checkout do FerrumWeave.

## Superfície 0.1-alpha

```console
dotnet new console  -lang Rust
dotnet new classlib -lang Rust
dotnet new xunit    -lang Rust
dotnet new nunit    -lang Rust
dotnet new mstest   -lang Rust
dotnet new web      -lang Rust
dotnet new webapi   -lang Rust
```

Um template só é suportado quando restore/build e o workflow significativo passam pelo backend real FerrumWeave.

## FerrumWeave MSBuild SDK

```xml
<Project Sdk="FerrumWeave.Sdk/0.1.0-alpha.1"><PropertyGroup><TargetFramework>net10.0</TargetFramework></PropertyGroup></Project>
```

```console
dotnet restore
```

O package contém `Sdk.props`, `Sdk.targets` e templates; usuários não devem copiá-los manualmente.

## Testar prerelease local

```xml
<?xml version="1.0" encoding="utf-8"?>
<configuration><packageSources><clear /><add key="ferrumweave-local" value="../artifacts/packages" /></packageSources></configuration>
```

Os mesmos comandos restore/build/test/run do package publicado devem funcionar sem conhecimento oculto de maintainer.

## Invariante de compatibilidade

```console
dotnet new console
dotnet new console -lang VB
dotnet new console -lang "F#"
dotnet new console -lang Rust
```

FerrumWeave adiciona Rust preservando o comportamento esperado do .NET SDK.
