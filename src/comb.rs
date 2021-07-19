pub struct Comb {
    delay: usize,
    gain: f32,
    pub output: Vec<f32>,
    prev_output: Vec<f32>,
}

impl Comb {
    pub fn new(delay: usize, gain: f32) -> Self {
        Self {
            delay,
            gain,
            output: Vec::new(),
            prev_output: Vec::new(),
        }
    }

    pub fn process(&mut self, input: &[f32]) {
        let samples = input.len();
        self.output.clear();
        for i in 0..samples {
            let echo = if self.delay >= self.prev_output.len() {
                0.0
            } else if i < self.delay {
                self.gain * self.prev_output[self.prev_output.len() - self.delay]
            } else {
                self.gain * self.output[i - self.delay]
            };
            let value = input[i] + echo;
            self.output.push(value);
            self.prev_output.push(value);
        }
        if self.prev_output.len() > self.delay {
            self.prev_output = self.output[self.prev_output.len() - self.delay..].into();
        }
    }
}
