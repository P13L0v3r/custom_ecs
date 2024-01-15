pub use ecs_proc_macros::Component;

pub trait Component {
    fn hash() -> usize
    where
        Self: Sized;
}

pub struct ComponentBundle {}
