open Containers

let rex = Re.compile (Re.Pcre.re {|X\+(\d+), Y\+(\d+)|})
let prize_rex = Re.compile (Re.Pcre.re {|Prize: X=(\d+), Y=(\d+)|})

let solve_eqn (ax, ay) (bx, by) (px, py) =
  (* part 2 *)
  let px, py = (10000000000000 + px, 10000000000000 + py) in
  let n = ((px * by) - (bx * py)) / ((ax * by) - (bx * ay)) in
  let m = ((py * ax) - (ay * px)) / ((by * ax) - (ay * bx)) in
  if (ax * n) + (bx * m) = px && (ay * n) + (by * m) = py then Some ((3 * n) + m)
  else None

let solve lines =
  lines |> List.chunks 4
  |> List.map (function
       | btn_a :: btn_b :: prize :: _ ->
           let a =
             match Re.Group.all (Re.exec rex btn_a) with
             | [| _; x; y |] -> (int_of_string x, int_of_string y)
             | _ -> failwith "invalid btn_a"
           in
           let b =
             match Re.Group.all (Re.exec rex btn_b) with
             | [| _; x; y |] -> (int_of_string x, int_of_string y)
             | _ -> failwith "invalid btn_b"
           in
           let p =
             match Re.Group.all (Re.exec prize_rex prize) with
             | [| _; x; y |] -> (int_of_string x, int_of_string y)
             | _ -> failwith "invalid btn_b"
           in
           solve_eqn a b p |> Option.get_or ~default:0
       | _ -> failwith "input error")
  |> List.fold_left ( + ) 0
