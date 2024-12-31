package karakuri

import ku "../kutils"
import kw "../kwindow"
import fps "../kwindow/fps_manager"
import input "../kwindow/input_manager"
import renderer "../kwindow/renderer"

// Contains the game info.
Game_Info :: struct {
	renderer_info: renderer.Renderer_Info,
}

// Set's up a new game and returns the game info.
new_game :: proc(
	title := "karakuri game",
	width: uint = 800,
	height: uint = 600,
	clear_color := ku.ColorBlue,
	fullscreen := true,
	vsync := true,
	target_fps: uint = 60,
) -> Game_Info {
	kw.create_window(
		title = title,
		width = width,
		height = height,
		fullscreen = fullscreen,
		vsync = vsync,
	)

	fps.set_target_fps(target_fps)

	return Game_Info{renderer_info = renderer.new_renderer_info(clear_color)}
}

// Cleans up the game.
destroy_game :: proc(game_info: Game_Info) {
	kw.destroy_window()
}

// Starts playing the passed scene.
// TODO: Disallow running several scene at once
// TODO: Maybe the previous scene should be destroyed here
start_scene :: proc(game_info: ^Game_Info, scene: ^Scene) {
	for !input.is_quit_requested() {
		scene_update(scene, &game_info.renderer_info)
	}
}
