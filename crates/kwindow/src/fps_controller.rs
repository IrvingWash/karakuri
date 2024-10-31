use raylib::RaylibHandle;

pub struct FpsController {}

impl FpsController {
    pub fn new(target_fps: u32) -> Self {
        Self {}
    }

    pub fn delta_time(&self, ctx: &RaylibHandle) -> f64 {
        ctx.get_frame_time() as f64
    }
}
