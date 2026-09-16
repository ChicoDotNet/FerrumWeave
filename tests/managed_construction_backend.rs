mod support;

use std::fs;
use std::path::{Path, PathBuf};

use support::{
    assert_success, build_codegen_backend, compile_rust_source, create_work_dir,
    make_csharp_consumer, remove_work_dir, run_managed_consumer,
};

#[test]
fn managed_construction_is_source_causal_through_the_ferrumweave_backend() {
    let backend = build_codegen_backend();
    let work = create_work_dir("managed-construction");

    let object_137 = compile_variant(
        &backend,
        &work,
        "object_137",
        "ferrumweave_system_object_new",
        137,
        "answer",
    );
    let object_211 = compile_variant(
        &backend,
        &work,
        "object_211",
        "ferrumweave_system_object_new",
        211,
        "answer",
    );
    let string_builder_137 = compile_variant(
        &backend,
        &work,
        "string_builder_137",
        "ferrumweave_system_text_string_builder_new",
        137,
        "answer",
    );
    let renamed = compile_variant(
        &backend,
        &work,
        "object_137_renamed",
        "ferrumweave_system_object_new",
        137,
        "compute_result",
    );

    assert_constructor_target(&object_137, &["System", "Object", ".ctor"], "Answer");
    assert_constructor_target(&object_211, &["System", "Object", ".ctor"], "Answer");
    assert_constructor_target(
        &string_builder_137,
        &["System", "Text", "StringBuilder", ".ctor"],
        "Answer",
    );
    assert_constructor_target(&renamed, &["System", "Object", ".ctor"], "ComputeResult");

    assert_eq!(run_consumer(&object_137, &work, "object_137", "Answer"), 137);
    assert_eq!(run_consumer(&object_211, &work, "object_211", "Answer"), 211);
    assert_eq!(
        run_consumer(&string_builder_137, &work, "string_builder_137", "Answer"),
        137
    );
    assert_eq!(
        run_consumer(&renamed, &work, "object_137_renamed", "ComputeResult"),
        137
    );

    let baseline = fs::read(&object_137).expect("baseline construction artifact should be readable");
    assert_ne!(
        baseline,
        fs::read(&object_211).expect("payload-mutated construction artifact should be readable"),
        "changing only the Rust payload 137 -> 211 must change the managed artifact"
    );
    assert_ne!(
        baseline,
        fs::read(&string_builder_137)
            .expect("constructor-mutated construction artifact should be readable"),
        "changing only the Rust constructor Object -> StringBuilder must change the managed artifact"
    );
    assert_ne!(
        baseline,
        fs::read(&renamed).expect("renamed construction artifact should be readable"),
        "changing only the Rust export name answer -> compute_result must change managed metadata"
    );

    remove_work_dir(&work);
}

fn rust_source(marker: &str, value: i32, export_name: &str) -> String {
    assert!(matches!(
        marker,
        "ferrumweave_system_object_new" | "ferrumweave_system_text_string_builder_new"
    ));
    format!(
        "#[inline(never)]\nfn ferrumweave_system_object_new(value: i32) -> i32 {{ value }}\n\n#[inline(never)]\nfn ferrumweave_system_text_string_builder_new(value: i32) -> i32 {{ value }}\n\n#[no_mangle]\npub extern \"C\" fn {export_name}() -> i32 {{ {marker}({value}) }}\n"
    )
}

fn compile_variant(
    backend: &Path,
    work: &Path,
    name: &str,
    marker: &str,
    value: i32,
    export_name: &str,
) -> PathBuf {
    compile_rust_source(
        backend,
        work,
        &format!("{name}.rs"),
        &format!("{name}.dll"),
        &rust_source(marker, value, export_name),
        &format!("FerrumWeave should lower managed construction variant {name}"),
    )
}

fn assert_constructor_target(artifact: &Path, required: &[&str], export_method: &str) {
    let image = fs::read(artifact).expect("managed construction artifact should be readable");
    for expected in ["MZ", "BSJB", "FerrumWeave.Generated", "RustApi", export_method]
        .into_iter()
        .chain(required.iter().copied())
    {
        assert!(
            image
                .windows(expected.len())
                .any(|window| window == expected.as_bytes()),
            "managed construction artifact is missing {expected:?}"
        );
    }
}

fn run_consumer(artifact: &Path, root: &Path, name: &str, method_name: &str) -> i32 {
    let program = format!(
        "using System.Reflection;\n\
var method = typeof(FerrumWeave.RustApi).GetMethod(\"{method_name}\", BindingFlags.Public | BindingFlags.Static)!;\n\
var il = method.GetMethodBody()!.GetILAsByteArray()!;\n\
if (!il.Contains((byte)0x73)) throw new System.Exception(\"{method_name} contains no managed newobj opcode\");\n\
System.Console.WriteLine(FerrumWeave.RustApi.{method_name}());\n"
    );
    let project = make_csharp_consumer(artifact, root, name, &program);
    let output = run_managed_consumer(
        &project,
        &format!("managed construction consumer for {name}"),
    );
    assert_success(
        &output,
        &format!("managed construction consumer should pass for {name}"),
    );

    let stdout = String::from_utf8(output.stdout).expect("consumer stdout should be UTF-8");
    let observed = stdout
        .lines()
        .rfind(|line| !line.trim().is_empty())
        .expect("managed construction consumer should produce an observable")
        .trim();
    observed.parse::<i32>().unwrap_or_else(|error| {
        panic!("unexpected managed construction observable for {name}: {observed:?}: {error}")
    })
}
