use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::{Cpu}; use crate::cpu::instructions::common::FetchedData;
use crate::cpu::stack::Stack;

#[derive(Debug, Clone, Copy)]
pub struct PushInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for PushInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        match self.address_mode {
            AddressMode::IMP | 
            AddressMode::R_D16(_) | 
            AddressMode::R_R(_, _) | 
            AddressMode::MR_R(_, _) | 
            AddressMode::R_D8(_) | 
            AddressMode::R_MR(_, _) | 
            AddressMode::R_HLI(_) | 
            AddressMode::R_HLD(_) | 
            AddressMode::HLI_R(_) | 
            AddressMode::HLD_R(_) | 
            AddressMode::R_A8(_) | 
            AddressMode::A8_R(_) | 
            AddressMode::HL_SPe8 | 
            AddressMode::D16 | 
            AddressMode::D8 | 
            AddressMode::D16_R(_) | 
            AddressMode::MR_D8(_) | 
            AddressMode::MR(_) | 
            AddressMode::A16_R(_) | 
            AddressMode::R_A16(_) => unreachable!("not used"),
            AddressMode::R(r1) => {
                let hi: u16 = (cpu.registers.read_register(r1) >> 8) & 0xFF;
                cpu.update_cycles(1);
                Stack::push(&mut cpu.registers, &mut cpu.bus, hi as u8);

                let lo: u16 = cpu.registers.read_register(r1) & 0xFF;
                cpu.update_cycles(1);
                Stack::push(&mut cpu.registers, &mut cpu.bus, lo as u8);
                
                cpu.update_cycles(1);
            }

        }
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
