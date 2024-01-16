use std::fmt::Debug;

use crate::{Component, Entity};

pub enum ECSEvent {
    EntitySpawned(Entity),
    ComponentAdded(Entity, usize),
    ComponentChanged(Entity, Box<(dyn Component + 'static)>),
    ComponentRemoved(Entity, Box<(dyn Component + 'static)>),
    EntityDespawned(Entity),
}

impl Debug for ECSEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EntitySpawned(arg0) => f.debug_tuple("EntitySpawned").field(arg0).finish(),
            Self::ComponentAdded(arg0, arg1) => f
                .debug_tuple("ComponentAdded")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::EntityDespawned(arg0) => f.debug_tuple("EntityDespawned").field(arg0).finish(),
            Self::ComponentChanged(arg0, _) => {
                f.debug_tuple("ComponentChanged").field(arg0).finish()
            }
            Self::ComponentRemoved(arg0, _) => {
                f.debug_tuple("ComponentRemoved").field(arg0).finish()
            }
        }
    }
}
