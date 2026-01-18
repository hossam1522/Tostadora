use crate::{bitboard::Bitboard, piece::Color, piece::Piece};

pub struct Board {
    pieces: [Bitboard; 12],
    side_to_move: Color,
}

impl Board {
    fn new() -> Board {
        let pos_pieces = [
            Bitboard::new(0xff00),
            Bitboard::new(0x81),
            Bitboard::new(0x42),
            Bitboard::new(0x24),
            Bitboard::new(0x8),
            Bitboard::new(0x10),
            Bitboard::new(0xff000000000000),
            Bitboard::new(0x8100000000000000),
            Bitboard::new(0x4200000000000000),
            Bitboard::new(0x2400000000000000),
            Bitboard::new(0x1000000000000000),
            Bitboard::new(0x800000000000000),
        ];
        Board {
            pieces: pos_pieces,
            side_to_move: Color::White,
        }
    }

    fn set_piece(self, piece: Piece, square: u8) {
        self.pieces[piece.index()].set_bit(square);
    }

    fn remove_piece(self, piece: Piece, square: u8) {
        self.pieces[piece.index()].pop_bit(square);
    }
}
