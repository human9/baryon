extern crate gl;

use self::gl::types::{GLuint, GLfloat};

#[derive(Debug)]
pub struct Object {
    pub element_array: Vec<GLfloat>,
    pub index_array: Vec<GLuint>,
}
