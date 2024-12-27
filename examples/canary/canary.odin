package canary

import "core:fmt"
import "core:math"
import v2 "karakuri:kmath/vector2"
import u "karakuri:kutils"
import w "karakuri:kwindow"
import fm "karakuri:kwindow/fps_manager"
import im "karakuri:kwindow/input_manager"
import renderer "karakuri:kwindow/renderer"

Player :: struct {
	position: v2.Vector2,
	width:    f64,
	height:   f64,
	color:    u.Color,
	rotation: f64,
	scale:    v2.Vector2,
	speed:    f64,
}

main :: proc() {
	player := Player {
		position = v2.Vector2{0, 0},
		color    = u.ColorBlack,
		height   = 100,
		width    = 100,
		rotation = 0,
		scale    = v2.Vector2{1, 1},
		speed    = 300,
	}

	w.create_window("Canary", 800, 600, false)
	defer w.destroy_window()

	fm.set_target_fps(60)

	r := renderer.new()

	for !im.is_quit_requested() {
		dt := math.min(fm.get_delta_time(), 0.032)

		if im.is_key_down(im.Key.W) {
			fmt.println(dt)
			player.position.y -= player.speed * dt
		}
		if im.is_key_down(im.Key.S) {
			player.position.y += player.speed * dt
		}
		if im.is_key_down(im.Key.D) {
			player.position.x += player.speed * dt
		}
		if im.is_key_down(im.Key.A) {
			player.position.x -= player.speed * dt
		}

		renderer.start_drawing(&r)

		renderer.draw_rectangle(
			r = r,
			position = player.position,
			width = player.width,
			height = player.height,
			scale = player.scale,
			rotation = player.rotation,
			color = player.color,
		)

		renderer.finish_drawing()
	}
}
