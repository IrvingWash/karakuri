package canary

import c "karakuri:kcore/components"
import "karakuri:kcore/game"
import "karakuri:kec"
import ku "karakuri:kutils"

main :: proc() {
	my_game := game.new_game("Sonic")
	defer game.destroy_game()

	player := kec.create_entity(&my_game.registry)
	kec.add_component(
		&my_game.registry,
		player,
		c.new_transform_component({100, 100}, {2, 2}, 45),
	)
	kec.add_component(
		&my_game.registry,
		player,
		c.Shape_Component{10, 20, ku.ColorRed},
	)

	game.start(&my_game)
}
