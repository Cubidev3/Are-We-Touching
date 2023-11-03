extern crate core;

mod dim2;

#[cfg(test)]
pub mod dim2_tests {
    use cubi_vectors::vector2::Vector2;
    use crate::dim2::convex::convex_shape_contains_point;
    use crate::dim2::gjk::convex_shapes_overlap;
    use crate::dim2::shapes::rectangle::Rectangle2D;

    #[test]
    pub fn rectangle() {
        let rect = Rectangle2D::new(Vector2::xy(3f32, 4f32), Vector2::xy(2f32, 1f32), 0f32);
        let point = Vector2::xy(5f32, 4f32);
        assert!(convex_shape_contains_point(&rect, &point))
    }

    #[test]
    pub fn aabb_and_gjk_false() {
        let rect1 = Rectangle2D::new(Vector2::xy(3f32, 4f32), Vector2::xy(2f32, 1f32), 30f32.to_radians());
        let rect2 = Rectangle2D::new(Vector2::xy(30f32, 4f32), Vector2::xy(2f32, 1f32), -30f32.to_radians());

        let gjk = convex_shapes_overlap(&rect1, &rect2);
        let aabb = rect1.aabb().collides_with(&rect2.aabb());

        assert_eq!(gjk, aabb)
    }
}