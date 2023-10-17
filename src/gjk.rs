use crate::gjk::Simplex2DArea::{InsideTriangle, RegionAB, RegionAC};
use crate::math::{dot_product, normalized_vector, opposite_vector, triple_cross_product, vector_from_points};

pub trait GJKShape2D {
    // Gets the closest point in shape in direction of direction vector
    // Use this function with a normalized non zero direction vector
    fn support(&self, normalized_direction: &(f32, f32)) -> (f32, f32);
}

pub fn gjk_check<T: GJKShape2D, K: GJKShape2D>(a: &T, b: &K) -> bool {
    const INITIAL_DIRECTION: (f32, f32) = (1f32, 0f32);
    let mut simplex_point_c = get_minkowski_difference_point(a, b, &INITIAL_DIRECTION);
    let mut simplex_point_b = get_minkowski_difference_point(a, b, &opposite_vector(&INITIAL_DIRECTION));

    if !is_point_past_origin_from_reference_point(&simplex_point_c, &simplex_point_b) {
        return false;
    }

    let bc_vector = vector_from_points(&simplex_point_b, &simplex_point_c);
    let bo_vector = opposite_vector(&simplex_point_b);
    let direction_to_origin = match normalized_vector(&triple_cross_product(&bc_vector, &bo_vector, &bc_vector)) {
        None => return true,
        Some(vector) => vector,
    };

    let mut simplex_point_a = get_minkowski_difference_point(a, b, &direction_to_origin);

    if !is_point_past_origin_from_direction(&direction_to_origin, &simplex_point_a) {
        return false;
    }

    loop {
        let ab = vector_from_points(&simplex_point_a, &simplex_point_b);
        let ac = vector_from_points(&simplex_point_a, &simplex_point_c);
        let ao = opposite_vector(&simplex_point_a);

        let ab_perpendicular = match normalized_vector(&triple_cross_product(&ac, &ab, &ab)) {
            None => return true,
            Some(vector) => vector
        };

        let ac_perpendicular = match normalized_vector(&triple_cross_product(&ab, &ac, &ac)) {
            None => return true,
            Some(vector) => vector
        };

        match simplex_2d_area_containing_origin(&ab_perpendicular, &ac_perpendicular, &ao) {
            RegionAB => {
                let new_simplex_point = get_minkowski_difference_point(a, b, &ab_perpendicular);

                if new_simplex_point == simplex_point_b || new_simplex_point == simplex_point_a {
                    return false;
                }

                simplex_point_c = simplex_point_b;
                simplex_point_b = simplex_point_a;
                simplex_point_a = new_simplex_point;
            },
            RegionAC => {
                let new_simplex_point = get_minkowski_difference_point(a, b, &ac_perpendicular);

                if new_simplex_point == simplex_point_c || new_simplex_point == simplex_point_a {
                    return false;
                }

                simplex_point_b = simplex_point_a;
                simplex_point_a = new_simplex_point;
            },
            InsideTriangle => return true,
        }
    }
}

pub fn get_minkowski_difference_point<T: GJKShape2D, K: GJKShape2D>(a: &T, b: &K, direction: &(f32, f32)) -> (f32, f32) {
    let (ax, ay) = a.support(&direction);
    let (bx, by) = b.support(&opposite_vector(&direction));

    (ax - bx, ay - by)
}

pub fn is_point_past_origin_from_reference_point(reference: &(f32, f32), point: &(f32, f32)) -> bool {
    let vector_to_origin = opposite_vector(&reference);
    dot_product(&vector_to_origin, &point) > 0f32
}

pub fn is_point_past_origin_from_direction(direction: &(f32, f32), point: &(f32, f32)) -> bool {
    dot_product(&direction, &point) > 0f32
}

pub enum Simplex2DArea {
    RegionAB,
    RegionAC,
    InsideTriangle
}

pub fn simplex_2d_area_containing_origin(ab_perpendicular: &(f32, f32), ac_perpendicular: &(f32, f32), ao: &(f32, f32)) -> Simplex2DArea {
    if is_origin_in_area(&ab_perpendicular, &ao) {
        return RegionAB
    }

    if is_origin_in_area(&ac_perpendicular, &ao) {
        return RegionAC;
    }

    InsideTriangle
}

pub fn is_origin_in_area(perpendicular: &(f32, f32), ao: &(f32, f32)) -> bool {
    dot_product(&perpendicular, &ao) > 0f32
}


