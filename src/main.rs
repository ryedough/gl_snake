use std::rc::Rc;

use crate::{
    app::{
        App, AppBootstraper,
        app_owned_data::{AppOwnedData, UpdtInpLstr},
    },
    object::Snek,
    shader::BasicShader,
};

mod app;
mod mesh;
mod object;
mod shader;

fn main() {
    let gl_app = AppBootstraper::new(on_app_init);
    gl_app.exec().unwrap();
}

fn on_app_init(app: &mut App) {
    let basic = Rc::new(BasicShader::new(&app.gl));
    {
        let square = Snek::new(app, basic.clone());
        app.take(AppOwnedData::from(Box::new(square) as Box<dyn UpdtInpLstr>));
    }
}
