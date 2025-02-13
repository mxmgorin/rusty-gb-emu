pub mod events;
pub mod ui;
mod debug_window;

pub use ui::*;

use sdl2::pixels::Color;

pub fn into_sdl_color(color_hex: u32) -> Color {
    Color::RGBA(
        ((color_hex >> 24) & 0xFF) as u8,
        ((color_hex >> 16) & 0xFF) as u8,
        ((color_hex >> 8) & 0xFF) as u8,
        (color_hex & 0xFF) as u8,
    )
}
