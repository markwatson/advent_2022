use std::{
    fmt::{Display, Formatter, Result},
    fs::read_to_string,
};
#[allow(dead_code)]
pub struct Matrix<T: Copy + Display> {
    pub data: Vec<Vec<T>>,
}

#[allow(dead_code)]
impl<T: Copy + Display> Display for Matrix<T> {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let mut str = "";
        for row in &self.data {
            for col in row {
                fmt.write_str(str)?;
                fmt.write_str(&format!("{}", col))?;
                str = ", ";
            }
            str = "";
            fmt.write_str("\n")?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
impl<T: Copy + Display> Matrix<T> {
    pub fn new(data: Vec<Vec<T>>) -> Matrix<T> {
        Matrix { data }
    }

    pub fn get_by_idx(&self, idx: usize) -> T {
        let r = idx / self.data[0].len();
        let c = idx % self.data[0].len();
        self.data[r][c]
    }

    pub fn len_cols(&self) -> usize {
        self.data[0].len()
    }

    pub fn len_rows(&self) -> usize {
        self.data.len()
    }

    pub fn up(&self, r: usize, c: usize) -> Option<T> {
        if r == 0 {
            None
        } else {
            Some(self.data[r - 1][c])
        }
    }

    pub fn down(&self, r: usize, c: usize) -> Option<T> {
        if r == self.data.len() - 1 {
            None
        } else {
            Some(self.data[r + 1][c])
        }
    }

    pub fn left(&self, r: usize, c: usize) -> Option<T> {
        if c == 0 {
            None
        } else {
            Some(self.data[r][c - 1])
        }
    }

    pub fn right(&self, r: usize, c: usize) -> Option<T> {
        if c == self.data[0].len() - 1 {
            None
        } else {
            Some(self.data[r][c + 1])
        }
    }

    pub fn neighbors(&self, r: usize, c: usize) -> Vec<T> {
        let mut neighbors = vec![];
        if let Some(n) = self.up(r, c) {
            neighbors.push(n);
        }
        if let Some(n) = self.down(r, c) {
            neighbors.push(n);
        }
        if let Some(n) = self.left(r, c) {
            neighbors.push(n);
        }
        if let Some(n) = self.right(r, c) {
            neighbors.push(n);
        }
        neighbors
    }

    pub fn neighbors_idx(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
        if let Some(_) = self.up(r, c) {
            neighbors.push((r - 1, c));
        }
        if let Some(_) = self.down(r, c) {
            neighbors.push((r + 1, c));
        }
        if let Some(_) = self.left(r, c) {
            neighbors.push((r, c - 1));
        }
        if let Some(_) = self.right(r, c) {
            neighbors.push((r, c + 1));
        }
        neighbors
    }
}

#[allow(dead_code)]
impl Matrix<char> {
    pub fn from_file(fname: &str) -> Matrix<char> {
        let input = read_to_string(fname).expect("Error reading file");
        let mut rows = vec![];
        for line in input.split('\n') {
            let mut cols = vec![];
            for c in line.trim().chars() {
                cols.push(c);
            }
            rows.push(cols);
        }
        Matrix::new(rows)
    }
}
