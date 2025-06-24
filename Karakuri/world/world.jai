#import "Basic";

init_world :: () {
    world = .{};
}

set_initial_entities :: (entities: [..]Entity_Params) {
    for entity: entities {
        array_add(
            *world.entities,
            create_entity(
                .{ id = xx it_index },
                entity,
            ),
        );
    }

    for *entity: world.entities {
        if entity.on_start != null {
            entity.on_start(entity);
        }
    }
}

entities :: () -> []Entity {
    return world.entities;
}

update :: (dt: float) {
    for *entity: world.entities {
        if entity.on_update != null {
            entity.on_update(entity, dt);
        }
    }
}

#scope_file

World :: struct {
    entities: [..]Entity;
}

world: World = ---;
