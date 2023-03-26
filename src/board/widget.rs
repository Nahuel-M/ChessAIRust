use eframe::{egui::{Widget, Response, Ui, Sense}, epaint::{Vec2, Rect, Pos2, FontId, Color32}, emath::Align2};

use crate::ui::add_empty_chessboard;

use super::{Board, position::Position};

impl Widget for Board{
    fn ui(self, ui: &mut Ui) -> Response {
        let size = Vec2::new(400., 400.);
        let (rect, response) = ui.allocate_exact_size(size, Sense::focusable_noninteractive());
        add_empty_chessboard(ui, rect);
        self.add_pieces(ui, rect);
        response
    }
}

impl Board{
    fn add_pieces(&self, ui: &mut Ui, board: Rect){
        for position in Position::all_positions_iter(){
            let potential_piece = self.get_piece_at_position(position);
            if potential_piece.is_some(){
                let symbol = potential_piece.symbol();
                ui.painter().text(
                    Pos2::new(
                        board.left() + (board.width() / 9.) * (position.x() as f32 + 1.5), 
                        board.top() + (board.height() / 9.) * (position.y() as f32 + 0.5), 
                    ),
                    Align2::CENTER_CENTER, 
                    symbol, 
                    FontId::monospace(board.width() / 10.), 
                    Color32::BLACK);
            }
        }
    }
}
