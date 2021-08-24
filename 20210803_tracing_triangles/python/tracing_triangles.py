def longest_path(array):
    # store a 1d array and update results
    store = [None] * len(array)
    n = len(array) - 1

    # store the bottom row
    for i in range(len(array[n])):
        store[i] = array[n][i]

    print(store)
    # walk backwards through the rows starting from the second to last
    for i in range(len(array) - 2, -1, -1):
        # walk through the items in each row by index
        for j in range(len(array[i])):
            # update the store at each index with the arrays current value plus the
            # max of the store current and  store next
            store[j] = array[i][j] + max(store[j], store[j + 1])
        print(store)
    # the 0th position should hold the largest value since we are walking backwards
    return store[0]

input_one = [[6], [4, 4], [1, 2, 1], [5, 4, 3, 2]]
input_two = [[1], [2, 3], [1, 5, 1]]
print(longest_path(input_one)) # 16
print(longest_path(input_two)) # 9
