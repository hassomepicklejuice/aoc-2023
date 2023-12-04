module Day2

export part1, part2

const COUNTS = Dict(:red => 12, :green => 13, :blue => 14)

function part1(input::String)::Int
    res = 0
    for line = eachsplit(strip(input), '\n')
        game, moves = split(line, ':')
        _, gameid = split(game)
        gameid = parse(Int, gameid)
        moves = map(split(moves, ';')) do grab
            map(split(grab, ',')) do cubes
                x, color = split(strip(cubes))
                [parse(Int, x), Symbol(color)]
            end
        end
        if ispossible(moves)
            res += gameid
        end
    end
    return res
end

function ispossible(game)::Bool
    for move in game
        for grab in move
            if grab[1] > COUNTS[grab[2]]
                return false
            end
        end
    end
    return true
end

function part2(input::String)::Int
    res = 0
    for line = eachsplit(strip(input), '\n')
        game, moves = split(line, ':')
        _, gameid = split(game)
        gameid = parse(Int, gameid)
        moves = map(split(moves, ';')) do grab
            map(split(grab, ',')) do cubes
                x, color = split(strip(cubes))
                [parse(Int, x), Symbol(color)]
            end
        end
        min = Dict(:red => 0, :green => 0, :blue => 0)
        for move in moves
            for grab in move
                if grab[1] > min[grab[2]]
                    min[grab[2]] = grab[1]
                end
            end
        end
        power = prod(values(min))
        res += power
    end
    return res
end

end # module Day2
