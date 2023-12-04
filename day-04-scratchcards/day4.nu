use std assert

const example = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"

#[test]
def test1 [] {
  assert (($example | part1) == 13)
}

#[test]
def test2 [] {
  assert (($example | part2) == 30)
}

def parse-input [] {
  str trim
  | split row "\n"
  | split column -r '\s*(\||:)\s*' card winning numbers
  | upsert card {|c| $c.card | split row -r '\s+' | get 1 | into int}
  | upsert winning {|w| $w.winning
    | split row -r '\s+'
    | each {|n| $n | str trim | into int}
  }
  | upsert numbers {|n| $n.numbers
    | split row -r '\s+'
    | each {|n| $n | str trim | into int}
  }
}

def part1 [] {
  parse-input
  | each {|card| $card.numbers
    | filter {|n| ($card.winning | find $n | length) > 0}
    | length
    | do {if $in == 0 {
      0
    } else {
      2 ** ($in - 1)
    }}
  }
  | math sum
}

def part2 [] {
  mut input = parse-input
  let original = $input
  mut index = 0
  while $index < ($input | length) {
    let winnings = {
      card: ($input | get $index | get card),
      count: ($input
      | get $index
      | do {let input = $in
        $input.numbers
        | filter {|n| $n in $input.winning}
        | length
      })
    }
    let copies = $original | skip $winnings.card | first $winnings.count
    $input = ($input | append $copies)
    $index = ($index + 1 | inspect)
  }
  $input | length
}
