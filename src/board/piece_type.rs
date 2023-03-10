use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}