package karakuri_world

import "../components"

// An entity
Entity :: struct {
	using token: Token,
	using data:  Entity_Payload,
}

// Unique identifier of an entity
Token :: struct {
	id:            int,
	generation_id: int,
}

// Entity data
Entity_Payload :: struct {
	using components: components.Component_Bundle,
	tag:              Maybe(string),
	behavior:         Maybe(^Behavior),
}

