use ecs_proc_macros::Component;

//use ecs_proc_macros::Component;

#[derive(Component)]
struct HealthBar;

//component!(HealthBar);

#[test]
fn initial_build() {
    println!("Hello World")
}
