use std::{mem, time::{self, Duration}};

use glow::{HasContext, NativeBuffer, NativeVertexArray, COLOR_BUFFER_BIT, STATIC_DRAW};

use crate::shader::BasicShader;

pub struct GlRenderer {
    gl: glow::Context,
    t_last_render: time::SystemTime,
    t_0 : time::SystemTime,
    vao : Option<NativeVertexArray>,
    vbo : Option<NativeBuffer>,
    ebo : Option<NativeBuffer>,
    basic_shader : BasicShader,
}

impl GlRenderer {
    pub fn new(gl: glow::Context) -> Self {
        let basic_shader = BasicShader::new(&gl);
        let mut renderer = Self {
            gl,
            vao : None, 
            vbo : None, 
            ebo : None, 
            basic_shader,
            t_last_render: time::SystemTime::now(),
            t_0: time::SystemTime::now(),
        };
        renderer.init_buffers();
        
        renderer
    }

    pub fn init_buffers(&mut self) {
        let gl = &self.gl; // i dont feel like to write `self` every single time
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
        self.vao = Some(vao);
        self.vbo = Some(vbo);
        self.ebo = Some(ebo);
    }

    pub fn render(&mut self) {
        self.calc_delta();

        unsafe {
            self.gl.clear_color(0., 0.5, 0.5, 1.);
            self.gl.clear(COLOR_BUFFER_BIT);
        }

        unsafe {
            self.basic_shader.use_shader(&self.gl);
            self.basic_shader.set_time(&self.gl, self.elapsed().as_secs_f32());
            self.gl.bind_vertex_array(self.vao);
            self.gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
        }
    }

    fn calc_delta(&mut self) -> time::Duration {
        let delta = time::SystemTime::now()
            .duration_since(self.t_last_render)
            .unwrap();
        self.t_last_render = time::SystemTime::now();
        delta
    }

    fn elapsed(&self) -> time::Duration {
        self.t_0.elapsed().unwrap()
    }
}