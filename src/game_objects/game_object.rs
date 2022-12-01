use {
    ggez::{
        graphics::{Canvas, InstanceArray},
        Context,
    },
    std::{iter::Chain, slice::IterMut},
};

pub trait GameObject {
    fn update(&mut self, ctx: &mut Context, dt: f32, others: View);
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas, batch: &mut InstanceArray);
    fn id(&self) -> &'static str;
}

pub type View<'a> = Chain<IterMut<'a, Box<dyn GameObject>>, IterMut<'a, Box<dyn GameObject>>>;

pub fn create_view(
    game_objects: &mut [Box<dyn GameObject>],
    idx: usize,
) -> (&mut Box<dyn GameObject>, View) {
    let (before, tmp) = game_objects.split_at_mut(idx);
    let (this, after) = tmp.split_first_mut().unwrap();
    let others = before.iter_mut().chain(after.iter_mut());
    (this, others)
}
