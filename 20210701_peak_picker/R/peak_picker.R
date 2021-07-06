new_peak_list <- function(pos, peaks) {
  stopifnot(is.numeric(pos))
  stopifnot(is.numeric(peaks))
  stopifnot(length(pos) == length(peaks))

  structure(list(pos = pos, peaks = peaks), class = "peak_list")
}

peak_picker <- function(nums = numeric(0)) {
  stopifnot(is.numeric(nums) & length(nums) > 1)

  # `rle()` produces a list where [['lengths']] is the run length of a given
  # value in the input, and [['values']] are the values
  with(rle(nums), {
    # Identify the RLE values that are greater than the value to the right
    # and to the left
    peak_locs <- values > shift_right(values) & values > shift_left(values)
    peaks <- values[peak_locs]

    # Calculate the index where each run starts
    indices <- cumsum(lengths) - (lengths - 1)
    pos <- indices[peak_locs]

    new_peak_list(pos, peaks)
  })
}

shift_left <- function(nums = numeric(0)) {
  stopifnot(is.numeric(nums) & length(nums) >= 1)
  c(nums[2:length(nums)], Inf)
}

shift_right <- function(nums = numeric(0)) {
  stopifnot(is.numeric(nums) & length(nums) >= 1)
  c(Inf, nums[1:length(nums) - 1])
}



require(testthat)

test_that("Can find a single peak", {
  nums <- c(0, 1, 2, 5, 1, 0)
  peak_list <- peak_picker(nums)

  expect_identical(peak_list$pos, 4)
  expect_identical(peak_list$peaks, 5)
})

test_that("Can find multiple peaks", {
  nums <- c(3, 2, 3, 6, 4, 1, 2, 3, 2, 1, 2, 3)
  peak_list <- peak_picker(nums)

  expect_identical(peak_list$pos, c(4, 8))
  expect_identical(peak_list$peaks, c(6, 3))
})

test_that("Ignores the first and last values", {
  nums <- c(100, 1, 2, 1, 100)
  peak_list <- peak_picker(nums)

  expect_identical(peak_list$pos, 3)
  expect_identical(peak_list$peaks, 2)
})

test_that("Handles plateaus", {
  nums <- c(1, 2, 2, 1, 0, 1, 1, 1, 1, 1, 0)
  peak_list <- peak_picker(nums)

  expect_identical(peak_list$pos, c(2, 6))
  expect_identical(peak_list$peaks, c(2, 1))
})
