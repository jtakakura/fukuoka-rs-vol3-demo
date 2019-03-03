extern crate quicksilver;

use quicksilver::{
    geom::Vector,
    graphics::Color,
    lifecycle::{run, Settings, State, Window},
    Result,
};

struct MainState;

impl State for MainState {
    fn new() -> Result<MainState> {
        Ok(MainState {})
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        Ok(())
    }
}

fn main() {
    run::<MainState>(
        "Fukuoka.rs vol.3",
        Vector::new(800, 600),
        Settings::default(),
    );
}
