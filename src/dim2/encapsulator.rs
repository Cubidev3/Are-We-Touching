use cubi_vectors::vector2::Vector2;
use crate::dim2::convex::Convex2D;
use crate::dim2::encapsulator::EncapsulationStatus::{ContainsTarget, Continue, DoesntContainTarget};
use crate::dim2::encapsulator::EncapsulatorArea::{AB, AC, InsideSimplex};
use crate::dim2::simplex::Simplex2D;
use crate::dim2::simplex::Simplex2D::{Point, Line, Triangle};

pub struct Encapsulator2D<'a, T: Convex2D> {
    shape: &'a T,
    target: Vector2,
    simplex: Simplex2D
}

pub enum EncapsulationStatus {
    ContainsTarget,
    DoesntContainTarget,
    Continue
}

enum EncapsulatorArea {
    InsideSimplex,
    AB,
    AC
}

impl<'a, T: Convex2D> Encapsulator2D<'a, T> {
    pub fn new(shape: &'a T, target: Vector2) -> Option<Encapsulator2D<'a, T>> {
        let starting_point = match shape.support(mint::Vector2 { x: 1.0, y: 0.0 }) {
            Some(point) => point.into(),
            _ => return None
        };

        Some(Encapsulator2D{ shape, target, simplex: Point { a: starting_point } })
    }

    pub fn update(&mut self) -> Result<EncapsulationStatus, ()> {
        match self.simplex {
            Point { a } => self.form_line(a),
            Line { a, b} => self.form_triangle(a, b),
            Triangle { a, b, c} => self.try_encapsulate(a, b, c)
        }
    }

    fn form_line(&mut self, a: Vector2) -> Result<EncapsulationStatus, ()> {
        let a_to_target = self.target - a;

        if a_to_target.is_almost_zero() {
            return Ok(ContainsTarget);
        }

        let next_point = match self.shape.support(a_to_target.into()) {
            Some(point) => point.into(),
            _ => return Err(())
        };

        if !is_point_past_area(next_point, self.target, a_to_target) {
            return Ok(DoesntContainTarget);
        }

        self.simplex = self.simplex.next(next_point);
        Ok(Continue)
    }

    fn form_triangle(&mut self, a: Vector2, b: Vector2) -> Result<EncapsulationStatus, ()> {
        let a_to_target = self.target - a;
        let a_to_b = b - a;

        let direction = a_to_target.rejection_from(a_to_b);

        if direction.is_almost_zero() {
            return Ok(ContainsTarget);
        }

        let next_point = match self.shape.support(direction.into()) {
            Some(point) => point.into(),
            _ => return Err(())
        };

        if !is_point_past_area(next_point, self.target, direction) {
            return Ok(DoesntContainTarget);
        }

        self.simplex = self.simplex.next(next_point);
        Ok(Continue)
    }

    fn try_encapsulate(&mut self, a: Vector2, b: Vector2, c: Vector2) -> Result<EncapsulationStatus, ()> {
        let a_to_b = b - a;
        let a_to_c = c - a;

        let ab_normal = -(a_to_c.rejection_from(a_to_b));
        let ac_normal = -(a_to_b.rejection_from(a_to_c));

        match self.area_containing_target(a, ab_normal, ac_normal) {
            AB => {
                let new_point: Vector2 = match self.shape.support(ab_normal.into()) {
                    Some(point) => point.into(),
                    _ => return Err(())
                };

                if (new_point - a).is_almost_zero() || (new_point - b).is_almost_zero() {
                    return Ok(DoesntContainTarget);
                }

                self.simplex = Triangle { a: new_point, b: a.clone(), c: b.clone() };
                Ok(Continue)
            },
            AC => {
                let new_point: Vector2 = match self.shape.support(ac_normal.into()) {
                    Some(point) => point.into(),
                    _ => return Err(())
                };

                if (new_point - a).is_almost_zero() || (new_point - c).is_almost_zero() {
                    return Ok(DoesntContainTarget);
                }

                self.simplex = Triangle { a: new_point, b: a.clone(), c: c.clone() };
                Ok(Continue)
            },
            InsideSimplex => Ok(ContainsTarget)
        }
    }

    fn area_containing_target(&self, a: Vector2, ab_normal: Vector2, ac_normal: Vector2) -> EncapsulatorArea {
        let a_to_target = self.target - a;

        if a_to_target.dot(ab_normal).is_sign_positive() {
            return AB;
        }

        if a_to_target.dot(ac_normal).is_sign_positive() {
            return AC;
        }

        InsideSimplex
    }
}

fn is_point_past_area(point: Vector2, limit: Vector2, direction: Vector2) -> bool {
    (point - limit).dot(direction).is_sign_positive()
}