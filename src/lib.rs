mod allpass;
mod comb;

#[macro_use]
extern crate vst;

use vst::buffer::AudioBuffer;
use vst::plugin::{Category, HostCallback, Info, Plugin};

use allpass::AllPass;
use comb::Comb;

struct Reverb {
    sample_rate: f64,
    comb1: Comb,
    comb2: Comb,
    comb3: Comb,
    comb4: Comb,
    comb_sum: Vec<f32>,
    ap1: AllPass,
    ap2: AllPass,
}

impl Plugin for Reverb {
    fn new(_host: HostCallback) -> Self {
        Reverb {
            sample_rate: 44100.0,
            comb1: Comb::new(1621, 0.876),
            comb2: Comb::new(1400, 0.900),
            comb3: Comb::new(1819, 0.852),
            comb4: Comb::new(1843, 0.831),
            comb_sum: Vec::new(),
            ap1: AllPass::new(451, 0.7),
            ap2: AllPass::new(199, 0.7),
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
            self.comb1.process(input_buffer);
            self.comb2.process(input_buffer);
            self.comb3.process(input_buffer);
            self.comb4.process(input_buffer);
            /* Sum comb filters */
            let comb1_out = &self.comb1.output;
            let comb2_out = &self.comb2.output;
            let comb3_out = &self.comb3.output;
            let comb4_out = &self.comb4.output;
            self.comb_sum.clear();
            for i in 0..samples {
                self.comb_sum
                    .push((comb1_out[i] + comb2_out[i] + comb3_out[i] + comb4_out[i]) / 4.0);
            }
            /* All pass filter */
            self.ap1.process(&self.comb_sum);
            self.ap2.process(&self.ap1.output);
            for i in 0..samples {
                //output_buffer[i] = ap2_out[i];
                output_buffer[i] = self.ap2.output[i];
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
