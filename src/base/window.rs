extern crate gl;
extern crate glutin;

use core::system;
use core::message::Message;
use core::bus::Bus;
use self::glutin::{Event, ElementState, VirtualKeyCode};

const W: i32 = 800;
const H: i32 = 600;

pub struct Windowing {
    name: &'static str,
    status: system::Status,
    window: glutin::Window,
    m_pos: (i32, i32),
    direction: (i32, i32, i32, i32),
}

fn shutdown(status: &mut system::Status, bus: &mut Bus) {
    *status = system::Status::Finished;
    bus.post(Message::Shutdown);
}

impl system::System for Windowing {
    fn init() -> Self {

        let window = glutin::WindowBuilder::new()
            .with_title("baryon")
            .with_dimensions(W as u32, H as u32)
            .build()
            .unwrap();

        unsafe { 
            
            let _ = window.make_current();

            window.set_cursor_state(glutin::CursorState::Grab);
            window.set_cursor(glutin::MouseCursor::NoneCursor);
            window.set_cursor_position(W/2, H/2);

            gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

            gl::ClearColor(0.2, 0.0, 0.2, 0.0);
        
        };

        Windowing { 
            name: "Windowing",
            status: system::Status::Okay,
            window: window, 
            m_pos: (W/2,H/2),
            direction: (0, 0, 0, 0),
        }
    }

    fn run(&mut self, bus: &mut Bus) -> &system::Status {

        if self.status == system::Status::Okay {
            //println!("{:.?}",self.m_pos);

            for event in self.window.poll_events() {

                match event {
                    Event::Closed => shutdown(&mut self.status, bus),
                    Event::KeyboardInput(ElementState::Pressed, _, key) => 
                    {
                        match key.unwrap() {
                            VirtualKeyCode::Escape => shutdown(&mut self.status, bus),
                            VirtualKeyCode::W => self.direction.0 = 1,
                            VirtualKeyCode::A => self.direction.1 = 1,
                            VirtualKeyCode::S => self.direction.2 = 1,
                            VirtualKeyCode::D => self.direction.3 = 1,
                            _ => (),
                        }
                    },

                    Event::KeyboardInput(ElementState::Released, _, key) => 
                    {
                        match key.unwrap() {
                            VirtualKeyCode::W => self.direction.0 = 0,
                            VirtualKeyCode::A => self.direction.1 = 0,
                            VirtualKeyCode::S => self.direction.2 = 0,
                            VirtualKeyCode::D => self.direction.3 = 0,
                            _ => (),
                        }
                    },

                    a @ Event::MouseInput(ElementState::Pressed, _) => {
                        println!("{:?}", a);
                    },

                    Event::MouseMoved(x, y) => {
                        if (x, y) != self.m_pos {
                            bus.post(Message::RotateCamera(x - self.m_pos.0, y - self.m_pos.1));

                            let size = self.window.get_outer_size().unwrap();
                            self.m_pos = (size.0 as i32 / 2, size.1 as i32 / 2);
                            self.window.set_cursor_position(self.m_pos.0, self.m_pos.1);
                        }
                    },

                    a @ Event::MouseWheel(_, _) => {
                        println!("{:?}", a);
                    },

                    Event::Resized(w, h) => {
                        bus.post(Message::Resized(w, h));
                        self.m_pos = (w as i32 / 2, h as i32 / 2);
                        self.window.set_cursor_position(self.m_pos.0, self.m_pos.1);
                    },
                    
                    _ => (),

                }

            }
            self.window.swap_buffers().unwrap();
                
            if self.direction != (0, 0, 0, 0) {
                let dir = (self.direction.3 - self.direction.1, self.direction.2 - self.direction.0);
                if dir != (0, 0) {
                    bus.post(Message::MoveCamera(dir));
                }
            }

        }
        
        &self.status
    }

    fn handle(&mut self, msg: &Message) {
        match msg {
            &Message::Shutdown => self.status = system::Status::Finished,
            _ => (),
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }
}
