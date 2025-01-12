package kwindow_renderer

import rl "vendor:raylib"
import "kutils:color"
import v2 "kmath:vector2"

// ====================================
// Renderer state
// ====================================

@(private = "file")
Renderer_Info :: struct {
	clear_color:   rl.Color,
	window_width:  uint,
	window_height: uint,
	canvas_origin: v2.Vector2,
}

@(private = "file")
renderer_info := Renderer_Info{}

// ====================================
// Public procs
// ====================================

// Initializes the global renderer
init :: proc(background_color: color.Color) {
	width := rl.GetScreenWidth()
	height := rl.GetScreenHeight()

	renderer_info = Renderer_Info {
		clear_color   = color_to_rl(background_color),
		window_width  = uint(width),
		window_height = uint(height),
		canvas_origin = v2.Vector2{f64(width) / 2, f64(height) / 2},
	}
}

// Starts a draw queue. All the draw calls should be done after this procedure is invoked
start_drawing :: proc() {
	update_renderer_info()

	rl.BeginDrawing()
	rl.ClearBackground(renderer_info.clear_color)
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

	origin :=
		custom_origin.? or_else v2.Vector2{scaled_width / 2, scaled_height / 2}

	rl.DrawRectanglePro(
		rec = rl.Rectangle {
			x = f32(position.x + renderer_info.canvas_origin.x),
			y = f32(position.y + renderer_info.canvas_origin.y),
			width = f32(scaled_width),
			height = f32(scaled_height),
		},
		origin = v2_to_rl(origin),
		rotation = f32(rotation),
		color = color_to_rl(color),
	)
}

draw_sprite :: proc(
	sprite: Sprite,
	position: v2.Vector2,
	scale: v2.Vector2,
	rotation: f64,
) {
	size :=
		sprite.clip_size.? or_else v2.Vector2 {
			f64(sprite.texture.width),
			f64(sprite.texture.height),
		}

	origin :=
		sprite.origin.? or_else v2.Vector2 {
			size.x * scale.x / 2,
			size.y * scale.y / 2,
		}

	rl.DrawTexturePro(
		sprite.texture,
		source = rl.Rectangle {
			x = f32(sprite.clip_position.x),
			y = f32(sprite.clip_position.y),
			width = f32(size.x) * (sprite.flip.x ? -1 : 1),
			height = f32(size.y) * (sprite.flip.y ? -1 : 1),
		},
		dest = rl.Rectangle {
			x = f32(position.x + renderer_info.canvas_origin.x),
			y = f32(position.y + renderer_info.canvas_origin.y),
			width = f32(size.x * scale.x),
			height = f32(size.y * scale.y),
		},
		origin = v2_to_rl(origin),
		rotation = f32(rotation),
		tint = color_to_rl(sprite.tint.? or_else color.White),
	)
}

// ====================================
// Private procs
// ====================================

update_renderer_info :: proc() {
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

@(private = "file")
v2_to_rl :: proc(vector2: v2.Vector2) -> rl.Vector2 {
	return rl.Vector2{f32(vector2.x), f32(vector2.y)}
}

