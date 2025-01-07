package kutils

Color :: struct {
	r, g, b, a: u8,
}

new :: proc(r, g, b: u8, a: u8 = 255) -> Color {
	return Color{r, g, b, a}
}

Red :: Color{255, 0, 0, 255}
Green :: Color{0, 255, 0, 255}
Blue :: Color{0, 0, 255, 255}
Black :: Color{0, 0, 0, 255}
White :: Color{255, 255, 255, 255}
Yellow :: Color{255, 255, 0, 255}
