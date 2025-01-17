use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

use crate::alien::Alien;
use crate::laser::Laser;
use crate::msytery_ship::MysteryShip;
use crate::obstacle::Obstacle;
use crate::spacehip::Spacehip;

pub struct Game {
    pub spacehip: Spacehip,
    pub mystery_ship: MysteryShip,
    pub obstacle: Vec<Obstacle>,
    pub aliens: Vec<Alien>,
    pub run: bool,
    pub alien_direction: f32,
    pub alien_lasers: Vec<Laser>,
    pub alien_laser_shot_interval: f32,
    pub time_alien_fired: f32,
    pub mystery_ship_interval: f32,
    pub time_last_spawn_mystery_ship: f32,
    pub player_lives: i32,
}

impl Game {
    pub fn new(rl: &mut RaylibHandle, filename: &'static str, t: &RaylibThread) -> Self {
        let mystery_interval = rl.get_random_value::<i32>(10..20);
        let mut game = Game {
            spacehip: Spacehip::new(rl, filename, t),
            mystery_ship: MysteryShip::new(rl, t),
            obstacle: Vec::new(),
            aliens: Vec::new(),
            run: true,
            alien_direction: 1.0,
            alien_lasers: Vec::new(),
            alien_laser_shot_interval: 0.35,
            time_alien_fired: 0.0,
            mystery_ship_interval: mystery_interval as f32,
            time_last_spawn_mystery_ship: 0.0,
            player_lives: 3,
        };
        let obstacle = Obstacle::new(Vector2::zero());
        game.create_obstacles(obstacle, rl);
        game.create_aliens(rl, t);
        return game;
    }

    pub fn game_draw(&mut self, d: &mut RaylibDrawHandle) {
        if self.run {
            self.spacehip.spaceship_draw(d);
            self.mystery_ship.ship_draw(d);
            for laser in self.spacehip.laser.iter() {
                laser.laser_draw(d);
            }

            for obstacle in self.obstacle.iter_mut() {
                obstacle.obstacle_draw(d);
            }

            for aliens in self.aliens.iter_mut() {
                aliens.alien_draw(d);
            }

            for laser in self.alien_lasers.iter_mut() {
                laser.laser_draw(d);
            }
        }
    }

    pub fn init_game(&mut self, rl: &mut RaylibHandle, t: &RaylibThread) {
        let mystery_interval = rl.get_random_value::<i32>(10..20);

        let obstacle = Obstacle::new(Vector2::zero());
        self.spacehip = Spacehip::new(rl, "res/spaceship.png", t);
        self.obstacle = self.create_obstacles(obstacle, rl);
        self.create_aliens(rl, t);
        self.run = true;
        self.alien_direction = 1.0;
        self.alien_lasers = Vec::new();
        self.alien_laser_shot_interval = 0.35;
        self.time_alien_fired = 0.0;
        self.mystery_ship_interval = mystery_interval as f32;
        self.time_last_spawn_mystery_ship = 0.0;
        self.player_lives = 3;
    }

    pub fn reset_game(&mut self, rl: &RaylibHandle) {
        self.spacehip.spaceship_reset(rl);
        self.alien_lasers.clear();
        self.aliens.clear();
        self.obstacle.clear();
    }

