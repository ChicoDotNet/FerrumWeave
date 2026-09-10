use std::collections::HashMap;

use ferrumweave_cil::{
    I32ArithmeticOp, I32ZeroPredicate, ManagedConstructor, ManagedInstanceReceiver, SystemMathMethod,
};
use rustc_middle::{
    mir::{
        BinOp, ConstValue, Operand, ProjectionElem, Rvalue, StatementKind, TerminatorKind,
        RETURN_PLACE, mono::MonoItem,
    },
    ty::{self, TyCtxt, TypingEnv},
};

const EXPORT_SYMBOL: &str = "answer";
const SYSTEM_MATH_ABS_MARKER: &str = "ferrumweave_system_math_abs";
const SYSTEM_MATH_SIGN_MARKER: &str = "ferrumweave_system_math_sign";
const SYSTEM_OBJECT_NEW_MARKER: &str = "ferrumweave_system_object_new";
const SYSTEM_TEXT_STRING_BUILDER_NEW_MARKER: &str =
    "ferrumweave_system_text_string_builder_new";
const SYSTEM_OBJECT_TO_STRING_MARKER: &str = "ferrumweave_system_object_to_string";
const SYSTEM_TEXT_STRING_BUILDER_TO_STRING_MARKER: &str =
    "ferrumweave_system_text_string_builder_to_string";
const SYSTEM_TEXT_STRING_BUILDER_LENGTH_MARKER: &str =
    "ferrumweave_system_text_string_builder_length";

pub(crate) enum LoweredI32Export {
    Constant(i32),
    Argument(u8),
    Arithmetic(I32ArithmeticOp),
    ControlFlow {
        predicate: I32ZeroPredicate,
        true_argument: u8,
        false_argument: u8,
    },
    DirectRustCall(I32ArithmeticOp),
    SystemMath {
        method: SystemMathMethod,
        argument: i32,
    },
    ManagedConstruction {
        constructor: ManagedConstructor,
        payload: i32,
    },
    ManagedInstance {
        receiver: ManagedInstanceReceiver,
        payload: i32,
    },
    ManagedStringBuilderLength {
        payload: i32,
    },
}

