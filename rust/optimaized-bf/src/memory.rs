
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
    self.put(self.get() as isize + 1);
  }

  pub fn dec(&mut self) {
    self.put(self.get() as isize - 1);
  }

  pub fn add(&mut self, val: i8) {
    self.put(self.get() as isize + val as isize);
  }

  pub fn mvv(&mut self, val: isize) {
    self.raw[add_ptr(self.ptr, val)] += self.raw[self.ptr];
    self.raw[self.ptr] = 0;
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
    self.ptr = add_ptr(self.ptr, val);
  }

  pub fn get(&self) -> i8 {
    self.raw[self.ptr]
  }

  pub fn put(&mut self, value: isize) {
    let mut wrapped_value = value;
    if wrapped_value > i8::MAX as isize {
      wrapped_value = i8::MIN as isize + wrapped_value - i8::MAX as isize;
    } else if wrapped_value < i8::MIN as isize {
      wrapped_value = i8::MAX as isize + wrapped_value - i8::MIN as isize;
    }
    self.raw[self.ptr] = wrapped_value as i8;
  }

}

fn add(u: usize, i: isize) -> isize {
    if i.is_negative() {
        u as isize - i.wrapping_abs() as isize
    } else {
        u as isize + i as isize
    }
}

fn add_ptr(ptr: usize, val: isize) -> usize {
  let mut new_val = add(ptr, val);
  if new_val > (MEMORY_SIZE - 1) as isize {
    new_val = new_val - MEMORY_SIZE  as isize;
  } else if new_val < 0 {
    new_val = MEMORY_SIZE as isize + new_val;
  }
  new_val as usize
}
