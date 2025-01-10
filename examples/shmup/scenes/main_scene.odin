#+feature dynamic-literals

package example_shmup_scenes

import "karakuri:scene"
import "../prefabs"

make_main_scene: scene.Scene_Maker_Proc : proc() -> scene.Scene {
	return scene.Scene {
		name = "Main",
		entities = {
			prefabs.bullet_destroyer_prefab({0, -300}),
			prefabs.bullet_destroyer_prefab({0, 300}),
			prefabs.player_prefab(),
			prefabs.enemy_spawner_prefab(),
		},
	}
}

