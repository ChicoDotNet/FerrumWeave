#![feature(rustc_private)]

//! rustc-facing adapter for the FerrumWeave CLR backend.
extern crate rustc_codegen_ssa;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use std::{any::Any, fs};
use ferrumweave_cil::{
    emit_constructible_i32_instance_assembly, emit_disposable_resource_assembly,
    emit_i32_argument_export_assembly, emit_i32_arithmetic_export_assembly,
    emit_i32_control_flow_export_assembly, emit_i32_direct_call_export_assembly,
    emit_i32_export_with_external_managed_transform, emit_i32_export_with_managed_construction,
    emit_i32_export_with_managed_instance_call, emit_i32_export_with_named_system_math_call,
    emit_i32_export_with_string_builder_length_property, emit_i32_invalid_operation_export_assembly,
    emit_named_i32_export_assembly, emit_named_i32_method_export_assembly,
    emit_option_reference_export_assembly, emit_option_value_export_assembly,
};
use rustc_codegen_ssa::{CodegenResults, CompiledModule, CrateInfo, ModuleKind, TargetConfig, traits::CodegenBackend};
use rustc_data_structures::fx::FxIndexMap;
use rustc_metadata::EncodedMetadata;
use rustc_middle::{dep_graph::{WorkProduct, WorkProductId}, ty::TyCtxt};
use rustc_session::{Session, config::{OutputFilenames, OutputType}};
use rustc_span::{Symbol, sym};

mod borrow_boundary;
mod disposable_resource_lowering;
mod external_managed_lowering;
mod lowering;
mod option_reference_lowering;
mod option_value_lowering;
mod panic_boundary;
mod result_failure_lowering;
mod result_success_lowering;
mod rust_type_lowering;
use borrow_boundary::reject_escaping_borrows;
use disposable_resource_lowering::lower_disposable_resource;
use external_managed_lowering::lower_external_managed_transform;
use lowering::{LoweredI32Export, lower_exported_i32};
use option_reference_lowering::lower_option_reference_exports;
use option_value_lowering::lower_option_value_exports;
use panic_boundary::reject_direct_uncontained_panics;
use result_failure_lowering::lower_result_failure;
use result_success_lowering::lower_result_success;
use rust_type_lowering::lower_constructible_i32_instance;

struct GeneratedArtifact { image: Vec<u8>, crate_info: CrateInfo }
struct FerrumWeaveCodegenBackend;

impl CodegenBackend for FerrumWeaveCodegenBackend {
    fn name(&self) -> &'static str { "ferrumweave" }
    fn locale_resource(&self) -> &'static str { "" }

