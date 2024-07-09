use raylib::prelude::*;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub speed_x: f32,
    pub speed_y: f32,
    pub radius: f32,
}

impl Ball {
    pub fn new() -> Self {
        return Ball {
            x: 100.0,
            y: 100.0,
            speed_x: 5.0,
            speed_y: 5.0,
            radius: 15.0,
        };
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        self.x += self.speed_x;
        self.y += self.speed_y;

        if self.x + self.radius >= rl.get_screen_width() as f32 || self.x - self.radius <= 0.0 {
            self.speed_x *= -1.0;
        }

        if self.y + self.radius >= rl.get_screen_height() as f32 || self.y - self.radius <= 0.0 {
            self.speed_y *= -1.0;
        }
    }

    pub fn draw(&self, rl: &mut RaylibHandle, t: &RaylibThread) {
        let color = Color::new(20, 160, 133, 255);
        let mut d = rl.begin_drawing(&t);
        d.clear_background(color);

        d.draw_circle(self.x as i32, self.y as i32, self.radius, Color::WHITE)
    }

    // pub fn delete_inactive_lasers(&mut self) {
    //     for laser in self.laser.iter_mut() {
    //         laser.active = false;
    //     }
    // }
}
