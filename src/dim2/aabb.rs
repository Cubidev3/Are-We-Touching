use cubi_vectors::vector2::Vector2;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct AABB2D {
    center: Vector2,
    extents: Vector2
}

impl AABB2D {
    pub fn new(center: Vector2, extents: Vector2) -> AABB2D {
        AABB2D { center, extents: extents.abs() }
    }

    pub fn from_polygon<const SIZE: usize>(vertices: [Vector2; SIZE]) -> AABB2D {
        let (min_x, min_y, max_x, max_y) = vertices.iter()
            .fold((f32::MAX, f32::MAX, f32::MIN, f32::MIN), |(min_x, min_y, max_x, max_y), Vector2 { x, y }| {
                (x.min(min_x), y.min(min_y), x.max(max_x), y.max(max_y))
            });

        let center = Vector2::from_xy(min_x + max_x, min_y + max_y) / 2f32;
        let extents = Vector2::from_xy(max_x, max_y) - center;

        AABB2D { center, extents }
    }

    pub fn collides_with(&self, other: &AABB2D) -> bool {
        let Vector2 { x: min_x, y: min_y } = self.min_point();
        let Vector2 { x: max_x, y: max_y } = self.max_point();

        let min_other = other.min_point();
        let max_other = other.max_point();

        let x_overlap = min_other.is_x_inside_interval(min_x, max_x) ||
            max_other.is_x_inside_interval(min_x, max_x);

        let y_overlap = min_other.is_y_inside_interval(min_y, max_y) ||
            max_other.is_y_inside_interval(min_y, max_y);

        x_overlap && y_overlap
    }

    fn min_point(&self) -> Vector2 {
        self.center - self.extents
    }

    fn max_point(&self) -> Vector2 {
        self.center + self.extents
    }
}