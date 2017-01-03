use interpreter::Interpreter;

#[test]
fn test1() {
  let mut stdout = Vec::new();
  assert!(Interpreter::new(0, vec![72, 111, 103, 101, 10, 70, 111, 111, 0])
    .stdout(&mut stdout)
    .eval("[.>]")
    .is_ok());
  assert_eq!(stdout, "Hoge\nFoo".as_bytes());
}

#[test]
fn test2() {
  #[cfg_attr(rustfmt, rustfmt_skip)]
  const SOURCE: &'static str = r#"
  ++            # v[0] += 2
  >+++++        # v[1] += 5
                #
                # (v[ptr] += v[ptr+1], v[ptr+1]=0, ptr++)
  [             # while v[1] != 0 {
    <+          #   v[0] += 1
    >-          #   v[1] -= 1
  ]             # }
                #
  ++++++++      # v[1] += 8
                #
                # (v[ptr] += 6 * v[ptr+1] : ptr=0)
  [             # while v[1] != 0 {
    <++++++     #   v[0] += 6
    >-          #   v[1] -= 1
  ]             # }
                #
  <.            # putchar(v[0])
  "#;

  let mut stdout = Vec::new();
  assert!(Interpreter::default().stdout(&mut stdout).eval(SOURCE).is_ok());
  assert_eq!(stdout, "7".as_bytes());
}

#[test]
fn test3() {
  #[cfg_attr(rustfmt, rustfmt_skip)]
  const SOURCE: &'static str = r#"
  2+        # v[0] += 2;
  >5+       # v[1] += 5;
  [<+>-]<   # v[0] += v[1]; v[1] = 0;
  >8+       # v[1] += 8;
  [<6+>-]<  # v[0] += 6*v[1]; v[1] = 0;
  .         # put(v[0]);
  "#;

  let mut stdout = Vec::new();
  assert!(Interpreter::default().stdout(&mut stdout).eval(SOURCE).is_ok());
  assert_eq!(stdout, "7".as_bytes());
}

#[test]
fn hello_world() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
  const SOURCE: &'static str =
      "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.++
       +.------.--------.>>+.>++.";

  let mut stdout = Vec::new();
  assert!(Interpreter::default().stdout(&mut stdout).eval(SOURCE).is_ok());
  assert_eq!(stdout, "Hello World!\n".as_bytes());
}

#[test]
fn hello_world2() {
  #[cfg_attr(rustfmt, rustfmt_skip)]
  const SOURCE: &'static str = r#"
  8+                    # v[0] += 8;
  [                     # while v[0] != 0 {
    >                   #   ptr = 1;
    4+[>2+>3+>3+>+4<-]  #   v[2,3,4,5] += 4*[2,3,3,1]; v[1] = 0;
    >+                  #   v[2] += 1;
    >+                  #   v[3] += 1;
    >-                  #   v[4] -= 1;
    2>+                 #   v[6] += 1;
    [<]<-               #   ptr = 0; v[0] -= 1;
  ]                     # }
  ?                     # // ptr = 0, v = [0, 0, 'H', 'h', 'X', ' ', 0x08]
  2>.                   #            putchar(v[2]);
  >3-.                  # v[3] -= 3; putchar(v[3]);
  7+..                  # v[3] += 7; putchar(v[3]); putchar(v[3]);
  3+.                   # v[3] += 3; putchar(v[3]);
  2>.                   #            putchar(v[5]);
  <-.                   # v[4] -= 1; putchar(v[4]);
  <.                    #            putchar(v[3]);
  3+.                   # v[3] += 3; putchar(v[3]);
  6-.                   # v[3] -= 6; putchar(v[3]);
  8-.                   # v[3] -= 8; putchar(v[3]);
  2>+.                  # v[5] += 1; putchar(v[5]);
  >2+.                  # v[6] += 2; putchar(v[6]);
  "#;

  let mut stdout = Vec::new();
  assert!(Interpreter::default().stdout(&mut stdout).eval(SOURCE).is_ok());
  assert_eq!(stdout, "Hello World!\n".as_bytes());
}

#[test]
fn fizz_buzz() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
  const SOURCE:&'static str =
      "++++++[->++++>>+>+>-<<<<<]>[<++++>>+++>++++>>+++>+++++>+++++>>>>>>++>>++<<<<<<<<
       <<<<<<-]<++++>+++>-->+++>->>--->++>>>+++++[->++>++<<]<<<<<<<<<<[->-[>>>>>>>]>[<+
       ++>.>.>>>>..>>>+<]<<<<<-[>>>>]>[<+++++>.>.>..>>>+<]>>>>+<-[<<<]<[[-<<+>>]>>>+>+<
       <<<<<[->>+>+>-<<<<]<]>>[[-]<]>[>>>[>.<<.<<<]<[.<<<<]>]>.<<<<<<<<<<<]";

  let mut stdout = Vec::new();
  assert!(Interpreter::default().stdout(&mut stdout).eval(SOURCE).is_ok());
  for (i, r) in String::from_utf8_lossy(&stdout[..(stdout.len() - 1)])
    .split("\n")
    .enumerate() {
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

  let mut stdout = Vec::new();
  assert!(Interpreter::default().stdout(&mut stdout).eval(SOURCE).is_ok());
  for (i, r) in String::from_utf8_lossy(&stdout[..(stdout.len() - 1)])
    .split("\n")
    .enumerate() {
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

  let mut stdout = Vec::new();
  assert!(Interpreter::default().stdout(&mut stdout).eval(SOURCE).is_ok());

  assert_eq!(stdout,
             "2 3 5 7 11 13 17 19 23 29 31 37 41 32 36 42 48 50 56 60 62 68 72 78 86".as_bytes());
}
