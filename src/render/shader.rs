extern crate gl;
extern crate glutin;

use self::gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Shader {
    pub program: GLuint,
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

unsafe fn glsl_init(frag: &str, vert: &str) -> Shader {

    let program = link_program(
        compile_shader(&file_to_string(vert), gl::VERTEX_SHADER),
        compile_shader(&file_to_string(frag), gl::FRAGMENT_SHADER)
    );

    let attributes = Attributes {
        vertex_attribute: gl::GetAttribLocation(program, CString::new("coord3d").unwrap().as_ptr()),
        uv_attribute: gl::GetAttribLocation(program, CString::new("uv").unwrap().as_ptr()),
        normals_attribute: gl::GetAttribLocation(program, CString::new("normals").unwrap().as_ptr()),
    };

    let uniforms = Uniforms {
        model_uniform: gl::GetUniformLocation(program, CString::new("model").unwrap().as_ptr()),
        view_uniform: gl::GetUniformLocation(program, CString::new("view").unwrap().as_ptr()),
        mvp_uniform: gl::GetUniformLocation(program, CString::new("mvp").unwrap().as_ptr()),
        sampler_uniform: gl::GetUniformLocation(program, CString::new("sampler").unwrap().as_ptr()),
    };

    Shader {
        program: program,
        attributes: attributes,
        uniforms: uniforms,
    }
}

fn file_to_string(string: &str) -> String
{
    let path = Path::new(string);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => (),
    }
    s
}

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint { unsafe {
    let program = gl::CreateProgram();
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::LinkProgram(program);
    // Get the link status
    let mut status = gl::FALSE as GLint;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

    // Fail on error
    if status != (gl::TRUE as GLint) {
        let mut len: GLint = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf = Vec::with_capacity(len as usize);
        buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
        gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
        panic!("{}", str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8"));
    }
    program
} }

