use custom_ecs::{*, table::NodeFilter};
use ecs_proc_macros::{name_to_type, evaluate_string_var};
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
    for _ in 0..10 {
        let entity = new_world.alloc_entity();
        new_world.enable_component_for_entity(
            entity,
            player::Health {
                max: 1.0,
                current: 1.0,
            },
        );
    }

    for bundle in new_world
        .component_node_bundles(Some(component_set!(player::Health)), None, None)
        .iter()
    {
        println!("{:?}", new_world.unpack_mut::<player::Health>(bundle));
    }

    let id: usize = 0;
}

fn pass_type<T>() -> T 
where T : Default 
{
    T::default()
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

    let filter = component_filter!((player::Health, enemy::Health));
    println!("{:?}", filter);

    let t: name_to_type!("player::Health") = player::Health::default();

    let n = "player::Health";

    //type_test::<(player::Health, enemy::Health),(),()>()
}

/* fn type_test<G,W,V>() {    
    let get = component_set!(G);
    let with = component_set!(W);
    let without = component_set!(V);
    
    let filter = NodeFilter { get, with, without };
    println!("{:?}", filter);
} */