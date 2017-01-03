#[derive(Debug, PartialEq)]
pub enum Ast {
  AddVal(isize),
  AddPtr(isize),
  PutChar,
  GetChar,
  CoreDump,
  Loop(Vec<Ast>),
}

#[derive(Debug,PartialEq)]
enum Token {
  SymWithOffset(char, isize),
  Symbol(char),
}

fn tokenize(s: &str) -> Result<Vec<Token>, String> {
  let inputs = s.split("\n")
    .map(|line| if let Some(pos) = line.find('#') {
      &line[0..pos]
    } else {
      line
    })
    .flat_map(|s| s.chars());

  let mut result = Vec::new();
  let mut buf_count = String::new();
  for c in inputs {
    match c {
      '0'...'9' => buf_count.push(c),
      '>' | '<' | '+' | '-' => {
        let n = if buf_count != "" {
          buf_count.parse::<isize>().map_err(|_| "failed to parse integer".to_owned())?
        } else {
          1
        };
        buf_count.clear();
        result.push(Token::SymWithOffset(c, n))
      }
      '.' | ',' | '[' | ']' | '?' => result.push(Token::Symbol(c)),
      _ => (),
    }
  }

  Ok(result)
}


fn build_ast(tokens: &[Token]) -> Result<Vec<Ast>, String> {
  let mut result = Vec::new();
  let mut index = 0;
  while let Some(t) = tokens.get(index) {
    match *t {
      Token::SymWithOffset('+', n) => result.push(Ast::AddVal(n)),
      Token::SymWithOffset('-', n) => result.push(Ast::AddVal(-n)),
      Token::SymWithOffset('>', n) => result.push(Ast::AddPtr(n)),
      Token::SymWithOffset('<', n) => result.push(Ast::AddPtr(-n)),
      Token::Symbol('.') => result.push(Ast::PutChar),
      Token::Symbol(',') => result.push(Ast::GetChar),
      Token::Symbol('?') => result.push(Ast::CoreDump),
      Token::Symbol('[') => {
        let mut nest = 1;
        let cursor = ((index + 1)..(tokens.len())).find(|&j| {
            match tokens[j] {
              Token::Symbol('[') => {
                nest += 1;
                false
              }
              Token::Symbol(']') => {
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
      Token::Symbol(']') => return Err("unexpected ']' is found".to_owned()),
      _ => unreachable!(),
    }
    index += 1;
  }
  Ok(result)
}


pub fn parse(s: &str) -> Result<Vec<Ast>, String> {
  build_ast(&tokenize(s)?)
}


#[test]
fn test_tokenize() {
  let inputs = r#"[>,.2<] # hoge
    # a.b.c
    "#;
  let tokens = tokenize(inputs);
  assert_eq!(tokens,
             Ok(vec![Token::Symbol('['),
                     Token::SymWithOffset('>', 1),
                     Token::Symbol(','),
                     Token::Symbol('.'),
                     Token::SymWithOffset('<', 2),
                     Token::Symbol(']')]));
}
