use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

use crate::spacehip::Spacehip;

pub struct Game {
    pub spacehip: Spacehip,
}

impl Game {
    pub fn new(rl: &mut RaylibHandle, filename: &'static str, t: &RaylibThread) -> Self {
        return Self {
            spacehip: Spacehip::new(rl, filename, t),
        };
    }

    pub fn game_draw(&self, d: &mut RaylibDrawHandle) {
        self.spacehip.spaceship_draw(d);

        for i in self.spacehip.laser.iter() {
            i.laser_draw(d);
        }
    }

    pub fn game_input(&mut self, rl: &RaylibHandle) {
        if rl.is_key_down(KEY_LEFT) {
            self.spacehip.move_left(rl);
        } else if rl.is_key_down(KEY_RIGHT) {
            self.spacehip.move_right(rl);
        } else if rl.is_key_down(KEY_SPACE) {
            self.spacehip.space_fire_laser(rl);
        }
    }

    pub fn game_update(&mut self, rl: &RaylibHandle) {
        for laser in self.spacehip.laser.iter_mut() {
            laser.laser_update(rl);
        }
        self.delete_inactive_laser();

        println!("{:?}", self.spacehip.laser.len());
    }

    pub fn delete_inactive_laser(&mut self) {
        self.spacehip.laser.retain(|laser| laser.active);
    }
}
