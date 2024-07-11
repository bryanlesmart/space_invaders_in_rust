use raylib::prelude::*;

use crate::laser::Laser;

pub struct Spacehip {
    pub laser: Vec<Laser>,
    pub position: Vector2,
    pub sprite: Texture2D,
    pub last_fire_time: f32,
}

impl Spacehip {
    pub fn new(rl: &mut RaylibHandle, filename: &'static str, t: &RaylibThread) -> Self {
        let sprite = rl.load_texture(&t, filename).unwrap();
        return Spacehip {
            laser: Vec::new(),
            position: Vector2::new(
                (rl.get_screen_width() as f32 - sprite.width() as f32) / 2.0,
                rl.get_screen_height() as f32 - sprite.height() as f32,
            ),
            last_fire_time: 0.0,
            sprite,
        };
    }

    pub fn move_left(&mut self, rl: &RaylibHandle) {
        self.position.x -= 7.0;
        if self.position.x >= rl.get_screen_width() as f32 - self.sprite.width() as f32 {
            self.position.x = rl.get_screen_width() as f32 - self.sprite.width() as f32;
        } else if self.position.x < 0.0 {
            self.position.x = 0.0;
        }
    }

    pub fn move_right(&mut self, rl: &RaylibHandle) {
        self.position.x += 7.0;

        if self.position.x >= rl.get_screen_width() as f32 - self.sprite.width() as f32 {
            self.position.x = rl.get_screen_width() as f32 - self.sprite.width() as f32;
        } else if self.position.x < 0.0 {
            self.position.x = 0.0;
        }
    }

    pub fn spaceship_draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_v(&self.sprite, self.position, Color::WHITE);
        for dr in self.laser.iter() {
            dr.laser_draw(d);
        }
    }

    pub fn space_fire_laser(&mut self, rl: &RaylibHandle) {
        if rl.get_time() - self.last_fire_time as f64 >= 0.35 {
            self.laser.push(Laser::new(
                Vector2::new(
                    self.position.x + self.sprite.width() as f32 / 2.0 - 2.0,
                    self.position.y,
                ),
                -6.0,
            ));
            self.last_fire_time = rl.get_time() as f32;
        }
    }

    pub fn spacehip_get_rect(&self) -> Rectangle {
        return Rectangle::new(
            self.position.x,
            self.position.y,
            self.sprite.width as f32,
            self.sprite.height as f32,
        );
    }
}
