package karakuri_scene

import "../entity"

// Contains initial information about the world
Scene :: struct {
	name:     string,
	entities: [dynamic]entity.Entity_Payload,
	assets:   bool, // TODO: assets go here
}

Scene_Maker_Proc :: proc() -> Scene

destroy :: proc(scene: Scene) {
	delete(scene.entities)
}

