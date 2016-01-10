use world::World;

pub struct Building {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
}

impl Building {
    pub fn new(width: i32, height: i32, x: i32, y: i32) -> Building {
        Building {width:width, height:height, x:x, y:y}
    }

    pub fn draw(&self, world: &mut World) {
        for x in 1..self.width {
            for y in 1..self.height {
                world.put_char(x + self.x, y + self.y, '#');
            }
        }
    }
}


