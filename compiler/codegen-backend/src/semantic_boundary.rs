use rustc_middle::{
    mir::{RETURN_PLACE, mono::MonoItem},
    ty::{Ty, TyCtxt, TyKind},
};

pub const DIAGNOSTIC: &str = "FERRUMWEAVE_SEMANTIC_BOUNDARY_REJECTED";
const EXPORT_SYMBOL: &str = "answer";

/// Reject Rust ABI shapes whose CLR projection is not yet owned by FerrumWeave.
///
/// This inspects rustc's monomorphized MIR, not Rust source text, and deliberately
/// runs only after richer supported projection lowerers have had first ownership.
pub fn reject_unsupported_export_semantics(tcx: TyCtxt<'_>) -> Result<(), String> {
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
            reject_type(tcx, mir.local_decls[RETURN_PLACE].ty)?;
            for argument in mir.args_iter() {
                reject_type(tcx, mir.local_decls[argument].ty)?;
            }
        }
    }

    Ok(())
}

fn reject_type(tcx: TyCtxt<'_>, ty: Ty<'_>) -> Result<(), String> {
    match ty.kind() {
        TyKind::Char => Err(format!(
            "{DIAGNOSTIC}: Rust char cannot cross the managed export boundary until FerrumWeave owns a scalar-value projection that preserves the full Unicode scalar range"
        )),
        TyKind::Adt(adt, _) if tcx.def_path_str(adt.did()) == "alloc::string::String" => Err(format!(
            "{DIAGNOSTIC}: Rust String cannot cross the managed export boundary until FerrumWeave owns an explicit UTF-8/UTF-16 projection policy"
        )),
        _ => Ok(()),
    }
}
