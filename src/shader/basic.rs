use std::{collections::HashMap, fs};

use glow::{HasContext, NativeProgram, NativeUniformLocation};

use crate::shader::{Shader, gen_program};
pub struct BasicShader {
    program: NativeProgram,
    u_circ_pos: NativeUniformLocation,
    u_circ_radius: NativeUniformLocation,
    attributes: HashMap<String, u32>,
}

impl BasicShader {
    pub fn new(gl: &glow::Context) -> Self {
        let attributes = [
                ("aPos".to_string(), 0)
            ];
        let (vs, fs) = {
            let vs = fs::read_to_string("./shader/.vs").expect("can't load vertex shader");
            let fs = fs::read_to_string("./shader/.fs").expect("can't load fragment shader");
            (vs, fs)
        };
        let program = gen_program(gl, &vs, &fs).unwrap();

        let (u_circ_pos, u_circ_radius) = unsafe { (
            gl.get_uniform_location(program, "uCircPos").unwrap(),
            gl.get_uniform_location(program, "uCircRadius").unwrap(),
        ) };

        Self {
            program,
            u_circ_pos,
            u_circ_radius,
            attributes: HashMap::from(attributes),
        }
    }

    pub fn set_circle_pos(&self, gl: &glow::Context, x: f32, y :f32) {
        unsafe {
            gl.uniform_2_f32(Some(&self.u_circ_pos), x, y);
        }
    }
    pub fn set_circle_radius(&self, gl: &glow::Context, x: f32) {
        unsafe {
            gl.uniform_1_f32(Some(&self.u_circ_radius), x);
        }
    }
}

impl Shader for BasicShader {
    fn get_attribute(&self, key: &str) -> Option<u32> {
        self.attributes.get(key).copied()
    }
    fn use_shader(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }
}
