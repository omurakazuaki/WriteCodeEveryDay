
const MEMORY_SIZE: usize = 1024;

pub struct Memory {
  raw: Vec<i8>,
  ptr: usize
}

impl Memory {
  pub fn new() -> Memory {
    Memory {raw: vec![0; MEMORY_SIZE], ptr: 0}
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

  pub fn get(&self) -> i8 {
    self.raw[self.ptr]
  }

  pub fn put(&mut self, value: i8) {
    self.raw[self.ptr] = value;
  }

}
