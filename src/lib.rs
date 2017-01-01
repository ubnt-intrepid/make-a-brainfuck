#[derive(Debug,PartialEq)]
enum Token {
  AddVal(usize),
  SubVal(usize),
  AddPtr(usize),
  SubPtr(usize),
  PutChar,
  GetChar,
  JumpForward(usize),
  JumpBackward(usize),
  Nop,
}

fn parse_to_int(s: &mut String) -> Result<(usize, usize), String> {
  let cnt = if s != "" {
    s.parse::<usize>().ok().ok_or("failed to parse integer".to_owned())?
  } else {
    1
  };
  let digit = s.len();
  s.clear();
  Ok((cnt, digit))
}

#[test]
fn parse_int_test() {
  assert_eq!(parse_to_int(&mut "".to_owned()), Ok((1, 0)));
  assert_eq!(parse_to_int(&mut "12".to_owned()), Ok((12, 2)));
}

fn parse(s: &str) -> Result<Vec<Token>, String> {
  let inputs: Vec<char> = s.trim().chars().filter(|&c| "><+-.,[]0123456789".contains(c)).collect();

  let mut result = Vec::new();
  let mut buf_count = String::new();
  for (i, &c) in inputs.iter().enumerate() {
    match c {
      c @ '0'...'9' => {
        buf_count.push(c);
      }
      '>' => {
        let (c, d) = parse_to_int(&mut buf_count)?;
        for _ in 0..d {
          result.push(Token::Nop);
        }
        result.push(Token::AddPtr(c));
      }
      '<' => {
        let (c, d) = parse_to_int(&mut buf_count)?;
        for _ in 0..d {
          result.push(Token::Nop);
        }
        result.push(Token::SubPtr(c));
      }
      '+' => {
        let (c, d) = parse_to_int(&mut buf_count)?;
        for _ in 0..d {
          result.push(Token::Nop);
        }
        result.push(Token::AddVal(c));
      }
      '-' => {
        let (c, d) = parse_to_int(&mut buf_count)?;
        for _ in 0..d {
          result.push(Token::Nop);
        }
        result.push(Token::SubVal(c));
      }
      '.' => result.push(Token::PutChar),
      ',' => result.push(Token::GetChar),
      '[' => {
        let mut nest = 1;
        let cursor = ((i + 1)..(inputs.len())).find(|&j| {
            match inputs[j as usize] {
              '[' => {
                nest += 1;
                false
              }
              ']' => {
                nest -= 1;
                nest == 0
              }
              _ => false,
            }
          })
          .ok_or("correspond ']' is missing.".to_owned())?;
        result.push(Token::JumpForward(cursor));
      }
      ']' => {
        let mut nest = 1;
        let cursor = (0..i).rev()
          .find(|&j| {
            match inputs[j as usize] {
              '[' => {
                nest -= 1;
                nest == 0
              }
              ']' => {
                nest += 1;
                false
              }
              _ => false,
            }
          })
          .ok_or("correspond '[' is missing.".to_owned())?;
        result.push(Token::JumpBackward(cursor));
      }
      _ => unreachable!(),
    }
  }

  Ok(result)
}

#[test]
fn case1() {
  let s = "[>,.2<]";
  let tokens = parse(s);
  assert_eq!(tokens,
             Ok(vec![Token::JumpForward(6),
                     Token::AddPtr(1),
                     Token::GetChar,
                     Token::PutChar,
                     Token::Nop,
                     Token::SubPtr(2),
                     Token::JumpBackward(0)]));
}


#[derive(Debug)]
pub struct Engine {
  pointer: isize,
  buffer: Vec<u8>,
}

impl Default for Engine {
  fn default() -> Engine {
    Engine::new(0, vec![0u8; 8096])
  }
}

impl Engine {
  pub fn new(pointer: isize, buffer: Vec<u8>) -> Engine {
    Engine {
      pointer: pointer,
      buffer: buffer,
    }
  }

  pub fn eval(&mut self, s: &str, stdin: &str) -> Result<String, String> {
    let mut stdin = stdin.chars();
    let mut stdout = String::new();

    let tokens = parse(s)?;

    let mut cursor: isize = 0;
    while let Some(token) = tokens.get(cursor as usize) {
      match *token {
        Token::AddPtr(n) => {
          self.pointer += n as isize;
          cursor += 1;
        }
        Token::SubPtr(n) => {
          self.pointer -= n as isize;
          cursor += 1;
        }
        Token::AddVal(n) => {
          safe_add(&mut self.buffer[self.pointer as usize], n);
          cursor += 1;
        }
        Token::SubVal(n) => {
          safe_sub(&mut self.buffer[self.pointer as usize], n);
          cursor += 1;
        }
        Token::PutChar => {
          stdout.push(self.buffer[self.pointer as usize] as char);
          cursor += 1;
        }
        Token::GetChar => {
          let c = stdin.next().ok_or("empty stdin".to_owned())?;
          self.buffer[self.pointer as usize] = c as u8;
          cursor += 1;
        }
        Token::JumpForward(c) => {
          if self.buffer[self.pointer as usize] == 0 {
            cursor = c as isize;
          } else {
            cursor += 1;
          }
        }
        Token::JumpBackward(c) => {
          if self.buffer[self.pointer as usize] != 0 {
            cursor = c as isize;
          } else {
            cursor += 1;
          }
        }
        Token::Nop => cursor += 1,
      }
    }
    Ok(stdout)
  }
}


fn safe_add(val: &mut u8, n: usize) {
  for _ in 0..n {
    if *val == 255 { *val = 0 } else { *val += 1 }
  }
}

