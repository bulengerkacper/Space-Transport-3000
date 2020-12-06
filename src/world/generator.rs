use crate::world::graphic::Move;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use na::{Translation3, Vector3};
use rand::Rng;
use std::collections::LinkedList;
use std::path::Path;

pub fn create_spaceship(window: &mut Window) -> SceneNode {
    let space_ship_obj = Path::new("./src/resources/spaceship/statek.obj");
    let mut space_ship = window.add_obj(
        &space_ship_obj,
        &space_ship_obj,
        Vector3::new(0.2, 0.2, 0.2),
    );
    space_ship.append_translation(&Translation3::new(0.0, 0.0, 4.0));
    space_ship.append_translation(&Translation3::new(0.0, -1.5, 0.0));
    space_ship.add_rotation_in_axis(3.15, 'y');
    let metal_texture = include_bytes!("../resources/textures/metal_texture.png");
    space_ship.set_texture_from_memory(metal_texture, "metal_texture");
    space_ship
}

pub fn generate_plantes(planet_number: u32, window: &mut Window) -> LinkedList<SceneNode> {
    let mut n = 0;
    let mut rng = rand::thread_rng();

    let mut planets = LinkedList::new();
    while n < planet_number {
        let mut sphere = window.add_sphere(0.1);
        let sphere_texture = include_bytes!("../resources/textures/sphere_texture.png");
        sphere.set_texture_from_memory(sphere_texture, "sphere_texture");
        sphere.append_translation(&Translation3::new(
            rng.gen_range(-15.0, 15.0),
            rng.gen_range(-10.0, 10.0),
            rng.gen_range(-8.0, 8.0),
        ));
        n += 1;
        planets.push_back(sphere);
    }
    planets
}
