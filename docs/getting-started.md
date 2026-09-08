# FerrumWeave 0.1 alpha — getting started

FerrumWeave's MSBuild SDK is distributed as a NuGet SDK package. For the current alpha, use the exact SDK reference:

```xml
<Project Sdk="FerrumWeave.Sdk/0.1.0-alpha.1">
  <PropertyGroup>
    <TargetFramework>net10.0</TargetFramework>
  </PropertyGroup>
</Project>
```

Restore the project through normal .NET/NuGet resolution:

```console
dotnet restore
```

When testing a locally packed alpha before publication, add the directory containing `FerrumWeave.Sdk.0.1.0-alpha.1.nupkg` as a NuGet source, for example:

```console
dotnet restore --source ./artifacts/packages
```

The SDK package contains the FerrumWeave `Sdk.props` and `Sdk.targets`; consumers should not copy those files into their projects manually.
