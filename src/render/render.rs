extern crate gl;

use core::system;
use core::message::Message;
use core::bus::Bus;
use core::scene::Scene;

use self::gl::types::*;
use std::mem;
use std::ptr;
use std::rc::Rc;
use std::option::Option;

pub struct Rendering {
    name: &'static str,
    status: system::Status,
    scene: Option<Rc<Scene>>,
}

impl Rendering {
    fn load_scene(&mut self, s: &Rc<Scene>) {
        self.scene = Some(s.clone());
    }

    fn shutdown(&mut self) {
        self.status = system::Status::Finished;
    }
}

impl system::System for Rendering {
    fn init() -> Self {
        Rendering { name: "Rendering", status: system::Status::Okay, scene: None }
    }

    fn run(&mut self, bus: &mut Bus) -> &system::Status {

        unsafe { 
            
            gl::Clear(gl::COLOR_BUFFER_BIT);
        
        };

        if let Some(ref scene) = self.scene { unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            for object in scene.objects.iter() {
                let ref o = object.0;
                let ref s = object.1;

                gl::UseProgram(s.program);
                
                gl::BindBuffer(gl::ARRAY_BUFFER, o.element_vbo);
                
                gl::EnableVertexAttribArray(s.attributes.vertex_attribute as GLuint);
                gl::VertexAttribPointer(
                    s.attributes.vertex_attribute as GLuint,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    (mem::size_of::<GLfloat>() * 8) as GLsizei,
                    ptr::null()
                );

                gl::EnableVertexAttribArray(s.attributes.uv_attribute as GLuint);
                gl::VertexAttribPointer(
                    s.attributes.uv_attribute as GLuint,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    (mem::size_of::<GLfloat>() * 8) as GLsizei,
                    (mem::size_of::<GLfloat>() * 3) as *const GLvoid,
                );
                
                gl::EnableVertexAttribArray(s.attributes.normals_attribute as GLuint);
                gl::VertexAttribPointer(
                    s.attributes.normals_attribute as GLuint,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    (mem::size_of::<GLfloat>() * 8) as GLsizei,
                    (mem::size_of::<GLfloat>() * 5) as *const GLvoid,
                );

                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, o.index_vbo);
                gl::DrawElements(
                    gl::TRIANGLES,
                    o.index_array.len() as GLint,
                    gl::UNSIGNED_INT,
                    ptr::null()
                );
            }
        }}

        &self.status
    }
    
    fn handle(&mut self, msg: &Message) {
        match msg {
            &Message::Shutdown => self.shutdown(),
            &Message::LoadScene(ref scene) => self.load_scene(scene),
            //_ => (),
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }
}
