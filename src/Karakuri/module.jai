#module_parameters()(DEBUG := false, UPDATE_MEMORY_VISUALIZER := false);

Maybe :: #import "Maybe";

#load "engine.jai";
#load "components.jai";
#load "entity.jai";
#load "scene.jai";
#load "math.jai";

get_component:           : world_get_component;
get_behavior:            : world_get_behavior;
get_entity_with_token:   : world_get_entity_with_token;
find_entity_with_name:   : world_find_entity_with_name;
find_entities_with_tags: : world_find_entities_with_tags;
spawn_entity:            : world_spawn_entity;
despawn_entity:          : world_despawn_entity;

is_key_pressed: : Input_Manager.is_key_pressed;
is_key_down:    : Input_Manager.is_key_down;
is_key_start:   : Input_Manager.is_key_start;
is_key_end:     : Input_Manager.is_key_end;
is_key_up:      : Input_Manager.is_key_up;

get_render_size :: Sprite_Renderer.get_render_dimensions;

set_timeout:    : Timer.set_timeout;
clear_interval: : Timer.clear_interval;
set_interval:   : Timer.set_interval;
clear_interval: : Timer.clear_interval;

#scope_module

Asset_Manager :: #import "Asset_Manager";
#load "signature.jai";
#load "world.jai";
