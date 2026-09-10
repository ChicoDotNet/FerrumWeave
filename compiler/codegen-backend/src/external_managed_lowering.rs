use rustc_middle::{
    mir::{ConstValue, Operand, TerminatorKind, RETURN_PLACE, mono::MonoItem},
    ty::{self, TyCtxt, TypingEnv},
};

const EXPORT_SYMBOL: &str = "answer";
const EXTERNAL_MANAGED_TRANSFORM_MARKER: &str = "ferrumweave_external_managed_transform";

/// Recognize the narrow external-managed marker from rustc MIR.
///
/// `Ok(None)` means this export is not the external-managed slice and lets the
/// normal FerrumWeave lowering continue. `Ok(Some(payload))` carries only the
/// Rust-derived semantic payload to the CIL emitter.
pub(crate) fn lower_external_managed_transform(
    tcx: TyCtxt<'_>,
) -> Result<Option<i32>, String> {
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
                let TerminatorKind::Call {
                    func,
                    args,
                    destination,
                    ..
                } = &block.terminator().kind
                else {
                    continue;
                };
                if destination.local != RETURN_PLACE || !destination.projection.is_empty() {
                    continue;
                }

                let func_ty = func.ty(&mir.local_decls, tcx);
                let ty::FnDef(def_id, _) = *func_ty.kind() else {
                    continue;
                };
                if tcx.item_name(def_id).as_str() != EXTERNAL_MANAGED_TRANSFORM_MARKER {
                    continue;
                }
                if args.len() != 1 {
                    return Err(format!(
                        "{EXPORT_SYMBOL} external managed marker requires exactly one i32 payload"
                    ));
                }
                return lower_i32_constant_operand(tcx, &args[0].node).map(Some);
            }
            return Ok(None);
        }
    }
    Ok(None)
}

fn lower_i32_constant_operand<'tcx>(
    tcx: TyCtxt<'tcx>,
    operand: &Operand<'tcx>,
) -> Result<i32, String> {
    let Operand::Constant(constant) = operand else {
        return Err(format!(
            "external managed marker requires a constant i32 MIR operand, found {operand:?}"
        ));
    };
    let evaluated = constant
        .const_
        .eval(tcx, TypingEnv::fully_monomorphized(), constant.span)
        .map_err(|_| "could not evaluate external managed i32 payload from MIR".to_owned())?;
    let ConstValue::Scalar(scalar) = evaluated else {
        return Err(format!("external managed MIR constant is not a scalar: {evaluated:?}"));
    };
    scalar
        .to_i32()
        .report_err()
        .map_err(|_| "external managed MIR scalar is not a valid i32".to_owned())
}
