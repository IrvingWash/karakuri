#[derive(Debug, PartialEq)]
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

    pub fn solid(red: u8, green: u8, blue: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha: 255,
        }
    }

    pub const fn to_tuple(&self) -> (u8, u8, u8, u8) {
        (self.red, self.green, self.blue, self.alpha)
    }

    pub const RED: Color = Color {
        red: 255,
        green: 0,
        blue: 0,
        alpha: 255,
    };

    pub const GREEN: Color = Color {
        red: 0,
        green: 255,
        blue: 0,
        alpha: 255,
    };

    pub const BLUE: Color = Color {
        red: 0,
        green: 0,
        blue: 255,
        alpha: 255,
    };

    pub const WHITE: Color = Color {
        red: 255,
        green: 255,
        blue: 255,
        alpha: 255,
    };

    pub const BLACK: Color = Color {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 255,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_RED: u8 = 29;
    const TEST_GREEN: u8 = 201;
    const TEST_BLUE: u8 = 103;
    const TEST_ALPHA: u8 = 50;

    #[test]
    fn test_new() {
        let color = Color::solid(TEST_RED, TEST_GREEN, TEST_BLUE);

        assert_eq!(color.red, TEST_RED);
        assert_eq!(color.green, TEST_GREEN);
        assert_eq!(color.blue, TEST_BLUE);
    }

    #[test]
    fn test_new_with_alpha() {
        let color = Color::new(TEST_RED, TEST_GREEN, TEST_BLUE, TEST_ALPHA);

        assert_eq!(color.red, TEST_RED);
        assert_eq!(color.green, TEST_GREEN);
        assert_eq!(color.blue, TEST_BLUE);
        assert_eq!(color.alpha, TEST_ALPHA);
    }

    #[test]
    fn test_to_tuple() {
        let color = Color::RED;

        assert_eq!(color.to_tuple(), (255, 0, 0, 255));
    }
}
