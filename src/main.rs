mod world;
use crate::world::generator::Visibility;
use crate::world::generator::Map;
use crate::world::generator::Element;


fn main() {
    let mapka:Map = world::generator::Map::create();
    // let mut map_xyz : Map = Map.create();
    // Map::make_visible(&mapka);
    mapka.make_visible(true);
}
