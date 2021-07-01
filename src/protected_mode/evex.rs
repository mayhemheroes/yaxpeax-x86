// use crate::long_mode::{OperandSpec, DecodeError, RegSpec, RegisterBank, Instruction, Opcode};
use crate::protected_mode::{DecodeError, RegSpec, RegisterBank, Instruction, Opcode};
use crate::protected_mode::{read_modrm, read_E_vex, read_imm_unsigned};

const DEFAULT_EVEX_REGISTER_SIZE: RegisterBank = RegisterBank::D;
const DEFAULT_EVEX_REGISTER_WIDTH: u8 = 4;

fn isa_has_qwords() -> bool {
    false
}

fn apply_disp_scale(inst: &mut Instruction) {
    inst.disp *= inst.mem_size as u32;
}

include!("../shared/generated_evex.in");
include!("../shared/evex.in");