pub(crate) fn lower_exported_i32(tcx: TyCtxt<'_>) -> Result<LoweredI32Export, String> {
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
            let mut local_aliases = HashMap::new();
            let mut arithmetic_locals = HashMap::new();
            let mut comparison_locals = HashMap::new();

            for block in mir.basic_blocks.iter() {
                for statement in &block.statements {
                    let StatementKind::Assign(assignment) = &statement.kind else {
                        continue;
                    };
                    let (place, rvalue) = assignment.as_ref();
                    if place.projection.is_empty() {
                        match rvalue {
                            Rvalue::Use(Operand::Copy(source) | Operand::Move(source)) => {
                                if source.projection.is_empty() {
                                    local_aliases.insert(place.local, source.local);
                                } else if source.projection.len() == 1 {
                                    if let ProjectionElem::Field(field, _) = source.projection[0] {
                                        if field.index() == 0 {
                                            if let Some(operation) =
                                                arithmetic_locals.get(&source.local).copied()
                                            {
                                                arithmetic_locals.insert(place.local, operation);
                                            }
                                        }
                                    }
                                }
                            }
                            Rvalue::BinaryOp(operation, operands) => {
                                let (left, right) = operands.as_ref();
                                match operation {
                                    BinOp::Eq | BinOp::Ne => {
                                        let predicate =
                                            lower_zero_comparison(tcx, mir, *operation, left, right)?;
                                        comparison_locals.insert(place.local, predicate);
                                    }
                                    BinOp::Add | BinOp::AddWithOverflow => {
                                        require_binary_argument_order(mir, left, right)?;
                                        arithmetic_locals.insert(place.local, I32ArithmeticOp::Add);
                                    }
                                    BinOp::Sub | BinOp::SubWithOverflow => {
                                        require_binary_argument_order(mir, left, right)?;
                                        arithmetic_locals.insert(
                                            place.local,
                                            I32ArithmeticOp::Subtract,
                                        );
                                    }
                                    other => {
                                        return Err(format!(
                                            "unsupported i32 binary operation {other:?} in {EXPORT_SYMBOL}"
                                        ));
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    if place.local == RETURN_PLACE && place.projection.is_empty() {
                        if let Rvalue::Use(operand) = rvalue {
                            if let Ok(value) = lower_i32_constant_operand(tcx, operand) {
                                return Ok(LoweredI32Export::Constant(value));
                            }
                        }
                    }
                }

                if let TerminatorKind::SwitchInt { discr, targets } = &block.terminator().kind {
                    let (Operand::Copy(condition) | Operand::Move(condition)) = discr else {
                        return Err(format!(
                            "{EXPORT_SYMBOL} control-flow discriminator is not a MIR place: {discr:?}"
                        ));
                    };
                    if !condition.projection.is_empty() {
                        return Err(format!(
                            "{EXPORT_SYMBOL} control-flow discriminator projection is unsupported: {condition:?}"
                        ));
                    }

                    if mir.args_iter().position(|argument| argument == condition.local) == Some(0) {
                        let zero_target = targets
                            .iter()
                            .find_map(|(value, target)| (value == 0).then_some(target))
                            .ok_or_else(|| {
                                format!(
                                    "{EXPORT_SYMBOL} selector SwitchInt does not expose a zero target"
                                )
                            })?;
                        let nonzero_target = targets.otherwise();
                        return Ok(LoweredI32Export::ControlFlow {
                            predicate: I32ZeroPredicate::Equal,
                            true_argument: branch_result_argument(mir, zero_target)?,
                            false_argument: branch_result_argument(mir, nonzero_target)?,
                        });
                    }

                    if let Some(predicate) = comparison_locals.get(&condition.local).copied() {
                        let false_target = targets
                            .iter()
                            .find_map(|(value, target)| (value == 0).then_some(target))
                            .ok_or_else(|| {
                                format!(
                                    "{EXPORT_SYMBOL} boolean SwitchInt does not expose a false target"
                                )
                            })?;
                        let true_target = targets.otherwise();
                        return Ok(LoweredI32Export::ControlFlow {
                            predicate,
                            true_argument: branch_result_argument(mir, true_target)?,
                            false_argument: branch_result_argument(mir, false_target)?,
                        });
                    }
                }

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
                    return Err(format!(
                        "{EXPORT_SYMBOL} call target is not a direct Rust function: {func_ty:?}"
                    ));
                };
                let callee_symbol = tcx.item_name(def_id);
                let callee_name = callee_symbol.as_str();

                let managed_method = match callee_name.as_ref() {
                    SYSTEM_MATH_ABS_MARKER => Some(SystemMathMethod::Abs),
                    SYSTEM_MATH_SIGN_MARKER => Some(SystemMathMethod::Sign),
                    _ => None,
                };
                if let Some(method) = managed_method {
                    if args.len() != 1 {
                        return Err(format!(
                            "{EXPORT_SYMBOL} managed static marker requires exactly one i32 argument"
                        ));
                    }
                    let argument = lower_i32_constant_operand(tcx, &args[0].node)?;
                    return Ok(LoweredI32Export::SystemMath { method, argument });
                }

                let managed_constructor = match callee_name.as_ref() {
                    SYSTEM_OBJECT_NEW_MARKER => Some(ManagedConstructor::Object),
                    SYSTEM_TEXT_STRING_BUILDER_NEW_MARKER => Some(ManagedConstructor::StringBuilder),
                    _ => None,
                };
                if let Some(constructor) = managed_constructor {
                    if args.len() != 1 {
                        return Err(format!(
                            "{EXPORT_SYMBOL} managed construction marker requires exactly one i32 payload"
                        ));
                    }
                    let payload = lower_i32_constant_operand(tcx, &args[0].node)?;
                    return Ok(LoweredI32Export::ManagedConstruction {
                        constructor,
                        payload,
                    });
                }

                let managed_instance = match callee_name.as_ref() {
                    SYSTEM_OBJECT_TO_STRING_MARKER => Some(ManagedInstanceReceiver::Object),
                    SYSTEM_TEXT_STRING_BUILDER_TO_STRING_MARKER => {
                        Some(ManagedInstanceReceiver::StringBuilder)
                    }
                    _ => None,
                };
                if let Some(receiver) = managed_instance {
                    if args.len() != 1 {
                        return Err(format!(
                            "{EXPORT_SYMBOL} managed instance marker requires exactly one i32 payload"
                        ));
                    }
                    let payload = lower_i32_constant_operand(tcx, &args[0].node)?;
                    return Ok(LoweredI32Export::ManagedInstance { receiver, payload });
                }

                if callee_name.as_ref() == SYSTEM_TEXT_STRING_BUILDER_LENGTH_MARKER {
                    if args.len() != 1 {
                        return Err(format!(
                            "{EXPORT_SYMBOL} managed property marker requires exactly one i32 payload"
                        ));
                    }
                    let payload = lower_i32_constant_operand(tcx, &args[0].node)?;
                    return Ok(LoweredI32Export::ManagedStringBuilderLength { payload });
                }

                if !def_id.is_local() {
                    return Err(format!(
                        "unsupported non-local Rust call target `{callee_name}` in {EXPORT_SYMBOL}"
                    ));
                }
                if args.len() != 2 {
                    return Err(format!(
                        "{EXPORT_SYMBOL} direct Rust call currently requires exactly two i32 arguments"
                    ));
                }
                require_binary_argument_order(mir, &args[0].node, &args[1].node)?;
                let callee_mir = tcx.instance_mir(ty::InstanceKind::Item(def_id));
                let operation = lower_direct_i32_callee(callee_mir)?;
                return Ok(LoweredI32Export::DirectRustCall(operation));
            }

            if mir.arg_count == 2 {
                let mut local = RETURN_PLACE;
                for _ in 0..=mir.local_decls.len() {
                    if let Some(operation) = arithmetic_locals.get(&local) {
                        return Ok(LoweredI32Export::Arithmetic(*operation));
                    }
                    if let Some(index) = mir.args_iter().position(|argument| argument == local) {
                        return Ok(LoweredI32Export::Argument(
                            u8::try_from(index).expect("two i32 arguments fit u8"),
                        ));
                    }
                    let Some(next) = local_aliases.get(&local) else {
                        break;
                    };
                    local = *next;
                }
            }

            return Err(format!(
                "{EXPORT_SYMBOL} MIR contains neither a supported constant return, simple i32 argument flow, i32 arithmetic, i32 control flow, direct Rust call, managed static call, managed construction, managed instance call, nor managed property access"
            ));
        }
    }
    Err(format!(
        "no monomorphized `{EXPORT_SYMBOL}` export reached FerrumWeave codegen"
    ))
}

fn lower_direct_i32_callee<'tcx>(
    mir: &rustc_middle::mir::Body<'tcx>,
) -> Result<I32ArithmeticOp, String> {
    if mir.arg_count != 2 {
        return Err("direct Rust callee currently requires exactly two i32 arguments".to_owned());
    }

    let mut local_aliases = HashMap::new();
    let mut arithmetic_locals = HashMap::new();
    for block in mir.basic_blocks.iter() {
        for statement in &block.statements {
            let StatementKind::Assign(assignment) = &statement.kind else {
                continue;
            };
            let (place, rvalue) = assignment.as_ref();
            if !place.projection.is_empty() {
                continue;
            }
            match rvalue {
                Rvalue::Use(Operand::Copy(source) | Operand::Move(source)) => {
                    if source.projection.is_empty() {
                        local_aliases.insert(place.local, source.local);
                    } else if source.projection.len() == 1 {
                        if let ProjectionElem::Field(field, _) = source.projection[0] {
                            if field.index() == 0 {
                                if let Some(operation) = arithmetic_locals.get(&source.local).copied()
                                {
                                    arithmetic_locals.insert(place.local, operation);
                                }
                            }
                        }
                    }
                }
                Rvalue::BinaryOp(operation, operands) => {
                    let (left, right) = operands.as_ref();
                    require_binary_argument_order(mir, left, right)?;
                    let operation = match operation {
                        BinOp::Add | BinOp::AddWithOverflow => I32ArithmeticOp::Add,
                        BinOp::Sub | BinOp::SubWithOverflow => I32ArithmeticOp::Subtract,
                        other => {
                            return Err(format!(
                                "direct Rust callee uses unsupported i32 operation {other:?}"
                            ));
                        }
                    };
                    arithmetic_locals.insert(place.local, operation);
                }
                _ => {}
            }
        }
    }

    let mut local = RETURN_PLACE;
    for _ in 0..=mir.local_decls.len() {
        if let Some(operation) = arithmetic_locals.get(&local) {
            return Ok(*operation);
        }
        let Some(next) = local_aliases.get(&local) else {
            break;
        };
        local = *next;
    }

    Err("direct Rust callee return is not causally sourced from supported i32 arithmetic".to_owned())
}

fn lower_zero_comparison<'tcx>(
    tcx: TyCtxt<'tcx>,
    mir: &rustc_middle::mir::Body<'tcx>,
    operation: BinOp,
    left: &Operand<'tcx>,
    right: &Operand<'tcx>,
) -> Result<I32ZeroPredicate, String> {
    let predicate = match operation {
        BinOp::Eq => I32ZeroPredicate::Equal,
        BinOp::Ne => I32ZeroPredicate::NotEqual,
        _ => return Err(format!("expected Eq/Ne MIR comparison, found {operation:?}")),
    };

    let left_is_selector = direct_argument_index(mir, left) == Some(0);
    let right_is_selector = direct_argument_index(mir, right) == Some(0);
    let left_is_zero = lower_i32_constant_operand(tcx, left).ok() == Some(0);
    let right_is_zero = lower_i32_constant_operand(tcx, right).ok() == Some(0);

    if (left_is_selector && right_is_zero) || (right_is_selector && left_is_zero) {
        Ok(predicate)
    } else {
        Err(format!(
            "{EXPORT_SYMBOL} control-flow comparison currently requires selector argument against i32 zero"
        ))
    }
}

