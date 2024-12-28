package components

import v2 "../../kmath/vector2"

Transform_Component :: struct {
	position: v2.Vector2,
	scale:    v2.Vector2,
	rotation: f64,
}

new_transform_component :: proc(
	position := v2.Vector2{0, 0},
	scale := v2.Vector2{1, 1},
	rotation: f64 = 0,
) -> Transform_Component {
	return Transform_Component{position, scale, rotation}
}
