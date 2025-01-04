package shmup

@(require) import "core:fmt"
@(require) import "core:log"
@(require) import "core:mem"
import karakuri "karakuri:karakuri/game"
import "karakuri:kutils"

DODONPACHI_WIDTH :: 240
DODONPACHI_HEIGHT :: 320

main :: proc() {
	when ODIN_DEBUG {
		context.logger = log.create_console_logger(lowest = log.Level.Debug)
		defer log.destroy_console_logger(context.logger)

		track: mem.Tracking_Allocator
		mem.tracking_allocator_init(&track, context.allocator)
		context.allocator = mem.tracking_allocator(&track)

		defer {
			if len(track.allocation_map) > 0 {
				fmt.eprintf(
					"=== %v allocations not freed: ===\n",
					len(track.allocation_map),
				)
				for _, entry in track.allocation_map {
					fmt.eprintf(
						"- %v bytes @ %v\n",
						entry.size,
						entry.location,
					)
				}
			}
			if len(track.bad_free_array) > 0 {
				fmt.eprintf(
					"=== %v incorrect frees: ===\n",
					len(track.bad_free_array),
				)
				for entry in track.bad_free_array {
					fmt.eprintf("- %p @ %v\n", entry.memory, entry.location)
				}
			}
			mem.tracking_allocator_destroy(&track)
		}
	}

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

	level_1 := karakuri.create_scene(
		{
			player_prefab(),
			enemy_spawner_prefab(),
			projectile_destroyer_prefab({0, -DODONPACHI_HEIGHT + 10}),
			projectile_destroyer_prefab({0, DODONPACHI_HEIGHT - 10}),
		},
	)

	karakuri.start_scene(&shmup_game, &level_1)
}

