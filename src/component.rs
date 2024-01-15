pub use ecs_proc_macros::Component;

pub trait Component {
    const COMPONENT_ID: usize;
}
