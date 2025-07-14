use std::rc::Rc;

use glutin::config::ConfigTemplateBuilder;
use crate::{gl_app::{GlApp, GlAppOwnedData}, gl_bootstraper::GlBootstraper, object::Square, shader::BasicShader};

mod shader;
mod mesh;
mod object;
mod gl_bootstraper;
mod gl_app;

fn main() {
    let template = ConfigTemplateBuilder::default();
    let gl_app = GlBootstraper::new(template, on_app_init);
    gl_app.exec().unwrap();
}

fn on_app_init(app : &mut GlApp) {
    let basic = Rc::new(BasicShader::new(&app.gl));
    let square = Square::new(app, basic.clone());
    app.take(GlAppOwnedData::All(Box::from(square)));
}