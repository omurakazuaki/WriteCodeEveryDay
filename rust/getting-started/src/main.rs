use std::env;
use std::fs;
use std::io;
use std::io::Read;
use bf::memory::Memory;

fn run(source: &str) {
    let tokens: Vec<char> = source.chars().collect();
    let mut memory: Memory = Memory::new();
    let mut jumps: Vec<usize> = vec![];
    let mut skips: Vec<usize> = vec![];
    let mut index: usize = 0;
    let mut read = stdin_reader();
    loop {
        match tokens.get(index) {
            None => break,
            Some('[') => {
                if memory.get() != 0 {
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
                    memory.nxt();
                }
            },
            Some('<') => {
                if skips.is_empty() {
                    memory.prv();
                }
            },
            Some('+') => {
                if skips.is_empty() {
                    memory.inc();
                }
            },
            Some('-') => {
                if skips.is_empty() {
                    memory.dec()
                }
            },
            Some('.') => {
                if skips.is_empty() {
                    match std::char::from_u32(memory.get() as u32) {
                        None => {},
                        Some(c) => { print!("{}", c); }
                    }
                }
            },
            Some(',') => {
                memory.put(read());
            }
            Some(_) => {}
        }
        index += 1;
    }
}

fn stdin_reader() -> impl FnMut() -> i8 {
    let mut buff: Vec<i8> = vec![];
    move || -> i8 {
        if buff.is_empty() {
            for byte in io::stdin().bytes() {
                match byte {
                    Ok(b) => buff.push(b as i8),
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
