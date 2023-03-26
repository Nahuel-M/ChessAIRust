#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerColor {
    Black,
    White,
}

pub const PLAYERS : [PlayerColor; 2] = [PlayerColor::Black, PlayerColor::White];