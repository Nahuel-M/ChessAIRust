type BitBoard = u64;

trait BitBoardImplementation{
    fn get_pieces(&self) -> Vec<(u8, u8)>;
    fn set_pieces(&self, positions : Vec<(u8, u8)>);
}

impl BitBoardImplementation for BitBoard{
    fn get_pieces(&self) -> Vec<(u8, u8)> {
        (0u8..64)
            .filter(|i| (self >> i) & 1 == 1)
            .map(|number| (number / 8, number % 8))
            .collect()
    }

    fn set_pieces(&self, positions : Vec<(u8, u8)>) {
        todo!()
    }


}

