module julia

# 1. Find the largest index k such that arr[k] < arr[k + 1].
# 2. If there is no such k, there is no next permutation. Return nothing.
# 3. Find the largest index l greater than k such that arr[k] < arr[l]
# 4. Swap the value of arr[k] with arr[l]
# 5. Reverse the sequence from a[k+1] up to and including the final element of arr
function next_permutation(arr)
    k = findlast(diff(arr) .> 0)    # 1
    k == nothing && return nothing  # 2
    l = findlast(arr .> arr[k])     # 3
    arr[k], arr[l] = arr[l], arr[k] # 4
    reverse!(arr, k + 1)            # 5
end

# A mutating version of `next_permutation` for in-place array modification
next_permutation!(arr) = arr = next_permutation(arr)

function swaps_to_kth_wonderful_number(num, k)
    num_arr = collect(num)  # To character array

    # Advance to the next 'wonderful number' k times
    kth_wonderful_number = copy(num_arr)
    [next_permutation!(num_arr) for _ in 1:k]

    # Subtract the index of the last changed number from the index
    # of the first changed number
    diffs = kth_wonderful_number .!= num_arr
    findlast(diffs) - findfirst(diffs)
end

end # module
