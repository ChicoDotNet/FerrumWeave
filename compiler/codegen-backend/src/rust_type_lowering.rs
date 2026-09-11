use rustc_middle::{
    mir::{ConstValue, Operand, RETURN_PLACE, Rvalue, StatementKind, TerminatorKind, mono::MonoItem},
    ty::{self, TyCtxt, TypingEnv},
};

const EXPORT_SYMBOL: &str = "answer";
const CLR_NAMESPACE: &str = "FerrumWeave";

pub(crate) struct LoweredRustInstanceType {
    pub(crate) namespace: &'static str,
    pub(crate) type_name: String,
    pub(crate) method_name: String,
    pub(crate) value: i32,
}

/// Recognizes the smallest Rust-owned type projection currently supported:
/// an exported i32 wrapper whose return is a direct call to a local method on
/// `&ADT`, where the method itself returns an i32 constant from MIR.
///
/// Non-matching exports deliberately return `Ok(None)` so the established
/// scalar/call lowering families retain ownership of their shapes.
pub(crate) fn lower_constructible_i32_instance(
    tcx: TyCtxt<'_>,
) -> Result<Option<LoweredRustInstanceType>, String> {
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
                if destination.local != RETURN_PLACE
                    || !destination.projection.is_empty()
                    || args.len() != 1
                {
                    continue;
                }

                let func_ty = func.ty(&mir.local_decls, tcx);
                let ty::FnDef(def_id, _) = *func_ty.kind() else {
                    continue;
                };
                if !def_id.is_local() {
                    continue;
                }

                let receiver_ty = args[0].node.ty(&mir.local_decls, tcx);
                let ty::Ref(_, referent, _) = *receiver_ty.kind() else {
                    continue;
                };
                let ty::Adt(adt, _) = *referent.kind() else {
                    continue;
                };

                let callee_mir = tcx.instance_mir(ty::InstanceKind::Item(def_id));
                if callee_mir.arg_count != 1 {
                    return Err(format!(
                        "{EXPORT_SYMBOL} Rust instance projection requires a receiver-only method"
                    ));
                }
                let value = lower_constant_i32_return(tcx, callee_mir)?;
                let type_symbol = tcx.item_name(adt.did());
                let type_name = type_symbol.as_str().as_ref().to_owned();
                let method_symbol = tcx.item_name(def_id);
                let method_name = clr_method_name(method_symbol.as_str().as_ref());
                return Ok(Some(LoweredRustInstanceType {
                    namespace: CLR_NAMESPACE,
                    type_name,
                    method_name,
                    value,
                }));
            }
        }
    }
    Ok(None)
}

fn lower_constant_i32_return<'tcx>(
    tcx: TyCtxt<'tcx>,
    mir: &rustc_middle::mir::Body<'tcx>,
) -> Result<i32, String> {
    for block in mir.basic_blocks.iter() {
        for statement in &block.statements {
            let StatementKind::Assign(assignment) = &statement.kind else {
                continue;
            };
            let (place, rvalue) = assignment.as_ref();
            if place.local != RETURN_PLACE || !place.projection.is_empty() {
                continue;
            }
            let Rvalue::Use(operand) = rvalue else {
                continue;
            };
            return lower_i32_constant_operand(tcx, operand);
        }
    }
    Err("Rust instance method return is not a direct i32 MIR constant".to_owned())
}

fn lower_i32_constant_operand<'tcx>(
    tcx: TyCtxt<'tcx>,
    operand: &Operand<'tcx>,
) -> Result<i32, String> {
    let Operand::Constant(constant) = operand else {
        return Err(format!(
            "expected constant i32 MIR operand, found {operand:?}"
        ));
    };
    let evaluated = constant
        .const_
        .eval(tcx, TypingEnv::fully_monomorphized(), constant.span)
        .map_err(|_| "could not evaluate i32 constant from Rust instance MIR".to_owned())?;
    let ConstValue::Scalar(scalar) = evaluated else {
        return Err(format!(
            "Rust instance MIR constant is not a scalar: {evaluated:?}"
        ));
    };
    scalar
        .to_i32()
        .report_err()
        .map_err(|_| "Rust instance MIR scalar is not a valid i32".to_owned())
}

fn clr_method_name(rust_name: &str) -> String {
    let mut chars = rust_name.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    first.to_uppercase().chain(chars).collect()
}
