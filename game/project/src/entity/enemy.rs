use super::entity::Entity;
use nalgebra::Point3;

pub struct Enemy {
    position: Point3<f32>,
    radius: f32,
}

impl Entity for Enemy {
    fn position(&self) -> Point3<f32> {
        self.position
    }

    fn radius(&self) -> f32 {
        self.radius
    }
}
