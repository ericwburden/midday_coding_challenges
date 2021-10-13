# Given a string and a number of lines k, print the string in zigzag form. In zigzag,
# characters are printed out diagonally from top left to bottom right until reaching
# the kth line, then back up to top right, and so on.
# 
# For example, given the sentence "thisisazigzag" and k = 4, you should print:
# 
# ```
# t     a     g
#  h   s z   a
#   i i   i z
#    s     g
# ```


module julia

export printzigzag

function printzigzag(sentence, k)
    out = fill(' ', (k, length(sentence)))
    direction = 1
    (row, col) = (1, 1)

    for c in sentence
        out[row, col] = c
        col += 1
        if row == 1; direction = 1; end
        if row == k; direction = -1; end
        row += direction
    end

    lines = mapslices(join, out, dims = 2)
    return join(lines, '\n')
end

end # module
