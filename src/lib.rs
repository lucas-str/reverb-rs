#[macro_use]
extern crate vst;

use vst::buffer::AudioBuffer;
use vst::plugin::{Category, HostCallback, Info, Plugin};

struct Reverb {
    sample_rate: f64,
    comb1_output: Vec<f32>,
    comb_sum: Vec<f32>,
    ap1_output: Vec<f32>,
}

fn comb_filter(input: &[f32], delay: usize, gain: f32, output: &mut Vec<f32>) {
    let samples = input.len();
    for sample_idx in 0..samples {
        let echo1 = match delay >= output.len() {
            true => 0.0,
            false => gain * output[output.len() - delay],
        };
        let value = input[sample_idx] + echo1;
        output.push(value);
    }
}

impl Plugin for Reverb {
    fn new(_host: HostCallback) -> Self {
        Reverb {
            sample_rate: 44100.0,
            comb1_output: Vec::new(),
            comb_sum: Vec::new(),
            ap1_output: Vec::new(),
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: "Reverb".to_string(),
            unique_id: 36175,
            version: 1,
            inputs: 2,
            outputs: 2,
            category: Category::Effect,
            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let samples = buffer.samples();
        for (input_buffer, output_buffer) in buffer.zip() {
            /* Comb filters */
            const COMB1_DELAY: usize = 4799;
            const COMB1_GAIN: f32 = 0.742;
            comb_filter(
                input_buffer,
                COMB1_DELAY,
                COMB1_GAIN,
                &mut self.comb1_output,
            );
            /* Sum comb filters */
            self.comb_sum.clear();
            //for sample_idx in 0..samples {
            for sample_idx in (self.comb1_output.len() - samples)..self.comb1_output.len() {
                self.comb_sum.push(self.comb1_output[sample_idx]);
            }
            /* Reduce outputs */
            let comb_output1_len = self.comb1_output.len();
            if comb_output1_len > COMB1_DELAY {
                self.comb1_output = self.comb1_output.split_off(comb_output1_len - COMB1_DELAY);
            }
            /* All pass filter */
            const AP1_DELAY: usize = 1051;
            for sample_idx in 0..samples {
                const AP1_GAIN: f32 = 0.7;
                let in_delay = match AP1_DELAY >= self.comb_sum.len() {
                    true => 0.0,
                    false => self.comb_sum[self.comb_sum.len() - AP1_DELAY],
                };
                let out_delay = match AP1_DELAY >= self.ap1_output.len() {
                    true => 0.0,
                    false => AP1_GAIN * self.ap1_output[self.ap1_output.len() - AP1_DELAY],
                };
                let output = (-AP1_GAIN * self.comb_sum[sample_idx]) + in_delay + out_delay;
                self.ap1_output.push(output);
                output_buffer[sample_idx] = output;
            }
            let ap1_output_len = self.ap1_output.len();
            if ap1_output_len > AP1_DELAY {
                self.ap1_output = self.ap1_output.split_off(ap1_output_len - AP1_DELAY);
            }
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = f64::from(rate);
    }
}

plugin_main!(Reverb);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn process() {
        const SIZE: usize = 1024;
        let mut in1 = vec![0.0; SIZE];
        in1[0] = 1.0;

        let mut out1 = vec![0.0; SIZE];

        let inputs = vec![in1.as_ptr()];
        let mut outputs = vec![out1.as_mut_ptr()];
        let mut buffer =
            unsafe { AudioBuffer::from_raw(1, 1, inputs.as_ptr(), outputs.as_mut_ptr(), SIZE) };

        let host: HostCallback = Default::default();
        let mut reverb = Reverb::new(host);

        reverb.process(&mut buffer);
    }
}
