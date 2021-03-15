#' **Array Extender**
#' Write a program that takes in 2 arrays of unequal length, and extends the
#' shorter array by duplicating its values until the lengths of the first array
#' and second array are equal in length. Return the array that was modified. If
#' they are equal in length, return nothing:
#'
#' Example:
#'   array1 = [1,2,3,4,5] # len 5
#'   array2 = ['a', 'b', 'c'] # len 3
#'   output: array2 = ['a', 'b', 'c', 'a', 'b'] # len 5
#'
#'   array1 = [1,2,3] # len 3
#'   array2 = ['Jack', 'Jill', 'Jimmy', 'Jan', 'Juniper', 'Jenn', 'Jane', 'Jeremy', 'Jerome', 'Jess'] # len 10
#'   output: array1 = [1,2,3,1,2,3,1,2,3,1] # len 10
#'
#'   array1 = [1,2,3]
#'   array2 = [4,5,6]
#'   output: empty return

library(testthat)

# Function ---------------------------------------------------------------------
extend_array <- function(...) {
  # In R, ... represents an arbitrary number of named or unnamed arguments
  args <- list(...)  # Putting the arguments in a list makes them easier to work with
  output_length <- max(lengths(args))  # Length of the longest vector

  # If all the vectors are the same length, return NA
  if (all(output_length == lengths(args))) { return(NA) }

  shortest_vec <- args[order(lengths(args))][[1]] # The shortest vector

  # rep_len will output items from the input vector into a new vector,
  # recycling items until outputting a vector of length == `length.out`
  rep_len(shortest_vec, length.out = output_length)
}


# Test Cases -------------------------------------------------------------------
test_that("shorter arrays are appropriately extended", {
  expect_identical(
    extend_array(c(1, 2, 3, 4, 5), c('a', 'b', 'c')),
    c('a', 'b', 'c', 'a', 'b')
  )

  expect_identical(
    extend_array(
      c(1, 2, 3),
      c('Jack', 'Jill'   , 'Jimmy' , 'Jan', 'Juniper', 'Jenn',
        'Jane', 'Jeremy' , 'Jerome', 'Jess')
    ),
    c(1, 2, 3, 1, 2, 3, 1, 2, 3, 1)
  )
})

test_that("same length arrays return `NA`", {
  expect_identical(extend_array(c(1, 2, 3), c(4, 5, 6)), NA)
  expect_identical(extend_array(c(1, 2, 3), c(4, 5, 6), c(7, 8, 9)), NA)
})

test_that("more than two arrays are handled correctly", {
  expect_identical(
    extend_array(c(1, 2, 3),
                 c(4, 5),
                 c(3, 6, 9, 10, 12),
                 c(T, F, T, F, T, F, T, F, T)),
    c(4, 5, 4, 5, 4, 5, 4, 5, 4)
  )
})
