module julia

export longest_path

function longest_path(tree, row=1, col=1)
    row == length(tree) && return tree[row][col]

    left = longest_path(tree, row + 1, col)
    right = longest_path(tree, row + 1, col + 1)
    return max(left, right) + tree[row][col]
end

function longest_path_iter(tree)
    length(tree) == 1 && return tree[1][1]

    working_copy = tree
    for row in (length(tree)-1):-1:1
        for col in 1:length(row)
            left = tree[row + 1][col]
            right = tree[row + 1][col + 1]
            working_copy[row][col] += max(left, right)
        end
    end

    return working_copy[1][1]
end

end # module
