use crate::board::Board;

mod board;
fn main() {
    let mut board = Board::new();
    for _ in 0..60{
        let legal_moves = board.legal_moves();
        board = pick_random_move(&legal_moves);
        print!("{}", legal_moves.len());
        print!("{board}");
    }
}

fn pick_random_move(moves : &Vec<Board>) -> Board{
    moves[pseudo_random_usize(moves.len())]
}
fn pseudo_random_usize(max_val : usize) -> usize{
    static SEED : usize = 1043724;
    const MULTIPLIER : usize = 154307;
    const ADDITION : usize = 15314;
    (SEED * MULTIPLIER + ADDITION) % max_val
}