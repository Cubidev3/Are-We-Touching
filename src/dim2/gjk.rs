use crate::dim2::convex::{Convex2D, convex_shape_contains_point};

struct ConvexMinkowskiDifference<'a, T: Convex2D, C: Convex2D> {
    a: &'a T,
    b: &'a C
}

impl<'a, T: Convex2D, C: Convex2D> Convex2D for ConvexMinkowskiDifference<'a, T, C> {
    fn support(&self, direction: mint::Vector2<f32>) -> Option<mint::Vector2<f32>> {
        let a_extreme = match self.a.support(direction) {
            Some(point) => point,
            _ => return None
        };

        let b_extreme = match self.b.support(mint::Vector2 { x: -direction.x, y: -direction.y }) {
            Some(point) => point,
            _ => return None
        };

        Some(mint::Vector2 { x: a_extreme.x - b_extreme.x, y: a_extreme.y - b_extreme.y })
    }
}

pub fn convex_shapes_overlap<T: Convex2D, C: Convex2D>(a: &T, b: &C) -> bool {
    let minkowski_difference = ConvexMinkowskiDifference { a, b };
    convex_shape_contains_point(&minkowski_difference, mint::Vector2 { x: 0.0, y: 0.0 })
}