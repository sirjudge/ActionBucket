//NOTE: random_range(..) works by accepting a rust range
// the `..` range notation is follows
// ( inclusive .. exclusive)
// to make sure the max num is inclusive we just add 1
// This is not ideal at all but think is fine since only f32 is gonna be wonk
// (I think)
pub fn generate_rand_i8(min: i8, max: i8) -> i8 {
    rand::random_range(min..max+1)
}

pub fn generate_rand_i16(min: i16, max: i16) -> i16 {
    rand::random_range(min..max+1)
}

pub fn generate_rand_f32(min: f32, max: f32) -> f32 {
    rand::random_range(min..max+1.0)
}
