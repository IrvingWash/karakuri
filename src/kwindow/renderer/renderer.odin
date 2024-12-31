package renderer

import rl "vendor:raylib"

import v2 "../../kmath/vector2"
import ku "../../kutils"

// Contains the info required for each draw call.
// If created with `new_renderer_info()`, the origin of the canvas will be at the center of the screen
// with X increasing to the right and Y increasing to the bottom.
Renderer_Info :: struct {
	clear_color:   ku.Color,
	window_width:  uint,
	window_height: uint,
	origin:        v2.Vector2,
}

// Initializes `Renderer_Info` with the origin of the canvas at the center of the screen
// with X increasing to the right and Y increasing to the bottom.
new_renderer_info :: proc(clear_color: ku.Color) -> Renderer_Info {
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

// Begins a draw call. Should be called each frame.
start_drawing :: proc(renderer_info: ^Renderer_Info) {
	update_window_size(renderer_info)

	rl.BeginDrawing()
	rl.ClearBackground(color_to_rl(renderer_info.clear_color))
}

// Submits the draw call. Should be called at the end of the each frame.
finish_drawing :: proc() {
	rl.EndDrawing()
}

// Draws a rectangle.
// The center of the rectangle is used as the origin by default.
// This can be overridden by passing `custom_origin`
draw_rectangle :: proc(
	renderer_info: Renderer_Info,
	position: v2.Vector2,
	width: f64,
	height: f64,
	scale: v2.Vector2,
	rotation: f64,
	color: ku.Color,
	custom_origin: Maybe(v2.Vector2) = nil,
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

// Updates the stored window size and the canvas origin.
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

// Converts karakuri's `kutils.Color` into `raylib.Color`
@(private = "file")
color_to_rl :: proc(color: ku.Color) -> rl.Color {
	using color

	return rl.Color{r, g, b, a}
}
