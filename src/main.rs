mod world;
use crate::world::generator::Element;
use crate::world::generator::Map;
use crate::world::generator::Visibility;

fn main() {
    let mapka: Map = world::generator::Map::create(900,900,900);
    mapka.make_visible(true);
    world::graphic::window_maker();
}
