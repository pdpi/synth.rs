pub trait Source {
  fn play(&mut self, output: &mut [f32]);
  fn trigger(&mut self, pitch: usize, amplitude: f32);
  fn release(&mut self);
}