use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

use crate::obstacle::Obstacle;
use crate::spacehip::Spacehip;

pub struct Game {
    pub spacehip: Spacehip,
    pub obstacle: Vec<Obstacle>,
}

impl Game {
    pub fn new(rl: &mut RaylibHandle, filename: &'static str, t: &RaylibThread) -> Self {
        let mut game = Game {
            spacehip: Spacehip::new(rl, filename, t),
            obstacle: Vec::new(),
        };
        let obstacle = Obstacle::new(Vector2::zero());
        game.create_obstacles(obstacle, rl);
        return game;
    }

    pub fn game_draw(&mut self, d: &mut RaylibDrawHandle) {
        self.spacehip.spaceship_draw(d);

        for laser in self.spacehip.laser.iter() {
            laser.laser_draw(d);
        }

        for obstacle in self.obstacle.iter_mut() {
            obstacle.obstacle_draw(d);
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

        // println!("{:?}", self.spacehip.laser.len());
    }

    pub fn create_obstacles(&mut self, obstacle: Obstacle, rl: &RaylibHandle) -> Vec<Obstacle> {
        let obs_width = obstacle.grid[0].len() as f32 * 3.0;
        let gap = (rl.get_screen_width() as f32 - (4.0 * obs_width)) / 5.0;

        for i in 0..4 {
            let off_set = (i as f32 + 1.0) * gap + i as f32 * obs_width;
            self.obstacle.push(Obstacle::new(Vector2::new(
                off_set,
                rl.get_screen_height() as f32 - 200.0,
            )))
        }
        return self.obstacle.clone();
    }

    pub fn delete_inactive_laser(&mut self) {
        self.spacehip.laser.retain(|laser| laser.active);
    }
}
