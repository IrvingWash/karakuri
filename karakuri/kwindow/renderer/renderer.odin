package renderer

import rl "vendor:raylib"

import v2 "../../kmath/vector2"
import u "../../kutils"

Renderer :: struct {
	clear_color:   u.Color,
	window_width:  uint,
	window_height: uint,
	origin:        v2.Vector2,
}

new :: proc(clear_color := u.ColorBlue) -> Renderer {
	window_width := rl.GetScreenWidth()
	window_height := rl.GetScreenHeight()

	return Renderer {
		clear_color = clear_color,
		window_width = uint(window_width),
		window_height = uint(window_height),
		origin = v2.Vector2 {
			x = f64(window_width) / 2,
			y = f64(window_height) / 2,
		},
	}
}

start_drawing :: proc(r: ^Renderer) {
	update_window_size(r)

	rl.BeginDrawing()
	rl.ClearBackground(color_to_rl(r.clear_color))
}

finish_drawing :: proc() {
	rl.EndDrawing()
}

draw_rectangle :: proc(
	r: Renderer,
	position: v2.Vector2,
	width: f64,
	height: f64,
	scale: v2.Vector2,
	rotation: f64,
	color: u.Color,
) {
	rl.DrawRectanglePro(
		rec = rl.Rectangle {
			x = f32(position.x + r.origin.x),
			y = f32(position.y + r.origin.y),
			width = f32(width * scale.x),
			height = f32(height * scale.y),
		},
		origin = rl.Vector2 {
			f32(width * scale.x / 2),
			f32(height * scale.y / 2),
		},
		rotation = f32(rotation),
		color = color_to_rl(color),
	)
}

@(private = "file")
update_window_size :: proc(r: ^Renderer) {
	width := uint(rl.GetScreenWidth())
	height := uint(rl.GetScreenHeight())

	if r.window_width == width && r.window_height == height {
		return
	}

	r.window_width = width
	r.window_height = height

	r.origin.x = f64(width) / 2
	r.origin.y = f64(height) / 2
}

@(private = "file")
color_to_rl :: proc(color: u.Color) -> rl.Color {
	using color

	return rl.Color{r, g, b, a}
}
