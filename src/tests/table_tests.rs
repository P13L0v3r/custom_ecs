use hashbrown::HashSet;

use crate::{
    hashset,
    table::{NodeFilter, Table, TableError},
};

#[test]
pub fn table_test() -> Result<(), TableError> {
    let mut new_table = Table::new();

    new_table.enable_node([0, 0])?;
    new_table.enable_node([1, 0])?;
    new_table.enable_node([1, 1])?;
    new_table.enable_node([0, 2])?;
    let node_bundles = new_table.get_dimension_at_indices(
        1,
        NodeFilter {
            get: hashset![0],
            without: hashset![2],
            ..Default::default()
        },
    )?;
    println!("{:?}", node_bundles);

    Ok(())
}
