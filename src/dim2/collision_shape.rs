use crate::dim2::aabb::ToAABB2D;
use crate::dim2::convex::Convex2D;
use crate::dim2::gjk::convex_shapes_overlap;

pub trait CollisionShape2D: ToAABB2D + Convex2D {}

pub fn shapes_overlap<T: CollisionShape2D, C: CollisionShape2D>(a: &T, b: &T) -> bool {
    a.to_aabb().collides_with(b.to_aabb()) && convex_shapes_overlap(a, b)
}