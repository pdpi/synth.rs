# A simple synthesizer in rust

## Prerequisites
 - Rust 1.3
 - Portaudio

On OSX, you can install portaudio from homebrew: `brew install portaudio`.

## Execution

You can build and run the program through cargo with `cargo run`.

## Controls

You can use your computer keyboard to play with the synthesizer.

'ZXCVBNNM,' map to the white keys on a piano, ranging from C to C
'SD GHJ' map to the black keys in the same range, C# to A#.

## To Do:

 - Figure out why sine waves don't sound right
 - Split core audio and synth control
 - Split input from main executable
 - Tweakable parameters
 - UI