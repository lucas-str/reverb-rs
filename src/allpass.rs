use std::cmp;

pub struct AllPass {
    pub output: Vec<f32>,
    delay: usize,
    gain: f32,
    prev_input: Vec<f32>,
    prev_output: Vec<f32>,
}

impl AllPass {
    pub fn new(delay: usize, gain: f32) -> Self {
        Self {
            output: Vec::new(),
            delay,
            gain,
            prev_input: Vec::new(),
            prev_output: Vec::new(),
        }
    }

    pub fn process(&mut self, input: &[f32]) {
        let samples = input.len();
        self.output.clear();
        for i in 0..samples {
            let in_delay = if self.delay >= self.prev_input.len() {
                self.prev_input.push(0.0);
                0.0
            } else if i < self.delay {
                self.prev_input[self.prev_input.len() - self.delay]
            } else {
                input[i - self.delay]
            };
            let out_delay = if self.delay >= self.prev_output.len() {
                self.prev_output.push(0.0);
                0.0
            } else if i < self.delay {
                self.gain * self.prev_output[self.prev_output.len() - self.delay]
            } else {
                self.gain * self.output[i - self.delay]
            };
            let value = (-self.gain * input[i]) + in_delay + out_delay;
            self.output.push(value);
        }
        self.prev_input.clear();
        let i = cmp::max((input.len() - self.delay) as i32, 0) as usize;
        for sample in input[i..].iter() {
            self.prev_output.push(*sample);
        }
        self.prev_output.clear();
        let i = cmp::max((self.output.len() - self.delay) as i32, 0) as usize;
        for sample in self.output[i..].iter() {
            self.prev_output.push(*sample);
        }
    }
}
