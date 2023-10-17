use std::ops::Neg;

pub fn dot_product((x, y): &(f32, f32), (a, b): &(f32, f32)) -> f32 {
    x * a + y * b
}

pub fn normalized_vector((x, y): &(f32, f32)) -> Option<(f32, f32)> {
    let len = length_of_vector(&(*x, *y));

    if len == 0f32 {
        return None;
    }

    Some((x / len, y / len))
}

pub fn length_of_vector((x, y): &(f32, f32)) -> f32 {
    (x * x + y * y).sqrt()
}

pub fn opposite_vector((x, y): &(f32, f32)) -> (f32, f32) {
    (x.neg(), y.neg())
}

pub fn vector_from_points((initial_x, initial_y): &(f32, f32), (final_x, final_y): &(f32, f32)) -> (f32, f32) {
    (final_x - initial_x, final_y - initial_y)
}

pub fn triple_cross_product((x, y): &(f32, f32), (a, b): &(f32, f32), (k, l): &(f32, f32)) -> (f32, f32) {
    (a*l*y - x*l*b, b*k*x - y*k*a)
}

pub fn rotated_vector((x, y): &(f32, f32), rotation: &f32) -> (f32, f32) {
    (x * rotation.cos() - y * rotation.sin(), x * rotation.sin() + y * rotation.cos())
}
