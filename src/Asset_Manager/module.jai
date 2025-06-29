#import "Hash_Table";
#import "Sprite_Renderer";

init :: () {
    manager = .{};
}

deinit :: () {
    for manager.textures {
        destroy_texture(it);
    }

    deinit(*manager.textures);
}

add_texture :: (path: string) -> Texture {
    ok, texture := table_find_new(*manager.textures, path);
    if ok {
        return texture;
    }

    texture = create_texture(path);

    table_set(*manager.textures, path, texture);

    return texture;
}

get_texture :: (path: string) -> ok: bool, texture: Texture {
    ok, texture := table_find_new(*manager.textures, path);

    return ok, texture;
}

#scope_module

Asset_Manager :: struct {
    textures: Table(string, Texture);
}

manager: Asset_Manager = ---;
