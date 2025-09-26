use nalgebra::Point3;

pub trait Entity {
    fn update(&mut self) -> ();

    fn on_damaged(&mut self, damage: i32) -> ();

    fn position(&self) -> &Point3<f32>;
    fn position_mut(&mut self) -> &mut Point3<f32>;
    fn radius(&self) -> f32;
    fn id(&self) -> u64;
}
