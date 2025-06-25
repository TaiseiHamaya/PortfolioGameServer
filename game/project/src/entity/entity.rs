use nalgebra::Point3;

pub trait Entity {
    fn update(&mut self) -> ();

    fn position(&self) -> Point3<f32>;
    fn radius(&self) -> f32;
    fn id(&self) -> u64;
}
