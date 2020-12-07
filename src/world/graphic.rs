use kiss3d;
use nalgebra as na;

use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use na::{UnitQuaternion, Vector3};
use std::env;

// here i wrote parts which can be used in future as separate crate
pub struct VisualEngine {}

impl VisualEngine {
    pub fn create_window(name: &str, x: kiss3d::light::Light, r: f32, g: f32, b: f32) -> Window {
        let mut window = Window::new(name);
        window.set_light(x);
        window.set_background_color(r, g, b);
        window
    }
}

pub trait Move {
    fn add_rotation_in_axis(&mut self, speed: f32, axis: char);
    fn detect_collision_with(&self, flying_object: &SceneNode);
}

impl Move for SceneNode {
    fn add_rotation_in_axis(&mut self, speed: f32, axis: char) {
        let rot;
        match axis {
            'x' => rot = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), speed),
            'y' => rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), speed),
            'z' => rot = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), speed),
            _ => return,
        }
        self.prepend_to_local_rotation(&rot);
    }

    fn detect_collision_with(&self, flying_object: &SceneNode) {
        println!(
            "{}",
            self.data().local_transformation().translation.vector.x
                - flying_object
                    .data()
                    .local_transformation()
                    .translation
                    .vector
                    .x,
        );

        println!(
            "{}",
            self.data().local_transformation().translation.vector.x
                - flying_object
                    .data()
                    .local_transformation()
                    .translation
                    .vector
                    .y,
        );

        println!(
            "{}",
            self.data().local_transformation().translation.vector.x
                - flying_object
                    .data()
                    .local_transformation()
                    .translation
                    .vector
                    .z,
        );

        if self.data().local_transformation().translation.vector.x
            - flying_object
                .data()
                .local_transformation()
                .translation
                .vector
                .x
            <= 1.0
            && self.data().local_transformation().translation.vector.x
                - flying_object
                    .data()
                    .local_transformation()
                    .translation
                    .vector
                    .x
                >= 0.00
            && self.data().local_transformation().translation.vector.y
                - flying_object
                    .data()
                    .local_transformation()
                    .translation
                    .vector
                    .y
                <= 1.0
            && self.data().local_transformation().translation.vector.y
                - flying_object
                    .data()
                    .local_transformation()
                    .translation
                    .vector
                    .y
                >= 0.00
            && self.data().local_transformation().translation.vector.z
                - flying_object
                    .data()
                    .local_transformation()
                    .translation
                    .vector
                    .z
                <= 1.0
            && self.data().local_transformation().translation.vector.z
                - flying_object
                    .data()
                    .local_transformation()
                    .translation
                    .vector
                    .z
                >= 1.0
        {
            println!("collision detected");
            quit::with_code(1);
        }
    }
}
