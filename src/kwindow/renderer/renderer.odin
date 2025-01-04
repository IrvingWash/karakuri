package renderer

import rl "vendor:raylib"

import "../../kmath"
import "../../kutils"

Renderer_Info :: struct {
	clear_color:   kutils.Color,
	window_width:  uint,
	window_height: uint,
	origin:        kmath.Vector2,
}

new_renderer_info :: proc(clear_color: kutils.Color) -> Renderer_Info {
	window_width := rl.GetScreenWidth()
	window_height := rl.GetScreenHeight()

	return Renderer_Info {
		clear_color = clear_color,
		window_width = uint(window_width),
		window_height = uint(window_height),
		origin = kmath.Vector2{f64(window_width) / 2, f64(window_height) / 2},
	}
}

start_drawing :: proc(renderer_info: ^Renderer_Info) {
	update_window_size(renderer_info)

	rl.BeginDrawing()
	rl.ClearBackground(color_to_rl(renderer_info.clear_color))
}

finish_drawing :: proc() {
	rl.EndDrawing()
}

draw_rectangle :: proc(
	renderer_info: Renderer_Info,
	position: kmath.Vector2,
	width: f64,
	height: f64,
	scale: kmath.Vector2,
	rotation: f64,
	color: kutils.Color,
	custom_origin: Maybe(kmath.Vector2) = nil,
) {
	scaled_width := width * scale.x
	scaled_height := height * scale.y

	origin: rl.Vector2 = ---
	if passed_origin, ok := custom_origin.?; ok {
		origin = rl.Vector2{f32(passed_origin.x), f32(passed_origin.y)}
	} else {
		origin = rl.Vector2{f32(scaled_width / 2), f32(scaled_height / 2)}
	}

	rl.DrawRectanglePro(
		rec = rl.Rectangle {
			x = f32(position.x + renderer_info.origin.x),
			y = f32(position.y + renderer_info.origin.y),
			width = f32(scaled_width),
			height = f32(scaled_height),
		},
		origin = origin,
		rotation = f32(rotation),
		color = color_to_rl(color),
	)
}

@(private = "file")
update_window_size :: proc(renderer_info: ^Renderer_Info) {
	width := uint(rl.GetScreenWidth())
	height := uint(rl.GetScreenHeight())

	if renderer_info.window_width == width &&
	   renderer_info.window_height == height {
		return
	}

	renderer_info.window_width = width
	renderer_info.window_height = height

	renderer_info.origin.x = f64(width) / 2
	renderer_info.origin.y = f64(height) / 2
}

@(private = "file")
color_to_rl :: proc(color: kutils.Color) -> rl.Color {
	using color

	return rl.Color{r, g, b, a}
}

