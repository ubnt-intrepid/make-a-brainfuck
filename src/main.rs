extern crate brainfuck;
use brainfuck::Engine;

fn main() {
  #[cfg_attr(rustfmt, rustfmt_skip)]
  const HELLO_WORLD: &'static str =
    "++++++++[>  ++++[>++>+  ++>+++>+<<  <<-]>+>+>-
     >>+[<]<-]>  >.>---.+++  ++++..+++.  >>.<-.<.++
     +.------.-  -------.>>  +.>++.";

  let mut stdout = Vec::new();
  let ret = {
    let mut engine = Engine::default().stdout(&mut stdout);
    engine.eval(HELLO_WORLD)
  };
  match ret {
    Ok(()) => println!("Success:\n\"{}\"", String::from_utf8_lossy(&stdout)),
    Err(err) => println!("Failure: {}", err),
  }
}
