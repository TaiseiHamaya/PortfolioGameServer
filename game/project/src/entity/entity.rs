use nalgebra::Point3;

pub trait Entity {
    fn position(&self) -> Point3<f32>;
    fn radius(&self) -> f32;
}
