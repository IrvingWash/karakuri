package game

import "../../kec"
import ku "../../kutils"
import kw "../../kwindow"
import fm "../../kwindow/fps_manager"
import im "../../kwindow/input_manager"
import ren "../../kwindow/renderer"

Game :: struct {
	registry: kec.Registry,
	renderer: ren.Renderer,
}

new_game :: proc(
	title: string = "karakuri game",
	width: uint = 800,
	height: uint = 600,
	target_fps: uint = 60,
	fullscreen: bool = true,
	vsync: bool = true,
	clear_color := ku.ColorBlue,
) -> Game {
	kw.create_window(title, width, height, fullscreen, vsync)

	fm.set_target_fps(target_fps)

	return Game {
		registry = kec.new_registry(),
		renderer = ren.new_renderer(clear_color),
	}
}

start :: proc(game: ^Game) {
	for !im.is_quit_requested() {
		ren.start_drawing(&game.renderer)

		ren.finish_drawing()
	}
}

destroy_game :: proc() {
	kw.destroy_window()
}
