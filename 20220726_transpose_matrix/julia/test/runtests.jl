using Test;
using julia;

@testset "Empty Matrix" begin
	empty_matrix = zeros(0, 0)
	@test empty_matrix isa Matrix
	@test isempty(empty_matrix)
	@test transpose_builtin(empty_matrix) == empty_matrix
	@test transpose_byhand(empty_matrix)  == empty_matrix
end

@testset "Single Item Matrix" begin
	single_item = fill(1, 1, 1)
	@test single_item isa Matrix
	@test size(single_item) == (1, 1)
	@test transpose_builtin(single_item) == single_item
	@test transpose_byhand(single_item)  == single_item
end

@testset "3x1 Example" begin
	input  = reshape([1, 2, 3], 1, 3)
	expect = reshape([1, 2, 3], 3, 1)
	@test transpose_builtin(input) == expect
	@test transpose_byhand(input)  == expect
end

@testset "3x2 Example" begin
	input  = [1 2; 3 4; 5 6]
	expect = [1 3 5; 2 4 6]
	@test transpose_builtin(input) == expect
	@test transpose_byhand(input)  == expect
end

@testset "2x3 Example" begin
	input  = [1 3 5; 2 4 6]
	expect = [1 2; 3 4; 5 6]
	@test transpose_builtin(input) == expect
	@test transpose_byhand(input)  == expect
end

@testset "3x4 Example" begin
	input  = [1 4 7 10; 2 5 8 11; 3 6 9 12]
	expect = [1 2 3; 4 5 6; 7 8 9; 10 11 12]
	@test transpose_builtin(input) == expect
	@test transpose_byhand(input)  == expect
end
