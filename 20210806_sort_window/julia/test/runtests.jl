using Test;
using julia;

@testset "Sample Test" begin
    @test sortwindow([3, 7, 5, 6, 9]) == [2, 4]
    @test sortwindow([1, 2, 6, 5, 4]) == [3, 5]
    @test sortwindow([3, 2, 1, 4, 5]) == [1, 3]
    @test sortwindow([5, 4, 3, 2, 1]) == [1, 5]
    @test sortwindow([1, 2, 3, 4, 5]) == []
    @test sortwindow([1]) == []
    @test sortwindow(Vector{Int}()) == []
end
