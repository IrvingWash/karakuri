package components

import "../../kmath"

Transform_Component :: struct {
	position: kmath.Vector2,
	scale:    kmath.Vector2,
	rotation: f64,
}

new_transform_component :: proc(
	position := kmath.Vector2{0, 0},
	scale := kmath.Vector2{1, 1},
	rotation: f64 = 0,
) -> Transform_Component {
	return Transform_Component {
		position = position,
		scale = scale,
		rotation = rotation,
	}
}
