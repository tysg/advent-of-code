open Advent_of_code

let () =
  match Sys.argv with
  | [| _; day; file |] ->
      print_newline ();
      print_endline ("Day " ^ day ^ " " ^ file);
      let input = In_channel.(with_open_bin file input_all) in
      let lines = String.split_on_char '\n' (String.trim input) in
      let output =
        match day with
        | "1" -> Day1.solve lines
        | "5" -> Day5.solve lines
        | "6" -> Day6.solve lines
        | _ -> failwith "Unknown day"
      in
      print_int output;
      print_newline ()
  | _ -> print_endline "Usage: ./main.exe <day> <input file>"
