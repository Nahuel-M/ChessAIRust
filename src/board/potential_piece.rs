use std::fmt::Display;

use super::{player::Player, piece_type::PieceType};


pub struct PotentialPiece(Option<(Player, PieceType)>);

impl From<Option<(Player, PieceType)>> for PotentialPiece{
    fn from(value: Option<(Player, PieceType)>) -> Self {
        Self(value)
    }
}

impl Display for PotentialPiece{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_none(){
            return write!(f, "{:^2}", "▯");
        }
        let symbol = self.symbol();
        
        write!(f, "{:^2}", symbol)
    }
}

impl PotentialPiece{

    pub fn is_some(&self) -> bool{
        self.0.is_some()
    } 
    pub fn symbol(&self) -> char{
        if self.0.is_none(){
            return '▯'
        }

        let (player, piece) = self.0.unwrap();
        match (player, piece){
            (Player::Black, PieceType::Pawn)   => '♙',
            (Player::Black, PieceType::Rook)   => '♖',
            (Player::Black, PieceType::Knight) => '♘',
            (Player::Black, PieceType::Bishop) => '♗',
            (Player::Black, PieceType::Queen)  => '♕',
            (Player::Black, PieceType::King)   => '♔',
            (Player::White, PieceType::Pawn)   => '♟',
            (Player::White, PieceType::Rook)   => '♜',
            (Player::White, PieceType::Knight) => '♞',
            (Player::White, PieceType::Bishop) => '♝',
            (Player::White, PieceType::Queen)  => '♛',
            (Player::White, PieceType::King)   => '♚',
        }
    }
}