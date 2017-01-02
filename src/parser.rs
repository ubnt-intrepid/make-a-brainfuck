mod phase1 {
  #[derive(Debug,PartialEq)]
  pub enum Token {
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

  pub fn parse(inputs: Vec<char>) -> Result<Vec<Token>, String> {
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
  fn parse_case1() {
    let inputs = "[>,.2<]".chars().collect();
    let tokens = parse(inputs);
    assert_eq!(tokens,
               Ok(vec![Token::JumpForward(6),
                       Token::AddPtr(1),
                       Token::GetChar,
                       Token::PutChar,
                       Token::Nop,
                       Token::SubPtr(2),
                       Token::JumpBackward(0)]));
  }
}

mod phase2 {
  pub use super::phase1::Token;

  pub fn parse(inputs: Vec<Token>) -> Result<Vec<Token>, String> {
    Ok(inputs)
  }
}

pub use self::phase2::Token;

pub fn parse(s: &str) -> Result<Vec<phase2::Token>, String> {
  let inputs: Vec<char> = s.trim().chars().filter(|&c| "><+-.,[]0123456789".contains(c)).collect();
  phase2::parse(phase1::parse(inputs)?)
}
