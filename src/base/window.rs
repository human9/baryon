extern crate gl;
extern crate glutin;
extern crate libc;

use core::system;
use core::message;

pub struct Windowing {
    name: &'static str,
    status: system::Status,
}

impl system::System for Windowing {
    fn init() -> Self {
        Windowing { name: "Windowing", status: system::Status::Okay }
    }

    fn run(&self) -> system::Status {
        self.status
    }

    fn name(&self) -> &'static str {
        self.name
    }
}


    /*
    let window = glutin::WindowBuilder::new()
        .with_title("baryon")
        .build()
        .unwrap();

    unsafe { window.make_current() };
    
    unsafe {
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
    }

    for event in window.poll_events() {
        //unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

        match event {
            glutin::Event::Closed => break,
            _ => ()
        }
    }
    window.swap_buffers();
*/

