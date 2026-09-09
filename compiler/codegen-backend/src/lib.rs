#![feature(rustc_private)]

//! rustc-facing adapter for the FerrumWeave CLR backend.
//!
//! This crate owns the compiler-private integration boundary. The first product
//! slice deliberately lowers only one mechanically falsifiable MIR shape: an
//! exported `answer() -> i32` whose return place is assigned an integer constant.
//! The value is read from MIR and passed into FerrumWeave's managed emitter.

extern crate rustc_codegen_ssa;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use std::{any::Any, fs};

use ferrumweave_cil::emit_i32_export_assembly;
use rustc_codegen_ssa::{
    CodegenResults, CompiledModule, CrateInfo, ModuleKind, TargetConfig,
    traits::CodegenBackend,
};
use rustc_data_structures::fx::FxIndexMap;
use rustc_metadata::EncodedMetadata;
use rustc_middle::{
    dep_graph::{WorkProduct, WorkProductId},
    mir::{ConstValue, Operand, Rvalue, StatementKind, RETURN_PLACE, mono::MonoItem},
    ty::{TyCtxt, TypingEnv},
};
use rustc_session::{
    Session,
    config::{OutputFilenames, OutputType},
};
use rustc_span::{Symbol, sym};

const EXPORT_SYMBOL: &str = "answer";

struct GeneratedArtifact {
    image: Vec<u8>,
    crate_info: CrateInfo,
}

struct FerrumWeaveCodegenBackend;

impl CodegenBackend for FerrumWeaveCodegenBackend {
    fn name(&self) -> &'static str {
        "ferrumweave"
    }

    fn locale_resource(&self) -> &'static str {
        ""
    }

    fn codegen_crate<'a>(&self, tcx: TyCtxt<'_>) -> Box<dyn Any> {
        let value = lower_exported_i32_constant(tcx).unwrap_or_else(|message| {
            panic!("FERRUMWEAVE_MIR_LOWERING_FAILED: {message}")
        });
        let image = emit_i32_export_assembly(value);

        Box::new(GeneratedArtifact {
            image,
            crate_info: CrateInfo::new(tcx, "ferrumweave".to_owned()),
        })
    }

    fn target_config(&self, sess: &Session) -> TargetConfig {
        let target_features = if sess.target.arch == "x86_64" && sess.target.os != "none" {
            vec![sym::sse, Symbol::intern("x87")]
        } else {
            vec![]
        };

        TargetConfig {
            unstable_target_features: target_features.clone(),
            target_features,
            has_reliable_f16: false,
            has_reliable_f16_math: false,
            has_reliable_f128: false,
            has_reliable_f128_math: false,
        }
    }

    fn join_codegen(
        &self,
        ongoing_codegen: Box<dyn Any>,
        _sess: &Session,
        outputs: &OutputFilenames,
    ) -> (CodegenResults, FxIndexMap<WorkProductId, WorkProduct>) {
        let GeneratedArtifact { image, crate_info } = *ongoing_codegen
            .downcast::<GeneratedArtifact>()
            .expect("FerrumWeave ongoing codegen state has the wrong type");

        let object = outputs.temp_path_for_cgu(OutputType::Object, "ferrumweave", None);
        fs::write(&object, image).expect("FerrumWeave could not write its managed codegen artifact");

        let modules = vec![CompiledModule {
            name: "ferrumweave".into(),
            kind: ModuleKind::Regular,
            object: Some(object),
            bytecode: None,
            dwarf_object: None,
            llvm_ir: None,
            assembly: None,
            links_from_incr_cache: Vec::new(),
        }];

        (
            CodegenResults {
                modules,
                allocator_module: None,
                crate_info,
            },
            FxIndexMap::default(),
        )
    }

    fn link(
        &self,
        _sess: &Session,
        codegen_results: CodegenResults,
        _metadata: EncodedMetadata,
        outputs: &OutputFilenames,
    ) {
        let source = codegen_results
            .modules
            .iter()
            .find_map(|module| module.object.as_ref())
            .expect("FerrumWeave link phase did not receive a managed artifact");
        let destination = outputs.path(OutputType::Exe);
        fs::copy(source, destination.as_path())
            .expect("FerrumWeave could not publish the managed assembly");
    }
}

fn lower_exported_i32_constant(tcx: TyCtxt<'_>) -> Result<i32, String> {
    let codegen_units = tcx.collect_and_partition_mono_items(());

    for cgu in codegen_units.codegen_units {
        for (item, _data) in cgu.items() {
            let MonoItem::Fn(instance) = *item else {
                continue;
            };
            if tcx.symbol_name(instance).name != EXPORT_SYMBOL {
                continue;
            }

            let mir = tcx.instance_mir(instance.def);
            for block in mir.basic_blocks.iter() {
                for statement in &block.statements {
                    let StatementKind::Assign(assignment) = &statement.kind else {
                        continue;
                    };
                    let (place, rvalue) = assignment.as_ref();
                    if place.local != RETURN_PLACE || !place.projection.is_empty() {
                        continue;
                    }
                    let Rvalue::Use(Operand::Constant(constant)) = rvalue else {
                        return Err(format!(
                            "{EXPORT_SYMBOL} return value is not a constant MIR operand: {rvalue:?}"
                        ));
                    };

                    let evaluated = constant
                        .const_
                        .eval(tcx, TypingEnv::fully_monomorphized(), constant.span)
                        .map_err(|_| {
                            format!("could not evaluate {EXPORT_SYMBOL} return constant from MIR")
                        })?;
                    let ConstValue::Scalar(scalar) = evaluated else {
                        return Err(format!(
                            "{EXPORT_SYMBOL} return constant is not a scalar: {evaluated:?}"
                        ));
                    };
                    return scalar.to_i32().report_err().map_err(|_| {
                        format!("{EXPORT_SYMBOL} return scalar is not a valid i32")
                    });
                }
            }

            return Err(format!(
                "{EXPORT_SYMBOL} MIR never assigns a constant to the return place"
            ));
        }
    }

    Err(format!(
        "no monomorphized `{EXPORT_SYMBOL}` export reached FerrumWeave codegen"
    ))
}

#[no_mangle]
pub extern "Rust" fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    Box::new(FerrumWeaveCodegenBackend)
}
