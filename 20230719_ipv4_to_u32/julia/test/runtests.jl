using Test;
using julia;

@testset "Sample Test" begin
  @test julia.int32toip(UInt32(2154959208)) == "128.114.17.104"
  @test julia.int32toip(UInt32(0)) == "0.0.0.0"
  @test julia.int32toip(UInt32(2149583361)) == "128.32.10.1"
end
