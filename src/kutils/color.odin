package kutils

Color :: struct {
	r, g, b, a: u8,
}

new_color :: proc(r, g, b: u8, a: u8 = 255) -> Color {
	return Color{r, g, b, a}
}

ColorRed :: Color{255, 0, 0, 255}
ColorGreen :: Color{0, 255, 0, 255}
ColorBlue :: Color{0, 0, 255, 255}
ColorBlack :: Color{0, 0, 0, 255}
ColorWhite :: Color{255, 255, 255, 255}
ColorYellow :: Color{255, 255, 0, 255}

