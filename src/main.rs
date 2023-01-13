use crate::board::Board;

mod board;
fn main() {
    let board = Board::new();
    print!("{board}");
}
