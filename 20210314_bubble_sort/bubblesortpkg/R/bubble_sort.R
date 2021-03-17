#' Bubble Sorting - Rust Implementation
#'
#' @param n numeric vector to sort
#' @return a sorted integer vector
#' @export
#' @useDynLib bubblesortpkg bubble_sort_wrapper
bubble_sort_rust <- function(n) {
  n <- if (missing(n)) {
    as.integer(trunc(stats::runif(1000, 0, 1000)))
  } else {
    as.integer(n)
  }
  .Call(bubble_sort_wrapper, n)
}


#' Bubble Sorting - R Implementation
#'
#' @param nums numeric vector to sort
#' @return a sorted integer vector
#' @export
bubble_sort_r <- function(nums) {
  nums <- if (missing(nums)) {
    as.integer(trunc(stats::runif(1000, 0, 1000)))
  } else {
    as.integer(nums)
  }

  n <- length(nums)
  for (i in 1:(n-1)) {
    for (j in 1:(n - i)) {
      if (nums[j] > nums[j + 1]) {
        temp <- nums[j]
        nums[j] <- nums[j + 1]
        nums[j + 1] <- temp
      }
    }
  }

  nums
}


#' Run a benchmark comparing Rust and R implementations
#'
#' @return NA
#' @export
bubble_bench <- function() {
  input <- as.integer(trunc(stats::runif(10000, 0, 10000)))
  benchmark <- microbenchmark::microbenchmark(
    bubble_sort_rust(input),
    bubble_sort_r(input)
  )
  print(benchmark)
  plot(benchmark)
}
