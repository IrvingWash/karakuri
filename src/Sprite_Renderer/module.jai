// A sprite renderer

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
}

deinit :: () {}

// Sets all the necessary data to begin drawing for the current frame
start_drawing :: () {
    using renderer.clear_color;

    Simp.clear_render_target(x, y, z, w);
}

// Draws a sprite
draw :: (
    position: Vector2,
    scale: Vector2,
    rotation: float,
    origin: Vector2,
    color: Vector4
) {
    Simp.set_shader_for_color();

    x0 := position.x - scale.x * origin.x;
    y0 := position.y + scale.y * origin.y;

    Simp.immediate_quad(
        x0,
        y0,
        x0 + scale.x,
        y0 - scale.y,
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

get_render_dimensions :: () -> u32, u32 {
    x, y := Simp.get_render_dimensions(renderer.window);

    return xx x, xx y;
}

#scope_file

Sprite_Renderer :: struct {
    window: Window_Type;
    clear_color: Vector4;
}

renderer: Sprite_Renderer = ---;
