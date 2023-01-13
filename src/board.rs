use std::fmt::Display;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

trait Index{
    fn at_index(&self, index: usize) -> bool;
}

impl Index for u64{
    #[inline]
    fn at_index(&self, index: usize) -> bool {
        self >> index & 1 == 1
    }
}

#[derive(Debug, EnumIter, Clone, Copy)]
enum Player {
    Black,
    White,
}

#[derive(Debug, EnumIter, Clone, Copy)]
enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[allow(dead_code)]
struct PlayerPieces {
    pawns: u64,
    rooks: u64,
    knights: u64,
    bishops: u64,
    queens: u64,
    kings: u64,
}

impl PlayerPieces{
    pub fn new(player : Player) -> Self{
        match player{
            Player::Black => {
                let pawns   : u64 = 0b0000000011111111000000000000000000000000000000000000000000000000;
                let rooks   : u64 = 0b1000000100000000000000000000000000000000000000000000000000000000;
                let knights : u64 = 0b0100001000000000000000000000000000000000000000000000000000000000;
                let bishops : u64 = 0b0010010000000000000000000000000000000000000000000000000000000000;
                let queens  : u64 = 0b0000100000000000000000000000000000000000000000000000000000000000;
                let kings   : u64 = 0b0001000000000000000000000000000000000000000000000000000000000000;
                PlayerPieces { pawns, rooks, knights, bishops, queens, kings }
            },

            Player::White => {
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
}

impl PlayerPieces{
    fn get_piece(&self, piece: Piece) -> u64{
        match piece {
            Piece::Pawn => self.pawns,
            Piece::Rook => self.rooks,
            Piece::Knight => self.knights,
            Piece::Bishop => self.bishops,
            Piece::Queen => self.queens,
            Piece::King => self.kings,
        }
    }
}
pub struct Board {
    white_pieces: PlayerPieces,
    black_pieces: PlayerPieces,

    turn: Player,
    // ToDo: Keep track of en passant, castling, repeated moves
}

impl Display for Board{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pieces_vector : Vec<PotentialPiece> = Vec::with_capacity(64);
        for index in 0..64 {
            pieces_vector.push(self.get_piece_at_index(index).into())
        }
        pieces_vector
            .chunks(8)
            .for_each(|chunk|{
                chunk.iter()
                    .for_each(|potential_piece| write!(f, "{}", potential_piece).unwrap());
                writeln!(f).unwrap();
            });
        write!(f, "")
    }
}

struct PotentialPiece(Option<(Player, Piece)>);

impl From<Option<(Player, Piece)>> for PotentialPiece{
    fn from(value: Option<(Player, Piece)>) -> Self {
        Self(value)
    }
}

impl Display for PotentialPiece{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_none(){
            return write!(f, "{:^2}", "🩠");
        }
        let (player, piece) = self.0.unwrap();
        let symbol = match (player, piece){
            (Player::Black, Piece::Pawn) => "♟",
            (Player::Black, Piece::Rook) => "♜",
            (Player::Black, Piece::Knight) => "♞",
            (Player::Black, Piece::Bishop) => "♝",
            (Player::Black, Piece::Queen) => "♛",
            (Player::Black, Piece::King) => "♚",
            (Player::White, Piece::Pawn) => "♙",
            (Player::White, Piece::Rook) => "♖",
            (Player::White, Piece::Knight) => "♘",
            (Player::White, Piece::Bishop) => "♗",
            (Player::White, Piece::Queen) => "♕",
            (Player::White, Piece::King) => "♔",
        };
        
        write!(f, "{:^2}", symbol)
    }
}

impl Board {
    pub fn new() -> Self{
        Board { white_pieces: PlayerPieces::new(Player::White), black_pieces: PlayerPieces::new(Player::Black), turn: Player::White }
    }
    pub fn legal_moves(&self) -> Vec<Board> {
        let turn_pieces = self.get_turn_pieces();
        Piece::iter().for_each(|piece|{
            let bit_board = turn_pieces.get_piece(piece);
        });

        Vec::<Board>::new()
    }

    fn get_turn_pieces(&self) -> &PlayerPieces {
        match self.turn {
            Player::Black => &self.black_pieces,
            Player::White => &self.white_pieces,
        }
    }
    
    fn get_non_turn_pieces(&self) -> &PlayerPieces {
        match self.turn {
            Player::White => &self.black_pieces,
            Player::Black => &self.white_pieces,
        }
    }

    fn get_player_pieces(&self, player : Player) -> &PlayerPieces{
        match player{
            Player::Black => &self.black_pieces,
            Player::White => &self.white_pieces,
        }
    }

    fn get_piece_at_index(&self, index: usize) -> Option<(Player, Piece)>{
        let mut result : Option<(Player, Piece)> = None;

        Player::iter().for_each(|player|{
            let pieces = self.get_player_pieces(player);
            let piece = Piece::iter()
                .find(|piece| pieces.get_piece(*piece).at_index(index));
            if let Some(piece) = piece{
                result = Some((player, piece));
            }
        });

        result
    }
}
