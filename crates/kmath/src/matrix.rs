use crate::VectorN;

#[derive(Debug)]
pub struct Matrix {
    m: usize,
    n: usize,

    rows: Vec<VectorN>,
}

impl Matrix {
    #[inline]
    pub fn new(m: usize, n: usize) -> Self {
        let rows = vec![VectorN::new(m); n];

        Self { m, n, rows }
    }

    #[inline]
    pub fn from_rows(rows: Vec<VectorN>) -> Self {
        let m = match rows.first() {
            Some(row) => row.len(),
            None => 0,
        };

        Self {
            m,
            n: rows.len(),
            rows,
        }
    }

    #[inline]
    pub fn create_copy(&self) -> Self {
        Self {
            m: self.m,
            n: self.n,
            rows: self.rows.clone(),
        }
    }

    #[inline]
    pub fn set(&mut self, other: &Matrix) {
        self.m = other.m;
        self.n = other.n;
        self.rows = other.rows.clone();
    }

    #[inline]
    pub fn transpose(&mut self) {}

    #[inline]
    pub fn to_transposed(&self) -> Matrix {
        let mut temp = self.create_copy();

        temp.transpose();

        temp
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn multiply_by_vector(&mut self, vector: &VectorN) {
        todo!()
    }

    #[inline]
    pub fn to_multiplied_by_vector(&self, vector: &VectorN) -> Matrix {
        let mut temp = self.create_copy();

        temp.multiply_by_vector(vector);

        temp
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn multiply_by_matrix(&mut self, other: &Matrix) {}

    #[inline]
    pub fn to_multiplied_by_matrix(&self, other: &Matrix) -> Matrix {
        let mut temp = self.create_copy();

        temp.multiply_by_matrix(other);

        temp
    }
}
