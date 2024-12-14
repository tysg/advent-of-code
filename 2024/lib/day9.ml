let parse l =
  let is_data = ref true in
  let id = ref 0 in
  let res = ref [] in
  String.iter
    (fun c ->
      let n = Char.code c - 48 in
      let () =
        match !is_data with
        | true ->
            res := `D (!id, n) :: !res;
            incr id
        | false -> res := `W n :: !res
      in
      is_data := not !is_data)
    l;
  List.rev !res

let take n cand =
  let rec aux acc n cand =
    if n = 0 then (acc, cand)
    else
      match cand with
      | (id, n') :: tl when n' <= n -> aux (`D (id, n') :: acc) (n - n') tl
      | (id, n') :: tl -> (`D (id, n) :: acc, (id, n' - n) :: tl)
      | _ -> failwith "empty cand?"
  in
  aux [] n cand

let solve lines =
  let ns = parse (List.hd lines) in
  let _expected_size =
    ns
    |> List.map (function `D (_, n) -> n | _ -> 0)
    |> List.fold_left ( + ) 0
  in
  let cand =
    ns |> List.filter_map (function `D t -> Some t | _ -> None) |> List.rev
  in

  let rec _naive_fit acc cand ns cnt =
    if cnt <= 0 then List.rev acc
    else
      match ns with
      | [] -> failwith "empty ns?"
      | `D (id, n) :: tl ->
          _naive_fit (`D (id, min n cnt) :: acc) cand tl (cnt - n)
      | `W size :: tl ->
          let ds, cand = take size cand in
          _naive_fit (ds @ acc) cand tl (cnt - size)
  in
  let first_fit (id, size) ns =
    let rec aux acc = function
      | `D (id', _) :: _ as l when id = id' -> List.rev_append acc l
      | `W size' :: tl when size' >= size ->
          (* fit *)
          let acc' = `D (id, size) :: acc in
          let acc'' = match size' - size with 0 -> acc' | n -> `W n :: acc' in
          (* remove (id, _) *)
          let tl' =
            List.map
              (function `D (id', size) when id' = id -> `W size | d -> d)
              tl
          in
          List.rev_append acc'' tl'
      | d :: tl -> aux (d :: acc) tl
      | [] -> List.rev acc
    in
    aux [] ns
  in

  let rec loop ns = function
    | [] -> ns
    | (id, size) :: tl ->
        (* let _part1 = loop (naive_fit [] cand ns expected_size) tl in *)
        (* part 2 *)
        loop (first_fit (id, size) ns) tl
  in
  let compact = loop ns cand in
  let _, acc =
    List.fold_left
      (fun (idx, acc) -> function
        | `W sz -> (idx + sz, acc)
        | `D (id, len) ->
            ( idx + len,
              acc
              + CCList.(
                  idx --^ (idx + len) |> map (( * ) id) |> fold_left ( + ) 0) ))
      (0, 0) compact
  in
  acc
