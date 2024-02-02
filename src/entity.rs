use std::fmt::Debug;

use crate::*;
use hashbrown::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub(crate) usize);

impl Entity {
    pub fn new(id: usize) -> Entity {
        Entity(id)
    }
}

impl Debug for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Entity({:#x})", &self.0))
    }
}

#[derive(Debug, Component, Clone)]
pub struct Children {
    pub(crate) children: HashSet<Entity>
}

#[derive(Debug, Component, Clone)]
pub struct Parent {
    pub(crate) parent: Entity
}