mod world;

extern crate nalgebra as na;
use crate::world::generator;
use crate::world::graphic::Move;
use crate::world::graphic::VisualEngine as ve;
use kiss3d::event::{Action, Key, WindowEvent};
use kiss3d::scene::SceneNode;
use na::Translation3;
use std::collections::LinkedList;
use std::mem;

use rand::Rng;

fn main() {
    let mut window = ve::create_window(
        "Space killer",
        kiss3d::light::Light::StickToCamera,
        0.0,
        0.0,
        0.0,
    );

    let mut space_ship = generator::create_spaceship(&mut window);
    let mut planets = generator::generate_plantes(10, &mut window);
    let mut rng = rand::thread_rng();
    // generator::move_spaceship(&mut space_ship,0.0,0.0,0.0);
    while window.render() {
        for planet in &mut planets {
            planet.add_rotation_in_axis(rng.gen_range(0.0, 0.01), 'y');
        }
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(Key::W, Action::Press, _) => {
                    generator::move_spaceship(&mut space_ship, 0.0, 0.05, 0.0);
                }
                WindowEvent::Key(Key::S, Action::Press, _) => {
                    generator::move_spaceship(&mut space_ship, 0.0, -0.05, 0.0);
                }
                WindowEvent::Key(Key::A, Action::Press, _) => {
                    generator::move_spaceship(&mut space_ship, 0.05, 0.0, 0.0);
                }
                WindowEvent::Key(Key::D, Action::Press, _) => {
                    generator::move_spaceship(&mut space_ship, -0.05, 0.0, 0.0);
                }
                WindowEvent::MouseButton(button, Action::Release, mods) => {
                    println!("You released the mouse button: {:?}", button);
                    println!("You released the mouse button with modifiers: {:?}", mods);
                    // dont override the default mouse handler
                }
                _ => {}
            }
        }
        generator::move_planets(&mut planets);
    }
}
