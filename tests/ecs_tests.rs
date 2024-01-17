use custom_ecs::*;
use hashbrown::HashSet;

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

    #[derive(Debug, Default)]
    pub struct Damage {
        pub max: f32,
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

#[test]
fn hash_test() {
    println!("{}", player::Health::hash());
    println!("{}", enemy::Health::hash());
}

#[test]
fn macro_test() {
    //use player::Health;
    let hash: HashSet<usize> = component_set!(player::Health, enemy::Health);
    println!("{:?}", hash);

    component_identifier!(player::Health);
    component_identifier!(enemy::Health);

    let filter = component_filter!((player::Health, enemy::Health));
    println!("{:?}", filter);
}
