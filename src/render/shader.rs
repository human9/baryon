extern crate gl;

use self::gl::types::GLint;

pub struct Shader {
    pub name: &'static str,
    pub attributes: Attributes,
    pub uniforms: Uniforms,
}

pub struct Attributes {
    pub vertex_attribute: GLint,
    pub uv_attribute: GLint, 
    pub normals_attribute: GLint,
}

pub struct Uniforms {
    pub model_uniform: GLint,
    pub view_uniform: GLint,
    pub mvp_uniform: GLint,
    pub sampler_uniform: GLint,
}


