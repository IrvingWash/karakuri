package shmup

import "karakuri:karakuri/components"
import "karakuri:kmath"
import "karakuri:kutils"

projectile_destroyer_prefub :: proc(
	position: kmath.Vector2,
) -> components.Component_Bundle {
	return {
		transform = components.new_transform_component(position = position),
		shape = components.Shape_Component {
			color = kutils.ColorGreen,
			size = {DODONPACHI_WIDTH * 2, 10},
		},
	}
}
