using Test;
using julia;

@testset "Sample Test" begin
    input_one = [[1], [2, 3], [1, 5, 1]]
    @test longest_path(input_one) == 9

    input_two = [[6], [4, 4], [1, 2, 1], [5, 4, 3, 2]]
    @test longest_path(input_two) == 16

    input_three = [[5]]
    @test longest_path(input_three) == 5

    input_four = [[1], [1, 1], [1, 1, 1], [2, 1, 1, 1]]
    @test longest_path(input_four) == 5

    input_five = [[0], [0, 0], [0, 0, 0]]
    @test longest_path(input_five) == 0

    input_six = [[1], [1, 1], [8, 1, 1], [1, 1, 1, 9]]
    @test longest_path(input_six) == 12

    input_seven = [[1], [1, 1], [9, 1, 1], [1, 1, 1, 8]]
    @tes longest_path(input_seven) == 12
end
