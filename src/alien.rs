use raylib::prelude::*;

pub struct Alien {
    pub position: Vector2,
    pub alien_sprite: Texture2D,
}

impl Alien {
    pub fn new(types: usize, position: Vector2, rl: &mut RaylibHandle, t: &RaylibThread) -> Self {
        let alien_sprite = match types {
            1 => rl.load_texture(&t, "res/alien_1.png").unwrap(),
            2 => rl.load_texture(&t, "res/alien_2.png").unwrap(),
            3 => rl.load_texture(&t, "res/alien_3.png").unwrap(),
            _ => rl.load_texture(&t, "/res/alien_1.png").unwrap(),
        };

        return Alien {
            position,
            alien_sprite,
        };
    }

    pub fn alien_draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_v(
            &self.alien_sprite,
            Vector2::new(self.position.x, self.position.y),
            Color::WHITE,
        );
    }

    pub fn alien_update(&mut self, direction: f32) {
        self.position.x += direction;
    }
}
