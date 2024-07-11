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

    pub fn laser_get_rec(&self) -> Rectangle {
        let mut rec = Rectangle::default();
        rec.x = self.position.x;
        rec.y = self.position.y;
        rec.width = 4.0;
        rec.height = 15.0;
        return rec;
    }

    pub fn laser_update(&mut self, rl: &RaylibHandle) {
        if self.active {
            if self.position.y > rl.get_screen_height() as f32 || self.position.y < 0.0 {
                return self.active = false;
            }
            self.position.y += self.speed;
        }
    }

    pub fn laser_draw(&self, d: &mut RaylibDrawHandle) {
        let color = Color::new(243, 216, 63, 255);
        if self.active {
            d.draw_rectangle(self.position.x as i32, self.position.y as i32, 4, 5, color);
        }
    }
}
