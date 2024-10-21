pub struct Size {
    pub width: i64,
    pub height: i64,
}

impl Size {
    pub fn new(width: i64, height: i64) -> Self {
        Self { width, height }
    }
}
