extern crate glutin;
extern crate synth;

use synth::audio::{Audio,Note,Sender};
use glutin::{Event,ElementState,VirtualKeyCode};

fn resize_callback(_: u32, _: u32) {}

fn main() {
  let mult : f32 = (2.0 as f32).powf(1.0/12.0);
  let base : f32 = 220.0;
  let notes = [
  (base * mult.powi(3)) as usize,
  (base * mult.powi(4)) as usize,
  (base * mult.powi(5)) as usize,
  (base * mult.powi(6)) as usize,
  (base * mult.powi(7)) as usize,
  (base * mult.powi(8)) as usize,
  (base * mult.powi(9)) as usize,
  (base * mult.powi(10)) as usize,
  (base * mult.powi(11)) as usize,
  (base * mult.powi(12)) as usize,
  (base * mult.powi(13)) as usize,
  (base * mult.powi(14)) as usize,
  (base * mult.powi(15)) as usize
  ];

  let mut window = glutin::WindowBuilder::new().with_dimensions(100,100).build().unwrap();
  window.set_title("Synth");
  window.set_window_resize_callback(Some(resize_callback as fn(u32, u32)));
  let _ = unsafe { window.make_current() };
  let mut audio : Audio = Audio::new();
  {
    let chan = audio.start();

    let mut last_note = None;
    let mut note = None;
    for event in window.wait_events() {
      match event {
        Event::Closed => break,
        Event::KeyboardInput(ElementState::Released, _, _) => { note = None; },
        Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Z)) => { note = Some(0); },
        Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::S)) => { note = Some(1); },
        Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::X)) => { note = Some(2); },
        Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::D)) => { note = Some(3); },
        Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::C)) => { note = Some(4); },
        Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::V)) => { note = Some(5); },
        Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::G)) => { note = Some(6); },
        Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::B)) => { note = Some(7); },
        Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::H)) => { note = Some(8); },
        Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::N)) => { note = Some(9); },
        Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::J)) => { note = Some(10); },
        Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::M)) => { note = Some(11); },
        Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Comma)) => { note = Some(12); },
        Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
          chan.send(Note::EndStream).unwrap();
          break;
        },
        _ => {}
      }

      if last_note != note {
        last_note = note;
        match note {
          None => send_note(&chan, Note::Release),
          Some(i) => send_note(&chan, Note::Trigger(notes[i])),
        }
      }
    }
  }
}

fn send_note(chan: &Sender, note: Note) {
  match chan.send(note) {
    Ok(_) => (),
    Err(e) => { println!("{:?}", e); }
  }
}