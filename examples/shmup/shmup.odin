package shmup

import karakuri "karakuri:karakuri/game"
import "karakuri:kutils"

DODONPACHI_WIDTH :: 240
DODONPACHI_HEIGHT :: 320

main :: proc() {
	shmup_game := karakuri.new_game(
		title = "Shmup",
		width = DODONPACHI_WIDTH * 2,
		height = DODONPACHI_HEIGHT * 2,
		clear_color = kutils.ColorBlack,
		fullscreen = false,
		vsync = true,
		target_fps = 60,
	)
	defer karakuri.destroy_game(shmup_game)

	level_1 := karakuri.create_scene({player_prefub(), projectile_destroyer_prefub({0, -DODONPACHI_HEIGHT})})

	karakuri.start_scene(&shmup_game, &level_1)
}
