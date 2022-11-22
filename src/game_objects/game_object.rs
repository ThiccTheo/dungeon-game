use ggez::{graphics::Canvas, Context};

pub trait GameObject {
    fn update(&mut self, ctx: &mut Context, dt: f32);
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas);
}
