#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    WP, // White Pawn
    WR, // White Rook
    WN, // White Knight
    WB, // White Bishop
    WQ, // White Queen
    WK, // White King
    BP, // Black Pawn
    BR, // Black Rook
    BN, // Black Knight
    BB, // Black Bishop
    BQ, // Black Queen
    BK, // Black King
    Empty,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Color {
    White,
    Black,
}

impl std::ops::Not for Color {
    type Output = Color;

    fn not(self) -> Self {
        unsafe { std::mem::transmute(self as u8 ^ 1) }
    }
}

impl Piece {
    pub fn color(self) -> Color {
        if (0..6).contains(&(self as u8)) {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn index(self) -> usize {
        self as usize
    }

    pub const fn get_piece(index: usize) -> Piece {
        match index {
            0 => Piece::WP,
            1 => Piece::WR,
            2 => Piece::WN,
            3 => Piece::WB,
            4 => Piece::WQ,
            5 => Piece::WK,
            6 => Piece::BP,
            7 => Piece::BR,
            8 => Piece::BN,
            9 => Piece::BB,
            10 => Piece::BQ,
            11 => Piece::BK,
            _ => Piece::Empty,
        }
    }
}
