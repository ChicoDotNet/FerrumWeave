use rustc_middle::{
    mir::{TerminatorKind, mono::MonoItem},
    ty::{self, TyCtxt},
};

const PANIC_BOUNDARY_DIAGNOSTIC: &str = "FERRUMWEAVE_PANIC_BOUNDARY_REJECTED";

/// Enforce FerrumWeave's current managed-boundary rule before any CLR artifact is emitted.
///
/// This first backend-owned enforcement increment deliberately recognizes direct panic calls in
/// monomorphized local Rust MIR. It does not parse Rust source and it does not attempt to model
/// transitive unwind containment yet; broader call-graph analysis belongs to a later contract.
pub(crate) fn reject_direct_uncontained_panics(tcx: TyCtxt<'_>) -> Result<(), String> {
    let codegen_units = tcx.collect_and_partition_mono_items(());

    for cgu in codegen_units.codegen_units {
        for (item, _data) in cgu.items() {
            let MonoItem::Fn(instance) = *item else {
                continue;
            };
            if !instance.def_id().is_local() {
                continue;
            }

            let mir = tcx.instance_mir(instance.def);
            for block in mir.basic_blocks.iter() {
                let TerminatorKind::Call { func, .. } = &block.terminator().kind else {
                    continue;
                };
                let func_ty = func.ty(&mir.local_decls, tcx);
                let ty::FnDef(def_id, _) = *func_ty.kind() else {
                    continue;
                };
                if def_id.is_local() {
                    continue;
                }

                let callee_name = tcx.item_name(def_id);
                if !is_direct_panic_target(callee_name.as_str()) {
                    continue;
                }

                let caller = tcx.symbol_name(instance).name;
                return Err(format!(
                    "{PANIC_BOUNDARY_DIAGNOSTIC}: direct Rust panic target `{callee_name}` is not permitted while compiling managed export candidate `{caller}`; contain or model failure explicitly"
                ));
            }
        }
    }

    Ok(())
}

fn is_direct_panic_target(name: &str) -> bool {
    matches!(
        name,
        "panic_fmt" | "panic" | "panic_nounwind_fmt" | "panic_nounwind" | "begin_panic"
    )
}

#[cfg(test)]
mod tests {
    use super::is_direct_panic_target;

    #[test]
    fn recognizes_external_panic_entrypoint_names_without_source_text() {
        assert!(is_direct_panic_target("panic_fmt"));
        assert!(is_direct_panic_target("panic"));
        assert!(is_direct_panic_target("panic_nounwind_fmt"));
        assert!(is_direct_panic_target("panic_nounwind"));
        assert!(is_direct_panic_target("begin_panic"));
        assert!(!is_direct_panic_target("panic_boundary_i32"));
        assert!(!is_direct_panic_target("helper_add"));
    }
}
