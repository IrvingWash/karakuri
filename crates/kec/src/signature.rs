use bit_set::BitSet;

#[derive(Debug, Default)]
pub struct Signature {
    data: BitSet,
}

impl From<Vec<usize>> for Signature {
    fn from(value: Vec<usize>) -> Self {
        let mut sig = Self::new();

        for v in value {
            sig.data.insert(v);
        }

        sig
    }
}

impl Signature {
    #[inline]
    pub fn new() -> Self {
        Self {
            data: BitSet::with_capacity(64),
        }
    }

    #[inline]
    pub fn set(&mut self, id: usize) {
        self.data.insert(id);
    }

    #[inline]
    pub fn reset(&mut self) {
        self.data.clear();
    }

    #[inline]
    pub fn is_superset(&self, other: &Self) -> bool {
        self.data.is_superset(&other.data)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
