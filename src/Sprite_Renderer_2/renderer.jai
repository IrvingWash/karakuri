#scope_module

// A sprite renderer

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
    glEnable(GL_CULL_FACE);
}

// Sets all the necessary data to begin drawing for the current frame
start_drawing :: () {
    using renderer.clear_color;

    glClearColor(x, y, z, w);
    glClear(GL_COLOR_BUFFER_BIT);
}

// Draws a sprite
draw :: (
    position: Vector2,
    scale: Vector2,
    color: Vector4,
    origin: Vector2
) {}

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
