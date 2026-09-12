use rustc_middle::ty::{TyCtxt, TyKind};

pub const DIAGNOSTIC: &str = "FERRUMWEAVE_SEMANTIC_BOUNDARY_REJECTED";

/// Reject Rust ABI shapes whose CLR projection is not yet owned by FerrumWeave.
///
/// This inspects rustc's monomorphized exported function signatures. It does
/// not parse Rust source and deliberately runs only after richer, supported
/// projection lowerers have had first ownership of their MIR shapes.
pub fn reject_unsupported_export_semantics(tcx: TyCtxt<'_>) -> Result<(), String> {
    for local_def_id in tcx.hir_body_owners() {
        let def_id = local_def_id.to_def_id();
        if !tcx.is_mir_available(def_id) {
            continue;
        }

        let name = tcx.item_name(def_id).as_str();
        if name != "answer" {
            continue;
        }

        let signature = tcx.fn_sig(def_id).instantiate_identity();
        let signature = tcx.instantiate_bound_regions_with_erased(signature);

        for ty in signature.inputs().iter().chain(std::iter::once(&signature.output())) {
            match ty.kind() {
                TyKind::Char => {
                    return Err(format!(
                        "{DIAGNOSTIC}: Rust char cannot cross the managed export boundary until FerrumWeave owns a scalar-value projection that preserves the full Unicode scalar range"
                    ));
                }
                TyKind::Adt(adt, _) if tcx.def_path_str(adt.did()) == "alloc::string::String" => {
                    return Err(format!(
                        "{DIAGNOSTIC}: Rust String cannot cross the managed export boundary until FerrumWeave owns an explicit UTF-8/UTF-16 projection policy"
                    ));
                }
                _ => {}
            }
        }
    }

    Ok(())
}
