use raylib::RaylibHandle;

#[derive(Debug, Default)]
pub struct FpsController {}

impl FpsController {
    #[inline]
    pub fn new() -> Self {
        Self {}
    }

    #[inline]
    pub fn delta_time(&self, ctx: &RaylibHandle) -> f64 {
        f64::from(ctx.get_frame_time() * 10.0)
    }

    #[inline]
    pub fn time(&self, ctx: &RaylibHandle) -> f64 {
        ctx.get_time() * 1000.0
    }
}
