use std::fmt::Debug;

#[derive(Clone, Copy)]
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
