use crate::gjk::GJKShape2D;
use crate::math::{dot_product, rotated_vector};

pub struct ConvexPolygon {
    pub points: Vec<(f32, f32)>
}

impl ConvexPolygon {
    pub fn new_polygon(points: Vec<(f32, f32)>) -> ConvexPolygon {
        ConvexPolygon { points }
    }

    pub fn new_rectangle((ext_x, ext_y): (f32, f32), (x, y): (f32, f32), rotation: f32) -> ConvexPolygon {
        let points = vec![
            (x + ext_x, y + ext_y),
            (x + ext_x, y - ext_y),
            (x - ext_x, y - ext_y),
            (x - ext_x, y + ext_y)
        ];

        if rotation == 0f32 {
            return ConvexPolygon { points };
        }

        let points: Vec<(f32, f32)> = points.iter()
            .map(|vector: &(f32, f32)| rotated_vector(&vector, &rotation))
            .collect();

        ConvexPolygon { points }
    }
}

impl GJKShape2D for ConvexPolygon {
    fn support(&self, normalized_direction: &(f32, f32)) -> (f32, f32) {
        self.points.iter()
            .max_by(|a, b| dot_product(&a, &normalized_direction).total_cmp(&dot_product(&b, &normalized_direction)))
            .copied().unwrap()
    }
}

