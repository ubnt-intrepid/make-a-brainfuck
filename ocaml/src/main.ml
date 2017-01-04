let () =
  let tokens = Brainfuck.tokenize "..++[>>+<<[<]\n]"
  in
  tokens
  |> List.map (fun token -> Printf.printf "%s\n" (Brainfuck.show_token token))
  |> ignore;
  let ast = Brainfuck.build_ast tokens
  in
  ast
  |> List.map (fun ast -> Printf.printf "%s\n" (Brainfuck.show_ast ast))
  |> ignore
