use std::rc::Rc;

use crate::{gl_app::{GlApp, GlAppOwnedData}, gl_bootstraper::GlBootstraper, object::Snek, shader::BasicShader};

mod shader;
mod mesh;
mod object;
mod gl_bootstraper;
mod gl_app;

fn main() {
    let gl_app = GlBootstraper::new(on_app_init);
    gl_app.exec().unwrap();
}

fn on_app_init(app : &mut GlApp) {
    let basic = Rc::new(BasicShader::new(&app.gl));
    { 
        let square = Snek::new(app, basic.clone());
        app.take(GlAppOwnedData::from_updateable_input_listener(square));
    }
}