use hashbrown::{hash_map::HashMap, HashSet};

#[derive(Debug, Default, Clone)]
pub struct Node {
    // For each dimension and direction, the indices of the neighbor on the opposing dimension
    forward_neighbors: [Option<usize>; 2],
    backward_neighbors: [Option<usize>; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub(crate) [usize; 2]);

#[derive(Debug)]
pub enum TableError {
    DimensionOutOfBounds(usize),
    NoEnabledNodeForId(NodeId),
}

#[derive(Debug, Default)]
pub(crate) struct NodeBundle {
    pub(crate) id: usize,              // index of the query dimension (entity)
    pub(crate) nodes: HashSet<NodeId>, // node id (components)
}

#[derive(Debug, Default)]
pub struct NodeFilter {
    pub get: HashSet<usize>,
    pub with: HashSet<usize>,
    pub without: HashSet<usize>,
}

#[derive(Debug)]
pub struct Table {
    first_nodes: [HashSet<NodeId>; 2],
    nodes: HashMap<NodeId, Node>,
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Table {
    pub fn new() -> Self {
        let first_nodes: [HashSet<NodeId>; 2] = Default::default();

        Self {
            first_nodes,
            nodes: HashMap::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub(crate) fn get_dimension_at_indices(
        &self,
        dim: usize,
        filter: NodeFilter,
    ) -> Result<Vec<NodeBundle>, TableError> {
        if dim > 1 {
            return Err(TableError::DimensionOutOfBounds(dim));
        }

        let mut node_bundles: Vec<NodeBundle> = Vec::new();
        let mut node_bundle_verification_map: HashMap<usize, usize> = HashMap::new(); // key - index; value - offset

        let mut current_node_ids: HashMap<usize, Option<NodeId>> = HashMap::new();
        let mut nearest_node_pos = usize::MAX;

        let mut node_filter_bitflag: usize = 0;

        for (offset, index) in filter
            .get
            .iter()
            .chain(filter.with.iter().chain(filter.without.iter()))
            .enumerate()
        {
            let try_first_node_id = self.first_nodes[dim]
                .iter()
                .find(|&node_id| node_id.0[dim] == *index);
            if let Some(node_id) = try_first_node_id {
                nearest_node_pos = nearest_node_pos.min(node_id.0[1 - dim]);
                current_node_ids.insert(*index, try_first_node_id.cloned());
            }
            node_bundle_verification_map.insert(*index, offset);

            if !filter.without.contains(index) {
                node_filter_bitflag |= 1 << offset;
            }
        }

        let number_of_indices = current_node_ids.len();

        loop {
            for node_id in current_node_ids.values().flatten() {
                nearest_node_pos = nearest_node_pos.min(node_id.0[1 - dim]);
            }

            let mut node_bundle = NodeBundle {
                id: nearest_node_pos,
                nodes: HashSet::new(),
            };
            let mut node_bundle_bitflag: usize = 0;

            let mut capped_lines: usize = 0;
            let mut node_pos_not_nearest: usize = 0;

            for (index, try_node_id) in current_node_ids.iter_mut() {
                if let Some(mut node_id) = try_node_id {
                    // Check alignment
                    if node_id.0[1 - dim] == nearest_node_pos {
                        // Add all filtered aligned nodes to node bundle
                        if filter.get.contains(index) {
                            node_bundle.nodes.insert(node_id);
                        }

                        // Assign to the bundle's bitflag
                        node_bundle_bitflag |= 1 << node_bundle_verification_map[index];

                        // Step node if it has a neighbor
                        if let Some(forward_neighbor_pos) =
                            self.nodes[&node_id].forward_neighbors[dim]
                        {
                            node_id.0[1 - dim] = forward_neighbor_pos;
                            *try_node_id = Some(node_id);
                        } else {
                            *try_node_id = None;
                        }
                    } else {
                        // Update seed for nearest_node_pos
                        node_pos_not_nearest = node_id.0[1 - dim];
                    }
                } else {
                    capped_lines += 1;
                }
            }

            // Verify node bundle
            if node_bundle_bitflag == node_filter_bitflag {
                node_bundles.push(node_bundle);
            }

            if capped_lines == number_of_indices {
                break;
            }
            // Seed nearest_node_pos
            nearest_node_pos = node_pos_not_nearest;
        }

        Ok(node_bundles)
    }

    pub fn enable_node(&mut self, position: [usize; 2]) -> Result<NodeId, TableError> {
        let new_node_id = NodeId(position);

        if self.nodes.contains_key(&new_node_id) {
            return Ok(new_node_id);
        }

        let mut enabled_node = Node::default();

        // Get the node vector for each dimension at the index indicated by [position].
        for (dim, index) in position.iter().enumerate() {
            let mut forward_neighbor_id = self.first_nodes[dim]
                .iter()
                .find(|&node| node.0[dim] == *index)
                .copied();

            let mut backward_neighbor_id: Option<NodeId> = None;

            while let Some(current_node_id) = forward_neighbor_id {
                // Must compare indices on the opposite dimension to see if the given position is being surrounded on the current dimension.
                // This cannot be equal because otherwise the node would already be enabled.
                if current_node_id.0[1 - dim] > position[1 - dim] {
                    break;
                }

                if let Some(current_node) = self.nodes.get(&current_node_id) {
                    backward_neighbor_id = forward_neighbor_id;
                    if let Some(opposing_index) = current_node.forward_neighbors[dim] {
                        let mut neigbor_position = current_node_id.0;
                        // The saved neighbor index is its index on the opposing dimension,
                        // because otherwise they would have the same index, as they are on the same row/column.
                        neigbor_position[1 - dim] = opposing_index;
                        let new_id = NodeId(neigbor_position);
                        if let Some((id, _)) = self.nodes.get_key_value(&new_id) {
                            forward_neighbor_id = Some(*id);
                        } else {
                            return Err(TableError::NoEnabledNodeForId(new_id));
                        }
                    } else {
                        forward_neighbor_id = None;
                    }
                } else {
                    return Err(TableError::NoEnabledNodeForId(current_node_id));
                }
            }

            let backward_neighbor_index: Option<usize> =
                backward_neighbor_id.map(|id| id.0[1 - dim]);

            let forward_neighbor_index: Option<usize> = forward_neighbor_id.map(|id| id.0[1 - dim]);

            // Set new node's neighbors on the current dimension as their indices on the opposing dimension
            enabled_node.backward_neighbors[dim] = backward_neighbor_index;
            enabled_node.forward_neighbors[dim] = forward_neighbor_index;

            // Add new node as a first node if it has no backwards neighbor in this dimension
            if backward_neighbor_id.is_none() {
                self.first_nodes[dim].insert(new_node_id);
            }

            // If the new node's forward neighbor was a first node, this node replaces it
            if let Some(forward_neighbor_id) = forward_neighbor_id {
                self.first_nodes[dim].remove(&forward_neighbor_id);
                if let Some(forward_neighbor) = self.nodes.get_mut(&forward_neighbor_id) {
                    forward_neighbor.backward_neighbors[dim] = Some(position[1 - dim]);
                }
            }

            if let Some(backward_neighbor_id) = backward_neighbor_id {
                if let Some(backward_neighbor) = self.nodes.get_mut(&backward_neighbor_id) {
                    backward_neighbor.forward_neighbors[dim] = Some(position[1 - dim]);
                }
            }
        }

        self.nodes.insert(new_node_id, enabled_node);

        Ok(new_node_id)
    }

    pub fn disable_node(&mut self, node_id: &NodeId) -> Result<NodeId, TableError> {
        if let Some(old_node) = self.nodes.get(node_id).cloned() {
            for (dim, (forward_neighbor_index, backward_neighbor_index)) in old_node
                .forward_neighbors
                .iter()
                .zip(old_node.backward_neighbors.iter())
                .enumerate()
            {
                let forward_neighbor_id: Option<NodeId> = forward_neighbor_index.map(|index| {
                    let mut n_id = *node_id;
                    n_id.0[1 - dim] = index;
                    n_id
                });

                let backward_neighbor_id: Option<NodeId> = backward_neighbor_index.map(|index| {
                    let mut n_id = *node_id;
                    n_id.0[1 - dim] = index;
                    n_id
                });

                if let Some(forward_neighbor_id) = forward_neighbor_id {
                    self.first_nodes[dim].insert(forward_neighbor_id);
                    if let Some(forward_neighbor) = self.nodes.get_mut(&forward_neighbor_id) {
                        forward_neighbor.backward_neighbors[dim] = *backward_neighbor_index;
                    }
                }

                if let Some(backward_neighbor_id) = backward_neighbor_id {
                    if let Some(backward_neighbor) = self.nodes.get_mut(&backward_neighbor_id) {
                        backward_neighbor.forward_neighbors[dim] = *forward_neighbor_index;
                    }
                }

                self.first_nodes[dim].remove(node_id);
            }

            self.nodes.remove(node_id);
        }

        Ok(*node_id)
    }
}
