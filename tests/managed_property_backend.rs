mod support;

use std::fs;
use std::path::{Path, PathBuf};

use support::{
    assert_success, build_codegen_backend, compile_rust_source, create_work_dir,
    make_csharp_consumer, remove_work_dir, run_managed_consumer,
};

#[test]
fn managed_property_is_source_causal_through_the_ferrumweave_backend() {
    let backend = build_codegen_backend();
    let work = create_work_dir("managed-property");

    let length_3 = compile_variant(&backend, &work, "length_3", 3, "answer");
    let length_7 = compile_variant(&backend, &work, "length_7", 7, "answer");
    let renamed = compile_variant(&backend, &work, "renamed_export", 3, "compute_result");

    assert_property_target(&length_3, "Answer");
    assert_property_target(&length_7, "Answer");
    assert_property_target(&renamed, "ComputeResult");

    assert_eq!(run_consumer(&length_3, &work, "length_3", "Answer"), 3);
    assert_eq!(run_consumer(&length_7, &work, "length_7", "Answer"), 7);
    assert_eq!(
        run_consumer(&renamed, &work, "renamed_export", "ComputeResult"),
        3
    );

    let baseline = fs::read(&length_3).expect("baseline property artifact should be readable");
    assert_ne!(
        baseline,
        fs::read(&length_7).expect("payload-mutated property artifact should be readable"),
        "changing only the Rust property payload 3 -> 7 must change the managed artifact"
    );
    assert_ne!(
        baseline,
        fs::read(&renamed).expect("renamed property artifact should be readable"),
        "changing only the Rust export name answer -> compute_result must change managed metadata"
    );

    remove_work_dir(&work);
}

fn rust_source(value: i32, export_name: &str) -> String {
    format!(
        "#[inline(never)]\nfn ferrumweave_system_text_string_builder_length(value: i32) -> i32 {{ value }}\n\n#[no_mangle]\npub extern \"C\" fn {export_name}() -> i32 {{ ferrumweave_system_text_string_builder_length({value}) }}\n"
    )
}

fn compile_variant(
    backend: &Path,
    work: &Path,
    name: &str,
    value: i32,
    export_name: &str,
) -> PathBuf {
    compile_rust_source(
        backend,
        work,
        &format!("{name}.rs"),
        &format!("{name}.dll"),
        &rust_source(value, export_name),
        &format!("FerrumWeave should lower managed property variant {name}"),
    )
}

fn assert_property_target(artifact: &Path, export_method: &str) {
    let image = fs::read(artifact).expect("managed property artifact should be readable");
    for expected in [
        "MZ",
        "BSJB",
        "FerrumWeave.Generated",
        "RustApi",
        export_method,
        "System",
        "Text",
        "StringBuilder",
        "set_Length",
        "get_Length",
    ] {
        assert!(
            image
                .windows(expected.len())
                .any(|window| window == expected.as_bytes()),
            "managed property artifact is missing {expected:?}"
        );
    }
}

fn run_consumer(artifact: &Path, root: &Path, name: &str, method_name: &str) -> i32 {
    let program = format!(
        "var method = typeof(FerrumWeave.RustApi).GetMethod(\"{method_name}\", System.Reflection.BindingFlags.Public | System.Reflection.BindingFlags.Static)!;\n\
var il = method.GetMethodBody()!.GetILAsByteArray()!;\n\
var callvirtCount = 0;\n\
foreach (var value in il) if (value == (byte)0x6F) callvirtCount++;\n\
if (callvirtCount < 2) throw new System.Exception(\"{method_name} must contain property setter and getter callvirt opcodes\");\n\
System.Console.WriteLine(FerrumWeave.RustApi.{method_name}());\n"
    );
    let project = make_csharp_consumer(artifact, root, name, &program);
    let output = run_managed_consumer(&project, &format!("managed property consumer for {name}"));
    assert_success(
        &output,
        &format!("managed property consumer should pass for {name}"),
    );

    let stdout = String::from_utf8(output.stdout).expect("consumer stdout should be UTF-8");
    let observed = stdout
        .lines()
        .rfind(|line| !line.trim().is_empty())
        .expect("managed property consumer should produce an observable")
        .trim();
    observed.parse::<i32>().unwrap_or_else(|error| {
        panic!("unexpected managed property observable for {name}: {observed:?}: {error}")
    })
}
