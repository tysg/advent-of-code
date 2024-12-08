open Util

let parse lines =
  Grid.(
    of_lines lines
    |> map (function
         | '.' -> `D
         | '#' -> `H
         | '^' -> `W
         | _ -> failwith "invalid"))

let next grid ((n, m), (dn, dm)) =
  let len = Array.length grid in
  let n', m' = (n + dn, m + dm) in
  if n' < 0 || n' >= len || m' < 0 || m' >= len then None
  else
    match grid.(n').(m') with
    | `D | `W -> Some ((n', m'), (dn, dm))
    | `H ->
        let dn', dm' =
          match (dn, dm) with
          | -1, 0 -> (0, 1)
          | 0, 1 -> (1, 0)
          | 1, 0 -> (0, -1)
          | 0, -1 -> (-1, 0)
          | _ -> failwith ""
        in
        Some ((n, m), (dn', dm'))

let solve1 lines =
  let grid = parse lines in
  let pos = Grid.indexof `W grid |> List.hd in
  let rec loop trail (pos, d) =
    match next grid (pos, d) with
    | None -> pos :: trail
    | Some (pos', d') -> loop (pos :: trail) (pos', d')
  in
  let trail = loop [] (pos, (-1, 0)) in
  CCList.uniq ~eq:(CCPair.equal Int.equal Int.equal) trail |> List.length

let solve lines =
  let grid = parse lines in
  let len = Array.length grid in
  let pos = Grid.indexof `W grid |> List.hd in
  let idxs = CCList.(product CCPair.make (0 --^ len) (0 --^ len)) in
  List.fold_left
    (fun cnt (i, j) ->
      match grid.(i).(j) with
      | `H | `W -> cnt
      | `D ->
          grid.(i).(j) <- `H;
          let rec loop tbl pos =
            match next grid pos with
            | None -> false
            | Some pos' ->
                if Hashtbl.mem tbl pos' then true
                else (
                  Hashtbl.replace tbl pos ();
                  loop tbl pos')
          in
          let tbl = Hashtbl.create 16 in
          let res = loop tbl (pos, (-1, 0)) in
          grid.(i).(j) <- `D;
          cnt + if res then 1 else 0)
    0 idxs
