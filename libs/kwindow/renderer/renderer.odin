package kwindow_renderer

import rl "vendor:raylib"
import "kutils:color"
import v2 "kmath:vector2"

// ====================================
// Renderer state
// ====================================

@(private = "file")
CLEAR_COLOR: rl.Color

@(private = "file")
WINDOW_WIDTH: uint

@(private = "file")
WINDOW_HEIGHT: uint

@(private = "file")
CANVAS_ORIGIN: v2.Vector2

// ====================================
// Public procs
// ====================================

// Initializes the global renderer
init :: proc(background_color: color.Color) {
	width := rl.GetScreenWidth()
	height := rl.GetScreenHeight()

	CLEAR_COLOR = color_to_rl(background_color)
	WINDOW_WIDTH = uint(width)
	WINDOW_HEIGHT = uint(height)
	CANVAS_ORIGIN = v2.Vector2{f64(WINDOW_WIDTH) / 2, f64(WINDOW_HEIGHT) / 2}
}

// Starts a draw queue. All the draw calls should be done after this procedure is invoked
start_drawing :: proc() {
	update_renderer_info()

	rl.BeginDrawing()
	rl.ClearBackground(CLEAR_COLOR)
}

// Finishes and submits a draw queue. All the draw calls should be done before this procedure is invoked
finish_drawing :: proc() {
	rl.EndDrawing()
}

// Draws a rectangle with the origin in the center by default
draw_rectangle :: proc(
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
			x = f32(position.x + CANVAS_ORIGIN.x),
			y = f32(position.y + CANVAS_ORIGIN.y),
			width = f32(scaled_width),
			height = f32(scaled_height),
		},
		origin = origin,
		rotation = f32(rotation),
		color = color_to_rl(color),
	)
}

// ====================================
// Private procs
// ====================================

@(private = "file")
update_renderer_info :: proc() {
	width := uint(rl.GetScreenWidth())
	height := uint(rl.GetScreenHeight())

	if WINDOW_WIDTH == width && WINDOW_HEIGHT == height {
		return
	}

	WINDOW_WIDTH = width
	WINDOW_HEIGHT = height

	CANVAS_ORIGIN.x = f64(width) / 2
	CANVAS_ORIGIN.y = f64(height) / 2
}

@(private = "file")
color_to_rl :: proc(color: color.Color) -> rl.Color {
	return rl.Color{color.r, color.g, color.b, color.a}
}

