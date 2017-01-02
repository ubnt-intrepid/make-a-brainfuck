#[derive(Debug)]
pub struct Tape {
  pointer: usize,
  buffer: Vec<u8>,
}

impl Tape {
  pub fn new(pointer: usize, buffer: Vec<u8>) -> Tape {
    Tape {
      pointer: pointer,
      buffer: buffer,
    }
  }

  pub fn add_ptr(&mut self, n: usize) {
    self.pointer = self.pointer.wrapping_add(n);
  }

  pub fn sub_ptr(&mut self, n: usize) {
    self.pointer = self.pointer.wrapping_sub(n);
  }

  pub fn add_val(&mut self, n: usize) {
    safe_add(&mut self.buffer[self.pointer], n);
  }

  pub fn sub_val(&mut self, n: usize) {
    safe_sub(&mut self.buffer[self.pointer], n);
  }

  pub fn get(&self) -> u8 {
    self.buffer[self.pointer]
  }

  pub fn get_mut(&mut self) -> &mut u8 {
    &mut self.buffer[self.pointer]
  }
}

impl Default for Tape {
  fn default() -> Tape {
    Tape::new(0, vec![0u8; 8096])
  }
}

fn safe_add(val: &mut u8, n: usize) {
  *val = (*val as usize).wrapping_add(n) as u8;
}

fn safe_sub(val: &mut u8, n: usize) {
  *val = (*val as usize).wrapping_sub(n) as u8;
}