fn branch_result_argument<'tcx>(
    mir: &rustc_middle::mir::Body<'tcx>,
    mut block: rustc_middle::mir::BasicBlock,
) -> Result<u8, String> {
    let mut aliases = HashMap::new();
    for _ in 0..=mir.basic_blocks.len() {
        let data = &mir.basic_blocks[block];
        for statement in &data.statements {
            let StatementKind::Assign(assignment) = &statement.kind else {
                continue;
            };
            let (place, rvalue) = assignment.as_ref();
            if !place.projection.is_empty() {
                continue;
            }
            let Rvalue::Use(operand) = rvalue else {
                continue;
            };

            if let Some(index) = direct_argument_index(mir, operand) {
                let index = u8::try_from(index).map_err(|_| "argument index does not fit u8")?;
                aliases.insert(place.local, index);
                if place.local == RETURN_PLACE {
                    return Ok(index);
                }
                continue;
            }

            let (Operand::Copy(source) | Operand::Move(source)) = operand else {
                continue;
            };
            if source.projection.is_empty() {
                if let Some(index) = aliases.get(&source.local).copied() {
                    aliases.insert(place.local, index);
                    if place.local == RETURN_PLACE {
                        return Ok(index);
                    }
                }
            }
        }

        match &data.terminator().kind {
            TerminatorKind::Goto { target } => block = *target,
            TerminatorKind::Return => break,
            other => {
                return Err(format!(
                    "{EXPORT_SYMBOL} branch result encountered unsupported terminator {other:?}"
                ));
            }
        }
    }

    Err(format!(
        "{EXPORT_SYMBOL} branch does not resolve to a direct i32 argument"
    ))
}

