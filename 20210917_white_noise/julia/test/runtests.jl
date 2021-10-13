using Test;
using julia;

@testset "Sample Test" begin
    @test "HELLO WORLD" == encode("HELLO WORLD") |> decode
end
