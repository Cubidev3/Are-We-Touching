use cubi_vectors::vector2::Vector2;
use crate::dim2::convex::{Convex2D, convex_shape_contains_point};

struct ConvexMinkowskiDifference<'a, T: Convex2D, C: Convex2D> {
    a: &'a T,
    b: &'a C
}

impl<'a, T: Convex2D, C: Convex2D> Convex2D for ConvexMinkowskiDifference<'a, T, C> {
    fn support(&self, direction: Vector2) -> Option<Vector2> {
        let a_extreme = match self.a.support(direction) {
            Some(point) => point,
            _ => return None
        };

        let b_extreme = match self.b.support(-direction) {
            Some(point) => point,
            _ => return None
        };

        Some(a_extreme - b_extreme)
    }
}

pub fn convex_shapes_overlap<T: Convex2D, C: Convex2D>(a: &T, b: &C) -> bool {
    let minkowski_difference = ConvexMinkowskiDifference { a, b };
    convex_shape_contains_point(&minkowski_difference, Vector2::ZERO)
}