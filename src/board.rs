mod player_pieces;
mod position;
mod potential_piece;
mod piece_type;
mod player;
mod index;
mod widget;

use std::fmt::Display;

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
        let occupied = occupied_turn | occupied_non_turn;

        let mut valid_moves = Vec::<Board>::with_capacity(32);

        let turn_pieces = self.get_turn_pieces();
        let pawn_direction = (self.turn == Player::White) as i8 * 2 - 1;

        for index in 0..64 {
            let position = Position(index);
            // PAWNS
            if turn_pieces.pawns.get_position(position) {
                self.add_pawn_moves(&mut valid_moves, position, pawn_direction, occupied, occupied_non_turn)
            }
            // KING
            else if turn_pieces.kings.get_position(position) {
                self.add_king_moves(&mut valid_moves, position, occupied, occupied_non_turn);
            }            
            // BISHOPS
            else if turn_pieces.bishops.get_position(position) {
                self.add_bishop_moves(&mut valid_moves, position, occupied, occupied_turn, occupied_non_turn)
            }
            
        }
        valid_moves
    }
    #[inline]
    fn add_pawn_moves(&self, valid_moves: &mut Vec<Board>, position : Position, direction : i8, occupied : u64, occupied_non_turn : u64){
        if let Some(new_position) = position.relative_position(0, direction){
            if !occupied.get_position(new_position) {
                let new_board =
                    self.make_move(self.turn, PieceType::Pawn, position, new_position);
                valid_moves.push(new_board);
            }
        }
        if let Some(new_position) = position.relative_position(direction, direction){
            if occupied_non_turn.get_position(new_position) {
                let new_board =
                    self.make_move(self.turn, PieceType::Pawn, position, new_position);
                valid_moves.push(new_board);
            }
        }
        if let Some(new_position) = position.relative_position(-direction, direction){
            if occupied_non_turn.get_position(new_position) {
                let new_board =
                    self.make_move(self.turn, PieceType::Pawn, position, new_position);
                valid_moves.push(new_board);
            }
        }
    }
    fn add_king_moves(&self, valid_moves: &mut Vec<Board>, position : Position, occupied : u64, occupied_non_turn : u64){
        let safe_positions : u64 = 0;
    }

    fn add_bishop_moves(&self, valid_moves: &mut Vec<Board>, position : Position, occupied : u64, occupied_turn: u64, occupied_non_turn : u64){
        static DIRECTIONS : [(i8, i8); 4] = [(1,1), (1,-1), (-1, 1), (-1,-1)];
        for direction in DIRECTIONS{
            let mut target_position = position;
            while let Some(temp_pos) = target_position.relative_position(direction.0, direction.1){
                target_position = temp_pos;
                if !occupied_turn.get_position(target_position){
                    let new_board =
                        self.make_move(self.turn, PieceType::Bishop, position, target_position);
                    valid_moves.push(new_board);
                }
            }
        }
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
        let pieces = player_pieces.get_piece_mut(piece);
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

    fn get_piece_at_position(&self, position: Position) -> PotentialPiece {
        let mut result: PotentialPiece = None.into();

        player::PLAYERS.into_iter().for_each(|player| {
            let mut pieces = *self.get_player_pieces(player);
            let piece = piece_type::PIECE_TYPES.into_iter().find(|piece| pieces.get_piece_mut(*piece).get_position(position));
            if let Some(piece) = piece {
                result = Some((player, piece)).into();
            }
        });

        result
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f).unwrap();
        let mut pieces_vector: Vec<PotentialPiece> = Vec::with_capacity(64);
        for position in Position::all_positions_iter() {
            pieces_vector.push(self.get_piece_at_position(position))
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

impl Default for Board{
    fn default() -> Board {
        Board { 
            white_pieces: PlayerPieces::new(Player::White), 
            black_pieces: PlayerPieces::new(Player::Black), 
            turn: Player::White }
    }
}
