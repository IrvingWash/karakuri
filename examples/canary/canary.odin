package canary

import "karakuri:karakuri"
import ku "karakuri:kutils"

main :: proc() {
	game := karakuri.new_game(
		title = "Sonic The Hedgehog",
		width = 800,
		height = 600,
		clear_color = ku.ColorWhite,
		fullscreen = false,
		vsync = true,
	)

	karakuri.destroy_game(game)
}
