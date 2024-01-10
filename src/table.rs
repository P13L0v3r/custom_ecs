#[derive(Debug, Default)]
pub struct Node {
    forward_neighbors: [Option<usize>; Table::DIMENIONS],
    backward_neighbors: [Option<usize>; Table::DIMENIONS],
    position: [usize; Table::DIMENIONS],
}

pub struct Table {
    first_nodes: [Vec<Node>; Table::DIMENIONS],
    middle_nodes: Vec<Node>,
    //last_nodes: Vec<Node>,
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Table {
    const DIMENIONS: usize = 2;

    pub fn new() -> Self {
        const EMPTY_VEC: Vec<Node> = Vec::new();

        Self {
            first_nodes: [EMPTY_VEC; Table::DIMENIONS],
            middle_nodes: Vec::new(),
        }
    }

    fn get_dimension_at_index(
        &self,
        dim: &usize,
        index: &usize,
    ) -> Result<Vec<&Node>, &'static str> {
        if *dim >= Self::DIMENIONS {
            return Err("Table does not contain that many dimensions");
        }

        let mut dimension_at_index: Vec<&Node> = Vec::new();

        let mut try_current_node = self.first_nodes[*dim]
            .iter()
            .find(|node| node.position[*dim] == *index);

        while let Some(current_node) = try_current_node {
            dimension_at_index.push(current_node);

            let next_node_index = current_node.forward_neighbors[*dim];
            
            try_current_node = if let Some(index) = next_node_index {
                Some(&self.middle_nodes[index])
            } else {
                None
            };
        }

        Ok(dimension_at_index)
    }

    fn enable_node(&mut self, position: [usize; Table::DIMENIONS]) {
        // Get the node vector for each dimension at the index indicated by [position].
        for (dim, index) in position.iter().enumerate() {
            // This is safe because we know that [dim] will always be within bounds of [Table::DIMENIONS] because it is an enumeration.
            let node_vector = self.get_dimension_at_index(&dim, index).unwrap();
            // For each node vector, find the nodes that surround the node being enabled by comparing their positions to [position] and
            //     update those nodes with their new neighbor, the newly-enabled node.
            let mut backward_neighbor: Option<usize> = None;
            let mut forward_neighbor: Option<usize> = None;
        }
        
    }
}
