mod player_pieces;
mod position;
mod potential_piece;
mod piece_type;
mod player;
mod index;

use std::fmt::Display;

use strum::IntoEnumIterator;

use self::{
    player_pieces::PlayerPieces, position::Position,
    potential_piece::PotentialPiece, piece_type::PieceType, player::Player, index::Index,
};


#[derive(Clone, Copy)]
pub struct Board {
    white_pieces: PlayerPieces,
    black_pieces: PlayerPieces,

    turn: Player,
    // ToDo: Keep track of en passant, castling, repeated moves
}

impl Board {
    pub fn new() -> Self {
        Board {
            white_pieces: PlayerPieces::new(Player::White),
            black_pieces: PlayerPieces::new(Player::Black),
            turn: Player::White,
        }
    }
    pub fn legal_moves(&self) -> Vec<Board> {
        let occupied_turn = self.get_turn_pieces().get_occupied_bitboard();
        let occupied_non_turn = self.get_non_turn_pieces().get_occupied_bitboard();
        let mut valid_moves = vec![];
        let occupied = occupied_turn | occupied_non_turn;

        let turn_pieces = self.get_turn_pieces();
        let direction = (self.turn == Player::White) as i8 * 2 - 1;
        let pawns = turn_pieces.pawns;
        for index in 0..64 {
            if pawns.get_position(index.into()) {
                let new_position = Position(index).relative_position(0, direction);
                if let Some(new_position) = new_position{
                    if !occupied.get_position(new_position) {
                        let new_board =
                            self.make_move(self.turn, PieceType::Pawn, index.into(), new_position);
                        valid_moves.push(new_board);
                    }
                }
                let new_position = Position(index).relative_position(direction, direction);
                if let Some(new_position) = new_position{
                    if occupied_non_turn.get_position(new_position) {
                        let new_board =
                            self.make_move(self.turn, PieceType::Pawn, index.into(), new_position);
                        valid_moves.push(new_board);
                    }
                }
                let new_position = Position(index).relative_position(-direction, direction);
                if let Some(new_position) = new_position{
                    if occupied_non_turn.get_position(new_position) {
                        let new_board =
                            self.make_move(self.turn, PieceType::Pawn, index.into(), new_position);
                        valid_moves.push(new_board);
                    }
                }
            }
        }
        valid_moves
    }

    #[inline]
    fn make_move(
        &self,
        player: Player,
        piece: PieceType,
        start_position: Position,
        end_position: Position,
    ) -> Board {
        let mut new_board = *self;
        let (player_pieces, opponent_pieces) = match player {
            Player::Black => (&mut new_board.black_pieces, &mut new_board.white_pieces),
            Player::White => (&mut new_board.white_pieces, &mut new_board.black_pieces),
        };
        let pieces = player_pieces.get_piece(piece);
        pieces.set_position(start_position, false);
        pieces.set_position(end_position, true);
        opponent_pieces.set_position(end_position, false);

        new_board.turn = match new_board.turn {
            Player::Black => Player::White,
            Player::White => Player::Black,
        };

        new_board
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

    fn get_player_pieces(&self, player: Player) -> &PlayerPieces {
        match player {
            Player::Black => &self.black_pieces,
            Player::White => &self.white_pieces,
        }
    }

    fn get_piece_at_index(&self, index: i8) -> Option<(Player, PieceType)> {
        let mut result: Option<(Player, PieceType)> = None;

        Player::iter().for_each(|player| {
            let mut pieces = *self.get_player_pieces(player);
            let piece = PieceType::iter().find(|piece| pieces.get_piece(*piece).get_position(Position(index)));
            if let Some(piece) = piece {
                result = Some((player, piece));
            }
        });

        result
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f).unwrap();
        let mut pieces_vector: Vec<PotentialPiece> = Vec::with_capacity(64);
        for index in 0..64 {
            pieces_vector.push(self.get_piece_at_index(index).into())
        }
        pieces_vector.chunks(8).for_each(|chunk| {
            chunk
                .iter()
                .for_each(|potential_piece| write!(f, "{}", potential_piece).unwrap());
            writeln!(f).unwrap();
        });
        writeln!(f)
    }
}

impl core::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

