use kmath::Vector2;
use kutils::{Color, Size};
use raylib::{
    color::Color as RaylibColor,
    prelude::{RaylibDraw, RaylibDrawHandle},
    RaylibHandle, RaylibThread,
};

pub struct Renderer {
    thread: RaylibThread,
    clear_color: RaylibColor,
}

impl Renderer {
    pub fn new(thread: RaylibThread, clear_color: Color) -> Renderer {
        Self {
            thread,
            clear_color: RaylibColor::new(
                clear_color.red,
                clear_color.green,
                clear_color.blue,
                clear_color.alpha,
            ),
        }
    }

    pub fn resolution(&self, ctx: &RaylibHandle) -> Size {
        Size {
            width: ctx.get_screen_width() as i64,
            height: ctx.get_screen_height() as i64,
        }
    }

    pub fn start_frame<'a>(&mut self, ctx: &'a mut RaylibHandle) -> RaylibDrawHandle<'a> {
        let mut d = ctx.begin_drawing(&self.thread);

        d.clear_background(self.clear_color);

        d
    }

    pub fn finish_frame(&mut self, d: RaylibDrawHandle) {
        drop(d);
    }

    pub fn draw_rect<'a>(
        &mut self,
        mut d: RaylibDrawHandle<'a>,
        position: &Vector2,
        size: &Size,
        color: &Color,
    ) -> RaylibDrawHandle<'a> {
        let half_width = size.width as f64 * 0.5;
        let half_height = size.height as f64 * 0.5;

        d.draw_rectangle(
            (position.x - half_width) as i32,
            (position.y - half_height) as i32,
            size.width as i32,
            size.height as i32,
            RaylibColor::new(color.red, color.green, color.blue, color.alpha),
        );

        d
    }
}
