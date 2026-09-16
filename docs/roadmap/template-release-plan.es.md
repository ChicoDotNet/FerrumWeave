<!--
translation-of: docs/roadmap/template-release-plan.md
locale: es
source-revision: 4ed0f1c8df9606bdd7b1135f186ef4bdd710c77a
-->

# Plan de releases de templates de FerrumWeave

[English](template-release-plan.md) · [Deutsch](template-release-plan.de.md) · **Español** · [Français](template-release-plan.fr.md) · [Italiano](template-release-plan.it.md) · [Português (Brasil)](template-release-plan.pt-BR.md) · [Русский](template-release-plan.ru.md) · [简体中文](template-release-plan.zh-Hans.md) · [日本語](template-release-plan.ja.md)

El roadmap de experiencia de desarrollo de FerrumWeave trata a Rust como un **lenguaje de proyecto .NET**, no como un tipo de proyecto independiente.

La UX principal de templates es:

```console
dotnet new <template> -lang Rust
```

Por ejemplo:

```console
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

La forma anterior `dotnet new rust` no es la experiencia objetivo. Una aplicación console sigue siendo un proyecto `console`; FerrumWeave agrega Rust como otra opción de lenguaje del template engine de .NET.

## Qué significa "soportado"

Un template no se considera soportado sólo porque pueda generar archivos.

Para aparecer como soportado en un prerelease de FerrumWeave:

1. `dotnet new <template> -lang Rust` debe crear el `.rsproj` y layout Rust esperados desde un paquete FerrumWeave instalado.
2. El proyecto generado debe restaurar y compilar mediante el workflow normal del SDK de .NET sin requerir checkout del repositorio FerrumWeave.
3. Debe completar su workflow significativo por defecto: `dotnet run` para aplicaciones ejecutables y `dotnet test` para proyectos de pruebas.
4. El comportamiento debe fluir causalmente por `rustc -> FerrumWeave CodegenBackend -> CIL/metadata -> CoreCLR`; emisores legacy por patrones de fuente o sustitutos generados en C#/VB/F# no satisfacen el contrato.
5. Los templates multiplataforma deben certificarse en Windows y Linux. WinForms y WPF se certifican en entornos Windows soportados y no implican ejecución en Linux.
6. Las limitaciones conocidas deben documentarse explícitamente.

Samples y fixtures de conformidad pueden demostrar contratos de prerelease, pero no satisfacen el gate de proyecto real de 1.0.

## Secuencia de versiones

| Release | Etapa | Templates introducidos como soportados |
| --- | --- | --- |
| `0.1` | Alpha | `console`, `classlib`, `xunit`, `nunit`, `mstest`, `web`, `webapi` |
| `0.2` | Alpha | `mvc`, `winforms` |
| `0.3` | Beta | `worker` |
| `0.4` | Beta | `wpf` |
| `0.5` | Beta | `grpc` |
| `0.6` | Beta | `blazor` |
| `1.0` | Stable | Todas las familias comprometidas tienen contratos certificados y al menos un proyecto real exitoso |

Los números pueden incluir sufijos normales como `0.1.0-alpha.1` o `0.5.0-beta.1`; esta tabla define fronteras de capacidad, no la cantidad exacta de prereleases.

## 0.1 alpha — experiencia base como lenguaje .NET

Contratos requeridos:

```console
dotnet new console  -lang Rust
dotnet new classlib -lang Rust
dotnet new xunit    -lang Rust
dotnet new nunit    -lang Rust
dotnet new mstest   -lang Rust
dotnet new web      -lang Rust
dotnet new webapi   -lang Rust
```

El release debe demostrar al menos:

- entry point ejecutable y salida de consola;
- class library administrada consumible desde otro proyecto .NET;
- descubrimiento y ejecución mediante xUnit, NUnit y MSTest;
- host ASP.NET Core iniciado con `dotnet run`;
- request/response HTTP mínimo para `web` y `webapi`;
- restore, build, run/test, `ProjectReference` y NuGet normales para cada familia;
- instalación y uso desde un directorio externo limpio sin checkout de FerrumWeave.

## 0.2 alpha — frameworks de aplicación

Agrega:

```console
dotnet new mvc      -lang Rust
dotnet new winforms -lang Rust
```

`mvc` debe probar una ruta MVC real. `winforms` debe probar un ciclo de vida Windows Forms real y la interacción soportada de eventos/delegates en Windows.

## 0.3 beta — servicios en segundo plano

```console
dotnet new worker -lang Rust
```

Debe ejercitar .NET Generic Host y un ciclo real de background service, no una console app renombrada.

## 0.4 beta — WPF

```console
dotnet new wpf -lang Rust
```

WPF es una frontera propia porque XAML, WindowsDesktop MSBuild targets, tipos/código generado, startup y modelo de eventos agregan requisitos específicos.

## 0.5 beta — gRPC

```console
dotnet new grpc -lang Rust
```

Debe cubrir el build generado de protobuf/gRPC y al menos un round trip real request/response.

## 0.6 beta — Blazor

Agrega la superficie Blazor soportada por la versión del SDK .NET objetivo. El short name exacto debe seguir al SDK contemporáneo y no congelar comandos obsoletos.

El contrato debe probar un componente realmente renderizado y la integración Razor/Blazor requerida.

## Gate estable 1.0 — evidencia de proyectos reales

FerrumWeave no llega a 1.0 solamente porque todas las suites de conformidad estén verdes.

Antes de `1.0.0`, **cada familia de templates comprometida debe tener al menos un proyecto real usando FerrumWeave exitosamente**.

Un proyecto real:

- no es fixture de compiler, test de conformidad, smoke test de template ni sample sólo documental;
- tiene un propósito real de aplicación o librería;
- usa la familia de template correspondiente en su estructura normal;
- puede restaurar/compilar y, cuando corresponda, ejecutar/probarse de forma reproducible;
- ejercita el template de forma representativa;
- documenta las limitaciones específicas de FerrumWeave en lugar de ocultarlas.

Puede ser open source o evidenciado privadamente, pero debe existir evidencia reproducible suficiente para justificar la afirmación de compatibilidad.

La pregunta estable no es sólo:

> ¿Puede FerrumWeave generar y certificar cada familia prometida?

También es:

> ¿Ha sobrevivido cada familia prometida al contacto con al menos una aplicación real?

Sólo cuando ambas respuestas sean sí, FerrumWeave cruza el gate `1.0.0`.

## Regla de compatibilidad

Instalar FerrumWeave no debe cambiar ni romper variantes de lenguaje existentes. R10 debe falsificar regresiones explícitamente:

```console
dotnet new console
dotnet new console -lang VB
dotnet new console -lang "F#"
dotnet new console -lang Rust
```

FerrumWeave extiende el ecosistema de lenguajes .NET; no debe secuestrarlo.
