open Util

let perimeter ps =
  (* O(n^2) but meh *)
  List.fold_left
    (fun acc (n, m) ->
      let neighbors =
        Grid.dirs |> List.map (fun (dn, dm) -> (n + dn, m + dm))
      in
      let n_neighbors =
        List.filter (fun p -> List.mem p ps) neighbors |> List.length
      in
      acc + 4 - n_neighbors)
    0 ps

let sides ps =
  let side_fragments =
    List.fold_left
      (fun acc (n, m) ->
        let frags =
          Grid.dirs
          |> List.filter (fun (dn, dm) -> not @@ List.mem (n + dn, m + dm) ps)
          |> List.map (fun d -> (d, (n, m)))
        in
        List.rev_append frags acc)
      [] ps
  in

  Grid.dirs
  |> List.map (fun d ->
         let sides =
           List.filter_map
             (fun (d', pos) -> if d = d' then Some pos else None)
             side_fragments
         in
         let segments =
           match d with
           | 0, _ ->
               let ps' =
                 List.sort
                   (fun (a, b) (c, d) -> Pos.compare (b, a) (d, c))
                   sides
               in

               let res =
                 CCList.group_succ
                   ~eq:(fun (n, m) (n', m') -> n - n' = 1 && m = m')
                   ps'
               in
               res
           | _, 0 ->
               let ps' = List.sort Pos.compare sides in
               CCList.group_succ
                 ~eq:(fun (n, m) (n', m') -> m - m' = 1 && n = n')
                 ps'
           | _ -> failwith "impossible"
         in
         List.length segments)
  |> List.fold_left ( + ) 0

let start g n m =
  let plot = ref [] in
  let char = g.(n).(m) in
  printfn "current char: %c" char;
  let len = Array.length g in
  let rec dfs n m =
    if n >= 0 && n < len && m >= 0 && m < len then
      match g.(n).(m) with
      | '.' -> ()
      | c when c = char ->
          plot := (n, m) :: !plot;
          g.(n).(m) <- '.';
          Grid.dirs |> List.iter (fun (dn, dm) -> dfs (dn + n) (dm + m))
      | _ -> ()
    else ()
  in
  dfs n m;
  List.length !plot * sides !plot

let solve l =
  let g = Grid.of_lines l in
  let res = ref 0 in
  g
  |> Array.iteri (fun n row ->
         row
         |> Array.iteri (fun m -> function
              | '.' -> () | _ -> res := !res + start g n m));
  !res
