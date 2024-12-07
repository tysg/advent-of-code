let parse line =
  match String.split_on_char ':' line with
  | [ sum; parts ] ->
      ( int_of_string sum,
        List.map int_of_string (String.split_on_char ' ' (String.trim parts)) )
  | _ -> failwith ""

let eval parts =
  let res = ref [] in
  let () =
    parts
    |> List.iter (fun n ->
           match !res with
           | [] -> res := [ n ]
           | _ ->
               let res' = !res in
               res := List.map (Int.mul n) res' @ List.map (( + ) n) res' @ !res)
  in
  !res

let eval2 parts =
  let res = ref [] in
  let () =
    parts
    |> List.iter (fun n ->
           match !res with
           | [] -> res := [ n ]
           | _ ->
               let res' = !res in
               res :=
                 List.map (Int.mul n) res'
                 @ List.map (( + ) n) res'
                 @ List.map
                     (fun m ->
                       int_of_string (string_of_int m ^ string_of_int n))
                     !res)
  in
  !res

let solve lines =
  let puzzles = List.map parse lines in
  List.fold_left
    (fun acc (sum, parts) ->
      if List.mem sum (eval2 parts) then acc + sum else acc)
    0 puzzles
