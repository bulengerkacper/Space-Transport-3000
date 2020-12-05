mod world;

extern crate nalgebra as na;
use crate::world::generator::Map;
use crate::world::graphic::Move;
use crate::world::graphic::VisualEngine as ve;
use na::{Translation3, UnitQuaternion, Vector3};
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
    let mut sphere = window.add_sphere(0.1);
    let texture_1 = include_bytes!("./resources/textures/sphere_texture.png");
    sphere.set_texture_from_memory(texture_1, "texture_1");

    let earth_obj = Path::new("./src/resources/textures/earth.obj");
    let earth_mtl = Path::new("./src/resources/textures");
    let cyber_gnat_obj = Path::new("./src/resources/gun.obj");

    // let mut earth = window.add_obj(&earth_obj, &earth_mtl, Vector3::new(0.1, 0.1, 0.1));
    let mut cyber_gnat = window.add_obj(&cyber_gnat_obj, &cyber_gnat_obj, Vector3::new(0.2,0.2,0.2));
    cyber_gnat.append_translation(&Translation3::new(-1.75, -3.00, 0.3));

    while window.render() {
        sphere.add_rotation_in_axis(0.01, 'x');
    }
}
