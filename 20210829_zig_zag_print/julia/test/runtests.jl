using Test;
using julia;

@testset "'thisisazigzag', k = 4" begin
    expected_output = "t     a     g\n h   s z   a \n  i i   i z  \n   s     g   "
    @test printzigzag("thisisazigzag", 4) == expected_output
end

@testset "'ilovechickenandwaffles', k = 3" begin
    expected_output = "i   e   c   a   a   e \n l v c i k n n w f l s\n  o   h   e   d   f   "
    @test printzigzag("ilovechickenandwaffles", 3) == expected_output
end
