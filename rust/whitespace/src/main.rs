use std::env;
use std::fs;
use std::io::{self};

struct Command<'a> {
    name: &'a str,
    token: &'a str,
    is_param: bool
}

struct Imp<'a> {
    name: &'a str,
    token: &'a str,
    commands: Vec<Command<'a>>,
}

fn run(source: &str) {
    let imps: Vec<Imp> = vec![
        Imp{name: "Stack Manipulation", token: " ", commands: vec![
            Command{name: "Push", token: " ", is_param: true},
            Command{name: "Duplicate", token: "\n ", is_param: false},
            Command{name: "Swap", token: "\n\t", is_param: false},
            Command{name: "Discard", token: "\n\n", is_param: false},
        ]},
        Imp{name: "Heap Access", token: "\t\t", commands: vec![
            Command{name: "Store", token: " ", is_param: false},
            Command{name: "Retrieve", token: "\t", is_param: false},
        ]},
        Imp{name: "I/O", token: "\t\n", commands: vec![
            Command{name: "Output char", token: "  ", is_param: false},
            Command{name: "Output number", token: " \t", is_param: false},
            Command{name: "Read char", token: "\t ", is_param: false},
            Command{name: "Read number", token: "\t\t", is_param: false},
        ]},
        Imp{name: "Flow Control", token: "\n", commands: vec![
            Command{name: "Mark", token: "  ", is_param: true},
            Command{name: "Call", token: " \t", is_param: true},
            Command{name: "Jump", token: " \n", is_param: true},
            Command{name: "Jump when zero", token: "\t ", is_param: true},
            Command{name: "Jump when negative", token: "\t\t", is_param: true},
            Command{name: "End subroutine", token: "\t\n", is_param: false},
            Command{name: "End", token: "\n\n", is_param: false},
        ]},
        Imp{name: "Arithmetic", token: "\t", commands: vec![
            Command{name: "Addition", token: "  ", is_param: false},
            Command{name: "Subtraction", token: " \t", is_param: false},
            Command{name: "Multiplication", token: " \n", is_param: false},
            Command{name: "Integer Division", token: "\t ", is_param: false},
            Command{name: "Modulo", token: "\t\t", is_param: false},
        ]},
    ];
    let code: String = source.chars().filter(|c| *c == ' ' || *c == '\t' || *c == '\n').collect();
    let mut stack: Vec<i8> = vec![];
    let mut ptr: usize = 0;
    let mut param: Option<&str> = None;
    loop {
        // find IMP
        match imps.iter().find(|imp| code[ptr..].find(imp.token) == Some(0)) {
            None => break,
            Some(imp) => {
                ptr += imp.token.len();
                match imp.commands.iter().find(|cmd| code[ptr..].find(cmd.token) == Some(0)) {
                    None => break,
                    Some(cmd) => {
                        ptr += cmd.token.len();
                        if cmd.is_param {
                            match code[ptr..].find("\n") {
                                None => break,
                                Some(n) => {
                                    param = Some(&code[ptr..ptr + n]);
                                    ptr += n + 1
                                }
                            }
                        } else {
                            param = None;
                        }
                        println!("{} {} {} {}", imp.name, cmd.name, str::replace(&str::replace(param.unwrap_or("None"), "\t", "[Tab]"), " ", "[Space]"), ptr);
                    }
                }
            }
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
