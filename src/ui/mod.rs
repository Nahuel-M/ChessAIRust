use eframe::{egui::Ui, epaint::{Vec2, Rect, Pos2, Rounding, Color32, Stroke, FontId}, emath::Align2};

pub fn add_empty_chessboard(ui : &mut Ui, board: Rect){
    let mut square_color = true;
    for x in 1..9{
        ui.horizontal(|ui| {
            add_number(ui, board, x);
            for y in 0..8{
                ui.painter().rect(
                    Rect::from_min_size(
                        Pos2::new(board.left() + x as f32 * board.width() / 9.,board.top() + y as f32 * board.width() / 9.), 
                        Vec2::new(board.width() / 9., board.height() / 9.)
                    ),
                    Rounding::none(),
                    Color32::from_gray(square_color as u8 * 200 + 55),
                    Stroke::NONE,
                );
                square_color = !square_color;
            }
            square_color = !square_color;
        });
    }
    add_letter_row(ui, board);
}

pub fn add_number(ui : &mut Ui, board : Rect, row: usize){
    ui.painter().text(
        Pos2::new(board.left() + board.width() / 18., board.top() + (row as f32 - 0.5) * board.height() / 9.), 
        Align2::CENTER_CENTER, 
        (9 - row).to_string(), 
        FontId::monospace(board.width() / 18.), 
        Color32::WHITE);
}

pub fn add_letter_row(ui : &mut Ui, board : Rect){
    for x in 1..9{
        ui.painter().text(
            Pos2::new(board.left() + (board.width() / 9.) * (x as f32 + 0.5), board.bottom() - board.height() / 18.), 
            Align2::CENTER_CENTER, 
            ((b'a' + x - 1) as char).to_string(), 
            FontId::monospace(board.width() / 18.), 
            Color32::WHITE);
    };
}

// pub fn add_checkerboard(ui : &mut Ui, size: Vec2) {
//     let mut square_color = true;
//     let (_id, board) = ui.allocate_space(size);
//     for x in 0..8{
//         ui.horizontal(|ui| {
//             for y in 0..8{
//                 ui.painter().rect(
//                     Rect::from_min_size(
//                         Pos2::new(board.left() + x as f32 * board.width() / 8.,board.top() + y as f32 * board.width() / 8.), 
//                         Vec2::new(board.width() / 8., board.height() / 8.)
//                     ),
//                     Rounding::none(),
//                     Color32::from_gray(square_color as u8 * 255),
//                     Stroke::NONE,
//                 );
//                 square_color = !square_color;
//             }
//             square_color = !square_color;
//         });
//     }
// }