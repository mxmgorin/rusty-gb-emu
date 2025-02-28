use crate::cpu::instructions::FetchedData;
use crate::cpu::instructions::{AddressMode, ExecutableInstruction, Instruction};
use crate::cpu::{Cpu, CpuCallback};

#[derive(Debug, Clone, Copy)]
pub struct RstInstruction {
    pub address: u16,
}

impl ExecutableInstruction for RstInstruction {
    fn execute(&self, cpu: &mut Cpu, callback: &mut impl CpuCallback, _fetched_data: FetchedData) {
        Instruction::goto_addr(cpu, None, self.address, true, callback);
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
