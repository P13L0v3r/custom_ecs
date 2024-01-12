use hashbrown::{hash_map::HashMap, HashSet};

#[derive(Debug, Default, Clone)]
pub struct Node {
    // For each dimension and direction, the indices of the neighbor on the opposing dimension
    forward_neighbors: [Option<usize>; 2],
    backward_neighbors: [Option<usize>; 2],
    //position: [usize; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId([usize; 2]);

#[derive(Debug)]
pub enum TableError {
    DimensionOutOfBounds(usize),
    NoEnabledNodeForId(NodeId),
}

#[derive(Debug)]
pub struct Table {
    first_nodes: [HashSet<NodeId>; 2],
    nodes: HashMap<NodeId, Node>,
    //last_nodes: Vec<Node>,
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

    pub fn get_dimension_at_index(
        &self,
        dim: &usize,
        index: &usize,
    ) -> Result<Vec<NodeId>, TableError> {
        if *dim > 1 {
            return Err(TableError::DimensionOutOfBounds(*dim));
        }

        let mut dimension_at_index: Vec<NodeId> = Vec::new();

        let mut try_first_node_id = self.first_nodes[*dim]
            .iter()
            .find(|&node| node.0[*dim] == *index);

        while let Some(current_node_id) = try_first_node_id {
            if let Some(current_node) = self.nodes.get(current_node_id) {
                dimension_at_index.push(*current_node_id);
                if let Some(position) = current_node.forward_neighbors[*dim] {
                    let mut neigbor_position = current_node_id.0;
                    neigbor_position[1 - *dim] = position;
                    let new_id = NodeId(neigbor_position);
                    if let Some((id, _)) = self.nodes.get_key_value(&new_id) {
                        try_first_node_id = Some(id);
                    } else {
                        return Err(TableError::NoEnabledNodeForId(new_id));
                    }
                } else {
                    try_first_node_id = None;
                }
            } else {
                return Err(TableError::NoEnabledNodeForId(*current_node_id));
            }
        }

        Ok(dimension_at_index)
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
        } else {
            return Err(TableError::NoEnabledNodeForId(*node_id));
        }

        self.nodes.remove(node_id);

        Ok(*node_id)
    }
}
