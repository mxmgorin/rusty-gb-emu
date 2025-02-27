use crate::apu::channel::ChannelType;
use crate::apu::length_timer::LengthTimer;
use crate::apu::registers::{NRX1, NRX2, NRX3X4};
use crate::apu::NR52;
use crate::get_bit_flag;

pub const CH1_START_ADDRESS: u16 = NR10_CH1_SWEEP_ADDRESS;
pub const CH1_END_ADDRESS: u16 = NR14_CH1_PERIOD_HIGH_CONTROL_ADDRESS;

pub const CH2_START_ADDRESS: u16 = NR21_CH2_LEN_TIMER_DUTY_CYCLE_ADDRESS;
pub const CH2_END_ADDRESS: u16 = NR24_CH2_PERIOD_HIGH_CONTROL_ADDRESS;

pub const NR10_CH1_SWEEP_ADDRESS: u16 = 0xFF10;
pub const NR11_CH1_LEN_TIMER_DUTY_CYCLE_ADDRESS: u16 = 0xFF11;
pub const NR12_CH1_VOL_ENVELOPE_ADDRESS: u16 = 0xFF12;
pub const NR13_CH1_PERIOD_LOW_ADDRESS: u16 = 0xFF13;
pub const NR14_CH1_PERIOD_HIGH_CONTROL_ADDRESS: u16 = 0xFF14;

pub const NR21_CH2_LEN_TIMER_DUTY_CYCLE_ADDRESS: u16 = 0xFF16;
pub const NR22_CH2_VOL_ENVELOPE_ADDRESS: u16 = 0xFF17;
pub const NR23_CH2_PERIOD_LOW_ADDRESS: u16 = 0xFF18;
pub const NR24_CH2_PERIOD_HIGH_CONTROL_ADDRESS: u16 = 0xFF19;

pub const WAVE_DUTY_PATTERNS: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 0],
];

#[derive(Debug, Clone)]
pub struct SquareChannel {
    // registers
    sweep: Option<NR10>,
    nrx1_len_timer_duty_cycle: NRX1,
    nrx2_volume_envelope: NRX2,
    nrx3x4_period_and_ctrl: NRX3X4,

    // other data
    length_timer: LengthTimer,
    period_timer: u16, // Internal timer for frequency stepping
    pattern_index: u8,
    wave_pattern: u8,

    output: u8,
}

impl SquareChannel {
    pub fn ch1() -> SquareChannel {
        Self {
            sweep: Some(Default::default()),
            nrx1_len_timer_duty_cycle: Default::default(),
            nrx2_volume_envelope: Default::default(),
            nrx3x4_period_and_ctrl: Default::default(),
            length_timer: LengthTimer::new(ChannelType::CH1),
            period_timer: 0,
            pattern_index: 0,
            wave_pattern: 0,
            output: 0,
        }
    }

    pub fn ch2() -> SquareChannel {
        Self {
            sweep: None,
            nrx1_len_timer_duty_cycle: Default::default(),
            nrx2_volume_envelope: Default::default(),
            nrx3x4_period_and_ctrl: Default::default(),
            length_timer: LengthTimer::new(ChannelType::CH2),
            period_timer: 0,
            pattern_index: 0,
            wave_pattern: 0,
            output: 0,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        // todo: handle for ch1
        match address {
            //NR10_CH1_SWEEP_ADDRESS => 0,
            NR21_CH2_LEN_TIMER_DUTY_CYCLE_ADDRESS => self.nrx1_len_timer_duty_cycle.byte,
            NR22_CH2_VOL_ENVELOPE_ADDRESS => self.nrx2_volume_envelope.byte,
            NR23_CH2_PERIOD_LOW_ADDRESS => 0xFF,
            NR24_CH2_PERIOD_HIGH_CONTROL_ADDRESS => {
                self.nrx3x4_period_and_ctrl.high_and_ctrl.read()
            }
            _ => panic!("Invalid Square address: {:#X}", address),
        }
    }

    pub fn tick_envelope(&mut self, _master_ctrl: &mut NR52) {
        // todo
    }

    pub fn tick_sweep(&mut self, _master_ctrl: &mut NR52) {
        // todo
    }

    pub fn tick_length(&mut self, master_ctrl: &mut NR52) {
        self.length_timer
            .tick(master_ctrl, &mut self.nrx3x4_period_and_ctrl.high_and_ctrl);
    }

    pub fn tick(&mut self, master_ctrl: &NR52) {
        if master_ctrl.is_ch_active(&self.length_timer.ch_type)
            && self.nrx2_volume_envelope.is_dac_enabled()
        {
            if self.period_timer > 0 {
                self.period_timer -= 1;
            }

            if self.period_timer == 0 {
                self.period_timer = (2048 - self.nrx3x4_period_and_ctrl.get_period()) * 4;
                // generate sample
                self.output = if WAVE_DUTY_PATTERNS[self.wave_pattern as usize]
                    [self.pattern_index as usize]
                    == 1
                {
                    self.nrx2_volume_envelope.initial_volume()
                } else {
                    0
                };

                self.period_timer += (2048 - self.nrx3x4_period_and_ctrl.get_period()) * 4;
                self.pattern_index = (self.pattern_index + 1) & 0x07;
            }
        }
    }
}

/// FF10 — NR10: Channel 1 sweep
/// This register controls CH1’s period sweep functionality.
#[derive(Debug, Clone, Default)]
pub struct NR10 {
    pub byte: u8,
}

impl NR10 {
    /// This dictates how often sweep “iterations” happen, in units of 128 Hz ticks5 (7.8 ms). Note
    /// that the value written to this field is not re-read by the hardware until a sweep iteration
    /// completes, or the channel is (re)triggered.
    /// However, if 0 is written to this field, then iterations are instantly disabled (but see below),
    /// and it will be reloaded as soon as it’s set to something else.
    pub fn pace(&self) -> u8 {
        self.byte & 0b0111_0000
    }

    /// 0 = Addition (period increases); 1 = Subtraction (period decreases)
    pub fn direction(&self) -> bool {
        get_bit_flag(self.byte, 3)
    }

    pub fn individual_step(&self) -> u8 {
        self.byte & 0b0000_0111
    }
}
