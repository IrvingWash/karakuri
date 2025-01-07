package example_canary

import "core:fmt"
import "karakuri:entity"

Player :: struct {
	using entity: entity.Entity,
	hp:           uint,
}

main :: proc() {
	entity.init_manager()
	defer entity.deinit_manager()

	player := new(Player)
	player^ = Player {
		transform = {position = {300, 50}},
		hp = 100,
	}

	player_id := entity.create_entity(player)

	player_gotten := entity.get_entity(player_id, Player)

	if p, ok := player_gotten.?; ok {
		fmt.println(p.transform.position)
		fmt.println(p.hp)
	}
}

