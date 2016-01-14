extern crate engine;
extern crate tcod;
extern crate geojson;

use tcod::input::Key;
use tcod::input::KeyCode::{Up, Down, Left, Right, Escape};
use engine::{World, Building};

use std::io::prelude::*;
use std::fs::File;

use geojson::GeoJson;
use geojson::FeatureCollection;

fn main() {
    let mut world = World::new(150, 80);
    let mut entities = Vec::new();
    let data = read_file("./data/output.geojson");

    let geojson = data.parse::<GeoJson>().unwrap() as FeatureCollection;

    println!("{}", geojson.features.get(0).unwrap().id.unwrap().to_string());

    entities.push(Building::new(5, 5, 5, 5));
    entities.push(Building::new(10, 15, 25, 25));

    while !world.dead() {
        for entitie in &mut entities {
            entitie.draw(&mut world);
        }

        world.draw();
        
        let keypress = world.root.wait_for_keypress(true);
        //println!("Pressed key: {:?}", key);
        if keypress.pressed {
            match keypress {
                Key { code: Escape, .. } => break,
                Key { code: Up, .. } => world.player.move_y(-1),
                Key { code: Down, .. } => world.player.move_y(1),
                Key { code: Left, .. } => world.player.move_x(-1),
                Key { code: Right, .. } => world.player.move_x(1),
                _ => {}
            }
        }

        world.clear();

    }
}

fn read_file(path: &str) -> String {
    let mut buffer = String::new();
    let _ = (File::open(path).unwrap())
        .read_to_string(&mut buffer);

    return buffer;
}
