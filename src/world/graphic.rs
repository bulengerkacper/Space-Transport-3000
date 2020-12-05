extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{Translation2, UnitComplex, UnitQuaternion, Vector3};
use std::path::Path;

pub fn window_maker() {
    let mut window = Window::new("Auto killah");
}