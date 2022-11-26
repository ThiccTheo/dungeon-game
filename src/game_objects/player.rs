use {
    super::game_object::GameObject,
    crate::states::app::App,
    ggez::{
        graphics::{Canvas, DrawParam, InstanceArray, Rect},
        input::keyboard::KeyCode,
        mint::Point2,
        Context,
    },
};

pub struct Player {
    body: Rect,
    camera: Rect,
    scale: Point2<f32>,
}

impl Player {
    pub const TEXTURE_ID: &str = "player";

    pub fn new() -> Self {
        let body = Rect::new(0.0, 0.0, 32.0, 32.0);

        let cam = Rect::new(
            body.x - App::WIDTH / 2.0 + body.w / 2.0,
            body.y - App::HEIGHT / 2.0 + body.h / 2.0,
            App::WIDTH,
            App::HEIGHT,
        );

        Self {
            body,
            camera: cam,
            scale: Point2 { x: 1.0, y: 1.0 },
        }
    }
}

impl GameObject for Player {
    fn update(&mut self, ctx: &mut Context, dt: f32) {
        let offset = (100.0 * dt).round();

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
            self.scale.x = -1.0;
        }

        if ctx.keyboard.is_key_pressed(KeyCode::D) {
            self.body.x += offset;
            self.camera.x += offset;
            self.scale.x = 1.0;
        }
    }

    fn draw(&mut self, canvas: &mut Canvas, batch: &mut InstanceArray) {
        canvas.set_screen_coordinates(self.camera.clone());

        batch.push(
            DrawParam::default()
                .dest(if self.scale.x > 0.0 {
                    self.body.point()
                } else {
                    Point2 {
                        x: self.body.right(),
                        y: self.body.y,
                    }
                })
                .scale(self.scale),
        );
    }

    fn texture_id(&self) -> String {
        Self::TEXTURE_ID.to_string()
    }
}
