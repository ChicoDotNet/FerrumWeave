<!--
translation-of: docs/README.md
locale: pt-BR
source-revision: 22aef36588bd35a433ff9564ab947703afd39c4f
-->

# Documentação do FerrumWeave

[English](README.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Italiano](README.it.md) · **Português (Brasil)** · [Русский](README.ru.md) · [简体中文](README.zh-Hans.md) · [日本語](README.ja.md)

[← README do projeto](../README.pt-BR.md)

A documentação do FerrumWeave é organizada conforme o caminho real do leitor. Documentos em inglês sem sufixo de locale são canônicos; quando existe uma versão localizada, a navegação deve permanecer no mesmo idioma sempre que for prático.

> A documentação é mantida em vários idiomas porque interoperabilidade também é sobre pessoas.

## Comece aqui

- [Primeiros passos](getting-started.pt-BR.md) — experiência alvo de instalação e primeiro projeto para o milestone ativo R10 / 0.1-alpha.
- [Plano de releases dos templates](roadmap/template-release-plan.pt-BR.md) — famílias padrão de templates .NET previstas e o significado de “suportado”.
- [README do projeto](../README.pt-BR.md) — visão, arquitetura, estado atual e entrada para a comunidade.
- [Site do projeto](https://chicodotnet.github.io/FerrumWeave/pt-BR/) — visão pública e developer experience alvo.

## Arquitetura

As páginas seguintes ainda usam a fonte canônica em inglês:

- [Repository layout](architecture/repository-layout.md) — limites de ownership e estrutura do repositório.
- [ADR 0004 — Rust as a .NET template language](architecture/adr/0004-r10-rust-as-dotnet-template-language.md) — por que o contrato público é `dotnet new <template> -lang Rust`.
- [Architecture Decision Records](architecture/adr/) — decisões históricas e sua justificativa.

## Capacidade e compatibilidade

- [Capability roadmap](roadmap/README.md) *(inglês)*
- [Compatibility](compatibility/README.md) *(inglês)*
- [CTS scalar mappings](compatibility/r04-cts-scalar-mappings.md) *(inglês)*
- [Coverage policy](quality/coverage-policy.md) *(inglês)*

## Comunidade e políticas do projeto

- [Contributing](../CONTRIBUTING.md) *(inglês)*
- [Governance](../GOVERNANCE.md) *(inglês)*
- [Support](../SUPPORT.md) *(inglês)*
- [Security](../SECURITY.md) *(inglês)*
- [Code of Conduct](../CODE_OF_CONDUCT.md) *(inglês)*
- [DCO](../DCO.md) *(inglês)*

## Internacionalização

- [Documentation internationalization](i18n.md) — locales, provenance, regras semânticas e política de desatualização. *(inglês por enquanto)*

Locales públicos atuais:

`en` · `de` · `es` · `fr` · `it` · `pt-BR` · `ru` · `zh-Hans` · `ja`

Nem todo documento especializado ou histórico já foi traduzido. Quando não existir uma versão localizada, a fonte canônica em inglês é o fallback.

## Modelo de navegação

```text
README do projeto
  → Início da documentação
    → Seção / documento
```

Documentos localizados devem retornar a este início no mesmo idioma. Um próximo incremento docgraph gerará e certificará automaticamente “O que aponta para cá?”.
