use raylib::prelude::*;

#[derive(Clone)]
pub struct Block {
    pub position: Vector2,
}

impl Block {
    pub fn new(position: Vector2) -> Self {
        return Block { position };
    }

    pub fn block_draw(&self, d: &mut RaylibDrawHandle) {
        let color = Color::new(243, 216, 63, 255);
        d.draw_rectangle(self.position.x as i32, self.position.y as i32, 3, 3, color);
    }

    pub fn block_get_rec(&self) -> Rectangle {
        let mut rec = Rectangle::default();
        rec.x = self.position.x;
        rec.y = self.position.y;
        rec.width = 3.0;
        rec.height = 3.0;
        return rec;
    }
}
