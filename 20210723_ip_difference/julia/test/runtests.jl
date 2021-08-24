using Test
using julia

@testset "Sample tests" begin
    @test ipsbetween("10.0.0.0",  "10.0.0.50") ==    50
    @test ipsbetween("10.0.0.0",  "10.0.1.0")  ==   256
    @test ipsbetween("20.0.0.10", "20.0.1.0") ==   246
    @test ipsbetween("20.0.0.10", "20.1.1.0") == 65782
end
