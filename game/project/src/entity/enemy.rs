use nalgebra::Point3;
use super::entity::Entity;
use super::entity_id::EntityId;

pub struct Enemy {
    id: EntityId,
    position: Point3<f32>,
    radius: f32,
}

impl Enemy {
    pub fn new(id: u64, position: Point3<f32>, radius: f32) -> Self {
        Enemy {
            id: EntityId::new(id),
            position,
            radius,
        }
    }
}

impl Entity for Enemy {
    fn update(&mut self) {
        // Update logic for the player can be added here
    }
    
    fn position(&self) -> Point3<f32> {
        self.position
    }

    fn radius(&self) -> f32 {
        self.radius
    }

    fn id(&self) -> u64 {
        self.id.id()
    }
}
