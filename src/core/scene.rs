use std::collections::LinkedList;
use core::object::Object;
use std::fmt;
use std::rc::Rc;
use render::shader::Shader;
use game::camera::Camera;

pub struct Scene {
    pub name: &'static str,
    pub objects: LinkedList<(Object, Rc<Shader>)>,
    pub camera: Camera,
}

impl fmt::Debug for Scene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<< !! SCENE !! >>")
    }
}
