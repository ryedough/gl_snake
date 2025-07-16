use core::time;
use std::{mem, rc::Rc};

use glow::{HasContext, NativeBuffer, NativeVertexArray};

use crate::shaders::Shader;

pub struct UnitRect<> {
    vao : NativeVertexArray,
    vbo : NativeBuffer, 
    ebo : NativeBuffer,
}

impl UnitRect {
    pub fn new(gl : &glow::Context, shader : &impl Shader) -> Self {
        let (vbo, vao, ebo) = unsafe {
            // mesh
            let vert = [
                -1.,  1., 0.0,   // top left 
                -1., -1., 0.0,  // bottom left
                1., -1., 0.0,  // bottom right
                1.0f32,  1., 0.0,  // top right
            ];
            let vert: &[u8] = core::slice::from_raw_parts(
                vert.as_ptr() as *const u8,
                vert.len() * core::mem::size_of::<f32>(),
            );
            let indices = [
                0u32, 1, 3,
                1, 2, 3 
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

            let pos_attr = shader.get_attribute("aPos").unwrap();
            gl.enable_vertex_attrib_array(pos_attr);
            gl.vertex_attrib_pointer_f32(pos_attr, 3, glow::FLOAT, false, 3 * mem::size_of::<f32>() as i32, 0);

            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
            (vbo, vao, ebo) 
        };

        Self { vao, vbo, ebo }
    }

    pub fn render(&mut self, gl : &glow::Context, _ : &time::Duration, _ : &time::Duration) {
        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
        }
    }
}