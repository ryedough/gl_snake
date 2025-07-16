use crate::{object::snek::DirKeypoint, shader::{gen_program, Shader}};
use glow::{HasContext, NativeProgram, NativeUniformLocation};
use std::{collections::HashMap, fs};

const MAX_KEYPOINTS: usize = 100;

struct UKeypoint {
    from: NativeUniformLocation,
    at: NativeUniformLocation,
    dst_head: NativeUniformLocation,
}

pub struct SnekShader {
    program: NativeProgram,
    u_circ_radius: NativeUniformLocation,
    u_keypoints: [UKeypoint; MAX_KEYPOINTS],
    u_keypoint_len: NativeUniformLocation,
    u_length: NativeUniformLocation,
    attributes: HashMap<String, u32>,
}

impl SnekShader {
    pub fn new(gl: &glow::Context) -> Self {
        let attributes = [("aPos".to_string(), 0)];
        let (vs, fs) = {
            let vs = fs::read_to_string("./shader/.vs").expect("can't load vertex shader");
            let fs = fs::read_to_string("./shader/snek.fs").expect("can't load fragment shader");
            (vs, fs)
        };
        let program = gen_program(gl, &vs, &fs).unwrap();

        let (u_circ_radius, u_keypoint_len, u_length) = unsafe {
            (
                gl.get_uniform_location(program, "uCircRadius").unwrap(),
                gl.get_uniform_location(program, "uKeypointLen").unwrap(),
                gl.get_uniform_location(program, "uLength").unwrap(),
            )
        };

        let u_keypoints: [UKeypoint; MAX_KEYPOINTS] = array_init::array_init(|i| unsafe {
            UKeypoint {
                at: gl
                    .get_uniform_location(program, &format!("uKeypoints[{i}].at"))
                    .unwrap(),
                dst_head : gl
                    .get_uniform_location(program, &format!("uKeypoints[{i}].dstHead"))
                    .unwrap(),
                from : gl
                    .get_uniform_location(program, &format!("uKeypoints[{i}].from"))
                    .unwrap(),
            }
        });

        Self {
            program,
            u_circ_radius,
            u_keypoint_len,
            u_keypoints,
            u_length,
            attributes: HashMap::from(attributes),
        }
    }

    pub fn set_keypoints(&self, gl: &glow::Context, keypoints : &[DirKeypoint]) {
        assert!(keypoints.len() <= MAX_KEYPOINTS);
        unsafe {
            gl.uniform_1_u32(Some(&self.u_keypoint_len), keypoints.len().try_into().unwrap());
        }
        for i in 0..keypoints.len() {
            let kp = &keypoints[i];
            let kp_loc = &self.u_keypoints[i];
            unsafe {
                gl.uniform_1_u32(Some(&kp_loc.from), kp.from.clone() as u32);
                gl.uniform_2_f32(Some(&kp_loc.at), kp.at.x, kp.at.y);
                gl.uniform_1_f32(Some(&kp_loc.dst_head), kp.dst_head);
            }
        }
    }
    pub fn set_length(&self, gl: &glow::Context, x: f32) {
        unsafe {
            gl.uniform_1_f32(Some(&self.u_length), x);
        }
    }
    pub fn set_circle_radius(&self, gl: &glow::Context, x: f32) {
        unsafe {
            gl.uniform_1_f32(Some(&self.u_circ_radius), x);
        }
    }
}

impl Shader for SnekShader {
    fn get_attribute(&self, key: &str) -> Option<u32> {
        self.attributes.get(key).copied()
    }
    fn use_shader(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }
}
