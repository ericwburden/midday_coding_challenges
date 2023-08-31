def get_parent_value(height, value):
    # The root of a sub-tree of height `height` numbered by post-order
    # traversal is the same as the number of nodes in the tree, and can be
    # calculated as...
    subtree_size = (2**height) - 1

    # If the value we're looking for is the root or is too big for our
    # tree, return -1.
    if subtree_size <= value:
        return -1

    # Used for calculating child node values. Represents the offset from the
    # leftmost branch.
    offset = 0

    # If the parent value is in our tree, start checking sub-trees for
    # the `value` we were given. We'll start by searching down the left
    # branch...
    while subtree_size > 0:
        # Each subtree is half the size of the previous sub-tree, truncated.
        subtree_size = subtree_size >> 1

        # If we're on the leftmost branch, the value of the left child
        # will be the number of nodes in its subtree. If we're shifted 
        # to a more rightward branch, it's the size of the subtree plus
        # the number of nodes to the left.
        left_child = offset + subtree_size

        # The value of the right child is the value of the left child plus
        # the size of the subtree.
        right_child = left_child + subtree_size

        # If we've found our target value as either a right or left child value
        # then we can calculate the parent value as one plus the right child
        # value (or 2 * the left child plus one).
        if left_child == value or right_child == value:
            return right_child + 1

        # If the target value is greater than our left child value, then
        # we need to search down the right side.
        if value > left_child:
            offset = left_child

    return -1

def solution(h, q):
    return [get_parent_value(h, n) for n in q]


# Sanity check
if __name__ == '__main__':
    print solution(3, [7, 3, 5, 1])
    print solution(5, [19, 14, 28])
