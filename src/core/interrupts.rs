const INTERRUPTS_BY_ADDRESSES: [(u16, InterruptType); 5] = [
    (0x40, InterruptType::VBlank),
    (0x48, InterruptType::LCDStat),
    (0x50, InterruptType::Timer),
    (0x58, InterruptType::Serial),
    (0x60, InterruptType::Joypad),
];

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptType {
    VBlank = 1,
    LCDStat = 2,
    Timer = 4,
    Serial = 8,
    Joypad = 16,
}

#[derive(Debug, Clone)]
pub struct Interrupts {
    /// Interrupt flags
    pub int_flags: u8,
    pub cpu_halted: bool,
    /// Interrupt master enable
    pub ime: bool,
    /// Interrupt enable register
    pub ie_register: u8,
}

impl Default for Interrupts {
    fn default() -> Self {
        Self::new()
    }
}

impl Interrupts {
    pub fn new() -> Self {
        Self {
            int_flags: 0,
            cpu_halted: false,
            ime: false,
            ie_register: 0,
        }
    }

    pub fn get_interrupt(&mut self) -> Option<(u16, InterruptType)> {
        for (address, interrupt_type) in INTERRUPTS_BY_ADDRESSES {
            if self.check_interrupt(interrupt_type) {
                return Some((address, interrupt_type));
            }
        }

        None
    }

    pub fn request_interrupt(&mut self, it: InterruptType) {
        self.int_flags |= it as u8;
    }

    pub fn handle_interrupt(&mut self, it: InterruptType) {
        let it = it as u8;
        
        self.int_flags &= !it;
        self.cpu_halted = false;
        self.ime = false;
    }

    fn check_interrupt(&self, it: InterruptType) -> bool {
        let it = it as u8;
        let is_requested = self.int_flags & it != 0;
        let is_enabled = self.ie_register & it != 0;

        is_requested && is_enabled
    }
}
