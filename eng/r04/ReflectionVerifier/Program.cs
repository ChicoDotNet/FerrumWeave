using System.Reflection;
using System.Runtime.Loader;

if (args.Length != 1)
{
    Console.Error.WriteLine("usage: ReflectionVerifier <assembly-path>");
    return 2;
}

var assemblyPath = Path.GetFullPath(args[0]);
var assembly = AssemblyLoadContext.Default.LoadFromAssemblyPath(assemblyPath);
var entryPoint = assembly.EntryPoint
    ?? throw new InvalidOperationException("emitted assembly has no CLR entry point");

if (entryPoint.ReturnType != typeof(void))
{
    throw new InvalidOperationException($"entry point return type was {entryPoint.ReturnType}");
}

if (entryPoint.GetParameters().Length != 0)
{
    throw new InvalidOperationException("entry point unexpectedly declares parameters");
}

if (!entryPoint.IsStatic || !entryPoint.IsPublic)
{
    throw new InvalidOperationException("entry point must remain public static");
}

AssertGlobalProbe(assembly.ManifestModule, "ProbeI32", typeof(int));
AssertGlobalProbe(assembly.ManifestModule, "ProbeBoolean", typeof(bool));
AssertGlobalProbe(assembly.ManifestModule, "ProbeString", typeof(string));
AssertGlobalProbe(assembly.ManifestModule, "ProbeObject", typeof(object));
AssertGlobalProbe(assembly.ManifestModule, "ProbeI32Array", typeof(int[]));

Console.WriteLine($"reflected {assembly.GetName().Name}::{entryPoint.Name} and representative R04 CTS signatures");
return 0;

static void AssertGlobalProbe(Module module, string methodName, Type parameterType)
{
    var method = module
        .GetMethods(BindingFlags.Public | BindingFlags.Static)
        .SingleOrDefault(candidate => candidate.Name == methodName)
        ?? throw new InvalidOperationException($"missing emitted global method {methodName}");

    if (method.ReturnType != typeof(void))
    {
        throw new InvalidOperationException($"{methodName} return type was {method.ReturnType}");
    }

    var parameters = method.GetParameters();
    if (parameters.Length != 1 || parameters[0].ParameterType != parameterType)
    {
        var observed = parameters.Length == 1 ? parameters[0].ParameterType.ToString() : $"{parameters.Length} parameters";
        throw new InvalidOperationException($"{methodName} parameter was {observed}, expected {parameterType}");
    }
}