fn safe_sub(val: &mut u8, n: usize) {
  for _ in 0..n {
    if *val == 0 { *val = 255 } else { *val -= 1 }
  }
}


#[cfg(test)]
mod tests {
  use super::Engine;

  #[test]
  fn test1() {
    let result = Engine::new(0, vec![72, 111, 103, 101, 10, 70, 111, 111, 0]).eval("[.>]", "");
    assert_eq!(result, Ok("Hoge\nFoo".to_owned()));
  }

  #[test]
  fn test2() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    const SOURCE: &'static str =
       "++>+++++[<+>-]++++++++[<++++++>-]<.";

    let result = Engine::new(0, vec![0; 8096]).eval(SOURCE, "");
    assert_eq!(result, Ok("7".to_owned()));
  }

  #[test]
  fn test3() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    const SOURCE: &'static str =
       "2+>5+[<+>-]8+[<6+>-]<.";

    let result = Engine::new(0, vec![0; 8096]).eval(SOURCE, "");
    assert_eq!(result, Ok("7".to_owned()));
  }

  #[test]
  fn hello_world() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    const HELLO_WORLD: &'static str =
      "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.++
       +.------.--------.>>+.>++.";

    let result = Engine::new(0, vec![0; 8096]).eval(HELLO_WORLD, "");
    assert_eq!(result, Ok("Hello World!\n".to_owned()));
  }

  #[test]
  fn hello_world2() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    const HELLO_WORLD: &'static str =
      "8+[>4+[>2+>3+>3+>+4<-]>+>+>-2>+[<]<-]2>.>3-.7+..3+.2>.<-.<.3+.6-.8-.2>+.>2+.";

    let result = Engine::new(0, vec![0; 8096]).eval(HELLO_WORLD, "");
    assert_eq!(result, Ok("Hello World!\n".to_owned()));
  }

  #[test]
  fn fizz_buzz() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    const SOURCE:&'static str =
      "++++++[->++++>>+>+>-<<<<<]>[<++++>>+++>++++>>+++>+++++>+++++>>>>>>++>>++<<<<<<<<
       <<<<<<-]<++++>+++>-->+++>->>--->++>>>+++++[->++>++<<]<<<<<<<<<<[->-[>>>>>>>]>[<+
       ++>.>.>>>>..>>>+<]<<<<<-[>>>>]>[<+++++>.>.>..>>>+<]>>>>+<-[<<<]<[[-<<+>>]>>>+>+<
       <<<<<[->>+>+>-<<<<]<]>>[[-]<]>[>>>[>.<<.<<<]<[.<<<<]>]>.<<<<<<<<<<<]";

    let result = Engine::new(0, vec![0; 8096]).eval(SOURCE, "");
    assert!(result.is_ok());

    for (i, r) in result.unwrap().split("\n").filter(|&s| s != "").enumerate() {
      if (i + 1) % 15 == 0 {
        assert_eq!(r, "FizzBuzz");
      } else if (i + 1) % 3 == 0 {
        assert_eq!(r, "Fizz");
      } else if (i + 1) % 5 == 0 {
        assert_eq!(r, "Buzz");
      } else {
        assert_eq!(r, format!("{}", i + 1));
      }
    }
  }

  #[test]
  fn fizz_buzz2() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    const SOURCE:&'static str =
      "6+[->4+2>+>+>-5<]>[<4+2>3+>4+>>3+>5+>5+6>2+2>2+14<-]
       <4+>3+>2->3+>-2>3->2+3>5+[->2+>2+2<]10<[->-[7>]>[<3+
       >.>.4>..3>+<]5<-[4>]>[<5+>.>.>..3>+<]4>+<-[3<]<[[-2<
       +2>]3>+>+6<[-2>+>+>-4<]<]2>[[-]<]>[3>[>.2<.3<]<[.4<]
       >]>.11<]";

    let result = Engine::new(0, vec![0; 8096]).eval(SOURCE, "");
    assert!(result.is_ok());

    for (i, r) in result.unwrap().split("\n").filter(|&s| s != "").enumerate() {
      if (i + 1) % 15 == 0 {
        assert_eq!(r, "FizzBuzz");
      } else if (i + 1) % 3 == 0 {
        assert_eq!(r, "Fizz");
      } else if (i + 1) % 5 == 0 {
        assert_eq!(r, "Buzz");
      } else {
        assert_eq!(r, format!("{}", i + 1));
      }
    }
  }

  #[test]
  fn primes() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    const SOURCE:&'static str =
      ">++++[<++++++++>-]>++++++++[<++++++>-]<++.<.>+.<.>++.<.>++.<.>>++[<--->-]<..<.>.
       ++.<.>--.>++[<+++>-]<.<.>>++[<--->-]<.>++[<++++>-]<.<.>>++[<--->-]<-.+.<.>-.>+++
       [<++>-]<+.<.>>++[<--->-]<.--.<.>++.++++.<.>---.---.<.>++.-.<.>+.+++.<.>--.--.<.>
       ++.++++.<.>---.-----.<.>+++++.+.<.>.>++[<--->-]<.<.>>++[<+++>-]<.----.<.>++++.++
       .<.>-.-----.<.>+++++.+.<.>.--.";

    let result = Engine::new(0, vec![0; 8096]).eval(SOURCE, "");
    assert_eq!(result,
               Ok("2 3 5 7 11 13 17 19 23 29 31 37 41 32 36 42 48 50 56 60 62 68 72 78 86"
                 .to_owned()));
  }
}
