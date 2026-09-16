<!--
translation-of: docs/roadmap/template-release-plan.md
locale: pt-BR
source-revision: 4ed0f1c8df9606bdd7b1135f186ef4bdd710c77a
-->

# Plano de releases dos templates FerrumWeave

[English](template-release-plan.md) · [Deutsch](template-release-plan.de.md) · [Español](template-release-plan.es.md) · [Français](template-release-plan.fr.md) · [Italiano](template-release-plan.it.md) · **Português (Brasil)** · [Русский](template-release-plan.ru.md) · [简体中文](template-release-plan.zh-Hans.md) · [日本語](template-release-plan.ja.md)

O roadmap de experiência de desenvolvimento do FerrumWeave trata Rust como uma **linguagem de projeto .NET**, não como um tipo de projeto separado.

A UX principal dos templates é:

```console
dotnet new <template> -lang Rust
```

Por exemplo:

```console
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

A forma anterior `dotnet new rust` não é a experiência-alvo. Uma aplicação console continua sendo um projeto `console`; FerrumWeave adiciona Rust como outra escolha de linguagem no template engine do .NET.

## O que significa "suportado"

Um template não é considerado suportado apenas porque consegue gerar arquivos.

Para ser anunciado como suportado em um prerelease FerrumWeave:

1. `dotnet new <template> -lang Rust` deve criar o `.rsproj` e o layout Rust esperados a partir de um pacote FerrumWeave instalado.
2. O projeto gerado deve fazer restore e build pelo workflow normal do .NET SDK sem exigir checkout do repositório FerrumWeave.
3. O workflow significativo padrão deve funcionar: `dotnet run` para aplicações executáveis e `dotnet test` para projetos de teste.
4. O comportamento deve passar causalmente por `rustc -> FerrumWeave CodegenBackend -> CIL/metadata -> CoreCLR`; emitters legacy baseados em padrões de source ou substitutos gerados em C#/VB/F# não satisfazem o contrato.
5. Templates multiplataforma devem ser certificados em Windows e Linux. WinForms e WPF são certificados em ambientes Windows suportados e não implicam execução no Linux.
6. Limitações conhecidas devem ser documentadas explicitamente.

Samples e fixtures de conformidade podem provar um contrato de prerelease, mas não atendem ao gate de projeto real de 1.0.

## Sequência de versões

| Release | Estágio | Templates introduzidos como suportados |
| --- | --- | --- |
| `0.1` | Alpha | `console`, `classlib`, `xunit`, `nunit`, `mstest`, `web`, `webapi` |
| `0.2` | Alpha | `mvc`, `winforms` |
| `0.3` | Beta | `worker` |
| `0.4` | Beta | `wpf` |
| `0.5` | Beta | `grpc` |
| `0.6` | Beta | `blazor` |
| `1.0` | Stable | Todas as famílias comprometidas têm contratos certificados e pelo menos um projeto real bem-sucedido |

Sufixos como `0.1.0-alpha.1` ou `0.5.0-beta.1` são normais; a tabela define fronteiras de capacidade, não o número exato de prereleases necessários.

## 0.1 alpha — experiência base como linguagem .NET

```console
dotnet new console  -lang Rust
dotnet new classlib -lang Rust
dotnet new xunit    -lang Rust
dotnet new nunit    -lang Rust
dotnet new mstest   -lang Rust
dotnet new web      -lang Rust
dotnet new webapi   -lang Rust
```

O release deve provar pelo menos: entry point executável e saída de console; class library gerenciada consumível por outro projeto .NET; descoberta e execução via xUnit, NUnit e MSTest; host ASP.NET Core iniciado por `dotnet run`; caminho HTTP request/response mínimo para `web` e `webapi`; restore/build/run/test, `ProjectReference` e NuGet normais; instalação e uso a partir de um diretório externo limpo.

## 0.2 alpha — frameworks de aplicação

```console
dotnet new mvc      -lang Rust
dotnet new winforms -lang Rust
```

`mvc` deve provar uma rota MVC real. `winforms` deve provar um lifecycle real do Windows Forms e interação suportada de eventos/delegates no Windows.

## 0.3 beta — serviços em background

```console
dotnet new worker -lang Rust
```

O template Worker deve exercitar o .NET Generic Host e um lifecycle real de background service, não apenas uma console app renomeada.

## 0.4 beta — WPF

```console
dotnet new wpf -lang Rust
```

WPF é uma fronteira própria porque XAML, WindowsDesktop MSBuild targets, tipos/code-behind gerados, startup e modelo de eventos introduzem requisitos específicos.

## 0.5 beta — gRPC

```console
dotnet new grpc -lang Rust
```

O contrato deve cobrir o caminho de build protobuf/gRPC gerado e pelo menos um round trip real request/response.

## 0.6 beta — Blazor

Adiciona a superfície de templates Blazor suportada pela versão-alvo do .NET SDK. O short name exato deve seguir o SDK contemporâneo em vez de congelar um comando obsoleto. O contrato deve provar um componente realmente renderizado e a integração Razor/Blazor necessária.

## Gate estável 1.0 — evidência de projetos reais

FerrumWeave não chega a 1.0 apenas porque todas as suites de conformidade estão verdes.

Antes de `1.0.0`, **cada família de template comprometida deve ter pelo menos um projeto real usando FerrumWeave com sucesso**.

Um projeto real:

- não é fixture do compiler, teste de conformidade, smoke test de template ou sample apenas documental;
- possui uma finalidade real de aplicação ou biblioteca;
- usa a família Rust correspondente em sua estrutura normal;
- consegue restore/build e, quando aplicável, run/test de forma reproduzível;
- exercita o template de maneira representativa;
- documenta limitações específicas do FerrumWeave.

Pode ser open source ou evidenciado privadamente, mas deve haver evidência reproduzível suficiente para justificar a afirmação de compatibilidade.

A pergunta para a stable release é tanto:

> FerrumWeave consegue scaffoldar e certificar cada família prometida?

quanto:

> Cada família prometida sobreviveu ao contato com pelo menos uma aplicação real?

Somente quando ambas as respostas forem sim FerrumWeave cruza o gate `1.0.0`.

## Regra de compatibilidade

```console
dotnet new console
dotnet new console -lang VB
dotnet new console -lang "F#"
dotnet new console -lang Rust
```

FerrumWeave estende o ecossistema de linguagens .NET; não deve sequestrar nem quebrar variantes existentes.
