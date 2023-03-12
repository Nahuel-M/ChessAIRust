#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Black,
    White,
}

pub const PLAYERS : [Player; 2] = [Player::Black, Player::White];