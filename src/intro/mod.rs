extern crate ggez;
use ggez::graphics;
use ggez::nalgebra;
use ggez::timer;
use ggez::*;

use rand::Rng;

pub struct State {
    vertigo_angle: f32,

    stars: graphics::Mesh,

    title: graphics::Text,
}

impl State {
    pub fn new(ctx: &mut Context) -> GameResult<State> {
        // Generate stars
        let mut rng = rand::thread_rng();

        let mut mesh_builder = graphics::MeshBuilder::new();

        for _x in 1..=320 {
            let star_color = graphics::Color::from_rgb(
                rng.gen_range(0, 10),
                rng.gen_range(10, 50),
                rng.gen_range(51, 255),
            );

            mesh_builder.circle(
                graphics::DrawMode::fill(),
                nalgebra::Point2::new(rng.gen_range(0.0, 640.0), rng.gen_range(0.0, 640.0)),
                rng.gen_range(0.0, 3.0),
                1.0,
                star_color,
            );
        }

        // Text initialization
        let text = graphics::Text::new("Rusty Gorillas");

        let s = State {
            vertigo_angle: 0.0,
            stars: mesh_builder.build(ctx)?,
            title: text,
        };

        Ok(s)
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Delta time for hardware independent speed
        let delta = timer::delta(ctx);
        let delta_millis = (delta.as_secs() as f64 + f64::from(delta.subsec_millis()) * 0.001) as f32; // TODO: Create helper function for it

        self.vertigo_angle = self.vertigo_angle % 360.0 + 11.25 * delta_millis;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Background
        let background_color = graphics::Color::from_rgb(0, 0, 32);
        graphics::clear(ctx, background_color);

        // Stars
        let param = graphics::DrawParam::new()
            .dest(nalgebra::Point2::new(0.0, 0.0))
            .rotation(self.vertigo_angle.to_radians())
            .offset(nalgebra::Point2::new(320.0, 320.0))
            .scale(nalgebra::Vector2::new(1.0, 1.0));

        graphics::draw(ctx, &self.stars, param)?;

        // Text
        graphics::queue_text(ctx, &self.title, nalgebra::Point2::new(0.0, 0.0), None);

        let duration_from_start = timer::time_since_start(ctx);
        let time_from_start_millis = duration_from_start.as_secs() as f64
            + f64::from(duration_from_start.subsec_millis()) * 0.001;

        let bouncing_angle = (time_from_start_millis.sin() * 10.0).to_radians();
        let text_param = graphics::DrawParam::new()
            .dest(nalgebra::Point2::new(320.0, 240.0))
            .rotation(bouncing_angle as f32)
            .offset(nalgebra::Point2::new(50.0, 5.0)); // TODO: Calculate text rectangle and align according to it

        graphics::draw_queued_text(ctx, text_param)?;

        graphics::present(ctx)?;

        Ok(())
    }
}
