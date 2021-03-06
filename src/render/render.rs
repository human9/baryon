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

#[inline]
fn b_look_at<T>(
    eye: Vector3<T>,
    center: Vector3<T>,
    up: Vector3<T>
) -> Matrix4<T>
where
    T : BaseFloat + GenFloat<T>
{
    let zero = num::zero::<T>();
    let one = num::one::<T>();
    let f = normalize(center - eye);
    //let up_n = normalize(up);
    let s = normalize(cross(f, up));
    let u = cross(s, f);
    Matrix4::new(
        Vector4::new(s.x, u.x,-f.x, zero),
        Vector4::new(s.y, u.y,-f.y, zero),
        Vector4::new(s.z, u.z,-f.z, zero),
        Vector4::new(-dot(s, eye), -dot(u, eye), dot(f, eye), one)
    )
}

impl system::System for Rendering {
    fn init() -> Self {

        unsafe{
            gl::ClearColor(0.2, 0.0, 0.2, 0.0);
            gl::PolygonMode( gl::FRONT_AND_BACK, gl::LINE );
        }

        Rendering { name: "Rendering", status: system::Status::Okay, scene: None, resolution: (800, 600)}

    }

    fn run(&mut self, bus: &mut Bus) -> &system::Status {

        unsafe { 
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };

        if let Some(ref scene) = self.scene { unsafe {

            let model: Mat4 = num::one();
            //println!("{:.?}", model);
            translate(&model, vec3(0.0, 0.0, -4.0));
            
            let eye = scene.camera.get_eye();
            let center = scene.camera.get_center();
            //println!("{:.?}", center);
            let up = vec3(0.0, 1.0, 0.0);
            let view = b_look_at(eye, center, up);

            let projection = perspective::<f32>(45.0, self.resolution.0 as f32/self.resolution.1 as f32, 0.1, 1000.0);

            let mvp = projection * view * model;

            for o in scene.objects.iter() {
                let ref s = o.shader;

                //gl::UseProgram(s.program);

                gl::UniformMatrix4fv(s.uniforms.mvp_uniform, 1, gl::FALSE, mem::transmute(&mvp));
                //gl::UniformMatrix4fv(s.uniforms.model_uniform, 1, gl::FALSE, mem::transmute(&model));
                //gl::UniformMatrix4fv(s.uniforms.view_uniform, 1, gl::FALSE, mem::transmute(&view));

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
