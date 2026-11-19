pub fn generate_rand_i8(min: i8, max: i8) -> i8 {
    rand::random_range(min..max)
}

pub fn generate_rand_i16(min: i16, max: i16) -> i16 {
    rand::random_range(min..max)
}

pub fn generate_rand_f32(min: f32, max: f32) -> f32 {
    rand::random_range(min..max)
}
