use kmath::Vector2;
use kutils::{Color, Size};
use raylib::{
    color::Color as RaylibColor,
    math::{Rectangle, Vector2 as RaylibVector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    texture::Texture2D,
    RaylibHandle, RaylibThread,
};

#[derive(Debug)]
pub struct Renderer {
    thread: RaylibThread,
    clear_color: RaylibColor,
}

impl Renderer {
    #[inline]
    pub fn new(thread: RaylibThread, clear_color: &Color) -> Renderer {
        Self {
            thread,
            clear_color: color_to_raylib(clear_color),
        }
    }

    #[inline]
    pub fn resolution(&self, ctx: &RaylibHandle) -> Size {
        Size {
            width: i64::from(ctx.get_screen_width()),
            height: i64::from(ctx.get_screen_height()),
        }
    }

    #[inline]
    pub fn start_frame<'a>(&self, ctx: &'a mut RaylibHandle) -> RaylibDrawHandle<'a> {
        let mut d = ctx.begin_drawing(&self.thread);

        d.clear_background(self.clear_color);

        d
    }

    #[inline]
    pub fn finish_frame(&self, d: RaylibDrawHandle) {
        drop(d);
    }

    #[inline]
    pub fn draw_rect(
        &self,
        d: &mut RaylibDrawHandle,
        position: &Vector2,
        size: &Vector2,
        color: &Color,
    ) {
        let x = position.x as i32;
        let y = position.y as i32;
        let width = size.x as i32;
        let height = size.y as i32;
        let color = color_to_raylib(color);

        d.draw_rectangle_lines(x, y, width, height, color);
    }

    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn draw_texture(
        &self,
        d: &mut RaylibDrawHandle,
        texture: &Texture2D,
        source_position: &Vector2,
        source_size: &Vector2,
        dest_position: &Vector2,
        dest_size: &Vector2,
        origin: &Vector2,
        rotation: f64,
        tint: &Color,
    ) {
        d.draw_texture_pro(
            texture,
            make_rectangle(source_position, source_size),
            make_rectangle(
                &Vector2::new(dest_position.x, dest_position.y),
                &Vector2::new(dest_size.x, dest_size.y),
            ),
            vector2_to_raylib(origin),
            rotation as f32,
            color_to_raylib(tint),
        );
    }

    #[inline]
    pub fn draw_text(
        &self,
        d: &mut RaylibDrawHandle,
        text: &str,
        position: &Vector2,
        font_size: usize,
        color: &Color,
    ) {
        d.draw_text(
            text,
            position.x as i32,
            position.y as i32,
            font_size as i32,
            color_to_raylib(color),
        );
    }
}

fn color_to_raylib(color: &Color) -> RaylibColor {
    RaylibColor::from(color.to_tuple())
}

const fn vector2_to_raylib(vector2: &Vector2) -> RaylibVector2 {
    RaylibVector2::new(vector2.x as f32, vector2.y as f32)
}

const fn make_rectangle(position: &Vector2, size: &Vector2) -> Rectangle {
    Rectangle {
        x: position.x as f32,
        y: position.y as f32,
        width: size.x as f32,
        height: size.y as f32,
    }
}
