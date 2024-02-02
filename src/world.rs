use std::{any::type_name, fmt::Debug, slice::Iter};

use crate::{
    events::ECSEvent, hashset, table::{NodeBundle, NodeFilter, NodeId, Table}, utils::entity_range::ValidEntityRange, Children, Component, Entity, Parent
};
use hashbrown::{HashMap, HashSet};

#[derive(Default)]
pub struct World {
    valid_entities: Vec<ValidEntityRange>,
    node_table: Table,
    node_data: HashMap<NodeId, Box<(dyn Component + 'static)>>,
    ecs_events: Vec<ECSEvent>,
    reverse_type_lookup: HashMap<usize, &'static str>,
}

impl World {
    pub fn new() -> Self {
        let mut new_world = Self::default();
        new_world
            .valid_entities
            .push(ValidEntityRange::new(0, None));
        new_world
    }

    pub fn enable_component_for_entity<T>(&mut self, entity: Entity, component: T)
    where
        T: Component + 'static,
    {
        let component_hash = T::hash();
        self.reverse_type_lookup
            .insert(component_hash, type_name::<T>());

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

    pub fn disable_component_for_entity<T>(&mut self, entity: Entity)
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

    pub fn alloc_entity(&mut self) -> Entity {
        let entity_id = self.first_valid_entity().unwrap();
        self.remove_valid_entity(entity_id);
        let new_entity = Entity(entity_id);
        self.ecs_events.push(ECSEvent::EntitySpawned(new_entity));
        new_entity
    }

    pub fn dealloc_entity(&mut self, entity: Entity) {
        // deallocate any children
        if let Some(children) = self.entity_component::<Children>(entity).cloned() {
            for child in children.children.iter() {
                self.dealloc_entity(*child);
            }
        }
        
        // delete data for this entity
        if let Ok(component_vector) = self.node_table.get_dimension_at_indices(
            0,
            NodeFilter {
                get: hashset!(entity.0),
                ..Default::default()
            },
        ) {
            for bundle in component_vector.iter() {
                for node_id in bundle.nodes.iter() {
                    if let Ok(node_id) = self.node_table.disable_node(node_id) {
                        self.node_data.remove(&node_id);
                    }
                }
            }
            self.add_valid_entity(entity.0);
            self.ecs_events.push(ECSEvent::EntityDespawned(entity));
        }
    }

    pub fn add_child(&mut self, parent: Entity, child: Entity) {
        if let Some(children) = self.entity_component_mut::<Children>(parent) {
            children.children.insert(child);
            
        } else {
            let mut children = Children { children: HashSet::new() };
            children.children.insert(child);
            self.enable_component_for_entity(parent, children);
        }
        self.enable_component_for_entity(child, Parent {parent: parent});
    }

    pub fn remove_child(&mut self, parent: Entity, child: Entity) {
        if let Some(children) = self.entity_component_mut::<Children>(parent) {
            children.children.remove(&child);
            if children.children.is_empty() {
                self.disable_component_for_entity::<Children>(parent);
            }
        }
        self.disable_component_for_entity::<Parent>(child);
    }

    pub fn entity_component<T>(&self, entity: Entity) -> Option<&T>
    where
        T: Component + 'static,
    {
        let component_hash = T::hash();
        let node_id = NodeId([entity.0, component_hash]);
        self.node_data
            .get(&node_id)
            .map(|data| data.as_any().downcast_ref::<T>().unwrap())
    }

    pub fn entity_component_mut<T>(&mut self, entity: Entity) -> Option<&mut T>
    where
        T: Component + 'static,
    {
        let component_hash = T::hash();
        let node_id = NodeId([entity.0, component_hash]);
        self.node_data
            .get_mut(&node_id)
            .map(|data| data.as_any_mut().downcast_mut::<T>().unwrap())
    }

    pub fn node_to_component<T>(&self, node_id: NodeId) -> Option<&T>
    where
        T: Component + 'static,
    {
        let component_hash = T::hash();
        if component_hash == node_id.0[1] {
            self.node_data
                .get(&node_id)
                .map(|data| data.as_any().downcast_ref::<T>().unwrap())
        } else {
            None
        }
    }

    pub fn node_to_component_mut<T>(&mut self, node_id: NodeId) -> Option<&mut T>
    where
        T: Component + 'static,
    {
        let component_hash = T::hash();
        if component_hash == node_id.0[1] {
            self.node_data
                .get_mut(&node_id)
                .map(|data| data.as_any_mut().downcast_mut::<T>().unwrap())
        } else {
            None
        }
    }

    pub fn unpack<T>(&self, node_bundle: &NodeBundle) -> Option<&T>
    where
        T: Component + 'static,
    {
        let component_hash = T::hash();
        let node_id = NodeId([node_bundle.id, component_hash]);
        if node_bundle.nodes.contains(&node_id) {
            self.node_data
                .get(&node_id)
                .unwrap()
                .as_any()
                .downcast_ref::<T>()
        } else {
            None
        }
    }

    pub fn unpack_mut<T>(&mut self, node_bundle: &NodeBundle) -> Option<&mut T>
    where
        T: Component + 'static,
    {
        let component_hash = T::hash();
        let node_id = NodeId([node_bundle.id, component_hash]);
        if node_bundle.nodes.contains(&node_id) {
            self.node_data
                .get_mut(&node_id)
                .unwrap()
                .as_any_mut()
                .downcast_mut::<T>()
        } else {
            None
        }
    }

    pub fn entity_of(node_bundle: &NodeBundle) -> Entity {
        Entity(node_bundle.id)
    }

    pub fn component_node_bundles(
        &self,
        get: Option<HashSet<usize>>,
        with: Option<HashSet<usize>>,
        without: Option<HashSet<usize>>,
    ) -> Vec<NodeBundle> {
        let component_filter = NodeFilter {
            get: get.unwrap_or_default(),
            with: with.unwrap_or_default(),
            without: without.unwrap_or_default(),
        };

        if let Ok(node_bundle) = self
            .node_table
            .get_dimension_at_indices(1, component_filter)
        {
            node_bundle
        } else {
            Vec::new()
        }
    }

    pub fn ecs_events_iter(&self) -> Iter<ECSEvent> {
        self.ecs_events.iter()
    }

    pub fn tick(&mut self) {
        self.ecs_events = Vec::new();
    }

    fn sort_entity_ranges(&mut self) {
        self.valid_entities
            .sort_by(|a, b| a.lower_bound.cmp(&b.lower_bound));
    }

    fn remove_valid_entity(&mut self, index: usize) {
        for (range_index, old_range) in self.valid_entities.iter_mut().enumerate() {
            if old_range.contains(&index) {
                if let Some(new_range) = old_range.split_at(&index) {
                    if new_range.is_valid() {
                        if old_range.is_valid() {
                            self.valid_entities.push(new_range);
                        } else {
                            *old_range = new_range;
                        }
                    } else if !old_range.is_valid() {
                        self.valid_entities.swap_remove(range_index);
                    }
                } else if !old_range.is_valid() {
                    self.valid_entities.swap_remove(range_index);
                }

                break;
            }
        }
        self.sort_entity_ranges();
    }

    fn add_valid_entity(&mut self, index: usize) {
        let mut range_glob: ValidEntityRange = ValidEntityRange {
            lower_bound: index,
            upper_bound: Some(index),
        };

        let mut new_valid_ranges: Vec<ValidEntityRange> = Vec::new();

        for old_range in self.valid_entities.iter() {
            if !range_glob.merge_with(old_range) {
                new_valid_ranges.push(old_range.clone());
            }
        }

        new_valid_ranges.push(range_glob);

        self.valid_entities = new_valid_ranges;

        self.sort_entity_ranges();
    }

    fn first_valid_entity(&self) -> Option<usize> {
        self.valid_entities
            .first()
            .map(|first_valid_range| first_valid_range.lower_bound)
    }
}

impl Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("World")
            .field("valid_entities", &self.valid_entities)
            .field("table_node_count", &self.node_table.size())
            .field("data_node_count", &self.node_data.len())
            .field("ecs_events_this_tick", &self.ecs_events)
            .finish()
    }
}
