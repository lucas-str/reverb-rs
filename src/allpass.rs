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
        let mut in_offset = 0;
        let mut out_offset = 0;
        for i in 0..samples {
            let in_delay = if self.delay > self.prev_input.len() {
                self.prev_input.push(0.0);
                in_offset += 1;
                0.0
            } else if i < self.delay {
                self.prev_input[self.prev_input.len() - self.delay + i - in_offset]
            } else {
                input[i - self.delay]
            };
            let out_delay = if self.delay > self.prev_output.len() {
                self.prev_output.push(0.0);
                out_offset += 1;
                0.0
            } else if i < self.delay {
                self.gain * self.prev_output[self.prev_output.len() - self.delay + i - out_offset]
            } else {
                self.gain * self.output[i - self.delay]
            };
            let value = (-self.gain * input[i]) + in_delay + out_delay;
            self.output.push(value);
        }
        self.prev_input.clear();
        let delay = cmp::min(self.delay, input.len());
        let i = cmp::max((input.len() - delay) as i32, 0) as usize;
        for sample in input[i..].iter() {
            self.prev_input.push(*sample);
        }
        self.prev_output.clear();
        let delay = cmp::min(self.delay, self.output.len());
        let i = cmp::max((self.output.len() - delay) as i32, 0) as usize;
        for sample in self.output[i..].iter() {
            self.prev_output.push(*sample);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_allpass() {
        let mut allpass = AllPass::new(3, 0.5);

        const SIZE: usize = 10;
        let mut in1 = vec![0.0; SIZE];
        in1[0] = 1.0;

        allpass.process(&in1);

        let expected = vec![-0.5, 0.0, 0.0, 0.75, 0.0, 0.0, 0.375, 0.0, 0.0, 0.1875];

        assert_eq!(expected, allpass.output);
    }

    #[test]
    fn test_allpass_multiple_inputs() {
        let mut allpass = AllPass::new(3, 0.5);

        const SIZE: usize = 5;
        let mut in1 = vec![0.0; SIZE];
        let in2 = vec![0.0; SIZE];
        in1[0] = 1.0;

        allpass.process(&in1);
        let out1 = allpass.output.clone();
        allpass.process(&in2);
        let out2 = allpass.output.clone();

        let expected1 = vec![-0.5, 0.0, 0.0, 0.75, 0.0];
        let expected2 = vec![0.0, 0.375, 0.0, 0.0, 0.1875];

        assert_eq!(expected1, out1);
        assert_eq!(expected2, out2);
    }

    #[test]
    fn compare_allpass_outputs() {
        const SIZE: usize = 100;
        const DELAY: usize = SIZE / 2 + 1;

        // Single input
        let mut allpass = AllPass::new(DELAY, 0.5);

        let mut in1 = vec![0.0; SIZE];
        in1[0] = 1.0;

        allpass.process(&in1);

        let out_single = allpass.output.clone();

        // Multiple inputs
        let mut allpass = AllPass::new(DELAY, 0.5);

        let mut in1 = vec![0.0; SIZE / 2];
        let in2 = vec![0.0; SIZE / 2];
        in1[0] = 1.0;

        allpass.process(&in1);
        let mut out_multiple = allpass.output.clone();
        allpass.process(&in2);
        out_multiple.append(&mut allpass.output);

        assert_eq!(out_single, out_multiple);
    }
}
