# A classroom consists of N students, whose friendships can be represented in an
# adjacency list. For example, the following descibes a situation where 0 is friends
# with 1 and 2, 3 is friends with 6, and so on.
# 
# {0: [1, 2],
#  1: [0, 5],
#  2: [0],
#  3: [6],
#  4: [],
#  5: [1],
#  6: [3]} 
# 
# Each student can be placed in a friend group, which can be defined as the
# transitive closure of that student's friendship relations. In other words, this is
# the smallest set such that no student in the group has any friends outside this
# group. For the example above, the friend groups would be {0, 1, 2, 5}, {3, 6}, {4}.
# 
# Given a friendship list such as the one above, determine the number of friend
# groups in the class.

module julia

export friendgroups

function friendgroups(friendlist::Dict{Int64, Vector{Int64}})::Int64
    groups = 0      # Initialize number of groups
    visited = Set() # The id's (nodes) we have visited

    # For each key/value pair in the input dictionary...
    for (key, value) in friendlist

        # If the key has already been visited, skip this one
        key in visited && continue

        # Otherwise, increase groups by one and visit all the friends for this key (ID)
        # We need to be sure that every id connected to `key` is added to `visited`. So,
        # we follow the id's through `friendlist` until they've all been visited.
        # `setdiff()` yields the id's in `value` that aren't in `visited`.
        groups += 1
        to_visit = setdiff(value, visited)
        while !isempty(to_visit)
            to_visit_next = Set()  # The next set of id's to visit

            # Add each id connected to an id in `to_visit` to `to_visit_next`
            map(id -> union!(to_visit_next, friendlist[id]), collect(to_visit))
            
            # Add all the id's from `to_visit` to the list of visited ids
            union!(visited, to_visit)

            # Set `to_visit` to the ids in `to_visit_next` that aren't in `visited`
            to_visit = setdiff(to_visit_next, visited)
        end
    end

    return groups
end

end # module
