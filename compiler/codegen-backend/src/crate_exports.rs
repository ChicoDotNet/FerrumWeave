use std::collections::HashMap;

use rustc_middle::{
    mir::{
        mono::MonoItem, BinOp, Local, Operand, Rvalue, StatementKind, RETURN_PLACE,
    },
    ty::{IntTy, TyCtxt, TyKind},
};

use crate::{managed_contract::managed_export_name, mir_lowering::lower_exported_i32};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrateI32Arithmetic {
    Add { left: usize, right: usize },
    Sub { left: usize, right: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoweredCrateI32Export {
    Constant { method_name: String, value: i32 },
    Argument { method_name: String, argument: usize },
    Arithmetic { method_name: String, operation: CrateI32Arithmetic },
}

pub fn lower_crate_i32_exports(tcx: TyCtxt<'_>) -> Result<Vec<LoweredCrateI32Export>, String> {
    let mono_items = tcx.collect_and_partition_mono_items(()).0;
    let mut exports = Vec::new();

    for mono_item in mono_items.iter() {
        let MonoItem::Fn(instance) = mono_item else { continue; };
        let def_id = instance.def_id();
        let Some(export_name) = managed_export_name(tcx, def_id) else { continue; };
        let signature = tcx.fn_sig(def_id).instantiate_identity().skip_binder();
        if !matches!(signature.output().kind(), TyKind::Int(IntTy::I32)) { continue; }
        if !signature.inputs().iter().all(|input| matches!(input.kind(), TyKind::Int(IntTy::I32))) {
            continue;
        }

        let mir = tcx.instance_mir(instance.def);
        if signature.inputs().is_empty() {
            if let Some(value) = direct_return_constant(tcx, mir)? {
                exports.push(LoweredCrateI32Export::Constant { method_name: export_name, value });
            }
            continue;
        }

        if signature.inputs().len() == 1 {
            if let Some(argument) = direct_return_argument(mir) {
                exports.push(LoweredCrateI32Export::Argument { method_name: export_name, argument });
            }
            continue;
        }

        if signature.inputs().len() == 2 {
            if let Some(operation) = direct_return_arithmetic(mir)? {
                exports.push(LoweredCrateI32Export::Arithmetic { method_name: export_name, operation });
            }
        }
    }

    Ok(exports)
}

pub fn lower_multiple_constant_exports(tcx: TyCtxt<'_>) -> Result<Vec<(String, i32)>, String> {
    let mono_items = tcx.collect_and_partition_mono_items(()).0;
    let mut exports = Vec::new();
    for mono_item in mono_items.iter() {
        let MonoItem::Fn(instance) = mono_item else { continue; };
        let def_id = instance.def_id();
        let Some(export_name) = managed_export_name(tcx, def_id) else { continue; };
        let signature = tcx.fn_sig(def_id).instantiate_identity().skip_binder();
        if !signature.inputs().is_empty() || !matches!(signature.output().kind(), TyKind::Int(IntTy::I32)) {
            continue;
        }
        if let Some(lowered) = lower_exported_i32(tcx)? {
            if lowered.method_name == export_name {
                exports.push((export_name, lowered.value));
            }
        }
    }
    Ok(exports)
}

fn direct_return_argument(mir: &rustc_middle::mir::Body<'_>) -> Option<usize> {
    let mut aliases = HashMap::<Local, Local>::new();
    for block in mir.basic_blocks.iter() {
        for statement in &block.statements {
            let StatementKind::Assign(assignment) = &statement.kind else { continue; };
            let (place, rvalue) = assignment.as_ref();
            if !place.projection.is_empty() { continue; }
            let Rvalue::Use(operand) = rvalue else { continue; };
            let Some(argument) = operand_place(operand) else { continue; };
            if !argument.projection.is_empty() { continue; }
            aliases.insert(place.local, argument.local);
        }
    }
    let mut local = RETURN_PLACE;
    for _ in 0..=mir.local_decls.len() {
        if let Some(index) = mir.args_iter().position(|argument| argument == local) { return Some(index); }
        let Some(next) = aliases.get(&local) else { break; };
        local = *next;
    }
    None
}

fn direct_return_arithmetic(mir: &rustc_middle::mir::Body<'_>) -> Result<Option<CrateI32Arithmetic>, String> {
    let mut aliases = HashMap::<Local, Local>::new();
    let mut arithmetic = HashMap::<Local, CrateI32Arithmetic>::new();
    for block in mir.basic_blocks.iter() {
        for statement in &block.statements {
            let StatementKind::Assign(assignment) = &statement.kind else { continue; };
            let (place, rvalue) = assignment.as_ref();
            if !place.projection.is_empty() { continue; }
            match rvalue {
                Rvalue::Use(operand) => {
                    let Some(source) = operand_place(operand) else { continue; };
                    if source.projection.is_empty() { aliases.insert(place.local, source.local); }
                }
                Rvalue::BinaryOp(operator, operands) | Rvalue::CheckedBinaryOp(operator, operands) => {
                    let Some(left) = direct_argument_index(mir, &operands.0) else { continue; };
                    let Some(right) = direct_argument_index(mir, &operands.1) else { continue; };
                    let operation = match operator {
                        BinOp::Add => CrateI32Arithmetic::Add { left, right },
                        BinOp::Sub => CrateI32Arithmetic::Sub { left, right },
                        _ => continue,
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
    let (Operand::Copy(place) | Operand::Move(place)) = operand else { return None; };
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
            let Operand::Constant(constant) = operand else { continue; };
            let scalar = constant.const_.eval(tcx, rustc_middle::ty::TypingEnv::fully_monomorphized()).map_err(|error| format!("failed to evaluate crate export constant: {error:?}"))?;
            if let rustc_middle::mir::ConstValue::Scalar(value) = scalar {
                return value.to_scalar_int().map(|integer| Some(integer.to_i32())).map_err(|error| format!("crate export constant is not i32: {error:?}"));
            }
        }
    }
    Ok(None)
}

fn operand_place<'tcx>(operand: &'tcx Operand<'tcx>) -> Option<&'tcx rustc_middle::mir::Place<'tcx>> {
    match operand {
        Operand::Copy(place) | Operand::Move(place) => Some(place),
        Operand::Constant(_) => None,
    }
}
