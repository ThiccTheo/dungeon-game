use ggez::{
    graphics::{Canvas, Image},
    Context,
};

pub trait GameObject {
    fn update(&mut self, ctx: &mut Context, dt: f32);
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas, texture: &Image);
    fn texture_id(&self) -> String;
}
