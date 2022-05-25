
zerosin(n::Int) = count(digits(n) .== 0)
countzerosto(target::Int) = mapreduce(zerosin, +, 1:target)
