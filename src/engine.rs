use std::str::Chars;
use parser;
use tape::Tape;

#[derive(Debug,Default)]
pub struct Engine {
  tape: Tape,
}

impl Engine {
  pub fn new(tape: Tape) -> Engine {
    Engine { tape: tape }
  }

  pub fn eval(&mut self, s: &str, stdin: &str) -> Result<String, String> {
    let mut stdin = stdin.chars();
    let mut stdout = String::new();
    let tokens = parser::parse(s)?;

    self.eval_lines(&tokens, &mut stdin, &mut stdout)?;

    Ok(stdout)
  }

  fn eval_lines(&mut self,
                tokens: &[parser::Ast],
                stdin: &mut Chars,
                stdout: &mut String)
                -> Result<(), String> {
    for token in tokens {
      self.eval_token(token, stdin, stdout)?;
    }
    Ok(())
  }

  fn eval_token(&mut self,
                token: &parser::Ast,
                stdin: &mut Chars,
                stdout: &mut String)
                -> Result<(), String> {
    use parser::Ast;
    match *token {
      Ast::AddPtr(n) => self.tape.add_ptr(n),
      Ast::SubPtr(n) => self.tape.sub_ptr(n),
      Ast::AddVal(n) => self.tape.add_val(n),
      Ast::SubVal(n) => self.tape.sub_val(n),
      Ast::PutChar => stdout.push(self.tape.get() as char),
      Ast::GetChar => {
        let c = stdin.next().ok_or("empty stdin".to_owned())?;
        *self.tape.get_mut() = c as u8;
      }
      Ast::Loop(ref ast) => {
        while self.tape.get() != 0 {
          self.eval_lines(&ast, stdin, stdout)?;
        }
      }
    }
    Ok(())
  }
}
