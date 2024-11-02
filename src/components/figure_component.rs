use kutils::{Color, Size};

#[derive(Debug)]
pub struct FigureComponent {
    pub size: Size,
    pub color: Color,
    pub layer: u8,
}

impl FigureComponent {
    pub fn new(size: Size, color: Color, layer: u8) -> Self {
        Self { size, color, layer }
    }
}
