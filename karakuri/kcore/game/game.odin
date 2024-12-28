package game

import "../../kec"
import ku "../../kutils"
import kw "../../kwindow"
import fm "../../kwindow/fps_manager"
import im "../../kwindow/input_manager"
import ren "../../kwindow/renderer"
import c "../components"

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
		update(game)
		render(game)
	}
}

destroy_game :: proc(game: Game) {
	kec.destroy_registry(game.registry)
	kw.destroy_window()
}

@(private = "file")
update :: proc(game: ^Game) {
	dt := fm.get_delta_time()

	updatable_entities_query := kec.start_query()
	kec.query_with(
		c.Behavior_Component,
		&updatable_entities_query,
		game.registry,
	)
	updatable_entities := kec.submit_query(
		updatable_entities_query,
		game.registry,
	)

	for entity in updatable_entities {
		behavior := kec.get_component(
			game.registry,
			entity,
			c.Behavior_Component,
		)

		on_update, ok := behavior.on_update.?
		if !ok {
			continue
		}

		on_update(
			{
				entity = entity,
				registry = game.registry,
				dt = dt,
				is_key_pressed = im.is_key_pressed,
				is_key_down = im.is_key_down,
				is_key_up = im.is_key_up,
				is_key_released = im.is_key_released,
			},
		)
	}

	delete(updatable_entities)
}

@(private = "file")
render :: proc(game: ^Game) {
	drawable_entities_query := kec.start_query()
	kec.query_with(
		c.Transform_Component,
		&drawable_entities_query,
		game.registry,
	)
	kec.query_with(c.Shape_Component, &drawable_entities_query, game.registry)
	drawable_entities := kec.submit_query(
		drawable_entities_query,
		game.registry,
	)
	defer delete(drawable_entities)

	ren.start_drawing(&game.renderer)
	defer ren.finish_drawing()

	for entity in drawable_entities {
		transform := kec.get_component(
			game.registry,
			entity,
			c.Transform_Component,
		)
		shape := kec.get_component(game.registry, entity, c.Shape_Component)

		ren.draw_rectangle(
			game.renderer,
			transform.position,
			shape.width,
			shape.height,
			transform.scale,
			transform.rotation,
			shape.color,
		)
	}
}
