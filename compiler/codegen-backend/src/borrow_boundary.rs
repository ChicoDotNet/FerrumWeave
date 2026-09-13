use rustc_middle::{
    mir::{RETURN_PLACE, mono::MonoItem},
    ty::{self, TyCtxt},
};

#[path = "semantic_boundary.rs"]
mod semantic_boundary;

const BORROW_BOUNDARY_DIAGNOSTIC: &str = "FERRUMWEAVE_BORROW_BOUNDARY_REJECTED";
const EXPORT_SYMBOL: &str = "answer";

/// Reject Rust references that would escape FerrumWeave's currently supported managed boundary.
///
/// The check runs against monomorphized MIR, not Rust source text. Managed references preserve
/// CLR reachability and identity, but they do not encode Rust's shared/exclusive borrow and
/// lifetime guarantees, so FerrumWeave must not erase those guarantees into a GC reference.
pub(crate) fn reject_escaping_borrows(tcx: TyCtxt<'_>) -> Result<(), String> {
    semantic_boundary::reject_unsupported_export_semantics(tcx)?;

    let codegen_units = tcx.collect_and_partition_mono_items(());

    for cgu in codegen_units.codegen_units {
        for (item, _data) in cgu.items() {
            let MonoItem::Fn(instance) = *item else {
                continue;
            };
            if !instance.def_id().is_local() || tcx.symbol_name(instance).name != EXPORT_SYMBOL {
                continue;
            }

            let mir = tcx.instance_mir(instance.def);
            let return_ty = mir.local_decls[RETURN_PLACE].ty;
            if is_rust_borrow(return_ty) {
                return Err(format!(
                    "{BORROW_BOUNDARY_DIAGNOSTIC}: managed export `{EXPORT_SYMBOL}` returns Rust borrow `{return_ty:?}`; CLR reachability cannot preserve Rust borrow/lifetime guarantees"
                ));
            }

            for argument in mir.args_iter() {
                let argument_ty = mir.local_decls[argument].ty;
                if is_rust_borrow(argument_ty) {
                    return Err(format!(
                        "{BORROW_BOUNDARY_DIAGNOSTIC}: managed export `{EXPORT_SYMBOL}` accepts Rust borrow `{argument_ty:?}`; CLR reachability cannot preserve Rust borrow/lifetime guarantees"
                    ));
                }
            }
        }
    }

    Ok(())
}

fn is_rust_borrow(ty: rustc_middle::ty::Ty<'_>) -> bool {
    matches!(ty.kind(), ty::Ref(..))
}
