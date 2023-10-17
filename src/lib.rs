mod gjk;
mod math;
mod polygon;
mod circle;

#[cfg(test)]
mod tests {
    use crate::circle::Circle;
    use crate::gjk::gjk_check;
    use crate::polygon::ConvexPolygon;

    #[test]
    fn test_gjk_collision_detection_polygon_polygon_collision() {
        // Create two convex polygons that intersect
        let polygon_a = ConvexPolygon::new_polygon(vec![(-1.0, 0.0), (0.0, 1.0), (1.0, 0.0)]);
        let polygon_b = ConvexPolygon::new_polygon(vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)]);

        // Check for collision
        assert!(gjk_check(&polygon_a, &polygon_b));
    }

    #[test]
    fn test_gjk_collision_detection_polygon_polygon_no_collision() {
        // Create two convex polygons that do not intersect
        let polygon_a = ConvexPolygon::new_polygon(vec![(-1.0, 0.0), (0.0, 1.0), (1.0, 0.0)]);
        let polygon_b = ConvexPolygon::new_polygon(vec![(2.0, 2.0), (3.0, 3.0), (4.0, 2.0)]);

        // Check for no collision
        assert!(!gjk_check(&polygon_a, &polygon_b));
    }

    #[test]
    fn test_gjk_collision_detection_circle_polygon_collision() {
        // Create a circle and a convex polygon that intersect
        let circle = Circle::new(1.0, (0.0, 0.0)).unwrap();
        let polygon = ConvexPolygon::new_polygon(vec![(-1.0, 0.0), (0.0, 1.0), (1.0, 0.0)]);

        // Check for collision
        assert!(gjk_check(&circle, &polygon));
    }

    #[test]
    fn test_gjk_collision_detection_circle_polygon_no_collision() {
        // Create a circle and a convex polygon that do not intersect
        let circle = Circle::new(1.0, (0.0, 0.0)).unwrap();
        let polygon = ConvexPolygon::new_polygon(vec![(2.0, 2.0), (3.0, 3.0), (4.0, 2.0)]);

        // Check for no collision
        assert!(!gjk_check(&circle, &polygon));
    }

    // Add more tests as needed to cover various scenarios and edge cases.
}

