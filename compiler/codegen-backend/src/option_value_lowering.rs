use rustc_middle::{
    mir::{
        AggregateKind, Body, ConstValue, Operand, RETURN_PLACE, Rvalue, StatementKind,
        mono::MonoItem,
    },
    ty::{TyCtxt, TypingEnv},
};

const CLR_NAMESPACE: &str = "FerrumWeave";
const CLR_TYPE_NAME: &str = "RustApi";

pub(crate) struct LoweredOptionValueExports {
    pub(crate) namespace: &'static str,
    pub(crate) type_name: &'static str,
    pub(crate) some_method_name: String,
    pub(crate) none_method_name: String,
    pub(crate) some_value: i32,
}

/// Recognizes a pair of exported Rust `Option<i32>` functions from MIR.
///
/// This lowering deliberately claims only scalar i32 payloads. A reference
/// payload is left for `option_reference_lowering`; no Rust source text,
/// generated C#, milestone emitter, or expected result participates.
pub(crate) fn lower_option_value_exports(
    tcx: TyCtxt<'_>,
) -> Result<Option<LoweredOptionValueExports>, String> {
    let mut some: Option<(String, i32)> = None;
    let mut none: Option<String> = None;
    let codegen_units = tcx.collect_and_partition_mono_items(());

    for cgu in codegen_units.codegen_units {
        for (item, _data) in cgu.items() {
            let MonoItem::Fn(instance) = *item else {
                continue;
            };
            let symbol = tcx.symbol_name(instance).name;
            if !symbol.starts_with("option_") {
                continue;
            }

            let mir = tcx.instance_mir(instance.def);
            let method_name = clr_method_name(symbol);
            for block in mir.basic_blocks.iter() {
                for statement in &block.statements {
                    let StatementKind::Assign(assignment) = &statement.kind else {
                        continue;
                    };
                    let (place, rvalue) = assignment.as_ref();
                    if place.local != RETURN_PLACE || !place.projection.is_empty() {
                        continue;
                    }
                    let Rvalue::Aggregate(kind, operands) = rvalue else {
                        continue;
                    };
                    let AggregateKind::Adt(adt, variant_index, ..) = kind.as_ref() else {
                        continue;
                    };
                    if tcx.item_name(*adt).as_str() != "Option" {
                        continue;
                    }

                    let adt_def = tcx.adt_def(*adt);
                    let variant_name = adt_def.variant(*variant_index).name.as_str();
                    match variant_name {
                        "Some" => {
                            if operands.len() != 1 {
                                return Err(format!(
                                    "Option Some export `{symbol}` must carry exactly one MIR operand"
                                ));
                            }
                            let operand = operands
                                .iter()
                                .next()
                                .expect("Option Some operand count was checked above");
                            let Some(value) = lower_i32_operand(tcx, mir, operand)? else {
                                return Ok(None);
                            };
                            some = Some((method_name.clone(), value));
                        }
                        "None" => {
                            if !operands.is_empty() {
                                return Err(format!(
                                    "Option None export `{symbol}` unexpectedly carries MIR operands"
                                ));
                            }
                            none = Some(method_name.clone());
                        }
                        other => {
                            return Err(format!(
                                "Option export `{symbol}` used unexpected variant `{other}`"
                            ));
                        }
                    }
                }
            }
        }
    }

    match (some, none) {
        (None, None) => Ok(None),
        (Some((some_method_name, some_value)), Some(none_method_name)) => {
            Ok(Some(LoweredOptionValueExports {
                namespace: CLR_NAMESPACE,
                type_name: CLR_TYPE_NAME,
                some_method_name,
                none_method_name,
                some_value,
            }))
        }
        (Some(_), None) => Err("Option value lowering found Some but no None export".to_owned()),
        (None, Some(_)) => Err("Option value lowering found None but no Some export".to_owned()),
    }
}

fn lower_i32_operand<'tcx>(
    tcx: TyCtxt<'tcx>,
    mir: &Body<'tcx>,
    operand: &Operand<'tcx>,
) -> Result<Option<i32>, String> {
    match operand {
        Operand::Constant(constant) => lower_i32_constant(tcx, constant),
        Operand::Copy(place) | Operand::Move(place) if place.projection.is_empty() => {
            let local = place.local;
            let mut source: Option<&Operand<'tcx>> = None;
            for block in mir.basic_blocks.iter() {
                for statement in &block.statements {
                    let StatementKind::Assign(assignment) = &statement.kind else {
                        continue;
                    };
                    let (assigned_place, rvalue) = assignment.as_ref();
                    if assigned_place.local != local || !assigned_place.projection.is_empty() {
                        continue;
                    }
                    let Rvalue::Use(candidate) = rvalue else {
                        continue;
                    };
                    if source.replace(candidate).is_some() {
                        return Err(format!(
                            "Option Some i32 payload local `{local:?}` has multiple MIR definitions"
                        ));
                    }
                }
            }
            let source = source.ok_or_else(|| {
                format!(
                    "Option Some i32 payload local `{local:?}` has no MIR Rvalue::Use definition"
                )
            })?;
            lower_i32_operand(tcx, mir, source)
        }
        _ => Err(format!(
            "Option Some i32 payload must resolve to a constant MIR operand, found {operand:?}"
        )),
    }
}

fn lower_i32_constant<'tcx>(
    tcx: TyCtxt<'tcx>,
    constant: &rustc_middle::mir::ConstOperand<'tcx>,
) -> Result<Option<i32>, String> {
    let evaluated = constant
        .const_
        .eval(tcx, TypingEnv::fully_monomorphized(), constant.span)
        .map_err(|_| "could not evaluate Option Some constant from MIR".to_owned())?;
    let ConstValue::Scalar(scalar) = evaluated else {
        return Ok(None);
    };
    let value = scalar
        .to_i32()
        .report_err()
        .map_err(|_| "Option Some MIR scalar is not a valid i32".to_owned())?;
    Ok(Some(value))
}

fn clr_method_name(rust_name: &str) -> String {
    rust_name
        .split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            let Some(first) = chars.next() else {
                return String::new();
            };
            first.to_uppercase().chain(chars).collect::<String>()
        })
        .collect()
}
