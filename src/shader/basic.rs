use std::fs;

use glow::{HasContext, NativeProgram, NativeUniformLocation};

use crate::shader::{gen_program, Shader};
pub struct BasicShader {
    program : NativeProgram,
    u_time_location : NativeUniformLocation
}

impl BasicShader {
    pub fn new(gl : &glow::Context) -> Self {
        let (vs, fs) = {
            let vs = fs::read_to_string("./shader/.vs").expect("can't load vertex shader");
            let fs = fs::read_to_string("./shader/.fs").expect("can't load fragment shader");
            (vs, fs)
        };

        let program = gen_program(gl, &vs, &fs).unwrap();

        let u_time_location = unsafe{ gl.get_uniform_location(program, "uTime").unwrap() };

        Self {
            program,
            u_time_location,
        }
    }

    pub fn set_time(&self, gl : &glow::Context, time : f32) {
        unsafe{
            gl.uniform_1_f32(Some(&self.u_time_location), time);
        }
    }
}

impl Shader for BasicShader {
    fn use_shader(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }
}