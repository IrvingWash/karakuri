package karakuri_scene

import "../entity"

Scene :: struct {
	name:     string,
	entities: []entity.Entity_Payload,
	assets:   bool, // TODO: assets go here
}

