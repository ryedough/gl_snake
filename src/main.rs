use std::rc::Rc;

use crate::{app::{App, AppBootstraper, AppOwnedData}, object::Snek, shader::BasicShader};

mod shader;
mod mesh;
mod object;
mod app;

fn main() {
    let gl_app = AppBootstraper::new(on_app_init);
    gl_app.exec().unwrap();
}

fn on_app_init(app : &mut App) {
    let basic = Rc::new(BasicShader::new(&app.gl));
    { 
        let square = Snek::new(app, basic.clone());
        app.take(AppOwnedData::from_updt_input(square));
    }
}