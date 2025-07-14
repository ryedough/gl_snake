use glutin::config::ConfigTemplateBuilder;
use crate::gl_windower::GlWindower;

mod shader;
mod gl_windower;
mod gl_renderer;

fn main() {
    let template = ConfigTemplateBuilder::default();
    let gl_app = GlWindower::new(template);
    gl_app.exec().unwrap();
}