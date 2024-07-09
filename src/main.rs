use laser::Laser;
use raylib::{color::Color, drawing::RaylibDraw, math::Vector2};
use spacehip::Spacehip;

mod laser;
mod spacehip;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGTH: f32 = 600.0;

fn main() {
    let (mut rl, t) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGTH as i32)
        .title("ball")
        .build();
    let gray = Color::new(29, 29, 27, 255);
    let mut spacehip = Spacehip::new(&mut rl, "res/spaceship.png", &t);
    let mut laser = Laser::new(Vector2::new(100.0, 100.0), -6.0);

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        spacehip.spaceship_update(&rl);
        laser.laser_update(&rl);
        let mut d = rl.begin_drawing(&t);
        d.clear_background(gray);
        laser.laser_draw(&mut d);
        spacehip.spaceship_draw(&mut d);
    }
}
