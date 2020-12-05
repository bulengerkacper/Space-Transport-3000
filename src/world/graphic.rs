use kiss3d;
use nalgebra as na;

use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use na::{UnitQuaternion, Vector3};

pub struct VisualEngine {}

impl VisualEngine {
    pub fn create_window(name: &str, x: kiss3d::light::Light) -> Window {
        let mut window = Window::new(name);
        window.set_light(x);
        window
    }
}

pub trait Move {
    fn add_rotation_in_axis(&mut self, speed: f32, axis: char);
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
}
