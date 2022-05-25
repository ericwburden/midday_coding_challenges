"""
    countingsort!(arr, place)

Sort the integers in `arr` according to the value in each integer's `place`
place value. If `place` is `1`, for example, sort the integers by the `1`'s 
place.

# Examples
```julia-repl
julia> countingsort!([19, 75, 31, 28, 6, 54], 10)
6-element Vector{Int64}
  6
 19
 28
 31
 54
 75
```
"""
function countingsort!(arr::Vector{Int}, place::Int)
    # Initialize a 10-length array to hold the counts of numbers in `arr`
    # with the value `index - 1` in their `place` place.
    count = zeros(Int, 10)

    # Count the number of values with each digit in their `place` place. After
    # this, `count[1]` will contain a count of the values in `arr` that have
    # `0` in the `place` place. The mismatch between count and value is due
    # to Julia being 1-indexed.
    for val in arr
        place_value = (val ÷ place) % 10
        count[place_value + 1] += 1
    end

    # Modify `count` to hold a cumulative sum of counts. The impact of this
    # is that, for example, the value in `count[1]` will now hold the 'last'
    # index in the output array where a number with `0` in the `place` place
    # can go.
    for idx in 2:length(count)
        count[idx] += count[idx - 1]
    end

    # Working through `arr` backwards, place each value in the position
    # indicated by the values in `count`. Note, the call to `reverse(arr)` 
    # creates an iterator, so we're not iterating the values in `arr` and
    # modifying them at the same time.
    for val in reverse(arr) 
        place_value = (val ÷ place) % 10
        arr[count[place_value + 1]] = val
        count[place_value + 1] -= 1
    end
end

# Helper functions to determine the largest power of 10 represented by an
# integer or vector of integers. For example, `maxpow(999) == 2`, since
# 10^2 <= 999 <= 10^3.
maxpow(arr::Vector{Int}) = length(arr) > 0 ? maximum(arr) |> maxpow : 0
maxpow(n::Int)::Int = floor(log10(n))

"""
    radixsort!(arr)

Sort the integers in `arr` using a radix sort. For each place value represented
in any value in `arr`, perform a counting sort on that place value until the
entire array is sorted.

# Examples
```julia-repl
julia> radixsort!([170, 45, 75, 90, 802, 24, 2, 66])
8-element Vector{Int64}:
   2
  24
  45
  66
  75
  90
 170
 802
```
"""
function radixsort!(arr::Vector{Int})
    foreach(0:maxpow(arr)) do power
        place = 10 ^ power
        countingsort!(arr, place)
    end
end


# Just adding the tests to the bottom of the script for convenience
using Test
using Random

@testset "Should sort examples correctly" begin
    input1 = [1, 2, 10, 50, 5]
    radixsort!(input1)
    @test input1 == [1, 2, 5, 10, 50]

    # Empty integer vector
    input2 = Vector{Int}()
    radixsort!(input2)
    @test input2 == []
end

@testset "Should sort random lists of integers correctly" begin
    input = abs.(rand(Int, 1000))
    expected = sort(input)
    radixsort!(input)
    @test input == expected
end