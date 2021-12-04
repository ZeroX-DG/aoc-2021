let file = "input.txt"

let read_input file =
  let ic = open_in file in
  let rec build_lines lines =
    match input_line ic with
    | line -> build_lines (line :: lines)
    | exception End_of_file -> close_in ic; List.rev lines in
  build_lines []

let char_to_int char =
  int_of_string (String.make 1 char)

let rec count_ones counts line =
  match line with
  | [] -> counts
  | char::rest -> count_ones (counts + (char_to_int char)) rest

let bits_at_index lines index =
  List.map (fun line -> line.[index]) lines

let puzzle1 file =
  let lines = read_input file in

  let max_bits =
    match lines with
    | [] -> 0
    | line::rest -> String.length line in

  let max_counts = List.length lines in

  let rec calculate_gamma_and_epsilon lines index gamma epsilon =
    if index == max_bits then
      let gamma_dec = int_of_string ("0b" ^ gamma) in
      let epsilon_dec = int_of_string ("0b" ^ epsilon) in
      (gamma_dec, epsilon_dec)
    else
      let ones = count_ones 0 (bits_at_index lines index) in
      let zeros = max_counts - ones in
      let new_gamma = gamma ^ (if ones > zeros then "1" else "0") in
      let new_epsilon = epsilon ^ (if ones > zeros then "0" else "1") in
      calculate_gamma_and_epsilon lines (index + 1) new_gamma new_epsilon
  in

  let (gamma, epsilon) = calculate_gamma_and_epsilon lines 0 "" "" in
  gamma * epsilon

let puzzle2 file =
  let lines = read_input file in

  let rec filter_number f lines index =
    let max_counts = List.length lines in

    match lines with
    | [] -> 0
    | x::[] -> int_of_string ("0b" ^ x)
    | _ ->
        let ones = count_ones 0 (bits_at_index lines index) in
        let zeros = max_counts - ones in
        let remain = List.filter (fun line -> line.[index] == f ones zeros) lines in
        filter_number f remain (index + 1) in

  let calculate_oxygen lines =
    filter_number (fun ones zeros -> if ones > zeros then '1' else if zeros > ones then '0' else '1') lines 0 in
  let calculate_co2 lines =
    filter_number (fun ones zeros -> if ones > zeros then '0' else if zeros > ones then '1' else '0') lines 0 in

  let oxygen = calculate_oxygen lines in
  let co2 = calculate_co2 lines in
  oxygen * co2

let () =
  let puzzle1_result = puzzle1 file in
  let puzzle2_result = puzzle2 file in
  Printf.printf "Puzzle 1: %d | Puzzle 2: %d\n" puzzle1_result puzzle2_result
