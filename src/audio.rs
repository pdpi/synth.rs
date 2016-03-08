extern crate portaudio;

use self::portaudio as pa;

use osc;
use envelope;
use types::Source;

const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES: u32 = 1024;
const CHANNELS: i32 = 2;
const INTERLEAVED: bool = true;

pub enum Note {
  Trigger(usize),
  Release,
  EndStream
}

pub struct Audio {
  pa: pa::PortAudio,
}

impl Audio {

  fn output_stream_parameters(& self) -> pa::StreamParameters<f32> {
    let dev = self.pa.default_output_device().unwrap();
    let dev_info = self.pa.device_info(dev).unwrap();
    pa::StreamParameters::<f32>::new(
      dev,
      CHANNELS,
      INTERLEAVED,
      dev_info.default_low_output_latency)
  }
  pub fn new() -> Audio {
    let pa = pa::PortAudio::new().unwrap();
    Audio {
      pa: pa
    }
  }

  pub fn init() {
    // pa::initialize().unwrap();
  }

  pub fn start<'a>(&'a mut self) -> Sender<'a> {
    let (sender, receiver) = ::std::sync::mpsc::channel();
    let osc = osc::Oscillator::sine();
    let mut env = envelope::Envelope::new(osc);

    let callback = move | pa::OutputStreamCallbackArgs {buffer, .. } | {
      let rec = receiver.try_recv();
      match rec {
        Ok(Note::Trigger(f)) => env.trigger(f, 0.7),
        Ok(Note::Release) => env.release(),
        _ => {}
      };

      env.play(buffer);

      if let Ok(Note::EndStream) = rec {
        pa::Complete
      } else {
        pa::Continue
      }
    };
    let params = self.output_stream_parameters();
    let settings = pa::OutputStreamSettings::new(params, SAMPLE_RATE, FRAMES);
    let mut stream = self.pa.open_non_blocking_stream(settings, callback).unwrap();
    stream.start().unwrap();
    Sender {
      stream: stream,
      sender: sender
    }
  }
}

pub struct Sender<'a> {
  stream: pa::Stream<'a, pa::stream::NonBlocking,pa::stream::Output<f32>>,
  sender: ::std::sync::mpsc::Sender<Note>
}

impl<'a> Sender<'a> {
  pub fn send(&self, n: Note) -> Result<(), ::std::sync::mpsc::SendError<Note>> {
    self.sender.send(n)
  }
}

impl<'a> Drop for Sender<'a> {
  fn drop(&mut self) {
    self.stream.close();
  }
}
