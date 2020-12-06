mod world;

extern crate nalgebra as na;
use crate::world::generator::Map;
use crate::world::graphic::Move;
use crate::world::graphic::VisualEngine as ve;
use kiss3d::window::Window;
use na::{Translation3, UnitQuaternion, Vector3};
use rand::Rng;
use std::path::Path;

fn main() {
    let _mapka: Map = world::generator::Map::create(900, 900, 900);
    let mut window = ve::create_window(
        "Space killer",
        kiss3d::light::Light::StickToCamera,
        0.0,
        0.0,
        0.0,
    );

    let space_ship_obj = Path::new("./src/resources/spaceship/statek.obj");
    let mut space_ship = window.add_obj(
        &space_ship_obj,
        &space_ship_obj,
        Vector3::new(0.2, 0.2, 0.2),
    );
    space_ship.append_translation(&Translation3::new(0.0,0.0,3.0));
    space_ship.append_translation(&Translation3::new(0.0,-1.0,0.0));
    space_ship.add_rotation_in_axis(3.15,'y');
    let texture_1 = include_bytes!("./resources/textures/sphere_texture.png");
    space_ship.set_texture_from_memory(texture_1, "texture_1");
    generate_plantes(50, &mut window);
    while window.render() {
        // sphere.add_rotation_in_axis(0.01, 'x');
        // sphere.add_rotation_in_axis(0.01, 'y');
    }
}

fn generate_plantes(planet_number: u32, window: &mut Window) {
    let mut n = 0;
    let mut rng = rand::thread_rng();

    while n < planet_number {
        let mut sphere = window.add_sphere(0.1);
        let texture_1 = include_bytes!("./resources/textures/sphere_texture.png");
        sphere.set_texture_from_memory(texture_1, "texture_1");
        sphere.append_translation(&Translation3::new(
            rng.gen_range(-15.0, 15.0),
            rng.gen_range(-5.0, 5.0),
            rng.gen_range(-15.0, 15.0),
        ));
        n += 1;
    }
}
