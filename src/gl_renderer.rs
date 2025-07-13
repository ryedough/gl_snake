use std::{ffi::c_uint, fs, mem, time};

use glow::{HasContext, NativeBuffer, NativeVertexArray, COLOR_BUFFER_BIT, STATIC_DRAW};

pub struct GlRenderer {
    gl: glow::Context,
    t_last_render: time::SystemTime,
    vao : Option<NativeVertexArray>,
    vbo : Option<NativeBuffer>,
    ebo : Option<NativeBuffer>,
    program: Option<glow::NativeProgram>,
}

impl GlRenderer {
    pub fn new(gl: glow::Context) -> Self {
        let mut renderer = Self {
            gl,
            vao : None, 
            vbo : None, 
            ebo : None, 
            t_last_render: time::SystemTime::now(),
            program: None,
        };
        renderer.init_shader();
        renderer.init_buffers();
        
        renderer
    }

    fn init_shader(&mut self) {
        let gl = &self.gl; // i dont feel like to write `self` every single time

        let (vs_src, fs_src) = {
            let vs = fs::read_to_string("./shader/.vs").expect("can't load vertex shader");
            let fs = fs::read_to_string("./shader/.fs").expect("can't load fragment shader");
            (vs, fs)
        };

        let (vs, fs) = unsafe {
            let vs = match gl.create_shader(glow::VERTEX_SHADER) {
                Ok(vs) => vs,
                Err(err) => panic!("{err}"),
            };
            gl.shader_source(vs, &vs_src);
            gl.compile_shader(vs);
            if !gl.get_shader_compile_status(vs) {
                panic!("{}", gl.get_shader_info_log(vs));
            };

            let fs = match gl.create_shader(glow::FRAGMENT_SHADER) {
                Ok(fs) => fs,
                Err(err) => panic!("{err}"),
            };
            gl.shader_source(fs, &fs_src);
            gl.compile_shader(fs);
            if !gl.get_shader_compile_status(fs) {
                panic!("{}", gl.get_shader_info_log(fs));
            };
            (vs, fs)
        };

        let program = unsafe {
            let program = match gl.create_program() {
                Ok(p) => p,
                Err(err) => {
                    panic!("{err}")
                }
            };
            gl.attach_shader(program, vs);
            gl.attach_shader(program, fs);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_link_status(program));
            }

            gl.delete_shader(fs);
            gl.delete_shader(vs);

            program
        };

        self.program = Some(program);
    }

    pub fn init_buffers(&mut self) {
        let gl = &self.gl; // i dont feel like to write `self` every single time
        let (vbo, vao) = unsafe {
            // This is a flat array of f32s that are to be interpreted as vec2s.
            let triangle_vertices = [0.5f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32];
            let triangle_vertices_u8: &[u8] = core::slice::from_raw_parts(
                triangle_vertices.as_ptr() as *const u8,
                triangle_vertices.len() * core::mem::size_of::<f32>(),
            );

            // We construct a buffer and upload the data
            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, triangle_vertices_u8, glow::STATIC_DRAW);

            // We now construct a vertex array to describe the format of the input buffer
            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);

            (vbo, vao)
        };
        self.vao = Some(vao);
        self.vbo = Some(vbo);
    }

    pub fn render(&mut self) {
        let delta = time::SystemTime::now()
            .duration_since(self.t_last_render)
            .unwrap();
        self.t_last_render = time::SystemTime::now();

        // println!(
        //     "{} fps",
        //     time::Duration::from_secs(1).div_duration_f32(delta) as u32
        // );

        unsafe {
            self.gl.clear_color(0., 0.5, 0.5, 1.);
            self.gl.clear(COLOR_BUFFER_BIT);
        }

        unsafe {
            self.gl.bind_vertex_array(self.vao);
            self.gl.use_program(self.program);
            self.gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }
}