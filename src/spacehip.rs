use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

pub struct Spacehip {
    pub position: Vector2,
    pub sprite: Texture2D,
}

impl Spacehip {
    pub fn new(rl: &mut RaylibHandle, filename: &'static str, t: &RaylibThread) -> Self {
        let sprite = rl.load_texture(&t, filename).unwrap();
        return Spacehip {
            position: Vector2::new(
                (rl.get_screen_width() as f32 - sprite.width() as f32) / 2.0,
                rl.get_screen_height() as f32 - sprite.height() as f32,
            ),
            sprite,
        };
    }

    pub fn spaceship_update(&mut self, rl: &RaylibHandle) {
        if rl.is_key_down(KEY_LEFT) {
            self.position.x -= 7.0;
        } else if rl.is_key_down(KEY_RIGHT) {
            self.position.x += 7.0;
        }

        //Collision to with of screen
        if self.position.x >= rl.get_screen_width() as f32 - self.sprite.width() as f32 {
            self.position.x = rl.get_screen_width() as f32 - self.sprite.width() as f32;
        } else if self.position.x < 0.0 {
            self.position.x = 0.0;
        }
    }

    pub fn spaceship_draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_v(&self.sprite, self.position, Color::WHITE);
    }
}
