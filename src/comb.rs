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

    fn update_prev_output(&mut self) {
        let prev_output_len = self.prev_output.len();
        if prev_output_len > self.delay {
            self.prev_output = self.prev_output.split_off(prev_output_len - self.delay);
        }
    }

    /// output[i] = input[i] + (gain * output[i - delay])
    pub fn process(&mut self, input: &[f32]) {
        let samples = input.len();
        self.output.clear();
        for i in 0..samples {
            let echo = if self.delay > self.prev_output.len() {
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
        self.update_prev_output();
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

    fn compare_comb_outputs(size: usize, sample_size: usize, delay: usize) {
        let mut input = vec![0.0; size];
        input[0] = 1.0;

        // Single input
        let mut comb = Comb::new(delay, 0.5);
        comb.process(&input);
        let out_single = comb.output.clone();

        // Sampled inputs
        let mut comb = Comb::new(delay, 0.5);

        let mut i = 0;
        let mut output_sampled = Vec::new();
        while i < size {
            let in_sampled = if i + sample_size < size {
                &input[i..i + sample_size]
            } else {
                &input[i..]
            };
            i += sample_size;
            comb.process(&in_sampled);
            output_sampled.append(&mut comb.output);
        }
        for i in 0..size {
            if out_single[i] != output_sampled[i] {
                println!("{}: {} != {}", i, out_single[i], output_sampled[i]);
            }
        }
        assert_eq!(out_single, output_sampled);
    }

    #[test]
    fn compare_comb_outputs_delay_lt_sample() {
        const SIZE: usize = 100;
        const SAMPLE_SIZE: usize = 10;
        const DELAY: usize = 4;
        compare_comb_outputs(SIZE, SAMPLE_SIZE, DELAY);
    }

    #[test]
    fn compare_comb_outputs_delay_eq_sample() {
        const SIZE: usize = 100;
        const SAMPLE_SIZE: usize = 10;
        const DELAY: usize = 10;
        compare_comb_outputs(SIZE, SAMPLE_SIZE, DELAY);
    }

    #[test]
    fn compare_comb_outputs_delay_gt_sample() {
        const SIZE: usize = 100;
        const SAMPLE_SIZE: usize = 5;
        const DELAY: usize = 11;
        compare_comb_outputs(SIZE, SAMPLE_SIZE, DELAY);
    }
}
