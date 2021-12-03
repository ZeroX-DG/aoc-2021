let file = "input.txt"

let read_input file =
  let ic = open_in file in
  let rec build_lines lines =
    match input_line ic with
    | line -> build_lines (line :: lines)
    | exception End_of_file -> close_in ic; List.rev lines in
  build_lines []


let puzzle1 input =
  let lines = read_input input in
  let depths = List.map int_of_string lines in
  let rec calculate_depth_increments increments depths =
    match depths with
    | [] -> increments
    | x::[] -> increments
    | previous::next::rest -> calculate_depth_increments (increments + (if next > previous then 1 else 0)) (next::rest) in
  calculate_depth_increments 0 depths

let puzzle2 input =
  let lines = read_input input in
  let depths = List.map int_of_string lines in
  let rec calculate_depth_increments increments depths =
    match depths with
    | [] -> increments
    | x::[] -> increments
    | x::y::[] -> increments
    | x::y::z::[] -> increments
    | a::b::c::d::rest ->
      let next = b + c + d in
      let previous = a + b + c in
      calculate_depth_increments (increments + (if next > previous then 1 else 0)) (b::c::d::rest) in
  
  calculate_depth_increments 0 depths 

let () =
  let puzzle1_result = puzzle1 file in
  let puzzle2_result = puzzle2 file in
  Printf.printf "Puzzle 1: %d | Puzzle 2: %d\n" puzzle1_result puzzle2_result
