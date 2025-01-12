package example_shmup_prefabs

import "karakuri:world"
import "karakuri:components"

stars_prefab :: proc() -> world.Entity_Payload {
	return world.Entity_Payload {
		transform = components.Transform_Component{scale = {2, 1.5}},
		sprite = components.Sprite_Component {
			sprite_name = "stars",
			sorting_layer = 1,
		},
	}
}

