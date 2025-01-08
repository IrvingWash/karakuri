package karakuri_game

import "kwindow:window_creation"
import "kwindow:renderer"
import "kutils:color"
import "../world"
import "../scene"

// ====================================
// Game state
// ====================================
@(private = "file")
current_world: world.World

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
) {
	window_creation.init_window(
		title = title,
		width = width,
		height = height,
		fullscreen = fullscreen,
		resizable = resizable,
		vsync = vsync,
	)

	renderer.init(background_color)

	current_world = world.new()
}

// Sets a scene created from the provided procedure
set_scene :: proc(scene_maker: scene.Scene_Maker_Proc) {
	world.destroy(&current_world)
	current_world = world.new()

	scene_to_set := scene_maker()
	defer scene.destroy(scene_to_set)

	for &entity in scene_to_set.entities {
		world.add_entity(&current_world, entity)
	}
}

// Destroys the game
destroy :: proc() {
	world.destroy(&current_world)
	window_creation.destroy_window()
}

