pub mod bus;
pub mod cart;
pub mod cpu;
pub mod debugger;
pub mod emu;
pub mod hardware;
pub mod ppu;
pub mod ui;

pub struct LittleEndianBytes {
    pub low_byte: u8,
    pub high_byte: u8,
}

impl Into<u16> for LittleEndianBytes {
    fn into(self) -> u16 {
        let low_byte = self.low_byte as u16;
        let high_byte = self.high_byte as u16;

        low_byte | (high_byte << 8)
    }
}

/// Returns true if the n-th bit of byte is set, false otherwise.
pub fn get_bit_flag(byte: u8, pos: u8) -> bool {
    byte & (1 << pos) != 0
}

/// Sets or clears the n-th bit of `a` based on the value of `on`.
pub fn set_bit(a: &mut u8, n: u8, on: bool) {
    if on {
        *a |= 1 << n; // Set the n-th bit to 1
    } else {
        *a &= !(1 << n); // Set the n-th bit to 0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_get_bit_flag_1() {
        assert!(get_bit_flag(0b0001, 0));
        assert!(!get_bit_flag(0b0010, 0));

        assert!(get_bit_flag(0b0010, 1));
        assert!(!get_bit_flag(0b0001, 1));

        assert!(get_bit_flag(0b10000000, 7));
        assert!(!get_bit_flag(0b01000000, 7));

        assert!(get_bit_flag(0b10101010, 1));
        assert!(!get_bit_flag(0b10101010, 2));
    }

    #[test]
    fn test_set_bit_1() {
        let mut a = 0b1010; // 10 in decimal

        set_bit(&mut a, 2, true);
        assert_eq!(a, 0b1110);

        set_bit(&mut a, 2, false);
        assert_eq!(a, 0b1010);
    }
}
