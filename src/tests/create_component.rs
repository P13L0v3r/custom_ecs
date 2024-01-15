use crate::component::{self, *};

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct Health;

#[test]
fn component_hash() {
    println!("{}", HealthBar::COMPONENT_ID);
    println!("{}", Health::COMPONENT_ID);
}