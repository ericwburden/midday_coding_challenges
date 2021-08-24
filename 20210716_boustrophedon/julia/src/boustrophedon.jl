module boustrophedon

export Node, nodepush!, readboustrophedon

"""
A node in a tree structure. Contains a value, a left node, and a right node.
"""
mutable struct Node{T}
    value::T
    left::Union{Node{T}, Nothing}
    right::Union{Node{T}, Nothing}
end

"A more terse form of the default constructor, no need to specify left/right."
function Node(value::T) where T <: Any
    Node(value, nothing, nothing)
end

"""
    nodepush!(node, value)

Push a new value onto a tree, starting with `node`. Will attempt to insert the new
value as a node into the first empty 'slot' on the tree, searching top to bottom,
left to right.
"""
function nodepush!(node::Node{T}, value::T) where T <: Any
    current_row = [node]
    while true
        next_row = []
        for n in current_row
            if isnothing(n.left)
                n.left = Node(value)
                return node
            else 
                push!(next_row, n.left)
            end

            if isnothing(n.right)
                n.right = Node(value)
                return node
            else
                push!(next_row, n.right)
            end
        end # for
        current_row = next_row
    end # while
end

function nodepush!(value1::T, value2::T) where T <: Any
    return nodepush!(Node(value1), value2)
end

"""
    readboustrophedon(head)

Given a link to a tree node `head`, read all the descendant values in that tree in
'boustrophedon' order, that is, alternating left-to-right and right-to-left down the 
levels of the tree.
"""
function readboustrophedon(head::Node{T})::Vector{T} where T <: Any
    output = []
    current_row = [head]
    ltr = true

    while !isempty(current_row)
        next_row = []
        buffer = []
        for node in current_row
            # Builds a buffer in the direction needed for the output, if outputting
            # this row left-to-right, append values to the right side of the buffer.
            # If outputting this row right-to-left, push values to the left side of
            # the buffer.
            if ltr; push!(buffer, node.value); else; pushfirst!(buffer, node.value); end
            isnothing(node.left) || push!(next_row, node.left)
            isnothing(node.right) || push!(next_row, node.right)
        end
        append!(output, buffer)
        ltr = !ltr
        current_row = next_row
    end
    return output
end

end # module

