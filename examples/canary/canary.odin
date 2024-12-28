package canary

import "karakuri:kcore/game"

main :: proc() {
	my_game := game.new_game("Sonic")
	defer game.destroy_game()

	game.start(&my_game)
}
