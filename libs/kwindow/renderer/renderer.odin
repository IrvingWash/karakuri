package renderer

import rl "vendor:raylib"
import "kutils:color"
import v2 "kmath:vector2"

Renderer_Info :: struct {
	clear_color:   color.Color,
	window_width:  uint,
	window_height: uint,
	canvas_origin: v2.Vector2,
}

init :: proc(clear_color: color.Color) -> Renderer_Info {
	window_width := rl.GetScreenWidth()
	window_height := rl.GetScreenHeight()

	return Renderer_Info {
		clear_color = clear_color,
		window_width = uint(window_width),
		window_height = uint(window_height),
		canvas_origin = v2.Vector2 {
			f64(window_width) / 2,
			f64(window_height) / 2,
		},
	}
}

start_drawing :: proc(renderer_info: ^Renderer_Info) {
	update_renderer_info(renderer_info)

	rl.BeginDrawing()
	rl.ClearBackground(color_to_rl(renderer_info.clear_color))
}

finish_drawing :: proc() {
	rl.EndDrawing()
}

draw_rectangle :: proc(
	renderer_info: Renderer_Info,
	position: v2.Vector2,
	size: v2.Vector2,
	scale: v2.Vector2,
	rotation: f64,
	color: color.Color,
	custom_origin: Maybe(v2.Vector2) = nil,
) {
	scaled_width := size.x * scale.x
	scaled_height := size.y * scale.y

	origin: rl.Vector2 = ---
	if passed_origin, ok := custom_origin.?; ok {
		origin = rl.Vector2{f32(passed_origin.x), f32(passed_origin.y)}
	} else {
		origin = rl.Vector2{f32(scaled_width / 2), f32(scaled_height / 2)}
	}

	rl.DrawRectanglePro(
		rec = rl.Rectangle {
			x = f32(position.x + renderer_info.canvas_origin.x),
			y = f32(position.y + renderer_info.canvas_origin.y),
			width = f32(scaled_width),
			height = f32(scaled_height),
		},
		origin = origin,
		rotation = f32(rotation),
		color = color_to_rl(color),
	)
}

@(private = "file")
update_renderer_info :: proc(renderer_info: ^Renderer_Info) {
	width := uint(rl.GetScreenWidth())
	height := uint(rl.GetScreenHeight())

	if renderer_info.window_width == width &&
	   renderer_info.window_height == height {
		return
	}

	renderer_info.window_width = width
	renderer_info.window_height = height

	renderer_info.canvas_origin.x = f64(width) / 2
	renderer_info.canvas_origin.y = f64(height) / 2
}

@(private = "file")
color_to_rl :: proc(color: color.Color) -> rl.Color {
	return rl.Color{color.r, color.g, color.b, color.a}
}

