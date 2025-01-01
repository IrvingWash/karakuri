package scene_tests

import "core:testing"
import "../../src/karakuri"

test_entity_cycle :: proc(t: ^testing.T) {
	using testing

	initial_entities := make([dynamic]karakuri.Component_Bundle)

	scene := karakuri.new_scene(initial_entities)
	defer karakuri.destroy_scene(scene)
}
