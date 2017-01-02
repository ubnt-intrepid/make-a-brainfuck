use engine::Engine;
use tape::Tape;

#[test]
fn test1() {
  let result = Engine::new(Tape::new(0, vec![72, 111, 103, 101, 10, 70, 111, 111, 0]))
    .eval("[.>]", "");
  assert_eq!(result, Ok("Hoge\nFoo".to_owned()));
}

#[test]
fn test2() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
  const SOURCE: &'static str =
       "++>+++++[<+>-]++++++++[<++++++>-]<.";

  let result = Engine::default().eval(SOURCE, "");
  assert_eq!(result, Ok("7".to_owned()));
}

#[test]
fn test3() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
  const SOURCE: &'static str =
       "2+>5+[<+>-]8+[<6+>-]<.";

  let result = Engine::default().eval(SOURCE, "");
  assert_eq!(result, Ok("7".to_owned()));
}

#[test]
fn hello_world() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
  const HELLO_WORLD: &'static str =
      "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.++
       +.------.--------.>>+.>++.";

  let result = Engine::default().eval(HELLO_WORLD, "");
  assert_eq!(result, Ok("Hello World!\n".to_owned()));
}

#[test]
fn hello_world2() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
  const HELLO_WORLD: &'static str =
      "8+[>4+[>2+>3+>3+>+4<-]>+>+>-2>+[<]<-]2>.>3-.7+..3+.2>.<-.<.3+.6-.8-.2>+.>2+.";

  let result = Engine::default().eval(HELLO_WORLD, "");
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

  let result = Engine::default().eval(SOURCE, "");
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

  let result = Engine::default().eval(SOURCE, "");
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

  let result = Engine::default().eval(SOURCE, "");
  assert_eq!(result,
             Ok("2 3 5 7 11 13 17 19 23 29 31 37 41 32 36 42 48 50 56 60 62 68 72 78 86"
               .to_owned()));
}
