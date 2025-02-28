use crate::cpu::instructions::FetchedData;
use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::{Cpu, CpuCallback};

/// Complement Carry Flag.
/// Cycles: 1
/// Bytes: 1
/// Flags:
/// N 0
/// H 0
/// C Inverted
#[derive(Debug, Clone, Copy)]
pub struct CcfInstruction;

impl ExecutableInstruction for CcfInstruction {
    fn execute(&self, cpu: &mut Cpu, _callback: &mut impl CpuCallback, _fetched_data: FetchedData) {
        cpu.registers.flags.set(
            None,
            Some(false),
            Some(false),
            Some(!cpu.registers.flags.get_c()),
        );
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