    pub fn game_input(&mut self, rl: &mut RaylibHandle) {
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

    pub fn game_update(&mut self, rl: &mut RaylibHandle, t: &RaylibThread) {
        if self.run {
            let curren_time = rl.get_time();
            let get_time = rl.get_random_value::<i32>(10..20);

            if curren_time as f32 - self.time_last_spawn_mystery_ship > self.mystery_ship_interval {
                self.time_last_spawn_mystery_ship = rl.get_time() as f32;
                self.mystery_ship.spawn_mystery_ship(rl);
                self.mystery_ship_interval = get_time as f32;
            }

            for laser in self.spacehip.laser.iter_mut() {
                laser.laser_update(rl);
            }
            self.move_alien(rl);
            self.alien_shoot_laser(rl);

            for laser in self.alien_lasers.iter_mut() {
                laser.laser_update(rl);
            }
            self.mystery_ship.mystery_ship_update(rl);
            self.check_collison();
            self.delete_inactive_laser();
        } else {
            if rl.is_key_down(KEY_ENTER) {
                self.reset_game(rl);
                self.init_game(rl, t);
            }
        }
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

    pub fn move_alien(&mut self, rl: &RaylibHandle) {
        let screen_width = rl.get_screen_width() as f32;
        let mut change_direction = None;

        for alien in self.aliens.iter_mut() {
            if alien.position.x + alien.alien_sprite.width() as f32 > screen_width {
                change_direction = Some(-1.0);
            }
            if alien.position.x < 0.0 {
                change_direction = Some(1.0);
            }
        }

        if let Some(direction) = change_direction {
            self.alien_direction = direction;
            self.move_alien_down(4.0);
        }

        for alien in self.aliens.iter_mut() {
            alien.alien_update(self.alien_direction);
        }
    }

    pub fn move_alien_down(&mut self, distance: f32) {
        for alien in self.aliens.iter_mut() {
            alien.position.y += distance;
        }
    }

    pub fn alien_shoot_laser(&mut self, rl: &RaylibHandle) {
        let curren_time = rl.get_time();
        if curren_time as f32 - self.time_alien_fired >= self.alien_laser_shot_interval
            && !self.aliens.is_empty()
        {
            let size = self.aliens.len() - 1;
            let randon_index: i32 = rl.get_random_value(0..size as i32);
            let alien = &self.aliens[randon_index as usize];

            self.alien_lasers.push(Laser::new(
                Vector2::new(
                    alien.position.x + alien.alien_sprite.width() as f32 / 2.0,
                    alien.position.y + alien.alien_sprite.height() as f32,
                ),
                6.0,
            ));
            self.time_alien_fired = rl.get_time() as f32;
        }
    }

    pub fn delete_inactive_laser(&mut self) {
        self.spacehip.laser.retain(|laser| laser.active);
        self.alien_lasers.retain(|laser| laser.active);
    }

    pub fn check_collison(&mut self) {
        // Check collisions with aliens
        for laser in self.spacehip.laser.iter_mut() {
            if laser.active {
                let mut i = 0;
                while i < self.aliens.len() {
                    if self.aliens[i]
                        .alien_get_rec()
                        .check_collision_recs(&laser.laser_get_rec())
                    {
                        self.aliens.remove(i);
                        laser.active = false;
                        break;
                    } else {
                        i += 1;
                    }
                }
            }
        }

        // Check collisions with obstacles
        for laser in self.spacehip.laser.iter_mut() {
            if laser.active {
                for obstacle in self.obstacle.iter_mut() {
                    let mut i = 0;
                    while i < obstacle.blocks.len() {
                        if obstacle.blocks[i]
                            .block_get_rec()
                            .check_collision_recs(&laser.laser_get_rec())
                        {
                            obstacle.blocks.remove(i);
                            laser.active = false;
                        } else {
                            i += 1;
                        }
                    }
                }
            }
        }

        // Check collision with the mystery ship
        for laser in self.spacehip.laser.iter_mut() {
            if laser.active {
                if self.mystery_ship.alive
                    && self
                        .mystery_ship
                        .mystery_ship_get_rec()
                        .check_collision_recs(&laser.laser_get_rec())
                {
                    self.mystery_ship.alive = false;
                    laser.active = false;
                    break;
                }
            }
        }

        // Check alien laser  collisions with the spaceship
        for laser in self.alien_lasers.iter_mut() {
            if laser.active {
                if self
                    .spacehip
                    .spacehip_get_rect()
                    .check_collision_recs(&laser.laser_get_rec())
                {
                    laser.active = false;
                    self.player_lives -= 1;
                    if self.player_lives == 0 {
                        self.game_over();
                    }
                    break;
                }
                for obstacle in self.obstacle.iter_mut() {
                    let mut i = 0;
                    while i < obstacle.blocks.len() {
                        if obstacle.blocks[i]
                            .block_get_rec()
                            .check_collision_recs(&laser.laser_get_rec())
                        {
                            obstacle.blocks.remove(i);
                            laser.active = false;
                        } else {
                            i += 1;
                        }
                    }
                }
            }
        }

        //Alien collision with obstacle
        for aliens in self.aliens.iter_mut() {
            for obstacle in self.obstacle.iter_mut() {
                let mut i = 0;
                while i < obstacle.blocks.len() {
                    if obstacle.blocks[i]
                        .block_get_rec()
                        .check_collision_recs(&aliens.alien_get_rec())
                    {
                        obstacle.blocks.remove(i);
                    } else {
                        i += 1;
                    }
                }
            }
        }

        //Aliens collison with spacehip
        let spacehip_rect = self.spacehip.spacehip_get_rect();
        if self
            .aliens
            .iter()
            .any(|f| f.alien_get_rec().check_collision_recs(&spacehip_rect))
        {
            self.game_over();
            return;
        }
    }

    pub fn game_over(&mut self) {
        self.run = false;
    }
}
