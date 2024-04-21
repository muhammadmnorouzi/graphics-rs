use crate::math::matrix::Matrix;

pub type LetterData = [[u8; 4]; 7];

pub struct Letter {
    letter: char,
    data: Matrix<u8>
}