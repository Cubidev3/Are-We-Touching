use cubi_vectors::vector2::Vector2;
use crate::dim2::aabb::AABB2D;
use crate::dim2::convex::Convex2D;

pub struct Rectangle2D {
    center: Vector2,
    extents: Vector2,
    rotation: f32
}

impl Rectangle2D {
    pub fn new(center: Vector2, extents: Vector2, rotation: f32) -> Rectangle2D {
        Rectangle2D { center, extents, rotation }
    }

    pub fn aabb(&self) -> AABB2D {
        AABB2D::from_polygon(self.vertices())
    }

    pub fn vertices(&self) -> [Vector2; 4] {
        let rotated_extents = self.rotated_extents();
        let rotated_flipped_extents = self.extents.flipped_x().rotated_by(self.rotation);

        [
            self.center + rotated_extents,
            self.center + rotated_flipped_extents,
            self.center - rotated_extents,
            self.center - rotated_flipped_extents
        ]
    }

    fn rotated_extents(&self) -> Vector2 {
        self.extents.rotated_by(self.rotation)
    }
}

impl Convex2D for Rectangle2D {
    fn support(&self, direction: &Vector2) -> Option<Vector2> {
        if direction.is_almost_zero() {
            return None;
        }

        Some(self.vertices().iter().max_by(|a, b| a.dot(&direction).partial_cmp(&b.dot(&direction)).unwrap()).unwrap().clone())
    }
}