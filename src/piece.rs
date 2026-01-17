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

pub enum Color {
    White,
    Black,
}

impl Piece {
    fn color(self) -> Color {
        if (0..6).contains(&(self as u8)) {
            Color::White
        } else {
            Color::Black
        }
    }
}
