use std::fmt::Display;

use super::{player_color::PlayerColor, piece_type::PieceType};


pub struct PotentialPiece(Option<(PlayerColor, PieceType)>);

impl From<Option<(PlayerColor, PieceType)>> for PotentialPiece{
    fn from(value: Option<(PlayerColor, PieceType)>) -> Self {
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
            (PlayerColor::Black, PieceType::Pawn)   => '♙',
            (PlayerColor::Black, PieceType::Rook)   => '♖',
            (PlayerColor::Black, PieceType::Knight) => '♘',
            (PlayerColor::Black, PieceType::Bishop) => '♗',
            (PlayerColor::Black, PieceType::Queen)  => '♕',
            (PlayerColor::Black, PieceType::King)   => '♔',
            (PlayerColor::White, PieceType::Pawn)   => '♟',
            (PlayerColor::White, PieceType::Rook)   => '♜',
            (PlayerColor::White, PieceType::Knight) => '♞',
            (PlayerColor::White, PieceType::Bishop) => '♝',
            (PlayerColor::White, PieceType::Queen)  => '♛',
            (PlayerColor::White, PieceType::King)   => '♚',
        }
    }
}