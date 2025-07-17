#[derive(Clone, Debug, PartialEq, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct Board {
    pub midpoints : Vec<(f32, Vec<f32>)>,
    pub grid_size : f32,
    pub width : f32,
    pub height : f32,
}

impl Board{
    pub fn new(
        screen_w : u16,
        screen_h : u16,
        grid_size : u16,
    )-> Self{
        let n_x_box = (screen_w as f32/grid_size as f32) as usize;
        let n_y_box = (screen_h as f32/grid_size as f32) as usize;

        let mut midpoints = Vec::with_capacity(n_y_box);
        for i in 0..n_y_box {
            let y = (i as f32 + 0.5) * grid_size as f32;
            let mut row_midpts = Vec::with_capacity(n_x_box);
            for j in 0..n_x_box {
                let x = (j as f32 + 0.5) * grid_size as f32;
                row_midpts.push(x);
            }
            midpoints.push((y, row_midpts));
        }
        Board{
            grid_size : grid_size as f32,
            midpoints,
            height : n_y_box as f32 * grid_size as f32,
            width : n_x_box as f32 * grid_size as f32,
        }
    }

    pub fn current_midpts(&self, pos : Position) -> Option<Position> {
        let x_idx = (pos.x / self.grid_size as f32) as usize;
        let y_idx = (pos.y / self.grid_size as f32) as usize;

        if y_idx < self.midpoints.len() && x_idx < self.midpoints[y_idx].1.len() {
            Some(Position { x: self.midpoints[y_idx].1[x_idx], y: self.midpoints[y_idx].0 })
        } else {
            None
        }
    }
}