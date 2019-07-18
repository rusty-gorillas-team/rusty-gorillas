use ggez::{event, graphics, nalgebra, timer};

use rand::Rng;

use std::vec::Vec;

use crate::collision;

pub struct State {
    screen_center: nalgebra::Point2<f32>,

    vertigo_angle: f32,

    stars: graphics::Mesh,
    city_colliders: Vec<collision::BoundingRectangle>,

    title: graphics::Text,
}

impl State {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<State> {
        // Generate stars
        let mut rng = rand::thread_rng();

        let mut mesh_builder = graphics::MeshBuilder::new();

        for _ in 0..320 {
            let star_color = graphics::Color::from_rgb(
                rng.gen_range(0, 10),
                rng.gen_range(10, 50),
                rng.gen_range(51, 255),
            );

            mesh_builder.circle(
                graphics::DrawMode::fill(),
                nalgebra::Point2::new(
                    rng.gen_range(0.0, ctx.conf.window_mode.width),
                    rng.gen_range(0.0, ctx.conf.window_mode.width),
                ),
                rng.gen_range(0.0, 3.0),
                1.0,
                star_color,
            );
        }

        // Generate city colliders
        let mut city_colliders: Vec<collision::BoundingRectangle> = Vec::new();

        for house_id in 0..10 {
            let house_height = rng.gen_range(120.0, 360.0);
            city_colliders.push(collision::BoundingRectangle {
                x: house_id as f32 * 64.0,
                y: ctx.conf.window_mode.height - house_height,
                width: 64.0,
                height: house_height,
            });
        }

        // Text initialization
        let mut text = graphics::Text::new("Rusty Gorillas");
        text.set_bounds(
            nalgebra::Point2::new(text.width(ctx) as f32, text.height(ctx) as f32),
            graphics::Align::Center,
        );

        let s = State {
            screen_center: nalgebra::Point2::new(
                ctx.conf.window_mode.width / 2.0,
                ctx.conf.window_mode.height / 2.0,
            ),
            vertigo_angle: 0.0,
            stars: mesh_builder.build(ctx)?,
            city_colliders: city_colliders,
            title: text,
        };

        Ok(s)
    }
}

fn draw_colliders(
    ctx: &mut ggez::Context,
    state: &mut State,
    seconds_since_start: f32,
) -> ggez::GameResult<()> {
    let mut mesh_builder = graphics::MeshBuilder::new();

    let banana_color = graphics::Color::from_rgb(255, 255, 0);
    let banana_collider = collision::BoundingRectangle {
        x: ctx.conf.window_mode.width / 2.0
            + seconds_since_start.sin() * ctx.conf.window_mode.width / 2.0,
        y: ctx.conf.window_mode.height / 2.0,
        width: 50.0,
        height: 50.0,
    };

    for collider in &state.city_colliders {
        let house_color = if banana_collider.collides_with(&collider) {
            graphics::Color::from_rgb(255, 0, 0)
        } else {
            graphics::Color::from_rgb(32, 64, 128)
        };

        mesh_builder.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(collider.x, collider.y, collider.width, collider.height),
            house_color,
        );
    }

    mesh_builder.rectangle(
        graphics::DrawMode::fill(),
        graphics::Rect::new(
            banana_collider.x,
            banana_collider.y,
            banana_collider.width,
            banana_collider.height,
        ),
        banana_color,
    );

    let collider_mesh = mesh_builder.build(ctx)?;

    graphics::draw(ctx, &collider_mesh, graphics::DrawParam::new())?;

    Ok(())
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

        let seconds_since_start = timer::time_since_start(ctx).as_millis() as f32 * 0.001;

        // Stars
        let param = graphics::DrawParam::new()
            .dest(nalgebra::Point2::new(0.0, 0.0))
            .rotation(self.vertigo_angle.to_radians())
            .offset(self.screen_center);

        graphics::draw(ctx, &self.stars, param)?;

        // Colliders
        draw_colliders(ctx, self, seconds_since_start)?;

        // Text
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

        graphics::draw_queued_text(ctx, text_param)?;

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
