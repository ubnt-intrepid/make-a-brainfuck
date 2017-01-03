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

  pub fn add_ptr(&mut self, n: isize) {
    self.pointer = (self.pointer as isize).wrapping_add(n) as usize;
  }

  pub fn add_val(&mut self, n: isize) {
    self.buffer[self.pointer] = (self.buffer[self.pointer] as isize).wrapping_add(n) as u8;
  }

  pub fn get_char(&self) -> u8 {
    self.buffer[self.pointer]
  }

  pub fn put_char(&mut self, c: u8) {
    self.buffer[self.pointer] = c;
  }

  pub fn coredump(&self) {
    let view: &[u8] = self.buffer
      .iter()
      .position(|&s| s != 0)
      .map(|beg| {
        let end = self.buffer
          .iter()
          .skip(beg)
          .position(|&s| s == 0)
          .map(|end| beg + end)
          .unwrap_or(self.buffer.len());
        &self.buffer[0..end]
      })
      .unwrap_or(&[]);

    println!(r#"[coredump]
  ptr = {}
  buffer = {:?}
"#,
             self.pointer,
             view);
  }
}

impl Default for Tape {
  fn default() -> Tape {
    Tape::new(0, vec![0u8; 8096])
  }
}
