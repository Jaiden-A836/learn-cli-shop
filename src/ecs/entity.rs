//! Defines entity id which is used for quick lookups.

/// An entity ID equipped with a generation counter to prevent stale references.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct EntityId {
    index: u32,
    generation: u32,
}

impl EntityId {
    pub fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }
}
