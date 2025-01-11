package karakuri_components

import v2 "kmath:vector2"

// A component that represents an entity's transform
Transform_Component :: struct {
	position: v2.Vector2,
	scale:    v2.Vector2,
	rotation: f64,
}

DEFAULT_TRANSFORM_COMPONENT :: Transform_Component {
	position = v2.Zero,
	scale    = v2.Unit,
	rotation = 0,
}

