use std::mem;

use crate::game::client;

pub struct ZonePlayerLogin {
    pub id: u64,
    pub client_cluster: client::Cluster,
}
pub struct ZonePlayerLogout {
    pub entity_id: u64,
}

pub struct ZoneRequestChash {
    pub login_chash: Vec<ZonePlayerLogin>,
    pub logout_chash: Vec<ZonePlayerLogout>,
}

impl ZoneRequestChash {
    pub fn new() -> Self {
        ZoneRequestChash {
            login_chash: Vec::new(),
            logout_chash: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.login_chash.clear();
        self.logout_chash.clear();
    }

    pub fn push_login(&mut self, client: client::Cluster) {
        self.login_chash.push(ZonePlayerLogin { id: client.id(), client_cluster: client });
    }

    pub fn push_logout(&mut self, id: u64) {
        self.logout_chash.push(ZonePlayerLogout { entity_id: id });
    }

    pub fn get_login_chash_take(&mut self) -> Vec<ZonePlayerLogin> {
        mem::take(&mut self.login_chash)
    }

    pub fn get_logout_chash_take(&mut self) -> Vec<ZonePlayerLogout> {
        mem::take(&mut self.logout_chash)
    }
}