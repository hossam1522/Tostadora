use crate::{
    bitboard::{self, Bitboard},
    moves::{Move, MoveType},
    piece::{Color, Piece},
};

pub struct Board {
    pieces: [Bitboard; 12],
    side_to_move: Color,
}

impl Board {
    pub fn new() -> Board {
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

    fn set_piece(&mut self, piece: Piece, square: u8) {
        self.pieces[piece.index()].set_bit(square);
    }

    fn remove_piece(&mut self, piece: Piece, square: u8) {
        self.pieces[piece.index()].pop_bit(square);
    }

    fn make_move(&mut self, piece: Piece, m: Move) {
        self.remove_piece(piece, m.src);
        if self.piece_at(m.dst) != Piece::Empty {
            self.remove_piece(self.piece_at(m.dst), m.dst);
        }
        self.set_piece(piece, m.dst);
        self.side_to_move = !self.side_to_move
    }

    fn piece_at(&self, square: u8) -> Piece {
        for (i, bitboard) in self.pieces.iter().enumerate() {
            if bitboard.get_bit(square) {
                return Piece::get_piece(i);
            }
        }
        Piece::Empty
    }
}
