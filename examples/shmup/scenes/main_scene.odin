package example_shmup_scenes

import "karakuri:scene"
import "../prefabs"

make_main_scene: scene.Scene_Maker_Proc : proc() -> scene.Scene {
	return scene.Scene {
		name = "Main",
		entities = {prefabs.player_prefab(), prefabs.enemy_prefab({0, -270})},
	}
}

