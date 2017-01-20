use std::collections::LinkedList;
use core::object::Object;

#[derive(Debug)]
pub struct Scene {
    pub objects: LinkedList<Object>,
}
