package karakuri_game

import "kwindow:window_creation"
import "kwindow:renderer"
import "kwindow:fps_manager"
import "kwindow:input_manager"
import "kutils:color"
import "ktimer:timer"
import "../world"
import "../scene"

// ====================================
// Game state
// ====================================
@(private = "file")
Game_Info :: struct {
	current_world: world.World,
	timer:         timer.Timer_Info,
}

@(private = "file")
game_info := Game_Info{}

// ====================================
// Public procedures
// ====================================

// Initializes a game
init :: proc(
	title: string,
	width, height: uint,
	fullscreen := true,
	resizable := true,
	vsync := true,
	background_color := color.Blue,
	target_fps: uint = 60,
) {
	window_creation.init_window(
		title = title,
		width = width,
		height = height,
		fullscreen = fullscreen,
		resizable = resizable,
		vsync = vsync,
	)

	fps_manager.set_target_fps(target_fps)

	renderer.init(background_color)

	// TODO: This is bad, but someone can call start without setting a scene
	// Maybe `Maybe`
	game_info.current_world = world.new({})
}

// Starts the game
start :: proc() {
	for {
		delta_time := fps_manager.get_delta_time()

		if input_manager.is_quit_requested() {
			break
		}

		world.update(&game_info.current_world, delta_time)

		render_entities()
	}
}

// Sets a scene created from the provided procedure
set_scene :: proc(scene_maker: scene.Scene_Maker_Proc) {
	scene_to_set := scene_maker()
	defer scene.destroy(scene_to_set)

	world.destroy(&game_info.current_world)
	game_info.current_world = world.new(scene_to_set.entities[:])
}

// Destroys the game
destroy :: proc() {
	world.destroy(&game_info.current_world)
	window_creation.destroy_window()
}

// ====================================
// Private procedures
// ====================================

@(private)
render_entities :: proc() {
	renderer.start_drawing()
	defer renderer.finish_drawing()

	for &entity in game_info.current_world.entities {
		shape, shape_ok := entity.shape.?
		if !shape_ok {
			continue
		}

		transform := entity.transform

		renderer.draw_rectangle(
			transform.position,
			shape.size,
			transform.scale,
			transform.rotation,
			shape.color,
		)
	}
}

