use kutils::{Color, Size};

#[derive(Debug)]
pub struct FigureComponent {
    pub size: Size,
    pub color: Color,
}

impl FigureComponent {
    pub fn new(size: Size, color: Color) -> Self {
        Self { size, color }
    }
}
