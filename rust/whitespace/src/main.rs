use std::env;
use std::fs;
use std::io::{self};
use std::collections::HashMap;

#[derive(Debug)]
struct Command<'a> {
    name: &'a str,
    token: &'a str,
    is_param: bool
}

#[derive(Debug)]
struct Imp<'a> {
    name: &'a str,
    token: &'a str,
    commands: Vec<Command<'a>>,
}

#[derive(Debug)]
struct Operation<'a> {
    name: &'a str,
    param: Option<&'a str>
}


fn tokenized(code: &str) -> Result<Vec<Operation>, ()> {
    let imps: Vec<Imp> = vec![
        Imp{name: "Stack Manipulation", token: " ", commands: vec![
            Command{name: "Push", token: " ", is_param: true},
            Command{name: "Duplicate", token: "\n ", is_param: false},
            Command{name: "Swap", token: "\n\t", is_param: false},
            Command{name: "Discard", token: "\n\n", is_param: false},
        ]},
        Imp{name: "Arithmetic", token: "\t ", commands: vec![
            Command{name: "Addition", token: "  ", is_param: false},
            Command{name: "Subtraction", token: " \t", is_param: false},
            Command{name: "Multiplication", token: " \n", is_param: false},
            Command{name: "Division", token: "\t ", is_param: false},
            Command{name: "Modulo", token: "\t\t", is_param: false},
        ]},
        Imp{name: "Heap Access", token: "\t\t", commands: vec![
            Command{name: "Store", token: " ", is_param: false},
            Command{name: "Retrieve", token: "\t", is_param: false},
        ]},
        Imp{name: "Flow Control", token: "\n", commands: vec![
            Command{name: "Mark", token: "  ", is_param: true},
            Command{name: "Call", token: " \t", is_param: true},
            Command{name: "Jump", token: " \n", is_param: true},
            Command{name: "JumpZero", token: "\t ", is_param: true},
            Command{name: "JumpNegative", token: "\t\t", is_param: true},
            Command{name: "Return", token: "\t\n", is_param: false},
            Command{name: "End", token: "\n\n", is_param: false},
        ]},
        Imp{name: "I/O", token: "\t\n", commands: vec![
            Command{name: "OutputChar", token: "  ", is_param: false},
            Command{name: "OutputNumber", token: " \t", is_param: false},
            Command{name: "ReadChar", token: "\t ", is_param: false},
            Command{name: "ReadNumber", token: "\t\t", is_param: false},
        ]},
    ];
    let mut ptr: usize = 0;
    let mut operations: Vec<Operation> = vec![];
    loop {
        if code.len() == ptr {
            break;
        }
        match imps.iter().find(|imp| code[ptr..].find(imp.token) == Some(0)) {
            None => {},
            Some(imp) => {
                ptr += imp.token.len();
                match imp.commands.iter().find(|cmd| code[ptr..].find(cmd.token) == Some(0)) {
                    None => {},
                    Some(cmd) => {
                        ptr += cmd.token.len();
                        let param = match cmd.is_param {
                            true => match code[ptr..].find("\n") {
                                None => None,
                                Some(n) => {
                                    ptr += n + 1;
                                    Some(&code[ptr - n - 1..ptr - 1])
                                }
                            },
                            false => None
                        };
                        operations.push(Operation{name: cmd.name, param: param});
                    }
                }
            }
        };
    }
    Ok(operations)
}

fn run(source: &str) {

    let code: String = source.chars().filter(|c| *c == ' ' || *c == '\t' || *c == '\n').collect();
    let operations: Vec<Operation> = tokenized(&code).unwrap();

    let mut stack: Vec<isize> = vec![];
    let mut call_stack: Vec<usize> = vec![];
    let marks: HashMap<&str, usize> = operations.iter()
        .enumerate()
        .filter(|(_, op)| op.name == "Mark")
        .fold(HashMap::new(), |mut acc, (i, op)| {
            acc.insert(op.param.unwrap(), i);
            acc
        });
    let mut ptr: usize = 0;
    loop {
        match operations.get(ptr) {
            None => break,
            Some(op) => {
                match op.name {
                    "Push" => {
                        let int_val = op.param.unwrap().chars().fold(0, |acc, c| {
                            (acc << 1) + if c == ' ' { 0 } else { 1 }
                        });
                        stack.push(int_val);
                    },
                    "Duplicate" => {
                        stack.push(stack.get(stack.len() - 1).unwrap().clone());
                    },
                    "Swap" => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(a);
                        stack.push(b);
                    },
                    "Discard" => {
                        stack.pop();
                    },
                    "Addition" => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b + a);
                    },
                    "Subtraction" => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b - a);
                    },
                    "Multiplication" => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b * a);
                    },
                    "Division" => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b / a);
                    },
                    "Modulo" => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b % a);
                    },
                    "Store" => {
                        //TODO
                    }
                    "Retrieve" => {
                        //TODO
                    }
                    "Call" => {
                        call_stack.push(ptr.clone());
                        ptr = marks.get(op.param.unwrap()).unwrap().clone();
                    },
                    "Jump" => {
                        ptr = marks.get(op.param.unwrap()).unwrap().clone();
                    },
                    "JumpZero" => {
                        if stack.pop().unwrap() == 0 {
                            ptr = marks.get(op.param.unwrap()).unwrap().clone();
                        }
                    },
                    "JumpNegative" => {
                        if stack.pop().unwrap() < 0 {
                            ptr = marks.get(op.param.unwrap()).unwrap().clone();
                        }
                    },
                    "Return" => {
                        ptr = call_stack.pop().unwrap();
                    },
                    "End" => {
                        break;
                    },
                    "OutputChar" => {
                        print!("{}", stack.pop().unwrap() as u8 as char);
                    },
                    "OutputNumber" => {
                        print!("{}", stack.pop().unwrap());
                    },
                    "ReadChar" => {
                        //TODO
                    }
                    "ReadNumber" => {
                        //TODO
                    }
                    _ => {}
                }
            }
        }
        ptr += 1;
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
