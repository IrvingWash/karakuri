package karakuri

import ku "../kutils"
import kw "../kwindow"
import im "../kwindow/input_manager"
import ren "../kwindow/renderer"

Game :: struct {
	renderer_info: ren.Renderer_Info,
}

new_game :: proc(
	title := "karakuri game",
	width: uint = 800,
	height: uint = 600,
	clear_color := ku.ColorBlue,
	fullscreen := true,
	vsync := true,
) -> Game {
	kw.create_window(
		title = title,
		width = width,
		height = height,
		fullscreen = fullscreen,
		vsync = vsync,
	)

	return Game {
		renderer_info = ren.new_renderer(clear_color),
	}
}

destroy_game :: proc(game: Game) {
	kw.destroy_window()
}

start_game :: proc(game: ^Game) {
	for !im.is_quit_requested() {
		update(game^)
		render(game)
	}
}

@(private = "file")
update :: proc(game: Game) {
}

@(private = "file")
render :: proc(game: ^Game) {
	ren.start_drawing(&game.renderer_info)
	ren.finish_drawing()
}
