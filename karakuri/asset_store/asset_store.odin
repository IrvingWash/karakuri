package karakuri_asset_store

import "core:strings"
import rl "vendor:raylib"
import "kwindow:renderer"

@(private = "file")
Asset_Store :: struct {
	textures: map[string]renderer.Texture,
}

@(private = "file")
asset_store := Asset_Store{}

init :: proc() {
	asset_store = Asset_Store {
		textures = make(map[string]renderer.Texture, 100),
	}
}

deinit :: proc() {
	for _, &texture in asset_store.textures {
		rl.UnloadTexture(texture)
	}

	delete(asset_store.textures)
}

load_texture :: proc(name: string, path: string) {
	if name in asset_store.textures {
		return
	}

	path_raw := strings.clone_to_cstring(path)
	defer delete(path_raw)

	texture := rl.LoadTexture(path_raw)

	map_insert(&asset_store.textures, name, texture)
}

get_texture :: proc(name: string) -> renderer.Texture {
	return asset_store.textures[name]
}
