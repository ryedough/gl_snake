use glow::{HasContext, NativeProgram};

mod basic;

pub use basic::BasicShader;

pub trait Shader {
    fn use_shader(&self, gl: &glow::Context);
}

fn gen_program(gl: &glow::Context, vs_str :&str, fs_str: &str) -> Result<NativeProgram, String> {
    let shader_srcs = [
        (glow::VERTEX_SHADER, &vs_str),
        (glow::FRAGMENT_SHADER, &fs_str),
    ]; 

    let shaders = unsafe {
        let mut res = Vec::with_capacity(shader_srcs.len()); 
        for (kind, src) in shader_srcs {
            let shader = gl.create_shader(kind)?;
            gl.shader_source(shader, src);
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                return Err(gl.get_shader_info_log(shader));
            };
            res.push(shader);
        }
        res
    };

    let program = unsafe {
        let program = gl.create_program()?;
        for shader in &shaders {
            gl.attach_shader(program, *shader);
        };
        
        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            return Err(gl.get_program_info_log(program));
        }
        for shader in &shaders {
            gl.detach_shader(program, *shader);
            gl.delete_shader(*shader);
        };
        program
    };

    Ok(program)
}