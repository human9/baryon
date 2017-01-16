extern crate gl;
extern crate glutin;
extern crate libc;

use core::system;

use std::time::Duration;
use std::thread;

pub struct Windowing {
    name: &'static str,
    status: system::Status,
    window: glutin::Window,
}

impl system::System for Windowing {
    fn init() -> Self {

        let window = glutin::WindowBuilder::new()
            .with_title("baryon")
            .build()
            .unwrap();

        unsafe { 
            
            let _ = window.make_current();

            gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        
        };

        Windowing { name: "Windowing", status: system::Status::Okay, window: window}
    }

    fn run(&mut self) -> system::Status {
        for event in self.window.poll_events() {
            //unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

            match event {
                glutin::Event::Closed => self.status = system::Status::Finished,
                _ => ()
            }
        }
        let _ = self.window.swap_buffers();

        thread::sleep(Duration::from_millis(10));

        self.status
    }

    fn name(&self) -> &'static str {
        self.name
    }
}
