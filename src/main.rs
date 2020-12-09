mod world;

extern crate nalgebra as na;
use crate::world::generator;
use crate::world::graphic::Move;
use crate::world::graphic::VisualEngine as ve;
use kiss3d::camera::FirstPerson;
use kiss3d::event::{Action, Key, WindowEvent};
use kiss3d::text::Font;
use na::{Point2, Point3};
use rand::Rng;

fn main() {
    let font = Font::default();
    let mut game_window = ve::create_window(
        "Space killer",
        kiss3d::light::Light::StickToCamera,
        0.0,
        0.0,
        0.0,
    );
    let mut space_ship = generator::create_spaceship(&mut game_window);
    let mut camera = prepare_camera();
    game_window.draw_text(
        "GAME LOADING. Please wait",
        &Point2::new(400.0, 400.0),
        120.0,
        &font,
        &Point3::new(10.0, 200.0, 0.0),
    );
    while game_window.render_with_camera(&mut camera) {
        if space_ship.data().has_object() {  //GAME LOADING
            break;
        }
    }
    let mut planets = generator::generate_plantes(50, &mut game_window);
    let mut rng = rand::thread_rng();
    let mut speed_of_plantes: f32 = 0.00;
    while game_window.render() {
        for planet in &mut planets {
            planet.add_rotation_in_axis(rng.gen_range(0.0, 0.01), 'y');
        }
        for mut event in game_window.events().iter() {
            match event.value {
                WindowEvent::Key(Key::W, Action::Press, _) => {
                    space_ship.move_obj(0.0, 0.05, 0.0);
                }
                WindowEvent::Key(Key::S, Action::Press, _) => {
                    space_ship.move_obj(0.0, -0.05, 0.0);
                }
                WindowEvent::Key(Key::A, Action::Press, _) => {
                    space_ship.move_obj(0.05, 0.0, 0.0);
                }
                WindowEvent::Key(Key::D, Action::Press, _) => {
                    space_ship.move_obj(-0.05, 0.0, 0.0);
                }
                WindowEvent::Key(Key::Z, Action::Press, _) => {
                    space_ship.add_rotation_in_axis(0.1, 'z');
                }
                WindowEvent::Key(Key::C, Action::Press, _) => {
                    space_ship.add_rotation_in_axis(-0.1, 'z');
                }
                WindowEvent::Key(Key::Space, Action::Press, _) => {
                    speed_of_plantes *= 1.0001;
                }
                WindowEvent::Key(Key::L, Action::Press, _) => {
                    quit::with_code(1);
                }
                WindowEvent::CursorPos(_x, _y, _) | WindowEvent::Scroll(_x, _y, _) => {
                    event.inhibited = true;
                }
                _ => {}
            }
        }
        speed_of_plantes -= 0.00001;
        let points = format!(
            "{:.7} of c\nSTERING\n  W \n ASD\n Z SPACE C",
            speed_of_plantes.abs()
        );
        let pts: &str = &points[..];
        game_window.draw_text(
            pts,
            &Point2::new(43.0, 43.0),
            90.0,
            &font,
            &Point3::new(10.0, 0.0, 0.0),
        );
        if generator::move_planets(&space_ship, &mut planets, speed_of_plantes) {
            speed_of_plantes = 0.0;
            game_window.draw_text(
                "YOU LOSE!\n MOVE TO RESTART\n L to LEAVE",
                &Point2::new(600.0, 600.0),
                120.0,
                &font,
                &Point3::new(10.0, 200.0, 0.0),
            );
        }
    }
}

fn prepare_camera() -> FirstPerson {
    let eye = Point3::new(0.0, 0.0, 0.0);
    let at = Point3::origin();
    let mut _first_person = FirstPerson::new(eye, at);
    _first_person
}
