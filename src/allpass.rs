use std::cmp::Ordering;

pub struct AllPass {
    pub output: Vec<f32>,
    delay: usize,
    gain: f32,
    prev_input: Vec<Vec<f32>>,
    prev_output: Vec<Vec<f32>>,
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

    fn update_prev_inout(&mut self, chan: usize) {
        let prev_input = &mut self.prev_input[chan];
        let prev_input_len = prev_input.len();
        if prev_input_len > self.delay {
            self.prev_input[chan] = prev_input.split_off(prev_input_len - self.delay);
        }
        let prev_output = &mut self.prev_output[chan];
        let prev_output_len = prev_output.len();
        if prev_output_len > self.delay {
            self.prev_output[chan] = prev_output.split_off(prev_output_len - self.delay);
        }
    }

    /// output[i] = -(gain * input[i]) + input[i - delay] + (gain * output[i - delay])
    pub fn process(&mut self, input: &[f32], chan: u8) {
        let samples = input.len();
        self.output.clear();
        let chan = chan as usize;
        match chan.cmp(&self.prev_output.len()) {
            Ordering::Greater => panic!("channel {} out of bound", chan),
            Ordering::Equal => {
                self.prev_output.push(Vec::new());
                self.prev_input.push(Vec::new());
            }
            Ordering::Less => {}
        }
        let prev_input = &mut self.prev_input[chan];
        let prev_output = &mut self.prev_output[chan];
        for sample in input.iter().take(samples) {
            let in_delay = if self.delay > prev_input.len() {
                0.0
            } else {
                prev_input[prev_input.len() - self.delay]
            };
            let out_delay = if self.delay > prev_output.len() {
                0.0
            } else {
                self.gain * prev_output[prev_output.len() - self.delay]
            };
            let value = (-self.gain * sample) + in_delay + out_delay;
            self.output.push(value);
            prev_output.push(value);
            prev_input.push(*sample);
        }
        self.update_prev_inout(chan);
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

        allpass.process(&in1, 0);

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

        allpass.process(&in1, 0);
        let out1 = allpass.output.clone();
        allpass.process(&in2, 0);
        let out2 = allpass.output.clone();

        let expected1 = vec![-0.5, 0.0, 0.0, 0.75, 0.0];
        let expected2 = vec![0.0, 0.375, 0.0, 0.0, 0.1875];

        assert_eq!(expected1, out1);
        assert_eq!(expected2, out2);
    }

    fn compare_allpass_outputs(size: usize, sample_size: usize, delay: usize) {
        let mut input = vec![0.0; size];
        input[0] = 1.0;

        // Single input
        let mut allpass = AllPass::new(delay, 0.5);
        allpass.process(&input, 0);
        let out_single = allpass.output.clone();

        // Multiple inputs
        let mut allpass = AllPass::new(delay, 0.5);

        let mut i = 0;
        let mut output_sampled = Vec::new();
        while i < size {
            let in_sampled = if i + sample_size < size {
                &input[i..i + sample_size]
            } else {
                &input[i..]
            };
            i += sample_size;
            allpass.process(&in_sampled, 0);
            output_sampled.append(&mut allpass.output);
        }
        for i in 0..size {
            if out_single[i] != output_sampled[i] {
                println!("{}: {} != {}", i, out_single[i], output_sampled[i]);
            }
        }
        assert_eq!(out_single, output_sampled);
    }

    #[test]
    fn compare_allpass_outputs_delay_lt_sample() {
        const SIZE: usize = 100;
        const SAMPLE_SIZE: usize = 10;
        const DELAY: usize = 4;
        compare_allpass_outputs(SIZE, SAMPLE_SIZE, DELAY);
    }

    #[test]
    fn compare_allpass_outputs_delay_eq_sample() {
        const SIZE: usize = 100;
        const SAMPLE_SIZE: usize = 10;
        const DELAY: usize = 10;
        compare_allpass_outputs(SIZE, SAMPLE_SIZE, DELAY);
    }

    #[test]
    fn compare_allpass_outputs_delay_gt_sample() {
        const SIZE: usize = 100;
        const SAMPLE_SIZE: usize = 10;
        const DELAY: usize = 21;
        compare_allpass_outputs(SIZE, SAMPLE_SIZE, DELAY);
    }
}
