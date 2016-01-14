use tcod::RootConsole;
use tcod::console::*;
use tcod::chars;

use camera::Camera;
use player::Player;

pub struct World {
    pub root: RootConsole,
    pub camera: Camera,
    pub player: Player
}

impl World {
    pub fn new(width: i32, height: i32) -> World {
        let root = RootConsole::initializer()
            .size(width, height)
            .title("Minimal libtcod loop")
            .init();
        
        World {
            root: root,
            camera: Camera::new(0, 0),
            player: Player::new(width/2, height/2),
        }
    }
    
    pub fn draw(&mut self) {
        let x = self.player.x;
        let y = self.player.y;

        self.put_char(x, y, '@');
        self.flush();
    }

    pub fn clear(&mut self) { 
        self.root.clear();
    }

    pub fn dead(&self) -> bool {
        self.root.window_closed()
    }

    pub fn flush(&mut self) {
        self.root.flush();
    }

    pub fn put_char(&mut self, x: i32, y: i32, c: char) {
        self.root.put_char(
            x + self.camera.x,
            y + self.camera.y,
            c,
            BackgroundFlag::None
        );
    }
}

