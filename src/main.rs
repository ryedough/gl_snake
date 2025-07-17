use std::rc::Rc;

use crate::{
    app::{
        app_owned_data::{AppOwnedData, CldrUpdt, CldrUpdtInpLstr, Updateable, UpdtInpLstr}, App, AppBootstraper
    },
    objects::{food::Food, snek::Snek},
    shaders::SnekShader,
};

mod app;
mod meshes;
mod objects;
mod shaders;

pub const MAX_FPS : f32 = 60.; 
pub const WINDOW_WIDTH : u16 = 400;
pub const WINDOW_HEIGHT : u16 = 400;

fn main() {
    let gl_app = AppBootstraper::new(on_app_init);
    gl_app.exec().unwrap();
}

fn on_app_init(app: &mut App) {
    let basic = Rc::new(SnekShader::new(&app.gl));
    let square = Snek::new(app, basic.clone());
    let food = Food::new(&app.gl);
    app.take(AppOwnedData::from(Box::new(square) as Box<dyn CldrUpdtInpLstr>));
    app.take(AppOwnedData::from(Box::new(food) as Box<dyn CldrUpdt>));
}