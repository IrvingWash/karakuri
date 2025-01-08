package karakuri_game

import "kwindow:window_creation"
import "kwindow:renderer"
import "kwindow:fps_manager"
import "kwindow:input_manager"
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

	current_world = world.new()
}

// Starts the game
start :: proc() {
	for {
		delta_time := fps_manager.get_delta_time()

		if input_manager.is_quit_requested() {
			break
		}

		update_entities(delta_time)
		render_entities()
	}
}

// Sets a scene created from the provided procedure
set_scene :: proc(scene_maker: scene.Scene_Maker_Proc) {
	destroy_all_entities_in_the_current_world(delta_time = 0)

	world.destroy(&current_world)

	current_world = world.new()

	scene_to_set := scene_maker()
	defer scene.destroy(scene_to_set)

	for &entity in scene_to_set.entities {
		world.add_entity(&current_world, entity)
	}

	start_all_entities_in_the_current_world(delta_time = 0)
}

// Destroys the game
destroy :: proc() {
	destroy_all_entities_in_the_current_world(delta_time = 0)
	world.destroy(&current_world)
	window_creation.destroy_window()
}

// ====================================
// Private procedures
// ====================================

@(private)
update_entities :: proc(delta_time: f64) {
	for &entity in current_world.entities {
		behavior, ok := entity.behavior.?
		if !ok {
			continue
		}

		on_update, on_update_ok := behavior.on_update.?
		if on_update_ok {
			on_update(make_behavior_context(&entity, delta_time))
		}
	}
}

@(private)
render_entities :: proc() {
	renderer.start_drawing()
	defer renderer.finish_drawing()

	for &entity in current_world.entities {
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

@(private)
start_all_entities_in_the_current_world :: proc(delta_time: f64) {
	for &entity in current_world.entities {
		behavior, ok := entity.behavior.?
		if !ok {
			continue
		}

		on_start, on_start_ok := behavior.on_start.?
		if !on_start_ok {
			continue
		}

		on_start(make_behavior_context(&entity, delta_time))
	}
}

@(private)
destroy_all_entities_in_the_current_world :: proc(delta_time: f64) {
	for &entity in current_world.entities {
		behavior, behavior_ok := entity.behavior.?
		if !behavior_ok {
			continue
		}

		on_destroy, on_destroy_ok := behavior.on_destroy.?
		if !on_destroy_ok {
			continue
		}

		on_destroy(make_behavior_context(&entity, delta_time))
	}
}

@(private = "file")
make_behavior_context :: proc(
	entity: ^world.Entity,
	delta_time: f64,
) -> world.Behavior_Context {
	return world.Behavior_Context {
		self = entity,
		delta_time = delta_time,
		world = &current_world,
	}
}

