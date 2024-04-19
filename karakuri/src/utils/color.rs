pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn to_tuple(&self) -> (u8, u8, u8, u8) {
        (self.red, self.green, self.blue, self.alpha)
    }

    pub fn white() -> Color {
        Color {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 255,
        }
    }

    pub fn black() -> Color {
        Color {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 255,
        }
    }

    pub fn red() -> Color {
        Color {
            red: 255,
            green: 0,
            blue: 0,
            alpha: 255,
        }
    }

    pub fn green() -> Color {
        Color {
            red: 0,
            green: 255,
            blue: 0,
            alpha: 255,
        }
    }

    pub fn blue() -> Color {
        Color {
            red: 0,
            green: 0,
            blue: 255,
            alpha: 255,
        }
    }
}
