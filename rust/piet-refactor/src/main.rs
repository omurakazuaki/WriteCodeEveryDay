use std::env;
use std::fs::File;
use piet::core::{ Codel, Piet };

fn main() {
    let source_path = env::args().nth(1).unwrap();
    let file = File::open(source_path).unwrap();
    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);
    let mut decoder = decoder.read_info(file).unwrap();

    let frame = decoder.read_next_frame().unwrap().unwrap();
    let piet = Piet::new(frame);
    let block: Vec<Codel> = piet.color_block();
    println!("{:?}", block.len());
}
