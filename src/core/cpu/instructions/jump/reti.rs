use crate::core::cpu::instructions::jump::ret::RetInstruction;
use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::FetchedData;

#[derive(Debug, Clone, Copy)]
pub struct RetiInstruction {
    pub ret_instruction: RetInstruction,
}

impl Default for RetiInstruction {
    fn default() -> Self {
        Self::new()
    }
}

impl RetiInstruction {
    pub const fn new() -> RetiInstruction {
        Self {
            ret_instruction: RetInstruction {
                condition_type: None,
            },
        }
    }
}

impl ExecutableInstruction for RetiInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.bus.io.interrupts.ime = true;
        self.ret_instruction.execute(cpu, fetched_data);
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
