
const MEMORY_SIZE: usize = 30000;

pub struct Memory {
  raw: Vec<i8>,
  ptr: usize
}

impl Memory {
  pub fn new() -> Self {
    Self {raw: vec![0; MEMORY_SIZE], ptr: 0}
  }

  pub fn clr(&mut self) {
    self.put(0);
  }

  pub fn inc(&mut self) {
    if self.get() < i8::MAX {
        self.put(self.get() + 1);
    } else {
        self.put(i8::MIN);
    }
  }

  pub fn dec(&mut self) {
    if self.get() > i8::MIN {
        self.put(self.get() - 1);
    } else {
        self.put(i8::MAX);
    }
  }

  pub fn add(&mut self, val: i8) {
    let mut new_val = self.get() + val;
    if new_val > i8::MAX {
      new_val = i8::MIN + new_val - i8::MAX;
    } else if new_val < i8::MIN {
      new_val = i8::MAX + new_val - i8::MIN;
    }
    self.put(new_val);
  }

  pub fn nxt(&mut self) {
    if self.ptr < MEMORY_SIZE - 1 {
        self.ptr += 1;
    } else {
        self.ptr = 0;
    }
  }

  pub fn prv(&mut self) {
    if self.ptr > 0 {
        self.ptr -= 1;
    } else {
        self.ptr = MEMORY_SIZE - 1;
    }
  }

  pub fn mov(&mut self, val: isize) {
    let mut new_val = add(self.ptr, val);
    if new_val > (MEMORY_SIZE - 1) as isize {
      new_val = new_val - MEMORY_SIZE  as isize;
    } else if new_val < 0 {
      new_val = MEMORY_SIZE as isize + new_val;
    }
    self.ptr = new_val as usize;
  }

  pub fn get(&self) -> i8 {
    self.raw[self.ptr]
  }

  pub fn put(&mut self, value: i8) {
    self.raw[self.ptr] = value;
  }

}

fn add(u: usize, i: isize) -> isize {
    if i.is_negative() {
        u as isize - i.wrapping_abs() as isize
    } else {
        u as isize + i as isize
    }
}
