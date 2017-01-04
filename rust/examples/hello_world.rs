extern crate brainfuck;
use brainfuck::Interpreter;

fn main() {
  #[cfg_attr(rustfmt, rustfmt_skip)]
  const HELLO_WORLD: &'static str =
    "++++++++[>  ++++[>++>+  ++>+++>+<<  <<-]>+>+>-
     >>+[<]<-]>  >.>---.+++  ++++..+++.  >>.<-.<.++
     +.------.-  -------.>>  +.>++.";

  let mut i = Interpreter::default();
  match i.eval(HELLO_WORLD) {
    Ok(()) => println!("Success:\n\"{}\"", String::from_utf8_lossy(i.stdout())),
    Err(err) => println!("Failure: {}", err),
  }
}
