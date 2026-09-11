use rustc_middle::{
    mir::{AggregateKind, ConstValue, Operand, RETURN_PLACE, Rvalue, StatementKind, mono::MonoItem},
    ty::{TyCtxt, TypingEnv},
};

const CLR_NAMESPACE: &str = "FerrumWeave";
const CLR_TYPE_NAME: &str = "RustApi";

pub(crate) struct LoweredOptionReferenceExports {
    pub(crate) namespace: &'static str,
    pub(crate) type_name: &'static str,
    pub(crate) some_method_name: String,
    pub(crate) none_method_name: String,
    pub(crate) some_value: String,
}

/// Recognizes a pair of exported Rust `Option<&str>` functions from MIR.
/// The Some payload is read from the MIR constant allocation; no Rust source
/// text, generated C#, milestone emitter, or expected result participates.
pub(crate) fn lower_option_reference_exports(
    tcx: TyCtxt<'_>,
) -> Result<Option<LoweredOptionReferenceExports>, String> {
    let mut some: Option<(String, String)> = None;
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
            let method_name = clr_method_name(symbol.as_str());
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
                    if tcx.item_name(adt.did()).as_str() != "Option" {
                        continue;
                    }

                    let variant_name = adt.variant(*variant_index).name.as_str();
                    match variant_name {
                        "Some" => {
                            if operands.len() != 1 {
                                return Err(format!(
                                    "Option Some export `{symbol}` must carry exactly one MIR operand"
                                ));
                            }
                            let value = lower_string_constant_operand(tcx, &operands[0])?;
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
            Ok(Some(LoweredOptionReferenceExports {
                namespace: CLR_NAMESPACE,
                type_name: CLR_TYPE_NAME,
                some_method_name,
                none_method_name,
                some_value,
            }))
        }
        (Some(_), None) => Err("Option reference lowering found Some but no None export".to_owned()),
        (None, Some(_)) => Err("Option reference lowering found None but no Some export".to_owned()),
    }
}

fn lower_string_constant_operand<'tcx>(
    tcx: TyCtxt<'tcx>,
    operand: &Operand<'tcx>,
) -> Result<String, String> {
    let Operand::Constant(constant) = operand else {
        return Err(format!(
            "Option Some payload must be a constant &str MIR operand, found {operand:?}"
        ));
    };
    let evaluated = constant
        .const_
        .eval(tcx, TypingEnv::fully_monomorphized(), constant.span)
        .map_err(|_| "could not evaluate Option Some &str constant from MIR".to_owned())?;
    let ConstValue::Slice { alloc_id, meta } = evaluated else {
        return Err(format!(
            "Option Some MIR constant is not a string slice: {evaluated:?}"
        ));
    };
    let length = usize::try_from(meta).map_err(|_| "Option Some string length does not fit usize")?;
    let allocation = tcx.global_alloc(alloc_id).unwrap_memory();
    let bytes = allocation
        .inner()
        .inspect_with_uninit_and_ptr_outside_interpreter(0..length);
    String::from_utf8(bytes.to_vec())
        .map_err(|_| "Option Some MIR constant is not valid UTF-8".to_owned())
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
