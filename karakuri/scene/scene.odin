package karakuri_scene

import "../entity"

// Contains initial information about the world
Scene :: struct {
	name:     string,
	entities: []entity.Entity_Payload,
	assets:   bool, // TODO: assets go here
}

