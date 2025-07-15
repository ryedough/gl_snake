use std::{collections::HashMap, time};

use glow::{COLOR_BUFFER_BIT, HasContext};

pub enum GlAppOwnedData {
    Updateable(Box<dyn Updateable>),
    InputListener(Box<dyn InputListener>),
    UpdateableInputListener(Box<dyn UpdateableInputListener>),
}

impl GlAppOwnedData {
    fn as_updateable(&mut self) -> Option<&mut dyn Updateable> {
        match self {
            Self::Updateable(x) => Some(x.as_mut()),
            Self::UpdateableInputListener(x) => Some(x.as_mut()),
            _ => None,
        }
    }
    fn as_input_listener(&mut self) -> Option<&mut dyn InputListener> {
        match self {
            Self::InputListener(x) => Some(x.as_mut()),
            Self::UpdateableInputListener(x) => Some(x.as_mut()),
            _ => None,
        }
    }

    pub fn from_updateable(data : impl Updateable + 'static) -> Self {
        Self::Updateable(Box::new(data))
    }
    pub fn from_input_listener(data : impl InputListener + 'static) -> Self {
        Self::InputListener(Box::new(data))
    }
    pub fn from_updateable_input_listener(data : impl UpdateableInputListener + 'static) -> Self {
        Self::UpdateableInputListener(Box::new(data))
    }
}

pub trait UpdateableInputListener: Updateable + InputListener {}
impl<T: InputListener + Updateable> UpdateableInputListener for T {}

pub trait InputListener {
    fn on_input(&mut self, event: &winit::event::WindowEvent);
}

pub trait Updateable {
    /// can also render inside this function
    fn on_tick(&mut self, gl: &glow::Context, delta: &time::Duration, since_0: &time::Duration);
    fn on_setup(&mut self, gl: &glow::Context);
}

pub struct GlApp {
    pub gl: glow::Context,
    t_0: time::SystemTime,
    t_last_render: time::SystemTime,

    updateable_ids: Vec<usize>,
    input_listener_ids: Vec<usize>,

    owned_data: HashMap<usize, GlAppOwnedData>,
    owned_data_counter: usize,
}

impl GlApp {
    pub fn new(gl: glow::Context) -> Self {
        let mut _self = Self {
            gl,
            t_last_render: time::SystemTime::now(),
            t_0: time::SystemTime::now(),
            updateable_ids: Vec::new(),
            input_listener_ids: Vec::new(),
            owned_data: HashMap::new(),
            owned_data_counter: 0,
        };

        _self
    }

    pub fn after_on_app_init(&mut self) {
        for idx in &mut self.updateable_ids {
            self.owned_data
                .get_mut(&idx)
                .expect("updateable ids should always updated to match existing item")
                .as_updateable()
                .expect("updateable ids should always fetch updateable from owned data")
                .on_setup(&self.gl);
        };
    }

    pub fn take(&mut self, data: GlAppOwnedData) {
        let curr_data_counter = self.owned_data_counter;
        match &data {
            GlAppOwnedData::InputListener(_) => {
                self.input_listener_ids.push(curr_data_counter);
                self.owned_data.insert(curr_data_counter, data);
            }
            GlAppOwnedData::Updateable(_) => {
                self.updateable_ids.push(curr_data_counter);
                self.owned_data.insert(curr_data_counter, data);
            }
            GlAppOwnedData::UpdateableInputListener(_) => {
                self.updateable_ids.push(curr_data_counter);
                self.input_listener_ids.push(curr_data_counter);
                self.owned_data.insert(curr_data_counter, data);
            }
        }
        self.owned_data_counter += 1;
    }

    pub fn render(&mut self) {
        let delta = self.calc_delta();

        unsafe {
            self.gl.clear_color(0., 0.5, 0.5, 1.);
            self.gl.clear(COLOR_BUFFER_BIT);
        }

        for r in &self.updateable_ids {
            let elapsed = self.elapsed();
            self.owned_data
                .get_mut(r)
                .expect("updateable ids should always updated to match existing item")
                .as_updateable()
                .expect("updateable ids should always fetch updateable from owned data")
                .on_tick(&self.gl, &delta, &elapsed);
        }
    }

    pub fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match &event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if event.logical_key
                    == winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape)
                {
                    event_loop.exit();
                }
            }
            _ => (),
        }
        for r in &self.input_listener_ids {
            self.owned_data
                .get_mut(r)
                .expect("input listener ids should always updated to match existing item")
                .as_input_listener()
                .expect("input listener ids should always fetch input listener from owned data")
                .on_input(&event);
        }
    }

    fn calc_delta(&mut self) -> time::Duration {
        let delta = time::SystemTime::now()
            .duration_since(self.t_last_render)
            .unwrap();
        self.t_last_render = time::SystemTime::now();
        delta
    }

    fn elapsed(&self) -> time::Duration {
        self.t_0.elapsed().unwrap()
    }
}
