open Util

let rec gcd a b = if b = 0 then a else gcd b (a mod b)

let solve lines =
  let grid = Grid.of_lines lines in
  let len = Array.length grid in
  let letters =
    let s = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz" in
    List.init (String.length s) (String.get s)
  in

  CCList.(
    letters
    |> flat_map (fun c ->
           let pos = Grid.indexof c grid in
           product
             (fun a b ->
               if a = b then []
               else
                 let (an, am), (bn, bm) = (a, b) in
                 let dn, dm = (an - bn, am - bm) in
                 let _part1 = [ (an + dn, am + dm) ] in
                 let part2 =
                   let f = gcd (abs dn) (abs dm) in
                   let dn, dm = (dn / f, dm / f) in
                   init len (fun i ->
                       (bn + ((i + 1) * dn), bm + ((i + 1) * dm)))
                 in
                 part2
                 |> filter (fun (n, m) ->
                        n >= 0 && n < len && m >= 0 && m < len))
             pos pos)
    |> flatten |> sort_uniq ~cmp:Pos.compare |> length)
