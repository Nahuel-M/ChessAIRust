#[derive(Clone,Copy, Debug)]
pub struct Position(pub i8);

impl Position{
    pub fn relative_position(&self, x: i8, y: i8) -> Option<Position>{
        let new_position = self.0 + x + y * 8;
        if (0..64).contains(&new_position) && new_position / 8 == self.0 / 8 + y{
            Some(Position(new_position))
        } else{
            None
        }
    }

    pub fn x(&self) -> i8{
        self.0 % 8
    }

    pub fn y(&self) -> i8{
        self.0 / 8
    }
    
    pub fn all_positions_iter() -> impl Iterator<Item = Position>{
        (0..64).into_iter().map(Position)
    }
}

impl From<(i8, i8)> for Position{
    fn from(value: (i8, i8)) -> Self {
        Position(value.0 + value.1 * 8)
    }
}

impl From<i8> for Position{
    fn from(value: i8) -> Self {
        Position(value)
    }
}