use std::{collections::HashMap, time::{self, Duration}};

use glow::{COLOR_BUFFER_BIT, HasContext};

mod app_bootstraper;
pub mod app_owned_data;
pub mod collider;
pub mod board;

pub use app_bootstraper::AppBootstraper;
pub use collider::ColliderLayer;

use crate::{app::{app_owned_data::{AppOwnedData, Time}, board::Board}, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct App {
    pub gl: glow::Context,
    t_0: time::SystemTime,
    t_last_render: time::SystemTime,

    updateable_ids: Vec<usize>,
    input_listener_ids: Vec<usize>,
    collider_ids: Vec<usize>,

    board : Board,
    owned_data: HashMap<usize, AppOwnedData>,
    owned_data_counter: usize,

    fps : Vec<f32>,
    record_fps : bool,
    render_count : usize,

    on_app_init : fn(&mut Self)
}

impl App {
    pub fn new(gl: glow::Context, on_app_init : fn(&mut Self)) -> Self {
        let mut _self = Self {
            gl,
            t_last_render: time::SystemTime::now(),
            t_0: time::SystemTime::now(),

            updateable_ids: Vec::new(),
            input_listener_ids: Vec::new(),
            collider_ids: Vec::new(),

            board : Board::new(WINDOW_WIDTH, WINDOW_HEIGHT, 25),
            owned_data: HashMap::new(),
            owned_data_counter: 0,

            fps: Vec::with_capacity(100),
            record_fps : false,
            render_count : 0,
            on_app_init,
        };

        on_app_init(&mut _self);
        _self.after_on_app_init();

        _self
    }

    fn clear(&mut self) {
        self.updateable_ids.clear();
        self.input_listener_ids.clear();
        self.collider_ids.clear();
        self.owned_data.clear();
        self.owned_data_counter = 0;
        self.fps.clear();
        self.render_count = 0;
    }

    fn after_on_app_init(&mut self) {
        for idx in &mut self.updateable_ids {
            self.owned_data
                .get_mut(&idx)
                .expect("updateable ids should always updated to match existing item")
                .as_updateable()
                .expect("updateable ids should always fetch updateable from owned data")
                .on_setup(&self.gl, *idx, &self.board);       
        }
    }

    fn on_game_over(&mut self) {
        // reset everything
        self.clear();
        self.board = Board::new(WINDOW_WIDTH, WINDOW_HEIGHT, 25);
        self.t_0 = time::SystemTime::now();
        self.t_last_render = time::SystemTime::now();
        (self.on_app_init)(self);
        self.after_on_app_init();
    }

    // become owner of taken data
    pub fn take(&mut self, mut data: AppOwnedData) {
        let curr_data_counter = self.owned_data_counter;

        if data.as_updateable().is_some() {
            self.updateable_ids.push(curr_data_counter);
        }
        if data.as_collider().is_some() {
            self.collider_ids.push(curr_data_counter);
        }
        if data.as_input_listener().is_some() {
            self.input_listener_ids.push(curr_data_counter);
        }

        self.owned_data.insert(curr_data_counter, data);
        self.owned_data_counter += 1;
    }

    pub fn render(&mut self) {
        self.render_count +=1;
        let delta = self.calc_delta();
        let time =  Time{
                    delta : &delta, 
                    elapsed : &self.elapsed(),
                };

        if self.record_fps {
            let fps = Duration::from_secs(1).div_duration_f32(delta);
            if self.render_count % 50 == 0 {
                println!("sampled : {fps} fps");
                self.fps.push(fps);
            }
        }

        unsafe {
            self.gl.clear_color(0., 0.5, 0.5, 1.);
            self.gl.clear(COLOR_BUFFER_BIT);
        }

        let mut is_game_over = false;

        for idx in &self.updateable_ids {
            self.owned_data
                .get_mut(idx)
                .expect("updateable ids should always updated to match existing item")
                .as_updateable()
                .expect("updateable ids should always fetch updateable from owned data")
                .on_tick(&self.gl, &time, &self.board, &mut || {is_game_over = true});
        }
        if is_game_over {
            return self.on_game_over();
        }
        for (arr_s, idx_a) in self.collider_ids.iter().enumerate() {
            for idx_b in &self.collider_ids[arr_s+1..] {
                {
                    // check a against b from a side
                    // removing the b to satisfy borrow checker (it cant do multiple borrow at once)
                    let cldr_b = self.owned_data
                        .remove(idx_b)
                        .expect("collider ids should always updated to match existing item");
                        
                    let cldr_a = self.owned_data
                        .get_mut(idx_a)
                        .expect("collider ids should always updated to match existing item")
                        .as_collider()
                        .expect("collider ids should always fetch collider from owned data");
                    
                    cldr_a.check_collision(cldr_b.as_ref_collider().expect("collider ids should always fetch collider from owned data"));
                    self.owned_data.insert(*idx_b, cldr_b);
                }
                {
                    // check a against b from b side
                    // removing the a to satisfy borrow checker (it cant do multiple borrow at once)
                    let cldr_a = self.owned_data
                        .remove(idx_a)
                        .expect("collider ids should always updated to match existing item");
                        
                    let cldr_b = self.owned_data
                        .get_mut(idx_b)
                        .expect("collider ids should always updated to match existing item")
                        .as_collider()
                        .expect("collider ids should always fetch collider from owned data");
                    
                    cldr_b.check_collision(cldr_a.as_ref_collider().expect("collider ids should always fetch collider from owned data"));
                    self.owned_data.insert(*idx_a, cldr_a);
                }
            }
        }
    }

    pub fn on_exit(&mut self) {
        if self.record_fps {
            let fps_len = self.fps.len();
            println!("fps_avg over {} sample : {} fps", fps_len, self.fps.iter().sum::<f32>() / fps_len as f32)
        }
    }

    pub fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::KeyboardInput { device_id : _, event : _, is_synthetic :_ }  => {
                for r in &self.input_listener_ids {
                    self.owned_data
                        .get_mut(r)
                        .expect("input listener ids should always updated to match existing item")
                        .as_input_listener()
                        .expect("input listener ids should always fetch input listener from owned data")
                        .on_input(&event, &self.board);
                }
            },
            _ => {},
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
