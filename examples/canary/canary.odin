package canary

import "core:fmt"
import "karakuri:kec"
import "karakuri:kutils"
import "karakuri:kwindow"
import "karakuri:kwindow/fps_manager"
import "karakuri:kwindow/input_manager"
import renderer "karakuri:kwindow/renderer"

_ :: input_manager

Vector2 :: [2]f64

Shape :: struct {
	size:  Vector2,
	color: kutils.Color,
}

Transform :: struct {
	position: Vector2,
	scale:    Vector2,
	rotation: f64,
}

main :: proc() {
	// Setup window
	kwindow.create_window(
		"Sonic The Hedgehog",
		800,
		600,
		fullscreen = false,
		vsync = true,
	)
	defer kwindow.destroy_window()

	fps_manager.set_target_fps(60)

	// Setup registry and renderer
	registry := kec.new_registry()
	defer kec.destroy_registry(registry)

	renderer_info := renderer.new_renderer_info(kutils.ColorBlack)

	// Setup game objects
	sonic := kec.create_entity(&registry)
	tails := kec.create_entity(&registry)

	kec.add_component(
		&registry,
		sonic,
		Shape{size = {100, 100}, color = kutils.ColorBlue},
	)
	kec.add_component(
		&registry,
		sonic,
		Transform{position = {0, 0}, scale = {1, 1}, rotation = 0},
	)

	kec.add_component(
		&registry,
		tails,
		Shape{size = {80, 80}, color = kutils.new_color(255, 255, 0)},
	)
	kec.add_component(
		&registry,
		tails,
		Transform{position = {100, 100}, scale = {1, 1}, rotation = 0},
	)

	// Game loop
	for !input_manager.is_quit_requested() {
		// Render
		renderer.start_drawing(&renderer_info)
		defer renderer.finish_drawing()

		renderable_query := kec.query_start()
		kec.query_with(Transform, &renderable_query, registry)
		kec.query_with(Shape, &renderable_query, registry)
		renderable_entities := kec.query_submit(renderable_query, registry)
		defer delete(renderable_entities)

		for entity in renderable_entities {
			transform := kec.get_component(registry, entity, Transform)
			shape := kec.get_component(registry, entity, Shape)

			fmt.println(entity)
			fmt.println(transform)
			fmt.println(shape)

			renderer.draw_rectangle(
				renderer_info,
				transform.position,
				shape.size.x,
				shape.size.y,
				transform.scale,
				transform.rotation,
				shape.color,
			)
		}
	}
}
