open Util

let trails g start =
  let len = Array.length g in
  let rec traverse expected_level frontier =
    let valid_frontier =
      List.filter
        (fun (n, m) ->
          n >= 0 && n < len && m >= 0 && m < len && g.(n).(m) = expected_level)
        frontier
    in
    match expected_level with
    | 9 -> valid_frontier
    | _ ->
        let frontier' =
          CCList.flat_map
            (fun (n, m) ->
              List.map (fun (dn, dm) -> (dn + n, dm + m)) Grid.dirs)
            valid_frontier
        in
        traverse (expected_level + 1) frontier'
  in
  List.length @@ List.sort_uniq Pos.compare @@ traverse 0 [ start ]

let rating g start =
  let len = Array.length g in
  let rec aux expected_level (n, m) =
    match expected_level with
    | 9 -> 1
    | _ ->
        let frontier =
          List.map (fun (dn, dm) -> (dn + n, dm + m)) Grid.dirs
          |> List.filter (fun (n, m) ->
                 n >= 0 && n < len && m >= 0 && m < len
                 && g.(n).(m) = expected_level + 1)
        in
        frontier
        |> List.map (aux (expected_level + 1))
        |> List.fold_left ( + ) 0
  in
  aux 0 start

let solve lines =
  let g = Grid.of_lines lines |> Grid.map (fun c -> Char.code c - 48) in
  let trail_heads = Grid.indexof 0 g in
  trail_heads |> List.map (rating g) |> List.fold_left ( + ) 0
