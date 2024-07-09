use ball::Ball;

mod ball;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGTH: f32 = 600.0;

fn main() {
    let (mut rl, t) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGTH as i32)
        .title("ball")
        .build();

    let mut ball = Ball::new();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        ball.update(&rl);
        ball.draw(&mut rl, &t);
    }
}
