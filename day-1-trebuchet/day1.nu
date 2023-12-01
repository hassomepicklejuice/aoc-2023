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
