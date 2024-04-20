pub trait IsColor {
    fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self;
    fn alpha(&self) -> u8;
    fn red(&self) -> u8;
    fn green(&self) -> u8;
    fn blue(&self) -> u8;
    fn mix(&self, rhs: Self) -> Self;
    fn with_alpha(&self, alpha: u8) -> Self;
}
