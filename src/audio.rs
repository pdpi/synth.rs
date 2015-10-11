extern crate portaudio;

use self::portaudio::pa;

use osc;
use envelope;
use types::Source;

const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES: u32 = 1024;

pub enum Note {
  Trigger(usize),
  Release,
  EndStream
}

pub struct Audio {
  stream: pa::Stream<f32, f32>,
  output_device_id: pa::DeviceIndex,
  output_device: pa::DeviceInfo
}

impl Audio {

  fn output_stream_parameters(&self) -> pa::StreamParameters {
    pa::StreamParameters {
        device : self.output_device_id,
        channel_count : 2,
        sample_format : pa::SampleFormat::Float32,
        suggested_latency : self.output_device.default_low_output_latency
    }
  }
  pub fn new() -> Audio {

    let device_id = pa::device::get_default_output();
    Audio {
      stream: pa::Stream::new(),
      output_device_id: device_id,
      output_device: pa::device::get_info(device_id).unwrap()
    }
  }

  pub fn init() {
    pa::initialize().unwrap();
  }

  pub fn cleanup(&mut self) {
    self.stream.close().unwrap();
    pa::terminate().unwrap();
  }

  pub fn start(&mut self) -> ::std::sync::mpsc::Sender<Note> {
    let (sender, receiver) = ::std::sync::mpsc::channel();
    let osc = osc::Oscillator::sine();
    let mut env = envelope::Envelope::new(osc);
    let callback = Box::new(move |
      _: &[f32],
      output: &mut[f32],
      _: u32,
      _: &pa::StreamCallbackTimeInfo,
      _flags: pa::StreamCallbackFlags
    | -> pa::StreamCallbackResult {
      let rec = receiver.try_recv();
      match rec {
        Ok(Note::Trigger(f)) => env.trigger(f, 0.7),
        Ok(Note::Release) => env.release(),
        _ => {}
      };

      env.play(output);

      if let Ok(Note::EndStream) = rec {
        pa::StreamCallbackResult::Complete
      } else {
        pa::StreamCallbackResult::Continue
      }
    });
    let params = self.output_stream_parameters();
    self.stream.open(
      None,
      Some(&params),
      SAMPLE_RATE,
      FRAMES,
      pa::StreamFlags::empty(),
      Some(callback)).unwrap();
    self.stream.start().unwrap();
    sender
  }
}
