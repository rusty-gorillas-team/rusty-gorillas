use ggez::{event, graphics, nalgebra, timer};

use rand::Rng;

pub struct State {
    screen_center: nalgebra::Point2<f32>,

    vertigo_angle: f32,

    stars: graphics::Mesh,

    title: graphics::Text,
}

impl State {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<State> {
        // Generate stars
        let mut rng = rand::thread_rng();

        let mut mesh_builder = graphics::MeshBuilder::new();

        let (window_width, window_height) = graphics::drawable_size(ctx);

        for _ in 0..320 {
            let star_color = graphics::Color::from_rgb(
                rng.gen_range(0, 10),
                rng.gen_range(10, 50),
                rng.gen_range(51, 255),
            );

            mesh_builder.circle(
                graphics::DrawMode::fill(),
                nalgebra::Point2::new(
                    rng.gen_range(0.0, window_width),
                    rng.gen_range(0.0, window_height),
                ),
                rng.gen_range(0.0, 3.0),
                1.0,
                star_color,
            );
        }

        // Text initialization
        let mut text = graphics::Text::new("Rusty Gorillas");
        text.set_bounds(
            nalgebra::Point2::new(text.width(ctx) as f32, text.height(ctx) as f32),
            graphics::Align::Center,
        );

        let s = State {
            screen_center: nalgebra::Point2::new(window_width / 2.0, window_height / 2.0),
            vertigo_angle: 0.0,
            stars: mesh_builder.build(ctx)?,
            title: text,
        };

        Ok(s)
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        // Delta time for hardware independent speed
        let seconds_took_in_last_frame = timer::delta(ctx).as_millis() as f32 * 0.001;

        self.vertigo_angle = self.vertigo_angle % 360.0 + seconds_took_in_last_frame;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        // Background
        let background_color = graphics::Color::from_rgb(0, 0, 32);
        graphics::clear(ctx, background_color);

        // Stars
        let param = graphics::DrawParam::new()
            .dest(nalgebra::Point2::new(0.0, 0.0))
            .rotation(self.vertigo_angle.to_radians())
            .offset(self.screen_center);

        graphics::draw(ctx, &self.stars, param)?;

        // Text
        let seconds_since_start = timer::time_since_start(ctx).as_millis() as f32 * 0.001;
        let bouncing_angle = (seconds_since_start.sin() * 10.0).to_radians();

        let text_width = self.title.width(ctx) as f32;
        let text_height = self.title.height(ctx) as f32;
        graphics::queue_text(ctx, &self.title, nalgebra::Point2::new(0.0, 0.0), None);

        let text_param = graphics::DrawParam::new()
            .dest(nalgebra::Point2::new(
                self.screen_center.x - text_width / 2.0,
                self.screen_center.y - text_height / 2.0,
            ))
            .rotation(bouncing_angle)
            .offset(nalgebra::Point2::new(text_width / 2.0, text_height / 2.0));

        graphics::draw_queued_text(ctx, text_param, None, graphics::FilterMode::Nearest)?;

        match graphics::present(ctx) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to render intro screen via graphics::present: {}", e);
                return Err(e);
            }
        };

        Ok(())
    }
}
