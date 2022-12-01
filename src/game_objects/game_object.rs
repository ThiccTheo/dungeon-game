use {
    ggez::{
        graphics::{Canvas, InstanceArray},
        Context,
    },
    std::{iter::Chain, slice::IterMut},
};

pub type View<'a> = Chain<IterMut<'a, Box<dyn GameObject>>, IterMut<'a, Box<dyn GameObject>>>;

pub trait GameObject {
    fn update(&mut self, ctx: &mut Context, dt: f32, others: View);
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas, batch: &mut InstanceArray);
    fn id(&self) -> &'static str;
}
