#[derive(Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn new(bitboard: u64) -> Bitboard {
        Bitboard(bitboard)
    }

    fn get_bit(self, square: u8) -> bool {
        self.0 & (1 << square) != 0
    }

    pub fn set_bit(mut self, square: u8) {
        self.0 |= 1 << square
    }

    /// This function changes the square position
    pub fn pop_bit(mut self, square: u8) {
        self.0 ^= 1 << square
    }
}
