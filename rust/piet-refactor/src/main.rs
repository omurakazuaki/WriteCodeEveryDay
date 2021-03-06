use std::env;
use std::fs::File;
use piet::core::Piet;

fn main() {
    let source_path = env::args().nth(1).unwrap();
    let file = File::open(source_path).unwrap();
    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);
    let mut decoder = decoder.read_info(file).unwrap();

    let frame = decoder.read_next_frame().unwrap().unwrap();
    let mut piet = Piet::new(frame);
    while piet.try_step() {
        //println!("{}", piet);
    }
}
