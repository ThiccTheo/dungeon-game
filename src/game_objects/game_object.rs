use std::{iter::Chain, slice::IterMut};

use ggez::{
    graphics::{Canvas, InstanceArray},
    Context,
};

pub trait GameObject {
    fn update(&mut self, ctx: &mut Context, dt: f32, others: Chain<IterMut<Box<dyn GameObject>>, IterMut<Box<dyn GameObject>>>);
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas, batch: &mut InstanceArray);
    fn id(&self) -> String;
}
