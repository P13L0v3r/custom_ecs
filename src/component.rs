use std::{any::Any, marker::PhantomData};

pub use ecs_proc_macros::Component;
use hashbrown::HashMap;

use crate::{World, table::NodeBundle};

pub trait Component {
    fn hash() -> usize
    where
        Self: Sized;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait ComponentUnPacker {
    type ComponentType: Component + 'static;
    
    fn unpack_from_node_bundle<'a>(world: &'a World, node_bundle: &'a NodeBundle) -> Option<&'a Self::ComponentType> {
        world.unpack::<Self::ComponentType>(node_bundle)
    }

    fn unpack_from_node_bundle_mut<'a>(world: &'a mut World, node_bundle: &'a NodeBundle) -> Option<&'a mut Self::ComponentType> {
        world.unpack_mut::<Self::ComponentType>(node_bundle)
    }
}

pub struct ComponentId<T> where T : Component + 'static {
    buf: T
}