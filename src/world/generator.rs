pub trait Visibility {
    fn make_visible(&self, can_be_seen: bool) -> bool;
}

pub struct Map {
    pub x: u128,
    pub y: u128,
    pub z: u128,
}

impl Map {
    pub fn create() -> Map {
        Map { x: 1, y: 1, z: 1 }
    }
}

impl Visibility for Map {
    fn make_visible(&self, can_be_seen: bool) -> bool {
        println!("I am there");
        true
    }
}

pub struct Element {
    pub size_x: u32,
    pub size_y: u32,
    pub begin_of_drawing_x: u32,
    pub begin_of_drawing_y: u32,
}

impl Element {
    pub fn create(x_size: u32, y_size: u32, begin_x: u32, begin_y: u32) -> Element {
        Element {
            size_x: x_size,
            size_y: y_size,
            begin_of_drawing_x: begin_x,
            begin_of_drawing_y: begin_y,
        }
    }
}

impl Visibility for Element {
    fn make_visible(&self, can_be_seen: bool) -> bool {
        println!("I am there");
        true
    }
}
