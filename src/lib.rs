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
    let mut stdout = String::new();

    let mut stdin = stdin.chars();

    let input: Vec<char> = s.chars().filter(|&c| c != '\t' && c != '\n' && c != ' ').collect();
    let mut cursor: isize = 0;
    let mut buffer = String::new();
    loop {
      match input[cursor as usize] {
        c @ '0'...'9' => {
          buffer.push(c);
        }
        '>' => {
          let cnt = if buffer != "" {
            buffer.parse::<usize>().ok().ok_or("failed to parse integer".to_owned())?
          } else {
            1
          };
          buffer.clear();

          self.pointer += cnt as isize
        }
        '<' => {
          let cnt = if buffer != "" {
            buffer.parse::<usize>().ok().ok_or("failed to parse integer".to_owned())?
          } else {
            1
          };
          buffer.clear();

          self.pointer -= cnt as isize
        }
        '+' => {
          let cnt = if buffer != "" {
            buffer.parse::<usize>().ok().ok_or("failed to parse integer".to_owned())?
          } else {
            1
          };
          buffer.clear();

          for _ in 0..cnt {
            safe_inc(&mut self.buffer[self.pointer as usize])
          }
        }
        '-' => {
          let cnt = if buffer != "" {
            buffer.parse::<usize>().ok().ok_or("failed to parse integer".to_owned())?
          } else {
            1
          };
          buffer.clear();

          for _ in 0..cnt {
            safe_dec(&mut self.buffer[self.pointer as usize])
          }
        }
        '.' => stdout.push(self.buffer[self.pointer as usize] as char),
        ',' => {
          let c = stdin.next().ok_or("empty stdin".to_owned())?;
          self.buffer[self.pointer as usize] = c as u8;
        }
        '[' => {
          if self.buffer[self.pointer as usize] == 0 {
            let mut nest = 1;
            cursor = ((cursor + 1)..(input.len() as isize)).find(|&j| {
                match input[j as usize] {
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
            continue;
          }
        }
        ']' => {
          if self.buffer[self.pointer as usize] != 0 {
            let mut nest = 1;
            cursor = (0..(cursor as isize)).rev()
              .find(|&j| {
                match input[j as usize] {
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
            continue;
          }
        }
        _ => (),
      }
      cursor += 1;
      if cursor == input.len() as isize {
        break;
      }
    }
    Ok(stdout)
  }
}


fn safe_inc(val: &mut u8) {
  if *val == 255 { *val = 0 } else { *val += 1 }
}

fn safe_dec(val: &mut u8) {
  if *val == 0 { *val = 255 } else { *val -= 1 }
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
