use custom_ecs::*;

mod player {
    use custom_ecs::*;

    #[derive(Debug, Component, Default)]
    pub struct Health {
        pub max: f32,
        pub current: f32,
    }
}

mod enemy {
    use custom_ecs::*;

    #[derive(Debug, Component, Default)]
    pub struct Health {
        pub max: f32,
        pub current: f32,
    }
}

#[test]
fn ecs_test() {
    let mut new_world = World::new();
    let entity = new_world.alloc_entity();
    new_world.enable_component_for_entity(
        entity,
        player::Health {
            max: 1.0,
            current: 1.0,
        },
    );
    let entity_health = new_world.query_entity_component::<player::Health>(entity);
    println!("{:?}", entity_health);

    if let Some(health) = new_world.query_entity_component_mut::<player::Health>(entity) {
        health.current -= 0.1;
    }

    let entity_health = new_world.query_entity_component::<player::Health>(entity);
    println!("{:?}", entity_health);

    new_world.disable_component_for_entity::<player::Health>(entity);

    let entity_health = new_world.query_entity_component::<player::Health>(entity);
    println!("{:?}", entity_health);
}
