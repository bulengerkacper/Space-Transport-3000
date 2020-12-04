mod world;
use crate::world::generator::Element;
use crate::world::generator::Map;
use crate::world::generator::Visibility;

fn main() {
    let mapka: Map = world::generator::Map::create();
    mapka.make_visible(true);
}
