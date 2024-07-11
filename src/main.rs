use game::Game;
use raylib::prelude::*;

mod alien;
mod block;
mod game;
mod laser;
mod msytery_ship;
mod obstacle;
mod spacehip;

const SCREEN_WIDTH: f32 = 750.0;
const SCREEN_HEIGTH: f32 = 700.0;

fn main() {
    let (mut rl, t) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGTH as i32)
        .title("ball")
        .build();
    let gray = Color::new(29, 29, 27, 255);
    let mut game = Game::new(&mut rl, "res/spaceship.png", &t);
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        game.game_input(&rl);
        game.game_update(&rl);
        let mut d = rl.begin_drawing(&t);
        d.clear_background(gray);
        game.game_draw(&mut d);
    }
}
