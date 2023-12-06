let _example1 = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"

let part1 (input: string): int =
  let digits = input
    |> String.split_on_char '\n'
    |> List.map (fun line ->
      let digits = line
      |> String.fold_left (fun acc ch -> if ch >= '0' && ch <= '9' then ch :: acc else acc) []
      |> List.map (fun ch -> Char.code ch - Char.code '0')
      |> List.rev in
      digits
    ) in
  let first = List.map List.hd digits in
  let last = List.map (fun l -> List.rev l |> List.hd) digits in
  List.fold_left2 (fun acc first last -> acc + first * 10 + last) 0 first last
  
let _example2 = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"

let (--) i j =
  let rec aux n acc =
    if n < i then acc else aux (n - 1) (n :: acc)
  in aux (j - 1) []

let get_digit (line: string): int option =
  match line.[0] with
  | '0' -> Some 0
  | '1' -> Some 1
  | '2' -> Some 2
  | '3' -> Some 3
  | '4' -> Some 4
  | '5' -> Some 5
  | '6' -> Some 6
  | '7' -> Some 7
  | '8' -> Some 8
  | '9' -> Some 9
  | 'o' when String.starts_with ~prefix:"one" line -> Some 1
  | 'e' when String.starts_with ~prefix:"eight" line -> Some 8
  | 'n' when String.starts_with ~prefix:"nine" line -> Some 9
  | 't' -> if String.starts_with ~prefix:"two" line then Some 2
      else if String.starts_with ~prefix:"three" line then Some 3
      else None
  | 'f' -> if String.starts_with ~prefix:"four" line then Some 4
      else if String.starts_with ~prefix:"five" line then Some 5
      else None
  | 's' -> if String.starts_with ~prefix:"six" line then Some 6
      else if String.starts_with ~prefix:"seven" line then Some 7
      else None
  | _ -> None

let part2 (input: string): int =
  let lines = input |> String.split_on_char '\n' in
  let nums = lines |> List.map (fun line ->
    let digits = 
      let l = String.length line in
      let ds = (0 -- l)
      |> List.map (fun n ->
        (String.sub line n (l - n)) |> get_digit |> Option.to_list)
      |> List.flatten in
      ds
    in
    let first = digits |> List.hd in
    let last = digits |> List.rev |> List.hd in
    first * 10 + last
  ) in
  nums |> List.fold_left (fun acc n -> acc + n) 0

let () =
  let input = Stdlib.stdin |> In_channel.input_all |> String.trim in
  (*let input = _example1 in*)
  print_endline "part1: ";
  part1 input |> print_int;
  print_string "\n\n";
  print_endline "part2: ";
  part2 input |> print_int
