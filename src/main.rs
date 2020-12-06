mod world;

extern crate nalgebra as na;
use crate::world::generator;
use crate::world::graphic::Move;
use crate::world::graphic::VisualEngine as ve;
use rand::Rng;

fn main() {
    let mut window = ve::create_window(
        "Space killer",
        kiss3d::light::Light::StickToCamera,
        0.0,
        0.0,
        0.0,
    );

    let _space_ship = generator::create_spaceship(&mut window);
    let mut planets = generator::generate_plantes(40, &mut window);
    let mut rng = rand::thread_rng();
    while window.render() {
        for planet in &mut planets {
            planet.add_rotation_in_axis(rng.gen_range(0.0, 0.2), 'x');
        }
    }
}
