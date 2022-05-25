module julia

"""
    calculate_beauty(pairs::Vector{Tuple{Int,Int}}, k::Int)::Int

Given a list of numeric pairs (`a`, `b`), calculate the maximum possible 
'beauty' that can be achieved by `k` pairs from that list. The 'beauty' of 
any number of pairs can be calculated as:

beaty = ((a₁ - a₂ + a₃ - (-1)^(k-1)×aₖ) * min(b₁, b₂, b₃, ..., bₖ))
"""
function calculate_beauty(pairs::Vector{Tuple{Int,Int}}, k::Int)::Int

    # Start by sorting the input array by the second item, breaking ties on
    # the first, in reverse order...
    sort!(pairs, by = x -> (x[2], x[1]), rev=true)

    # Take the first `k` items, these will be our beautiful pairs. This 
    # maximizes the second term of the formula as given. We'll sort these
    # items in decreasing order of the first item, breaking ties with the
    # second item. We can identify `b` as the second value of the `k`th pair.
    second_term = pairs[k][2]
    remaining   = sort(pairs[1:k], rev=true)

    # Now we can calculate `a`, by alternately adding all the first values in
    # each pair up to and including the middle pair, then subtracting all the
    # remaining first values. This has the same effect as interleaving the first
    # values in such a way that you add the largest, subtract the smallest, 
    # add the next largest, subtract the next smallest, and so on. This 
    # maximizes the value of the first term of the formula.
    mid         = ceil(Int, k/2)
    first_term  = sum(v for (v, _) in remaining[1:mid])
    first_term -= sum(v for (v, _) in remaining[(mid+1):end])

    return first_term * second_term
end


end # module
