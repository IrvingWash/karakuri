package karakuri_game

import "core:slice"
import "kwindow:window_creation"
import "kwindow:renderer"
import "kwindow:fps_manager"
import "kwindow:input_manager"
import "kutils:color"
import "ktimer:timer"
@(require) import v2 "kmath:vector2"
import "../world"
import "../scene"
import "../asset_store"

// ====================================
// Game state
// ====================================
@(private = "file")
Game_Info :: struct {
	current_world: world.World,
	timer_info:    timer.Timer_Info,
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

	timer_info := timer.new(0)
	current_world := world.new({}, &timer_info)

	game_info = Game_Info {
		current_world = current_world,
		timer_info    = timer_info,
	}
}

// Starts the game
start :: proc() {
	for {
		delta_time := fps_manager.get_delta_time()
		time := fps_manager.get_time()

		if input_manager.is_quit_requested() {
			break
		}

		finished_timers := timer.update(&game_info.timer_info, time)
		defer delete(finished_timers)

		world.update(
			&game_info.current_world,
			delta_time,
			&game_info.timer_info,
			finished_timers,
		)

		render_entities()
	}
}

// Sets a scene created from the provided procedure
set_scene :: proc(scene_maker: scene.Scene_Maker_Proc) {
	scene_to_set := scene_maker()
	defer scene.destroy(scene_to_set)

	asset_store.destroy()
	asset_store.init()
	for &texture in scene_to_set.assets.textures {
		asset_store.load_texture(texture.name, texture.path)
	}

	world.destroy(&game_info.current_world, &game_info.timer_info)
	game_info.current_world = world.new(
		scene_to_set.entities[:],
		&game_info.timer_info,
	)
}

// Destroys the game
destroy :: proc() {
	world.destroy(&game_info.current_world, &game_info.timer_info)
	asset_store.destroy()
	timer.destroy(game_info.timer_info)
	window_creation.destroy_window()
}

// ====================================
// Private procedures
// ====================================

@(private)
render_entities :: proc() {
	renderer.start_drawing()
	defer renderer.finish_drawing()

	renderables := make(
		[dynamic]^world.Entity,
		0,
		len(game_info.current_world.entities),
	)
	defer delete(renderables)
	for &entity in game_info.current_world.entities {
		if _, ok := entity.sprite.?; ok {
			append(&renderables, &entity)
		}
	}

	slice.sort_by(renderables[:], proc(a, b: ^world.Entity) -> bool {
		return a.sprite.?.sorting_layer < b.sprite.?.sorting_layer
	})

	for &entity in renderables {
		renderer.draw_sprite(
			entity.sprite.?.sprite,
			entity.transform.position,
			entity.transform.scale,
			entity.transform.rotation,
		)
	}

	// Debug drawing
	when ODIN_DEBUG {
		for &entity in game_info.current_world.entities {
			if box_collider, ok := &entity.box_collider.?; ok {
				transform := &entity.transform

				origin: Maybe(v2.Vector2) = nil
				if sprite, ok := &entity.sprite.?; ok {
					origin = sprite.origin
				}

				renderer.draw_rectangle(
					transform.position + box_collider.offset,
					box_collider.size,
					transform.scale,
					transform.rotation,
					color.Color{0, 255, 0, 50},
					origin,
				)
			}
		}
	}
}

