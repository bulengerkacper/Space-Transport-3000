pub struct Map {
    pub size_x: u128,
    pub size_y: u128,
    pub size_z: u128,
}

impl Map {
    pub fn create(x_size: u128, y_size: u128, z_size: u128) -> Map {
        Map {
            size_x: x_size,
            size_y: y_size,
            size_z: z_size,
        }
    }
}
