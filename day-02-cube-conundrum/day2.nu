let counts = {
  red: 12,
  green: 13,
  blue: 14,
}

def parse-input [] {
  str trim
    | split row "\n"
    | each {|line| $line
      | parse "Game {id}: {moves}"
      | get 0
      | upsert id {|i| $i.id | into int}
    }
    | upsert moves {|e| $e.moves
      | split row ";"
      | each {|grab| $grab
        | split row ","
        | each {|cube| $cube
          | str trim
          | parse "{count} {color}"
          | get 0
          | upsert count {|c| $c.count | into int}
        }
      }
    }
}

def part1 [] {
  parse-input
  | upsert moves {|game|
    $game.moves | each {|move|
      $move | where {|grab| $grab.count > ($counts | get $grab.color)}
    }
    | flatten
  }
  | where {|game| ($game.moves | length) == 0}
  | get "id"
  | math sum
}

def part2 [] {
  parse-input
  | get moves
  | each {|moves|
    # calculate power
    $moves | each {|move| $move
      | move color --before count
      | transpose -r -d
      | default 0 red | default 0 green | default 0 blue
    }
    | math max
    | transpose color count | get count
    | math product
  }
  | math sum
}
