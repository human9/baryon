extern crate gl;
extern crate tobj;

use std::collections::HashSet;
use std::mem;
use self::gl::types::*;
use render::shader::Shader;
use std::rc::Rc;

pub struct Object {
    pub element_vbo: GLuint,
    pub index_vbo: GLuint,
    pub element_array: Vec<GLfloat>,
    pub index_array: Vec<GLuint>,
    pub shader: Rc<Shader>,
}

pub fn tobj_to_object(model: &tobj::Model, shader: Rc<Shader>) -> Object {

    let ref mesh = model.mesh;
    println!("vert: {}", mesh.positions.len());
    println!("norm: {}", mesh.normals.len());
    println!("text: {}", mesh.texcoords.len());
    println!("indc: {}", mesh.indices.len());

    /*
     * OpenGL only supports a single index. So we make the indices
     * into tuples and then use a hashset to identify duplicates.
     */
    let mut vec = Vec::new();
    for i in 0..mesh.indices.len() / 3 {
        let index = i * 3;
        let tuple = (mesh.indices.get(index),
                     mesh.indices.get(index + 1),
                     mesh.indices.get(index + 2));
        vec.push(tuple);
    }
    let mut set = HashSet::new();
    let mut i = 0;
    let mut elements = Vec::new(); // temp for retaining index
    let mut element_array = Vec::new(); // actual element array
    let mut index_array = Vec::new(); // actual index array
    for index in vec.iter() {
        if !set.contains(index) {
            set.insert(index);
            
            elements.push(index); // push the unique element indeces
            index_array.push(i as u32); // push the index of this element
            i = i+1;

            let v = index.0.unwrap();
            element_array.push(*mesh.positions.get(*v as usize).unwrap());
            element_array.push(*mesh.positions.get(*v as usize + 1).unwrap());
            element_array.push(*mesh.positions.get(*v as usize + 2).unwrap());

            let t = index.1.unwrap();
            element_array.push(*mesh.texcoords.get(*t as usize).unwrap());
            element_array.push(*mesh.texcoords.get(*t as usize + 1).unwrap());

            let n = index.2.unwrap();
            element_array.push(*mesh.normals.get(*n as usize).unwrap());
            element_array.push(*mesh.normals.get(*n as usize + 1).unwrap());
            element_array.push(*mesh.normals.get(*n as usize + 2).unwrap());
        }
        else {
            let pos = elements.iter().position(|&a| a == index).unwrap();
            index_array.push(pos as u32); // push index of non-unique element
        }
    }

    let mut element_vbo = 0;
    let mut index_vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut element_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, element_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (element_array.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&element_array[0]),
            gl::STATIC_DRAW
        );

        gl::GenBuffers(1, &mut index_vbo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_vbo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (index_array.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
            mem::transmute(&index_array[0]),
            gl::STATIC_DRAW
        );
    }

    
    Object { 
        element_vbo: element_vbo,
        index_vbo: index_vbo,
        element_array: element_array,
        index_array: index_array,
        shader: shader.clone(),
    }

}
