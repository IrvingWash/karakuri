// A sprite renderer

#import "Window_Type";
Simp :: #import "Simp";
#import "Math";

// Initializes the sprite renderer
init_sprite_renderer :: (window: Window_Type, clear_color: Vector4) {
    renderer = .{
        window = window,
        clear_color = clear_color,
    };

    Simp.set_render_target(renderer.window);
}

// Sets all the necessary data to begin drawing for the current frame
start_drawing :: () {
    using renderer.clear_color;

    Simp.clear_render_target(x, y, z, w);
}

// Draws a sprite
draw_sprite :: (
    position: Vector3,
    scale: Vector3,
    color: Vector4
) {
    Simp.set_shader_for_color();

    Simp.immediate_quad(
        position.x,
        position.y,
        position.x + scale.x,
        position.y + scale.y,
        color,
    );
}

// Finishes the drawing
finish_drawing :: () {
    Simp.swap_buffers(renderer.window);
}

// Updates window if it was resized or something
update_window :: () {
    Simp.update_window(renderer.window);
}

#scope_file

Sprite_Renderer :: struct {
    window: Window_Type;
    clear_color: Vector4;
}

renderer: Sprite_Renderer = ---;
