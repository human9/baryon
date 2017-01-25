extern crate gl;
extern crate glm;
extern crate num;

use self::glm::*;
use self::glm::ext::*;

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
    resolution: (u32, u32),
}

impl Rendering {
    fn load_scene(&mut self, s: &Rc<Scene>) {
        self.scene = Some(s.clone());
    }

    fn shutdown(&mut self) {
        self.status = system::Status::Finished;
    }

    fn rotate_camera(&mut self, x: i32, y: i32) {
        if let Some(ref mut scene) = self.scene {
            let s = Rc::get_mut(scene).unwrap();
            s.camera.add_azimuth(x as f32/40.0); 
            s.camera.add_elevation(-y as f32/40.0);
        }
    }

    fn move_camera(&mut self, movement: (i32, i32)) {
        if let Some(ref mut scene) = self.scene {
            let s = Rc::get_mut(scene).unwrap();
            s.camera.add_strafe(movement.0 as f32);
            s.camera.add_forward(movement.1 as f32);
        }
    }
}

impl system::System for Rendering {
    fn init() -> Self {
        Rendering { name: "Rendering", status: system::Status::Okay, scene: None, resolution: (800, 600)}
    }

    fn run(&mut self, bus: &mut Bus) -> &system::Status {

        unsafe { 
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        };

        if let Some(ref scene) = self.scene { unsafe {

            let model: Mat4 = num::one();
            //println!("{:.?}", model);
            translate(&model, vec3(0.0, 0.0, -4.0));
            
            let eye = scene.camera.get_eye();
            let center = scene.camera.get_center();
            //println!("{:.?}", model);
            let up = vec3(0.0, 1.0, 0.0);
            let view = look_at(eye, center, up);

            let projection = perspective::<f32>(45.0, self.resolution.0 as f32/self.resolution.1 as f32, 0.1, 1000.0);

            let mvp = projection * view * model;

            for o in scene.objects.iter() {
                let ref s = o.shader;

                gl::UseProgram(s.program);

                gl::PolygonMode( gl::FRONT_AND_BACK, gl::LINE );

                gl::UniformMatrix4fv(s.uniforms.mvp_uniform, 1, gl::FALSE, mem::transmute(&mvp));
                gl::UniformMatrix4fv(s.uniforms.model_uniform, 1, gl::FALSE, mem::transmute(&model));
                gl::UniformMatrix4fv(s.uniforms.view_uniform, 1, gl::FALSE, mem::transmute(&view));
                

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
            &Message::Resized(w, h) => {
                unsafe {gl::Viewport(0, 0, w as i32, h as i32)};
                self.resolution = (w, h);
            },
            &Message::RotateCamera(x, y) => {
                self.rotate_camera(x, y);
            },
            &Message::MoveCamera((x, y)) => {
                self.move_camera((x,y));
            },
            //_ => (),
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }
}
