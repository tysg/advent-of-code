open Containers

let rex = Re.compile (Re.Pcre.re {|(\d+)\s+(\d+)|})

let parse_line line =
  match Re.Group.all (Re.exec rex line) with
  | [| _; a; b |] -> (int_of_string a, int_of_string b)
  | _ -> failwith "Invalid line"

let parse lines =
  let pairs = List.map parse_line lines in
  let left = List.sort Int.compare @@ List.map fst pairs in
  let right = List.sort Int.compare @@ List.map snd pairs in
  (left, right)

let solve1 lines =
  let left, right = parse lines in

  List.(
    combine left right
    |> map (fun (a, b) -> Int.abs (a - b))
    |> fold_left ( + ) 0)

let solve lines =
  let left, right = parse lines in

  List.(
    left
    |> map (fun n -> n * length (filter (( = ) n) right))
    |> fold_left ( + ) 0)
