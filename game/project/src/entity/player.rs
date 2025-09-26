use nalgebra::Point3;

use super::entity::Entity;
use super::entity_id::EntityId;

pub struct Player {
    id: EntityId,
    position: Point3<f32>,
    radius: f32,
    hitpoint: i32,
}

impl Entity for Player {
    fn update(&mut self) {
    }

    fn on_damaged(&mut self, damage: i32) -> () {
        self.hitpoint -= damage;
        if self.hitpoint < 0 {
            self.hitpoint = 0;
            return;
        }
    }

    fn position(&self) -> &Point3<f32> {
        &self.position
    }
    fn position_mut(&mut self) -> &mut Point3<f32> {
        &mut self.position
    }
    fn radius(&self) -> f32 {
        self.radius
    }
    fn id(&self) -> u64 {
        self.id.id()
    }
}

impl Player {
    pub fn new(id: u64, position: Point3<f32>, hitpoint: i32) -> Self {
        Player {
            id: EntityId::new(id),
            position,
            radius: 1.0,
            hitpoint,
        }
    }
}
