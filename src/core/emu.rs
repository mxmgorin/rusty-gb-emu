use crate::auxiliary::clock::Clock;
use crate::bus::Bus;
use crate::core::cart::Cart;
use crate::core::cpu::Cpu;
use crate::core::ui::Ui;
use crate::debugger::{CpuLogType, Debugger};
use crate::ui::{UiEvent, UiEventHandler};
use std::borrow::Cow;
use std::path::Path;
use std::time::Duration;
use std::{fs, thread};

#[derive(Debug)]
pub struct Emu {
    running: bool,
    paused: bool,
    ctx: EmuCtx,
}

#[derive(Debug, Clone)]
pub struct EmuCtx {
    pub clock: Clock,
    pub debugger: Option<Debugger>,
}

impl Default for EmuCtx {
    fn default() -> Self {
        Self::new()
    }
}

impl EmuCtx {
    pub fn new() -> EmuCtx {
        Self {
            clock: Default::default(),
            debugger: None,
        }
    }

    pub fn with_debugger(debugger: Debugger) -> EmuCtx {
        Self {
            clock: Default::default(),
            debugger: Some(debugger),
        }
    }

    pub fn get_serial_msg(&self) -> Option<Cow<str>> {
        if let Some(debugger) = &self.debugger {
            return Some(debugger.get_serial_msg());
        };

        None
    }
}

impl UiEventHandler for Emu {
    fn on_event(&mut self, event: UiEvent) {
        match event {
            UiEvent::Quit => self.running = false,
        }
    }
}

impl Emu {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            running: false,
            paused: false,
            ctx: EmuCtx {
                clock: Default::default(),
                debugger: Some(Debugger::new(CpuLogType::None, false)),
            },
        })
    }

    pub fn run(&mut self, cart_bytes: Vec<u8>) -> Result<(), String> {
        let cart = Cart::new(cart_bytes)?;
        let mut cpu = Cpu::new(Bus::new(cart));
        let mut ui = Ui::new(true)?;
        let mut prev_frame = 0;
        let mut last_fps_timestamp = Duration::new(0, 0);
        self.running = true;

        while self.running {
            if self.paused {
                thread::sleep(Duration::from_millis(50));
                continue;
            }

            ui.handle_events(self);
            cpu.step(&mut self.ctx)?;

            if let Some(msg) = self.ctx.get_serial_msg() {
                if !msg.is_empty() {
                    println!("Serial: {msg}");
                }
            }

            if prev_frame != cpu.clock.ppu.current_frame {
                ui.draw(&cpu.clock.ppu, &cpu.bus);
            }

            if (cpu.clock.ppu.instant.elapsed() - last_fps_timestamp).as_millis() >= 1000 {
                println!("FPS: {}", cpu.clock.ppu.fps);
                last_fps_timestamp = cpu.clock.ppu.instant.elapsed();
            }

            prev_frame = cpu.clock.ppu.current_frame;
        }

        Ok(())
    }

    fn _print_cart(&self, cart: &Cart) {
        println!("Cart Loaded:");
        println!("\t Title    : {}", cart.header.title);
        println!("\t Type     : {:?}", cart.header.cart_type);
        println!("\t ROM Size : {:?}", cart.header.rom_size);
        println!("\t RAM Size : {:?}", cart.header.ram_size);
        println!("\t LIC Code : {:?} ", cart.header.new_licensee_code);
        println!("\t ROM Version : {:02X}", cart.header.mask_rom_version);
    }
}

pub fn read_bytes(file_path: &str) -> Result<Vec<u8>, String> {
    if !Path::new(file_path).exists() {
        return Err(format!("File not found: {}", file_path));
    }

    fs::read(file_path).map_err(|e| format!("Failed to read file: {}", e))
}
