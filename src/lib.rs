#![crate_type = "lib"]
#![allow(dead_code, unused_macros, unused_macro_rules)]
pub mod component;
pub mod entity;
pub mod macros;
pub mod table;

pub use component::*;
pub use entity::*;
pub use hashbrown;

use hashbrown::HashMap;
use table::{NodeId, Table};

pub(crate) enum ECSEvent {
    EntitySpawned(Entity),
    ComponentAdded(Entity, usize),
    ComponentChanged(Entity, Box<(dyn Component + 'static)>),
    ComponentRemoved(Entity, Box<(dyn Component + 'static)>),
    EntityDespawned(Entity),
}

#[derive(Default)]
pub struct World {
    entity_count: usize,
    node_table: Table,
    node_data: HashMap<NodeId, Box<(dyn Component + 'static)>>,
    ecs_events: Vec<ECSEvent>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    fn insert_component<T>(&mut self, entity: Entity, component: T)
    where
        T: Component + 'static,
    {
        let component_hash = T::hash();
        let new_node_position = [entity.0, component_hash];
        if let Ok(enabled_node_id) = self.node_table.enable_node(new_node_position) {
            if let Some(old_data) = self.node_data.insert(enabled_node_id, Box::new(component)) {
                self.ecs_events
                    .push(ECSEvent::ComponentChanged(entity, old_data));
            } else {
                self.ecs_events
                    .push(ECSEvent::ComponentAdded(entity, component_hash));
            }
        }
    }

    fn remove_component<T>(&mut self, entity: Entity)
    where
        T: Component + 'static,
    {
        let node_id_to_remove = NodeId([entity.0, T::hash()]);
        if let Ok(disabled_node_id) = self.node_table.disable_node(&node_id_to_remove) {
            if let Some(old_data) = self.node_data.remove(&disabled_node_id) {
                self.ecs_events
                    .push(ECSEvent::ComponentRemoved(entity, old_data));
            }
        }
    }
}
