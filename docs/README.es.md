<!--
translation-of: docs/README.md
locale: es
source-revision: 22aef36588bd35a433ff9564ab947703afd39c4f
-->

# Documentación de FerrumWeave

[English](README.md) · [Deutsch](README.de.md) · **Español** · [Français](README.fr.md) · [Italiano](README.it.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [简体中文](README.zh-Hans.md) · [日本語](README.ja.md)

[← README del proyecto](../README.es.md)

La documentación de FerrumWeave se organiza alrededor del camino que intenta seguir el lector. Los documentos en inglés sin sufijo de locale son canónicos; cuando existe una versión localizada, la documentación debe mantener al lector en el mismo idioma siempre que sea práctico.

> La documentación se mantiene en varios idiomas porque la interoperabilidad también trata de personas.

## Comienza aquí

- [Primeros pasos](getting-started.es.md) — experiencia objetivo de instalación y primer proyecto para el milestone activo R10 / 0.1-alpha.
- [Plan de releases de templates](roadmap/template-release-plan.es.md) — familias estándar de templates .NET que FerrumWeave pretende soportar y qué significa “soportado”.
- [README del proyecto](../README.es.md) — visión, dirección arquitectónica, estado actual y entrada a la comunidad.
- [Sitio del proyecto](https://chicodotnet.github.io/FerrumWeave/es/) — panorama público y experiencia objetivo de desarrollo.

## Arquitectura

Las siguientes piezas todavía usan su fuente canónica en inglés:

- [Repository layout](architecture/repository-layout.md) — límites de ownership, estructura del repositorio y ubicación del trabajo de compiler/SDK/sitio.
- [ADR 0004 — Rust as a .NET template language](architecture/adr/0004-r10-rust-as-dotnet-template-language.md) — por qué el contrato público es `dotnet new <template> -lang Rust`.
- [Architecture Decision Records](architecture/adr/) — decisiones históricas y su justificación.

## Capacidad y compatibilidad

- [Capability roadmap](roadmap/README.md) — evidencia por milestone desde R00 y la frontera activa R10. *(Inglés)*
- [Compatibility](compatibility/README.md) — afirmaciones de compatibilidad y límites de evidencia. *(Inglés)*
- [CTS scalar mappings](compatibility/r04-cts-scalar-mappings.md) — superficie documentada Rust ↔ CLR. *(Inglés)*
- [Coverage policy](quality/coverage-policy.md) — expectativas de cobertura funcional y de código. *(Inglés)*

## Comunidad y políticas del proyecto

- [Contributing](../CONTRIBUTING.md) — flujo de contribución y expectativas de evidencia. *(Inglés)*
- [Governance](../GOVERNANCE.md) — toma de decisiones y evolución de stewardship. *(Inglés)*
- [Support](../SUPPORT.md) — dónde hacer preguntas y reportar problemas ordinarios. *(Inglés)*
- [Security](../SECURITY.md) — guía para reportar vulnerabilidades de forma privada. *(Inglés)*
- [Code of Conduct](../CODE_OF_CONDUCT.md) — estándares de colaboración. *(Inglés)*
- [DCO](../DCO.md) — certificación y sign-off de contribuciones. *(Inglés)*

## Internacionalización

- [Internacionalización de la documentación](i18n.md) — convención de locales, provenance, reglas semánticas y política de obsolescencia. *(Inglés por ahora)*

Locales públicos actuales:

`en` · `de` · `es` · `fr` · `it` · `pt-BR` · `ru` · `zh-Hans` · `ja`

No todos los documentos especializados o históricos están traducidos todavía. Cuando no exista una versión localizada, la fuente canónica inglesa es el fallback.

## Modelo de navegación

```text
README del proyecto
  → Inicio de documentación
    → Sección / documento
```

Los documentos localizados deben regresar a este home en el mismo idioma. Un incremento posterior de grafo documental generará y certificará “Qué enlaza aquí” en vez de mantener esos backlinks manualmente.
