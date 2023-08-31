using Test;
using julia;

@testset "Can determine the largest rectangle under a histogram" begin
  @test julia.max_rect_histogram([1, 2, 3, 4, 5]) == 9
  @test julia.max_rect_histogram([5, 4, 3, 2, 1]) == 9
  @test julia.max_rect_histogram([2, 5, 5, 5, 2, 2, 2]) == 15
  @test julia.max_rect_histogram([2, 2, 5, 5, 5, 2, 2, 2]) == 16
  @test julia.max_rect_histogram([5, 5, 2, 2, 5, 5]) == 12
  @test julia.max_rect_histogram([5, 5, 5, 50, 5, 5]) == 50
end

@testset "Can find largest rectangle in a matrix" begin
  input = [
    "111110",
    "100000",
    "101110",
    "101110",
    "101110"
  ]
  @test julia.matrix_challenge(input) == 9

  input = [
    "000000000000",
    "011111111110",
    "011110111110",
    "011110111110",
    "010000000010",
    "011110111110",
    "011110111110",
    "011111111110",
    "000000000000"
  ]
  @test julia.matrix_challenge(input) == 15
end
