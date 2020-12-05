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
    // let mut cube = window.add_cube(0.1, 0.1, 0.1);
    // let texture_1 = include_bytes!("./resources/1.png");
    // cube.set_texture_from_memory(texture_1, "texture_1");

    let earth_obj = Path::new("./src/resources/earth.obj");
    let earth_mtl = Path::new("./src/resources/textures");
    let mut earth = window.add_obj(&earth_obj, &earth_mtl, Vector3::new(0.1, 0.1, 0.1));
    while window.render() {
        // cube.add_rotation_in_axis(0.01, 'x');
    }
}