fn require_binary_argument_order<'tcx>(
    mir: &rustc_middle::mir::Body<'tcx>,
    left: &Operand<'tcx>,
    right: &Operand<'tcx>,
) -> Result<(), String> {
    let left_index = argument_index(mir, left)?;
    let right_index = argument_index(mir, right)?;
    if left_index != 0 || right_index != 1 {
        return Err(format!(
            "{EXPORT_SYMBOL} arithmetic currently requires left/right argument order"
        ));
    }
    Ok(())
}

fn direct_argument_index<'tcx>(
    mir: &rustc_middle::mir::Body<'tcx>,
    operand: &Operand<'tcx>,
) -> Option<usize> {
    let (Operand::Copy(place) | Operand::Move(place)) = operand else {
        return None;
    };
    if !place.projection.is_empty() {
        return None;
    }
    mir.args_iter().position(|argument| argument == place.local)
}

fn argument_index<'tcx>(
    mir: &rustc_middle::mir::Body<'tcx>,
    operand: &Operand<'tcx>,
) -> Result<usize, String> {
    direct_argument_index(mir, operand)
        .ok_or_else(|| format!("expected direct i32 argument operand, found {operand:?}"))
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
        .map_err(|_| "could not evaluate i32 constant from MIR".to_owned())?;
    let ConstValue::Scalar(scalar) = evaluated else {
        return Err(format!("MIR constant is not a scalar: {evaluated:?}"));
    };
    scalar
        .to_i32()
        .report_err()
        .map_err(|_| "MIR scalar is not a valid i32".to_owned())
}
