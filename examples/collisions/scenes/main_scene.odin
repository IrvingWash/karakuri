#+feature dynamic-literals

package example_shmup_scenes

import "karakuri:scene"
import "../prefabs"

make_main_scene: scene.Scene_Maker_Proc : proc() -> scene.Scene {
	return scene.Scene {
		name = "Main",
		entities = {prefabs.box_prefab(.Dynamic), prefabs.box_prefab(.Static)},
		assets = {
			textures = {
				{
					name = "square",
					path = "examples/collisions/assets/sprites/square.png",
				},
			},
		},
	}
}

