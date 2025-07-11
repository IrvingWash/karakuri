#import "Basic";
#import "GL";
#import "stb_image";

Texture :: struct {
    id:     u32;
    width:  u32;
    height: u32;
}

create_texture :: (image_path: string, flip := false) -> Texture {
    texture: Texture;

    channels: s32;
    c_image_path := temp_c_string(image_path);
    stbi_set_flip_vertically_on_load(xx flip);
    data := stbi_load(c_image_path, xx *texture.width, xx *texture.height, *channels, 0);
    if !data {
        log_error("Failed to load image: %\n", image_path);
    }
    defer stbi_image_free(data);

    glGenTextures(1, *texture.id);

    glBindTexture(GL_TEXTURE_2D, texture.id);

    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);

    // @Todo this is bad
    format := ifx channels == 3
        then GL_RGB
        else GL_RGBA;

    glTexImage2D(
        GL_TEXTURE_2D,
        0, // The mipmap level for which we are generating a texture
        xx format, // internal format
        xx texture.width,
        xx texture.height,
        0, // always 0
        xx format, // format
        GL_UNSIGNED_BYTE, // data type
        data,
    );

    glGenerateMipmap(GL_TEXTURE_2D);

    glBindTexture(GL_TEXTURE_2D, 0);

    return texture;
}

destroy_texture :: (texture: Texture) {
    glDeleteTextures(1, *texture.id);
}

#scope_module

bind_texture :: (texture: Texture) {
    glBindTexture(GL_TEXTURE_2D, texture.id);
}
