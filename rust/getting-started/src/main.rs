use std::env;
use std::fs;
use std::io;
use std::io::Read;

const MEMORY_SIZE: usize = 1024;

fn run(source: &str) {
    let tokens: Vec<char> = source.chars().collect();
    let mut memory: [i8; MEMORY_SIZE] = [0; MEMORY_SIZE];
    let mut ptr: usize = 0;
    let mut jumps: Vec<usize> = vec![];
    let mut skips: Vec<usize> = vec![];
    let mut index: usize = 0;
    let mut buff: Vec<i8> = vec![];
    loop {
        match tokens.get(index) {
            None => break,
            Some('[') => {
                if memory[ptr] != 0 {
                    jumps.push(index)
                } else {
                    skips.push(index)
                }
            },
            Some(']') => {
                if skips.is_empty() {
                    index = jumps.pop().unwrap();
                    continue;
                } else {
                    skips.pop();
                }
            },
            Some('>') => {
                if skips.is_empty() {
                    if ptr < MEMORY_SIZE - 1 {
                        ptr += 1;
                    } else {
                        ptr = 0;
                    }
                }
            },
            Some('<') => {
                if skips.is_empty() {
                    if ptr > 0 {
                        ptr -= 1;
                    } else {
                        ptr = MEMORY_SIZE - 1;
                    }
                }
            },
            Some('+') => {
                if skips.is_empty() {
                    if memory[ptr] < i8::MAX {
                        memory[ptr] += 1;
                    } else {
                        memory[ptr] = i8::MIN;
                    }
                }
            },
            Some('-') => {
                if skips.is_empty() {
                    if memory[ptr] > i8::MIN {
                        memory[ptr] -= 1;
                    } else {
                        memory[ptr] = i8::MAX;
                    }
                }
            },
            Some('.') => {
                if skips.is_empty() {
                    match std::char::from_u32(memory[ptr] as u32) {
                        None => {},
                        Some(c) => { print!("{}", c); }
                    }
                }
            },
            Some(',') => {
                if buff.is_empty() {
                    for byte in io::stdin().bytes() {
                        match byte {
                            Ok(b) => buff.push(b as i8),
                            Err(_) => buff.push(-1)
                        }
                    }
                }
                if buff.is_empty() {
                    memory[ptr] = -1;
                } else {
                    memory[ptr] = buff.drain(0..1).as_slice()[0];
                }
            }
            Some(_) => {}
        }
        index += 1;
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
