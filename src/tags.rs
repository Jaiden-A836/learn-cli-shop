//! Tags system which provides tag definitions and containers for storage.

use crate::ecs::entity::EntityId;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Tag(&'static str);

impl Tag {
    pub const fn new(name: &'static str) -> Self {
        Tag(name)
    }
}

pub struct TagManager {
    tags: HashMap<EntityId, HashSet<Tag>>,
}

impl TagManager {
    pub fn new() -> Self {
        Self {
            tags: HashMap::new(),
        }
    }

    pub fn add(&mut self, entity: EntityId, tag: Tag) {
        self.tags.entry(entity).or_default().insert(tag);
    }

    pub fn has(&self, entity: EntityId, tag: Tag) -> bool {
        self.tags.get(&entity).is_some_and(|set| set.contains(&tag))
    }

    pub fn remove(&mut self, entity: EntityId, tag: Tag) -> bool {
        // Exit early if entity is not in set
        let Entry::Occupied(mut entry) = self.tags.entry(entity) else {
            return false;
        };

        let removed = entry.get_mut().remove(&tag);

        // Remove map entry if set is empty
        if entry.get().is_empty() {
            entry.remove();
        }

        removed
    }

    pub fn get(&self, entity: EntityId) -> Option<&HashSet<Tag>> {
        self.tags.get(&entity)
    }

    pub fn remove_entity(&mut self, entity: EntityId) {
        self.tags.remove(&entity);
    }
}

impl Default for TagManager {
    fn default() -> Self {
        Self::new()
    }
}

pub mod defs {
    //! Predefines all used and needed tags
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn manager() -> TagManager {
        TagManager::new()
    }

    #[fixture]
    fn entity() -> EntityId {
        EntityId::new(1, 1)
    }

    #[rstest]
    fn has_returns_true_when_added(mut manager: TagManager, entity: EntityId) {
        let tag = Tag::new("foo");
        manager.add(entity, Tag::new("foo"));
        assert!(manager.has(entity, tag));
    }

    #[rstest]
    fn has_returns_false_when_removed(mut manager: TagManager, entity: EntityId) {
        let tag = Tag::new("foo");
        manager.add(entity, tag);
        manager.remove(entity, tag);
        assert!(!manager.has(entity, tag));
    }

    #[rstest]
    fn get_returns_tag_set(mut manager: TagManager, entity: EntityId) {
        let tag = Tag::new("foo");
        manager.add(entity, tag);

        let tags = manager.get(entity).unwrap();
        assert!(tags.contains(&tag));
    }
}
