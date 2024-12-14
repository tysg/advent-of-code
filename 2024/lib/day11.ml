let results = Hashtbl.create 16
let n_digits n = int_of_float (floor (log10 (float_of_int n))) + 1

let split_stone n =
  let fac = CCInt.pow 10 (n_digits n / 2) in
  (n / fac, n mod fac)

let rec blink n_times n =
  match Hashtbl.find_opt results (n_times, n) with
  | Some ans -> ans
  | None ->
      let ans =
        match n_times with
        | 0 -> 1
        | _ -> (
            match n with
            | 0 -> blink (n_times - 1) 1
            | n when n_digits n mod 2 = 0 ->
                let l, r = split_stone n in
                blink (n_times - 1) l + blink (n_times - 1) r
            | n -> blink (n_times - 1) (2024 * n))
      in
      Hashtbl.replace results (n_times, n) ans;
      ans

let solve lines =
  let stones =
    String.split_on_char ' ' (List.hd lines) |> List.map int_of_string
  in
  stones |> List.map (blink 75) |> List.fold_left ( + ) 0
