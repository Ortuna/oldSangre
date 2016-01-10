extern crate engine;

use engine::{World, Building};

fn main() {
    let mut world = World::new(150, 80);
    let mut entities = Vec::new();

    entities.push(Building::new(5, 5, 5, 5));
    entities.push(Building::new(10, 15, 25, 25));

    while !world.dead() {
        for entitie in &mut entities {
            entitie.draw(&mut world);
        }

        world.draw();
        
        let key = world.root.wait_for_keypress(true);
        //println!("Pressed key: {:?}", key);

        if key.printable == 'w' {
            world.player.move_y(-1);
        }

        if key.printable == 's' {
            world.player.move_y(1);
        }

        if key.printable == 'a' {
            world.player.move_x(-1);
        }
        if key.printable == 'd' {
            world.player.move_x(1);
        }

        world.clear();

    }
}

