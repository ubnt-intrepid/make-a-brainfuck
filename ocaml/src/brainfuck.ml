open Batteries

type ast =
  | AddVal of int
  | AddPtr of int
  | PutChar
  | GetChar
  | CoreDump
  | Loop of ast list
[@@deriving show]


type token =
  | Symbol of char
  | SymbolWithOffset of char * int
[@@deriving show]

let tokenize source =
  let remove_comment line =
    try String.sub line 0 (String.index line '#')
    with _ -> line
  in
  let to_token =
    function
    | c when String.contains ".,[]?" c -> Some (Symbol c)
    | c when String.contains "+-><" c -> Some (SymbolWithOffset (c, 1))
    | _ -> None
  in
  let (>>) g f x = f (g x) in
  source
  |> Str.split (Str.regexp "\n")
  |> List.map (remove_comment >> String.to_list)
  |> List.flatten
  |> List.filter_map to_token


type progress =
  | InNest of int * int
  | Done of int

let rec _build_ast result tokens =
  match tokens with
  | [] -> result
  | head :: tail ->
    let (command, offset) = match head with
      | Symbol '.' -> (PutChar, 0)
      | Symbol ',' -> (GetChar, 0)
      | Symbol '?' -> (CoreDump, 0)
      | SymbolWithOffset ('+', n) -> (AddVal n, 0)
      | SymbolWithOffset ('-', n) -> (AddVal (-n), 0)
      | SymbolWithOffset ('>', n) -> (AddPtr n, 0)
      | SymbolWithOffset ('<', n) -> (AddPtr (-n), 0)
      | Symbol ']' -> failwith "unexpected ']' is found"
      | Symbol '[' ->
        begin
          let find_paren progress token =
            match progress with
            | Done p -> Done p
            | InNest (count, nest) ->
              begin match token with
                | Symbol '[' ->
                  InNest (count + 1, nest + 1)
                | Symbol ']' ->
                  if nest - 1 = 0 then Done count
                  else InNest (count + 1, nest - 1)
                | _ ->
                  InNest (count + 1, nest)
              end
          in
          let cursor = tail |> List.fold_left find_paren (InNest (0, 1)) in
          match cursor with
          | Done cursor ->
            let stmt = _build_ast [] (tail |> List.take cursor) in
            (Loop stmt, (cursor + 1))
          | _ -> failwith "incorrect nest"
        end
      | _ -> failwith "Unreachable"
    in
    _build_ast (List.append result [command]) (tail |> List.drop offset)

let build_ast tokens = _build_ast [] tokens
