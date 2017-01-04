type ast =
  | AddVal of int
  | AddPtr of int
  | PutChar
  | GetChar
  | CoreDump
  | Loop of ast list
[@@deriving show]
