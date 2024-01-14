use custom_ecs::table::{*, self};

#[test]
pub fn table_test() -> Result<(),TableError>{
    let mut new_table = Table::new();

    new_table.enable_node([0,0])?;
    new_table.enable_node([1,0])?;
    let node_id = new_table.enable_node([1,1])?;
    new_table.enable_node([0,2])?;
    //println!("{:?}", new_table);

    Ok(())
}