use std::collections::HashMap;

use ferrumweave_cil::I32ArithmeticOp;
use ferrumweave_projection_types::managed_method_name_from_export_symbol;
use rustc_middle::{
    mir::{BinOp, ConstValue, Operand, ProjectionElem, Rvalue, StatementKind, RETURN_PLACE, mono::MonoItem},
    ty::{TyCtxt, TyKind, TypingEnv},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoweredConstantExport { pub(crate) method_name: String, pub(crate) value: i32 }

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoweredCrateI32Export {
    Constant { method_name: String, value: i32 },
    Argument { method_name: String, index: u32 },
    Arithmetic { method_name: String, operation: I32ArithmeticOp },
}

pub(crate) fn lower_heterogeneous_i32_exports(tcx: TyCtxt<'_>) -> Result<Option<Vec<LoweredCrateI32Export>>, String> {
    let codegen_units = tcx.collect_and_partition_mono_items(());
    let mut exports = Vec::new();
    let mut shape_count = 0_u8;
    for cgu in codegen_units.codegen_units {
        for (item, _data) in cgu.items() {
            let MonoItem::Fn(instance) = *item else { continue; };
            if !tcx.codegen_fn_attrs(instance.def_id()).contains_extern_indicator() { continue; }
            let mir = tcx.instance_mir(instance.def);
            if !matches!(mir.local_decls[RETURN_PLACE].ty.kind(), TyKind::Int(rustc_middle::ty::IntTy::I32)) { continue; }
            let method_name = managed_method_name_from_export_symbol(tcx.symbol_name(instance).name.as_ref());
            let lowered = if mir.arg_count == 0 {
                direct_return_constant(tcx, mir)?.map(|value| LoweredCrateI32Export::Constant { method_name, value })
            } else if mir.arg_count == 1 && direct_return_argument(mir) == Some(0) {
                Some(LoweredCrateI32Export::Argument { method_name, index: 0 })
            } else if mir.arg_count == 2 {
                direct_return_arithmetic(mir)?.map(|operation| LoweredCrateI32Export::Arithmetic { method_name, operation })
            } else {
                None
            };
            let Some(lowered) = lowered else { continue; };
            let bit = match lowered {
                LoweredCrateI32Export::Constant { .. } => 1,
                LoweredCrateI32Export::Argument { .. } => 2,
                LoweredCrateI32Export::Arithmetic { .. } => 4,
            };
            shape_count |= bit;
            exports.push(lowered);
        }
    }
    if exports.len() < 2 || shape_count.count_ones() < 2 { return Ok(None); }
    exports.sort_by(|left, right| crate_export_name(left).cmp(crate_export_name(right)));
    Ok(Some(exports))
}

fn crate_export_name(export: &LoweredCrateI32Export) -> &str {
    match export {
        LoweredCrateI32Export::Constant { method_name, .. }
        | LoweredCrateI32Export::Argument { method_name, .. }
        | LoweredCrateI32Export::Arithmetic { method_name, .. } => method_name,
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

fn direct_return_arithmetic(mir: &rustc_middle::mir::Body<'_>) -> Result<Option<I32ArithmeticOp>, String> {
    let mut aliases = HashMap::new();
    let mut arithmetic = HashMap::new();
    for block in mir.basic_blocks.iter() {
        for statement in &block.statements {
            let StatementKind::Assign(assignment) = &statement.kind else { continue; };
            let (place, rvalue) = assignment.as_ref();
            if !place.projection.is_empty() { continue; }
            match rvalue {
                Rvalue::Use(Operand::Copy(source) | Operand::Move(source)) if source.projection.is_empty() => {
                    aliases.insert(place.local, source.local);
                }
                Rvalue::Use(Operand::Copy(source) | Operand::Move(source)) if source.projection.len() == 1 => {
                    if let ProjectionElem::Field(field, _) = source.projection[0] {
                        if field.index() == 0 {
                            if let Some(operation) = arithmetic.get(&source.local).copied() {
                                arithmetic.insert(place.local, operation);
                            }
                        }
                    }
                }
                Rvalue::BinaryOp(operation, operands) => {
                    let (left, right) = operands.as_ref();
                    if direct_argument_index(mir, left) != Some(0) || direct_argument_index(mir, right) != Some(1) {
                        continue;
                    }
                    let operation = match operation {
                        BinOp::Add | BinOp::AddWithOverflow => I32ArithmeticOp::Add,
                        BinOp::Sub | BinOp::SubWithOverflow => I32ArithmeticOp::Subtract,
                        other => return Err(format!("unsupported crate-level i32 arithmetic operation {other:?}")),
                    };
                    arithmetic.insert(place.local, operation);
                }
                _ => {}
            }
        }
    }
    let mut local = RETURN_PLACE;
    for _ in 0..=mir.local_decls.len() {
        if let Some(operation) = arithmetic.get(&local) { return Ok(Some(*operation)); }
        let Some(next) = aliases.get(&local) else { break; };
        local = *next;
    }
    Ok(None)
}

fn direct_argument_index(mir: &rustc_middle::mir::Body<'_>, operand: &Operand<'_>) -> Option<usize> {
    let Operand::Copy(place) | Operand::Move(place) = operand else { return None; };
    if !place.projection.is_empty() { return None; }
    mir.args_iter().position(|argument| argument == place.local)
}

fn direct_return_constant<'tcx>(tcx: TyCtxt<'tcx>, mir: &rustc_middle::mir::Body<'tcx>) -> Result<Option<i32>, String> {
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

pub(crate) fn lower_multiple_constant_exports(tcx: TyCtxt<'_>) -> Result<Option<Vec<LoweredConstantExport>>, String> {
    let codegen_units = tcx.collect_and_partition_mono_items(());
    let mut exports = Vec::new();
    for cgu in codegen_units.codegen_units {
        for (item, _data) in cgu.items() {
            let MonoItem::Fn(instance) = *item else { continue; };
            if !tcx.codegen_fn_attrs(instance.def_id()).contains_extern_indicator() { continue; }
            let mir = tcx.instance_mir(instance.def);
            if mir.arg_count != 0 || !matches!(mir.local_decls[RETURN_PLACE].ty.kind(), TyKind::Int(rustc_middle::ty::IntTy::I32)) { continue; }
            let Some(value) = direct_return_constant(tcx, mir)? else { continue; };
            exports.push(LoweredConstantExport { method_name: managed_method_name_from_export_symbol(tcx.symbol_name(instance).name.as_ref()), value });
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
