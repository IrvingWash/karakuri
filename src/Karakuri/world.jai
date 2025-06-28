#scope_module

init_world :: () {
    world = .{};
}

deinit_world :: () {
    for entity: world.entities {
        destroy_entity(entity);
    }

    for behavior: world.behavior_components {
        if behavior != null {
            free(behavior);
        }
    }

    array_reset(*world.entities);
    array_reset(*world.free_tokens);
    array_reset(*world.sprite_components);
    array_reset(*world.behavior_components);
}

world_set_entities :: (entities: []Entity_Params) {
    for entity: entities {
        add_entity(entity);
    }

    for *entity: world.entities {
        start_entity(entity);
    }

    array_free(entities);
}

world_get_entities :: () -> []Entity {
    return world.entities;
}

world_get_entity_with_token :: (token: Entity_Token) -> ok: bool, *Entity {
    entity := *world.entities[token.id];

    if token == entity.token {
        return true, entity;
    }

    return false, null;
}

world_get_entity_with_name :: (name: string) -> ok: bool, *Entity {
    result: *Entity;
    for *entity: world.entities {
        if entity.name == name {
            result = entity;
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
        return true, *world.behavior_components[entity.id];
    }

    return false, null;
}

world_get_downcasted_behavior :: (entity: Entity, $B: Type) -> ok: bool, *B
#modify { return uses_behavior_with_as(B); }
{
    result := cast(*B) world.behavior_components[entity.id];

    return result != null, result;
}

world_get_entities_with_components :: ($Components: ..Type) -> []*Entity {
    signature: Signature;
    for component: Components {
        signature_set_component(*signature, component);
    }

    result: [..]*Entity;
    result.allocator = temp;
    for *entity: world.entities {
        if signature_is_subset(signature, entity.signature) {
            array_add(*result, entity);
        };
    }

    return result;
}

world_get_entities_with_tag :: (tag: string) -> []*Entity {
    result: [..]*Entity;
    result.allocator = temp;

    for *entity: world.entities {
        if array_find(entity.tags, tag) {
            array_add(*result, entity);
        }
    }

    return result;
}

world_get_entities_with_tags :: (tags: ..string) -> []*Entity {
    result: [..]*Entity;
    result.allocator = temp;

    // @Todo this looks very insufficient. Try to optimize with a Table of tag <-> tokens
    for *entity: world.entities {
        has_all_tags := true;

        for tag: tags {
            if !array_find(entity.tags, tag) {
                has_all_tags = false;

                break;
            }
        }

        if has_all_tags {
            array_add(*result, entity);
        }
    }

    return result;
}

update_world :: (dt: float) {
    for *entity: world.entities {
        update_entity(entity, dt);
    }
}

#scope_file

add_entity :: (params: Entity_Params) {
    token: Entity_Token = ---;
    if world.free_tokens.count > 0 {
        token = pop(*world.free_tokens);
        token.generation += 1;

        world.entities[token.id] = create_entity(token, params);
    } else {
        token = .{ id = xx world.entities.count };

        array_add(*world.entities, create_entity(token, params));

        array_add(*world.sprite_components);
        array_add(*world.behavior_components);
    }

    _, entity := world_get_entity_with_token(token);

    if Maybe.is_some(params.sprite_component) {
        signature_set_component(*entity.signature, Sprite_Component);
        world.sprite_components[token.id] = params.sprite_component.some;
    }
    if Maybe.is_some(params.behavior_component) {
        signature_set_component(*entity.signature, Behavior_Component);
        world.behavior_components[token.id] = params.behavior_component.some;
    }
}

World :: struct {
    entities: [..]Entity;
    free_tokens: [..]Entity_Token;

    sprite_components: [..]Sprite_Component;
    behavior_components: [..]*Behavior_Component;
}

world: World = ---;
