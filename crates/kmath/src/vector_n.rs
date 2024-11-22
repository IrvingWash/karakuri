use std::ops;

#[derive(Debug)]
pub struct VectorN {
    n: usize,
    data: Vec<f64>,
}

impl VectorN {
    #[inline]
    pub fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![0.0; n],
        }
    }

    #[inline]
    pub fn from_vec(vec: &[f64]) -> Self {
        Self {
            n: vec.len(),
            data: vec.to_vec(),
        }
    }

    #[inline]
    pub fn add(&mut self, other: &VectorN) {
        self.panic_varying_size(other);

        for (i, v) in self.data.iter_mut().enumerate() {
            *v += other.data[i];
        }
    }

    #[inline]
    pub fn subtract(&mut self, other: &VectorN) {
        self.panic_varying_size(other);

        for (i, v) in self.data.iter_mut().enumerate() {
            *v -= other.data[i];
        }
    }

    #[inline]
    pub fn scale(&mut self, value: f64) {
        for v in self.data.iter_mut() {
            *v *= value;
        }
    }

    #[inline]
    pub fn to_added(&self, other: &VectorN) -> VectorN {
        let mut temp = self.create_copy();

        temp.add(other);

        temp
    }

    #[inline]
    pub fn to_subtracted(&self, other: &VectorN) -> VectorN {
        let mut temp = self.create_copy();

        temp.subtract(other);

        temp
    }

    #[inline]
    pub fn to_scaled(&self, value: f64) -> VectorN {
        let mut temp = self.create_copy();

        temp.scale(value);

        temp
    }

    #[inline]
    pub fn dot_product(&self, other: &VectorN) -> f64 {
        self.panic_varying_size(other);

        let mut result = 0.0;

        for (i, v) in self.data.iter().enumerate() {
            result += v * other.data.get(i).unwrap_or(&1.0);
        }

        result
    }

    #[inline]
    pub fn set(&mut self, other: &VectorN) {
        self.panic_varying_size(other);

        for (i, v) in self.data.iter_mut().enumerate() {
            *v = other.data[i];
        }
    }

    #[inline]
    pub fn get(&self, id: usize) -> Option<&f64> {
        self.data.get(id)
    }

    #[inline]
    pub fn get_mut(&mut self, id: usize) -> Option<&mut f64> {
        self.data.get_mut(id)
    }

    #[inline]
    pub fn create_copy(&self) -> Self {
        Self {
            n: self.n,
            data: self.data.clone(),
        }
    }

    fn panic_varying_size(&self, other: &VectorN) {
        assert!(
            self.data.len() == other.data.len(),
            "Attempt to operate on `VectorN`s of different sizes"
        );
    }
}

impl ops::Index<usize> for VectorN {
    type Output = f64;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl ops::IndexMut<usize> for VectorN {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod vector_n_tests {
    use super::VectorN;

    #[test]
    fn test_new() {
        let vec_5 = VectorN::new(5);

        assert_eq!(vec_5.data.len(), 5);

        for v in vec_5.data {
            assert_eq!(v, 0.0);
        }
    }

    #[test]
    fn test_from_vec() {
        let vec = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let vec_n = VectorN::from_vec(&vec);

        for (i, v) in vec_n.data.iter().enumerate() {
            assert_eq!(*v, vec[i]);
        }
    }

    #[test]
    fn test_add() {
        let vec = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let mut first = VectorN::from_vec(&vec);
        let second = VectorN::from_vec(&vec);

        first.add(&second);

        for (i, v) in first.data.iter().enumerate() {
            assert_eq!(*v, vec[i] + vec[i]);
        }
    }

    #[test]
    fn test_subtract() {
        let vec = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let mut first = VectorN::from_vec(&vec);
        let second = VectorN::from_vec(&vec);

        first.subtract(&second);

        for (i, v) in first.data.iter().enumerate() {
            assert_eq!(*v, vec[i] - vec[i]);
        }
    }

    #[test]
    fn test_scale() {
        let vec = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let mut first = VectorN::from_vec(&vec);

        first.scale(3.0);

        for (i, v) in first.data.iter().enumerate() {
            assert_eq!(*v, vec[i] * 3.0);
        }
    }
}
