use yaxpeax_arch::{Arch, Decoder, LengthedInstruction, AddressBase};
use yaxpeax_x86::protected_mode;

#[no_mangle]
pub unsafe extern "C" fn yaxpeax_x86_32_decode_optimistic(data: *const u8, length: u64, inst: *mut protected_mode::Instruction) -> bool {
    let inst: &mut protected_mode::Instruction = core::mem::transmute(inst);
    <protected_mode::Arch as Arch>::Decoder::default().decode_into(inst, core::slice::from_raw_parts(data as *const u8, length as usize).iter().cloned()).is_err()
}

#[no_mangle]
pub unsafe extern "C" fn yaxpeax_x86_32_instr_length(inst: *mut protected_mode::Instruction) -> usize {
    let inst: &mut protected_mode::Instruction = core::mem::transmute(inst);
    0.wrapping_offset(inst.len()).to_linear()
}

#[cfg(fmt)]
mod write_sink;

#[cfg(fmt)]
mod fmt {
    use write_sink::InstructionSink;

    use core::fmt::Write;

    #[no_mangle]
    pub unsafe extern "C" fn yaxpeax_x86_32_fmt(inst: *mut protected_mode::Instruction, text: *mut u8, len: usize) {
        let inst: &mut protected_mode::Instruction = core::mem::transmute(inst);
        let res = core::slice::from_raw_parts_mut(text, len);

        write!(InstructionSink { buf: res, offs: 0 }, "{}", inst).unwrap();
    }
}

#[cfg(fmt)]
pub use fmt::yaxpeax_x86_32_fmt;