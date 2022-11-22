use {
    super::game_object::GameObject,
    ggez::{
        graphics::{Canvas, Color, DrawMode, DrawParam, FillOptions, Mesh, MeshBuilder, Rect},
        input::keyboard::KeyCode,
        Context,
    },
};

pub struct Player {
    body: Rect,
    camera: Rect,
    mesh: Mesh,
}

impl Player {
    pub fn new(ctx: &mut Context) -> Self {
        let (res_x, res_y) = ctx.gfx.size();

        let body = Rect::new(0.0, 0.0, 32.0, 32.0);

        let camera = Rect::new(
            body.x - res_x / 2.0 + body.w / 2.0,
            body.y - res_y / 2.0 + body.h / 2.0,
            res_x,
            res_y,
        );

        let mesh = Mesh::from_data(
            &ctx.gfx,
            MeshBuilder::new()
                .rectangle(
                    DrawMode::Fill(FillOptions::DEFAULT),
                    body.clone(),
                    Color::RED,
                )
                .unwrap()
                .build(),
        );

        Self { body, camera, mesh }
    }
}

impl GameObject for Player {
    fn update(&mut self, ctx: &mut Context, dt: f32) {
        let offset = 100.0 * dt;

        if ctx.keyboard.is_key_pressed(KeyCode::W) {
            self.body.y -= offset;
            self.camera.y -= offset;
        }

        if ctx.keyboard.is_key_pressed(KeyCode::S) {
            self.body.y += offset;
            self.camera.y += offset;
        }

        if ctx.keyboard.is_key_pressed(KeyCode::A) {
            self.body.x -= offset;
            self.camera.x -= offset;
        }

        if ctx.keyboard.is_key_pressed(KeyCode::D) {
            self.body.x += offset;
            self.camera.x += offset;
        }
    }

    fn draw(&mut self, _ctx: &mut Context, canvas: &mut Canvas) {
        canvas.set_screen_coordinates(self.camera.clone());
        canvas.draw(&self.mesh, DrawParam::default().dest(self.body.point()));
    }
}
