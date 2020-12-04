pub trait Visibility {
    fn make_visible(&self, can_be_seen: bool) -> bool;
}

pub struct Map {
    pub size_x: u128,
    pub size_y: u128,
    pub size_z: u128,
}

impl Map {
    pub fn create(x_size:u128,y_size:u128,z_size:u128) -> Map {
        Map { size_x: x_size, size_y: y_size, size_z: z_size }
    }
}

impl Visibility for Map {
    fn make_visible(&self, can_be_seen: bool) -> bool {
        if(can_be_seen) {

        } else {

        }
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
        if(can_be_seen) {

        } else {
            
        }
        true
    }
}
