extern crate gl;
extern crate tobj;

use std::collections::HashSet;
use std::collections::HashMap;
use std::mem;
use self::gl::types::*;
use render::shader::Shader;
use std::rc::Rc;
use std::ptr;

pub struct Object {
    pub vao: GLuint,
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


    let has_normals = mesh.normals.len() > 0;
    let has_texcoords = mesh.texcoords.len() > 0;

    /*
     * OpenGL only supports a single index. So we make the indices
     * into tuples and then use a hashset to identify duplicates.
     */
    
    let mut div = 1; // we have all things
    if has_normals { div += 1 };
    if has_texcoords { div += 1 };

    let mut raw_indices = Vec::new();
    for i in 0..mesh.indices.len()/div {
        let index = i * div;
        let p = *mesh.indices.get(index).unwrap();
        let mut t = 0;
        let mut n = 0;
        if has_normals && has_texcoords {
            t = *mesh.indices.get(index + 1).unwrap();
            n = *mesh.indices.get(index + 2).unwrap();
        } else {
            if has_normals {
                n = *mesh.indices.get(index + 1).unwrap();
            }
            if has_texcoords {
                t = *mesh.indices.get(index + 1).unwrap();
            }
        }
        raw_indices.push( (p, t, n) );
    }
    let mut map = HashMap::new();
    let mut i: u32 = 0;
    let mut elements = Vec::new(); // temp for retaining index
    let mut element_array = Vec::new(); // actual element array
    let mut index_array = Vec::new(); // actual index array
    for index in raw_indices.iter() {
        if !map.contains_key(index) {
            map.insert(index, i);
            
            elements.push(index); // push the unique element indeces
            index_array.push(i as u32); // push the index of this element
            i = i+1;

            let v = (index.0 * 3) as usize;
            element_array.push(*mesh.positions.get(v).unwrap());
            element_array.push(*mesh.positions.get(v + 1).unwrap());
            element_array.push(*mesh.positions.get(v + 2).unwrap());

            let t = (index.1 * 2) as usize;
            element_array.push(*mesh.texcoords.get(t).unwrap_or(&0.));
            element_array.push(*mesh.texcoords.get(t + 1).unwrap_or(&0.));

            let n = (index.2 * 3) as usize;
            element_array.push(*mesh.normals.get(n).unwrap_or(&0.));
            element_array.push(*mesh.normals.get(n + 1).unwrap_or(&0.));
            element_array.push(*mesh.normals.get(n + 2).unwrap_or(&0.));
        }
        else {
            index_array.push(*map.get(index).unwrap() as u32); // push index of non-unique element
        }
    }
    //println!("{:.?}", element_array);

    let mut element_vbo = 0;
    let mut index_vbo = 0;
    let mut vao = 0;

    unsafe {

        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

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

        gl::UseProgram(shader.program);

        gl::Enable(gl::BLEND);
        gl::Enable(gl::DEPTH_TEST);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        gl::BindBuffer(gl::ARRAY_BUFFER, element_vbo);
        
        gl::EnableVertexAttribArray(shader.attributes.vertex_attribute as GLuint);
        gl::EnableVertexAttribArray(shader.attributes.uv_attribute as GLuint);
        gl::EnableVertexAttribArray(shader.attributes.normals_attribute as GLuint);

        gl::VertexAttribPointer(
            shader.attributes.vertex_attribute as GLuint,
            3,
            gl::FLOAT,
            gl::FALSE,
            (mem::size_of::<GLfloat>() * 8) as GLsizei,
            ptr::null()
        );
        gl::VertexAttribPointer(
            shader.attributes.uv_attribute as GLuint,
            2,
            gl::FLOAT,
            gl::FALSE,
            (mem::size_of::<GLfloat>() * 8) as GLsizei,
            (mem::size_of::<GLfloat>() * 3) as *const GLvoid,
        );
        gl::VertexAttribPointer(
            shader.attributes.normals_attribute as GLuint,
            3,
            gl::FLOAT,
            gl::FALSE,
            (mem::size_of::<GLfloat>() * 8) as GLsizei,
            (mem::size_of::<GLfloat>() * 5) as *const GLvoid,
        );

        
    }

    
    Object { 
        vao: vao,
        element_vbo: element_vbo,
        index_vbo: index_vbo,
        element_array: element_array,
        index_array: index_array,
        shader: shader.clone(),
    }

}
