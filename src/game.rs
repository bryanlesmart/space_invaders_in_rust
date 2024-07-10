use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

use crate::alien::Alien;
use crate::obstacle::Obstacle;
use crate::spacehip::Spacehip;

pub struct Game {
    pub spacehip: Spacehip,
    pub obstacle: Vec<Obstacle>,
    pub aliens: Vec<Alien>,
    pub run: bool,
}

impl Game {
    pub fn new(rl: &mut RaylibHandle, filename: &'static str, t: &RaylibThread) -> Self {
        let mut game = Game {
            spacehip: Spacehip::new(rl, filename, t),
            obstacle: Vec::new(),
            aliens: Vec::new(),
            run: true,
        };
        let obstacle = Obstacle::new(Vector2::zero());
        game.create_obstacles(obstacle, rl);
        game.create_aliens(rl, t);
        return game;
    }

    pub fn game_draw(&mut self, d: &mut RaylibDrawHandle) {
        if self.run {
            self.spacehip.spaceship_draw(d);

            for laser in self.spacehip.laser.iter() {
                laser.laser_draw(d);
            }

            for obstacle in self.obstacle.iter_mut() {
                obstacle.obstacle_draw(d);
            }

            for aliens in self.aliens.iter_mut() {
                aliens.alien_draw(d);
            }
        }
    }

    pub fn game_input(&mut self, rl: &RaylibHandle) {
        if self.run {
            if rl.is_key_down(KEY_LEFT) {
                self.spacehip.move_left(rl);
            } else if rl.is_key_down(KEY_RIGHT) {
                self.spacehip.move_right(rl);
            } else if rl.is_key_down(KEY_SPACE) {
                self.spacehip.space_fire_laser(rl);
            }
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

    pub fn create_aliens(&mut self, rl: &mut RaylibHandle, t: &RaylibThread) {
        for row in 0..5 {
            for col in 0..11 {
                let alien_type = match row {
                    0 => 3,
                    1 | 2 => 2,
                    _ => 1,
                };
                let x = 75.0 + col as f32 * 55.0;
                let y = 110.0 + row as f32 * 55.0;
                self.aliens
                    .push(Alien::new(alien_type, Vector2::new(x, y), rl, t));
            }
        }
    }

    pub fn delete_inactive_laser(&mut self) {
        self.spacehip.laser.retain(|laser| laser.active);
    }
}
