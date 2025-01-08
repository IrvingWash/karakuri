package karakuri_scene

import "../world"

// Contains initial information about the world
Scene :: struct {
	name:     string,
	entities: [dynamic]world.Entity_Payload,
	assets:   bool, // TODO: assets go here
}

// Scene maker type that should be implemented for lazy loading
Scene_Maker_Proc :: proc() -> Scene

// Destroys the scene
destroy :: proc(scene: Scene) {
	delete(scene.entities)
}

