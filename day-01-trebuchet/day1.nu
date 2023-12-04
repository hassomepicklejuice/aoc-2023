def part1 [] {
  str trim
    | split row "\n"
    | each { |line|
      let $first = ($line | parse -r '\D*(?P<first>\d).*').first.0
      let $last = ($line | parse -r '.*(?P<last>\d)\D*').last.0
      $first + $last | into int
    }
    | math sum
}

def str2int [s: string] {
  match $s {
    "one" => 1
    "two" => 2
    "three" => 3
    "four" => 4
    "five" => 5
    "six" => 6
    "seven" => 7
    "eight" => 8
    "nine" => 9
    _ => ($s | into int)
  }
}

def part2 [] {
  str trim
    | split row "\n"
    | each { |line|
      let $first = ($line | parse -r '(?P<first>(one|two|three|four|five|six|seven|eight|nine|\d)).*').first.0
      let $last = ($line | parse -r '.*(?P<last>(one|two|three|four|five|six|seven|eight|nine|\d))').last.0
      (str2int $first) * 10 + (str2int $last)
    }
    | math sum
}

