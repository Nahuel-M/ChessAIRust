use crate::ai::Ai;

pub enum PlayerType{
    Human,
    Ai(Ai)
}