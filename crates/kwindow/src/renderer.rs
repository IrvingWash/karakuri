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
    pub fn new(thread: RaylibThread, clear_color: &Color) -> Renderer {
        Self {
            thread,
            clear_color: color_to_raylib(clear_color),
        }
    }

    pub fn resolution(&self, ctx: &RaylibHandle) -> Size {
        Size {
            width: i64::from(ctx.get_screen_width()),
            height: i64::from(ctx.get_screen_height()),
        }
    }

    pub fn start_frame<'a>(&self, ctx: &'a mut RaylibHandle) -> RaylibDrawHandle<'a> {
        let mut d = ctx.begin_drawing(&self.thread);

        d.clear_background(self.clear_color);

        d
    }

    pub fn finish_frame(&self, d: RaylibDrawHandle) {
        drop(d);
    }

    pub fn draw_rect(
        &self,
        d: &mut RaylibDrawHandle,
        position: &Vector2,
        size: &Size,
        color: &Color,
    ) {
        let half_width = size.width as f64 * 0.5;
        let half_height = size.height as f64 * 0.5;

        d.draw_rectangle(
            (position.x - half_width) as i32,
            (position.y - half_height) as i32,
            size.width as i32,
            size.height as i32,
            color_to_raylib(color),
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_texture(
        &self,
        d: &mut RaylibDrawHandle,
        texture: &Texture2D,
        source_position: &Vector2,
        source_size: &Size,
        dest_position: &Vector2,
        scale: &Vector2,
        origin: &Vector2,
        rotation: f64,
        tint: &Color,
    ) {
        let dest_width = texture.width * scale.x as i32;
        let dest_height = texture.height * scale.y as i32;

        d.draw_texture_pro(
            texture,
            make_rectangle(source_position, source_size),
            make_rectangle(
                dest_position,
                &Size::new(i64::from(dest_width), i64::from(dest_height)),
            ),
            vector2_to_raylib(origin),
            rotation as f32,
            color_to_raylib(tint),
        );
    }
}

fn color_to_raylib(color: &Color) -> RaylibColor {
    RaylibColor::from(color.to_tuple())
}

fn vector2_to_raylib(vector2: &Vector2) -> RaylibVector2 {
    RaylibVector2::new(vector2.x as f32, vector2.y as f32)
}

fn make_rectangle(position: &Vector2, size: &Size) -> Rectangle {
    Rectangle {
        x: position.x as f32,
        y: position.y as f32,
        width: size.width as f32,
        height: size.height as f32,
    }
}
