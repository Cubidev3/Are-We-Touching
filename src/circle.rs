use crate::circle::CircleError::NegativeRadius;
use crate::gjk::GJKShape2D;

pub struct Circle {
    radius: f32,
    pub center: (f32, f32)
}

pub enum CircleError {
    NegativeRadius
}

impl Circle {
    pub fn new(radius: f32, center: (f32, f32)) -> Option<Circle> {
        if radius < 0f32 {
            return None;
        }

        Some(Circle { radius, center })
    }

    pub fn radius(&self) -> &f32 {
        &self.radius
    }

    pub fn set_radius(&mut self, radius: f32) -> Result<(),CircleError> {
        if radius < 0f32 {
            return Err(NegativeRadius);
        }

        self.radius = radius;
        Ok(())
    }
}

impl GJKShape2D for Circle {
    fn support(&self, (dx, dy): &(f32, f32)) -> (f32, f32) {
        let (x, y) = self.center;
        (x + dx * self.radius(), y + dy * self.radius())
    }
}