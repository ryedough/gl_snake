use std::rc::Rc;

use crate::{
    app::{
        App, AppBootstraper,
        app_owned_data::{AppOwnedData, UpdtInpLstr},
    },
    objects::snek::Snek,
    shaders::SnekShader,
};

mod app;
mod meshes;
mod objects;
mod shaders;

fn main() {
    let gl_app = AppBootstraper::new(on_app_init);
    gl_app.exec().unwrap();
}

fn on_app_init(app: &mut App) {
    let basic = Rc::new(SnekShader::new(&app.gl));
    {
        let square = Snek::new(app, basic.clone());
        app.take(AppOwnedData::from(Box::new(square) as Box<dyn UpdtInpLstr>));
    }
}
