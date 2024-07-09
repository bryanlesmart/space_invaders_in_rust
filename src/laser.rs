use raylib::prelude::*;

pub struct Laser {
    pub speed: f32,
    pub position: Vector2,
    pub active: bool,
}

impl Laser {
    pub fn new(position: Vector2, speed: f32) -> Self {
        return Laser {
            speed,
            position,
            active: true,
        };
    }

    pub fn laser_update(&mut self, rl: &RaylibHandle) {
        if self.active {
            if self.position.y > rl.get_screen_height() as f32 {
                return self.active = false;
            }
            self.position.y += self.speed;
        }
    }

    pub fn laser_draw(&self, d: &mut RaylibDrawHandle) {
        if self.active {
            d.draw_rectangle(
                self.position.x as i32,
                self.position.y as i32,
                4,
                5,
                Color::GRAY,
            );
        }
    }
}