use ggez::*;

mod intro;

pub fn main() {
    let mut cfg = conf::Conf::new(); // TODO: Load from configuration file
    cfg.window_setup.title = "Rusty Gorillas".to_string();
    cfg.window_mode.width = 640.0;
    cfg.window_mode.height = 480.0;

    let context_builder = ggez::ContextBuilder::new("super_simple", "ggez").conf(cfg);
    let (ctx, event_loop) = &mut context_builder.build().unwrap();

    let state = &mut intro::State::new(ctx).unwrap();

    match event::run(ctx, event_loop, state) {
        Ok(_) => (),
        Err(e) => println!("Error occured: {}", e),
    }
}
