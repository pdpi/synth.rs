use std::f32;
use types::Source;

pub struct Oscillator {
  sample_rate: usize,
  phase: usize,
  pub amplitude: f32,
  pub pitch: usize,
  waveform: Vec<f32>
}

impl Oscillator {
  pub fn new() -> Oscillator {
    Oscillator {
      sample_rate: 44_100,
      phase: 0,
      amplitude: 0.25,
      pitch: 440,
      waveform: Vec::with_capacity(44_100)
    }
  }

  pub fn sine() -> Oscillator {
    let mut osc = Oscillator::new();
    for i in  0..osc.sample_rate {
      osc.waveform.push((f32::consts::PI * 2.0 * i as f32 / osc.sample_rate as f32).sin());
    }
    osc
  }

  pub fn square() -> Oscillator {
    let mut osc = Oscillator::new();
    for i in  0..osc.sample_rate {
      osc.waveform.push(if i < osc.sample_rate / 2 {-0.5} else {0.5});
    }
    osc
  }
}

impl Source for Oscillator {
  fn play(&mut self, output: &mut [f32]) {
    let mut i = 0;
    while i < output.len() {
      let out = self.amplitude * self.waveform[self.phase];
      /* left */
      output[i] = out;
      /* right */
      output[i + 1] = out;
      self.phase += self.pitch;
      while self.phase >= self.sample_rate { self.phase -= self.sample_rate; }
      i+=2;
    }
  }

  fn release(&mut self) {}
  fn trigger(&mut self, pitch: usize, amplitude: f32) {
    self.pitch = pitch;
    self.amplitude = amplitude;
  }

}