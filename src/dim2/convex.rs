use mint::Vector2;
use crate::dim2::encapsulator::Encapsulator2D;
use crate::dim2::encapsulator::EncapsulationStatus::*;

pub trait Convex2D {
    // Returns furthest vertex in specified direction
    fn support(&self, direction: Vector2<f32>) -> Option<Vector2<f32>>;
}

pub fn convex_shape_contains_point<T: Convex2D>(shape: &T, point_to_find: Vector2<f32>) -> bool {
    let mut encapsulator = match Encapsulator2D::new(shape, point_to_find.into()) {
        Some(encapsulator) => encapsulator,
        _ => return false
    };

    loop {
        match encapsulator.update() {
            Ok(ContainsTarget) => return true,
            Ok(DoesntContainTarget) => return false,
            Ok(Continue) => continue,
            _ => return false
        }
    }
}