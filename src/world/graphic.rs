use kiss3d;
use nalgebra as na;

use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{UnitQuaternion, Vector3};

pub fn window_maker() {
    let mut window = Window::new("Auto killah");
    window.set_background_color(0.0,0.0,0.0);
    let mut main_cube = window.add_cube(0.4,0.2,0.5);
    main_cube.set_color(0.0,102.0,0.0);
    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render() {
        main_cube.prepend_to_local_rotation(&rot);
    }
}

pub fn add_rotation_in_x() {

}

pub fn add_rotation_in_y() {

}

pub fn add_rotation_in_z() {
    
}