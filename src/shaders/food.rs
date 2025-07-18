use std::{collections::HashMap, fs};

use glow::{HasContext, NativeProgram, NativeUniformLocation};

use crate::shaders::{gen_program, Shader};

pub struct FoodShader {
    program: NativeProgram,
    u_radius: NativeUniformLocation,
    u_position: NativeUniformLocation,
    u_time: NativeUniformLocation,
    attributes: HashMap<String, u32>,
}

impl FoodShader {
    pub fn new(gl: &glow::Context) -> Self {
        let attributes = [("aPos".to_string(), 0)];
        let (vs, fs) = {
            let vs = fs::read_to_string("./shader/.vs").expect("can't load vertex shader");
            let fs = fs::read_to_string("./shader/food.fs").expect("can't load fragment shader");
            (vs, fs)
        };
        let program = gen_program(gl, &vs, &fs).unwrap();

        let (u_radius, u_position, u_time) = unsafe {
            (
                gl.get_uniform_location(program, "uRadius").unwrap(),
                gl.get_uniform_location(program, "uPosition").unwrap(),
                gl.get_uniform_location(program, "uTime").unwrap(),
            )
        };

        Self {
            program,
            u_radius,
            u_position,
            u_time,
            attributes: HashMap::from(attributes),
        }
    }
    pub fn set_position(&self, gl: &glow::Context, x: f32, y: f32) {
        unsafe {
            gl.uniform_2_f32(Some(&self.u_position), x, y);
        }
    }
    pub fn set_time(&self, gl: &glow::Context, x: f32) {
        unsafe {
            gl.uniform_1_f32(Some(&self.u_time), x);
        }
    }
    pub fn set_radius(&self, gl: &glow::Context, x: f32) {
        unsafe {
            gl.uniform_1_f32(Some(&self.u_radius), x);
        }
    }
}

impl Shader for FoodShader {
    fn get_attribute(&self, key: &str) -> Option<u32> {
        self.attributes.get(key).copied()
    }
    fn use_shader(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }
}