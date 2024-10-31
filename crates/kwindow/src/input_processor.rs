use raylib::{consts::KeyboardKey, RaylibHandle};

pub struct InputProcessor {}

impl InputProcessor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn should_close(&self, ctx: &RaylibHandle) -> bool {
        ctx.window_should_close()
    }

    pub fn is_pressed(&self, key: KeyboardKey, ctx: &RaylibHandle) -> bool {
        ctx.is_key_pressed(key)
    }

    pub fn is_down(&self, key: KeyboardKey, ctx: &RaylibHandle) -> bool {
        ctx.is_key_down(key)
    }
}
