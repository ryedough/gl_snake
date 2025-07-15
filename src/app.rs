use std::{collections::HashMap, time};

use glow::{COLOR_BUFFER_BIT, HasContext};

mod app_bootstraper;
mod app_owned_data;
mod registered_collider;

pub use app_bootstraper::AppBootstraper;
pub use app_owned_data::{AppOwnedData, InputListener, Updateable};
pub use registered_collider::RegisteredCollider;

pub struct App {
    pub gl: glow::Context,
    t_0: time::SystemTime,
    t_last_render: time::SystemTime,

    updateable_ids: Vec<usize>,
    input_listener_ids: Vec<usize>,

    owned_data: HashMap<usize, AppOwnedData>,
    owned_data_counter: usize,
}

impl App {
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

    // become owner of taken data
    pub fn take(&mut self, data: AppOwnedData) {
        let curr_data_counter = self.owned_data_counter;
        match &data {
            AppOwnedData::InputListener(_) => {
                self.input_listener_ids.push(curr_data_counter);
                self.owned_data.insert(curr_data_counter, data);
            }
            AppOwnedData::Updateable(_) => {
                self.updateable_ids.push(curr_data_counter);
                self.owned_data.insert(curr_data_counter, data);
            }
            AppOwnedData::UpdateableInputListener(_) => {
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
