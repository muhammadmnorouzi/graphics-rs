use std::fmt::Display;

#[derive(Clone)]
pub struct Matrix<T: 'static> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Default for Matrix<T> {
    fn default() -> Self {
        Self {
            rows: Default::default(),
            cols: Default::default(),
            data: Default::default(),
        }
    }
}

impl<T> Matrix<T>
where
    T: Clone
        + Default
        + Display
        + std::ops::Mul<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::AddAssign
        + std::ops::MulAssign
        + Copy,
{
    pub fn create(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }

    pub fn at(&self, row: usize, col: usize) -> T {
        assert!(row < self.rows);
        assert!(col < self.cols);

        self.data[self.cols * row + col]
    }

    pub fn set_at(&mut self, row: usize, col: usize, value: T) {
        self.data[self.cols * row + col] = value;
    }

    pub fn modify_by_at(&mut self, row: usize, col: usize, value: T) {
        self.data[self.cols * row + col] += value;
    }

    pub fn multiply_by_at(&mut self, row: usize, col: usize, value: T) {
        self.set_at(row, col, self.at(row, col) * value);
    }

    pub fn print(&self, name: &str, padding: Option<usize>) {
        let padding: usize = padding.unwrap_or(0);
        let padding: String = " ".repeat(padding);

        println!("{}{} : [", padding, name);
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!("{}\t{}", padding, self.at(i, j));
            }

            print!("\n");
        }
        println!("{}]", padding);
    }

    pub fn sum(&mut self, mat: &Matrix<T>) {
        assert!(self.rows == mat.rows);
        assert!(self.cols == mat.cols);

        for i in 0..self.data.len() {
            self.data[i] += mat.data[i];
        }
    }

    pub fn fill(&mut self, value: T) {
        for i in 0..self.data.len() {
            self.data[i] = value;
        }
    }

    pub fn multiply(&self, rhs: &Matrix<T>) -> Matrix<T> {
        assert!(self.cols == rhs.rows);
        let mut result = Self::create(self.rows, rhs.cols);

        for i in 0..result.rows {
            for j in 0..result.cols {
                let mut value: T = Default::default();

                for k in 0..self.cols {
                    value += self.at(i, k) * rhs.at(k, j);
                }

                result.set_at(i, j, value);
            }
        }

        result
    }

    pub fn row(&self, row_buffer: &mut Matrix<T>, row_index: usize) {
        assert!(self.rows > row_index);
        assert!(row_buffer.rows == 1);
        assert!(row_buffer.cols == self.cols);

        for col_index in 0..self.cols {
            row_buffer.data[col_index] = self.at(row_index, col_index);
        }
    }

    pub fn copy_from(&mut self, src: &Matrix<T>) {
        assert!(
            self.rows == src.rows,
            "self.rows == src.rows expected {} but was {}",
            self.rows,
            src.rows
        );
        assert!(
            self.cols == src.cols,
            "self.cols == src.cols expected {} but was {}",
            self.cols,
            src.cols
        );

        for i in 0..self.data.len() {
            self.data[i] = src.data[i];
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}
