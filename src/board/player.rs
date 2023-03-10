use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Black,
    White,
}