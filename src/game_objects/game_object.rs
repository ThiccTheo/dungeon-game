use ggez::{
    graphics::{Canvas, Image},
    Context,
};

pub trait GameObject {
    fn update(&mut self, ctx: &mut Context, dt: f32);
    fn draw(&mut self, canvas: &mut Canvas, img: &Image);
    fn texture_id(&self) -> String;
}
