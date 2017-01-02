extern crate brainfuck;
use brainfuck::Engine;

fn main() {
  #[cfg_attr(rustfmt, rustfmt_skip)]
  const HELLO_WORLD: &'static str =
    "++++++++[>  ++++[>++>+  ++>+++>+<<  <<-]>+>+>-
     >>+[<]<-]>  >.>---.+++  ++++..+++.  >>.<-.<.++
     +.------.-  -------.>>  +.>++.";

  let mut engine = Engine::default();
  match engine.eval(HELLO_WORLD, "") {
    Ok(stdout) => println!("Success:\n\"{}\"", stdout),
    Err(err) => println!("Failure: {}", err),
  }
}
