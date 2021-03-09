use std::env;
use std::fs;
use std::io::{self, Read};
use std::collections::HashMap;
use bf::memory::Memory;
use bf::optimizer::Optimizer;

fn run(source: &str) {
    let mut optimized = Optimizer::new(source.chars().collect());
    optimized.delete_comments();
    optimized.optimize_set_zero();
    optimized.optimize_continuos();
    let jumps = optimized.jump_points();
    let jump_by_open = jumps.iter().fold(HashMap::new(), |mut map, v| {
        map.insert(v[0], v[1]);
        map
    });
    let jump_by_close = jumps.iter().fold(HashMap::new(), |mut map, v| {
        map.insert(v[1], v[0]);
        map
    });
    let mut memory: Memory = Memory::new();
    let mut index: usize = 0;
    let mut read = stdin_reader();
    let (tokens, continuos) = optimized.build();
    loop {
        match tokens.get(index) {
            None => break,
            Some('[') => {
                if memory.get() == 0 {
                    index = *jump_by_open.get(&index).unwrap();
                }
            },
            Some(']') => {
                if memory.get() != 0 {
                    index = *jump_by_close.get(&index).unwrap() - 1;
                }
            },
            Some('>') => memory.mov(continuos[index]),
            Some('<') => memory.mov(-continuos[index]),
            Some('+') => memory.add(continuos[index]),
            Some('-') => memory.add(-continuos[index]),
            Some('0') => memory.clr(),
            Some('.') => { print!("{}", memory.get() as u8 as char)},
            Some(',') => memory.put(read()),
            Some(_) => {}
        }
        index += 1;
    }
}

fn stdin_reader() -> impl FnMut() -> isize {
    let mut buff: Vec<isize> = vec![];
    move || -> isize {
        if buff.is_empty() {
            for byte in io::stdin().bytes() {
                match byte {
                    Ok(b) => buff.push(b as isize),
                    Err(_) => buff.push(-1)
                }
            }
        }
        if buff.is_empty() {
            return -1;
        } else {
            return buff.drain(0..1).as_slice()[0];
        }
    }
}

fn read_source(path: &str) -> io::Result<String> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e),
    }
}

fn main() {
    let source_path = env::args().nth(1).unwrap();
    let source = read_source(&source_path).unwrap();
    run(&source);
}
