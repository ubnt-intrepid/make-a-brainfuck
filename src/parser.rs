mod phase1 {
  use regex::Regex;

  #[derive(Debug,PartialEq)]
  pub enum Token {
    AddVal(isize),
    AddPtr(isize),
    PutChar,
    GetChar,
    JumpForward,
    JumpBackward,
    CoreDump,
  }

  fn parse_to_int(s: &mut String) -> Result<isize, String> {
    let cnt = if s != "" {
      s.parse::<isize>().ok().ok_or("failed to parse integer".to_owned())?
    } else {
      1
    };
    s.clear();
    Ok(cnt)
  }

  #[test]
  fn parse_int_test() {
    assert_eq!(parse_to_int(&mut "".to_owned()), Ok(1));
    assert_eq!(parse_to_int(&mut "12".to_owned()), Ok(12));
  }

  pub fn tokenize(s: &str) -> Result<Vec<Token>, String> {
    let mut result = Vec::new();
    let mut buf_count = String::new();

    let remove_comments = Regex::new(r"#(?:.*)$").unwrap();
    let inputs = s.trim()
      .split("\n")
      .map(|line| remove_comments.replace(line, ""))
      .flat_map(|s| s.chars().collect::<Vec<_>>())
      .filter(|&c| "><+-.,[]?0123456789".contains(c));

    for c in inputs {
      match c {
        c @ '0'...'9' => buf_count.push(c),
        '>' => result.push(Token::AddPtr(parse_to_int(&mut buf_count)?)),
        '<' => result.push(Token::AddPtr(-parse_to_int(&mut buf_count)?)),
        '+' => result.push(Token::AddVal(parse_to_int(&mut buf_count)?)),
        '-' => result.push(Token::AddVal(-parse_to_int(&mut buf_count)?)),
        '.' => result.push(Token::PutChar),
        ',' => result.push(Token::GetChar),
        '[' => result.push(Token::JumpForward),
        ']' => result.push(Token::JumpBackward),
        '?' => result.push(Token::CoreDump),
        _ => unreachable!(),
      }
    }

    Ok(result)
  }

  #[test]
  fn case1() {
    let inputs = r#"[>,.2<] # hoge
    # a.b.c
    "#;
    let tokens = tokenize(inputs);
    assert_eq!(tokens,
               Ok(vec![Token::JumpForward,
                       Token::AddPtr(1),
                       Token::GetChar,
                       Token::PutChar,
                       Token::AddPtr(-2),
                       Token::JumpBackward]));
  }
}

mod phase2 {
  use super::phase1::Token;

  #[derive(Debug,PartialEq)]
  pub enum Ast {
    AddVal(isize),
    AddPtr(isize),
    PutChar,
    GetChar,
    CoreDump,
    Loop(Vec<Ast>),
  }

  pub fn build_ast(tokens: &[Token]) -> Result<Vec<Ast>, String> {
    let mut result = Vec::new();
    let mut index = 0;
    while let Some(ref t) = tokens.get(index) {
      match **t {
        Token::AddVal(n) => result.push(Ast::AddVal(n)),
        Token::AddPtr(n) => result.push(Ast::AddPtr(n)),
        Token::PutChar => result.push(Ast::PutChar),
        Token::GetChar => result.push(Ast::GetChar),
        Token::CoreDump => result.push(Ast::CoreDump),
        Token::JumpForward => {
          let mut nest = 1;
          let cursor = ((index + 1)..(tokens.len())).find(|&j| {
              match tokens[j] {
                Token::JumpForward => {
                  nest += 1;
                  false
                }
                Token::JumpBackward => {
                  nest -= 1;
                  nest == 0
                }
                _ => false,
              }
            })
            .ok_or("nest error".to_owned())?;
          result.push(Ast::Loop(build_ast(&tokens[(index + 1)..cursor])?));
          index = cursor + 1;
          continue;
        }
        Token::JumpBackward => return Err("unexpected ']' is found".to_owned()),
      }
      index += 1;
    }
    Ok(result)
  }
}

pub use self::phase2::Ast;

pub fn parse(s: &str) -> Result<Vec<phase2::Ast>, String> {
  phase2::build_ast(&phase1::tokenize(s)?)
}
