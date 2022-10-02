class_photos <- function(red_shirt_heights, blue_shirt_heights) {
  # Error if either vector is not numeric
  stopifnot(is.numeric(red_shirt_heights) && is.numeric(blue_shirt_heights))

  # Error if the vectors are two different sizes
  stopifnot(length(red_shirt_heights) == length(blue_shirt_heights))

  # Sort both
  sorted_red  <- sort(red_shirt_heights)
  sorted_blue <- sort(blue_shirt_heights)

  # Calculate a vector of 1, 0, -1 as the sign of the result of subtracting
  # the sorted blue heights from the sorted red heights
  diff_signs  <- sign(sorted_red - sorted_blue)

  # Get the sum of all the sign values. If lines can be made then this
  # value will either be the length of one of the input vectors or the
  # negative length of the input vector. This happens if `diff_signs`
  # values are all either 1 or -1.
  total_diff  <- sum(diff_signs)

  abs(total_diff) == length(red_shirt_heights)
}

# Tests -----------------------------------------------------------------------
require(testthat)

test_that("Possible arrangements are found", {
  class_photos(c(6, 9, 2, 4, 5), c(5, 8, 1, 3, 4)) |> expect_true()
  class_photos(c(5, 8, 1, 3, 4), c(6, 9, 2, 4, 5)) |> expect_true()
  class_photos(c(2, 3, 4, 5, 6), c(1, 2, 3, 4, 5)) |> expect_true()
  class_photos(c(2, 2, 2, 2, 2), c(1, 1, 1, 1, 1)) |> expect_true()

  class_photos(c(2), c(1)) |> expect_true()
  class_photos(c(1), c(2)) |> expect_true()
})

test_that("Impossible arrangements are found", {
  class_photos(c(5, 8, 1, 3, 4, 9), c(6, 9, 2, 4, 5, 1)) |> expect_false()
  class_photos(
    c(5, 6, 7, 2, 3, 1, 2, 3),
    c(1, 1, 1, 1, 1, 1, 1, 1)
  ) |> expect_false()

  class_photos(c(1), c(1)) |> expect_false()
  class_photos(c(2), c(2)) |> expect_false()
})
