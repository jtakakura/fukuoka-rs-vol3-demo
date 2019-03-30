extern crate quicksilver;

mod assets;

use assets::Assets;
use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Background, Color, View},
    lifecycle::{run, Asset, Settings, State, Window},
    Result,
};
use std::f32::consts::PI;

const WORLD_WIDTH: i32 = 160;
const WORLD_HEIGHT: i32 = 144;

struct Bullet {
    asset_id: i32,
    position: Vector,
    angle: f32,
    velocity: f32,
    angular_velocity: f32,
    acceleration: f32,
    fired: bool,
}

impl Bullet {
    fn new(
        asset_id: i32,
        x: i32,
        y: i32,
        angle: f32,
        velocity: f32,
        angular_velocity: f32,
        acceleration: f32,
        fired: bool,
    ) -> Result<Bullet> {
        let bullet = Bullet {
            asset_id,
            position: Vector::new(x, y),
            angle,
            velocity,
            angular_velocity,
            acceleration,
            fired,
        };
        Ok(bullet)
    }

    fn update(&mut self, delta_time: f64) -> Result<()> {
        let radian = self.angle * PI * 2.;
        self.position.x += self.velocity * (delta_time as f32) * radian.cos();
        self.position.y += self.velocity * (delta_time as f32) * radian.sin();

        self.angle += self.angular_velocity;
        self.velocity += self.acceleration;

        if self.position.x > WORLD_WIDTH as f32 || self.position.y > WORLD_HEIGHT as f32 {
            self.fired = false;
        }

        println!("x: {:?} / y: {:?} / fired: {:?}", self.position.x, self.position.y, self.fired);

        Ok(())
    }

    fn draw(&mut self, window: &mut Window, assets: &mut Asset<Assets>) -> Result<()> {
        assets.execute(|assets| {
            let image = &(assets.bullet_b);
            window.draw(
                &image
                    .area()
                    .with_center((self.position.x, self.position.y)),
                Background::Img(&image),
            );
            Ok(())
        })?;

        Ok(())
    }
}

struct Bullets(Vec<Bullet>);

impl Bullets {
    fn new(capacity: usize) -> Result<Bullets> {
        Ok(Bullets(Vec::with_capacity(capacity)))
    }

    fn spawn(
        &mut self,
        asset_id: i32,
        x: i32,
        y: i32,
        angle: f32,
        velocity: f32,
        angular_velocity: f32,
        acceleration: f32,
    ) -> Result<()> {
        if let Some(bullet) = self.0.iter_mut().find(|b| !b.fired) {
            bullet.asset_id = asset_id;
            bullet.position = Vector::new(x, y);
            bullet.angle = angle;
            bullet.velocity = velocity;
            bullet.angular_velocity = angular_velocity;
            bullet.acceleration = acceleration;
            bullet.fired = true;
        } else {
            let bullet = Bullet::new(
                asset_id,
                x,
                y,
                angle,
                velocity,
                angular_velocity,
                acceleration,
                true,
            )
            .unwrap();
            self.0.push(bullet);
        }

        Ok(())
    }

    fn update(&mut self, delta_time: f64) -> Result<()> {
        for bullet in self.0.iter_mut().filter(|b| b.fired) {
            bullet.update(delta_time)?;
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window, assets: &mut Asset<Assets>) -> Result<()> {
        for bullet in self.0.iter_mut().filter(|b| b.fired) {
            bullet.draw(window, assets)?;
        }

        Ok(())
    }
}

struct Enemy {
    asset_id: i32,
    position: Vector,
    firing_interval: f32,
    reloading_time: f32,
    bullet_angle: f32,
    bullet_velocity: f32,
    bullets: Bullets,
}

impl Enemy {
    fn new(
        asset_id: i32,
        x: i32,
        y: i32,
        firing_interval: f32,
        bullet_angle: f32,
        bullet_velocity: f32,
    ) -> Result<Enemy> {
        let bullets = Bullets::new(5).unwrap();

        Ok(Enemy {
            asset_id,
            position: Vector::new(x, y),
            firing_interval,
            reloading_time: 0.,
            bullet_angle,
            bullet_velocity,
            bullets,
        })
    }

    fn update(&mut self, delta_time: f64) -> Result<()> {
        if self.reloading_time < 0. {
            self.bullets.spawn(0, self.position.x as i32, self.position.y as i32, self.bullet_angle, self.bullet_velocity, 0., 0.)?;
            self.reloading_time = self.firing_interval;
        } else {
            self.reloading_time -= delta_time as f32;
        }

        self.bullets.update(delta_time)?;

        Ok(())
    }

    fn draw(&mut self, window: &mut Window, assets: &mut Asset<Assets>) -> Result<()> {
        self.bullets.draw(window, assets)?;

        Ok(())
    }

    // fn bullets(&self) -> Vec<Bullet> {
    //     self.bullets.0
    // }
}

struct MainState {
    assets: Asset<Assets>,
    enemies: Vec<Enemy>,
}

impl State for MainState {
    fn new() -> Result<MainState> {
        let assets = Assets::new();
        let mut enemies = Vec::new();

        enemies.push(Enemy::new(0, 25, 25, 0.05, 0.25, 100.).unwrap());

        Ok(MainState { assets, enemies })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        let delta_time = window.update_rate() / 1000.;

        // println!("fps: {:?}", 1. / delta_time);

        for enemy in self.enemies.iter_mut() {
            enemy.update(delta_time)?;
        }

        window.set_view(View::new(Rectangle::new_sized((WORLD_WIDTH, WORLD_HEIGHT))));

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // window.clear(Color {
        //     r: 101.0 / 256.0,
        //     g: 220.0 / 256.0,
        //     b: 231.0 / 256.0,
        //     a: 1.0,
        // })?;
        window.clear(Color::WHITE)?;

        for enemy in self.enemies.iter_mut() {
            enemy.draw(window, &mut self.assets)?;
        }

        Ok(())
    }
}

fn main() {
    // NOTE: Set HIDPI to 1.0 to get pixel-perfect rendering.
    // Otherwise the window resizes to whatever value the OS sets and
    // scales the contents.
    // https://docs.rs/glutin/0.19.0/glutin/dpi/index.html
    std::env::set_var("WINIT_HIDPI_FACTOR", "1.0");

    run::<MainState>(
        "Fukuoka.rs vol.3",
        Vector::new(800, 720),
        Settings::default(),
    );
}
