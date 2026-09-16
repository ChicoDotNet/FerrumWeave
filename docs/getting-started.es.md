<!--
translation-of: docs/getting-started.md
locale: es
source-revision: e4bcd0d4b4c4b73232823896144adf098b30758b
-->

# FerrumWeave 0.1 alpha — primeros pasos

[English](getting-started.md) · [Deutsch](getting-started.de.md) · **Español** · [Français](getting-started.fr.md) · [Italiano](getting-started.it.md) · [Português (Brasil)](getting-started.pt-BR.md) · [Русский](getting-started.ru.md) · [简体中文](getting-started.zh-Hans.md) · [日本語](getting-started.ja.md)

> **Contrato de trabajo R10:** este documento describe la experiencia externa que FerrumWeave 0.1 alpha debe certificar antes del release. Los comandos que aún no estén GREEN en CI son objetivos de producto, no afirmaciones de capacidad ya liberada.

FerrumWeave busca extender los templates normales de proyectos .NET agregando **Rust como opción de lenguaje**. Por eso la experiencia principal es `dotnet new <template> -lang Rust`, no un tipo de proyecto propio llamado `dotnet new rust`.

La secuencia de releases y la definición de soporte están documentadas en [`template-release-plan.es.md`](roadmap/template-release-plan.es.md). La decisión arquitectónica está registrada en [ADR 0004](architecture/adr/0004-r10-rust-as-dotnet-template-language.md).

## Primer uso objetivo de 0.1-alpha

Desde un entorno con los prerrequisitos documentados de .NET y Rust, instala la distribución alpha de FerrumWeave y crea un proyecto console .NET normal usando Rust:

```console
dotnet new install FerrumWeave.Sdk::0.1.0-alpha.1
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

`FerrumWeave.Sdk` es la identidad actual del paquete que contiene el MSBuild SDK y los templates de proyecto; el release no debe requerir un segundo paquete de templates no publicado a menos que un ADR posterior cambie explícitamente esa decisión.

El resultado de aceptación es un proyecto escrito en Rust que compila mediante el workflow normal del .NET SDK y alcanza causalmente la ruta de producto:

```text
Rust source
  -> rustc
  -> FerrumWeave CodegenBackend
  -> CIL / CLR metadata
  -> CoreCLR
```

El contrato de primer uso debe funcionar desde un directorio externo limpio donde no exista el repositorio fuente de FerrumWeave.

## Superficie de templates de 0.1-alpha

La primera alpha apunta a estas familias:

```console
dotnet new console  -lang Rust
dotnet new classlib -lang Rust
dotnet new xunit    -lang Rust
dotnet new nunit    -lang Rust
dotnet new mstest   -lang Rust
dotnet new web      -lang Rust
dotnet new webapi   -lang Rust
```

Un template no se considera soportado sólo porque genere archivos. Debe restaurar/compilar y completar su workflow significativo por defecto (`dotnet run` o `dotnet test`) mediante el backend real de FerrumWeave, con la certificación de plataforma correspondiente.

## FerrumWeave MSBuild SDK

El MSBuild SDK de FerrumWeave se distribuye como paquete NuGet SDK. Los proyectos generados usan conceptualmente el contrato versionado:

```xml
<Project Sdk="FerrumWeave.Sdk/0.1.0-alpha.1">
  <PropertyGroup>
    <TargetFramework>net10.0</TargetFramework>
  </PropertyGroup>
</Project>
```

Restore usa la resolución normal de .NET/NuGet:

```console
dotnet restore
```

El paquete contiene `Sdk.props`, `Sdk.targets` y el contenido de templates; los consumidores no deben copiar esos archivos manualmente.

## Probar prereleases empaquetados localmente

Antes de publicar, una alpha empacada localmente puede probarse registrando el directorio que contiene los `.nupkg` como fuente NuGet mediante un `NuGet.Config` de repositorio o proyecto:

```xml
<?xml version="1.0" encoding="utf-8"?>
<configuration>
  <packageSources>
    <clear />
    <add key="ferrumweave-local" value="../artifacts/packages" />
  </packageSources>
</configuration>
```

Luego se usan los mismos comandos normales de restore/build/test/run. El layout local o conocimiento exclusivo de maintainers no debe convertirse en prerrequisito oculto.

## Invariante de compatibilidad

Instalar FerrumWeave debe extender el template engine sin secuestrar variantes existentes:

```console
dotnet new console
dotnet new console -lang VB
dotnet new console -lang "F#"
dotnet new console -lang Rust
```

La meta es hacer de Rust otra opción seria de lenguaje .NET preservando el comportamiento esperado del SDK.
