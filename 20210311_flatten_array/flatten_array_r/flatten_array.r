library(testthat)

# Given a nested list as input, flatten into an atomic vector
flatten <- function(input) {
  output <- c()
  for (v in input) {
    if (length(v) == 1) {
      output <- c(output, v[[1]])
    } else {
      output <- c(output, flatten(v))
    }
  }
  output
}

test_that("`flatten` function works", {
  input <- list(1, 2, 3, list(4, 5, 6, list(7, 8, 9)))
  expected <- c(1, 2, 3, 4, 5, 6, 7, 8, 9)
  
  expect_equal(flatten(input), expected)
})