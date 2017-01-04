module B = Batteries


type token =
  | Symbol of char
  | SymbolWithOffset of char * int

let tokenize (source: string) : token list =
  let open Angstrom in

  let to_int s =
    try int_of_string (String.trim s)
    with _ -> 1
  in

  let is_digit = function
    | '0'..'9' -> true
    | _ -> false
  in

  let is_symbol =
    String.contains ".,[]?"
  in
  let is_symbol_with_offset =
    String.contains "+-><"
  in

  let to_symbol c = Symbol c in
  let to_symbol_with_offset (o,c) = SymbolWithOffset (c,o) in

  (* supplemental parsers *)
  let (<!>) a b = lift2 (fun a b -> (a,b)) a b in
  let digits = take_while is_digit >>| to_int in

  let symbol =
    satisfy is_symbol >>| to_symbol
  in

  let symbol_with_offset =
    digits <!> (satisfy is_symbol_with_offset) >>| to_symbol_with_offset
  in

  let parser =
    many (symbol <|> symbol_with_offset)
  in

  let remove_comment line =
    try String.sub line 0 (String.index line '#')
    with _ -> line
  in

  (* primitive operators and functions *)
  let (>>) g f x = f (g x) in
  let flat_map f = List.map f >> List.flatten in

  let source =
    Str.split (Str.regexp "\n") source
    |> flat_map (remove_comment >> B.String.to_list)
    |> B.String.of_list
  in

  match parse_only parser (`String source) with
  | Result.Ok v -> v
  | Result.Error msg -> failwith msg


let build_ast tokens =
  let module Local = struct
    type progress =
      | Continue of continue_t
      | Done of int
    and
      continue_t = { count: int; nest: int }

    (* state function *)
    let update_progress p token =
      match (p, token) with
      (* add nest *)
      | (Continue {count; nest}, Symbol '[') -> Continue {count=count+1; nest=nest+1}
      (* found unmatched rparen *)
      | (Continue {count; nest}, Symbol ']') when nest > 1 -> Continue {count=count+1; nest=nest-1}
      (* found matched rparen *)
      | (Continue {count; nest}, Symbol ']') when nest = 1 -> Done count
      (* not paren *)
      | (Continue {count; nest}, _) -> Continue {count=count+1; nest}
      (* done *)
      | (Done count, _) -> Done count


    let find_rparen_position tokens =
      let result = List.fold_left update_progress (Continue {count=0; nest=1}) tokens in
      match result with
      | Done cursor -> Some cursor
      | Continue _ -> None

  end in

  let rec _body result tokens =
    match tokens with
    | [] -> result
    | head :: tail ->
      let (command, offset) = match head with
        | Symbol '.' -> (Ast.PutChar, 0)
        | Symbol ',' -> (Ast.GetChar, 0)
        | Symbol '?' -> (Ast.CoreDump, 0)
        | SymbolWithOffset ('+', n) -> (Ast.AddVal n, 0)
        | SymbolWithOffset ('-', n) -> (Ast.AddVal (-n), 0)
        | SymbolWithOffset ('>', n) -> (Ast.AddPtr n, 0)
        | SymbolWithOffset ('<', n) -> (Ast.AddPtr (-n), 0)
        | Symbol ']' -> failwith "unexpected ']' is found"
        | Symbol '[' ->
          let n = match Local.find_rparen_position tail with
            | Some n -> n
            | None -> failwith "incorrect nest"
          in
          let stmt = _body [] (B.List.take n tail) in
          (Ast.Loop stmt, (n + 1))
        | _ -> failwith "Unreachable"
      in
      _body (List.append result [command]) (tail |> B.List.drop offset)
  in
  _body [] tokens


let parse source = source |> tokenize |> build_ast
