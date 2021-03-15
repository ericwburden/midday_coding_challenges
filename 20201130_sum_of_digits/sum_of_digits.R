#' **Summing a number’s digits**
#' 
#' Write a function named sumDigits which takes a number as input and returns 
#' the sum of the absolute value of each of the number’s decimal digits. 
#' For example:
#' 
#'   sumDigits(10);  // Returns 1
#'   sumDigits(99);  // Returns 18
#'   sumDigits(-32); // Returns 5
#'   
#' Assume that all numbers in the input will be integer values

library(testthat)

# Functions --------------------------------------------------------------------

#' Helper function, counts the number of digits in a given number
count_digits <- function(n) {
  truncated <- floor(abs(n))
  if (truncated > 1) { floor(log10(truncated)) + 1 } else { 1 }
}

#' Helper function, gets the digit in a number `n` for a given place value `p`
place_value <- function(p, n) {
  (n %/% p) %% 10
}

#' Converts a single number `n` to a vector containing the number's digits,
#' ignoring negative values
digits <- function(n) {
  num_digits <- count_digits(n)
  if (num_digits == 0) { 0 } else {
    places <- 10^(c(num_digits:1) - 1)
    sapply(places, place_value, abs(n))
  }
}

#' Calculates the sum of the absolute value of all the digits in a given integer
sum_of_digits <- function(n) {
  sum(digits(n))
}

# Tests ------------------------------------------------------------------------

test_that("sum_of_digits() returns expected values", {
  expect_identical(sum_of_digits(10), 1)
  expect_identical(sum_of_digits(99), 18)
  expect_identical(sum_of_digits(-32), 5)
})