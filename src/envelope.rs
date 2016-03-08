use osc::Oscillator;
use types::Source;

enum EnvelopeState {
  Attack,
  Decay,
  Sustain,
  Release,
  Idle
}
pub struct Envelope {
  state: EnvelopeState,
  output: f32,
  attack_rate: f32,
  // target_ratio_a: f32,
  decay_rate: f32,
  sustain_value: f32,
  release_rate: f32,
  osc: Box<Oscillator>
}

impl Envelope {
  pub fn new(osc: Oscillator) -> Envelope {
    Envelope {
      output: 0.0,
      state: EnvelopeState::Idle,
      attack_rate: 0.0003,
      decay_rate: 1.0005,
      sustain_value: 0.75,
      release_rate: 0.00005,
      osc: Box::new(osc)
    }
  }

  fn control_output(&mut self) -> f32 {
    match self.state {
      EnvelopeState::Attack => {
        self.output += self.attack_rate;
        if self.output > 1.0 {
          self.output = 1.0;
          self.state = EnvelopeState::Decay;
        }
      },
      EnvelopeState::Decay =>{
        self.output /= self.decay_rate;
        if self.output < self.sustain_value {
          self.output = self.sustain_value;
          self.state = EnvelopeState::Sustain;
        }
      }
      EnvelopeState::Sustain => { /* Do nothing. output is already at sustain level */ },
      EnvelopeState::Release => {
        self.output -= self.release_rate;
        if self.output < 0.0 {
          self.output = 0.0;
          self.state = EnvelopeState::Idle;
        }
      },
      EnvelopeState::Idle => { self.output = 0.0; }
    }
    self.output
  }
}

impl Source for Envelope {
  fn release(&mut self) {
    self.state = EnvelopeState::Release;
    self.osc.release();
  }

  fn trigger(&mut self, pitch: usize, amplitude: f32) {
    self.state = EnvelopeState::Attack;
    self.osc.trigger(pitch, amplitude);
  }

  fn play(&mut self, output: &mut [f32]) {
    self.osc.play(output);
    let mut i = 0;
    while i < output.len() {
      let control = self.control_output();
      /* left */
      output[i] *= control;
      /* right */
      output[i + 1] *= control;
      i+=2;
    }
  }
}