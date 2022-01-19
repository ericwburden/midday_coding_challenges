# Converts a single number into digits
to_digits <- function(n) {
  stopifnot(length(n) == 1)
  (n
    |> as.character()
    |> strsplit("")
    |> unlist()
    |> as.numeric())
}

# Returns the combinations of a vector up to `len`
# `combn(c(1, 2, 3, 4), 2)` yields:
#      [,1] [,2] [,3] [,4] [,5] [,6]
# [1,]    1    1    1    2    2    3
# [2,]    2    3    4    3    4    4
combn_of_len <- function(len, vec) {
  stopifnot(length(vec) >= len)
  (vec
    |> combn(len)
    |> apply(2, paste, collapse = ""))
}

# Returns all combinations of the digits of `n` such that:
# - The order of the digits is not changed
# - The value of the combined digits is less than n
digit_combinations <- function(n) {
  digits <- to_digits(n)
  len <- length(digits) - 1
  (seq(len)
    |> lapply(combn_of_len, digits)
    |> unlist()
    |> as.numeric())
}

# Find the combinations of the digits of `m` that are divisible by `n`
like_numbers <- function(m, n) {
  dc <- digit_combinations(m)
  sum(dc %% n == 0)
}

# Tests ------------------------------------------------------------------------

library(testthat)

testthat::test_that("Should pass the examples", {
  m <- 1234
  n <- 2
  like_numbers <- like_numbers(m, n)
  expect_equal(like_numbers, 9)

  m <- 768
  n <- 4
  like_numbers <- like_numbers(m, n)
  expect_equal(like_numbers, 3)
})
