use crate::{board::Board, pick_random_move};

pub struct Ai{

}

impl Ai{
    pub fn make_move(&self, board: &Board) -> Board{
        pick_random_move(
            &board.legal_moves()
        )
    }
}