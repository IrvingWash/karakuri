package karakuri

import "../../kutils"
import "../../kwindow"
import "../../kwindow/fps_manager"
import "../../kwindow/input_manager"
import "../../kwindow/renderer"
import "../components"
import "../scene"

Game_Info :: struct {
	renderer_info: renderer.Renderer_Info,
}

new_game :: proc(
	title := "karakuri game",
	width: uint = 800,
	height: uint = 600,
	clear_color := kutils.ColorBlue,
	fullscreen := true,
	vsync := true,
	target_fps: uint = 60,
) -> Game_Info {
	kwindow.create_window(
		title = title,
		width = width,
		height = height,
		fullscreen = fullscreen,
		vsync = vsync,
	)

	fps_manager.set_target_fps(target_fps)

	return Game_Info{renderer_info = renderer.new_renderer_info(clear_color)}
}

destroy_game :: proc(game_info: Game_Info) {
	kwindow.destroy_window()
}

create_scene :: proc(
	entities: [dynamic]components.Component_Bundle,
) -> scene.Scene_Info {
	return scene.new_scene(entities)
}

// TODO: Disallow running several scene at once
// TODO: Maybe the previous scene should be destroyed here
start_scene :: proc(game_info: ^Game_Info, scene_info: ^scene.Scene_Info) {
	for !input_manager.is_quit_requested() {
		scene.update(scene_info, &game_info.renderer_info)
	}

	// TODO: Is it nice to do this here?
	scene.destroy_scene_info(scene_info^)
}

