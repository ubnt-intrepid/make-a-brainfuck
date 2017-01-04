
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

val tokenize: string -> token list

val build_ast: token list -> ast list
