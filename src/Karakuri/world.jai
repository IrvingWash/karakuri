#scope_module

init_world :: () {
    world = .{};
}

deinit_world :: () {
    destroy_camera(*world.camera);

    for *entity: world.entities {
        if !Maybe.is_some(entity) {
            continue;
        }

        destroy_entity(*entity.some);
    }

    for entity: world.entities_to_spawn {
        destroy_entity_params(entity);
    }

    array_reset(*world.entities_to_despawn);
    array_reset(*world.entities_to_spawn);
    array_reset(*world.entities);
    array_reset(*world.free_tokens);
    array_reset(*world.sprite_components);
    array_reset(*world.behavior_components);
}

world_set_entities :: (scene: Scene) {
    entities: []Entity_Params;
    if scene.load_entities {
        entities = scene.load_entities();
    } else {
        log("Entities loader is not provided in the Scene", flags = .WARNING);
    }

    if !scene.load_camera {
        log("Camera loader is not provided in the Scene", flags = .VERBOSE_ONLY);
    } else {
        world.camera = create_camera(scene.load_camera());
    }

    for entity: entities {
        add_entity(entity);
    }

    for *entity: world.entities {
        start_entity(*entity.some);
    }

    if world.camera.on_start {
        start_camera(*world.camera);
    }

    array_free(entities);
}

world_get_entities :: () -> []Entity {
    return world.entities;
}

world_get_entity_with_token :: (token: Entity_Token) -> ok: bool, *Entity {
    entity := *world.entities[token.id];
    if !Maybe.is_some(entity) {
        return false, null;
    }

    if token == entity.some.token {
        return true, *entity.some;
    }

    return false, null;
}

world_find_entity_with_name :: (name: string) -> ok: bool, *Entity {
    result: *Entity;
    for *entity: world.entities {
        if !Maybe.is_some(entity) {
            continue;
        }

        if entity.some.name == name {
            result = *entity.some;
            break;
        }
    }

    return result != null, result;
}

world_get_component :: (entity: Entity, $Component: Type) -> ok: bool, *Component {
    if !signature_has_component(entity.signature, Component) {
        return false, null;
    }

    #if Component == Sprite_Component {
        return true, *world.sprite_components[entity.id];
    }
    #if Component == Behavior_Component {
        return true, world.behavior_components[entity.id];
    }

    return false, null;
}

world_get_behavior :: (entity: Entity, $B: Type) -> ok: bool, *B
#modify { return uses_behavior_with_as(B); }
{
    ok, behavior := world_get_component(entity, Behavior_Component);

    return ok && behavior != null, cast(*B) behavior;
}

world_find_entities_with_components :: ($Components: ..Type) -> []*Entity {
    signature: Signature;
    for component: Components {
        signature_set_component(*signature, component);
    }

    result: [..]*Entity;
    result.allocator = temp;
    for *entity: world.entities {
        if !Maybe.is_some(entity) {
            continue;
        }

        if signature_is_subset(signature, entity.some.signature) {
            array_add(*result, *entity.some);
        };
    }

    return result;
}

world_find_entities_with_tags :: (tags: ..string) -> []*Entity {
    result: [..]*Entity;
    result.allocator = temp;

    // @Todo this looks very insufficient. Try to optimize with a Table of tag <-> tokens
    for *entity: world.entities {
        if !Maybe.is_some(entity) {
            continue;
        };

        has_all_tags := true;

        for tag: tags {
            if !array_find(entity.some.tags, tag) {
                has_all_tags = false;

                break;
            }
        }

        if has_all_tags {
            array_add(*result, *entity.some);
        }
    }

    return result;
}

world_get_camera :: () -> *Camera {
    return *world.camera;
}

world_spawn_entity :: (entity: Entity_Params) {
    array_add(*world.entities_to_spawn, entity);
}

world_despawn_entity :: (entity: Entity) {
    array_add(*world.entities_to_despawn, entity.token);
}

update_world :: (dt: float) {
    // Destroy scheduled entities
    for token: world.entities_to_despawn {
        ok, entity := world_get_entity_with_token(token);
        assert(ok);

        // @Todo entities destroyed one by one. If one of them wants to the something with the other in on_destroy the things will go badly
        destroy_entity(entity);

        world.entities[entity.token.id] = Maybe.none(Entity);
    }
    array_reset(*world.entities_to_despawn);

    // Create scheduled entities
    spawned_entities_tokens: [..]Entity_Token;
    spawned_entities_tokens.allocator = temp;
    for entity: world.entities_to_spawn {
        token := add_entity(entity);

        array_add(*spawned_entities_tokens, token);
    }
    array_reset(*world.entities_to_spawn);
    for token: spawned_entities_tokens {
        ok, entity := world_get_entity_with_token(token);
        assert(ok);

        start_entity(entity);
    }

    // Update
    for *entity: world.entities {
        if !Maybe.is_some(entity) {
            continue;
        };

        update_entity(*entity.some, dt);
    }

    update_camera(*world.camera, dt);
}

#scope_file

add_entity :: (params: Entity_Params) -> Entity_Token {
    token: Entity_Token = ---;
    if world.free_tokens.count > 0 {
        token = pop(*world.free_tokens);
        token.generation += 1;

        world.entities[token.id] = Maybe.some(create_entity(token, params));
    } else {
        token = .{ id = xx world.entities.count };

        array_add(*world.entities, Maybe.some(create_entity(token, params)));

        array_add(*world.sprite_components);
        array_add(*world.behavior_components);
    }

    ok, entity := world_get_entity_with_token(token);
    assert(ok);

    if Maybe.is_some(params.sprite_component) {
        signature_set_component(*entity.signature, Sprite_Component);

        sprite := params.sprite_component.some;
        // @Todo I don't like using asset manager here
        sprite.texture = Asset_Manager.add_texture(
            sprite.texture_path,
        );

        world.sprite_components[token.id] = sprite;
    }
    if Maybe.is_some(params.behavior_component) {
        signature_set_component(*entity.signature, Behavior_Component);
        world.behavior_components[token.id] = params.behavior_component.some;
    }

    return token;
}

World :: struct {
    entities: [..]Maybe.Maybe(Entity);
    free_tokens: [..]Entity_Token;
    entities_to_spawn: [..]Entity_Params;
    entities_to_despawn: [..]Entity_Token;

    sprite_components: [..]Sprite_Component;
    behavior_components: [..]*Behavior_Component;

    camera: Camera;
}

world: World = ---;
