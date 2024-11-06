use raylib::{consts::KeyboardKey, RaylibHandle};

#[derive(Debug, Default)]
pub struct InputProcessor {}

impl InputProcessor {
    #[inline]
    pub fn new() -> Self {
        Self {}
    }

    #[inline]
    pub fn should_close(&self, ctx: &RaylibHandle) -> bool {
        ctx.window_should_close()
    }

    #[inline]
    pub fn is_pressed(&self, key: KeyboardKey, ctx: &RaylibHandle) -> bool {
        ctx.is_key_pressed(key)
    }

    #[inline]
    pub fn is_down(&self, key: KeyboardKey, ctx: &RaylibHandle) -> bool {
        ctx.is_key_down(key)
    }
}
