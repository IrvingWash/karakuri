#import "Window_Type";
Simp :: #import "Simp";
#import "Math";

init :: (window: Window_Type) {
    renderer = .{
        window = window,
    };

    Simp.set_render_target(renderer.window);
}

start_drawing :: () {
    Simp.clear_render_target(0, 0, 0.7, 1);
}

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

finish_drawing :: () {
    Simp.swap_buffers(renderer.window);
}

update_window :: () {
    Simp.update_window(renderer.window);
}

#scope_file

Sprite_Renderer :: struct {
    window: Window_Type;
}

renderer: Sprite_Renderer = ---;
