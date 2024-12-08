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
                 let an, am = a in
                 let bn, bm = b in
                 let dn, dm = (an - bn, am - bm) in
                 let _part1 = [ (an + dn, am + dm); (bn - dn, bm - dm) ] in
                 let f = gcd dn dm in
                 let dn, dm = (dn / f, dm / f) in

                 init len (fun i -> (an + ((i + 1) * dn), am + ((i + 1) * dm)))
                 @ init len (fun i ->
                       (bn - ((i + 1) * dn), bm - ((i + 1) * dm))))
             pos pos
           |> flatten
           |> filter (fun (n, m) -> n >= 0 && n < len && m >= 0 && m < len))
    |> sort_uniq ~cmp:Pos.compare |> length)
