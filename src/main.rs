mod world;
use crate::world::generator::Element;
use crate::world::generator::Map;
use crate::world::generator::Visibility;
use crate::world::graphic::VisualEngine as ve;

fn main() {
    let mapka: Map = world::generator::Map::create(900, 900, 900);
    mapka.make_visible(true);
    let mut window = ve::create_window("autokilah", kiss3d::light::Light::StickToCamera);
}
