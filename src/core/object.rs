extern crate gl;

use self::gl::types::GLuint;
use self::gl::types::GLfloat;

pub struct Object {
    pub element_array: Vec<GLfloat>,
    pub index_array: Vec<GLuint>,
}
