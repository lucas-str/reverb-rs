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

    /// output[i] = input[i] + (gain * output[i - delay])
    pub fn process(&mut self, input: &[f32]) {
        let samples = input.len();
        self.output.clear();
        let mut offset = 0;
        for i in 0..samples {
            let echo = if self.delay > self.prev_output.len() {
                self.prev_output.push(0.0);
                offset += 1;
                0.0
            } else if i < self.delay {
                self.gain * self.prev_output[self.prev_output.len() - self.delay + i - offset]
            } else {
                self.gain * self.output[i - self.delay]
            };
            let value = input[i] + echo;
            self.output.push(value);
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
    fn test_comb() {
        let mut comb = Comb::new(3, 0.5);

        const SIZE: usize = 10;
        let mut in1 = vec![0.0; SIZE];
        in1[0] = 1.0;

        comb.process(&in1);

        let expected = vec![1.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.25, 0.0, 0.0, 0.125];

        assert_eq!(expected, comb.output);
    }

    #[test]
    fn test_comb_multiple_inputs() {
        let mut comb = Comb::new(3, 0.5);

        const SIZE: usize = 5;
        let mut in1 = vec![0.0; SIZE];
        let in2 = vec![0.0; SIZE];
        in1[0] = 1.0;

        comb.process(&in1);
        let out1 = comb.output.clone();
        comb.process(&in2);
        let out2 = comb.output.clone();

        let expected1 = vec![1.0, 0.0, 0.0, 0.5, 0.0];
        let expected2 = vec![0.0, 0.25, 0.0, 0.0, 0.125];

        assert_eq!(expected1, out1);
        assert_eq!(expected2, out2);
    }

    #[test]
    fn test_comb_multiple_inputs_2() {
        let mut comb = Comb::new(6, 0.5);

        const SIZE: usize = 5;
        let mut in1 = vec![0.0; SIZE];
        let in2 = vec![0.0; SIZE];
        in1[0] = 1.0;

        comb.process(&in1);
        let out1 = comb.output.clone();
        comb.process(&in2);
        let out2 = comb.output.clone();

        let expected1 = vec![1.0, 0.0, 0.0, 0.0, 0.0];
        let expected2 = vec![0.0, 0.5, 0.0, 0.0, 0.0];

        assert_eq!(expected1, out1);
        assert_eq!(expected2, out2);
    }

    #[test]
    fn compare_comb_outputs() {
        const SIZE: usize = 100;
        const DELAY: usize = SIZE / 2 + 1;

        // Single input
        let mut comb = Comb::new(DELAY, 0.5);

        let mut in1 = vec![0.0; SIZE];
        in1[0] = 1.0;

        comb.process(&in1);

        let out_single = comb.output.clone();

        // Multiple inputs
        let mut comb = Comb::new(DELAY, 0.5);

        let mut in1 = vec![0.0; SIZE / 2];
        let in2 = vec![0.0; SIZE / 2];
        in1[0] = 1.0;

        comb.process(&in1);
        let mut out_multiple = comb.output.clone();
        comb.process(&in2);
        out_multiple.append(&mut comb.output);

        assert_eq!(out_single, out_multiple);
    }
}
