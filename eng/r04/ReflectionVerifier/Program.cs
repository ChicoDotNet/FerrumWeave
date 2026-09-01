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

Console.WriteLine($"reflected {assembly.GetName().Name}::{entryPoint.Name} as public static void {entryPoint.Name}()");
return 0;
