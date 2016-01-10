use world::World;

pub struct Player {
    pub x: i32,
    pub y: i32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        Player {x: x, y: y}
    }

    pub fn move_x(&mut self, move_by: i32) {
        self.x += move_by;
    }

    pub fn move_y(&mut self, move_by: i32) {
        self.y += move_by;
    }

}
