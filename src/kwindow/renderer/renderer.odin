package renderer

import rl "vendor:raylib"

import v2 "../../kmath/vector2"
import ku "../../kutils"

Renderer_Info :: struct {
	clear_color:   ku.Color,
	window_width:  uint,
	window_height: uint,
	origin:        v2.Vector2,
}

new_renderer :: proc(clear_color: ku.Color) -> Renderer_Info {
	window_width := rl.GetScreenWidth()
	window_height := rl.GetScreenHeight()

	return Renderer_Info {
		clear_color = clear_color,
		window_width = uint(window_width),
		window_height = uint(window_height),
		origin = v2.Vector2 {
			x = f64(window_width) / 2,
			y = f64(window_height) / 2,
		},
	}
}

start_drawing :: proc(ri: ^Renderer_Info) {
	update_window_size(ri)

	rl.BeginDrawing()
	rl.ClearBackground(color_to_rl(ri.clear_color))
}

finish_drawing :: proc() {
	rl.EndDrawing()
}

draw_rectangle :: proc(
	renderer_info: Renderer_Info,
	position: v2.Vector2,
	width: f64,
	height: f64,
	scale: v2.Vector2,
	rotation: f64,
	color: ku.Color,
) {
	rl.DrawRectanglePro(
		rec = rl.Rectangle {
			x = f32(position.x + renderer_info.origin.x),
			y = f32(position.y + renderer_info.origin.y),
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
update_window_size :: proc(ri: ^Renderer_Info) {
	width := uint(rl.GetScreenWidth())
	height := uint(rl.GetScreenHeight())

	if ri.window_width == width && ri.window_height == height {
		return
	}

	ri.window_width = width
	ri.window_height = height

	ri.origin.x = f64(width) / 2
	ri.origin.y = f64(height) / 2
}

@(private = "file")
color_to_rl :: proc(color: ku.Color) -> rl.Color {
	using color

	return rl.Color{r, g, b, a}
}
