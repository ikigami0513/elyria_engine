pub struct Time {
    delta_time: f32,
    last_frame: f32,
    fps: f32,
    frame_count: u32,
    last_fps_update: f32
}

#[allow(dead_code)]
impl Time {
    pub fn new() -> Self {
        Self {
            delta_time: 0.0,
            last_frame: 0.0,
            fps: 0.0,
            frame_count: 0,
            last_fps_update: 0.0
        }
    }

    pub fn update(&mut self, current_time: f64) {
        let current_frame = current_time as f32;
        self.delta_time = current_frame - self.last_frame;
        self.last_frame = current_frame;

        self.frame_count += 1;
        if current_frame - self.last_fps_update >= 1.0 {
            self.fps = self.frame_count as f32 / (current_frame - self.last_fps_update);
            self.frame_count = 0;
            self.last_fps_update = current_frame;
        }
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }
}