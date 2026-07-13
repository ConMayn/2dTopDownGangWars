//! ECS — tynd wrapper om hecs med hjælpemetoder.
//!
//! World ejer alle entiteter og komponenter. Systems kører queries over World.

use hecs::{Entity, World as HecsWorld};

/// Wrapper om hecs::World med convenience-metoder.
pub struct World {
    inner: HecsWorld,
}

impl World {
    pub fn new() -> Self {
        Self {
            inner: HecsWorld::new(),
        }
    }

    pub fn inner(&self) -> &HecsWorld {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut HecsWorld {
        &mut self.inner
    }

    /// Spawn en entitet med et enkelt komponent.
    pub fn spawn_one<T: Component>(&mut self, component: T) -> Entity {
        self.inner.spawn((component,))
    }

    /// Spawn en entitet med flere komponenter (tuple).
    pub fn spawn<T: hecs::DynamicBundle>(&mut self, components: T) -> Entity {
        self.inner.spawn(components)
    }

    /// Despawn en entitet.
    pub fn despawn(&mut self, entity: Entity) -> bool {
        self.inner.despawn(entity).is_ok()
    }

    pub fn entity_count(&self) -> usize {
        self.inner.len() as usize
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

/// Re-export hecs Component-trait.
pub use hecs::Component;

/// Re-export hecs Entity.
pub use hecs::Entity as EntityId;