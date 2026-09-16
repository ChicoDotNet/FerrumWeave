use rustc_middle::{
    mir::{StatementKind, TerminatorKind, RETURN_PLACE},
    ty::{Instance, TyCtxt, TyKind},
};

/// R10's first executable contract deliberately accepts only an ordinary,
/// source-causal no-op Rust `fn main()`. Richer entry bodies must enter through
/// their own lowering contracts rather than being silently discarded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LoweredManagedEntryPoint;

pub(crate) fn lower_managed_entry_point(
    tcx: TyCtxt<'_>,
) -> Result<Option<LoweredManagedEntryPoint>, String> {
    let Some((main_def_id, _entry_type)) = tcx.entry_fn(()) else {
        return Ok(None);
    };

    let instance = Instance::mono(tcx, main_def_id);
    let mir = tcx.instance_mir(instance.def);

    if mir.arg_count != 0 {
        return Err("Rust main currently requires zero source parameters".to_owned());
    }
    if !matches!(mir.local_decls[RETURN_PLACE].ty.kind(), TyKind::Tuple(fields) if fields.is_empty()) {
        return Err(
            "Rust main currently requires the unit return type; Result/Termination mapping is a later contract"
                .to_owned(),
        );
    }
    if mir.basic_blocks.len() != 1 {
        return Err(
            "Rust main currently supports only a no-op body; control flow requires an explicit entry-body lowering contract"
                .to_owned(),
        );
    }

    let block = mir
        .basic_blocks
        .iter()
        .next()
        .expect("one basic block was required above");
    if block
        .statements
        .iter()
        .any(|statement| matches!(statement.kind, StatementKind::Assign(_)))
    {
        return Err(
            "Rust main currently supports only a no-op body; assignments require an explicit entry-body lowering contract"
                .to_owned(),
        );
    }
    if !matches!(block.terminator().kind, TerminatorKind::Return) {
        return Err(
            "Rust main currently supports only a no-op body ending in Return"
                .to_owned(),
        );
    }

    Ok(Some(LoweredManagedEntryPoint))
}