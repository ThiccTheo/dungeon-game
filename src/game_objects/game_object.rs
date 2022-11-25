use ggez::{
    graphics::{Canvas, InstanceArray},
    Context,
};

pub trait GameObject {
    fn update(&mut self, ctx: &mut Context, dt: f32);
    fn draw(&mut self, canvas: &mut Canvas, batch: &mut InstanceArray);
    fn texture_id(&self) -> String;
}
