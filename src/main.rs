mod world;

extern crate nalgebra as na;
use crate::world::generator as generator;
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
    let _planets = generator::generate_plantes(100, &mut window);
    while window.render() {}
}

// fn create_spaceship(window: &mut Window) -> SceneNode {
//     let space_ship_obj = Path::new("./src/resources/spaceship/statek.obj");
//     let mut space_ship = window.add_obj(
//         &space_ship_obj,
//         &space_ship_obj,
//         Vector3::new(0.2, 0.2, 0.2),
//     );
//     space_ship.append_translation(&Translation3::new(0.0, 0.0, 4.0));
//     space_ship.append_translation(&Translation3::new(0.0, -1.5, 0.0));
//     space_ship.add_rotation_in_axis(3.15, 'y');
//     let metal_texture = include_bytes!("./resources/textures/metal_texture.png");
//     space_ship.set_texture_from_memory(metal_texture, "metal_texture");
//     space_ship
// }

// fn generate_plantes(planet_number: u32, window: &mut Window) -> LinkedList<SceneNode> {
//     let mut n = 0;
//     let mut rng = rand::thread_rng();

//     let mut planets = LinkedList::new();
//     while n < planet_number {
//         let mut sphere = window.add_sphere(0.1);
//         let sphere_texture = include_bytes!("./resources/textures/sphere_texture.png");
//         sphere.set_texture_from_memory(sphere_texture, "sphere_texture");
//         sphere.append_translation(&Translation3::new(
//             rng.gen_range(-15.0, 15.0),
//             rng.gen_range(-10.0, 10.0),
//             rng.gen_range(-15.0, 15.0),
//         ));
//         n += 1;
//         planets.push_back(sphere);
//     }
//     planets
// }
