use custom_ecs::table::{*, self};

#[test]
pub fn table_test() -> Result<(),TableError>{
    let mut new_table = table::Table::new();

    new_table.enable_node([0,0])?;
    new_table.enable_node([1,0])?;
    let node_id = new_table.enable_node([1,1])?;
    new_table.enable_node([0,2])?;
    //println!("{:?}", new_table);

    let old_node_id = new_table.disable_node(&node_id)?;
    println!("{:?}", new_table);
    assert_eq!(node_id, old_node_id);

    Ok(())
}