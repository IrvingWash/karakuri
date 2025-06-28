#module_parameters()(DEBUG := false);

Maybe :: #import "Maybe";

#load "engine.jai";
#load "components.jai";
#load "entity.jai";
#load "scene.jai";

get_component :: world_get_component;
get_behavior :: world_get_downcasted_behavior;
get_entity_with_name :: world_get_entity_with_name;
get_entity_with_token :: world_get_entity_with_token;
get_entities_with_tag :: world_get_entities_with_tag;
get_entities_with_tags :: world_get_entities_with_tags;

is_key_pressed :: Input_Manager.is_key_pressed;
is_key_down :: Input_Manager.is_key_down;
is_key_start :: Input_Manager.is_key_start;
is_key_end :: Input_Manager.is_key_end;
is_key_up :: Input_Manager.is_key_up;

get_render_size :: Sprite_Renderer.get_render_dimensions;

#scope_module

#load "signature.jai";
#load "world.jai";
