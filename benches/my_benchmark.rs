use criterion::{criterion_group, criterion_main, Criterion};
use custom_ecs::table::*;

pub fn table_test() -> Result<(), TableError> {
    let mut new_table = Table::new();

    new_table.enable_node([0, 0])?;
    new_table.enable_node([1, 0])?;
    let node_id = new_table.enable_node([1, 1])?;
    new_table.enable_node([0, 2])?;
    new_table.disable_node(&node_id)?;

    Ok(())
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("table_test", |b| b.iter(table_test));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
