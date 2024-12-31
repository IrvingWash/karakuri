package karakuri

import v2 "../kmath/vector2"

Transform_Component :: struct {
	position: v2.Vector2,
	scale:    v2.Vector2,
	rotation: f64,
}

new_transform_component :: proc(
	position := v2.ZERO,
	scale := v2.UNIT,
	rotation: f64 = 0,
) -> Transform_Component {
	return Transform_Component {
		position = position,
		scale = scale,
		rotation = rotation,
	}
}