    fn codegen_crate<'a>(&self, tcx: TyCtxt<'_>) -> Box<dyn Any> {
        reject_direct_uncontained_panics(tcx).unwrap_or_else(|message| panic!("{message}"));
        reject_escaping_borrows(tcx).unwrap_or_else(|message| panic!("{message}"));
        let assembly_name = tcx.sess.opts.crate_name.as_deref().unwrap_or("FerrumWeave.Generated");
        let result_failure = lower_result_failure(tcx)
            .unwrap_or_else(|message| panic!("FERRUMWEAVE_MIR_LOWERING_FAILED: {message}"));
        let result_success = if result_failure.is_none() {
            lower_result_success(tcx)
                .unwrap_or_else(|message| panic!("FERRUMWEAVE_MIR_LOWERING_FAILED: {message}"))
        } else {
            None
        };
        let option_value = if result_failure.is_none() && result_success.is_none() {
            lower_option_value_exports(tcx).unwrap_or_else(|message| panic!("FERRUMWEAVE_MIR_LOWERING_FAILED: {message}"))
        } else { None };
        let option_reference = if result_failure.is_none() && result_success.is_none() && option_value.is_none() {
            lower_option_reference_exports(tcx).unwrap_or_else(|message| panic!("FERRUMWEAVE_MIR_LOWERING_FAILED: {message}"))
        } else { None };
        let disposable_resource = lower_disposable_resource(tcx)
            .unwrap_or_else(|message| panic!("FERRUMWEAVE_MIR_LOWERING_FAILED: {message}"));
        let rust_instance = if disposable_resource.is_none() {
            lower_constructible_i32_instance(tcx)
                .unwrap_or_else(|message| panic!("FERRUMWEAVE_MIR_LOWERING_FAILED: {message}"))
        } else {
            None
        };
        let external_payload = if disposable_resource.is_none() {
            lower_external_managed_transform(tcx)
                .unwrap_or_else(|message| panic!("FERRUMWEAVE_MIR_LOWERING_FAILED: {message}"))
        } else {
            None
        };

        let image = if let Some(result) = result_failure {
            emit_i32_invalid_operation_export_assembly(
                assembly_name,
                "FerrumWeave",
                "RustApi",
                &result.method_name,
                &result.value.to_string(),
            )
        } else if let Some(result) = result_success {
            emit_named_i32_method_export_assembly(assembly_name, "FerrumWeave", "RustApi", &result.method_name, result.value)
        } else if let Some(option) = option_value {
            emit_option_value_export_assembly(assembly_name, option.namespace, option.type_name, &option.some_method_name, &option.none_method_name, option.some_value)
        } else if let Some(option) = option_reference {
            emit_option_reference_export_assembly(assembly_name, option.namespace, option.type_name, &option.some_method_name, &option.none_method_name, &option.some_value)
        } else if let Some(resource) = disposable_resource {
            emit_disposable_resource_assembly(
                assembly_name,
                resource.namespace,
                &resource.type_name,
                resource.seed,
                resource.release_increment,
            )
        } else if let Some(instance) = rust_instance {
            emit_constructible_i32_instance_assembly(assembly_name, instance.namespace, &instance.type_name, &instance.method_name, instance.value)
        } else if let Some(payload) = external_payload {
            emit_i32_export_with_external_managed_transform(payload)
        } else {
            let lowered = lower_exported_i32(tcx).unwrap_or_else(|message| panic!("FERRUMWEAVE_MIR_LOWERING_FAILED: {message}"));
            match lowered {
                LoweredI32Export::Constant(value) => emit_named_i32_export_assembly(assembly_name, value),
                LoweredI32Export::Argument(index) => emit_i32_argument_export_assembly(index),
                LoweredI32Export::Arithmetic(operation) => emit_i32_arithmetic_export_assembly(operation),
                LoweredI32Export::ControlFlow { predicate, true_argument, false_argument } => emit_i32_control_flow_export_assembly(predicate, true_argument, false_argument),
                LoweredI32Export::DirectRustCall(operation) => emit_i32_direct_call_export_assembly(operation),
                LoweredI32Export::SystemMath { method, argument } => emit_i32_export_with_named_system_math_call(assembly_name, method, argument),
                LoweredI32Export::ManagedConstruction { constructor, payload } => emit_i32_export_with_managed_construction(constructor, payload),
                LoweredI32Export::ManagedInstance { receiver, payload } => emit_i32_export_with_managed_instance_call(receiver, payload),
                LoweredI32Export::ManagedStringBuilderLength { payload } => emit_i32_export_with_string_builder_length_property(payload),
            }
        };
        Box::new(GeneratedArtifact { image, crate_info: CrateInfo::new(tcx, "ferrumweave".to_owned()) })
    }

    fn target_config(&self, sess: &Session) -> TargetConfig {
        let target_features = if sess.target.arch == "x86_64" && sess.target.os != "none" { vec![sym::sse, sym::sse2, Symbol::intern("x87")] } else { vec![] };
        TargetConfig { unstable_target_features: target_features.clone(), target_features, has_reliable_f16: false, has_reliable_f16_math: false, has_reliable_f128: false, has_reliable_f128_math: false }
    }

    fn join_codegen(&self, ongoing_codegen: Box<dyn Any>, _sess: &Session, outputs: &OutputFilenames) -> (CodegenResults, FxIndexMap<WorkProductId, WorkProduct>) {
        let GeneratedArtifact { image, crate_info } = *ongoing_codegen.downcast::<GeneratedArtifact>().expect("FerrumWeave ongoing codegen state has the wrong type");
        let object = outputs.temp_path_for_cgu(OutputType::Object, "ferrumweave", None);
        fs::write(&object, image).expect("FerrumWeave could not write its managed codegen artifact");
        let modules = vec![CompiledModule { name: "ferrumweave".into(), kind: ModuleKind::Regular, object: Some(object), bytecode: None, dwarf_object: None, llvm_ir: None, assembly: None, links_from_incr_cache: Vec::new() }];
        (CodegenResults { modules, allocator_module: None, crate_info }, FxIndexMap::default())
    }

    fn link(&self, _sess: &Session, codegen_results: CodegenResults, _metadata: EncodedMetadata, outputs: &OutputFilenames) {
        let source = codegen_results.modules.iter().find_map(|module| module.object.as_ref()).expect("FerrumWeave link phase did not receive a managed artifact");
        let destination = outputs.path(OutputType::Exe);
        fs::copy(source, destination.as_path()).expect("FerrumWeave could not publish the managed assembly");
    }
}

#[no_mangle]
pub extern "Rust" fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> { Box::new(FerrumWeaveCodegenBackend) }
