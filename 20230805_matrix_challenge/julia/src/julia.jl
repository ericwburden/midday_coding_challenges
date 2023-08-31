module julia

"""
  max_rect_histogram(heights::AbstractVector{Numeric})

Given a numeric vector where each value can represent the height 
of a bar in a histogram, calculate the largest rectangular area
covered by the bars of such a histogram.

For example, the values [6, 2, 5, 4, 5, 1, 6] can be interpreted
as the histogram shown below, with the largest rectangular area
indicated by the `#` characters and other bars by `=`.

  |
  |=     =
  |= = = =
  |= ### =
  |= ### =
  |==### =
  |==###==
  |--------
"""
function max_rect_histogram(heights)
  stack = Vector()
  max_area = 0
  index = 1

  # Iterate over the heights given in the input. Here, we're using a while loop
  # since sometimes we want to process one or more bars from the top of the 
  # stack before moving on to the next height in the input.
  while index <= length(heights)
    # Set up "bars" as named tuples. Get the bar from the current index
    # of the input.
    current_bar = (index=index, height=heights[index])

    if isempty(stack) || last(stack).height <= current_bar.height
      # If the stack is empty or the current bar is taller than the 
      # bar on top of the stack, add the current bar to the stack
      # and move on to the next input index.
      push!(stack, current_bar)
      index += 1
    else
      # If the top of the stack is at least as tall as the current bar,
      # we remove it from the stack and calculate the area of the 
      # largest rectangle possible with a height equal to the (now)
      # previous bar at the top of the stack. The width of the rectangle
      # is given by the current index - the index of the (new) top of the
      # stack. The extra `-1` is to account for Julia being 1-indexed.
      current_bar = pop!(stack)
      width = isempty(stack) ? index - 1 : index - last(stack).index - 1
      area = current_bar.height * width
      max_area = max(area, max_area)
    end
  end

  # Check any bars remaining on the stack after iterating over the
  # heights.
  while !isempty(stack)
    current_bar = pop!(stack)
    width = isempty(stack) ? index - 1 : index - last(stack).index - 1
    area = current_bar.height * width
    max_area = max(area, max_area)
  end

  max_area
end


"""
  matrix_challenge(str_arr::AbstractVector{String})

Given a list of strings containing only 1's and 0's representing a rectangular
matrix, determine the area of the largest rectangle composed of the character
'1' and return that area.
"""
function matrix_challenge(str_arr)
  # Conver the input list of strings into a matrix of 1's and 0's
  (matrix = str_arr
            |> (x -> map(collect, x))
            |> (x -> map(y -> parse.(Int, y), x))
            |> (x -> map(y -> reshape(y, 1, :), x))
            |> (x -> reduce(vcat, x)))

  rows, cols = size(matrix)

  # First we iterate over every column, transforming every value in
  # the matrix to represent the number of 1's between the current value
  # and the next 0 below it in the same column (or the bottom of the column).
  # In this way, each row becomes a 'histogram' representing the height
  # of all 1-wide rectangular bars in that row.
  for col in 1:cols
    height = 0
    for row in rows:-1:1
      if matrix[row, col] == 0
        height = 0
      else
        matrix[row, col] += height
        height += 1
      end
    end
  end

  # With each row representing a histogram, we determine the maximum
  # rectangle for each histogram, then return the largest overall 
  # area.
  (eachrow(matrix)
   |> (x -> map(max_rect_histogram, x))
   |> maximum)
end

end # module julia
