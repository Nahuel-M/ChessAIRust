use super::position::Position;

pub trait Index {
    fn get_position(&self, position: Position) -> bool;
    fn set_position(&mut self, position: Position, value: bool);
}

impl Index for u64 {
    #[inline]
    fn get_position(&self, position: Position) -> bool {
        self >> position.0 & 1 == 1
    }

    fn set_position(&mut self, position: Position, value: bool) {
        *self = (*self & (u64::MAX ^ (1 << position.0))) + ((value as u64) << position.0);
    }
}