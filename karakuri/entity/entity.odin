package karakuri_entity

import "../components"

Entity :: struct {
	using token: Token,
	using data:  Entity_Payload,
}

Token :: struct {
	id:            int,
	generation_id: int,
}

Entity_Payload :: struct {
	using components: components.Component_Bundle,
	tag:              Maybe(string),
	behavior:         Maybe(^Behavior),
}

