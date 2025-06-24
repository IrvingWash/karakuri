#scope_module

#import "Maybe";

init_world :: () {
    world = .{};
}

deinit_world :: () {
    for behavior: world.behavior_components {
        if behavior != null {
            free(behavior);
        }
    }

    array_reset(entities);
    array_reset(free_tokens);
    array_reset(sprite_components);
    array_reset(behavior_components);
}

set_entities :: (entities: []Entity_Params) { // @Todo maybe we'll need to change this to [..]
    for entity: entities {
        add_entity(entity);
    }

    for *entity: world.entities {
        start_entity(entity);
    }
}

get_entities :: () -> []Entity {
    return world.entities;
}

get_component :: (entity: Entity, $Component: Type) -> *Component {
    #if Component == Sprite_Component {
        return *world.sprite_components[entity.id];
    } else #if Component == Behavior_Component {
        return *world.behavior_components[entity.id];
    }

    return null;
}

update_world :: (dt: float) {
    for *entity: world.entities {
        update_entity(entity, dt);
    }
}

#scope_file

add_entity :: (params: Entity_Params) {
    if world.free_tokens.count > 0 {
        token := pop(*world.free_tokens);
        token.generation += 1;

        world.entities[token.id] = create_entity(token, params);

        if is_some(params.sprite_component) {
            world.sprite_components[token.id] = params.sprite_component.some;
        }
        if is_some(params.behavior_component) {
            world.behavior_components[token.id] = params.behavior_component.some;
        }
    } else {
        token: Entity_Token = .{ id = xx world.entities.count };

        array_add(*world.entities, create_entity(token, params));

        array_add(*world.sprite_components);
        array_add(*world.behavior_components);
    }
}

World :: struct {
    entities: [..]Entity;
    free_tokens: [..]Entity_Token;

    sprite_components: [..]Sprite_Component;
    behavior_components: [..]Behavior_Component;
}

world: World = ---;
