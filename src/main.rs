use ggez::event::{self, EventHandler};
use ggez::graphics::Color;
use ggez::{graphics, Context, ContextBuilder};

struct Game {}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        Game {}
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        canvas.finish(ctx)
    }
}

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("krakie", "Rico Bakker & Jari Maijenburg")
        .build()
        .expect("could not create ggez context!");

    let game = Game::new(&mut ctx);

    event::run(ctx, event_loop, game);
}
