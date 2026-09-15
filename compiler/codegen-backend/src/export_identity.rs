use ferrumweave_projection_types::managed_method_name_from_export_symbol;
use rustc_middle::{mir::mono::MonoItem, ty::TyCtxt};

/// Projects the rustc-owned symbol of the externally visible Rust export into
/// the managed public method name used by FerrumWeave metadata emission.
pub(crate) fn managed_export_method_name(tcx: TyCtxt<'_>) -> Result<String, String> {
    let codegen_units = tcx.collect_and_partition_mono_items(());
    for cgu in codegen_units.codegen_units {
        for (item, _data) in cgu.items() {
            let MonoItem::Fn(instance) = *item else {
                continue;
            };
            if !tcx
                .codegen_fn_attrs(instance.def_id())
                .contains_extern_indicator()
            {
                continue;
            }

            let export_symbol = tcx.symbol_name(instance).name;
            return Ok(managed_method_name_from_export_symbol(export_symbol.as_ref()));
        }
    }

    Err("no externally visible monomorphized export has a managed identity".to_owned())
}
