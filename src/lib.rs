#[macro_use]
extern crate vst;

use vst::buffer::AudioBuffer;
use vst::plugin::{Category, HostCallback, Info, Plugin};

mod comb;

struct Reverb {
    sample_rate: f64,
    comb1_out: Vec<f32>,
    comb2_out: Vec<f32>,
    comb3_out: Vec<f32>,
    comb4_out: Vec<f32>,
    comb_sum: Vec<f32>,
    ap1_output: Vec<f32>,
    ap2_input: Vec<f32>,
    ap2_output: Vec<f32>,
}

fn comb_filter(input: &[f32], delay: usize, gain: f32, output: &mut Vec<f32>) {
    let samples = input.len();
    for sample_idx in 0..samples {
        let echo = match delay >= output.len() {
            true => 0.0,
            false => gain * output[output.len() - delay],
        };
        let value = input[sample_idx] + echo;
        output.push(value);
    }
}

fn all_pass_filter(input: &[f32], delay: usize, gain: f32, output: &mut Vec<f32>) {
    let samples = input.len();
    for sample_idx in 0..samples {
        let in_delay = match sample_idx < delay {
            true => 0.0,
            false => input[sample_idx - delay],
        };
        let out_delay = match delay >= output.len() {
            true => 0.0,
            false => gain * output[output.len() - delay],
        };
        let value = (-gain * input[sample_idx]) + in_delay + out_delay;
        output.push(value);
    }
}

impl Plugin for Reverb {
    fn new(_host: HostCallback) -> Self {
        Reverb {
            sample_rate: 44100.0,
            comb1_out: Vec::new(),
            comb2_out: Vec::new(),
            comb3_out: Vec::new(),
            comb4_out: Vec::new(),
            comb_sum: Vec::new(),
            ap1_output: Vec::new(),
            ap2_input: Vec::new(),
            ap2_output: Vec::new(),
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
            const COMB1_DELAY: usize = 1621 * 2;
            const COMB2_DELAY: usize = 1400 * 2;
            const COMB3_DELAY: usize = 1819 * 2;
            const COMB4_DELAY: usize = 1843 * 2;
            const COMB_MULT: f32 = 1.0; // 0.835
            const COMB1_GAIN: f32 = 0.876 * COMB_MULT; // 0.805
            const COMB2_GAIN: f32 = 0.900 * COMB_MULT; // 0.827
            const COMB3_GAIN: f32 = 0.852 * COMB_MULT; // 0.783
            const COMB4_GAIN: f32 = 0.831 * COMB_MULT; // 0.764
            comb_filter(input_buffer, COMB1_DELAY, COMB1_GAIN, &mut self.comb1_out);
            comb_filter(input_buffer, COMB2_DELAY, COMB2_GAIN, &mut self.comb2_out);
            comb_filter(input_buffer, COMB3_DELAY, COMB3_GAIN, &mut self.comb3_out);
            comb_filter(input_buffer, COMB4_DELAY, COMB4_GAIN, &mut self.comb4_out);
            /* Sum comb filters */
            self.comb_sum.clear();
            let c1_len = self.comb1_out.len();
            let comb1_out = &self.comb1_out[(c1_len - samples)..c1_len];
            let c2_len = self.comb2_out.len();
            let comb2_out = &self.comb2_out[(c2_len - samples)..c2_len];
            let c3_len = self.comb3_out.len();
            let comb3_out = &self.comb3_out[(c3_len - samples)..c3_len];
            let c4_len = self.comb4_out.len();
            let comb4_out = &self.comb4_out[(c4_len - samples)..c4_len];
            for i in 0..samples {
                self.comb_sum
                    .push((comb1_out[i] + comb2_out[i] + comb3_out[i] + comb4_out[i]) / 4.0);
            }
            //for i in 0..samples {
            //    output_buffer[i] = self.comb_sum[i];
            //}
            /* Reduce outputs */
            if c1_len > COMB1_DELAY {
                self.comb1_out = self.comb1_out.split_off(c1_len - COMB1_DELAY);
            }
            if c2_len > COMB2_DELAY {
                self.comb2_out = self.comb2_out.split_off(c2_len - COMB2_DELAY);
            }
            if c3_len > COMB3_DELAY {
                self.comb3_out = self.comb3_out.split_off(c3_len - COMB3_DELAY);
            }
            if c4_len > COMB4_DELAY {
                self.comb4_out = self.comb4_out.split_off(c4_len - COMB4_DELAY);
            }
            /* All pass filter */
            const AP1_DELAY: usize = 451;
            const AP1_GAIN: f32 = 0.7;
            const AP2_DELAY: usize = 199;
            const AP2_GAIN: f32 = 0.7;
            all_pass_filter(&self.comb_sum, AP1_DELAY, AP1_GAIN, &mut self.ap1_output);
            self.ap2_input.clear();
            let ap1_len = self.ap1_output.len();
            for sample_idx in (ap1_len - samples)..ap1_len {
                self.ap2_input.push(self.ap1_output[sample_idx]);
            }
            all_pass_filter(&self.ap2_input, AP2_DELAY, AP2_GAIN, &mut self.ap2_output);

            let ap2_len = self.ap2_output.len();
            let ap2_out = &self.ap2_output[(ap2_len - samples)..ap2_len];
            for i in 0..samples {
                output_buffer[i] = ap2_out[i];
            }

            if ap1_len > AP1_DELAY {
                self.ap1_output = self.ap1_output.split_off(ap1_len - AP1_DELAY);
            }
            if ap2_len > AP2_DELAY {
                self.ap2_output = self.ap2_output.split_off(ap2_len - AP2_DELAY);
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
