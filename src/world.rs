use std::{fmt::Debug, slice::Iter};

use crate::{
    events::ECSEvent,
    hashset,
    table::{NodeFilter, NodeId, Table},
    Component, Entity,
};
use hashbrown::HashMap;

#[derive(Debug, Clone)]
struct ValidEntityRange {
    lower_bound: usize,
    upper_bound: Option<usize>,
}

impl ValidEntityRange {
    fn new(lower_bound: usize, upper_bound: Option<usize>) -> Self {
        Self {
            lower_bound,
            upper_bound,
        }
    }

    fn split_at(&mut self, index: &usize) -> Option<ValidEntityRange> {
        if *index >= self.lower_bound {
            let new_range = match *index == self.lower_bound {
                false => Some(ValidEntityRange {
                    lower_bound: self.lower_bound,
                    upper_bound: Some(index - 1),
                }),
                true => None,
            };
            self.lower_bound = index + 1;
            new_range
        } else {
            None
        }
    }

    fn merge_with(&mut self, other: &ValidEntityRange) -> bool {
        if self.touches(other) {
            let max_upper_bound: Option<usize> = match (self.upper_bound, other.upper_bound) {
                (None, None) => None,
                (None, Some(_)) | (Some(_), None) => None,
                (Some(a), Some(b)) => Some(a.max(b)),
            };
            let min_lower_bound: usize = self.lower_bound.min(other.lower_bound);

            self.lower_bound = min_lower_bound;
            self.upper_bound = max_upper_bound;

            true
        } else {
            false
        }
    }

    fn contains(&self, index: &usize) -> bool {
        if *index >= self.lower_bound {
            if let Some(upper_bound) = self.upper_bound {
                *index <= upper_bound
            } else {
                true
            }
        } else {
            false
        }
    }

    fn intersects(&self, other: &ValidEntityRange) -> bool {
        let min_upper_bound: Option<usize> = match (self.upper_bound, other.upper_bound) {
            (None, None) => None,
            (None, Some(a)) | (Some(a), None) => Some(a),
            (Some(a), Some(b)) => Some(a.min(b)),
        };
        let max_lower_bound: usize = self.lower_bound.max(other.lower_bound);

        if let Some(upper_bound) = min_upper_bound {
            max_lower_bound <= upper_bound
        } else {
            true
        }
    }

    fn touches(&self, other: &ValidEntityRange) -> bool {
        let min_upper_bound: Option<usize> = match (self.upper_bound, other.upper_bound) {
            (None, None) => None,
            (None, Some(a)) | (Some(a), None) => Some(a),
            (Some(a), Some(b)) => Some(a.min(b)),
        };
        let max_lower_bound: usize = self.lower_bound.max(other.lower_bound);

        if let Some(upper_bound) = min_upper_bound {
            max_lower_bound <= upper_bound || max_lower_bound.abs_diff(upper_bound) <= 1
        } else {
            true
        }
    }

    fn is_valid(&self) -> bool {
        if let Some(upper_bound) = self.upper_bound {
            self.lower_bound <= upper_bound
        } else {
            true
        }
    }
}

#[derive(Default)]
pub struct World {
    valid_entities: Vec<ValidEntityRange>,
    node_table: Table,
    node_data: HashMap<NodeId, Box<(dyn Component + 'static)>>,
    ecs_events: Vec<ECSEvent>,
}

impl World {
    pub fn new() -> Self {
        let mut new_world = Self::default();
        new_world
            .valid_entities
            .push(ValidEntityRange::new(0, None));
        new_world
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

    pub fn enable_component_for_entity<T>(&mut self, entity: Entity, component: T)
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

    pub fn query_entity_component<T>(&self, entity: Entity) -> Option<&T>
    where
        T: Component + 'static,
    {
        let component_hash = T::hash();
        let node_id = NodeId([entity.0, component_hash]);
        self.node_data
            .get(&node_id)
            .map(|data| data.as_any().downcast_ref::<T>().unwrap())
    }

    pub fn query_entity_component_mut<T>(&mut self, entity: Entity) -> Option<&mut T>
    where
        T: Component + 'static,
    {
        let component_hash = T::hash();
        let node_id = NodeId([entity.0, component_hash]);
        self.node_data
            .get_mut(&node_id)
            .map(|data| data.as_any_mut().downcast_mut::<T>().unwrap())
    }

    /* pub fn query(&self, components: ) {

    } */

    pub fn ecs_events_iter(&self) -> Iter<ECSEvent> {
        self.ecs_events.iter()
    }

    pub fn tick(&mut self) {
        self.ecs_events = Vec::new();
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
