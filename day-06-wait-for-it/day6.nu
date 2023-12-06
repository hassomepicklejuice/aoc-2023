const EXAMPLE = "Time:      7  15   30
Distance:  9  40  200"

def parse-input1 [] {
  split row "\n"
  | parse "{column}: {vals}"
  | upsert vals {|it| $it.vals
    | str trim
    | split row -r '\s+'
    | each {|it| $it | into int}
  }
  | each {|it| $it.vals | wrap $it.column}
  | do {
    let input = $in
    $input.0 | merge $input.1
  }
}

def part1 [] {
  par-each {|it|
    let range = 0..$it.Time
    $range | par-each {|time| $time * ($it.Time - $time)}
      | take while {|dist| $dist < $it.Distance}
      | length
      | inspect
  }
  | math product
}

def parse-input2 [] {
  split row "\n"
  | parse "{column}: {vals}"
  | upsert vals {|it| $it.vals | str trim | split row -r '\s+' | str join | into int}
  | each {|it| $it.vals | wrap $it.column}
  | do {
    let input = $in
    $input.0 | merge $input.1
  }
}

def part2 [] {
  let input = $in
  let time = $input.Time
  let dist = $input.Distance
  let min = ($time - ($time ** 2 - 4 * $dist | math sqrt)) / 2
  let max = ($time + ($time ** 2 - 4 * $dist | math sqrt)) / 2
  $max - $min | math floor
}


