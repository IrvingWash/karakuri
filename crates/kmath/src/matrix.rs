use crate::VectorN;

#[derive(Debug)]
pub struct Matrix {
    row_length: usize,
    column_length: usize,

    data: Vec<VectorN>,
}

impl Matrix {
    #[inline]
    pub fn new(row_length: usize, column_length: usize) -> Self {
        let data = vec![VectorN::new(column_length); row_length];

        Self {
            row_length,
            column_length,
            data,
        }
    }

    #[inline]
    pub fn from_data(data: &[VectorN]) -> Self {
        Self::panic_deformed(data);

        let n = match data.first() {
            Some(column) => column.len(),
            None => 0,
        };

        Self {
            row_length: data.len(),
            column_length: n,
            data: data.to_vec(),
        }
    }

    #[inline]
    pub fn create_copy(&self) -> Self {
        Self {
            row_length: self.row_length,
            column_length: self.column_length,
            data: self.data.clone(),
        }
    }

    #[inline]
    pub fn set(&mut self, other: &Matrix) {
        Self::panic_deformed_pair(&self.data, &other.data);

        self.row_length = other.row_length;
        self.column_length = other.column_length;
        self.data = other.data.clone();
    }

    #[inline]
    pub fn to_transposed(&self) -> Matrix {
        let mut result = Matrix::new(self.column_length, self.row_length);

        for i in 0..self.row_length {
            for j in 0..self.column_length {
                result.data[j][i] = self.data[i][j];
            }
        }

        result
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
    pub fn multiply_by_matrix(&mut self, other: &Matrix) {
        todo!()
    }

    #[inline]
    pub fn to_multiplied_by_matrix(&self, other: &Matrix) -> Matrix {
        let mut temp = self.create_copy();

        temp.multiply_by_matrix(other);

        temp
    }

    fn panic_deformed(data: &[VectorN]) {
        if let Some(first_column) = data.first() {
            let gauge = first_column.len();

            assert!(data.iter().all(|column| column.len() == gauge));
        }
    }

    fn panic_deformed_pair(a: &[VectorN], b: &[VectorN]) {
        if let Some(first_column) = a.first() {
            let gauge = first_column.len();

            assert!([a, b].concat().iter().all(|column| column.len() == gauge));
        }
    }
}

#[cfg(test)]
mod matrix_tests {
    use crate::VectorN;

    use super::Matrix;

    #[test]
    fn test_new() {
        let matrix = Matrix::new(3, 2);

        assert_eq!(matrix.row_length, 3);
        assert_eq!(matrix.column_length, 2);

        assert_eq!(matrix.data.len(), 3);

        for column in matrix.data {
            assert_eq!(column.len(), 2);
            assert_eq!(column[0], 0.0);
            assert_eq!(column[1], 0.0);
            assert!(column.get(2).is_none());
        }
    }

    #[test]
    fn test_from_data() {
        let data = vec![
            VectorN::from_vec(&vec![1.0, 2.0, 3.0]),
            VectorN::from_vec(&vec![4.0, 5.0, 6.0]),
        ];

        let matrix = Matrix::from_data(&data);

        assert_eq!(matrix.row_length, 2);
        assert_eq!(matrix.column_length, 3);

        assert_eq!(matrix.data.len(), 2);

        for column in matrix.data {
            assert_eq!(column.len(), 3);
        }
    }

    #[test]
    fn test_set() {
        let mut first = Matrix::new(2, 3);

        let data = vec![
            VectorN::from_vec(&vec![1.0, 2.0, 3.0]),
            VectorN::from_vec(&vec![4.0, 5.0, 6.0]),
        ];

        let second = Matrix::from_data(&data);

        first.set(&second);

        for (i, column) in first.data.iter().enumerate() {
            for (j, v) in column.data().iter().enumerate() {
                assert_eq!(*v, second.data[i].data()[j]);
            }
        }
    }

    #[test]
    fn test_to_transposed() {
        let data = vec![
            VectorN::from_vec(&vec![1.0, 2.0, 3.0]),
            VectorN::from_vec(&vec![4.0, 5.0, 6.0]),
        ];

        let first = Matrix::from_data(&data);

        let transposed = first.to_transposed();

        assert_eq!(transposed.row_length, 3);
        assert_eq!(transposed.column_length, 2);

        assert_eq!(transposed.data.len(), 3);

        assert_eq!(*transposed.data[0].data(), vec![1.0, 4.0]);
        assert_eq!(*transposed.data[1].data(), vec![2.0, 5.0]);
        assert_eq!(*transposed.data[2].data(), vec![3.0, 6.0]);
    }
}
