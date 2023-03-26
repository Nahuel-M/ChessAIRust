use super::{player_color::PlayerColor, piece_type::PieceType, index::Index, position::Position};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct PlayerPieces {
    pub pawns: u64,
    pub rooks: u64,
    pub knights: u64,
    pub bishops: u64,
    pub queens: u64,
    pub kings: u64,
}

impl PlayerPieces{
    pub fn new(player : PlayerColor) -> Self{
        match player{
            PlayerColor::Black => {
                let pawns   : u64 = 0b0000000011111111000000000000000000000000000000000000000000000000;
                let rooks   : u64 = 0b1000000100000000000000000000000000000000000000000000000000000000;
                let knights : u64 = 0b0100001000000000000000000000000000000000000000000000000000000000;
                let bishops : u64 = 0b0010010000000000000000000000000000000000000000000000000000000000;
                let queens  : u64 = 0b0000100000000000000000000000000000000000000000000000000000000000;
                let kings   : u64 = 0b0001000000000000000000000000000000000000000000000000000000000000;
                PlayerPieces { pawns, rooks, knights, bishops, queens, kings }
            },

            PlayerColor::White => {
                let pawns   : u64 = 0b0000000000000000000000000000000000000000000000001111111100000000;
                let rooks   : u64 = 0b0000000000000000000000000000000000000000000000000000000010000001;
                let knights : u64 = 0b0000000000000000000000000000000000000000000000000000000001000010;
                let bishops : u64 = 0b0000000000000000000000000000000000000000000000000000000000100100;
                let queens  : u64 = 0b0000000000000000000000000000000000000000000000000000000000001000;
                let kings   : u64 = 0b0000000000000000000000000000000000000000000000000000000000010000;
                PlayerPieces { pawns, rooks, knights, bishops, queens, kings }
            },
        }
    }

    pub fn get_occupied_bitboard(&self) -> u64{
        self.pawns | self.rooks | self.kings | self.knights | self.bishops | self.queens
    }
    pub fn get_piece_mut(&mut self, piece: PieceType) -> &mut u64{
        match piece {
            PieceType::Pawn => &mut self.pawns,
            PieceType::Rook => &mut self.rooks,
            PieceType::Knight => &mut self.knights,
            PieceType::Bishop => &mut self.bishops,
            PieceType::Queen => &mut self.queens,
            PieceType::King => &mut self.kings,
        }
    }

    pub fn set_position(&mut self, position : Position, value: bool){
        self.pawns.set_position(position, value);
        self.rooks.set_position(position, value);
        self.knights.set_position(position, value);
        self.bishops.set_position(position, value);
        self.queens.set_position(position, value);
        self.kings.set_position(position, value);
    }
}