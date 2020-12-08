use kiss3d;
use nalgebra as na;

use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use na::{Translation3, UnitQuaternion, Vector3};

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
    fn detect_collision_with(&self, flying_object: &SceneNode) -> bool;
    fn move_obj(&mut self, x: f32, y: f32, z: f32);
}

impl Move for SceneNode {
    fn move_obj(&mut self, x: f32, y: f32, z: f32) {
        self.append_translation(&Translation3::new(x, y, z));
    }

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

    fn detect_collision_with(&self, flying_object: &SceneNode) -> bool {
        if self.data().local_transformation().translation.vector.x
            - flying_object
                .data()
                .local_transformation()
                .translation
                .vector
                .x
            <= 0.8
            && self.data().local_transformation().translation.vector.x
                - flying_object
                    .data()
                    .local_transformation()
                    .translation
                    .vector
                    .x
                >= -0.8
            && self.data().local_transformation().translation.vector.y
                - flying_object
                    .data()
                    .local_transformation()
                    .translation
                    .vector
                    .y
                <= 0.15
            && self.data().local_transformation().translation.vector.y
                - flying_object
                    .data()
                    .local_transformation()
                    .translation
                    .vector
                    .y
                >= -0.15
            && self.data().local_transformation().translation.vector.z
                - flying_object
                    .data()
                    .local_transformation()
                    .translation
                    .vector
                    .z
                <= 0.1
            && self.data().local_transformation().translation.vector.z
                - flying_object
                    .data()
                    .local_transformation()
                    .translation
                    .vector
                    .z
                >= -0.1
        {
            true
        } else {
            false
        }
    }
}
