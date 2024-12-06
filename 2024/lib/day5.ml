let mid xs = xs.(Array.length xs / 2)

let parse lines =
  let idx = List.find_index (( = ) "") lines |> Option.get in
  let pairs, lines = CCList.take_drop idx lines in

  let pairs =
    pairs
    |> List.map (fun l ->
           match String.split_on_char '|' l with
           | [ l; r ] -> (int_of_string l, int_of_string r)
           | _ -> failwith "")
  in

  let lines =
    lines |> List.tl
    |> List.map (fun l -> List.map int_of_string @@ String.split_on_char ',' l)
    |> List.map Array.of_list
  in
  (pairs, lines)

let solve1 lines =
  let pairs, lines = parse lines in
  lines
  |> List.filter (fun xs ->
         List.for_all
           (fun (l, r) ->
             let li = Array.find_index (( = ) l) xs in
             let ri = Array.find_index (( = ) r) xs in
             match (li, ri) with Some i, Some r -> i < r | _ -> true)
           pairs)
  |> List.map mid |> List.fold_left ( + ) 0

(* brute force, computer goes brrrr *)
let solve lines =
  let pairs, lines = parse lines in
  let wrongs =
    lines
    |> List.filter (fun xs ->
           not
           @@ List.for_all
                (fun (l, r) ->
                  let li = Array.find_index (( = ) l) xs in
                  let ri = Array.find_index (( = ) r) xs in
                  match (li, ri) with Some i, Some r -> i < r | _ -> true)
                pairs)
  in

  let () =
    List.iter
      (Array.sort (fun l r ->
           if List.mem (l, r) pairs then -1
           else if List.mem (r, l) pairs then 1
           else 0))
      wrongs
  in
  wrongs |> List.map mid |> List.fold_left ( + ) 0
