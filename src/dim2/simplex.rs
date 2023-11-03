use cubi_vectors::vector2::Vector2;
use crate::dim2::simplex::Simplex2D::{Line, Point, Triangle};

pub enum Simplex2D {
    Point {
        a: Vector2
    },
    Line {
        a: Vector2,
        b: Vector2
    },
    Triangle {
        a: Vector2,
        b: Vector2,
        c: Vector2
    }
}

impl Simplex2D {
    pub fn next(&self, point: Vector2) -> Simplex2D {
        match self {
            Point{ a: b } => Line { a: point, b: b.clone() },
            Line { a: b, b: c} => Triangle { a: point, b: b.clone(), c: c.clone() },
            Triangle { a: b, b: c, c: _ } => Triangle { a: point, b: b.clone(), c: c.clone() }
        }
    }
}

