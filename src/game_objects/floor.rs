use {
    super::game_object::GameObject,
    ggez::{
        graphics::{Canvas, DrawParam, InstanceArray, Rect},
        Context,
    },
};

pub struct Floor {
    body: Rect,
}

impl Floor {
    pub const TEXTURE_ID: &str = "floor";

    pub fn new(x: usize, y: usize) -> Self {
        Self {
            body: Rect::new(x as f32 * 32.0, y as f32 * 32.0, 32.0, 32.0),
        }
    }
}

impl GameObject for Floor {
    fn update(&mut self, _ctx: &mut Context, _dt: f32) {}

    fn draw(&mut self, canvas: &mut Canvas, batch: &mut InstanceArray) {
        let sub_rect = batch.image().uv_rect(0, 0, 32, 32);
        batch.push(DrawParam::default().dest(self.body.point()).src(sub_rect));
    }

    fn texture_id(&self) -> String {
        Self::TEXTURE_ID.to_string()
    }
}
