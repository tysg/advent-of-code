let dbg ?__POS__ fmt =
  let k =
    Format.fprintf Format.err_formatter
      ("%s: " ^^ fmt ^^ "\n%!")
      (match __POS__ with
      | None -> ""
      | Some (file, lnum, _, _) -> Printf.sprintf "[%s:%d] " file lnum)
  in
  k
