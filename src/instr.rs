use std::ffi::c_void;

use bochscpu::opcode::*;

#[allow(non_camel_case_types)]
pub type bochscpu_instr_t = *mut c_void;

#[no_mangle]
pub unsafe extern "C" fn bochscpu_instr_bx_opcode(p: bochscpu_instr_t) -> u32 {
    instr_bx_opcode(p)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_instr_imm16(p: bochscpu_instr_t) -> u16 {
    instr_imm16(p)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_instr_imm32(p: bochscpu_instr_t) -> u32 {
    instr_imm32(p)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_instr_imm64(p: bochscpu_instr_t) -> u64 {
    instr_imm64(p)
}
