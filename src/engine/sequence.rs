#[derive(Clone)]
pub struct Frame {
    cell_name: String,
    duration: f32,
}

impl Frame {
    pub fn new(cell_name: String, duration: f32) -> Self {
        Self {
            cell_name: cell_name,
            duration: duration,
        }
    }
}

#[derive(Clone)]
pub struct Sequence {
    frames: Vec<Frame>,
    acc_time: f32,
    current_key_point_index: usize,
}

impl Sequence {
    pub fn new(frames: Vec<Frame>) -> Self {
        Self {
            frames: frames,
            acc_time: 0.0,
            current_key_point_index: 0,
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.acc_time += delta;
        if self.frames[self.current_key_point_index].duration < self.acc_time {
            self.acc_time -= self.frames[self.current_key_point_index].duration;
            self.current_key_point_index += 1;
        }

        if self.frames.len() <= self.current_key_point_index {
            self.current_key_point_index = 0;
        }
    }

    pub fn current_frame_cell_name(&self) -> &str {
        &self.frames[self.current_key_point_index].cell_name
    }
}
