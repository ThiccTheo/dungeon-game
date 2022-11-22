use {
    super::state::Action,
    crate::game_objects::player::Player,
    ggez::{
        event::EventHandler,
        graphics::{Canvas, Color},
        Context,
    },
};

pub struct Game {
    player: Player,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            player: Player::new(ctx),
        }
    }
}

impl EventHandler<Action> for Game {
    fn update(&mut self, ctx: &mut Context) -> Result<(), Action> {
        let dt = ctx.time.delta().as_secs_f32();
        self.player.update(ctx, dt);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), Action> {
        let mut canvas = Canvas::from_frame(&ctx.gfx, Color::BLACK);
        self.player.draw(ctx, &mut canvas);
        canvas.finish(&mut ctx.gfx).unwrap();
        Ok(())
    }
}
