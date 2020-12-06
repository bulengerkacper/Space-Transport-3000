mod world;

extern crate nalgebra as na;
use crate::world::generator;
use crate::world::graphic::Move;
use crate::world::graphic::VisualEngine as ve;

fn main() {
    let mut window = ve::create_window(
        "Space killer",
        kiss3d::light::Light::StickToCamera,
        0.0,
        0.0,
        0.0,
    );

    let _space_ship = generator::create_spaceship(&mut window);
    let planets = generator::generate_plantes(100, &mut window);
    while window.render() {
        for planet in planets.iter() {
            planet.add_rotation_in_axis(0.01, 'x');
        }
    }
}
