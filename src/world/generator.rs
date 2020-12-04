// pub fn create_map() {
//     println!("Shape of you");
// }

pub trait Creation {
    fn create() -> Self;
    fn make_visible() -> bool {
        println!("I am visible");
        true
    }
}

pub struct Map {
    pub x: u128,
    pub y: u128,
    pub z: u128,
}

impl Creation for Map {
    fn create() -> Map {
        Map{x:1,y:1,z:1}
    }
}