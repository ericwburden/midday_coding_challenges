using Test;
using boustrophedon;

@testset "Sample Test" begin
    input = foldl(nodepush!, collect("abcdefg"))
    @test readboustrophedon(input) == collect("acbdefg")

    input = foldl(nodepush!, collect("porgnosnoitacit"))
    @test readboustrophedon(input) == collect("prognostication")
end
