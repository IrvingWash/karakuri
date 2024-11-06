use kwindow::{InputProcessor, KeyboardKey, WindowCtx};

#[derive(Debug)]
pub struct InputProcessorAdapter<'a> {
    input_processor: &'a InputProcessor,
    ctx: &'a WindowCtx,
}

impl<'a> InputProcessorAdapter<'a> {
    #[inline]
    pub const fn new(input_processor: &'a InputProcessor, ctx: &'a WindowCtx) -> Self {
        Self {
            input_processor,
            ctx,
        }
    }

    #[inline]
    pub fn should_close(&self) -> bool {
        self.input_processor.should_close(self.ctx)
    }

    #[inline]
    pub fn is_pressed(&self, key: KeyboardKey) -> bool {
        self.input_processor.is_pressed(key, self.ctx)
    }

    #[inline]
    pub fn is_down(&self, key: KeyboardKey) -> bool {
        self.input_processor.is_down(key, self.ctx)
    }
}
