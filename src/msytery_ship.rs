use raylib::prelude::*;

pub struct MysteryShip {
    pub postion: Vector2,
    pub mystery_ship_sprite: Texture2D,
    pub speed: f32,
    pub active: bool,
}

impl MysteryShip {
    pub fn new(rl: &mut RaylibHandle, t: &RaylibThread) -> Self {
        return MysteryShip {
            postion: Vector2::zero(),
            mystery_ship_sprite: rl.load_texture(&t, "res/mystery.png").unwrap(),
            speed: 0.0,
            active: false,
        };
    }

    pub fn spawn_mystery_ship(&mut self, rl: &RaylibHandle) {
        self.postion.y = 90.0;
        let side: i32 = rl.get_random_value(0..1);

        if side == 0 {
            self.postion.x = 0.0;
            self.speed = 3.0;
        } else {
            self.postion.x = rl.get_screen_width() as f32 - self.mystery_ship_sprite.width() as f32;
            self.speed = -3.0;
        }

        self.active = true;
    }

    pub fn mystery_ship_update(&mut self, rl: &RaylibHandle) {
        if self.active {
            self.postion.x += self.speed;
            if self.postion.x
                > rl.get_screen_width() as f32 - self.mystery_ship_sprite.width() as f32
                || self.postion.x < 0.0
            {
                self.active = false;
            }
        }
    }

    pub fn ship_draw(&self, d: &mut RaylibDrawHandle) {
        if self.active {
            d.draw_texture(
                &self.mystery_ship_sprite,
                self.postion.x as i32,
                self.postion.y as i32,
                Color::WHITE,
            );
        }
    }
}
