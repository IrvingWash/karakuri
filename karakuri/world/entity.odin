package karakuri_world

import "../components"

// An entity
Entity :: struct {
	using token:      Token,
	tag:              Maybe(string),
	behavior:         Maybe(^Behavior),
	transform:        components.Transform_Component,
	using components: Component_Bundle,
}

// Unique identifier of an entity
Token :: struct {
	id:            int,
	generation_id: int,
}

// Entity data
Entity_Payload :: struct {
	tag:              Maybe(string),
	behavior:         Maybe(^Behavior),
	transform:        Maybe(components.Transform_Component),
	using components: Component_Bundle,
}

@(private = "file")
Component_Bundle :: struct {
	sprite:       Maybe(components.Sprite_Component),
	box_collider: Maybe(components.Box_Collider_Component),
}

