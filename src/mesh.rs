use core::time;
use std::{mem, rc::Rc};

use glow::{HasContext, NativeBuffer, NativeVertexArray};

use crate::{gl_app::Updateable, shader::Shader};

pub struct Mesh<T : Shader> {
    shader : Rc<T>,
    vao : NativeVertexArray,
    vbo : NativeBuffer, 
    ebo : NativeBuffer,
}

impl<T:Shader> Mesh<T> {
    pub fn new(gl : &glow::Context, shader : Rc<T>) -> Self {
        let (vbo, vao, ebo) = unsafe {
            // mesh
            let vert = [
                0.5f32,  0.5, 0.0,  // top right
                1.,  0., 0.0,  // color
                0.5, -0.5, 0.0,  // bottom right
                0.,  1., 0.0,  // color
                -0.5, -0.5, 0.0,  // bottom left
                0.,  0., 1.,  // color
                -0.5,  0.5, 0.0,   // top left 
                1.,  1., 1.,  // color
            ];
            let vert: &[u8] = core::slice::from_raw_parts(
                vert.as_ptr() as *const u8,
                vert.len() * core::mem::size_of::<f32>(),
            );
            let indices = [  // note that we start from 0!
                0u32, 1, 3,   // first triangle
                1, 2, 3    // second triangle
            ];
            let indices = core::slice::from_raw_parts(
                indices.as_ptr() as *const u8,
                indices.len() * mem::size_of::<u32>()
            );

            // vertex arrays
            let vao = gl.create_vertex_array().unwrap();
            let vbo = gl.create_buffer().unwrap();
            let ebo = gl.create_buffer().unwrap();
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vert, glow::STATIC_DRAW);
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, indices, glow::STATIC_DRAW);

            gl.enable_vertex_attrib_array(0);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 6 * mem::size_of::<f32>() as i32, 0);
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 6 * mem::size_of::<f32>() as i32, 3 * mem::size_of::<f32>() as i32);

            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
            (vbo, vao, ebo) 
        };

        Self { shader, vao, vbo, ebo }
    }
}

impl<T:Shader> Updateable for Mesh<T> {
    fn on_tick(&mut self, gl : &glow::Context, _ : &time::Duration, _ : &time::Duration) {
        self.shader.use_shader(gl);

        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
        }
    }
}