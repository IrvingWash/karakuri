// A sprite renderer

#import "Basic";
#import "GL";
#import "Window_Type";
Simp :: #import "Simp";
#import "Math";

// Initializes the sprite renderer
init :: (window: Window_Type, clear_color: Vector4) {
    renderer = .{
        window = window,
        clear_color = clear_color,
    };

    Simp.set_render_target(renderer.window);

    renderer.projection_matrix = make_projection_matrix();

    renderer.shader = create_shader(VERTEX_TEXTURE, FRAGMENT_TEXTURE);
    use_shader(renderer.shader);
    set_uniform(renderer.shader, "texture1", 0);

    renderer.mesh = create_mesh();
}

deinit :: () {
    destroy_shader(renderer.shader);
    destroy_mesh(renderer.mesh);
}

// Sets all the necessary data to begin drawing for the current frame
start_drawing :: (view_matrix: Matrix4) {
    using renderer.clear_color;

    glClearColor(x, y, z, w);
    glClear(GL_COLOR_BUFFER_BIT);

    use_shader(renderer.shader);
    set_uniform(renderer.shader, "projection", renderer.projection_matrix);
    set_uniform(renderer.shader, "view", view_matrix);

    glBindVertexArray(renderer.mesh.vao);
}

// Draws a sprite
draw :: (
    position: Vector2,
    scale: Vector2,
    rotation: float,
    origin: Vector2, // @Todo use origin
    color: Vector4,
    texture: Texture
) {
    glActiveTexture(GL_TEXTURE0);
    bind_texture(texture);

    translation_matrix := make_translation_matrix4(Vector3.{position.x, position.y, 1});
    scale_matrix := make_scale_matrix4(Vector3.{scale.x, scale.y, 1});
    orientation: Quaternion;
    set_from_axis_and_angle(*orientation, .{0, 0, 1}, rotation);
    _rotation_matrix := rotation_matrix(
        Matrix4,
        orientation,
    );

    model_transform := translation_matrix * scale_matrix * _rotation_matrix;
    set_uniform(renderer.shader, "model", model_transform);
    set_uniform(renderer.shader, "color", ..color.component);

    glDrawElements(
        GL_TRIANGLES,
        renderer.mesh.indices.count,
        GL_UNSIGNED_INT,
        cast(*void) 0,
    );
}

// Finishes the drawing
finish_drawing :: () {
    Simp.swap_buffers(renderer.window);
}

// Updates window if it was resized or something
update_window :: () {
    Simp.update_window(renderer.window);

    renderer.projection_matrix = make_projection_matrix();
}

get_render_dimensions :: () -> u32, u32 {
    x, y := Simp.get_render_dimensions(renderer.window);

    return xx x, xx y;
}

#scope_file

Sprite_Renderer :: struct {
    window: Window_Type;
    clear_color: Vector4;
    projection_matrix: Matrix4;
    shader: Shader;
    mesh: Quad_Mesh; // @Todo meshes per texture
}

renderer: Sprite_Renderer = ---;

make_projection_matrix :: () -> Matrix4 {
    width, height := get_render_dimensions();

    return orthographic_projection_matrix(
        -(width / 2.0),
        width / 2.0,
        -(height / 2.0),
        height / 2.0,
        -1,
        1,
    );
}

#scope_module

#load "shader.jai";
#load "texture.jai";
#load "quad_mesh.jai";
#load "shaders/vertex_texture.jai";
#load "shaders/fragment_texture.jai";
