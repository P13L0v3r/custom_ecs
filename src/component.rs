use std::any::Any;

pub use ecs_proc_macros::Component;

pub trait Component {
    fn hash() -> usize
    where
        Self: Sized;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct ComponentBundle {}
