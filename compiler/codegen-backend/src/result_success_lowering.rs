use rustc_middle::{
    mir::{AggregateKind, Body, ConstValue, Operand, RETURN_PLACE, Rvalue, StatementKind, mono::MonoItem},
    ty::{TyCtxt, TypingEnv},
};

pub(crate) struct LoweredResultSuccess {
    pub(crate) method_name: String,
    pub(crate) value: i32,
}

/// Recognizes an exported `Result<i32, i32>::Ok` directly from monomorphized MIR.
pub(crate) fn lower_result_success(tcx: TyCtxt<'_>) -> Result<Option<LoweredResultSuccess>, String> {
    let codegen_units = tcx.collect_and_partition_mono_items(());
    for cgu in codegen_units.codegen_units {
        for (item, _data) in cgu.items() {
            let MonoItem::Fn(instance) = *item else { continue; };
            let symbol = tcx.symbol_name(instance).name;
            if !symbol.starts_with("result_") { continue; }
            let mir = tcx.instance_mir(instance.def);
            for block in mir.basic_blocks.iter() {
                for statement in &block.statements {
                    let StatementKind::Assign(assignment) = &statement.kind else { continue; };
                    let (place, rvalue) = assignment.as_ref();
                    if place.local != RETURN_PLACE || !place.projection.is_empty() { continue; }
                    let Rvalue::Aggregate(kind, operands) = rvalue else { continue; };
                    let AggregateKind::Adt(adt, variant_index, ..) = kind.as_ref() else { continue; };
                    if tcx.item_name(*adt).as_str() != "Result" { continue; }
                    let variant = tcx.adt_def(*adt).variant(*variant_index).name.as_str();
                    if variant != "Ok" { return Ok(None); }
                    if operands.len() != 1 {
                        return Err(format!("Result Ok export `{symbol}` must carry exactly one MIR operand"));
                    }
                    let operand = operands.iter().next().expect("Result Ok operand count checked");
                    let value = lower_i32_operand(tcx, mir, operand)?;
                    return Ok(Some(LoweredResultSuccess {
                        method_name: clr_method_name(symbol),
                        value,
                    }));
                }
            }
        }
    }
    Ok(None)
}

fn lower_i32_operand<'tcx>(tcx: TyCtxt<'tcx>, mir: &Body<'tcx>, operand: &Operand<'tcx>) -> Result<i32, String> {
    match operand {
        Operand::Constant(constant) => {
            let evaluated = constant.const_.eval(tcx, TypingEnv::fully_monomorphized(), constant.span)
                .map_err(|_| "could not evaluate Result Ok constant from MIR".to_owned())?;
            let ConstValue::Scalar(scalar) = evaluated else {
                return Err("Result Ok MIR constant is not scalar i32".to_owned());
            };
            scalar.to_i32().report_err().map_err(|_| "Result Ok MIR scalar is not i32".to_owned())
        }
        Operand::Copy(place) | Operand::Move(place) if place.projection.is_empty() => {
            let local = place.local;
            let mut source = None;
            for block in mir.basic_blocks.iter() {
                for statement in &block.statements {
                    let StatementKind::Assign(assignment) = &statement.kind else { continue; };
                    let (assigned, rvalue) = assignment.as_ref();
                    if assigned.local != local || !assigned.projection.is_empty() { continue; }
                    let Rvalue::Use(candidate) = rvalue else { continue; };
                    if source.replace(candidate).is_some() {
                        return Err(format!("Result Ok payload local `{local:?}` has multiple MIR definitions"));
                    }
                }
            }
            let source = source.ok_or_else(|| format!("Result Ok payload local `{local:?}` has no MIR Rvalue::Use definition"))?;
            lower_i32_operand(tcx, mir, source)
        }
        _ => Err(format!("Result Ok payload must resolve to constant i32 MIR operand, found {operand:?}")),
    }
}

fn clr_method_name(rust_name: &str) -> String {
    rust_name.split('_').filter(|part| !part.is_empty()).map(|part| {
        let mut chars = part.chars();
        let Some(first) = chars.next() else { return String::new(); };
        first.to_uppercase().chain(chars).collect::<String>()
    }).collect()
}
