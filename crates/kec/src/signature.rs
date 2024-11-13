use bit_set::BitSet;

#[derive(Debug, Default)]
pub struct Signature {
    data: BitSet,
}

impl From<Vec<usize>> for Signature {
    #[inline]
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

#[cfg(test)]
mod signature_tests {
    use super::*;

    #[test]
    fn test_new() {
        let signature = Signature::new();

        assert!(signature.is_empty());
    }

    #[test]
    fn test_supersetting() {
        let mut signature_a = Signature::new();
        let mut signature_b = Signature::new();

        signature_a.set(3);
        signature_a.set(4);
        signature_b.set(3);

        assert!(signature_a.is_superset(&signature_b));
        assert!(!signature_b.is_superset(&signature_a));
    }

    #[test]
    fn test_reset() {
        let mut signature_a = Signature::new();
        signature_a.set(3);
        signature_a.set(4);

        signature_a.reset();

        assert!(signature_a.data.is_empty());
    }
}
