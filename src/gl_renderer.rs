use std::time;

use glow::{HasContext, COLOR_BUFFER_BIT};

pub struct GlRenderer {
    gl : glow::Context,  
    t_last_render : time::SystemTime 
}

impl GlRenderer {
    pub fn new(gl : glow::Context) -> Self {
        Self { gl, t_last_render : time::SystemTime::now() }
    }
    pub fn render(&mut self) {
        let delta = time::SystemTime::now().duration_since(self.t_last_render).unwrap();
        self.t_last_render = time::SystemTime::now();

        println!("{} fps", time::Duration::from_secs(1).div_duration_f32(delta) as u32);

        unsafe {
            self.gl.clear_color(0., 0.5, 0.5, 1.);
            self.gl.clear(COLOR_BUFFER_BIT);
        }
    }
}