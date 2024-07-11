use raylib::prelude::*;

pub struct MysteryShip {
    pub postion: Vector2,
    pub mystery_ship_sprite: Texture2D,
    pub speed: f32,
    pub alive: bool,
}

impl MysteryShip {
    pub fn new(rl: &mut RaylibHandle, t: &RaylibThread) -> Self {
        return MysteryShip {
            postion: Vector2::zero(),
            mystery_ship_sprite: rl.load_texture(&t, "res/mystery.png").unwrap(),
            speed: 0.0,
            alive: false,
        };
    }

    pub fn mystery_ship_get_rec(&self) -> Rectangle {
        if self.alive {
            Rectangle::new(
                self.postion.x,
                self.postion.y,
                self.mystery_ship_sprite.width as f32,
                self.mystery_ship_sprite.height as f32,
            )
        } else {
            Rectangle::default()
        }
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

        self.alive = true;
    }

    pub fn mystery_ship_update(&mut self, rl: &RaylibHandle) {
        if self.alive {
            self.postion.x += self.speed;
            if self.postion.x
                > rl.get_screen_width() as f32 - self.mystery_ship_sprite.width() as f32
                || self.postion.x < 0.0
            {
                self.alive = false;
            }
        }
    }

    pub fn ship_draw(&self, d: &mut RaylibDrawHandle) {
        if self.alive {
            d.draw_texture(
                &self.mystery_ship_sprite,
                self.postion.x as i32,
                self.postion.y as i32,
                Color::WHITE,
            );
        }
    }
}
