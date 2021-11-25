use std::cmp;

pub struct Comb {
    pub output: Vec<f32>,
    delay: usize,
    gain: f32,
    prev_output: Vec<f32>,
}

impl Comb {
    pub fn new(delay: usize, gain: f32) -> Self {
        Self {
            output: Vec::new(),
            delay,
            gain,
            prev_output: Vec::new(),
        }
    }

    pub fn process(&mut self, input: &[f32]) {
        let samples = input.len();
        self.output.clear();
        for i in 0..samples {
            let echo = if self.delay >= self.prev_output.len() {
                self.prev_output.push(0.0);
                0.0
            } else if i < self.delay {
                self.gain * self.prev_output[self.prev_output.len() - self.delay]
            } else {
                self.gain * self.output[i - self.delay]
            };
            let value = input[i] + echo;
            self.output.push(value);
        }
        self.prev_output.clear();
        let i = cmp::max((self.output.len() - self.delay) as i32, 0) as usize;
        for sample in self.output[i..].iter() {
            self.prev_output.push(*sample);
        }
    }
}
