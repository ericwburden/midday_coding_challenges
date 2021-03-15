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
#' @param n numeric vector to sort
#' @return a sorted integer vector
#' @export
bubble_sort_r <- function(n) {
  n <- if (missing(n)) {
    as.integer(trunc(stats::runif(1000, 0, 1000)))
  } else {
    as.integer(n)
  }

  idx <- 1
  last_idx <- length(n) - 1
  swapped <- FALSE

  while (TRUE) {
    while (idx <= last_idx) {
      if (n[idx] > n[idx + 1]) {
        temp <- n[idx]
        n[idx] <- n[idx + 1]
        n[idx + 1] <- temp
        swapped <- TRUE
      }
      idx <- idx + 1
    }

    if (swapped) {
      idx <- 1
      swapped <- FALSE
      next
    }

    break
  }

  n
}


#' Run a benchmark comparing Rust and R implementations
#'
#' @return NA
#' @export
bubble_bench <- function() {
  input <- as.integer(trunc(stats::runif(1000, 0, 1000)))
  benchmark <- microbenchmark::microbenchmark(
    bubble_sort_rust(input),
    bubble_sort_r(input)
  )
  print(benchmark)
  plot(benchmark)
}
