mod world;
use crate::world::generator::Element;
use crate::world::generator::Map;
use crate::world::generator::Visibility;
use crate::world::graphic::VisualEngine as ve;
use kiss3d::window::Window;

fn main() {
    let mapka: Map = world::generator::Map::create(900, 900, 900);
    mapka.make_visible(true);
    let mut window = ve::create_window("autokilah", kiss3d::light::Light::StickToCamera);
    window.set_background_color(0.0,0.0,0.0);
    let cube = window.add_cube(0.5,0.2,0.1);
    while window.render() {
        cube.add_rotation_in_axis(0.01,'x');
    }
}
