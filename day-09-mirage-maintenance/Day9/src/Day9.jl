module Day9

export parseinput, extrapolate

import Polynomials

function parseinput(input::String)
    map(eachsplit(input, '\n')) do line
        map(x -> parse(Int, x), eachsplit(line))
    end |> collect
end

function extrapolate(input::String)
    input = parseinput(input)

    part1 = 0
    part2 = 0

    for line in input
        len = length(line)
        f = Polynomials.fit(1:len, line)
        part1 += round(Int, f(len + 1))
        part2 += round(Int, f(0))
    end

    (part1, part2)
end

end # module Day9
