use std::collections::HashMap;

use crate::entity::entity::Entity;
use crate::entity::enemy::Enemy;

pub struct ContaintsDirector {
    enemies: HashMap<u64, Enemy>,
}

impl ContaintsDirector {
    pub fn update(&mut self) {
        self.enemies.iter_mut().for_each(|(_, enemy)| {
            enemy.update();
        });
    }
}
