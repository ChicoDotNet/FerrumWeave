#![feature(rustc_private)]

//! Thin rustc-facing adapter for the FerrumWeave CLR backend.
//!
//! This crate intentionally owns only the compiler-private integration boundary.
//! MIR lowering and CIL emission belong to FerrumWeave crates and are not
//! substituted by an upstream backend.

extern crate rustc_codegen_ssa;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use std::any::Any;

use rustc_codegen_ssa::{traits::CodegenBackend, CodegenResults, TargetConfig};
use rustc_data_structures::fx::FxIndexMap;
use rustc_metadata::EncodedMetadata;
use rustc_middle::{
    dep_graph::{WorkProduct, WorkProductId},
    ty::TyCtxt,
};
use rustc_session::{config::OutputFilenames, Session};
use rustc_span::{sym, Symbol};

const LOWERING_NOT_IMPLEMENTED: &str =
    "FERRUMWEAVE_BACKEND_REACHED_CODEGEN_CRATE: MIR lowering is not implemented";

struct FerrumWeaveCodegenBackend;

impl CodegenBackend for FerrumWeaveCodegenBackend {
    fn name(&self) -> &'static str {
        "ferrumweave"
    }

    fn locale_resource(&self) -> &'static str {
        ""
    }

    fn codegen_crate<'a>(&self, _tcx: TyCtxt<'_>) -> Box<dyn Any> {
        panic!("{LOWERING_NOT_IMPLEMENTED}");
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
        _ongoing_codegen: Box<dyn Any>,
        _sess: &Session,
        _outputs: &OutputFilenames,
    ) -> (CodegenResults, FxIndexMap<WorkProductId, WorkProduct>) {
        unreachable!("join_codegen cannot run before FerrumWeave MIR lowering exists")
    }

    fn link(
        &self,
        _sess: &Session,
        _codegen_results: CodegenResults,
        _metadata: EncodedMetadata,
        _outputs: &OutputFilenames,
    ) {
        unreachable!("link cannot run before FerrumWeave MIR lowering exists")
    }
}

#[no_mangle]
pub extern "Rust" fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    Box::new(FerrumWeaveCodegenBackend)
}
