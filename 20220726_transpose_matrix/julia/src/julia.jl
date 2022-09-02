module julia

export transpose_builtin, transpose_byhand

transpose_builtin(mx::Matrix) = permutedims(mx)

function transpose_byhand(mx::Matrix)
    rows, cols = size(mx)
    [mx[j, i] for i in 1:cols, j in 1:rows]
end

end # module
