using Test
using julia

@testset "Examples" begin
    @test format_duration(0)    == "now"
    @test format_duration(1)    == "1 second"
    @test format_duration(62)   == "1 minute and 2 seconds"
    @test format_duration(120)  == "2 minutes"
    @test format_duration(3600) == "1 hour"
    @test format_duration(3662) == "1 hour, 1 minute and 2 seconds"
end