use crate::board::Board;
use eframe::egui;

mod board;
mod ui;
fn main() -> Result<(), eframe::Error> {
    let mut board = Board::new();
    for _ in 0..60{
        let legal_moves = board.legal_moves();
        print!("{}", legal_moves.len());
        print!("{board}");
        board = pick_random_move(&legal_moves);
    }

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(420.0, 500.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Chess AI",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

fn pick_random_move(moves : &Vec<Board>) -> Board{
    moves[pseudo_random_usize(moves.len())]
}
fn pseudo_random_usize(max_val : usize) -> usize{
    static mut SEED : usize = 1043724;
    const MULTIPLIER : usize = 154307;
    const ADDITION : usize = 15314;

    unsafe {
        SEED = (SEED * MULTIPLIER + ADDITION) % max_val;
        SEED
    }
}
struct MyApp {
    board_history: Vec<Board>,
    selected_move : usize,
}

impl Default for MyApp{
    fn default() -> Self {
        Self { board_history: vec![Board::new()], selected_move: 0 }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Chess AI");
                ui.add(self.board_history[self.selected_move]);
                ui.add(egui::Slider::new(&mut self.selected_move, 0..=(self.board_history.len()-1)).text("Move"));
                if ui.button("Make random move").clicked() {
                    let new_move = pick_random_move(&self.board_history.last().unwrap().legal_moves());
                    self.board_history.push(new_move);
                    self.selected_move = self.board_history.len()-1;
                }
                // ui.label(format!("Hello '{}', age {}", self.name, self.age));
            });
           
        });
    }
}