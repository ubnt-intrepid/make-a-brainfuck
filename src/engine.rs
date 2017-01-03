use std::io::Cursor;
use std::io::Read;
use parser;
use tape::Tape;

#[derive(Debug,Default)]
pub struct Engine<'i, 'o> {
  tape: Tape,
  stdin: Option<Cursor<&'i [u8]>>,
  stdout: Option<&'o mut Vec<u8>>,
}

impl<'i, 'o> Engine<'i, 'o> {
  pub fn new(tape: Tape) -> Self {
    Engine {
      tape: tape,
      stdin: None,
      stdout: None,
    }
  }

  pub fn stdin(mut self, stdin: &'i [u8]) -> Self {
    self.stdin = Some(Cursor::new(stdin));
    self
  }

  pub fn stdout(mut self, stdout: &'o mut Vec<u8>) -> Self {
    self.stdout = Some(stdout);
    self
  }

  pub fn eval(&mut self, s: &str) -> Result<(), String> {
    self.eval_lines(&parser::parse(s)?)
  }

  fn eval_lines(&mut self, tokens: &[parser::Ast]) -> Result<(), String> {
    for token in tokens {
      self.eval_token(token)?;
    }
    Ok(())
  }

  fn eval_token(&mut self, token: &parser::Ast) -> Result<(), String> {
    use parser::Ast;
    match *token {
      Ast::AddPtr(n) => Ok(self.tape.add_ptr(n)),
      Ast::AddVal(n) => Ok(self.tape.add_val(n)),
      Ast::PutChar => {
        let c = self.tape.get_char();
        self.put_char(c)?;
        Ok(())
      }
      Ast::GetChar => {
        let c = self.get_char()?;
        self.tape.put_char(c);
        Ok(())
      }
      Ast::Loop(ref ast) => {
        while self.tape.get_char() != 0 {
          self.eval_lines(&ast)?;
        }
        Ok(())
      }
    }
  }

  fn put_char(&mut self, c: u8) -> Result<(), String> {
    if let Some(ref mut stdout) = self.stdout {
      stdout.push(c)
    }
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
