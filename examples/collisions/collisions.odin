package example_collisions

@(require) import "core:fmt"
@(require) import "core:log"
@(require) import "core:mem"
import "karakuri:game"
import "kutils:color"
import "scenes"

main :: proc() {
	// =============================
	// Memory tracking
	// =============================
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

	// =============================
	// The actual code
	// =============================
	game.init(
		title = "Collisions",
		width = 800,
		height = 600,
		fullscreen = false,
		resizable = false,
		vsync = true,
		background_color = color.Black,
		target_fps = 60,
	)
	defer game.destroy()

	game.set_scene(scenes.make_main_scene)

	game.start()
}

