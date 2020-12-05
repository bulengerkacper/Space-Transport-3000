mod world;
use crate::world::generator::Element;
use crate::world::generator::Map;
use crate::world::generator::Visibility;
use crate::world::graphic::Move;
use crate::world::graphic::VisualEngine as ve;
use kiss3d::window::Window;
use std::path::Path;


fn main() {
    let mapka: Map = world::generator::Map::create(900, 900, 900);
    mapka.make_visible(true);
    let mut window = ve::create_window("autokilah", kiss3d::light::Light::StickToCamera,0.0,0.0,0.0);
    let mut cube = window.add_cube(0.1, 0.1, 0.1);
    cube.set_texture_from_file(&Path::new("./resources/1.png"),"1");
    while window.render() {
        cube.add_rotation_in_axis(0.01, 'x');
    }
}
