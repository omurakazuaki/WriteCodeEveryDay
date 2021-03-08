
pub struct Optimizer {
  tokens: Vec<char>,
  continuos: Vec<isize>
}

impl Optimizer {
    pub fn new(tokens: Vec<char>) -> Self {
      Optimizer {tokens: tokens, continuos: vec![] }
    }

    pub fn delete_comments(&mut self) {
        self.tokens = self.tokens.clone().into_iter().filter(|c|"[]><+-.,".find(*c).is_some()).collect();
    }

    pub fn optimize_set_zero(&mut self) {
        let tokens = self.tokens.clone();
        let mut vec: Vec<char> = vec![];
        let mut ptr: usize = 0;
        loop {
            match tokens.get(ptr) {
                None => break,
                Some(']') => {
                    let p1 = tokens.get(ptr-1).unwrap();
                    let p2 = tokens.get(ptr-2).unwrap();
                    if (*p1 == '-' || *p1 == '+') && *p2 == '[' {
                        vec.pop();
                        vec.pop();
                        vec.push('0');
                    } else {
                        vec.push(']');
                    }
                },
                Some(t) => {
                    vec.push(*t);
                }
            }
            ptr += 1;
        }
        self.tokens = vec;
    }

    pub fn optimize_continuos(&mut self) {
        let mut chars: Vec<char> = vec![];
        let mut continuos: Vec<isize> = vec![];
        let mut ptr: usize = 0;
        loop {
            match self.tokens.get(ptr) {
                None => break,
                Some(t) => {
                    match chars.get(chars.len() - 1) {
                        None => {
                            chars.push(*t);
                            continuos.push(1);
                        },
                        Some(c) => {
                            if *c == *t && *c != '[' && *c != ']' && *c != '.' && *c != ',' {
                                let cont = continuos.pop().unwrap() + 1;
                                continuos.push(cont);
                            } else {
                                chars.push(*t);
                                continuos.push(1);
                            }
                        }
                    }
                }
            }
            ptr += 1;
        }
        self.tokens = chars;
        self.continuos = continuos;
    }

    pub fn build(self) -> (Vec<char>, Vec<isize>) {
      (self.tokens, self.continuos)
    }

    pub fn jump_points(&mut self) -> Vec<[usize;2]> {
        let mut vec: Vec<[usize;2]> = vec![];
        let mut ptr: usize = 0;
        let mut start: usize = 0;
        let mut nest_count = 0;
        loop {
            match self.tokens.get(ptr) {
                None => break,
                Some('[') => {
                    if nest_count == 0 {
                        start = ptr;
                    }
                    nest_count += 1;
                },
                Some(']') => {
                    if start != 0 {
                        nest_count -= 1;
                        if nest_count == 0 {
                            vec.push([start, ptr]);
                            ptr = start;
                            start = 0;
                        }
                    }
                },
                Some(_) => {
                }
            }
            ptr += 1;
        }
        vec
    }

}
