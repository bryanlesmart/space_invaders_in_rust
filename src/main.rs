use raylib::{color::Color, drawing::RaylibDraw};
use spacehip::Spacehip;

mod spacehip;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGTH: f32 = 600.0;

fn main() {
    let (mut rl, t) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGTH as i32)
        .title("ball")
        .build();
    let gray = Color::new(29, 29, 27, 255);
    let spacehip = Spacehip::new(&mut rl, "res/spacehip.png", &t);

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&t);

        d.clear_background(gray);

        spacehip.spaceship_draw(&mut d);
    }
}
