use std::rc::Rc;
use core::scene::Scene;

#[derive(Debug)]
pub enum Message {
    Shutdown,
    LoadScene(Rc<Scene>),
    Resized(u32, u32),
    RotateCamera(i32, i32),
    MoveCamera((i32, i32)),
}
