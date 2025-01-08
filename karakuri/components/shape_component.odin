package karakuri_components

import v2 "kmath:vector2"
import "kutils:color"

// A component that represents an entity's shape for rendering
Shape_Component :: struct {
	size:  v2.Vector2,
	color: color.Color,
}

