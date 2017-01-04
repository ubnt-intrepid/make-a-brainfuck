open Ast
open Angstrom


type token =
  | Symbol of char
  | SymbolWithOffset of char * int
[@@deriving show]


let tokenize (source:string) : token list =
  let remove_comment line =
    try String.sub line 0 (String.index line '#')
    with _ -> line
  in
  let parser =
    let symbol =
      satisfy (String.contains ".,[]?") >>| (fun c -> Symbol c)
    in
    let symbol_with_offset =
      let digits =
        take_while (fun c -> c >= '0' && c <= '9')
        >>| (fun d -> if d = "" then 1 else int_of_string d)
      in
      let symbol = satisfy (String.contains "+-><") in
      lift2 (fun d s -> (d, s)) digits symbol
      >>| (fun (offset, symbol) -> SymbolWithOffset (symbol, offset))
    in
    Angstrom.many (symbol <|> symbol_with_offset)
  in
  let (>>) g f x = f (g x) in
  source
  |> Str.split (Str.regexp "\n")
  |> Batteries.List.map (remove_comment >> Batteries.String.to_list)
  |> List.flatten
  |> Batteries.String.of_list
  |> (fun s -> parse_only parser (`String s))
  |> (fun v -> match v with
      | Result.Ok v -> v
      | Result.Error msg -> failwith msg)


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
            let stmt = _build_ast [] (tail |> Batteries.List.take cursor) in
            (Loop stmt, (cursor + 1))
          | _ -> failwith "incorrect nest"
        end
      | _ -> failwith "Unreachable"
    in
    _build_ast (List.append result [command]) (tail |> Batteries.List.drop offset)

let build_ast tokens = _build_ast [] tokens


let parse source = source |> tokenize |> build_ast
