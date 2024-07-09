use raylib::prelude::*;

pub struct Spacehip {
    pub position: Vector2,
    pub sprite: Texture2D,
}

impl Spacehip {
    pub fn new(rl: &mut RaylibHandle, filename: &'static str, t: &RaylibThread) -> Self {
        return Spacehip {
            position: Vector2::zero(),
            sprite: rl.load_texture(&t, filename).unwrap(),
        };
    }

    pub fn spaceship_update(&mut self, rl: &RaylibHandle) {
        self.position = Vector2::new(
            rl.get_screen_width() as f32 / 2.0,
            rl.get_screen_height() as f32 / 2.0,
        );
    }

    pub fn spaceship_draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_v(&self.sprite, self.position, Color::WHITE);
    }
}
