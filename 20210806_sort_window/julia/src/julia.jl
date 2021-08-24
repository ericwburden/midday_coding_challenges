# Given an array of integers out of order, determine the bounds of the smallest
# window that must be sorted in order for the entire array to be sorted. For example,
# given [3, 7, 5, 6, 9], you should return (1, 3).
    
module julia

export sortwindow

function sortwindow(arr::Vector{T})::Vector{Int} where T <: Any
    sorted_arr = sort(arr)
    unsorted_idx = []
    
    for idx in 1:length(arr)
        arr[idx] != sorted_arr[idx] && push!(unsorted_idx, idx)
    end

    isempty(unsorted_idx) && return []
    return [first(unsorted_idx), last(unsorted_idx)]
end

end # module
