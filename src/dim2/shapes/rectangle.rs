use cubi_vectors::vector2::Vector2;
use crate::dim2::convex::Convex2D;

pub struct Rectangle {
    center: Vector2,
    extents: Vector2,
    rotation: f32
}

impl Rectangle {
    pub fn new(center: Vector2, extents: Vector2, rotation: f32) -> Rectangle {
        Rectangle { center, extents, rotation }
    }
}

impl Convex2D for Rectangle {
    fn support(&self, direction: &Vector2) -> Option<Vector2> {
        if direction.is_almost_zero() {
            return None;
        }

        let rotated_direction = direction.rotated_by(-self.rotation);
        let vertex = self.extents.with_sign_of(&rotated_direction);

        Some(self.center + vertex.rotated_by(self.rotation))
    }
}