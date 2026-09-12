use rustc_middle::{
    mir::{
        BinOp, ConstValue, Operand, ProjectionElem, RETURN_PLACE, Rvalue, StatementKind,
        TerminatorKind, mono::MonoItem,
    },
    ty::{TyCtxt, TypingEnv},
};

const EXPORT_SYMBOL: &str = "answer";
const CLR_NAMESPACE: &str = "FerrumWeave";

pub(crate) struct LoweredDisposableResource {
    pub(crate) namespace: &'static str,
    pub(crate) type_name: String,
    pub(crate) seed: i32,
    pub(crate) release_increment: i32,
}

/// Recognizes the currently certified Rust Drop/RAII shape from MIR rather than
/// from source text. The projection is claimed only when one monomorphized ADT
/// exposes a constructor seed, a field-backed release counter, and a guarded
/// `Drop::drop` body with an i32 increment.
pub(crate) fn lower_disposable_resource(
    tcx: TyCtxt<'_>,
) -> Result<Option<LoweredDisposableResource>, String> {
    let codegen_units = tcx.collect_and_partition_mono_items(());
    let mut saw_export = false;
    let mut resource_type: Option<String> = None;
    let mut seed: Option<i32> = None;
    let mut release_counter_type: Option<String> = None;
    let mut release_increment: Option<i32> = None;

    for cgu in codegen_units.codegen_units {
        for (item, _data) in cgu.items() {
            let MonoItem::Fn(instance) = *item else {
                continue;
            };
            if tcx.symbol_name(instance).name == EXPORT_SYMBOL {
                saw_export = true;
            }

            let def_id = instance.def_id();
            let item_name = tcx.item_name(def_id);
            let mir = tcx.instance_mir(instance.def);

            match item_name.as_str() {
                "new" => {
                    if let Some((type_name, value)) = lower_constructor_seed(tcx, mir)? {
                        resource_type = Some(type_name);
                        seed = Some(value);
                    }
                }
                "release_count" => {
                    if let Some(type_name) = lower_release_counter_reader(tcx, mir) {
                        release_counter_type = Some(type_name);
                    }
                }
                "drop" => {
                    if let Some(value) = lower_guarded_drop_increment(tcx, mir)? {
                        release_increment = Some(value);
                    }
                }
                _ => {}
            }
        }
    }

    if !saw_export {
        return Ok(None);
    }
    let (Some(type_name), Some(seed), Some(reader_type), Some(release_increment)) = (
        resource_type,
        seed,
        release_counter_type,
        release_increment,
    ) else {
        return Ok(None);
    };
    if type_name != reader_type {
        return Err(format!(
            "Drop resource constructor type `{type_name}` does not match release-counter receiver `{reader_type}`"
        ));
    }

    Ok(Some(LoweredDisposableResource {
        namespace: CLR_NAMESPACE,
        type_name,
        seed,
        release_increment,
    }))
}

fn lower_constructor_seed<'tcx>(
    tcx: TyCtxt<'tcx>,
    mir: &rustc_middle::mir::Body<'tcx>,
) -> Result<Option<(String, i32)>, String> {
    let return_ty = mir.local_decls[RETURN_PLACE].ty;
    let rustc_middle::ty::Adt(adt, _) = *return_ty.kind() else {
        return Ok(None);
    };

    for block in mir.basic_blocks.iter() {
        for statement in &block.statements {
            let StatementKind::Assign(assignment) = &statement.kind else {
                continue;
            };
            let (place, rvalue) = assignment.as_ref();
            if place.local != RETURN_PLACE || !place.projection.is_empty() {
                continue;
            }
            let Rvalue::Aggregate(_, fields) = rvalue else {
                continue;
            };
            for field in fields.iter() {
                if let Ok(value) = lower_i32_constant_operand(tcx, field) {
                    return Ok(Some((tcx.item_name(adt.did()).to_string(), value)));
                }
            }
        }
    }
    Ok(None)
}

fn lower_release_counter_reader(
    tcx: TyCtxt<'_>,
    mir: &rustc_middle::mir::Body<'_>,
) -> Option<String> {
    if mir.arg_count != 1 {
        return None;
    }
    let receiver_ty = mir.local_decls[mir.args_iter().next()?].ty;
    let rustc_middle::ty::Ref(_, referent, _) = *receiver_ty.kind() else {
        return None;
    };
    let rustc_middle::ty::Adt(adt, _) = *referent.kind() else {
        return None;
    };

    for block in mir.basic_blocks.iter() {
        for statement in &block.statements {
            let StatementKind::Assign(assignment) = &statement.kind else {
                continue;
            };
            let (place, rvalue) = assignment.as_ref();
            if place.local != RETURN_PLACE || !place.projection.is_empty() {
                continue;
            }
            let Rvalue::Use(Operand::Copy(source) | Operand::Move(source)) = rvalue else {
                continue;
            };
            if source.projection.iter().any(|element| {
                matches!(element, ProjectionElem::Field(field, _) if field.index() == 0)
            }) {
                return Some(tcx.item_name(adt.did()).to_string());
            }
        }
    }
    None
}

fn lower_guarded_drop_increment<'tcx>(
    tcx: TyCtxt<'tcx>,
    mir: &rustc_middle::mir::Body<'tcx>,
) -> Result<Option<i32>, String> {
    if mir.arg_count != 1
        || !mir
            .basic_blocks
            .iter()
            .any(|block| matches!(block.terminator().kind, TerminatorKind::SwitchInt { .. }))
    {
        return Ok(None);
    }

    for block in mir.basic_blocks.iter() {
        for statement in &block.statements {
            let StatementKind::Assign(assignment) = &statement.kind else {
                continue;
            };
            let (_, Rvalue::BinaryOp(BinOp::Add | BinOp::AddWithOverflow, operands)) =
                assignment.as_ref()
            else {
                continue;
            };
            let (left, right) = operands.as_ref();
            if let Ok(value) = lower_i32_constant_operand(tcx, left) {
                return Ok(Some(value));
            }
            if let Ok(value) = lower_i32_constant_operand(tcx, right) {
                return Ok(Some(value));
            }
        }
    }
    Ok(None)
}

fn lower_i32_constant_operand<'tcx>(
    tcx: TyCtxt<'tcx>,
    operand: &Operand<'tcx>,
) -> Result<i32, String> {
    let Operand::Constant(constant) = operand else {
        return Err(format!("expected constant i32 MIR operand, found {operand:?}"));
    };
    let evaluated = constant
        .const_
        .eval(tcx, TypingEnv::fully_monomorphized(), constant.span)
        .map_err(|_| "could not evaluate i32 constant from Drop resource MIR".to_owned())?;
    let ConstValue::Scalar(scalar) = evaluated else {
        return Err(format!(
            "Drop resource MIR constant is not a scalar: {evaluated:?}"
        ));
    };
    scalar
        .to_i32()
        .report_err()
        .map_err(|_| "Drop resource MIR scalar is not a valid i32".to_owned())
}
