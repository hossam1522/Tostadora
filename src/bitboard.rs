pub struct Bitboard(u64);

impl Bitboard {
    pub fn new(bitboard: u64) -> Bitboard {
        Bitboard(bitboard)
    }

    fn get_bit(self, square: u8) -> bool {
        self.0 & (1 << square) != 0
    }

    fn set_bit(mut self, square: u8) {
        self.0 |= 1 << square
    }
}
