open Ast

class tape = object (self)
  val mutable the_pointer = (0 : int)
  val mutable the_buffer = (Array.make 8096 (Char.chr 0) : char array)

  method add_ptr n =
    the_pointer <- the_pointer + n

  method add_val n =
    the_buffer.(the_pointer) <- Char.chr ((Char.code the_buffer.(the_pointer)) + n)

  method get_char =
    the_buffer.(the_pointer)

  method put_char c =
    the_buffer.(the_pointer) <- c

  method core_dump =
    ()
end

class interpreter = object (self)
  val mutable the_tape = new tape

  method eval source =
    Parse.parse source
    |> self#eval_lines

  method eval_lines tokens =
    tokens |> List.iter self#eval_token

  method eval_token token =
    match token with
    | AddPtr n -> the_tape#add_ptr n
    | AddVal n -> the_tape#add_val n
    | PutChar ->
      self#put_char the_tape#get_char
    | GetChar -> ()
    | CoreDump -> ()
    | Loop ast ->
      while the_tape#get_char != (Char.chr 0) do
        self#eval_lines ast
      done

  method put_char c =
    Printf.printf "%c" c
end
