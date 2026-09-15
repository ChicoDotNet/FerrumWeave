use ferrumweave_projection_types::managed_method_name_from_export_symbol;
use rustc_middle::{
    mir::{ConstValue, Operand, Rvalue, StatementKind, RETURN_PLACE, mono::MonoItem},
    ty::{TyCtxt, TyKind, TypingEnv},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoweredConstantExport {
    pub(crate) method_name: String,
    pub(crate) value: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoweredCrateI32Export {
    Constant { method_name: String, value: i32 },
    Argument { method_name: String, index: u32 },
}

pub(crate) fn lower_heterogeneous_i32_exports(
    tcx: TyCtxt<'_>,
) -> Result<Option<Vec<LoweredCrateI32Export>>, String> {
    let codegen_units = tcx.collect_and_partition_mono_items(());
    let mut exports = Vec::new();
    let mut saw_constant = false;
    let mut saw_argument = false;

    for cgu in codegen_units.codegen_units {
        for (item, _data) in cgu.items() {
            let MonoItem::Fn(instance) = *item else { continue; };
            if !tcx.codegen_fn_attrs(instance.def_id()).contains_extern_indicator() { continue; }
            let mir = tcx.instance_mir(instance.def);
            if !matches!(mir.local_decls[RETURN_PLACE].ty.kind(), TyKind::Int(rustc_middle::ty::IntTy::I32)) { continue; }
            let method_name = managed_method_name_from_export_symbol(tcx.symbol_name(instance).name.as_ref());

            if mir.arg_count == 0 {
                if let Some(value) = direct_return_constant(tcx, mir)? {
                    exports.push(LoweredCrateI32Export::Constant { method_name, value });
                    saw_constant = true;
                }
            } else if mir.arg_count == 1
                && matches!(mir.local_decls[rustc_middle::mir::Local::from_usize(1)].ty.kind(), TyKind::Int(rustc_middle::ty::IntTy::I32))
                && direct_return_argument(mir) == Some(0)
            {
                exports.push(LoweredCrateI32Export::Argument { method_name, index: 0 });
                saw_argument = true;
            }
        }
    }

    if !saw_constant || !saw_argument { return Ok(None); }
    exports.sort_by(|left, right| crate_export_name(left).cmp(crate_export_name(right)));
    Ok(Some(exports))
}

fn crate_export_name(export: &LoweredCrateI32Export) -> &str {
    match export {
        LoweredCrateI32Export::Constant { method_name, .. } | LoweredCrateI32Export::Argument { method_name, .. } => method_name,
    }
}

fn direct_return_argument(mir: &rustc_middle::mir::Body<'_>) -> Option<u32> {
    for block in mir.basic_blocks.iter() {
        for statement in &block.statements {
            let StatementKind::Assign(assignment) = &statement.kind else { continue; };
            let (place, rvalue) = assignment.as_ref();
            if place.local != RETURN_PLACE || !place.projection.is_empty() { continue; }
            let Rvalue::Use(Operand::Copy(source) | Operand::Move(source)) = rvalue else { continue; };
            if source.projection.is_empty() {
                let local = source.local.as_usize();
                if local >= 1 && local <= mir.arg_count { return u32::try_from(local - 1).ok(); }
            }
        }
    }
    None
}

fn direct_return_constant(tcx: TyCtxt<'_>, mir: &rustc_middle::mir::Body<'_>) -> Result<Option<i32>, String> {
    for block in mir.basic_blocks.iter() {
        for statement in &block.statements {
            let StatementKind::Assign(assignment) = &statement.kind else { continue; };
            let (place, rvalue) = assignment.as_ref();
            if place.local != RETURN_PLACE || !place.projection.is_empty() { continue; }
            let Rvalue::Use(operand) = rvalue else { continue; };
            if matches!(operand, Operand::Constant(_)) { return lower_i32_constant_operand(tcx, operand).map(Some); }
        }
    }
    Ok(None)
}

/// Collects every externally visible zero-argument `i32` function whose MIR
/// return is a direct constant. Returning `None` means this crate is not a
/// multi-export constant slice and the existing single-export lowering should continue to own it.
pub(crate) fn lower_multiple_constant_exports(
    tcx: TyCtxt<'_>,
) -> Result<Option<Vec<LoweredConstantExport>>, String> {
    let codegen_units = tcx.collect_and_partition_mono_items(());
    let mut exports = Vec::new();
    for cgu in codegen_units.codegen_units {
        for (item, _data) in cgu.items() {
            let MonoItem::Fn(instance) = *item else { continue; };
            if !tcx.codegen_fn_attrs(instance.def_id()).contains_extern_indicator() { continue; }
            let mir = tcx.instance_mir(instance.def);
            if mir.arg_count != 0 || !matches!(mir.local_decls[RETURN_PLACE].ty.kind(), TyKind::Int(rustc_middle::ty::IntTy::I32)) { continue; }
            let Some(value) = direct_return_constant(tcx, mir)? else { continue; };
            let export_symbol = tcx.symbol_name(instance).name;
            exports.push(LoweredConstantExport { method_name: managed_method_name_from_export_symbol(export_symbol.as_ref()), value });
        }
    }
    if exports.len() < 2 { return Ok(None); }
    exports.sort_by(|left, right| left.method_name.cmp(&right.method_name));
    Ok(Some(exports))
}

fn lower_i32_constant_operand<'tcx>(tcx: TyCtxt<'tcx>, operand: &Operand<'tcx>) -> Result<i32, String> {
    let Operand::Constant(constant) = operand else { return Err(format!("expected constant i32 MIR operand, found {operand:?}")); };
    let evaluated = constant.const_.eval(tcx, TypingEnv::fully_monomorphized(), constant.span).map_err(|_| "could not evaluate i32 constant from MIR".to_owned())?;
    let ConstValue::Scalar(scalar) = evaluated else { return Err(format!("MIR constant is not a scalar: {evaluated:?}")); };
    scalar.to_i32().report_err().map_err(|_| "MIR scalar is not a valid i32".to_owned())
}
