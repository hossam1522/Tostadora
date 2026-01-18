pub struct Move {
    pub src: u8,
    pub dst: u8,
    pub move_type: MoveType,
}

impl Move {
    pub fn new(src: u8, dst: u8, move_type: MoveType) -> Move {
        Move {
            src,
            dst,
            move_type,
        }
    }
}

#[derive(PartialEq)]
pub enum MoveType {
    Quiet = 0b0000,
    DoublePawn = 0b0001,
    KingCastle = 0b0010,
    QueenCastle = 0b0011,
    Captures = 0b0100,
    EnPassant = 0b0101,
    KnightPromotion = 0b1000,
    BishopPromotion = 0b1001,
    RookPromotion = 0b1010,
    QueenPromotion = 0b1011,
    KnightPromoCapture = 0b1100,
    BishopPromoCapture = 0b1101,
    RookPromoCapture = 0b1110,
    QueenPromoCapture = 0b1111,
}
