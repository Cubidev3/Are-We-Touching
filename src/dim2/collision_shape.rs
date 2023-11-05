use crate::dim2::aabb::ToAABB2D;
use crate::dim2::convex::Convex2D;
use crate::dim2::gjk::convex_shapes_overlap;

pub fn shapes_overlap<T: Convex2D + ToAABB2D, C: Convex2D + ToAABB2D>(a: &T, b: &T) -> bool {
    a.to_aabb().collides_with(b.to_aabb()) && convex_shapes_overlap(a, b)
}