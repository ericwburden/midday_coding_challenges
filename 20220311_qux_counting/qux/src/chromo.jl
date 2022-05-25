@enum Chromo begin
    Red
    Blue
    Green
end

function to_chromo(c::Char)
    c == 'R' && return Red
    c == 'G' && return Green
    c == 'B' && return Blue
    error("Don't know how to transform $c into a Chromo!")
end

function try_merge(a::Chromo, b::Chromo)
    a == Red   && b == Blue  && return Green
    a == Blue  && b == Red   && return Green
    a == Green && b == Blue  && return Red
    a == Blue  && b == Green && return Red
    a == Green && b == Red   && return Blue
    a == Red   && b == Green && return Blue
    return nothing
end

function possible_transformations(chromoes::Vector{Chromo})
    possibilities = Set{Vector{Chromo}}()

    for idx in 1:(length(chromoes)-1)
        merged = try_merge(chromoes[idx:idx+1]...)
        if !isnothing(merged)
            possibility = [chromoes...]
            splice!(possibility, idx:(idx+1), [merged])
            push!(possibilities, possibility)
        end
    end

    return possibilities
end

function fewest_remaining(chromoes::Vector{Chromo})
    stack  = Vector{Vector{Chromo}}([chromoes])
    seen   = Set{Vector{Chromo}}([chromoes])
    fewest = typemax(Int)

    while !isempty(stack)
        current = pop!(stack)
        fewest  = min(fewest, length(current))
        fewest == 1 && return 1 # Can't get smaller than that!

        for possibility in possible_transformations(current)
            possibility ∈ seen && continue
            push!(seen,  possibility)
            push!(stack, possibility)
        end
    end

    return fewest
end

function solve(chromo_string::AbstractString)
    return to_chromo.(collect(chromo_string)) |> fewest_remaining
end


using Test

@testset "Should return 1 if there's only one Chromo" begin
    @test solve("R") == 1
    @test solve("G") == 1
    @test solve("B") == 1
end

@testset "Should return 1 if there's a mismatched pair" begin
    @test solve("RB") == 1
    @test solve("BR") == 1
    @test solve("GB") == 1
    @test solve("BG") == 1
    @test solve("RG") == 1
    @test solve("GR") == 1
end

@testset "Should return 2 if there's a matched pair" begin
    @test solve("BB") == 2
    @test solve("RR") == 2
    @test solve("GG") == 2
end

@testset "Should return the length of a uniform line" begin
    @test solve("BB")      == 2
    @test solve("RRR")     == 3
    @test solve("GGGG")    == 4
    @test solve("BBBBB")   == 5
    @test solve("RRRRRR")  == 6
    @test solve("GGGGGGG") == 7
end

@testset "Should properly reduce longer lines" begin
    @test solve("RGBGB")          == 1
    @test solve("RGRGRGRGRG")     == 1
    @test solve("BBRRBBRRBBRR")   == 2
    @test solve("RGBRBBBBBBRBGR") == 2
    @test solve("GGGGGGGBBBBBBB") == 1
    @test solve("GGGGGGRRGGGGGG") == 2
    @test solve("GGG")            == 3
end


