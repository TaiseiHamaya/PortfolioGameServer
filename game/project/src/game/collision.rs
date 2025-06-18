use crate::entity::entity::Entity;
use nalgebra::Point3;

pub fn is_hit_entity<T>(entity: T, pos: Point3<f32>) -> bool 
where
    T: Entity,
{
    let entity_pos = entity.position();
    let entity_radius = entity.radius();

    let distance = (entity_pos - pos).norm();
    if distance < entity_radius {
        return true;
    }
    return false;
}
