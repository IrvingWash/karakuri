package karakuri

import ku "../kutils"
import kw "../kwindow"
import im "../kwindow/input_manager"
import ren "../kwindow/renderer"

Game_Info :: struct {
	renderer_info: ren.Renderer_Info,
}

new_game :: proc(
	title := "karakuri game",
	width: uint = 800,
	height: uint = 600,
	clear_color := ku.ColorBlue,
	fullscreen := true,
	vsync := true,
) -> Game_Info {
	kw.create_window(
		title = title,
		width = width,
		height = height,
		fullscreen = fullscreen,
		vsync = vsync,
	)

	return Game_Info{renderer_info = ren.new_renderer_info(clear_color)}
}

destroy_game :: proc(game_info: Game_Info) {
	kw.destroy_window()
}

start_scene :: proc(game_info: ^Game_Info, scene: ^Scene) {
	for !im.is_quit_requested() {
		scene_update(scene, &game_info.renderer_info)
	}
}
