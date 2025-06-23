#import "Basic";

init_world :: () {
    world = .{};
}

set_initial_entities :: (entities: []Entity_Params) {
    for entity: entities {
        array_add(
            *world.entities,
            create_entity(
                .{ id = xx it_index },
                entity,
            ),
        );
    }

    print("%\n", world.entities);
}

#scope_file

World :: struct {
    entities: [..]Entity;
}

world: World = ---;
