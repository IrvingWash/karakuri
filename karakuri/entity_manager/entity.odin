package entity_manager

import comp "../components"

Entity :: struct {
	id:        uint,
	tags:      [dynamic]string,
	transform: comp.Transform_Component,
	shape:     Maybe(comp.Shape_Component),
}

