#![forbid(unsafe_code)]

//! Managed export emission for FerrumWeave MIR i32 arithmetic.
//!
//! The rustc-facing lowering layer selects the arithmetic operation from MIR.
//! This emitter translates that lowered operation into CIL; it does not inspect
//! Rust source or know the certifier's input values.

use crate::emit_i32_argument_export_assembly;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I32ArithmeticOp {
    Add,
    Subtract,
}

/// Emit `public static int Answer(int left, int right)` performing the selected
/// arithmetic operation. Metadata/signature are shared with the argument export;
/// only the method body is widened from `ldarg; ret` to two loads + op + ret.
#[must_use]
pub fn emit_i32_arithmetic_export_assembly(operation: I32ArithmeticOp) -> Vec<u8> {
    let mut image = emit_i32_argument_export_assembly(0);

    const HEADERS_SIZE: usize = 0x200;
    const CLR_HEADER_SIZE: usize = 0x48;
    const OLD_METADATA_OFFSET: usize = 0x4c;
    const NEW_METADATA_OFFSET: usize = 0x50;
    const SECTION_RVA: u32 = 0x2000;
    const PE_OFFSET: usize = 0x80;
    const OPTIONAL_HEADER_SIZE: usize = 0xE0;

    let clr = HEADERS_SIZE;
    let metadata_size = u32::from_le_bytes(image[clr + 0x0c..clr + 0x10].try_into().unwrap()) as usize;
    let old_start = HEADERS_SIZE + OLD_METADATA_OFFSET;
    let new_start = HEADERS_SIZE + NEW_METADATA_OFFSET;
    image.copy_within(old_start..old_start + metadata_size, new_start);

    let opcode = match operation {
        I32ArithmeticOp::Add => 0x58,
        I32ArithmeticOp::Subtract => 0x59,
    };
    let body = [0x12, 0x02, 0x03, opcode, 0x2a];
    let method_start = HEADERS_SIZE + CLR_HEADER_SIZE;
    image[method_start..method_start + body.len()].copy_from_slice(&body);
    image[method_start + body.len()..new_start].fill(0);

    image[clr + 0x08..clr + 0x0c]
        .copy_from_slice(&(SECTION_RVA + NEW_METADATA_OFFSET as u32).to_le_bytes());

    let section_header = PE_OFFSET + 4 + 20 + OPTIONAL_HEADER_SIZE;
    let old_virtual_size = u32::from_le_bytes(
        image[section_header + 8..section_header + 12]
            .try_into()
            .unwrap(),
    );
    image[section_header + 8..section_header + 12]
        .copy_from_slice(&(old_virtual_size + 4).to_le_bytes());

    image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic_export_is_operation_sensitive() {
        let add = emit_i32_arithmetic_export_assembly(I32ArithmeticOp::Add);
        let subtract = emit_i32_arithmetic_export_assembly(I32ArithmeticOp::Subtract);
        assert_ne!(add, subtract);
    }
}
