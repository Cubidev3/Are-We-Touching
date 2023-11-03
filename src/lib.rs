extern crate core;

mod dim2;

#[cfg(test)]
pub mod dim2_tests {
    use cubi_vectors::vector2::Vector2;
    use crate::dim2::convex::convex_shape_contains_point;
    use crate::dim2::shapes::rectangle::Rectangle;

    #[test]
    pub fn rectangle() {
        let rect = Rectangle::new(Vector2::xy(3f32, 4f32), Vector2::xy(2f32, 1f32), 0f32);
        let point = Vector2::xy(4f32, 4f32);
        assert!(convex_shape_contains_point(&rect, &point))
    }
}