use std::collections::LinkedList;
use core::object::Object;
use std::fmt;

pub struct Scene {
    pub name: &'static str,
    pub objects: LinkedList<Object>,
    // camera
}

impl fmt::Debug for Scene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<< !! SCENE !! >>")
    }
}
