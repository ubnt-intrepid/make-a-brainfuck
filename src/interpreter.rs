use std::io::Cursor;
use std::io::Read;
use parser::{Ast, parse};


#[derive(Debug)]
struct Tape {
  pointer: usize,
  buffer: Vec<u8>,
}

impl Default for Tape {
  fn default() -> Tape {
    Tape::new(0, vec![0u8; 8096])
  }
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
    let end = {
      let beg = self.buffer.iter().position(|&s| s != 0);
      if let Some(beg) = beg {
        let end = self.buffer.iter().skip(beg).position(|&s| s == 0);
        end.map(|end| beg + end).unwrap_or(self.buffer.len())
      } else {
        0
      }
    };

    println!("[coredump] ptr = {}, buffer = {:?}",
             self.pointer,
             &self.buffer[0..end]);
  }
}


#[derive(Debug, Default)]
pub struct Interpreter<'i> {
  tape: Tape,
  stdin: Option<Cursor<&'i [u8]>>,
  stdout: Vec<u8>,
}

impl<'i> Interpreter<'i> {
  pub fn new(pointer: usize, buffer: Vec<u8>) -> Self {
    Interpreter {
      tape: Tape::new(pointer, buffer),
      stdin: None,
      stdout: Vec::new(),
    }
  }

  pub fn stdin(mut self, stdin: &'i [u8]) -> Self {
    self.stdin = Some(Cursor::new(stdin));
    self
  }

  pub fn stdout(&self) -> &[u8] {
    self.stdout.as_slice()
  }

  pub fn eval(&mut self, s: &str) -> Result<(), String> {
    self.eval_lines(&parse(s)?)
  }

  fn eval_lines(&mut self, tokens: &[Ast]) -> Result<(), String> {
    for token in tokens {
      self.eval_token(token)?;
    }
    Ok(())
  }

  fn eval_token(&mut self, token: &Ast) -> Result<(), String> {
    match *token {
      Ast::AddPtr(n) => {
        self.tape.add_ptr(n);
      }
      Ast::AddVal(n) => {
        self.tape.add_val(n);
      }
      Ast::PutChar => {
        let c = self.tape.get_char();
        self.put_char(c)?;
      }
      Ast::GetChar => {
        let c = self.get_char()?;
        self.tape.put_char(c);
      }
      Ast::CoreDump => {
        self.tape.coredump();
      }
      Ast::Loop(ref ast) => {
        while self.tape.get_char() != 0 {
          self.eval_lines(&ast)?;
        }
      }
    }
    Ok(())
  }

  fn put_char(&mut self, c: u8) -> Result<(), String> {
    self.stdout.push(c);
    Ok(())
  }

  fn get_char(&mut self) -> Result<u8, String> {
    if let Some(ref mut stdin) = self.stdin {
      let mut buf = [0u8];
      stdin.read_exact(&mut buf).map_err(|e| e.to_string())?;
      Ok(buf[0])
    } else {
      Err("empty stdin".to_owned())
    }
  }
}
