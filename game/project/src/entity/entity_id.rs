#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EntityId {
    id: u64,
}

impl EntityId {
    pub fn new(id: u64) -> Self {
        EntityId { id }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}
