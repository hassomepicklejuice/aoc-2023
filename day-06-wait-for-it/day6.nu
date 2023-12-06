const EXAMPLE = "Time:      7  15   30
Distance:  9  40  200"

def parse-input [] {
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
  each {|it| 0..$it.Time
    | each {|time| $time * ($it.Time - $time)}
    | filter {|dist| $dist > $it.Distance}
    | length
  }
  | math product
}
