extern crate brainfuck;

fn main() {
  #[cfg_attr(rustfmt, rustfmt_skip)]
  const HELLO_WORLD: &'static str =
    "++++++++[>  ++++[>++>+  ++>+++>+<<  <<-]>+>+>-
     >>+[<]<-]>  >.>---.+++  ++++..+++.  >>.<-.<.++
     +.------.-  -------.>>  +.>++.";

  let mut engine = brainfuck::Engine::new(0, vec![0;8096]);
  match engine.eval(HELLO_WORLD, "") {
    Ok(stdout) => println!("Success:\n\"{}\"", stdout),
    Err(err) => println!("Failure: {}", err),
  }


  let s = "++>+++++[<+>-]++++++++[<++++++>-]<.";
  let result = brainfuck::parser::parse(s);
  for ref p in result.as_ref().unwrap() {
    println!("{:?}", p);
  }
}
