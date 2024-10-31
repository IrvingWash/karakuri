use raylib::RaylibHandle;

#[derive(Debug, Default)]
pub struct FpsController {}

impl FpsController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn delta_time(&self, ctx: &RaylibHandle) -> f64 {
        ctx.get_frame_time() as f64
    }
}
