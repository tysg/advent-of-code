module Pos = struct
  type t = int * int

  let compare = CCPair.compare Int.compare Int.compare
  let show (a, b) = Printf.sprintf "%d,%d" a b
end

module Grid = struct
  type 'a t = 'a array array

  let dirs = [ (0, 1); (1, 0); (0, -1); (-1, 0) ]

  let indexof x g =
    let res = ref [] in
    Array.iteri
      (fun n row ->
        Array.iteri (fun m c -> if x = c then res := (n, m) :: !res) row)
      g;
    !res

  let of_lines l =
    l
    |> List.map (fun s -> Array.init (String.length s) (String.get s))
    |> Array.of_list

  let map f g = Array.map (fun row -> Array.map f row) g

  let show show_c g =
    CCArray.to_string (CCArray.to_string show_c ~sep:" ") ~sep:"\n" g
end

let printfn x = Printf.ksprintf print_endline x
