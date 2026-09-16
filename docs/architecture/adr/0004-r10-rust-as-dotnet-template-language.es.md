<!--
doc-id: architecture.adr.0004-rust-as-dotnet-template-language
locale: es
translation-of: docs/architecture/adr/0004-r10-rust-as-dotnet-template-language.md
source-revision: 0059e447c7b43940ac034f423b863d598f24b309
-->

<!-- ferrumweave-nav:start -->
[← Documentación](../../README.es.md) · [README del proyecto](../../../README.es.md)
<!-- ferrumweave-nav:end -->

# ADR 0004 — Registrar Rust como lenguaje de templates .NET

[English](0004-r10-rust-as-dotnet-template-language.md) · [Deutsch](0004-r10-rust-as-dotnet-template-language.de.md) · **Español** · [Français](0004-r10-rust-as-dotnet-template-language.fr.md) · [Italiano](0004-r10-rust-as-dotnet-template-language.it.md) · [Português (Brasil)](0004-r10-rust-as-dotnet-template-language.pt-BR.md) · [Русский](0004-r10-rust-as-dotnet-template-language.ru.md) · [简体中文](0004-r10-rust-as-dotnet-template-language.zh-Hans.md) · [日本語](0004-r10-rust-as-dotnet-template-language.ja.md)

- Estado: Aceptado para R10 y el roadmap de prereleases
- Fecha: 2026-09-15
- Milestone: R10 — Developer experience / 0.1 alpha

## Contexto

El propósito de FerrumWeave es convertir Rust en un lenguaje de primera clase dentro del ecosistema .NET. El prototipo original de R10 exponía un comando de template dedicado:

```console
dotnet new rust
```

Ese comando es técnicamente cómodo, pero modela Rust como si fuera un tipo de proyecto. En el modelo de templates .NET, `console`, `classlib`, `web`, proyectos de test, proyectos desktop y otras familias de workload son templates de proyecto; el lenguaje fuente es una dimensión de selección distinta.

Un developer .NET ya espera que familias de proyecto equivalentes se distingan por lenguaje y no por nombres de template desconectados entre sí. FerrumWeave debe conservar ese modelo mental.

## Decisión

La UX principal de templates de FerrumWeave es:

```console
dotnet new <template> -lang Rust
```

Por tanto, el contrato canónico de primer uso es:

```console
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

`dotnet new rust` no es el contrato público objetivo del producto.

Los templates FerrumWeave deben declarar Rust como su lenguaje y participar en las familias normales de templates .NET donde el template engine permita una integración limpia. La implementación debe validarse mediante contratos ejecutables de resolución de templates, no suponiendo que basta con imitar metadata de templates incorporados.

Instalar FerrumWeave no debe romper ni alterar el comportamiento normal de las variantes existentes de C#, F# o Visual Basic.

## Secuencia de releases

El soporte de templates se introduce en estos releases de capacidad:

- `0.1-alpha`: `console`, `classlib`, `xunit`, `nunit`, `mstest`, `web`, `webapi`;
- `0.2-alpha`: `mvc`, `winforms`;
- `0.3-beta`: `worker`;
- `0.4-beta`: `wpf`;
- `0.5-beta`: `grpc`;
- `0.6-beta`: `blazor`.

Los contratos detallados de soporte y el gate de release estable se mantienen en [`docs/roadmap/template-release-plan.es.md`](../../roadmap/template-release-plan.es.md).

## Soportado significa ejecutable

Que un directorio se genere correctamente no es evidencia suficiente de que un template esté soportado.

Para una afirmación de compatibilidad prerelease, el template debe como mínimo:

- generarse mediante la distribución instalada de FerrumWeave;
- restaurar y compilar sin un checkout del código fuente de FerrumWeave;
- ejecutar el workflow significativo por defecto (`dotnet run` o `dotnet test`, según corresponda);
- alcanzar la ruta real de producto `rustc -> CodegenBackend -> CIL/metadata -> CoreCLR` de FerrumWeave;
- pasar los contratos CI/plataforma pertinentes;
- documentar sus limitaciones con honestidad.

Los templates específicos de framework deben demostrar el comportamiento del framework que los distingue de una aplicación console genérica.

## Gate de 1.0

FerrumWeave no se vuelve estable únicamente porque pasen tests sintéticos de conformidad.

Antes de `1.0.0`, cada familia de template de la secuencia prerelease comprometida debe contar con al menos un proyecto real que use exitosamente ese template FerrumWeave.

Un proyecto real no es un fixture del compiler, un smoke test de template ni un sample de documentación. Debe tener un propósito genuino distinto de testing y evidencia reproducible adecuada para esa familia de proyecto.

## Consecuencias

Consecuencias positivas:

- FerrumWeave se comporta como una extensión de .NET y no como un ecosistema CLI paralelo;
- los comandos de templates siguen siendo naturales para developers de C#, F# y Visual Basic;
- el roadmap del producto se vuelve medible mediante contratos por familia de proyecto;
- pueden agregarse futuros templates sin inventar taxonomías de comandos específicas de FerrumWeave;
- el gate de 1.0 vincula las afirmaciones de compatibilidad con evidencia del mundo real.

Costos y restricciones:

- deben revisarse la metadata y los tests del prototipo R10 que asumen `dotnet new rust`;
- la resolución de templates debe verificarse contra el template engine real de .NET;
- los templates específicos de framework pueden requerir nuevas capacidades del compiler/runtime antes de poder declararse honestamente como soportados;
- los templates desktop y específicos de framework mantienen sus propias restricciones de plataforma/build.

## Validación

R10 debe agregar falsificadores ejecutables que demuestren al menos que:

```console
dotnet new console -lang Rust
dotnet new console -lang VB
dotnet new console -lang "F#"
```

coexisten después de instalar FerrumWeave, y que el proyecto Rust generado usa causalmente el backend real del producto FerrumWeave, no un emitter legacy ni una sustitución de lenguaje fuente.

<!-- ferrumweave-backlinks:start -->
## Qué enlaza aquí

- [Documentación de FerrumWeave](../../README.es.md)
<!-- ferrumweave-backlinks:end -->
