using Test;
using julia;

@testset "Sample Test" begin
    input_one = Dict{Int64, Vector{Int64}}(
        0 => [1, 2], 1 => [0, 5], 2 => [0],
        3 => [6]   , 4 => []    , 5 => [1],
        6 => [3]
    )
    @test friendgroups(input_one) == 3

    input_two = Dict{Int64, Vector{Int64}}(
        0 => [1, 2], 1 => [0, 2], 2 => [0, 1, 3],
        3 => [2, 4], 4 => [3, 5], 5 => [3, 4]
    )
    @test friendgroups(input_two) == 1

    input_three = Dict{Int64, Vector{Int64}}(
        0 => [1], 1 => [0], 2 => [3], 3 => [2],
        4 => [5], 5 => [4], 6 => [7], 7 => [6]
    )
    @test friendgroups(input_three) == 4
end
