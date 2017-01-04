let () =
  let i = new Brainfuck.interpreter in
  i#eval "++>+++++[<+>-]++++++++[<++++++>-]<."

