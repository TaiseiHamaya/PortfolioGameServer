use crate::game::zone;

pub trait CommandTrait {
    fn execute(&self, zone: &mut zone::Zone);
}

pub struct LogoutRequestCommand {
    player_id: u64,
}

impl LogoutRequestCommand {
    pub fn new(player_id: u64) -> Self {
        LogoutRequestCommand { player_id }
    }
}

impl CommandTrait for LogoutRequestCommand {
    fn execute(&self, zone: &mut zone::Zone) {
        zone.dissconnect_request(&self.player_id);
    }
}

pub struct DisconnectForceCommand {
    player_id: u64,
}

impl DisconnectForceCommand {
    pub fn new(player_id: u64) -> Self {
        DisconnectForceCommand { player_id }
    }
}

impl CommandTrait for DisconnectForceCommand {
    fn execute(&self, zone: &mut zone::Zone) {
        zone.dissconnect_client_force(&self.player_id);
    }
}

pub struct ChatBroadcastCommand {
    id: u64,
    message: String,
}

impl ChatBroadcastCommand {
    pub fn new(id: u64, message: String) -> Self {
        ChatBroadcastCommand { id, message }
    }
}

impl CommandTrait for ChatBroadcastCommand {
    fn execute(&self, zone: &mut zone::Zone) {
        zone.broadcast_chat_message(self.id, &self.message);
    }
}
