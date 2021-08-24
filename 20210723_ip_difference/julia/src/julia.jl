# Implement a function that receives two IPv4 addresses, and returns the number of
# addresses between them (including the first one, excluding the last one).
# 
# All inputs will be valid IPv4 addresses in the form of strings. The last address
# will always be greater than the first one.
#
# Examples
# 
# ipsbetween("10.0.0.0", "10.0.0.50")  ==   50 
# ipsbetween("10.0.0.0", "10.0.1.0")   ==  256 
# ipsbetween("20.0.0.10", "20.0.1.0")  ==  246

module julia

export ipsbetween

function ipsbetween(start::String, finish::String)
    tovec(x) = parse.(Int, split(x, "."))
    diff = tovec(finish) .- tovec(start)
    return sum((256 .^ [3, 2, 1, 0]) .* diff)
end

end # module
