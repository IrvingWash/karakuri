#+feature dynamic-literals

package example_shmup_scenes

import "karakuri:scene"
import "../prefabs"

make_main_scene: scene.Scene_Maker_Proc : proc() -> scene.Scene {
	return scene.Scene {
		name = "Main",
		entities = {
			prefabs.space_prefab(),
			prefabs.stars_prefab(),
			prefabs.player_prefab(),
			prefabs.enemy_spawner_prefab(),
			prefabs.bullet_destroyer_prefab({0, -300}),
			prefabs.bullet_destroyer_prefab({0, 300}),
		},
		assets = {
			textures = {
				{
					name = "bullet_green",
					path = "examples/shmup/assets/sprites/bullet_green.png",
				},
				{
					name = "bullet_blue",
					path = "examples/shmup/assets/sprites/bullet_blue.png",
				},
				{
					name = "space",
					path = "examples/shmup/assets/sprites/space.png",
				},
				{
					name = "stars",
					path = "examples/shmup/assets/sprites/stars.png",
				},
				{
					name = "player_straight",
					path = "examples/shmup/assets/sprites/player_straight.png",
				},
				{
					name = "enemy_straight",
					path = "examples/shmup/assets/sprites/enemy_straight.png",
				},
				{
					name = "laser_blue",
					path = "examples/shmup/assets/sprites/laser_blue.png",
				},
			},
		},
	}
}

