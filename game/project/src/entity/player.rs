use nalgebra::Point3;
use tokio::net::TcpStream;
use tokio::stream;

use super::entity::Entity;
use super::entity_id::EntityId;

pub struct Player {
    id: EntityId,
    position: Point3<f32>,
    radius: f32,

    stream: TcpStream,
}

impl Entity for Player {
    fn update(&mut self) {
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

impl Player {
    pub fn new(id: u64, stream: TcpStream, position: Point3<f32>) -> Self {
        Player {
            id: EntityId::new(id),
            position,
            radius: 1.0,

            stream,
        }
    }
}
